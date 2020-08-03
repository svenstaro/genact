//! Petend to do some downloading
use rand::prelude::*;
use std::cmp::max;

use crate::data::CFILES_LIST;
use crate::data::EXTENSIONS_LIST;
use crate::generators::gen_file_name_with_ext;
use crate::io::{csleep, erase_line, newline, print, get_terminal_width};
use crate::parse_args::AppConfig;
use file_size_opts::FileSizeOpts;
use humansize::{file_size_opts, FileSize};
use humantime::format_duration;
use std::time::Duration;

pub async fn run(appconfig: &AppConfig) {
    let mut rng = thread_rng();

    // We'll use the same extension for all files of this whole run to make things seem more
    // realistic.
    let extension = EXTENSIONS_LIST.choose(&mut rng).unwrap_or(&".wat");

    // Choose speed. We'll choose an approximate speed that we'll vary a little bit.
    // Download speed in bytes per second.
    let download_speed = rng.gen_range(10_000_000, 100_000_000);

    let num_files = rng.gen_range(3, 10);

    for _ in 0..num_files {
        // File size in bytes.
        let file_bytes = rng.gen_range(30_000_000, 300_000_000);

        // How long to sleep for in ms.
        let sleep_millis = 50;

        let file_name = &gen_file_name_with_ext(&mut rng, &CFILES_LIST, extension);
        let mut bar = progress_string::BarBuilder::new()
            .total(file_bytes as usize)
            .full_char('=')
            .width(50)
            .get_bar();

        let mut bytes_downloaded = 0u64;
        loop {
            let download_speed_offset = rng.gen_range(-5_000_000i32, 5_000_000i32);
            let actual_download_speed = max(100_000, download_speed + download_speed_offset) as u64;
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
            bar.replace(bytes_downloaded as usize);
            let size_opts = FileSizeOpts {
                space: false,
                ..file_size_opts::BINARY
            };
            let speed_opts = FileSizeOpts {
                space: false,
                suffix: "/s",
                ..file_size_opts::BINARY
            };
            print(format!(
                "{file_name} {percent:.0}%{progress_bar} {bytes_downloaded} {download_speed} eta {eta} {width}",
                file_name = file_name,
                percent = percent,
                progress_bar = bar.to_string(),
                bytes_downloaded = bytes_incoming.file_size(size_opts).unwrap(),
                download_speed = actual_download_speed.file_size(speed_opts).unwrap(),
                eta = format_duration(eta).to_string(),
                width = get_terminal_width(),
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
