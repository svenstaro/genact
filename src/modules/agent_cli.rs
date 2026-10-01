//! Pretend to run a streaming AI coding agent CLI in the terminal.
use async_trait::async_trait;
use instant::Instant;
use rand::seq::IndexedRandom;
use rand::{RngExt, rng};
use yansi::Paint;

use crate::args::AppConfig;
use crate::io::{csleep, erase_line, hide_cursor, newline, print, show_cursor};
use crate::modules::Module;

pub struct AgentCli;

#[derive(Clone, Copy)]
enum TestStyle {
    Rust,
    Pytest,
    Node,
    Go,
    Ctest,
    Shell,
}

struct ScenarioWorld {
    task: &'static str,
    files: &'static [&'static str],
    search_term: &'static str,
    test_command: &'static str,
    test_style: TestStyle,
    failing_test: &'static str,
    snippet: &'static str,
    diagnosis: &'static str,
    retry_diagnosis: &'static str,
    summary: &'static str,
}

const SCENARIOS: &[ScenarioWorld] = &[
    ScenarioWorld {
        task: "Fix the flaky websocket reconnect test",
        files: &[
            "src/net/client.rs",
            "src/net/retry.rs",
            "tests/reconnect.rs",
        ],
        search_term: "retry_count",
        test_command: "cargo test reconnect",
        test_style: TestStyle::Rust,
        failing_test: "websocket::reconnect_preserves_backoff",
        snippet: "self.retry_count = 0;",
        diagnosis: "The retry counter is reset before the transient failure is classified.",
        retry_diagnosis: "The timeout branch still bypasses the shared retry transition.",
        summary: "Reconnect handling now preserves retry state across transient failures.",
    },
    ScenarioWorld {
        task: "Make environment variables override values from the config file",
        files: &["src/config.rs", "src/args.rs", "tests/config.rs"],
        search_term: "merge_config",
        test_command: "cargo test config",
        test_style: TestStyle::Rust,
        failing_test: "config::environment_has_highest_precedence",
        snippet: "config.merge(file_values);",
        diagnosis: "The merge order applies file values after the environment layer.",
        retry_diagnosis: "The optional CLI layer also needs to preserve the same precedence order.",
        summary: "Configuration precedence is now defaults < file < environment < CLI.",
    },
    ScenarioWorld {
        task: "Handle UTF-8 boundaries correctly in the streaming parser",
        files: &["src/parser.rs", "src/buffer.rs", "tests/parser_unicode.rs"],
        search_term: "split_at",
        test_command: "cargo test parser_unicode",
        test_style: TestStyle::Rust,
        failing_test: "parser_unicode::split_multibyte_character",
        snippet: "let (head, tail) = input.split_at(limit);",
        diagnosis: "The chunk limit is measured in bytes but can land inside a multibyte character.",
        retry_diagnosis: "The carry buffer must also keep incomplete bytes between reads.",
        summary: "Streaming parsing now keeps chunk boundaries on valid UTF-8 offsets.",
    },
    ScenarioWorld {
        task: "Prevent duplicate jobs when the scheduler wakes up concurrently",
        files: &["src/scheduler.rs", "src/job.rs", "tests/scheduler.rs"],
        search_term: "pending_jobs",
        test_command: "cargo test scheduler",
        test_style: TestStyle::Rust,
        failing_test: "scheduler::does_not_dispatch_job_twice",
        snippet: "if self.pending_jobs.contains(&job.id) {",
        diagnosis: "The membership check and insertion happen in separate critical sections.",
        retry_diagnosis: "The retry path performs the same check outside the guarded state.",
        summary: "Job reservation is now atomic across normal dispatch and retry paths.",
    },
    ScenarioWorld {
        task: "Add exponential backoff to the async API client",
        files: &["client/http.py", "client/retry.py", "tests/test_retry.py"],
        search_term: "asyncio.sleep",
        test_command: "pytest -q tests/test_retry.py",
        test_style: TestStyle::Pytest,
        failing_test: "test_retry_uses_exponential_backoff",
        snippet: "await asyncio.sleep(self.retry_delay)",
        diagnosis: "Every retry currently sleeps for the same fixed interval.",
        retry_diagnosis: "The maximum delay cap is applied before jitter instead of after it.",
        summary: "Transient API failures now use bounded exponential backoff with jitter.",
    },
    ScenarioWorld {
        task: "Preserve Decimal values when serializing API responses",
        files: &["app/json_encoder.py", "app/api.py", "tests/test_json.py"],
        search_term: "json.dumps",
        test_command: "pytest -q tests/test_json.py",
        test_style: TestStyle::Pytest,
        failing_test: "test_decimal_is_serialized_without_float_rounding",
        snippet: "return float(value)",
        diagnosis: "Converting Decimal to float introduces rounding before serialization.",
        retry_diagnosis: "Nested response objects still use the default encoder recursively.",
        summary: "Decimal response values are now serialized without an intermediate float conversion.",
    },
    ScenarioWorld {
        task: "Fix stale cache entries surviving past their TTL",
        files: &["app/cache.py", "app/clock.py", "tests/test_cache.py"],
        search_term: "expires_at",
        test_command: "pytest -q tests/test_cache.py",
        test_style: TestStyle::Pytest,
        failing_test: "test_expired_entry_is_removed_on_read",
        snippet: "if entry.expires_at < time.time():",
        diagnosis: "The boundary check treats an entry expiring exactly now as still valid.",
        retry_diagnosis: "The bulk lookup path has its own expiration check with the old comparison.",
        summary: "Cache reads now consistently evict entries at and beyond their TTL boundary.",
    },
    ScenarioWorld {
        task: "Normalize Windows paths before comparing workspace roots",
        files: &[
            "tools/paths.py",
            "tools/workspace.py",
            "tests/test_paths.py",
        ],
        search_term: "normpath",
        test_command: "pytest -q tests/test_paths.py",
        test_style: TestStyle::Pytest,
        failing_test: "test_windows_drive_letter_case_is_ignored",
        snippet: "return os.path.normpath(path)",
        diagnosis: "Path normalization does not normalize drive-letter case on Windows.",
        retry_diagnosis: "The containment check still compares an unresolved relative path.",
        summary: "Workspace path comparisons now normalize drive case and resolve relative components.",
    },
    ScenarioWorld {
        task: "Avoid sending two refresh-token requests at the same time",
        files: &[
            "src/auth/client.ts",
            "src/auth/session.ts",
            "test/auth.test.ts",
        ],
        search_term: "refreshPromise",
        test_command: "npm test -- --runInBand auth",
        test_style: TestStyle::Node,
        failing_test: "shares one in-flight refresh request",
        snippet: "const token = await refreshToken();",
        diagnosis: "Each failed request starts a new refresh operation instead of sharing one promise.",
        retry_diagnosis: "The shared promise is not cleared after a rejected refresh attempt.",
        summary: "Concurrent authentication failures now share one refresh request and clean up correctly.",
    },
    ScenarioWorld {
        task: "Fix the search box debounce race",
        files: &[
            "src/search/controller.ts",
            "src/search/api.ts",
            "test/search.test.ts",
        ],
        search_term: "setTimeout",
        test_command: "npm test -- search",
        test_style: TestStyle::Node,
        failing_test: "ignores responses from superseded searches",
        snippet: "timer = setTimeout(() => runSearch(query), delay);",
        diagnosis: "A slower response from an older query can overwrite the latest result.",
        retry_diagnosis: "The empty-query branch returns without invalidating the current request id.",
        summary: "Debounced search now ignores stale responses and cancels obsolete state consistently.",
    },
    ScenarioWorld {
        task: "Return field-level errors from request validation",
        files: &[
            "src/http/validate.ts",
            "src/http/routes.ts",
            "test/validate.test.ts",
        ],
        search_term: "ValidationError",
        test_command: "npm test -- validate",
        test_style: TestStyle::Node,
        failing_test: "returns all invalid field paths",
        snippet: "throw new ValidationError(errors[0]);",
        diagnosis: "Validation throws only the first error and discards the remaining field paths.",
        retry_diagnosis: "Nested array fields need their indices preserved in the response path.",
        summary: "Validation responses now include all field-level failures with stable nested paths.",
    },
    ScenarioWorld {
        task: "Stop fetching one extra page after the API reports the final cursor",
        files: &[
            "src/api/pager.ts",
            "src/api/client.ts",
            "test/pager.test.ts",
        ],
        search_term: "nextCursor",
        test_command: "npm test -- pager",
        test_style: TestStyle::Node,
        failing_test: "does_not_request_after_final_cursor",
        snippet: "while (cursor !== undefined) {",
        diagnosis: "A null final cursor is treated differently from an absent cursor and loops once more.",
        retry_diagnosis: "The async iterator wrapper duplicates the old cursor termination condition.",
        summary: "Pagination now stops on both null and absent final cursors without an extra request.",
    },
    ScenarioWorld {
        task: "Propagate context cancellation through the HTTP worker",
        files: &[
            "internal/http/worker.go",
            "internal/http/client.go",
            "internal/http/worker_test.go",
        ],
        search_term: "context.Background",
        test_command: "go test ./internal/http/...",
        test_style: TestStyle::Go,
        failing_test: "TestWorkerStopsOnContextCancel",
        snippet: "req = req.WithContext(context.Background())",
        diagnosis: "The request replaces the caller context with a new background context.",
        retry_diagnosis: "The retry helper creates a fresh request without copying the original context.",
        summary: "HTTP work now preserves caller cancellation through initial and retried requests.",
    },
    ScenarioWorld {
        task: "Fix a goroutine leak in the worker pool shutdown path",
        files: &[
            "internal/pool/pool.go",
            "internal/pool/worker.go",
            "internal/pool/pool_test.go",
        ],
        search_term: "close(p.jobs)",
        test_command: "go test ./internal/pool -run Shutdown",
        test_style: TestStyle::Go,
        failing_test: "TestShutdownWaitsForWorkers",
        snippet: "close(p.jobs)",
        diagnosis: "Workers can block while sending results after the receiver has already stopped.",
        retry_diagnosis: "The error-result channel needs the same shutdown select as successful results.",
        summary: "Worker shutdown now drains in-flight sends without leaving goroutines blocked.",
    },
    ScenarioWorld {
        task: "Close response bodies on every retry path",
        files: &[
            "internal/client/client.go",
            "internal/client/retry.go",
            "internal/client/client_test.go",
        ],
        search_term: "resp.Body",
        test_command: "go test ./internal/client/...",
        test_style: TestStyle::Go,
        failing_test: "TestRetryClosesPreviousResponseBody",
        snippet: "if shouldRetry(resp.StatusCode) {",
        diagnosis: "The retry branch continues before closing the previous response body.",
        retry_diagnosis: "The redirect retry path has a separate early-continue with the same leak.",
        summary: "Every retry path now closes the previous response body before issuing another request.",
    },
    ScenarioWorld {
        task: "Reject truncated packets before reading the payload header",
        files: &[
            "src/protocol/parser.cpp",
            "src/protocol/packet.hpp",
            "tests/parser_test.cpp",
        ],
        search_term: "payload_length",
        test_command: "ctest --test-dir build -R parser",
        test_style: TestStyle::Ctest,
        failing_test: "Parser.TruncatedPayloadHeader",
        snippet: "auto len = buffer[offset + 3];",
        diagnosis: "The parser reads the payload-length byte before verifying the header is complete.",
        retry_diagnosis: "The extended-header branch needs a second bound check before reading its length.",
        summary: "Packet parsing now validates header bounds before every payload-length read.",
    },
    ScenarioWorld {
        task: "Fix include-directory propagation for the static library target",
        files: &[
            "CMakeLists.txt",
            "src/CMakeLists.txt",
            "tests/CMakeLists.txt",
        ],
        search_term: "target_include_directories",
        test_command: "cmake --build build && ctest --test-dir build",
        test_style: TestStyle::Ctest,
        failing_test: "headers_are_visible_to_consumers",
        snippet: "target_include_directories(core PRIVATE include)",
        diagnosis: "Public headers are exposed with PRIVATE visibility, so consumers cannot include them.",
        retry_diagnosis: "The generated-header directory must also be exported on the build interface.",
        summary: "The static library now exports both source and generated public include directories.",
    },
    ScenarioWorld {
        task: "Fix the ring buffer wraparound when the write fills the final slot",
        files: &[
            "src/ring_buffer.cpp",
            "include/ring_buffer.hpp",
            "tests/ring_buffer_test.cpp",
        ],
        search_term: "write_index_",
        test_command: "ctest --test-dir build -R ring_buffer",
        test_style: TestStyle::Ctest,
        failing_test: "RingBuffer.WrapsAtExactCapacity",
        snippet: "write_index_ = write_index_ + written % capacity_;",
        diagnosis: "Operator precedence applies modulo only to the new write length, not the combined index.",
        retry_diagnosis: "The bulk-write branch duplicates the same wraparound expression.",
        summary: "Ring-buffer write indices now wrap correctly at exact and oversized writes.",
    },
    ScenarioWorld {
        task: "Make file replacement atomic when saving the local state",
        files: &[
            "scripts/save_state.sh",
            "scripts/common.sh",
            "tests/state_save.sh",
        ],
        search_term: "mv ",
        test_command: "bash tests/state_save.sh",
        test_style: TestStyle::Shell,
        failing_test: "interrupted_write_keeps_previous_state",
        snippet: "cat \"$tmp\" > \"$state_file\"",
        diagnosis: "The final state file is overwritten in place, so interruption can leave it truncated.",
        retry_diagnosis: "The temporary file is created on a different filesystem in the fallback path.",
        summary: "State saving now writes a sibling temporary file and atomically replaces the destination.",
    },
    ScenarioWorld {
        task: "Redact API keys from debug logging",
        files: &[
            "scripts/log_request.sh",
            "scripts/redact.sh",
            "tests/redaction.sh",
        ],
        search_term: "Authorization",
        test_command: "bash tests/redaction.sh",
        test_style: TestStyle::Shell,
        failing_test: "query_string_api_key_is_redacted",
        snippet: "printf '%s\\n' \"$request\"",
        diagnosis: "Header redaction works, but credentials embedded in query parameters are still logged.",
        retry_diagnosis: "Percent-encoded query parameter names need to be normalized before matching.",
        summary: "Debug logs now redact credentials from headers and query parameters.",
    },
];

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
            0..=24 => Self::Small,
            25..=74 => Self::Normal,
            75..=94 => Self::Complex,
            _ => Self::Deep,
        }
    }

    fn read_count(self) -> usize {
        match self {
            Self::Small => 1,
            Self::Normal => 2,
            Self::Complex => 2,
            Self::Deep => 3,
        }
    }

    fn web_search_probability(self) -> f64 {
        match self {
            Self::Small => 0.03,
            Self::Normal => 0.12,
            Self::Complex => 0.32,
            Self::Deep => 0.58,
        }
    }

    fn wrong_turn_probability(self) -> f64 {
        match self {
            Self::Small => 0.02,
            Self::Normal => 0.07,
            Self::Complex => 0.12,
            Self::Deep => 0.16,
        }
    }

    fn first_failure_probability(self) -> f64 {
        match self {
            Self::Small => 0.24,
            Self::Normal => 0.38,
            Self::Complex => 0.48,
            Self::Deep => 0.56,
        }
    }
}

#[derive(Clone, Copy)]
enum DelayKind {
    InitialThink,
    Think,
    DeepThink,
    Explore,
    Search,
    Read,
    WebSearch,
    Edit,
    Test,
    Git,
    ToolStart,
    AfterThinking,
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
        DelayKind::InitialThink => tiered_delay((2_500, 5_500), (5_500, 8_500), (8_500, 14_000)),
        DelayKind::Think => tiered_delay((1_400, 3_800), (3_800, 6_500), (6_500, 10_500)),
        DelayKind::DeepThink => tiered_delay((3_500, 6_500), (6_500, 10_000), (10_000, 15_500)),
        DelayKind::Explore => tiered_delay((1_400, 3_200), (3_200, 5_200), (5_200, 7_800)),
        DelayKind::Search => tiered_delay((1_800, 4_200), (4_200, 7_000), (7_000, 10_500)),
        DelayKind::Read => tiered_delay((1_500, 3_600), (3_600, 5_800), (5_800, 8_500)),
        DelayKind::WebSearch => tiered_delay((3_800, 7_000), (7_000, 10_500), (10_500, 15_000)),
        DelayKind::Edit => tiered_delay((2_100, 4_800), (4_800, 7_600), (7_600, 11_000)),
        DelayKind::Test => tiered_delay((6_500, 12_000), (12_000, 20_000), (20_000, 30_000)),
        DelayKind::Git => tiered_delay((900, 2_000), (2_000, 3_400), (3_400, 5_000)),
        DelayKind::ToolStart => tiered_delay((350, 900), (900, 1_600), (1_600, 2_600)),
        DelayKind::AfterThinking => tiered_delay((1_000, 2_200), (2_200, 3_500), (3_500, 5_200)),
        DelayKind::BetweenTasks => tiered_delay((1_400, 2_600), (2_600, 4_000), (4_000, 5_800)),
    }
}

#[derive(Clone, Copy)]
enum Tone {
    Explore,
    Search,
    Read,
    Edit,
    Run,
    Success,
    Failure,
}

struct AgentState {
    seen_files: Vec<&'static str>,
    modified_files: Vec<&'static str>,
    searches: Vec<String>,
    additions: u32,
    deletions: u32,
    tests_run: u32,
    test_failures: u32,
    used_web_search: bool,
    wrong_turns: u32,
}

impl AgentState {
    fn new() -> Self {
        Self {
            seen_files: Vec::new(),
            modified_files: Vec::new(),
            searches: Vec::new(),
            additions: 0,
            deletions: 0,
            tests_run: 0,
            test_failures: 0,
            used_web_search: false,
            wrong_turns: 0,
        }
    }

    fn saw(&mut self, file: &'static str) {
        if !self.seen_files.contains(&file) {
            self.seen_files.push(file);
        }
    }

    fn modified(&mut self, file: &'static str, additions: u32, deletions: u32) {
        if !self.modified_files.contains(&file) {
            self.modified_files.push(file);
        }
        self.additions += additions;
        self.deletions += deletions;
    }

    fn reverted(&mut self, file: &'static str) {
        self.modified_files.retain(|candidate| *candidate != file);
        // Wrong-turn edits are intentionally small. We do not preserve their exact diff
        // counts; trimming a few lines keeps the final summary consistent with a revert.
        self.additions = self.additions.saturating_sub(8);
        self.deletions = self.deletions.saturating_sub(4);
    }
}

struct Renderer<'a> {
    appconfig: &'a AppConfig,
}

impl<'a> Renderer<'a> {
    fn new(appconfig: &'a AppConfig) -> Self {
        Self { appconfig }
    }

    fn dim<S: Into<String>>(value: S) -> String {
        Paint::new(value.into()).dim().to_string()
    }

    async fn wait_ms(&self, millis: u64) -> bool {
        csleep(millis).await;
        !self.appconfig.should_exit()
    }

    async fn wait(&self, kind: DelayKind) -> bool {
        self.wait_ms(sample_delay(kind)).await
    }

    async fn header(&self, task: &str) {
        print(format!("{}", Paint::cyan("AI Agent").bold())).await;
        newline().await;
        print(format!("> {task}")).await;
        newline().await;
        newline().await;
    }

    async fn tool_line(&self, kind: &str, detail: &str, tone: Tone) {
        let label = match tone {
            Tone::Explore => Paint::cyan(kind).bold().to_string(),
            Tone::Search => Paint::cyan(kind).bold().to_string(),
            Tone::Read => Paint::blue(kind).bold().to_string(),
            Tone::Edit => Paint::yellow(kind).bold().to_string(),
            Tone::Run => Paint::magenta(kind).bold().to_string(),
            Tone::Success => Paint::green(kind).bold().to_string(),
            Tone::Failure => Paint::red(kind).bold().to_string(),
        };
        let bullet = match tone {
            Tone::Success => Paint::green("●").bold().to_string(),
            Tone::Failure => Paint::red("●").bold().to_string(),
            _ => Paint::cyan("●").bold().to_string(),
        };

        if detail.is_empty() {
            print(format!("{bullet} {label}")).await;
        } else {
            print(format!("{bullet} {label}  {}", Self::dim(detail))).await;
        }
        newline().await;
    }

    async fn tool_start_pause(&self) -> bool {
        self.wait(DelayKind::ToolStart).await
    }

    async fn after_thinking_pause(&self) -> bool {
        self.wait(DelayKind::AfterThinking).await
    }

    async fn activity_spinner(&self, label: &str, kind: DelayKind) -> bool {
        const FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let total = sample_delay(kind);
        let mut elapsed = 0u64;
        let mut frame = 0usize;
        let mut rng = rng();

        while elapsed < total {
            erase_line().await;
            print(Self::dim(format!(
                "  {} {label}",
                FRAMES[frame % FRAMES.len()]
            )))
            .await;
            let step = rng.random_range(180..360).min(total - elapsed);
            csleep(step).await;
            elapsed += step;
            frame += 1;

            if self.appconfig.should_exit() {
                erase_line().await;
                return false;
            }
        }

        erase_line().await;
        true
    }

    async fn thinking(&self, lines: &[String], deep: bool) -> bool {
        const FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let total = sample_delay(if deep {
            DelayKind::DeepThink
        } else {
            DelayKind::Think
        });
        let mut elapsed = 0u64;
        let mut frame = 0usize;
        let mut rng = rng();

        while elapsed < total {
            erase_line().await;
            let seconds = elapsed.div_ceil(1000).max(1);
            print(Self::dim(format!(
                "{} Thinking… {seconds}s",
                FRAMES[frame % FRAMES.len()]
            )))
            .await;

            let step = rng.random_range(220..420).min(total - elapsed);
            csleep(step).await;
            elapsed += step;
            frame += 1;

            if self.appconfig.should_exit() {
                erase_line().await;
                return false;
            }
        }

        erase_line().await;
        print(Self::dim("✻ Thinking")).await;
        newline().await;
        newline().await;

        for paragraph in lines {
            for line in wrap_text(paragraph, 74) {
                print("  ").await;
                if !self.stream_dim_line(&line).await {
                    return false;
                }
                newline().await;
            }
            newline().await;
        }

        self.after_thinking_pause().await
    }

    async fn initial_thinking(&self, lines: &[String]) -> bool {
        const FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let total = sample_delay(DelayKind::InitialThink);
        let mut elapsed = 0u64;
        let mut frame = 0usize;
        let mut rng = rng();

        while elapsed < total {
            erase_line().await;
            let seconds = elapsed.div_ceil(1000).max(1);
            print(Self::dim(format!(
                "{} Thinking… {seconds}s",
                FRAMES[frame % FRAMES.len()]
            )))
            .await;
            let step = rng.random_range(220..420).min(total - elapsed);
            csleep(step).await;
            elapsed += step;
            frame += 1;
            if self.appconfig.should_exit() {
                erase_line().await;
                return false;
            }
        }

        erase_line().await;
        print(Self::dim("✻ Thinking")).await;
        newline().await;
        newline().await;

        for paragraph in lines {
            for line in wrap_text(paragraph, 74) {
                print("  ").await;
                if !self.stream_dim_line(&line).await {
                    return false;
                }
                newline().await;
            }
            newline().await;
        }
        self.after_thinking_pause().await
    }

    async fn stream_dim_line(&self, line: &str) -> bool {
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
            print(Self::dim(burst)).await;

            let last_word = words[end - 1];
            let pause =
                if last_word.ends_with('.') || last_word.ends_with('?') || last_word.ends_with('!')
                {
                    rng.random_range(150..430)
                } else if last_word.ends_with(',')
                    || last_word.ends_with(';')
                    || last_word.ends_with(':')
                {
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

    async fn explore(&self, scenario: &'static ScenarioWorld, state: &mut AgentState) -> bool {
        self.tool_line("Explore", "repository structure", Tone::Explore)
            .await;
        if !self.tool_start_pause().await {
            return false;
        }
        if !self
            .activity_spinner("Scanning workspace…", DelayKind::Explore)
            .await
        {
            return false;
        }

        let mut rng = rng();
        let count = rng.random_range(2..=scenario.files.len().min(4));
        for (idx, file) in scenario.files.iter().take(count).enumerate() {
            let branch = if idx + 1 == count { "└─" } else { "├─" };
            print(Self::dim(format!("  {branch} {file}"))).await;
            newline().await;
            state.saw(file);
            if !self.wait_ms(rng.random_range(220..650)).await {
                return false;
            }
        }
        newline().await;
        true
    }

    async fn search(&self, scenario: &'static ScenarioWorld, state: &mut AgentState) -> bool {
        let query = scenario.search_term.to_string();
        self.tool_line("Search", &format!("\"{query}\""), Tone::Search)
            .await;
        if !self.tool_start_pause().await {
            return false;
        }
        if !self
            .activity_spinner("Searching workspace…", DelayKind::Search)
            .await
        {
            return false;
        }

        let mut rng = rng();
        let match_files = rng.random_range(1..=scenario.files.len().min(3));
        let matches = rng.random_range(match_files..=(match_files * 4 + 2));
        for file in scenario.files.iter().take(match_files) {
            print(Self::dim(format!("  └ {file}"))).await;
            newline().await;
            state.saw(file);
            if !self.wait_ms(rng.random_range(220..650)).await {
                return false;
            }
        }
        print(Self::dim(format!(
            "  {matches} matches in {match_files} files"
        )))
        .await;
        newline().await;
        newline().await;
        state.searches.push(query);
        true
    }

    async fn read(
        &self,
        scenario: &'static ScenarioWorld,
        file: &'static str,
        state: &mut AgentState,
    ) -> bool {
        self.tool_line("Read", file, Tone::Read).await;
        if !self.tool_start_pause().await {
            return false;
        }
        if !self
            .activity_spinner("Reading file…", DelayKind::Read)
            .await
        {
            return false;
        }
        state.saw(file);

        let mut rng = rng();
        if !self.wait_ms(rng.random_range(350..1_050)).await {
            return false;
        }
        let roll = rng.random_range(0..100);
        if roll < 58 {
            print(Self::dim(format!("  … {} lines", rng.random_range(18..96)))).await;
            newline().await;
        } else {
            let line_no: u32 = rng.random_range(24..320);
            print(Self::dim(format!("  {:4} │ …", line_no.saturating_sub(1)))).await;
            newline().await;
            if !self.wait_ms(rng.random_range(180..520)).await {
                return false;
            }
            print(Self::dim(format!("  {line_no:4} │ {}", scenario.snippet))).await;
            newline().await;
            if !self.wait_ms(rng.random_range(260..760)).await {
                return false;
            }
            print(Self::dim(format!("  {:4} │ …", line_no + 1))).await;
            newline().await;
            if roll > 88 {
                print(Self::dim(format!(
                    "       … {} more lines",
                    rng.random_range(8..34)
                )))
                .await;
                newline().await;
            }
        }
        newline().await;
        true
    }

    async fn web_search(&self, scenario: &'static ScenarioWorld, state: &mut AgentState) -> bool {
        let query = web_query(scenario);
        self.tool_line("Web Search", &query, Tone::Search).await;
        if !self.tool_start_pause().await {
            return false;
        }
        if !self
            .activity_spinner("Searching the web…", DelayKind::WebSearch)
            .await
        {
            return false;
        }

        let results = web_results(scenario.test_style);
        let mut rng = rng();
        let count = rng.random_range(2..=results.len().min(4));
        for title in results.iter().take(count) {
            print(Self::dim(format!("  └ {title}"))).await;
            newline().await;
            if !self.wait_ms(rng.random_range(280..820)).await {
                return false;
            }
        }
        print(Self::dim(format!("  Found {count} relevant results"))).await;
        newline().await;
        newline().await;
        state.used_web_search = true;
        true
    }

    async fn edit(&self, file: &'static str, state: &mut AgentState) -> bool {
        self.tool_line("Edit", file, Tone::Edit).await;
        if !self.tool_start_pause().await {
            return false;
        }
        if !self
            .activity_spinner("Applying changes…", DelayKind::Edit)
            .await
        {
            return false;
        }

        let mut rng = rng();
        let additions = rng.random_range(3..19);
        let deletions = rng.random_range(1..9);
        state.modified(file, additions, deletions);
        print(format!(
            "  {}  {}",
            Paint::new(format!("+{additions}")).green().bold(),
            Paint::new(format!("-{deletions}")).red().bold()
        ))
        .await;
        newline().await;
        newline().await;
        true
    }

    async fn revert(&self, file: &'static str, state: &mut AgentState) -> bool {
        self.tool_line("Edit", file, Tone::Edit).await;
        print(Self::dim("  reverting previous change")).await;
        newline().await;
        if !self.tool_start_pause().await {
            return false;
        }
        if !self
            .activity_spinner("Restoring previous version…", DelayKind::Edit)
            .await
        {
            return false;
        }
        print(Self::dim("  reverted")).await;
        newline().await;
        newline().await;
        state.reverted(file);
        true
    }

    async fn run_tests(
        &self,
        scenario: &'static ScenarioWorld,
        state: &mut AgentState,
        passed: bool,
        total: u32,
    ) -> bool {
        self.tool_line("Run", scenario.test_command, Tone::Run)
            .await;
        if !self.tool_start_pause().await {
            return false;
        }
        state.tests_run += 1;
        if !passed {
            state.test_failures += 1;
        }

        if !self.render_test_startup(scenario.test_style).await {
            return false;
        }
        if !self
            .activity_spinner("Running tests…", DelayKind::Test)
            .await
        {
            return false;
        }
        if !self
            .render_test_result(scenario.test_style, passed, total, scenario.failing_test)
            .await
        {
            return false;
        }
        newline().await;
        true
    }

    async fn render_test_startup(&self, style: TestStyle) -> bool {
        let mut rng = rng();
        match style {
            TestStyle::Rust => {
                let crates = ["core", "client", "protocol", "integration-tests"];
                let count = rng.random_range(1..=3);
                for name in crates.iter().take(count) {
                    print(Self::dim(format!("   Compiling {name} v0.1.0"))).await;
                    newline().await;
                    if !self.wait_ms(rng.random_range(900..2_600)).await {
                        return false;
                    }
                }
                print(Self::dim(
                    "    Finished `test` profile [unoptimized + debuginfo] target(s)",
                ))
                .await;
                newline().await;
            }
            TestStyle::Pytest => {
                print(Self::dim("============================= test session starts ==============================")).await;
                newline().await;
                if !self.wait_ms(rng.random_range(800..2_000)).await {
                    return false;
                }
            }
            TestStyle::Node => {
                print(Self::dim("$ test --runInBand")).await;
                newline().await;
                if !self.wait_ms(rng.random_range(700..1_800)).await {
                    return false;
                }
            }
            TestStyle::Go => {
                print(Self::dim("go: building test binary…")).await;
                newline().await;
                if !self.wait_ms(rng.random_range(800..2_000)).await {
                    return false;
                }
            }
            TestStyle::Ctest => {
                print(Self::dim("[100%] Built target tests")).await;
                newline().await;
                if !self.wait_ms(rng.random_range(900..2_300)).await {
                    return false;
                }
            }
            TestStyle::Shell => {
                print(Self::dim("running integration checks…")).await;
                newline().await;
                if !self.wait_ms(rng.random_range(650..1_600)).await {
                    return false;
                }
            }
        }
        true
    }

    async fn render_test_result(
        &self,
        style: TestStyle,
        passed: bool,
        total: u32,
        failing_test: &str,
    ) -> bool {
        let mut rng = rng();

        match style {
            TestStyle::Rust => {
                print(Self::dim(format!("running {total} tests"))).await;
                newline().await;
                if !self.wait_ms(rng.random_range(650..1_500)).await {
                    return false;
                }
                if !self.fake_passing_test("initializes_state").await {
                    return false;
                }
                if !self.fake_passing_test("handles_common_path").await {
                    return false;
                }
                if passed {
                    print(format!(
                        "{} ... {}",
                        Self::dim(format!("test {failing_test}")),
                        Paint::green("ok").bold()
                    ))
                    .await;
                    newline().await;
                } else {
                    print(format!(
                        "{} ... {}",
                        Self::dim(format!("test {failing_test}")),
                        Paint::red("FAILED").bold()
                    ))
                    .await;
                    newline().await;
                }

                if !self.wait_ms(rng.random_range(900..2_200)).await {
                    return false;
                }

                if passed {
                    print(format!(
                        "test result: {}. {total} passed; 0 failed; 0 ignored",
                        Paint::green("ok").bold()
                    ))
                    .await;
                } else {
                    print(format!(
                        "test result: {}. {} passed; 1 failed; 0 ignored",
                        Paint::red("FAILED").bold(),
                        total.saturating_sub(1)
                    ))
                    .await;
                }
                newline().await;
            }
            TestStyle::Pytest => {
                print(Self::dim(format!("collected {total} items"))).await;
                newline().await;
                if !self.wait_ms(rng.random_range(700..1_600)).await {
                    return false;
                }

                if passed {
                    print(Self::dim(
                        "tests/ ........................................................",
                    ))
                    .await;
                    newline().await;
                    if !self.wait_ms(rng.random_range(1_100..2_500)).await {
                        return false;
                    }
                    print(format!(
                        "{} {total} passed",
                        Paint::green("================").bold()
                    ))
                    .await;
                } else {
                    print(format!(
                        "tests/ ........{}...............................................",
                        Paint::red("F").bold()
                    ))
                    .await;
                    newline().await;
                    if !self.wait_ms(rng.random_range(900..2_100)).await {
                        return false;
                    }
                    print(format!("{} {failing_test}", Paint::red("FAILED").bold())).await;
                    newline().await;
                    if !self.wait_ms(rng.random_range(650..1_500)).await {
                        return false;
                    }
                    print(format!("1 failed, {} passed", total.saturating_sub(1))).await;
                }
                newline().await;
            }
            TestStyle::Node => {
                if passed {
                    print(format!(
                        "{} {}",
                        Paint::green("PASS").bold(),
                        Self::dim(failing_test)
                    ))
                    .await;
                } else {
                    print(format!(
                        "{} {}",
                        Paint::red("FAIL").bold(),
                        Self::dim(failing_test)
                    ))
                    .await;
                }
                newline().await;

                if !self.wait_ms(rng.random_range(900..2_200)).await {
                    return false;
                }

                if passed {
                    print(format!("Tests: {total} passed, {total} total")).await;
                } else {
                    print(format!(
                        "Tests: 1 failed, {} passed, {total} total",
                        total.saturating_sub(1)
                    ))
                    .await;
                }
                newline().await;
            }
            TestStyle::Go => {
                if passed {
                    print(Self::dim(format!("=== RUN   {failing_test}"))).await;
                    newline().await;
                    if !self.wait_ms(rng.random_range(800..1_900)).await {
                        return false;
                    }
                    print(format!(
                        "{}\tproject/package\t1.{}s",
                        Paint::green("ok").bold(),
                        total
                    ))
                    .await;
                } else {
                    print(Self::dim(format!("=== RUN   {failing_test}"))).await;
                    newline().await;
                    if !self.wait_ms(rng.random_range(800..1_900)).await {
                        return false;
                    }
                    print(format!(
                        "--- {}: {failing_test} (0.02s)",
                        Paint::red("FAIL").bold()
                    ))
                    .await;
                    newline().await;
                    if !self.wait_ms(rng.random_range(650..1_500)).await {
                        return false;
                    }
                    print(format!("{}\tproject/package", Paint::red("FAIL").bold())).await;
                }
                newline().await;
            }
            TestStyle::Ctest => {
                print(Self::dim("Test project build")).await;
                newline().await;
                if !self.wait_ms(rng.random_range(700..1_700)).await {
                    return false;
                }

                print(Self::dim(format!("    Start 1: {failing_test}"))).await;
                newline().await;
                if !self.wait_ms(rng.random_range(1_100..2_700)).await {
                    return false;
                }

                if passed {
                    print(format!("100% tests passed, 0 tests failed out of {total}")).await;
                } else {
                    print(format!("{} {failing_test}", Paint::red("FAILED:").bold())).await;
                    newline().await;
                    if !self.wait_ms(rng.random_range(650..1_500)).await {
                        return false;
                    }
                    print(format!(
                        "{}% tests passed, 1 test failed out of {total}",
                        (total.saturating_sub(1) * 100) / total.max(1)
                    ))
                    .await;
                }
                newline().await;
            }
            TestStyle::Shell => {
                print(Self::dim("checking command exit status…")).await;
                newline().await;
                if !self.wait_ms(rng.random_range(750..1_800)).await {
                    return false;
                }

                if passed {
                    print(format!(
                        "{} {total} checks passed",
                        Paint::green("[ok]").bold()
                    ))
                    .await;
                } else {
                    print(format!("{} {failing_test}", Paint::red("[failed]").bold())).await;
                    newline().await;
                    if !self.wait_ms(rng.random_range(650..1_500)).await {
                        return false;
                    }
                    print(format!("{} passed, 1 failed", total.saturating_sub(1))).await;
                }
                newline().await;
            }
        }

        true
    }

    async fn fake_passing_test(&self, name: &str) -> bool {
        let mut rng = rng();
        print(format!(
            "{} ... {}",
            Self::dim(format!("test {name}")),
            Paint::green("ok").bold()
        ))
        .await;
        newline().await;
        self.wait_ms(rng.random_range(450..1_250)).await
    }

    async fn git_check(&self) -> bool {
        self.tool_line("Git", "git diff --check", Tone::Run).await;
        if !self.tool_start_pause().await {
            return false;
        }
        if !self
            .activity_spinner("Checking working tree…", DelayKind::Git)
            .await
        {
            return false;
        }
        print(Self::dim("  no whitespace errors")).await;
        newline().await;
        newline().await;
        true
    }

    async fn done(&self, scenario: &'static ScenarioWorld, state: &AgentState, started: Instant) {
        self.tool_line("Done", scenario.summary, Tone::Success)
            .await;
        let files = state.modified_files.len();
        print(Self::dim(format!(
            "  {files} file{} changed  +{} -{}",
            if files == 1 { "" } else { "s" },
            state.additions,
            state.deletions
        )))
        .await;
        newline().await;

        let elapsed = started.elapsed();
        let seconds = elapsed.as_secs() as f32 + elapsed.subsec_nanos() as f32 / 1_000_000_000.0;
        let mut extras = format!(
            "{} test run{}",
            state.tests_run,
            if state.tests_run == 1 { "" } else { "s" }
        );
        if state.test_failures > 0 {
            extras.push_str(&format!(
                " · {} failed attempt{} recovered",
                state.test_failures,
                if state.test_failures == 1 { "" } else { "s" }
            ));
        }
        if state.wrong_turns > 0 {
            extras.push_str(" · reverted one speculative edit");
        }
        if state.used_web_search {
            extras.push_str(" · web search used");
        }
        print(Self::dim(format!(
            "  {extras} · completed in {seconds:.1}s"
        )))
        .await;
        newline().await;
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
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}

fn web_query(scenario: &ScenarioWorld) -> String {
    let prefix = match scenario.test_style {
        TestStyle::Rust => "rust async",
        TestStyle::Pytest => "python",
        TestStyle::Node => "typescript",
        TestStyle::Go => "go",
        TestStyle::Ctest => "c++",
        TestStyle::Shell => "posix shell",
    };
    format!("{prefix} {} behavior", scenario.search_term)
}

fn web_results(style: TestStyle) -> &'static [&'static str] {
    match style {
        TestStyle::Rust => &[
            "docs.rs — API reference",
            "Rust standard library documentation",
            "crate repository issue discussion",
            "Rust async patterns guide",
        ],
        TestStyle::Pytest => &[
            "Python documentation — library reference",
            "pytest documentation — fixtures and async behavior",
            "CPython issue tracker discussion",
            "package API reference",
        ],
        TestStyle::Node => &[
            "TypeScript handbook — async control flow",
            "Node.js API documentation",
            "library repository issue discussion",
            "MDN — Promise and event-loop behavior",
        ],
        TestStyle::Go => &[
            "Go package documentation",
            "Go blog — context and cancellation",
            "standard library source documentation",
            "Go issue tracker discussion",
        ],
        TestStyle::Ctest => &[
            "cppreference — language/library reference",
            "CMake documentation",
            "project issue discussion",
            "compiler documentation",
        ],
        TestStyle::Shell => &[
            "POSIX shell command language",
            "GNU coreutils documentation",
            "Bash reference manual",
            "shell portability notes",
        ],
    }
}

fn initial_reasoning(scenario: &ScenarioWorld, complexity: Complexity) -> Vec<String> {
    let mut lines = vec![
        format!(
            "I'll start by locating `{}` and tracing the surrounding control flow. I want to understand the path that produces the failure before changing anything.",
            scenario.search_term
        ),
        format!(
            "I'll compare the implementation with `{}` as well, because the test should tell me which behavior is actually part of the contract.",
            scenario
                .files
                .last()
                .copied()
                .unwrap_or("the relevant test")
        ),
    ];

    if matches!(complexity, Complexity::Complex | Complexity::Deep) {
        lines.push(
            "If the local control flow does not fully explain it, I'll check the runtime or library behavior involved in this path before committing to a fix."
                .to_string(),
        );
    }
    lines
}

fn tentative_reasoning(scenario: &ScenarioWorld) -> Vec<String> {
    vec![
        format!(
            "The helper around `{}` is the first suspicious point. It could be producing the bad state before the main path sees it.",
            scenario.search_term
        ),
        format!(
            "I'll make the smallest change in `{}` first and use the focused test to verify that hypothesis.",
            scenario.files.get(1).copied().unwrap_or(scenario.files[0])
        ),
    ]
}

fn diagnosis_reasoning(scenario: &ScenarioWorld) -> Vec<String> {
    vec![
        scenario.diagnosis.to_string(),
        format!(
            "That makes `{}` the better place to fix the behavior. Changing the helper in isolation would only hide the symptom for one path.",
            scenario.files[0]
        ),
        "I'll keep the patch narrow, then run the focused tests before looking at the rest of the suite.".to_string(),
    ]
}

fn wrong_turn_reasoning(scenario: &ScenarioWorld) -> Vec<String> {
    vec![
        "That change was at the wrong layer. The failing behavior is still present, which means the helper was only downstream of the actual state transition.".to_string(),
        scenario.diagnosis.to_string(),
        format!(
            "I'll revert the speculative change and move the fix into `{}` instead.",
            scenario.files[0]
        ),
    ]
}

fn retry_reasoning(scenario: &ScenarioWorld) -> Vec<String> {
    vec![
        "The first patch fixed the common path, but the focused test is still exercising a separate branch.".to_string(),
        scenario.retry_diagnosis.to_string(),
        format!(
            "I'll trace `{}` before making another change so the second patch addresses that branch directly rather than broadening the first edit.",
            scenario.files.get(1).copied().unwrap_or(scenario.files[0])
        ),
    ]
}

fn second_retry_reasoning(scenario: &ScenarioWorld) -> Vec<String> {
    vec![
        "The implementation is now behaving correctly on the main and fallback paths, so this remaining failure looks like an ordering or boundary detail rather than the original bug.".to_string(),
        format!(
            "I'll re-read `{}` and the assertion around `{}` to make sure the edge case matches the existing contract.",
            scenario.files.last().copied().unwrap_or(scenario.files[0]),
            scenario.failing_test
        ),
        "This should only need a small adjustment; I don't want to rewrite the working part of the fix.".to_string(),
    ]
}

#[async_trait(?Send)]
impl Module for AgentCli {
    fn name(&self) -> &'static str {
        "agent_cli"
    }

    fn signature(&self) -> String {
        "agent-cli".to_string()
    }

    async fn run(&self, appconfig: &AppConfig) {
        hide_cursor().await;

        (async {
        let mut rng = rng();
        let scenario = SCENARIOS
            .choose(&mut rng)
            .expect("agent_cli must contain at least one scenario");
        let complexity = Complexity::choose();
        let renderer = Renderer::new(appconfig);
        let mut state = AgentState::new();
        let started = Instant::now();

        renderer.header(scenario.task).await;
        if !renderer
            .initial_thinking(&initial_reasoning(scenario, complexity))
            .await
        {
            return;
        }

        if (complexity != Complexity::Small || rng.random_bool(0.55))
            && !renderer.explore(scenario, &mut state).await
        {
            return;
        }

        if !renderer.search(scenario, &mut state).await {
            return;
        }

        let read_count = complexity.read_count().min(scenario.files.len());
        for file in scenario.files.iter().take(read_count) {
            if !renderer.read(scenario, file, &mut state).await {
                return;
            }
        }

        let wrong_turn =
            scenario.files.len() > 1 && rng.random_bool(complexity.wrong_turn_probability());
        if wrong_turn {
            if !renderer
                .thinking(&tentative_reasoning(scenario), false)
                .await
            {
                return;
            }
            let wrong_file = scenario.files[1];
            if !renderer.edit(wrong_file, &mut state).await {
                return;
            }
            let total_tests = rng.random_range(7..24);
            if !renderer
                .run_tests(scenario, &mut state, false, total_tests)
                .await
            {
                return;
            }
            state.wrong_turns += 1;

            if !renderer
                .thinking(&wrong_turn_reasoning(scenario), true)
                .await
            {
                return;
            }
            if !renderer.revert(wrong_file, &mut state).await {
                return;
            }
        } else if !renderer
            .thinking(
                &diagnosis_reasoning(scenario),
                complexity == Complexity::Deep,
            )
            .await
        {
            return;
        }

        if rng.random_bool(complexity.web_search_probability()) {
            if !renderer.web_search(scenario, &mut state).await {
                return;
            }
            let web_thought = vec![
                "The external reference matches what the local code suggested. The library/runtime behavior does not add an extra guarantee here, so the fix should stay in the repository's own state handling."
                    .to_string(),
                format!(
                    "I'll proceed with the narrow change in `{}` and verify it against the existing test suite.",
                    scenario.files[0]
                ),
            ];
            if !renderer.thinking(&web_thought, false).await {
                return;
            }
        }

        if !renderer.edit(scenario.files[0], &mut state).await {
            return;
        }

        let total_tests = rng.random_range(7..24);
        let first_passes = !rng.random_bool(complexity.first_failure_probability());
        if !renderer
            .run_tests(scenario, &mut state, first_passes, total_tests)
            .await
        {
            return;
        }

        if !first_passes {
            renderer
                .tool_line("Failed", scenario.failing_test, Tone::Failure)
                .await;
            newline().await;

            if !renderer.thinking(&retry_reasoning(scenario), true).await {
                return;
            }

            let retry_file = scenario.files.get(1).copied().unwrap_or(scenario.files[0]);
            if !renderer.read(scenario, retry_file, &mut state).await {
                return;
            }
            if !renderer.edit(retry_file, &mut state).await {
                return;
            }

            let second_fails = rng.random_bool(0.12);
            if !renderer
                .run_tests(scenario, &mut state, !second_fails, total_tests)
                .await
            {
                return;
            }

            if second_fails {
                renderer
                    .tool_line("Failed", scenario.failing_test, Tone::Failure)
                    .await;
                newline().await;

                if !renderer
                    .thinking(&second_retry_reasoning(scenario), true)
                    .await
                {
                    return;
                }
                let test_file = scenario.files.last().copied().unwrap_or(scenario.files[0]);
                if !renderer.read(scenario, test_file, &mut state).await {
                    return;
                }
                if !renderer.edit(scenario.files[0], &mut state).await {
                    return;
                }
                if !renderer
                    .run_tests(scenario, &mut state, true, total_tests)
                    .await
                {
                    return;
                }
            }
        }

        if rng.random_bool(0.48) && !renderer.git_check().await {
            return;
        }

        renderer.done(scenario, &state, started).await;
        newline().await;
        let _ = renderer.wait(DelayKind::BetweenTasks).await;
        }).await;

        show_cursor().await;
    }
}
