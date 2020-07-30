//! Petend to do some downloading
use rand::prelude::*;
use std::cmp::max;

use crate::data::CFILES_LIST;
use crate::data::EXTENSIONS_LIST;
use crate::generators::gen_file_name_with_ext;
use crate::io::{newline, cursor_left, print, erase_line, csleep};
use crate::parse_args::AppConfig;

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

        let download_speed_offset = rng.gen_range(-5_000_000i32, 5_000_000i32);
        let actual_download_speed = max(100_000, download_speed + download_speed_offset) as u64;

        // How much to increase this by per sleep cycle.
        let bytes_per_sleep = (actual_download_speed * sleep_millis) / 1000;

        // How many cycles we need.
        let cycles = file_bytes / bytes_per_sleep;

        let mut bar = progress_string::BarBuilder::new()
            .total(cycles as usize)
            .full_char('=')
            .width(50)
            .get_bar();

        for i in 1..=cycles {
            erase_line().await;
            bar.replace(i as usize);
            cursor_left(bar.get_last_width() as u64).await;
            print(format!("{}", bar.to_string())).await;
            csleep(sleep_millis).await;

            if appconfig.should_exit() {
                return;
            }
        }
        newline().await;
    }
}
