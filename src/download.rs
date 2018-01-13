/// Module that dumps some random memory locations in a slightly cool fashion.

use rand::{thread_rng, Rng};
use yansi::Paint;
use std::io::Write;
use std::io::stdout;

use utils;

pub fn run() {
    let mut rng = thread_rng();

    // let count = 1000;
    // let mut pb = ProgressBar::new(count);
    // pb.format("╢▌▌░╟");
    // for _ in 0..count {
    //     pb.inc();
    //     #[cfg(not(target_os = "emscripten"))]
    //     thread::sleep(time::Duration::from_millis(200));
    //
    //     #[cfg(target_os = "emscripten")]
    //     unsafe {
    //         emscripten_sys::emscripten_sleep(200u32);
    //         // For some reason, we actually have to manually print a newline here even if we
    //         // flush it manually in order to get it to draw anything at all. This is really
    //         // weird but I'll figure it out some other time.
    //         println!();
    //     }
    // }
    // pb.finish_print("done");

    let row_delay = rng.gen_range(100, 200);
    utils::sleep(row_delay);
}
