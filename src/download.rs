use rand::{thread_rng, Rng};
use std::cmp::max;

use pbr::{ProgressBar, Units};

#[cfg(target_os = "emscripten")]
use utils::TermWriter;

use utils::{csleep, gen_file_name_with_ext};
use CFILES_LIST;
use EXTENSIONS_LIST;
use parse_args::AppConfig;

pub fn run(appconfig: &AppConfig) {
    let mut rng = thread_rng();

    // We'll use the same extension for all files of this whole run to make things seem more
    // realistic.
    let extension = rng.choose(EXTENSIONS_LIST).unwrap_or(&".wat");

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

        #[cfg(target_os = "emscripten")]
        let mut pb = ProgressBar::on(TermWriter, file_bytes);
        #[cfg(not(target_os = "emscripten"))]
        let mut pb = ProgressBar::new(file_bytes);
        pb.set_units(Units::Bytes);
        pb.message(&gen_file_name_with_ext(&mut rng, &CFILES_LIST, extension));
        for _ in 0..cycles {
            pb.add(bytes_per_sleep);
            csleep(sleep_millis);

            if appconfig.should_exit() {
                return;
            }
        }
        pb.finish_println("");
    }
}
