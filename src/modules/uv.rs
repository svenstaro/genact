//! Pretend to set uv environment
use crate::args::AppConfig;
use crate::data::PACKAGES_LIST;
use crate::generators::gen_package_version;
use crate::io::{csleep, cursor_up, erase_line, newline, print};
use crate::modules::Module;
use async_trait::async_trait;
use rand::seq::IndexedRandom;
use rand::{RngExt, rng};
use std::time::Instant;
use yansi::Paint;

struct PackageInfo {
    name: String,
    version: String,
    size: f32,
    download_speed: f32,
}

// Convert KiB to MiB for better readability
fn format_size(kib: f32) -> String {
    if kib >= 1024.0 {
        format!("{:>7.2} MiB", kib / 1024.0)
    } else {
        format!("{:>7.2} KiB", kib)
    }
}

// Rewrite the current line with the given string
async fn rewrite_line<S: Into<String>>(s: S) {
    print(format!("\r\x1b[2K{}", s.into())).await;
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
                size: rng.random_range(64.0..2048.0),
                download_speed: rng.random_range(1024.0..2048.0),
            })
            .collect();

        // Initialize virtualenv: Environment Setup
        print("Using CPython 3.12.2\n").await;
        print("Creating virtualenv at: .venv\n").await;
        csleep(512).await;

        // Resolve dependencies: Simulates the look-up process of multiple package versions
        let start_resolve = Instant::now();
        for (idx, pkg) in pkgs.iter().enumerate() {
            let frame = SPINNER_FRAME[idx % SPINNER_FRAME.len()];
            rewrite_line(format!(
                "{} {}",
                frame,
                format!("{}=={}", pkg.name, pkg.version)
            ))
            .await;
            csleep(rng.random_range(32..64)).await;
        }
        let resolve_duration = start_resolve.elapsed();
        rewrite_line(format!(
            "Resolved {} in {:.2?}\n",
            Paint::new(format!("{} packages", num_resolved_pkgs).bold()),
            resolve_duration
        ))
        .await;
        csleep(512).await;

        // Package Preparation
        let num_prepared_pkgs = rng.random_range(64..num_resolved_pkgs);
        let start_prepare = Instant::now();
        for i in 0..num_prepared_pkgs {
            rewrite_line(format!(
                "{} Preparing packages... ({}/{})",
                SPINNER_FRAME[i % SPINNER_FRAME.len()],
                i + 1,
                num_prepared_pkgs
            ))
            .await;
            csleep(rng.random_range(64..128)).await;
        }
        let prepare_duration = start_prepare.elapsed();
        rewrite_line(format!(
            "Prepared {} in {:.2?}\n",
            Paint::new(format!("{} packages", num_prepared_pkgs).bold()),
            prepare_duration
        ))
        .await;
        csleep(512).await;

        // Download packages Chunked Download, Using chunks prevents the terminal from scrolling past the viewport
        let chunk_size: usize = 8;
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
                for i in 0..current_chunk_len {
                    let pkg = &chunk[i];
                    let mut downloaded = elapsed_time * pkg.download_speed;
                    if downloaded > pkg.size {
                        downloaded = pkg.size;
                    } else {
                        chunk_finished = false;
                    }

                    let progress_ratio = downloaded / pkg.size;
                    let bar_len = (progress_ratio * 30.0) as usize;
                    let bar = Paint::new("-".repeat(bar_len)).green();

                    // Fixed width formatting ensures the bars don't jump around
                    rewrite_line(format!(
                        "{:40} {:30} {:>11}/{:<11}\n",
                        pkg.name,
                        bar,
                        format_size(downloaded),
                        format_size(pkg.size)
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
        for i in 0..=pb_width {
            if appconfig.should_exit() {
                return;
            }
            let progress_count = (i as f32 / pb_width as f32 * num_prepared_pkgs as f32) as usize;
            let progress = "█".repeat(i);
            let empty = "░".repeat(pb_width - i);
            rewrite_line(format!(
                "{}{} [{}/{}] Installing wheels...",
                progress, empty, progress_count, num_prepared_pkgs,
            ))
            .await;
            csleep(64).await;
        }

        let install_duration = start_install.elapsed();
        rewrite_line(format!(
            "Installed {} in {:.2?}\n",
            Paint::new(format!("{} packages", num_prepared_pkgs)).bold(),
            install_duration
        ))
        .await;

        for pkg in pkgs {
            if appconfig.should_exit() {
                return;
            }
            print(format!(
                " {} {}\n",
                Paint::new("+").green(),
                format!("{}=={}", Paint::new(pkg.name).bold(), pkg.version),
            ))
            .await;
            csleep(32).await;
        }
        csleep(512).await;
    }
}
