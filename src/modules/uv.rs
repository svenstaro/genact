//! Pretend to set uv environment
use async_trait::async_trait;
use humansize::{BINARY, format_size};
use instant::Instant;
use rand::seq::IndexedRandom;
use rand::{RngExt, rng};
use yansi::Paint;

use crate::args::AppConfig;
use crate::data::PACKAGES_LIST;
use crate::generators::gen_package_version;
use crate::io::{csleep, cursor_up, erase_line, newline, print};
use crate::modules::Module;

struct PackageInfo {
    name: String,
    version: String,
    size: u32,
    download_speed: u32,
}

// Rewrite the current line with the given string
async fn rewrite_line<S: Into<String>>(s: S) {
    erase_line().await;
    print(s).await;
}

pub struct Uv;

#[async_trait(?Send)]
impl Module for Uv {
    fn name(&self) -> &'static str {
        "uv"
    }

    fn signature(&self) -> String {
        "uv add".to_string()
    }

    async fn run(&self, appconfig: &AppConfig) {
        const SPINNER_FRAME: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

        let mut rng = rng();
        let num_resolved_pkgs = rng.random_range(128..256);
        let pkgs: Vec<PackageInfo> = PACKAGES_LIST
            .sample(&mut rng, num_resolved_pkgs)
            .map(|&name| PackageInfo {
                name: name.to_string(),
                version: gen_package_version(&mut rng),
                size: rng.random_range(50_000..20_000_000),
                download_speed: rng.random_range(1_000_000..10_000_000),
            })
            .collect();

        // Initialize virtualenv: Environment Setup
        let python_versions = [
            "3.8.20", "3.9.25", "3.10.20", "3.11.15", "3.12.13", "3.13.16", "3.14.13",
        ];
        print(format!(
            "Using CPython {}\n",
            python_versions.choose(&mut rng).unwrap()
        ))
        .await;
        print("Creating virtualenv at: .venv\n").await;
        csleep(512).await;

        // Resolve dependencies: Simulates the look-up process of multiple package versions
        let start_resolve = Instant::now();
        for (idx, pkg) in pkgs.iter().enumerate() {
            let frame = SPINNER_FRAME[idx % SPINNER_FRAME.len()];
            rewrite_line(format!("{} {}=={}", frame, pkg.name, pkg.version)).await;
            csleep(rng.random_range(32..64)).await;
        }
        let resolve_duration = start_resolve.elapsed();
        rewrite_line(
            Paint::new(format!(
                "Resolved {} packages in {:.2?}",
                num_resolved_pkgs, resolve_duration
            ))
            .dim()
            .to_string(),
        )
        .await;
        csleep(512).await;

        // Package Preparation
        let num_prepared_pkgs = rng.random_range(64..num_resolved_pkgs);
        let start_prepare = Instant::now();
        for i in 0..num_prepared_pkgs {
            rewrite_line(
                Paint::new(format!(
                    "{} Preparing packages... ({}/{})",
                    SPINNER_FRAME[i % SPINNER_FRAME.len()],
                    i + 1,
                    num_prepared_pkgs
                ))
                .dim()
                .to_string(),
            )
            .await;
            csleep(rng.random_range(64..128)).await;
        }
        let prepare_duration = start_prepare.elapsed();
        rewrite_line(
            Paint::new(format!(
                "Prepared {} packages in {:.2?}\n",
                num_prepared_pkgs, prepare_duration
            ))
            .dim()
            .to_string(),
        )
        .await;
        csleep(512).await;

        // Download packages Chunked Download
        let chunk_size = {
            #[cfg(not(target_arch = "wasm32"))]
            {
                terminal_size::terminal_size()
                    .map(|(_, h)| (h.0 * 2 / 3).max(1))
                    .unwrap_or(1) as usize
            }
            #[cfg(target_arch = "wasm32")]
            {
                8
            }
        };
        let time_step: f32 = 0.1;
        let start_install = Instant::now();
        for chunk in pkgs.chunks(chunk_size) {
            if appconfig.should_exit() {
                return;
            }
            let mut elapsed_time: f32 = 0.0; // Reset download time for each chunk
            let current_chunk_len = chunk.len();

            // Pre-allocate empty lines to avoid flickering during first 'cursor_up'
            for _ in 0..current_chunk_len {
                newline().await;
            }
            loop {
                if appconfig.should_exit() {
                    return;
                }
                cursor_up(current_chunk_len as u64).await;
                let mut chunk_finished = true;
                for pkg in chunk.iter().take(current_chunk_len) {
                    let mut downloaded = elapsed_time * pkg.download_speed as f32;
                    if downloaded > pkg.size as f32 {
                        downloaded = pkg.size as f32;
                    } else {
                        chunk_finished = false;
                    }

                    let progress_ratio = downloaded / pkg.size as f32;
                    let bar_len = (progress_ratio * 30.0) as usize;
                    let bar = Paint::new("-".repeat(bar_len)).green();

                    // Fixed width formatting ensures the bars don't jump around
                    rewrite_line(format!(
                        "{:40} {:30} {:>11}/{:<11}\n",
                        Paint::new(&pkg.name).dim().to_string(),
                        bar,
                        format_size(downloaded as u32, BINARY),
                        format_size(pkg.size, BINARY)
                    ))
                    .await;
                }

                if chunk_finished {
                    break;
                }
                elapsed_time += time_step;
                csleep(64).await;
            }

            // Clean up the chunk UI to prepare for the next set or next phase
            for _ in 0..current_chunk_len {
                cursor_up(1).await;
                erase_line().await;
            }
        }

        // Install wheels
        let pb_width = 30;
        let mut progress_bar = progress_string::BarBuilder::new()
            .total(num_prepared_pkgs)
            .width(pb_width)
            .empty_char('░')
            .build();

        for i in 0..=num_prepared_pkgs {
            if appconfig.should_exit() {
                return;
            }
            progress_bar.replace(i);
            rewrite_line(format!(
                "{} [{}/{}] Installing wheels...",
                progress_bar, i, num_prepared_pkgs,
            ))
            .await;
            csleep(64).await;
        }

        let install_duration = start_install.elapsed();
        rewrite_line(
            Paint::new(format!(
                "Installed {} packages in {:.2?}\n",
                num_prepared_pkgs, install_duration
            ))
            .dim()
            .to_string(),
        )
        .await;

        for pkg in pkgs {
            if appconfig.should_exit() {
                return;
            }
            print(format!(
                " {} {}{}{}\n",
                Paint::new("+").green(),
                Paint::new(pkg.name).bold(),
                Paint::new("==").dim(),
                Paint::new(pkg.version).dim()
            ))
            .await;
            csleep(32).await;
        }
        csleep(512).await;
    }
}
