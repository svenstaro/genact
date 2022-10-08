//! Petend to do some downloading
use async_trait::async_trait;
use humansize::{format_size, FormatSizeOptions};
use humantime::format_duration;
use rand::prelude::*;
use std::cmp::max;
use std::time::Duration;

use crate::args::AppConfig;
use crate::data::{CFILES_LIST, EXTENSIONS_LIST};
use crate::generators::gen_file_name_with_ext;
use crate::io::dprint;
use crate::io::{csleep, erase_line, get_terminal_width, newline, print};
use crate::modules::Module;

pub struct Download;

#[async_trait(?Send)]
impl Module for Download {
    fn name(&self) -> &'static str {
        "download"
    }

    fn signature(&self) -> String {
        "wget -i downloads.txt".to_string()
    }

    async fn run(&self, appconfig: &AppConfig) {
        let mut rng = thread_rng();

        // We'll use the same extension for all files of this whole run to make things seem more
        // realistic.
        let extension = EXTENSIONS_LIST.choose(&mut rng).unwrap_or(&".wat");

        // Choose speed. We'll choose an approximate speed that we'll vary a little bit.
        // Download speed in bytes per second.
        let download_speed = rng.gen_range(10_000_000..100_000_000);

        let num_files = rng.gen_range(3..10);

        for _ in 0..num_files {
            // File size in bytes.
            let file_bytes = rng.gen_range(30_000_000..300_000_000);

            // How long to sleep for in ms.
            let sleep_millis = 50;

            let file_name = &gen_file_name_with_ext(&mut rng, &CFILES_LIST, extension);

            let stats_width = 32; // Fixed sum of the width of the right side info stats.
            let rest_padding = 16; // Magic number is chars around the progress bar and other padding.
            if get_terminal_width() < stats_width + rest_padding + 7 {
                dprint("Terminal too small to display download progress\n", 10).await;
                continue;
            }
            let remaining_width = get_terminal_width() - stats_width;
            let file_name_width = remaining_width / 3;
            let full_progress_bar_size = remaining_width - file_name_width - rest_padding;
            let mut progress_bar = progress_string::BarBuilder::new()
                .total(file_bytes as usize)
                .full_char('=')
                .width(full_progress_bar_size)
                .build();

            let mut bytes_downloaded = 0u64;
            loop {
                let download_speed_offset = rng.gen_range(-5_000_000i32..5_000_000i32);
                let actual_download_speed =
                    max(100_000, download_speed + download_speed_offset) as u64;
                let percent = ((100.0 / file_bytes as f64) * bytes_downloaded as f64).min(100.0);

                // How much we'll download in this cycle.
                let bytes_incoming = (actual_download_speed / 1000) * sleep_millis;

                // How long the download will likely take.
                let eta = if bytes_downloaded == 0 {
                    Duration::default()
                } else {
                    let remaining_secs = (file_bytes as i64 - bytes_downloaded as i64).max(0)
                        / actual_download_speed as i64;
                    Duration::from_secs(remaining_secs as u64)
                };

                erase_line().await;
                progress_bar.replace(bytes_downloaded as usize);
                let size_opts = FormatSizeOptions::from(humansize::BINARY).space_after_value(false);
                let speed_opts = FormatSizeOptions::from(humansize::BINARY)
                    .space_after_value(false)
                    .suffix("/s");

                print(format!(
                        "{file_name:<file_name_width$} {percent:>4.0}%{progress_bar} {bytes_downloaded:<10} {download_speed:<12} eta {eta:<10}",
                        file_name = file_name.chars().take(file_name_width).collect::<String>(),
                        percent = percent,
                        progress_bar = progress_bar,
                        bytes_downloaded = format_size(bytes_incoming, size_opts),
                        download_speed = format_size(actual_download_speed, speed_opts),
                        eta = format_duration(eta),
                        file_name_width = file_name_width,
                        ))
                    .await;
                csleep(sleep_millis).await;

                bytes_downloaded += bytes_incoming;

                if percent >= 100.0 {
                    break;
                }

                if appconfig.should_exit() {
                    return;
                }
            }
            newline().await;
        }
    }
}
