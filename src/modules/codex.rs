//! Pretend to run a Codex coding session
use async_trait::async_trait;
use rand::rngs::ThreadRng;
use rand::seq::IndexedRandom;
use rand::{RngExt, rng};
use yansi::Paint;

use crate::args::AppConfig;
use crate::generators::gen_hex_string;
use crate::io::{csleep, dprint, erase_line, newline, print};
use crate::modules::Module;

pub struct Codex;

struct ToolGroup {
    title: &'static str,
    status: &'static str,
    steps: &'static [&'static str],
}

const PROMPTS: &[&str] = &[
    "add retries to the file watcher and update tests",
    "trace the login regression and make the smallest fix",
    "wire the new config option through the CLI",
    "clean up the flaky snapshot and explain the behavior",
    "add a terminal animation module to genact",
    "review the checkout flow and patch the race condition",
    "add snapshot coverage for the compact status footer",
    "make the config parser reject ambiguous profile names",
    "find why the stream stalls after tool output and fix it",
    "port the new approval prompt into the terminal UI",
];

const INTROS: &[&str] = &[
    "I’ll inspect the relevant code paths first, then make a focused change.",
    "I’m going to read the surrounding implementation before editing.",
    "I’ll reproduce the shape of the failure, patch it, and run the targeted checks.",
    "I’m checking the existing patterns so the change fits the project.",
    "I’ll start from the failing surface, then follow the data flow into the shared code.",
    "I’m going to keep the patch small and verify the behavior at the boundary.",
    "I’ll compare the snapshots with the renderer before touching the tests.",
];

const PLAN_STEPS: &[&str] = &[
    "Map the request to the owning module",
    "Read the nearby implementation and tests",
    "Patch the narrow behavior",
    "Run formatter and targeted checks",
    "Review the diff for accidental churn",
];

const THINKING_NOTES: &[&str] = &[
    "The likely boundary is the module registry plus a single rendering file.",
    "I should avoid touching shared output helpers unless the local module cannot express this.",
    "The safest verification path is CLI parsing, a fast compile check, and one real module run.",
    "This looks like a presentation-layer change; no data model migration should be needed.",
    "If the animation uses cursor rewrites, exit checks need to happen inside the tick loop.",
    "The user-facing surface is the terminal transcript, so the sample output matters here.",
    "A narrow diff is better than extracting a shared agent animation framework right now.",
    "The README list and clap possible values need to agree after registration.",
    "Repeated tool groups would look artificial, so this run should sample each phase once.",
    "Color should create hierarchy without making the transcript unreadable on dark themes.",
];

const TOOL_GROUPS: &[ToolGroup] = &[
    ToolGroup {
        title: "Explored",
        status: "Reading project context",
        steps: &[
            "List src",
            "Read src/main.rs",
            "Read src/modules/mod.rs",
            "Search \"impl Module\"",
        ],
    },
    ToolGroup {
        title: "Explored",
        status: "Tracing the relevant flow",
        steps: &["Search \"AppConfig\"", "Read src/args.rs", "Read src/io.rs"],
    },
    ToolGroup {
        title: "Explored",
        status: "Finding the owning renderer",
        steps: &[
            "Search \"StatusIndicatorWidget\"",
            "Read src/chatwidget/status.rs",
            "Read src/bottom_pane/mod.rs",
            "Find snapshot fixture names",
        ],
    },
    ToolGroup {
        title: "Explored",
        status: "Checking existing tests",
        steps: &[
            "List tests/status_and_layout.rs",
            "Search \"assert_snapshot\"",
            "Read status fixture helpers",
            "Compare old vt100 output",
        ],
    },
    ToolGroup {
        title: "Updated Plan",
        status: "Narrowing the implementation plan",
        steps: &[
            "completed: identify render surface",
            "in_progress: update terminal transcript output",
            "pending: run focused cargo checks",
        ],
    },
    ToolGroup {
        title: "Edited src/modules/codex.rs",
        status: "Applying focused edits",
        steps: &[
            "Add Codex transcript renderer",
            "Add animated status row",
            "Add randomized tool-call groups",
        ],
    },
    ToolGroup {
        title: "Edited src/modules/mod.rs",
        status: "Registering the new module",
        steps: &["Add codex module export", "Insert codex into ALL_MODULES"],
    },
    ToolGroup {
        title: "Edited tests/status_and_layout.rs",
        status: "Updating expected terminal snapshots",
        steps: &[
            "Add active status case",
            "Refresh compact footer fixture",
            "Preserve narrow terminal coverage",
        ],
    },
    ToolGroup {
        title: "Patched",
        status: "Applying repository patch",
        steps: &[
            "Update parser branch for missing value",
            "Keep existing error wording stable",
            "Add regression fixture",
        ],
    },
    ToolGroup {
        title: "Ran rg \"TODO|unwrap\"",
        status: "Scanning for obvious follow-ups",
        steps: &[
            "No new TODO markers",
            "Existing unwraps are in test helpers",
            "No unrelated files touched",
        ],
    },
    ToolGroup {
        title: "Ran cargo fmt",
        status: "Formatting Rust sources",
        steps: &["formatted src/modules/codex.rs"],
    },
    ToolGroup {
        title: "Ran cargo check",
        status: "Checking the crate",
        steps: &["Finished `dev` profile target(s)", "0 warnings"],
    },
    ToolGroup {
        title: "Ran cargo test -p codex-tui status_and_layout",
        status: "Running focused regression tests",
        steps: &[
            "status footer snapshot passed",
            "approval modal snapshot passed",
            "queued message layout passed",
        ],
    },
    ToolGroup {
        title: "Ran cargo clippy -- -D warnings",
        status: "Checking lint cleanliness",
        steps: &["No warnings emitted", "No new allow attributes"],
    },
    ToolGroup {
        title: "Reviewed",
        status: "Scanning the diff",
        steps: &[
            "Read git diff -- src/modules",
            "Confirm module list order",
            "Verify exit checks during animation",
        ],
    },
];

const ACTIVE_COMMANDS: &[&str] = &[
    "rg \"Module for\" src/modules",
    "sed -n '1,140p' src/modules/mod.rs",
    "cargo fmt --all",
    "cargo check",
    "cargo test --lib",
    "git diff --stat",
    "cargo clippy -- -D warnings",
    "cargo insta pending-snapshots -p codex-tui",
    "rg \"StatusIndicator\" codex-rs/tui/src",
    "sed -n '220,320p' tui/src/chatwidget/tests/status_and_layout.rs",
];

const FINAL_MESSAGES: &[&str] = &[
    "Implemented the module and kept the change scoped to genact.",
    "Added the animation path and verified the module registration.",
    "The module now mimics Codex-style reasoning, tool calls, edits, and checks.",
    "The patch is scoped, formatted, and verified by the targeted checks.",
    "I updated the terminal flow and left unrelated files alone.",
    "The new behavior is covered by the focused status-layout path.",
];

const DIFF_STATS: &[&str] = &[
    "src/modules/codex.rs      | 118 +++++++++++++++++++++++++++++++++++++++++",
    "src/modules/mod.rs        |   2 +",
    "README.md                 |   2 +-",
    "tests/status_layout.rs    |  36 +++++++++++--",
    "config/schema.json        |   8 ++-",
];

const QUEUED_MESSAGES: &[&str] = &[
    "tab to queue message",
    "queued: also check the web build",
    "queued: explain why the parser needed this guard",
    "queued: keep the README example in sync",
];

#[async_trait(?Send)]
impl Module for Codex {
    fn name(&self) -> &'static str {
        "codex"
    }

    fn signature(&self) -> String {
        "codex".to_string()
    }

    async fn run(&self, appconfig: &AppConfig) {
        let mut rng = rng();

        prompt(&mut rng).await;

        let intro = INTROS.choose(&mut rng).unwrap();
        print(format!("{} ", Paint::cyan("•").bold())).await;
        dprint(*intro, rng.random_range(4..12)).await;
        newline().await;
        newline().await;

        render_thinking_block(&mut rng).await;
        newline().await;

        render_plan(&mut rng).await;
        newline().await;

        let groups_to_render = rng.random_range(7..=11);
        let selected_groups: Vec<&ToolGroup> =
            TOOL_GROUPS.sample(&mut rng, groups_to_render).collect();
        for group in selected_groups {
            if appconfig.should_exit() {
                return;
            }

            if !animate_status(group.status, &mut rng, appconfig).await {
                return;
            }
            render_group(group, &mut rng).await;

            if rng.random_bool(0.32) {
                render_thinking_note(&mut rng).await;
            }
            if rng.random_bool(0.35) {
                render_background_terminal(&mut rng).await;
            }
            if rng.random_bool(0.25) {
                render_diff_stat(&mut rng).await;
            }
            if rng.random_bool(0.18) {
                render_queued_message(&mut rng).await;
            }

            newline().await;
            csleep(rng.random_range(200..700)).await;
        }

        if !animate_status("Preparing final response", &mut rng, appconfig).await {
            return;
        }

        print(format!("{} ", Paint::green("•").bold())).await;
        dprint(
            *FINAL_MESSAGES.choose(&mut rng).unwrap(),
            rng.random_range(3..9),
        )
        .await;
        newline().await;
        render_final_summary(&mut rng).await;
    }
}

async fn render_thinking_block(rng: &mut ThreadRng) {
    print(format!(
        "{} {}",
        Paint::magenta("•").bold(),
        Paint::magenta("Thinking").bold(),
    ))
    .await;
    newline().await;

    let notes_to_show = rng.random_range(3..=5);
    for note in THINKING_NOTES.sample(rng, notes_to_show) {
        print(format!("  {} ", Paint::new("└").dim())).await;
        dprint(Paint::new(note).dim().to_string(), rng.random_range(8..18)).await;
        newline().await;
        csleep(rng.random_range(120..260)).await;
    }
}

async fn render_thinking_note(rng: &mut ThreadRng) {
    let note = THINKING_NOTES.choose(rng).unwrap();

    print(format!("{} ", Paint::magenta("  thinking:").bold())).await;
    dprint(Paint::new(note).dim().to_string(), rng.random_range(8..18)).await;
    newline().await;
}

async fn render_plan(rng: &mut ThreadRng) {
    print(format!(
        "{} {}",
        Paint::yellow("•").bold(),
        Paint::yellow("Plan").bold(),
    ))
    .await;
    newline().await;

    let active_step = rng.random_range(1..=2);
    for (index, step) in PLAN_STEPS.iter().enumerate() {
        let status = if index < active_step {
            Paint::green("completed").to_string()
        } else if index == active_step {
            Paint::cyan("in_progress").to_string()
        } else {
            Paint::new("pending").dim().to_string()
        };
        print(format!("  {} {} {}", Paint::new("└").dim(), status, step)).await;
        newline().await;
        csleep(rng.random_range(50..140)).await;
    }
}

async fn prompt(rng: &mut ThreadRng) {
    let cwd = [
        "/home/user/project",
        "/workspaces/app",
        "/tmp/genact",
        "~/src/service",
    ]
    .choose(rng)
    .unwrap();
    let branch = ["main", "fix/codex-animation", "feature/tui", "cleanup"]
        .choose(rng)
        .unwrap();
    let prompt = PROMPTS.choose(rng).unwrap();

    print(format!(
        "{} {}  {} {}\r\n\r\n",
        Paint::cyan("codex").bold(),
        Paint::new(cwd).dim(),
        Paint::new("git:").dim(),
        Paint::yellow(branch),
    ))
    .await;
    print(format!("{} {prompt}", Paint::magenta("›").bold())).await;
    newline().await;
    newline().await;
}

async fn animate_status(status: &str, rng: &mut ThreadRng, appconfig: &AppConfig) -> bool {
    let ticks = rng.random_range(6..16);
    let tick_ms = rng.random_range(90..160);

    for tick in 0..ticks {
        let elapsed = tick * tick_ms / 1000;
        let dots = ".".repeat((tick as usize % 3) + 1);
        erase_line().await;
        print(format!(
            "{} {} ({} {} {})",
            Paint::cyan("•").bold(),
            Paint::new(format!("{status}{dots}")).cyan(),
            Paint::new(format!("{elapsed}s")).yellow(),
            Paint::new("•").dim(),
            Paint::new("esc to interrupt").dim(),
        ))
        .await;
        csleep(tick_ms).await;

        if appconfig.should_exit() {
            erase_line().await;
            return false;
        }
    }

    erase_line().await;
    true
}

async fn render_group(group: &ToolGroup, rng: &mut ThreadRng) {
    print(format!(
        "{} {}",
        Paint::green("•").bold(),
        Paint::green(group.title).bold(),
    ))
    .await;
    newline().await;

    for (index, step) in group.steps.iter().enumerate() {
        let marker = if index == 0 { "  └ " } else { "    " };
        print(format!("{}", Paint::new(marker).dim())).await;
        dprint(*step, rng.random_range(1..5)).await;
        newline().await;
        csleep(rng.random_range(60..180)).await;
    }
}

async fn render_diff_stat(rng: &mut ThreadRng) {
    let lines = rng.random_range(2..=4);

    print(format!(
        "{} {}",
        Paint::blue("•").bold(),
        Paint::blue("Diff"),
    ))
    .await;
    newline().await;

    for stat in DIFF_STATS.sample(rng, lines) {
        print(format!("{} {}", Paint::new("  └").dim(), stat)).await;
        newline().await;
        csleep(rng.random_range(40..110)).await;
    }
}

async fn render_queued_message(rng: &mut ThreadRng) {
    let message = QUEUED_MESSAGES.choose(rng).unwrap();

    print(format!(
        "{} {}",
        Paint::new("›").dim(),
        Paint::new(message).dim(),
    ))
    .await;
    newline().await;
}

async fn render_background_terminal(rng: &mut ThreadRng) {
    let command = ACTIVE_COMMANDS.choose(rng).unwrap();
    let request_id = gen_hex_string(rng, 6);

    print(format!(
        "{} {} {}",
        Paint::magenta("↳").bold(),
        Paint::new("Interacted with background terminal ·").dim(),
        Paint::blue(command),
    ))
    .await;
    newline().await;
    print(format!(
        "{} {}",
        Paint::new("  └ session").dim(),
        Paint::new(request_id).dim(),
    ))
    .await;
    newline().await;
}

async fn render_final_summary(rng: &mut ThreadRng) {
    let changed = rng.random_range(1..=4);
    let checks = [
        "cargo fmt --all",
        "cargo check",
        "cargo test --lib",
        "cargo clippy -- -D warnings",
    ];
    let checks_to_show = rng.random_range(2..=checks.len());

    print(format!(
        "{} {}",
        Paint::green("•").bold(),
        Paint::green("Verification").bold(),
    ))
    .await;
    newline().await;
    print(format!(
        "{} {} files changed",
        Paint::new("  └").dim(),
        Paint::new(changed).yellow(),
    ))
    .await;
    newline().await;

    for check in checks.sample(rng, checks_to_show) {
        print(format!(
            "{} {} {}",
            Paint::new("    ").dim(),
            Paint::green("passed"),
            check,
        ))
        .await;
        newline().await;
        csleep(rng.random_range(50..140)).await;
    }
}
