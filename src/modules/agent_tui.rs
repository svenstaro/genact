//! Pretend to run a modern coding agent TUI.
use async_trait::async_trait;
use instant::Instant;
use rand::seq::IndexedRandom;
use rand::{RngExt, rng};
use yansi::Paint;

use crate::args::AppConfig;
use crate::io::{
    csleep, cursor_up, erase_line, get_terminal_width, hide_cursor, newline, print, show_cursor,
};
use crate::modules::Module;

pub struct AgentTui;

#[derive(Clone, Copy)]
enum TestStyle {
    Rust,
    Pytest,
    Node,
    Go,
    Ctest,
    Shell,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum AgentMode {
    Plan,
    Build,
}

impl AgentMode {
    fn label(self) -> &'static str {
        match self {
            Self::Plan => "plan",
            Self::Build => "build",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Complexity {
    Small,
    Normal,
    Complex,
    Deep,
}

impl Complexity {
    fn choose() -> Self {
        let mut rng = rng();
        match rng.random_range(0..100) {
            0..=19 => Self::Small,
            20..=69 => Self::Normal,
            70..=92 => Self::Complex,
            _ => Self::Deep,
        }
    }

    fn initial_reads(self) -> usize {
        match self {
            Self::Small => 1,
            Self::Normal => 2,
            Self::Complex => 2,
            Self::Deep => 3,
        }
    }

    fn web_probability(self) -> f64 {
        match self {
            Self::Small => 0.03,
            Self::Normal => 0.11,
            Self::Complex => 0.28,
            Self::Deep => 0.48,
        }
    }

    fn subagent_probability(self) -> f64 {
        match self {
            Self::Small => 0.02,
            Self::Normal => 0.10,
            Self::Complex => 0.28,
            Self::Deep => 0.46,
        }
    }

    fn first_failure_probability(self) -> f64 {
        match self {
            Self::Small => 0.22,
            Self::Normal => 0.36,
            Self::Complex => 0.47,
            Self::Deep => 0.55,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum EntryStrategy {
    Reproduce,
    Explore,
    Read,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum VerificationDepth {
    FocusedOnly,
    Package,
    Full,
}

impl VerificationDepth {
    fn needs_package(self) -> bool {
        matches!(self, Self::Package | Self::Full)
    }

    fn needs_full(self) -> bool {
        matches!(self, Self::Full)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TestScope {
    Focused,
    Package,
    Full,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TodoPhase {
    Investigating,
    Editing,
    Verifying,
    Reviewing,
    Done,
}

struct BehaviorPlan {
    entry: EntryStrategy,
    verification: VerificationDepth,
    analysis_only: bool,
    use_todo: bool,
    extra_searches: usize,
    extra_reads: usize,
    review_diff: bool,
    refine_patch: bool,
    confirm_external: bool,
    preflight_diagnostics: bool,
}

impl BehaviorPlan {
    fn choose(complexity: Complexity, mode: AgentMode) -> Self {
        let mut rng = rng();

        let entry_roll: u32 = rng.random_range(0..100);
        let entry = match complexity {
            Complexity::Small => match entry_roll {
                0..=24 => EntryStrategy::Reproduce,
                25..=74 => EntryStrategy::Explore,
                _ => EntryStrategy::Read,
            },
            Complexity::Normal => match entry_roll {
                0..=31 => EntryStrategy::Reproduce,
                32..=74 => EntryStrategy::Explore,
                _ => EntryStrategy::Read,
            },
            Complexity::Complex | Complexity::Deep => match entry_roll {
                0..=36 => EntryStrategy::Reproduce,
                37..=71 => EntryStrategy::Explore,
                _ => EntryStrategy::Read,
            },
        };

        let verification = match complexity {
            Complexity::Small => {
                if rng.random_bool(0.28) {
                    VerificationDepth::Package
                } else {
                    VerificationDepth::FocusedOnly
                }
            }
            Complexity::Normal => match rng.random_range(0..100) {
                0..=34 => VerificationDepth::FocusedOnly,
                35..=89 => VerificationDepth::Package,
                _ => VerificationDepth::Full,
            },
            Complexity::Complex => match rng.random_range(0..100) {
                0..=14 => VerificationDepth::FocusedOnly,
                15..=74 => VerificationDepth::Package,
                _ => VerificationDepth::Full,
            },
            Complexity::Deep => match rng.random_range(0..100) {
                0..=7 => VerificationDepth::FocusedOnly,
                8..=54 => VerificationDepth::Package,
                _ => VerificationDepth::Full,
            },
        };

        let analysis_probability: f64 = match complexity {
            Complexity::Small => 0.08,
            Complexity::Normal => 0.08,
            Complexity::Complex => 0.06,
            Complexity::Deep => 0.05,
        } + if mode == AgentMode::Plan { 0.04 } else { 0.0 };

        let extra_searches: usize = match complexity {
            Complexity::Small => rng.random_range(0..=1),
            Complexity::Normal => rng.random_range(0..=1),
            Complexity::Complex => rng.random_range(0..=2),
            Complexity::Deep => rng.random_range(1..=2),
        };
        let extra_reads: usize = match complexity {
            Complexity::Small => 0,
            Complexity::Normal => rng.random_range(0..=1),
            Complexity::Complex => rng.random_range(0..=2),
            Complexity::Deep => rng.random_range(1..=2),
        };

        let review_probability: f64 = match complexity {
            Complexity::Small => 0.24,
            Complexity::Normal => 0.52,
            Complexity::Complex => 0.76,
            Complexity::Deep => 0.90,
        };
        let review_diff = rng.random_bool(review_probability);
        let refine_patch = review_diff
            && rng.random_bool(match complexity {
                Complexity::Small => 0.05,
                Complexity::Normal => 0.11,
                Complexity::Complex => 0.20,
                Complexity::Deep => 0.29,
            });

        Self {
            entry,
            verification,
            use_todo: matches!(complexity, Complexity::Complex | Complexity::Deep)
                && rng.random_bool(0.34),
            analysis_only: rng.random_bool(analysis_probability),
            extra_searches,
            extra_reads,
            review_diff,
            refine_patch,
            confirm_external: rng.random_bool(complexity.web_probability() * 0.62),
            preflight_diagnostics: rng.random_bool(match complexity {
                Complexity::Small => 0.08,
                Complexity::Normal => 0.18,
                Complexity::Complex => 0.30,
                Complexity::Deep => 0.42,
            }),
        }
    }
}

struct Scenario {
    task: &'static str,
    cwd: &'static str,
    files: &'static [&'static str],
    symbol: &'static str,
    snippet: &'static str,
    test_command: &'static str,
    test_style: TestStyle,
    failing_test: &'static str,
    diagnosis: &'static str,
    retry_diagnosis: &'static str,
    summary: &'static str,
}

const SCENARIOS: &[Scenario] = &[
    Scenario {
        task: "Fix the flaky websocket reconnect test",
        cwd: "~/code/websocket-client",
        files: &[
            "src/net/client.rs",
            "src/net/retry.rs",
            "tests/reconnect.rs",
        ],
        symbol: "retry_count",
        snippet: "self.retry_count = 0;",
        test_command: "cargo test reconnect",
        test_style: TestStyle::Rust,
        failing_test: "websocket::reconnect_preserves_backoff",
        diagnosis: "The retry counter is reset inside the connection attempt, so timeout reconnects never advance past attempt zero.",
        retry_diagnosis: "The attempt-limit check still happens after connect_once(), which gives the timeout path one extra retry.",
        summary: "Retry state now survives timeout reconnects while preserving the existing attempt limit.",
    },
    Scenario {
        task: "Make environment variables override values from the config file",
        cwd: "~/projects/config-engine",
        files: &["src/config.rs", "src/args.rs", "tests/config.rs"],
        symbol: "merge_config",
        snippet: "config.merge(file_values);",
        test_command: "cargo test config",
        test_style: TestStyle::Rust,
        failing_test: "config::environment_has_highest_precedence",
        diagnosis: "File values are merged after the environment layer, so the precedence order is reversed for conflicting keys.",
        retry_diagnosis: "The optional CLI layer uses the same helper and still needs to stay above environment values.",
        summary: "Configuration precedence is now defaults < file < environment < CLI.",
    },
    Scenario {
        task: "Handle UTF-8 boundaries correctly in the streaming parser",
        cwd: "~/src/stream-parser",
        files: &["src/parser.rs", "src/buffer.rs", "tests/parser_unicode.rs"],
        symbol: "split_at",
        snippet: "let (head, tail) = input.split_at(limit);",
        test_command: "cargo test parser_unicode",
        test_style: TestStyle::Rust,
        failing_test: "parser_unicode::split_multibyte_character",
        diagnosis: "The chunk limit is measured in bytes and can split a multibyte code point before the parser validates the boundary.",
        retry_diagnosis: "The carry buffer also drops incomplete bytes when the next read starts with the remaining code-point bytes.",
        summary: "Streaming parsing now keeps chunk boundaries on valid UTF-8 offsets.",
    },
    Scenario {
        task: "Add bounded exponential backoff to the async API client",
        cwd: "~/work/python-api-client",
        files: &["client/http.py", "client/retry.py", "tests/test_retry.py"],
        symbol: "asyncio.sleep",
        snippet: "await asyncio.sleep(self.retry_delay)",
        test_command: "pytest -q tests/test_retry.py",
        test_style: TestStyle::Pytest,
        failing_test: "test_retry_uses_exponential_backoff",
        diagnosis: "Every retry currently sleeps for the same fixed interval instead of deriving the delay from the attempt number.",
        retry_diagnosis: "Jitter is applied after the configured cap, allowing the final delay to exceed the maximum.",
        summary: "Transient API failures now use bounded exponential backoff with jitter.",
    },
    Scenario {
        task: "Fix stale cache entries surviving exactly at their TTL boundary",
        cwd: "~/projects/service-cache",
        files: &["app/cache.py", "app/clock.py", "tests/test_cache.py"],
        symbol: "expires_at",
        snippet: "if entry.expires_at < time.time():",
        test_command: "pytest -q tests/test_cache.py",
        test_style: TestStyle::Pytest,
        failing_test: "test_expired_entry_is_removed_on_read",
        diagnosis: "The expiration comparison treats an entry expiring exactly now as valid for one more read.",
        retry_diagnosis: "The bulk lookup path duplicates the old boundary comparison instead of sharing the helper.",
        summary: "Cache reads now consistently evict entries at and beyond their TTL boundary.",
    },
    Scenario {
        task: "Avoid sending duplicate refresh-token requests",
        cwd: "~/dev/dashboard-web",
        files: &[
            "src/auth/client.ts",
            "src/auth/session.ts",
            "test/auth.test.ts",
        ],
        symbol: "refreshPromise",
        snippet: "const token = await refreshToken();",
        test_command: "npm test -- --runInBand auth",
        test_style: TestStyle::Node,
        failing_test: "shares one in-flight refresh request",
        diagnosis: "Each failed request starts a new refresh operation instead of awaiting one shared in-flight promise.",
        retry_diagnosis: "The shared promise is retained after rejection, so later requests keep observing the rejected refresh.",
        summary: "Concurrent authentication failures now share one refresh request and clean up after completion.",
    },
    Scenario {
        task: "Fix the search box debounce race",
        cwd: "~/dev/search-ui",
        files: &[
            "src/search/controller.ts",
            "src/search/api.ts",
            "test/search.test.ts",
        ],
        symbol: "setTimeout",
        snippet: "timer = setTimeout(() => runSearch(query), delay);",
        test_command: "npm test -- search",
        test_style: TestStyle::Node,
        failing_test: "ignores responses from superseded searches",
        diagnosis: "A slower response from an older query can overwrite the latest result because responses are not tied to a request generation.",
        retry_diagnosis: "The empty-query path returns before invalidating the previous request generation.",
        summary: "Debounced search now ignores stale responses and invalidates obsolete requests.",
    },
    Scenario {
        task: "Propagate context cancellation through the HTTP worker",
        cwd: "~/work/go-worker",
        files: &[
            "internal/http/worker.go",
            "internal/http/client.go",
            "internal/http/worker_test.go",
        ],
        symbol: "context.Background",
        snippet: "req = req.WithContext(context.Background())",
        test_command: "go test ./internal/http/...",
        test_style: TestStyle::Go,
        failing_test: "TestWorkerStopsOnContextCancel",
        diagnosis: "The worker replaces the caller context with a background context before issuing the request.",
        retry_diagnosis: "The retry helper creates a fresh request without copying the original context.",
        summary: "HTTP work now preserves caller cancellation through initial and retried requests.",
    },
    Scenario {
        task: "Close response bodies on every retry path",
        cwd: "~/code/go-http-client",
        files: &[
            "internal/client/client.go",
            "internal/client/retry.go",
            "internal/client/client_test.go",
        ],
        symbol: "resp.Body",
        snippet: "if shouldRetry(resp.StatusCode) {",
        test_command: "go test ./internal/client/...",
        test_style: TestStyle::Go,
        failing_test: "TestRetryClosesPreviousResponseBody",
        diagnosis: "The retry branch continues before closing the previous response body, leaking connections under repeated failures.",
        retry_diagnosis: "The redirect retry path has a separate early continue with the same leak.",
        summary: "Every retry path now closes the previous response body before issuing another request.",
    },
    Scenario {
        task: "Reject truncated packets before reading the payload header",
        cwd: "~/src/packet-parser",
        files: &[
            "src/protocol/parser.cpp",
            "src/protocol/packet.hpp",
            "tests/parser_test.cpp",
        ],
        symbol: "payload_length",
        snippet: "auto len = buffer[offset + 3];",
        test_command: "ctest --test-dir build -R parser",
        test_style: TestStyle::Ctest,
        failing_test: "Parser.TruncatedPayloadHeader",
        diagnosis: "The parser reads the payload-length byte before checking that the full header is available.",
        retry_diagnosis: "The extended-header branch needs its own bounds check before reading the extended length.",
        summary: "Packet parsing now validates header bounds before every payload-length read.",
    },
    Scenario {
        task: "Fix include-directory propagation for the static library target",
        cwd: "~/projects/native-core",
        files: &[
            "CMakeLists.txt",
            "src/CMakeLists.txt",
            "tests/CMakeLists.txt",
        ],
        symbol: "target_include_directories",
        snippet: "target_include_directories(core PRIVATE include)",
        test_command: "cmake --build build && ctest --test-dir build",
        test_style: TestStyle::Ctest,
        failing_test: "headers_are_visible_to_consumers",
        diagnosis: "Public headers are exposed with PRIVATE visibility, so downstream targets cannot resolve the include path.",
        retry_diagnosis: "Generated headers need to be exported through the build interface as well.",
        summary: "The static library now exports both source and generated public include directories.",
    },
    Scenario {
        task: "Make file replacement atomic when saving local state",
        cwd: "~/tools/ops-toolkit",
        files: &[
            "scripts/save_state.sh",
            "scripts/common.sh",
            "tests/state_save.sh",
        ],
        symbol: "mv ",
        snippet: "cat \"$tmp\" > \"$state_file\"",
        test_command: "bash tests/state_save.sh",
        test_style: TestStyle::Shell,
        failing_test: "interrupted_write_keeps_previous_state",
        diagnosis: "The destination is overwritten in place, so interruption can leave the state file truncated.",
        retry_diagnosis: "The fallback temporary file is created on a different filesystem, so rename is no longer guaranteed to be atomic.",
        summary: "State saving now writes a sibling temporary file and atomically replaces the destination.",
    },
];

#[derive(Clone, Copy)]
enum DelayKind {
    Think,
    DeepThink,
    ToolStart,
    Explore,
    Search,
    Read,
    Web,
    Edit,
    Test,
    Lsp,
    Subagent,
    Git,
    AfterText,
    BetweenTasks,
}

fn tiered_delay(normal: (u64, u64), slow: (u64, u64), very_slow: (u64, u64)) -> u64 {
    let mut rng = rng();
    match rng.random_range(0..100) {
        0..=69 => rng.random_range(normal.0..normal.1),
        70..=89 => rng.random_range(slow.0..slow.1),
        _ => rng.random_range(very_slow.0..very_slow.1),
    }
}

fn sample_delay(kind: DelayKind) -> u64 {
    match kind {
        DelayKind::Think => tiered_delay((2_200, 5_200), (5_200, 8_500), (8_500, 13_000)),
        DelayKind::DeepThink => tiered_delay((4_000, 7_500), (7_500, 12_000), (12_000, 18_000)),
        DelayKind::ToolStart => tiered_delay((450, 1_100), (1_100, 1_900), (1_900, 3_000)),
        DelayKind::Explore => tiered_delay((1_800, 4_000), (4_000, 6_500), (6_500, 9_500)),
        DelayKind::Search => tiered_delay((2_000, 4_800), (4_800, 7_800), (7_800, 11_500)),
        DelayKind::Read => tiered_delay((1_600, 3_900), (3_900, 6_300), (6_300, 9_000)),
        DelayKind::Web => tiered_delay((4_000, 7_500), (7_500, 11_500), (11_500, 16_000)),
        DelayKind::Edit => tiered_delay((2_300, 5_000), (5_000, 8_000), (8_000, 12_000)),
        DelayKind::Test => tiered_delay((7_000, 13_000), (13_000, 21_000), (21_000, 31_000)),
        DelayKind::Lsp => tiered_delay((800, 1_900), (1_900, 3_200), (3_200, 5_000)),
        DelayKind::Subagent => tiered_delay((4_000, 8_000), (8_000, 13_000), (13_000, 19_000)),
        DelayKind::Git => tiered_delay((1_000, 2_400), (2_400, 3_800), (3_800, 5_500)),
        DelayKind::AfterText => tiered_delay((900, 2_100), (2_100, 3_600), (3_600, 5_500)),
        DelayKind::BetweenTasks => tiered_delay((1_500, 3_000), (3_000, 4_600), (4_600, 6_500)),
    }
}

struct AgentState {
    modified_files: Vec<&'static str>,
    additions: u32,
    deletions: u32,
    tests_run: u32,
    failures: u32,
    used_web: bool,
    used_subagent: bool,
    used_lsp: bool,
    reviewed_diff: bool,
    refined_patch: bool,
}

impl AgentState {
    fn new() -> Self {
        Self {
            modified_files: Vec::new(),
            additions: 0,
            deletions: 0,
            tests_run: 0,
            failures: 0,
            used_web: false,
            used_subagent: false,
            used_lsp: false,
            reviewed_diff: false,
            refined_patch: false,
        }
    }

    fn modified(&mut self, file: &'static str, additions: u32, deletions: u32) {
        if !self.modified_files.contains(&file) {
            self.modified_files.push(file);
        }
        self.additions += additions;
        self.deletions += deletions;
    }

    fn refine_patch(&mut self, removed_additions: u32, removed_deletions: u32) {
        self.additions = self.additions.saturating_sub(removed_additions);
        self.deletions = self.deletions.saturating_sub(removed_deletions);
        self.refined_patch = true;
    }
}

struct Renderer<'a> {
    appconfig: &'a AppConfig,
    started: Instant,
    mode: AgentMode,
    model: &'static str,
    width: usize,
    footer_visible: bool,
}

impl<'a> Renderer<'a> {
    fn new(appconfig: &'a AppConfig, mode: AgentMode, model: &'static str) -> Self {
        let width = get_terminal_width().saturating_sub(2).clamp(60, 160);
        Self {
            appconfig,
            started: Instant::now(),
            mode,
            model,
            width,
            footer_visible: false,
        }
    }

    fn dim<S: Into<String>>(value: S) -> String {
        Paint::new(value.into()).dim().to_string()
    }

    fn elapsed_label(&self) -> String {
        let seconds = self.started.elapsed().as_secs();
        if seconds < 60 {
            format!("{seconds}s")
        } else {
            format!("{}m {:02}s", seconds / 60, seconds % 60)
        }
    }

    fn rule(&self) -> String {
        "─".repeat(self.width)
    }

    fn status_line(&self, status: &str) -> String {
        let raw = format!(
            " {} · {}   {}   {}",
            self.mode.label(),
            self.model,
            status,
            self.elapsed_label()
        );
        raw.chars().take(self.width).collect()
    }

    async fn show_footer(&mut self, status: &str) {
        if self.footer_visible {
            cursor_up(1).await;
            erase_line().await;
            print(Self::dim(self.status_line(status))).await;
            newline().await;
            return;
        }
        print(Self::dim(self.rule())).await;
        newline().await;
        print(Self::dim(self.status_line(status))).await;
        newline().await;
        self.footer_visible = true;
    }

    async fn update_footer(&mut self, status: &str) {
        if !self.footer_visible {
            print(Self::dim(self.rule())).await;
            newline().await;
            print(Self::dim(self.status_line(status))).await;
            newline().await;
            self.footer_visible = true;
            return;
        }
        cursor_up(1).await;
        erase_line().await;
        print(Self::dim(self.status_line(status))).await;
        newline().await;
    }

    async fn clear_footer(&mut self) {
        if !self.footer_visible {
            return;
        }
        cursor_up(1).await;
        erase_line().await;
        cursor_up(1).await;
        erase_line().await;
        self.footer_visible = false;
    }

    async fn wait_ms(&mut self, millis: u64) -> bool {
        csleep(millis).await;
        if self.appconfig.should_exit() {
            self.clear_footer().await;
            return false;
        }
        true
    }

    async fn status_wait(&mut self, status: &str, kind: DelayKind) -> bool {
        const FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        const SPINNER_FRAME_MS: u64 = 80;

        let total = sample_delay(kind);
        let mut elapsed = 0u64;
        let mut frame = 0usize;

        self.show_footer(status).await;
        while elapsed < total {
            let label = format!("{} {status}", FRAMES[frame % FRAMES.len()]);
            self.update_footer(&label).await;
            let step = SPINNER_FRAME_MS.min(total - elapsed);
            csleep(step).await;
            elapsed += step;
            frame += 1;

            if self.appconfig.should_exit() {
                self.clear_footer().await;
                return false;
            }
        }
        self.update_footer("working").await;
        true
    }

    async fn header(&mut self, scenario: &'static Scenario, prompt: &str) {
        self.clear_footer().await;
        // Each module run is one coding-agent session. Reset the viewport so the
        // outer genact loop looks like opening a fresh session rather than
        // appending another log block below the previous one.
        print("\x1b[2J\x1b[H").await;

        print(format!("{}", Paint::cyan("genact code").bold())).await;
        print(Self::dim(format!("  {}", scenario.cwd))).await;
        newline().await;
        print(Self::dim(format!("{} · {}", self.mode.label(), self.model))).await;
        newline().await;
        print(Self::dim(self.rule())).await;
        newline().await;
        newline().await;
        print(format!("> {prompt}")).await;
        newline().await;
        newline().await;

        self.show_footer("thinking").await;
    }

    async fn reasoning(
        &mut self,
        paragraphs: &[String],
        deep: bool,
        highlight_paragraph: Option<usize>,
    ) -> bool {
        if !self
            .status_wait(
                "thinking",
                if deep {
                    DelayKind::DeepThink
                } else {
                    DelayKind::Think
                },
            )
            .await
        {
            return false;
        }

        for (paragraph_index, paragraph) in paragraphs.iter().enumerate() {
            let highlighted = highlight_paragraph == Some(paragraph_index);
            for line in wrap_text(paragraph, self.width.saturating_sub(6).min(92)) {
                self.clear_footer().await;
                print("  ").await;
                if !self.stream_line(&line, !highlighted).await {
                    return false;
                }
                newline().await;
                self.show_footer("thinking").await;
            }

            self.clear_footer().await;
            newline().await;
            self.show_footer("thinking").await;
        }

        self.update_footer("working").await;
        self.wait_ms(sample_delay(DelayKind::AfterText)).await
    }

    async fn stream_line(&mut self, line: &str, dimmed: bool) -> bool {
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.is_empty() {
            return true;
        }

        let mut rng = rng();
        let mut index = 0usize;
        while index < words.len() {
            let count = rng.random_range(2..=6).min(words.len() - index);
            let end = index + count;
            let mut burst = words[index..end].join(" ");
            if end < words.len() {
                burst.push(' ');
            }

            if dimmed {
                print(Self::dim(burst)).await;
            } else {
                print(burst).await;
            }

            let last = words[end - 1];
            let pause = if last.ends_with('.') || last.ends_with('?') || last.ends_with('!') {
                rng.random_range(150..430)
            } else if last.ends_with(',') || last.ends_with(';') || last.ends_with(':') {
                rng.random_range(70..220)
            } else {
                rng.random_range(25..105)
            };
            csleep(pause).await;
            if self.appconfig.should_exit() {
                return false;
            }
            index = end;
        }
        true
    }

    async fn tool_header(&mut self, label: &str, detail: &str) {
        self.clear_footer().await;
        let line = if detail.is_empty() {
            format!("  ┃ {label}")
        } else {
            format!("  ┃ {label:<11}{detail}")
        };
        print(Paint::new(line).dim().to_string()).await;
        newline().await;
        self.show_footer("working").await;
    }

    async fn tool_output(&mut self, text: &str, status: &str, pause: u64) -> bool {
        self.clear_footer().await;
        print(Self::dim(format!("  ┃ {text}"))).await;
        newline().await;
        self.show_footer(status).await;
        self.wait_ms(pause).await
    }

    async fn tool_output_colored(
        &mut self,
        prefix: &str,
        value: &str,
        success: bool,
        status: &str,
        pause: u64,
    ) -> bool {
        self.clear_footer().await;
        print(Self::dim(format!("  ┃ {prefix}"))).await;
        if success {
            print(Paint::green(value).bold().to_string()).await;
        } else {
            print(Paint::red(value).bold().to_string()).await;
        }
        newline().await;
        self.show_footer(status).await;
        self.wait_ms(pause).await
    }

    async fn explore(&mut self, scenario: &'static Scenario) -> bool {
        self.tool_header("Explore", "workspace").await;
        if !self.wait_ms(sample_delay(DelayKind::ToolStart)).await {
            return false;
        }
        if !self
            .status_wait("scanning workspace", DelayKind::Explore)
            .await
        {
            return false;
        }
        self.clear_footer().await;

        let mut rng = rng();
        let expanded = rng.random_bool(0.36);
        if expanded {
            for file in scenario.files {
                print(Self::dim(format!("  ┃ {file}"))).await;
                newline().await;
                if !self.wait_ms(rng.random_range(180..520)).await {
                    return false;
                }
            }
        } else {
            print(Self::dim(format!(
                "  ┃ {} files in likely path",
                scenario.files.len()
            )))
            .await;
            newline().await;
        }
        newline().await;
        self.show_footer("working").await;
        true
    }

    async fn search(&mut self, scenario: &'static Scenario) -> bool {
        self.search_term(scenario, scenario.symbol).await
    }

    async fn search_term(&mut self, scenario: &'static Scenario, term: &str) -> bool {
        self.tool_header("Search", term).await;
        if !self.wait_ms(sample_delay(DelayKind::ToolStart)).await {
            return false;
        }
        if !self
            .status_wait("searching workspace", DelayKind::Search)
            .await
        {
            return false;
        }
        self.clear_footer().await;

        let mut rng = rng();
        let match_count = rng.random_range(2..15);
        if rng.random_bool(0.38) {
            for file in scenario
                .files
                .iter()
                .take(rng.random_range(1..=scenario.files.len()))
            {
                let line: u32 = rng.random_range(20..280);
                print(Self::dim(format!("  ┃ {file}:{line}"))).await;
                newline().await;
                if !self.wait_ms(rng.random_range(220..620)).await {
                    return false;
                }
            }
            print(Self::dim(format!("  ┃ {match_count} matches"))).await;
        } else {
            print(Self::dim(format!(
                "  ┃ {match_count} matches across {} files",
                scenario.files.len()
            )))
            .await;
        }
        newline().await;
        newline().await;
        self.show_footer("working").await;
        true
    }

    async fn read(&mut self, scenario: &'static Scenario, file: &'static str) -> bool {
        self.tool_header("Read", file).await;
        if !self.wait_ms(sample_delay(DelayKind::ToolStart)).await {
            return false;
        }
        if !self.status_wait("reading file", DelayKind::Read).await {
            return false;
        }
        self.clear_footer().await;

        let mut rng = rng();
        if rng.random_bool(0.46) {
            let line: u32 = rng.random_range(30..260);
            print(Self::dim(format!("  ┃ {line:4} │ …"))).await;
            newline().await;
            if !self.wait_ms(rng.random_range(180..520)).await {
                return false;
            }
            print(Self::dim(format!(
                "  ┃ {:4} │ {}",
                line + 1,
                scenario.snippet
            )))
            .await;
            newline().await;
            if !self.wait_ms(rng.random_range(250..760)).await {
                return false;
            }
            print(Self::dim(format!("  ┃ {:4} │ …", line + 2))).await;
            newline().await;
        } else {
            print(Self::dim(format!(
                "  ┃ … {} lines",
                rng.random_range(24..110)
            )))
            .await;
            newline().await;
        }
        newline().await;
        self.show_footer("working").await;
        true
    }

    async fn lsp_symbols(&mut self, scenario: &'static Scenario, state: &mut AgentState) -> bool {
        self.tool_header("Symbols", scenario.symbol).await;
        if !self
            .status_wait("querying language server", DelayKind::Lsp)
            .await
        {
            return false;
        }
        self.clear_footer().await;
        let symbols = [scenario.symbol, "handle_error", "apply_state"];
        for symbol in symbols {
            print(Self::dim(format!("  ┃ {symbol}"))).await;
            newline().await;
        }
        newline().await;
        state.used_lsp = true;
        self.show_footer("working").await;
        true
    }

    async fn todo(&mut self, scenario: &'static Scenario, phase: TodoPhase) -> bool {
        self.tool_header("Todo", "").await;
        if !self.wait_ms(sample_delay(DelayKind::ToolStart)).await {
            return false;
        }
        self.clear_footer().await;

        let active_index = match phase {
            TodoPhase::Investigating => 0usize,
            TodoPhase::Editing => 2,
            TodoPhase::Verifying => 3,
            TodoPhase::Reviewing => 4,
            TodoPhase::Done => 5,
        };
        let rows = [
            "inspect the reported behavior".to_string(),
            format!("trace {} and the affected call path", scenario.symbol),
            format!("patch {}", scenario.files[0]),
            "run focused and surrounding tests".to_string(),
            "review the final diff".to_string(),
        ];

        for (index, row) in rows.iter().enumerate() {
            print(Self::dim("  ┃ ")).await;
            if phase == TodoPhase::Done || index < active_index {
                print(Paint::green("✓").to_string()).await;
            } else if index == active_index {
                print(Paint::cyan("◉").bold().to_string()).await;
            } else {
                print(Self::dim("○")).await;
            }
            print(Self::dim(format!(" {row}"))).await;
            newline().await;
            if !self.wait_ms(140).await {
                return false;
            }
        }
        newline().await;
        self.show_footer("working").await;
        true
    }

    async fn diagnostics(
        &mut self,
        file: &'static str,
        clean: bool,
        state: &mut AgentState,
    ) -> bool {
        self.tool_header("Diagnostics", file).await;
        if !self
            .status_wait("checking diagnostics", DelayKind::Lsp)
            .await
        {
            return false;
        }
        self.clear_footer().await;
        if clean {
            print(Self::dim("  ┃ no problems found")).await;
        } else {
            let mut rng = rng();
            let warnings: u32 = rng.random_range(1..3);
            print(Self::dim(format!(
                "  ┃ 0 errors, {warnings} warning{}",
                if warnings == 1 { "" } else { "s" }
            )))
            .await;
        }
        newline().await;
        newline().await;
        state.used_lsp = true;
        self.show_footer("working").await;
        true
    }

    async fn subagent(&mut self, scenario: &'static Scenario, state: &mut AgentState) -> bool {
        self.tool_header("Task", "trace related call paths").await;
        if !self
            .status_wait("general · exploring", DelayKind::Subagent)
            .await
        {
            return false;
        }
        self.clear_footer().await;
        print(Self::dim("  ┃ Found two relevant execution paths")).await;
        newline().await;
        for file in scenario.files.iter().take(2) {
            print(Self::dim(format!("  ┃ • {file}"))).await;
            newline().await;
        }
        print(Self::dim(format!(
            "  ┃ Both converge around `{}`",
            scenario.symbol
        )))
        .await;
        newline().await;
        newline().await;
        state.used_subagent = true;
        self.show_footer("working").await;
        true
    }

    async fn web_search(&mut self, scenario: &'static Scenario, state: &mut AgentState) -> bool {
        let query = web_query(scenario);
        self.tool_header("WebSearch", &query).await;
        if !self.status_wait("searching web", DelayKind::Web).await {
            return false;
        }
        self.clear_footer().await;
        let results = web_results(scenario.test_style);
        let mut rng = rng();
        let count = rng.random_range(2..=4);
        for result in results.iter().take(count) {
            print(Self::dim(format!("  ┃ {result}"))).await;
            newline().await;
            if !self.wait_ms(rng.random_range(280..780)).await {
                return false;
            }
        }
        print(Self::dim(format!("  ┃ {count} relevant results"))).await;
        newline().await;
        newline().await;
        state.used_web = true;
        self.show_footer("working").await;
        true
    }

    async fn switch_mode(&mut self, mode: AgentMode) -> bool {
        self.clear_footer().await;
        let old = self.mode.label();
        let new = mode.label();
        print(Self::dim(format!("  ┃ Agent      {old} → {new}"))).await;
        newline().await;
        self.mode = mode;
        self.show_footer("switching agent").await;
        if !self.wait_ms(1_250).await {
            return false;
        }
        self.update_footer("working").await;
        true
    }

    async fn edit(&mut self, file: &'static str, state: &mut AgentState) -> bool {
        self.tool_header("Edit", file).await;
        if !self.wait_ms(sample_delay(DelayKind::ToolStart)).await {
            return false;
        }
        if !self.status_wait("applying patch", DelayKind::Edit).await {
            return false;
        }
        self.clear_footer().await;

        let mut rng = rng();
        let additions = rng.random_range(4..20);
        let deletions = rng.random_range(1..10);
        state.modified(file, additions, deletions);
        print(Self::dim("  ┃ ")).await;
        print(
            Paint::new(format!("+{additions}"))
                .green()
                .bold()
                .to_string(),
        )
        .await;
        print(" ").await;
        print(Paint::new(format!("-{deletions}")).red().bold().to_string()).await;
        newline().await;
        newline().await;
        self.show_footer("working").await;
        true
    }

    async fn run_tests(
        &mut self,
        scenario: &'static Scenario,
        state: &mut AgentState,
        command: &str,
        passed: bool,
        total: u32,
    ) -> bool {
        self.tool_header("Bash", command).await;
        state.tests_run += 1;
        if !passed {
            state.failures += 1;
        }

        if !self.wait_ms(sample_delay(DelayKind::ToolStart)).await {
            return false;
        }
        if !self.render_test_startup(scenario.test_style).await {
            return false;
        }
        if !self.status_wait("running command", DelayKind::Test).await {
            return false;
        }
        if !self
            .render_test_result(scenario.test_style, scenario.failing_test, passed, total)
            .await
        {
            return false;
        }
        self.show_footer("working").await;
        true
    }

    async fn render_test_startup(&mut self, style: TestStyle) -> bool {
        let mut rng = rng();
        match style {
            TestStyle::Rust => {
                if !self
                    .tool_output(
                        "Compiling workspace…",
                        "building tests",
                        rng.random_range(900..2_200),
                    )
                    .await
                {
                    return false;
                }
                self.tool_output(
                    "Finished `test` profile [unoptimized + debuginfo]",
                    "starting tests",
                    rng.random_range(700..1_600),
                )
                .await
            }
            TestStyle::Pytest => {
                self.tool_output(
                    "================ test session starts ================",
                    "collecting tests",
                    rng.random_range(700..1_700),
                )
                .await
            }
            TestStyle::Node => {
                self.tool_output(
                    "RUNS test suite",
                    "starting test runner",
                    rng.random_range(700..1_600),
                )
                .await
            }
            TestStyle::Go => {
                self.tool_output(
                    "go: building test binary…",
                    "building tests",
                    rng.random_range(800..1_900),
                )
                .await
            }
            TestStyle::Ctest => {
                self.tool_output(
                    "[100%] Built target tests",
                    "starting ctest",
                    rng.random_range(900..2_100),
                )
                .await
            }
            TestStyle::Shell => {
                self.tool_output(
                    "running integration checks…",
                    "starting checks",
                    rng.random_range(650..1_500),
                )
                .await
            }
        }
    }

    async fn render_test_result(
        &mut self,
        style: TestStyle,
        failing_test: &str,
        passed: bool,
        total: u32,
    ) -> bool {
        let mut rng = rng();
        match style {
            TestStyle::Rust => {
                if !self
                    .tool_output(
                        &format!("running {total} tests"),
                        "running command",
                        rng.random_range(700..1_500),
                    )
                    .await
                {
                    return false;
                }
                if !self
                    .tool_output(
                        "test initializes_state ... ok",
                        "running command",
                        rng.random_range(450..1_200),
                    )
                    .await
                {
                    return false;
                }
                if passed {
                    if !self
                        .tool_output_colored(
                            &format!("test {failing_test} ... "),
                            "ok",
                            true,
                            "running command",
                            rng.random_range(700..1_500),
                        )
                        .await
                    {
                        return false;
                    }
                    self.tool_output_colored(
                        "test result: ",
                        "ok",
                        true,
                        "working",
                        rng.random_range(500..1_100),
                    )
                    .await
                } else {
                    if !self
                        .tool_output_colored(
                            &format!("test {failing_test} ... "),
                            "FAILED",
                            false,
                            "running command",
                            rng.random_range(800..1_800),
                        )
                        .await
                    {
                        return false;
                    }
                    self.tool_output_colored(
                        &format!("{} passed; 1 failed · ", total.saturating_sub(1)),
                        "FAILED",
                        false,
                        "working",
                        rng.random_range(600..1_300),
                    )
                    .await
                }
            }
            TestStyle::Pytest => {
                if !self
                    .tool_output(
                        &format!("collected {total} items"),
                        "running command",
                        rng.random_range(650..1_500),
                    )
                    .await
                {
                    return false;
                }
                if passed {
                    self.tool_output_colored(
                        "tests/ ........................ ",
                        "PASSED",
                        true,
                        "working",
                        rng.random_range(900..2_000),
                    )
                    .await
                } else {
                    if !self
                        .tool_output_colored(
                            "tests/ ............... ",
                            "FAILED",
                            false,
                            "running command",
                            rng.random_range(800..1_800),
                        )
                        .await
                    {
                        return false;
                    }
                    self.tool_output_colored(
                        &format!("{failing_test} · "),
                        "1 failed",
                        false,
                        "working",
                        rng.random_range(600..1_300),
                    )
                    .await
                }
            }
            TestStyle::Node => {
                if passed {
                    self.tool_output_colored(
                        &format!("{failing_test} · "),
                        "PASS",
                        true,
                        "working",
                        rng.random_range(900..2_000),
                    )
                    .await
                } else {
                    self.tool_output_colored(
                        &format!("{failing_test} · "),
                        "FAIL",
                        false,
                        "working",
                        rng.random_range(900..2_000),
                    )
                    .await
                }
            }
            TestStyle::Go => {
                if !self
                    .tool_output(
                        &format!("=== RUN   {failing_test}"),
                        "running command",
                        rng.random_range(800..1_800),
                    )
                    .await
                {
                    return false;
                }
                if passed {
                    self.tool_output_colored(
                        "project/package · ",
                        "ok",
                        true,
                        "working",
                        rng.random_range(700..1_500),
                    )
                    .await
                } else {
                    self.tool_output_colored(
                        &format!("--- {failing_test} · "),
                        "FAIL",
                        false,
                        "working",
                        rng.random_range(700..1_500),
                    )
                    .await
                }
            }
            TestStyle::Ctest => {
                if !self
                    .tool_output(
                        &format!("Start 1: {failing_test}"),
                        "running command",
                        rng.random_range(900..2_000),
                    )
                    .await
                {
                    return false;
                }
                if passed {
                    self.tool_output_colored(
                        &format!("{total}/{total} tests · "),
                        "passed",
                        true,
                        "working",
                        rng.random_range(700..1_500),
                    )
                    .await
                } else {
                    self.tool_output_colored(
                        &format!("{} passed, 1 failed · ", total.saturating_sub(1)),
                        "FAILED",
                        false,
                        "working",
                        rng.random_range(700..1_500),
                    )
                    .await
                }
            }
            TestStyle::Shell => {
                if passed {
                    self.tool_output_colored(
                        &format!("{total} checks · "),
                        "passed",
                        true,
                        "working",
                        rng.random_range(700..1_500),
                    )
                    .await
                } else {
                    self.tool_output_colored(
                        &format!("{failing_test} · "),
                        "failed",
                        false,
                        "working",
                        rng.random_range(700..1_500),
                    )
                    .await
                }
            }
        }
    }

    async fn review_diff(&mut self, scenario: &'static Scenario, state: &mut AgentState) -> bool {
        let command = format!("git diff -- {}", scenario.files[0]);
        self.tool_header("Bash", &command).await;
        if !self.status_wait("reviewing diff", DelayKind::Git).await {
            return false;
        }
        self.clear_footer().await;

        let mut rng = rng();
        let line: u32 = rng.random_range(40..220);
        print(Self::dim(format!("  ┃ @@ -{line},5 +{line},7 @@"))).await;
        newline().await;
        print(Self::dim("  ┃ ")).await;
        print(
            Paint::new(format!("-{}", scenario.snippet))
                .red()
                .to_string(),
        )
        .await;
        newline().await;
        print(Self::dim("  ┃ ")).await;
        print(
            Paint::new(format!("+{}", fixed_snippet(scenario)))
                .green()
                .to_string(),
        )
        .await;
        newline().await;
        print(Self::dim(format!(
            "  ┃ {} file{} changed · +{} -{}",
            state.modified_files.len(),
            if state.modified_files.len() == 1 {
                ""
            } else {
                "s"
            },
            state.additions,
            state.deletions
        )))
        .await;
        newline().await;
        newline().await;
        state.reviewed_diff = true;
        self.show_footer("working").await;
        true
    }

    async fn refine_patch(&mut self, scenario: &'static Scenario, state: &mut AgentState) -> bool {
        self.tool_header("Edit", scenario.files[0]).await;
        if !self.wait_ms(sample_delay(DelayKind::ToolStart)).await {
            return false;
        }
        if !self.status_wait("simplifying patch", DelayKind::Edit).await {
            return false;
        }
        self.clear_footer().await;

        let mut rng = rng();
        let removed_additions = rng.random_range(1..=state.additions.clamp(1, 6));
        let removed_deletions = if state.deletions > 1 {
            rng.random_range(0..=state.deletions.min(3))
        } else {
            0
        };
        state.refine_patch(removed_additions, removed_deletions);
        let reduced = removed_additions + removed_deletions;
        print(Self::dim(format!(
            "  ┃ narrowed the patch by {reduced} line{}",
            if reduced == 1 { "" } else { "s" }
        )))
        .await;
        newline().await;
        newline().await;
        self.show_footer("working").await;
        true
    }

    async fn git_diff(&mut self, state: &AgentState) -> bool {
        self.tool_header("Bash", "git diff --stat").await;
        if !self
            .status_wait("reading working tree", DelayKind::Git)
            .await
        {
            return false;
        }
        self.clear_footer().await;
        for file in &state.modified_files {
            print(Self::dim(format!("  ┃ {file} | changed"))).await;
            newline().await;
        }
        print(Self::dim(format!(
            "  ┃ {} file{} changed, {} insertions(+), {} deletions(-)",
            state.modified_files.len(),
            if state.modified_files.len() == 1 {
                ""
            } else {
                "s"
            },
            state.additions,
            state.deletions
        )))
        .await;
        newline().await;
        newline().await;
        self.show_footer("working").await;
        true
    }

    async fn finish(&mut self, scenario: &'static Scenario, state: &AgentState) {
        self.clear_footer().await;
        print(format!("  {}", scenario.summary)).await;
        newline().await;
        newline().await;

        print(Self::dim(format!(
            "  Changed {} file{} (+{} -{})",
            state.modified_files.len(),
            if state.modified_files.len() == 1 {
                ""
            } else {
                "s"
            },
            state.additions,
            state.deletions,
        )))
        .await;
        newline().await;

        let test_summary = if state.failures == 0 {
            if state.tests_run == 1 {
                "Tests passed".to_string()
            } else {
                format!("Tests passed in {} runs", state.tests_run)
            }
        } else {
            format!(
                "Tests passed after {} retr{}",
                state.failures,
                if state.failures == 1 { "y" } else { "ies" }
            )
        };

        let mut details = vec![test_summary];
        if state.refined_patch {
            details.push("patch narrowed after review".to_string());
        } else if state.reviewed_diff {
            details.push("diff reviewed".to_string());
        }
        if state.used_web {
            details.push("web search used".to_string());
        }
        if state.used_subagent {
            details.push("subagent used".to_string());
        }
        if state.used_lsp {
            details.push("diagnostics checked".to_string());
        }
        print(Self::dim(format!("  {}", details.join(" · ")))).await;
        newline().await;
        newline().await;

        self.show_footer("done").await;
        let _ = self.wait_ms(sample_delay(DelayKind::BetweenTasks)).await;
        self.clear_footer().await;
    }

    async fn finish_no_changes(&mut self, scenario: &'static Scenario, state: &AgentState) {
        self.clear_footer().await;
        print("  No changes needed.").await;
        newline().await;
        newline().await;
        let conclusion = format!(
            "The current path around `{}` already matches the expected contract; the reported symptom does not require a production-code change in this checkout.",
            scenario.symbol
        );
        for line in wrap_text(&conclusion, self.width.saturating_sub(6).min(92)) {
            print(format!("  {line}")).await;
            newline().await;
        }
        newline().await;

        let mut details = vec!["0 files changed".to_string()];
        if state.tests_run > 0 {
            details.push(if state.failures == 0 {
                "focused check passes".to_string()
            } else {
                "behavior reproduced and isolated".to_string()
            });
        }
        if state.used_web {
            details.push("external behavior verified".to_string());
        }
        if state.used_lsp {
            details.push("diagnostics checked".to_string());
        }
        print(Self::dim(format!("  {}", details.join(" · ")))).await;
        newline().await;
        newline().await;

        self.show_footer("done").await;
        let _ = self.wait_ms(sample_delay(DelayKind::BetweenTasks)).await;
        self.clear_footer().await;
    }
}

fn wrap_text(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        let extra = if current.is_empty() {
            word.len()
        } else {
            word.len() + 1
        };
        if !current.is_empty() && current.len() + extra > width {
            lines.push(current);
            current = String::new();
        }
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(word);
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

fn prompt_for(scenario: &Scenario, plan: &BehaviorPlan) -> String {
    let mut rng = rng();
    if plan.analysis_only {
        return format!(
            "Investigate `{}` around @{} and tell me whether a code change is actually needed",
            scenario.symbol,
            scenario.files.first().copied().unwrap_or("src")
        );
    }

    if rng.random_bool(0.34) {
        format!(
            "{} in @{}",
            scenario.task,
            scenario.files.first().copied().unwrap_or("src")
        )
    } else if rng.random_bool(0.10) {
        format!(
            "Compare @{} with @{} and {}",
            scenario.files.first().copied().unwrap_or("src"),
            scenario.files.last().copied().unwrap_or("tests"),
            scenario.task.to_lowercase()
        )
    } else {
        scenario.task.to_string()
    }
}

fn initial_reasoning(
    scenario: &Scenario,
    complexity: Complexity,
    entry: EntryStrategy,
) -> Vec<String> {
    let mut result = match entry {
        EntryStrategy::Reproduce => vec![
            format!(
                "I'll reproduce `{}` first so I have a concrete failing path before reading through the implementation.",
                scenario.failing_test
            ),
            format!(
                "After that I'll trace `{}` from the failing test into the relevant call sites and keep the first edit as narrow as possible.",
                scenario.symbol
            ),
        ],
        EntryStrategy::Explore => vec![
            format!(
                "I'll trace `{}` first and compare the implementation with `{}` before changing anything.",
                scenario.symbol,
                scenario.files.last().copied().unwrap_or("the focused test")
            ),
            "I want to keep the first pass narrow: identify the state transition, confirm the contract in the test, then edit only the path responsible for the mismatch.".to_string(),
        ],
        EntryStrategy::Read => vec![
            format!(
                "I'll start in `{}` because it is the most likely owner of `{}`, then search outward from the concrete implementation rather than scanning the whole repository first.",
                scenario.files[0], scenario.symbol
            ),
            format!(
                "I'll compare what I find with `{}` before deciding whether this is a local fix or a wider control-flow issue.",
                scenario.files.last().copied().unwrap_or("the focused test")
            ),
        ],
    };

    if matches!(complexity, Complexity::Complex | Complexity::Deep) {
        result.push(
            "If the local code leaves an ambiguity around runtime or library behavior, I'll verify that separately rather than guessing from the failing test.".to_string(),
        );
    }
    result
}

fn diagnosis_reasoning(scenario: &Scenario) -> Vec<String> {
    vec![
        scenario.diagnosis.to_string(),
        format!(
            "The behavior is owned by `{}` rather than the helper around `{}`. I'll make the change there and keep the existing public contract intact.",
            scenario.files[0], scenario.symbol
        ),
    ]
}

fn plan_reasoning(scenario: &Scenario) -> Vec<String> {
    vec![
        scenario.diagnosis.to_string(),
        format!(
            "The implementation plan is small: update `{}`, keep `{}` as the regression target, then run the focused test before broadening verification.",
            scenario.files[0],
            scenario.files.last().copied().unwrap_or("the focused test")
        ),
        "I don't see a reason to broaden the refactor beyond that path, so I'll switch to implementation.".to_string(),
    ]
}

fn retry_reasoning(scenario: &Scenario) -> Vec<String> {
    vec![
        "The first change fixed the main path, but the focused test is still reaching a second branch.".to_string(),
        scenario.retry_diagnosis.to_string(),
        format!(
            "I'll inspect `{}` and adjust only that branch before rerunning the same focused command.",
            scenario.files.get(1).copied().unwrap_or(scenario.files[0])
        ),
    ]
}

fn external_question_reasoning(scenario: &Scenario) -> Vec<String> {
    vec![
        format!(
            "One piece is still ambiguous from the repository alone: whether the runtime or library guarantees the behavior around `{}` that this code appears to assume.",
            scenario.symbol
        ),
        "I'll verify that external contract before deciding whether the local branch needs another guard.".to_string(),
    ]
}

fn web_reasoning(scenario: &Scenario) -> Vec<String> {
    vec![
        "The external reference matches the local control flow, so there isn't an extra runtime guarantee to rely on here.".to_string(),
        format!(
            "That keeps the fix local to `{}`. I'll proceed with the repository-level state change and verify it with the existing tests.",
            scenario.files[0]
        ),
    ]
}

fn broader_test_reasoning(scope: TestScope) -> Vec<String> {
    match scope {
        TestScope::Focused => vec![
            "I'll rerun the focused regression once more before moving on.".to_string(),
        ],
        TestScope::Package => vec![
            "The focused regression is passing now. I'll run the surrounding package as a broader check before I treat the patch as complete.".to_string(),
        ],
        TestScope::Full => vec![
            "The package-level checks are green. I'll finish with the full workspace so this doesn't hide a cross-module regression.".to_string(),
        ],
    }
}

fn broad_failure_reasoning(scenario: &Scenario) -> Vec<String> {
    vec![
        "The focused regression passes, but the broader suite exposed a nearby compatibility path that the first patch did not cover.".to_string(),
        format!(
            "I'll inspect `{}` once more, make the smallest follow-up adjustment, and rerun the broader scope rather than expanding the refactor.",
            scenario.files.get(1).copied().unwrap_or(scenario.files[0])
        ),
    ]
}

fn diff_review_reasoning(scenario: &Scenario, refine: bool) -> Vec<String> {
    if refine {
        vec![
            format!(
                "The tests are green, but the diff around `{}` is broader than it needs to be.",
                scenario.symbol
            ),
            "I can preserve the behavior change while dropping the extra cleanup from the patch. I'll narrow it before the final verification.".to_string(),
        ]
    } else {
        vec![
            "The diff is limited to the path I intended to change and doesn't pull unrelated cleanup into the patch.".to_string(),
        ]
    }
}

fn analysis_only_reasoning(scenario: &Scenario) -> Vec<String> {
    vec![
        format!(
            "The current implementation around `{}` already follows the expected state transition in this checkout.",
            scenario.symbol
        ),
        "The reported symptom is consistent with an older assumption or stale reproduction rather than the code that is currently present, so changing production code here would be speculative.".to_string(),
    ]
}

fn test_command(scenario: &Scenario, scope: TestScope) -> String {
    match (scenario.test_style, scope) {
        (TestStyle::Rust, TestScope::Focused) => {
            let filter = scenario
                .failing_test
                .rsplit("::")
                .next()
                .unwrap_or(scenario.failing_test);
            format!("cargo test {filter}")
        }
        (TestStyle::Rust, TestScope::Package) => scenario.test_command.to_string(),
        (TestStyle::Rust, TestScope::Full) => "cargo test --workspace".to_string(),

        (TestStyle::Pytest, TestScope::Focused) => {
            format!("{} -k {}", scenario.test_command, scenario.failing_test)
        }
        (TestStyle::Pytest, TestScope::Package) => scenario.test_command.to_string(),
        (TestStyle::Pytest, TestScope::Full) => "pytest -q".to_string(),

        (TestStyle::Node, TestScope::Focused) => scenario.test_command.to_string(),
        (TestStyle::Node, TestScope::Package) => "npm test -- --runInBand".to_string(),
        (TestStyle::Node, TestScope::Full) => "npm test".to_string(),

        (TestStyle::Go, TestScope::Focused) => {
            format!("{} -run {}", scenario.test_command, scenario.failing_test)
        }
        (TestStyle::Go, TestScope::Package) => scenario.test_command.to_string(),
        (TestStyle::Go, TestScope::Full) => "go test ./...".to_string(),

        (TestStyle::Ctest, TestScope::Focused) => {
            if scenario.test_command.contains(" -R ") {
                scenario.test_command.to_string()
            } else {
                format!("ctest --test-dir build -R {}", scenario.failing_test)
            }
        }
        (TestStyle::Ctest, TestScope::Package) => "ctest --test-dir build".to_string(),
        (TestStyle::Ctest, TestScope::Full) => {
            "cmake --build build && ctest --test-dir build".to_string()
        }

        (TestStyle::Shell, TestScope::Focused) | (TestStyle::Shell, TestScope::Package) => {
            scenario.test_command.to_string()
        }
        (TestStyle::Shell, TestScope::Full) => "bash tests/run.sh".to_string(),
    }
}

fn fixed_snippet(scenario: &Scenario) -> String {
    match scenario.symbol {
        "retry_count" => "retry_state.advance_after_failure();".to_string(),
        "merge_config" => "config.merge(env_values);".to_string(),
        "split_at" => "let boundary = floor_char_boundary(input, limit);".to_string(),
        "asyncio.sleep" => "await asyncio.sleep(min(delay, self.max_retry_delay))".to_string(),
        "expires_at" => "if entry.expires_at <= time.time():".to_string(),
        "refreshPromise" => "const token = await getSharedRefreshPromise();".to_string(),
        "setTimeout" => "const generation = ++requestGeneration;".to_string(),
        "context.Background" => "req = req.WithContext(ctx)".to_string(),
        "resp.Body" => "defer resp.Body.Close()".to_string(),
        "payload_length" => "if (remaining < payload_length) return truncated;".to_string(),
        "target_include_directories" => {
            "target_include_directories(core PUBLIC include)".to_string()
        }
        "mv " => "mv -f \"$tmp\" \"$state_file\"".to_string(),
        _ => format!("/* adjusted {} handling */", scenario.symbol),
    }
}

fn web_query(scenario: &Scenario) -> String {
    let prefix = match scenario.test_style {
        TestStyle::Rust => "rust",
        TestStyle::Pytest => "python",
        TestStyle::Node => "typescript",
        TestStyle::Go => "go",
        TestStyle::Ctest => "c++",
        TestStyle::Shell => "posix shell",
    };
    format!("{prefix} {} behavior", scenario.symbol)
}

fn web_results(style: TestStyle) -> &'static [&'static str] {
    match style {
        TestStyle::Rust => &[
            "docs.rs API reference",
            "Rust standard library",
            "upstream issue discussion",
            "async patterns guide",
        ],
        TestStyle::Pytest => &[
            "Python library reference",
            "pytest documentation",
            "CPython issue discussion",
            "package API reference",
        ],
        TestStyle::Node => &[
            "Node.js API documentation",
            "TypeScript handbook",
            "MDN async behavior",
            "upstream issue discussion",
        ],
        TestStyle::Go => &[
            "Go package documentation",
            "Go context guide",
            "standard library source",
            "Go issue discussion",
        ],
        TestStyle::Ctest => &[
            "cppreference",
            "CMake documentation",
            "compiler documentation",
            "project issue discussion",
        ],
        TestStyle::Shell => &[
            "POSIX shell language",
            "Bash reference manual",
            "GNU coreutils manual",
            "portability notes",
        ],
    }
}

#[async_trait(?Send)]
impl Module for AgentTui {
    fn name(&self) -> &'static str {
        "agent_tui"
    }

    fn signature(&self) -> String {
        "agent-tui".to_string()
    }

    async fn run(&self, appconfig: &AppConfig) {
        hide_cursor().await;

        (async {
        let mut rng = rng();
        let scenario = SCENARIOS
            .choose(&mut rng)
            .expect("agent_tui must contain at least one scenario");
        let complexity = Complexity::choose();
        let start_mode = if rng.random_bool(0.24) {
            AgentMode::Plan
        } else {
            AgentMode::Build
        };
        let plan = BehaviorPlan::choose(complexity, start_mode);
        let models = ["openai/gpt", "anthropic/sonnet", "google/gemini-pro"];
        let model = models.choose(&mut rng).copied().unwrap_or("coding/model");
        let mut renderer = Renderer::new(appconfig, start_mode, model);
        let mut state = AgentState::new();
        let prompt = prompt_for(scenario, &plan);
        let focused_command = test_command(scenario, TestScope::Focused);

        renderer.header(scenario, &prompt).await;
        if !renderer
            .reasoning(
                &initial_reasoning(scenario, complexity, plan.entry),
                complexity == Complexity::Deep,
                None,
            )
            .await
        {
            return;
        }

        if plan.use_todo && !renderer.todo(scenario, TodoPhase::Investigating).await {
            return;
        }

        // V1.2 deliberately varies how the agent enters the problem instead of
        // always following Explore -> Search -> Read.
        match plan.entry {
            EntryStrategy::Reproduce => {
                let reproduction_passes = plan.analysis_only;
                if !renderer
                    .run_tests(
                        scenario,
                        &mut state,
                        &focused_command,
                        reproduction_passes,
                        rng.random_range(3..10),
                    )
                    .await
                {
                    return;
                }
                if complexity != Complexity::Small && !renderer.search(scenario).await {
                    return;
                }
                if !renderer.read(scenario, scenario.files[0]).await {
                    return;
                }
            }
            EntryStrategy::Explore => {
                if (complexity != Complexity::Small || rng.random_bool(0.58))
                    && !renderer.explore(scenario).await
                {
                    return;
                }
                if !renderer.search(scenario).await {
                    return;
                }
                let read_count = complexity.initial_reads().min(scenario.files.len());
                for file in scenario.files.iter().take(read_count) {
                    if !renderer.read(scenario, file).await {
                        return;
                    }
                }
            }
            EntryStrategy::Read => {
                if !renderer.read(scenario, scenario.files[0]).await {
                    return;
                }
                if rng.random_bool(0.72) && !renderer.search(scenario).await {
                    return;
                }
                if let Some(test_file) = scenario.files.last().copied()
                    && rng.random_bool(0.78)
                    && !renderer.read(scenario, test_file).await
                {
                    return;
                }
            }
        }

        if plan.preflight_diagnostics
            && !renderer
                .diagnostics(scenario.files[0], false, &mut state)
                .await
        {
            return;
        }

        if rng.random_bool(0.26) && !renderer.lsp_symbols(scenario, &mut state).await {
            return;
        }

        for search_index in 0..plan.extra_searches {
            let term = if search_index % 2 == 0 {
                scenario.failing_test
            } else {
                scenario.files.get(1).copied().unwrap_or(scenario.symbol)
            };
            if !renderer.search_term(scenario, term).await {
                return;
            }
        }

        for file in scenario.files.iter().rev().take(plan.extra_reads) {
            if !renderer.read(scenario, file).await {
                return;
            }
        }

        if rng.random_bool(complexity.subagent_probability())
            && !renderer.subagent(scenario, &mut state).await
        {
            return;
        }

        if plan.confirm_external {
            if !renderer
                .reasoning(&external_question_reasoning(scenario), false, None)
                .await
            {
                return;
            }
            if !renderer.web_search(scenario, &mut state).await {
                return;
            }
            if !renderer
                .reasoning(&web_reasoning(scenario), false, Some(0))
                .await
            {
                return;
            }
        }

        if plan.analysis_only {
            if !renderer
                .reasoning(&analysis_only_reasoning(scenario), true, Some(0))
                .await
            {
                return;
            }
            if state.tests_run == 0
                && !renderer
                    .run_tests(
                        scenario,
                        &mut state,
                        &focused_command,
                        true,
                        rng.random_range(3..9),
                    )
                    .await
            {
                return;
            }
            if plan.use_todo && !renderer.todo(scenario, TodoPhase::Done).await {
                return;
            }
            renderer.finish_no_changes(scenario, &state).await;
            return;
        }

        if renderer.mode == AgentMode::Plan {
            if !renderer
                .reasoning(&plan_reasoning(scenario), true, Some(0))
                .await
            {
                return;
            }
            if !renderer.switch_mode(AgentMode::Build).await {
                return;
            }
        } else if !renderer
            .reasoning(&diagnosis_reasoning(scenario), false, Some(0))
            .await
        {
            return;
        }

        if plan.use_todo && !renderer.todo(scenario, TodoPhase::Editing).await {
            return;
        }

        if !renderer.edit(scenario.files[0], &mut state).await {
            return;
        }

        if rng.random_bool(0.30)
            && !renderer
                .diagnostics(scenario.files[0], false, &mut state)
                .await
        {
            return;
        }

        let total_tests: u32 = rng.random_range(7..24);
        let first_passes = !rng.random_bool(complexity.first_failure_probability());
        if !renderer
            .run_tests(
                scenario,
                &mut state,
                &focused_command,
                first_passes,
                total_tests,
            )
            .await
        {
            return;
        }

        if !first_passes {
            if !renderer
                .reasoning(&retry_reasoning(scenario), true, Some(1))
                .await
            {
                return;
            }
            let retry_file = scenario.files.get(1).copied().unwrap_or(scenario.files[0]);
            if !renderer.read(scenario, retry_file).await {
                return;
            }
            if rng.random_bool(0.36) {
                let retry_term = scenario
                    .retry_diagnosis
                    .split_whitespace()
                    .take(3)
                    .collect::<Vec<_>>()
                    .join(" ");
                if !renderer.search_term(scenario, &retry_term).await {
                    return;
                }
            }
            if !renderer.edit(retry_file, &mut state).await {
                return;
            }

            let second_fails = rng.random_bool(0.11);
            if !renderer
                .run_tests(
                    scenario,
                    &mut state,
                    &focused_command,
                    !second_fails,
                    total_tests,
                )
                .await
            {
                return;
            }

            if second_fails {
                let final_reasoning = vec![
                    "The remaining failure is now limited to the boundary assertion rather than the original control-flow bug.".to_string(),
                    format!(
                        "I'll re-read `{}` and make the smallest compatibility adjustment before one final focused run.",
                        scenario.files.last().copied().unwrap_or(scenario.files[0])
                    ),
                ];
                if !renderer.reasoning(&final_reasoning, true, Some(0)).await {
                    return;
                }
                if !renderer
                    .read(
                        scenario,
                        scenario.files.last().copied().unwrap_or(scenario.files[0]),
                    )
                    .await
                {
                    return;
                }
                if !renderer.edit(scenario.files[0], &mut state).await {
                    return;
                }
                if !renderer
                    .run_tests(scenario, &mut state, &focused_command, true, total_tests)
                    .await
                {
                    return;
                }
            }
        }

        if plan.use_todo && !renderer.todo(scenario, TodoPhase::Verifying).await {
            return;
        }

        // A focused regression passing is not always enough. More involved
        // sessions naturally widen verification to the package or workspace.
        if plan.verification.needs_package() {
            if !renderer
                .reasoning(&broader_test_reasoning(TestScope::Package), false, None)
                .await
            {
                return;
            }
            let package_command = test_command(scenario, TestScope::Package);
            let broad_failure_probability: f64 = match complexity {
                Complexity::Small => 0.02,
                Complexity::Normal => 0.04,
                Complexity::Complex => 0.08,
                Complexity::Deep => 0.12,
            };
            let package_passes = !rng.random_bool(broad_failure_probability);
            if !renderer
                .run_tests(
                    scenario,
                    &mut state,
                    &package_command,
                    package_passes,
                    rng.random_range(14..48),
                )
                .await
            {
                return;
            }

            if !package_passes {
                if !renderer
                    .reasoning(&broad_failure_reasoning(scenario), true, Some(0))
                    .await
                {
                    return;
                }
                let compatibility_file =
                    scenario.files.get(1).copied().unwrap_or(scenario.files[0]);
                if !renderer.read(scenario, compatibility_file).await {
                    return;
                }
                if !renderer.edit(compatibility_file, &mut state).await {
                    return;
                }
                if !renderer
                    .run_tests(
                        scenario,
                        &mut state,
                        &package_command,
                        true,
                        rng.random_range(14..48),
                    )
                    .await
                {
                    return;
                }
            }
        }

        if plan.verification.needs_full() {
            if !renderer
                .reasoning(&broader_test_reasoning(TestScope::Full), false, None)
                .await
            {
                return;
            }
            let full_command = test_command(scenario, TestScope::Full);
            if !renderer
                .run_tests(
                    scenario,
                    &mut state,
                    &full_command,
                    true,
                    rng.random_range(35..96),
                )
                .await
            {
                return;
            }
        }

        if (state.used_lsp || rng.random_bool(0.36))
            && !renderer
                .diagnostics(scenario.files[0], true, &mut state)
                .await
        {
            return;
        }

        if plan.review_diff {
            if plan.use_todo && !renderer.todo(scenario, TodoPhase::Reviewing).await {
                return;
            }
            if !renderer.review_diff(scenario, &mut state).await {
                return;
            }
            if !renderer
                .reasoning(
                    &diff_review_reasoning(scenario, plan.refine_patch),
                    plan.refine_patch,
                    Some(0),
                )
                .await
            {
                return;
            }
            if plan.refine_patch {
                if !renderer.refine_patch(scenario, &mut state).await {
                    return;
                }
                // After narrowing the diff, do one cheap focused verification.
                if !renderer
                    .run_tests(
                        scenario,
                        &mut state,
                        &focused_command,
                        true,
                        rng.random_range(5..18),
                    )
                    .await
                {
                    return;
                }
            }
        } else if rng.random_bool(0.48) && !renderer.git_diff(&state).await {
            return;
        }

        if plan.use_todo && !renderer.todo(scenario, TodoPhase::Done).await {
            return;
        }

        renderer.finish(scenario, &state).await;
        }).await;

        show_cursor().await;
    }
}
