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

struct ActiveDownload {
    pkg: PackageInfo,
    downloaded: f32,
    finished: bool,
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
        let mut pkgs: Vec<PackageInfo> = PACKAGES_LIST
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
            erase_line().await;
            print(format!("{} {}=={}", frame, pkg.name, pkg.version)).await;
            csleep(rng.random_range(32..64)).await;
        }
        let resolve_duration = start_resolve.elapsed();
        erase_line().await;
        print(
            Paint::new(format!(
                "Resolved {} packages in {:.2?}",
                num_resolved_pkgs, resolve_duration
            ))
            .dim()
            .to_string(),
        )
        .await;
        csleep(512).await;

        // Prepare packages for installation
        let num_prepared_pkgs = rng.random_range(64..num_resolved_pkgs);
        let start_prepare = Instant::now();
        for i in 0..num_prepared_pkgs {
            erase_line().await;
            print(
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
        erase_line().await;
        print(
            Paint::new(format!(
                "Prepared {} packages in {:.2?}\n",
                num_prepared_pkgs, prepare_duration
            ))
            .dim()
            .to_string(),
        )
        .await;
        csleep(512).await;

        // Set max slots for concurrent download packages
        let max_slots = {
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

        // Pre-allocate empty lines to avoid flickering during first 'cursor_up'
        for _ in 0..max_slots {
            newline().await;
        }

        // Initialize download tracking variables
        let mut active_downloads: Vec<ActiveDownload> = Vec::new();
        let mut completed_count = 0;
        let total_pkgs = pkgs.len();
        let time_step: f32 = 0.2;

        let start_install = Instant::now();
        while completed_count < total_pkgs {
            if appconfig.should_exit() {
                return;
            }
            // Fill empty slots with pending downloads
            while active_downloads.len() < max_slots && !pkgs.is_empty() {
                active_downloads.push(ActiveDownload {
                    pkg: pkgs.remove(0),
                    downloaded: 0.0,
                    finished: false,
                });
            }

            // Move cursor to the top of the download area
            cursor_up(max_slots as u64).await;

            // Update and render progress for each active download slot
            for i in 0..max_slots {
                erase_line().await;
                if let Some(download) = active_downloads.get_mut(i) {
                    if !download.finished {
                        download.downloaded += download.pkg.download_speed as f32 * time_step;
                        if download.downloaded >= download.pkg.size as f32 {
                            download.downloaded = download.pkg.size as f32;
                            download.finished = true;
                        }
                    }

                    let progress_ratio = download.downloaded / download.pkg.size as f32;
                    let bar_len = (progress_ratio * 30.0) as usize;
                    print(format!(
                        "{:40} {:30} {:>11}/{:<11}\n",
                        Paint::new(&download.pkg.name).dim().to_string(),
                        Paint::new("-".repeat(bar_len)).green(),
                        format_size(download.downloaded as u32, BINARY),
                        format_size(download.pkg.size, BINARY)
                    ))
                    .await;
                } else {
                    // Render empty line for unused slots
                    print("\n").await;
                }
            }

            // Clean up finished downloads and track total completion
            let before_len = active_downloads.len();
            active_downloads.retain(|d| !d.finished);
            completed_count += before_len - active_downloads.len();
            csleep(50).await;
        }

        // Clear the download display area
        for _ in 0..max_slots {
            cursor_up(1).await;
            erase_line().await;
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
            erase_line().await;
            print(format!(
                "{} [{}/{}] Installing wheels...",
                progress_bar, i, num_prepared_pkgs,
            ))
            .await;
            csleep(64).await;
        }

        let install_duration = start_install.elapsed();
        erase_line().await;
        print(
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
