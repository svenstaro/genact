use rand::{thread_rng, Rng};
use rand::distributions::{Normal, IndependentSample};
use std::time::Instant;
use yansi::Paint;
use chrono::prelude::*;
use chrono::Duration;

use utils;

pub fn run() {
    let mut rng = thread_rng();
    let line_count = rng.gen_range(30, 200);

    // How often to receive a new job.
    let new_job_every_n_lines = rng.gen_range(20, 50);
    let mut remaining_until_new_job = new_job_every_n_lines;

    let approximate_mhs_per_gpu = rng.gen_range(1.0, 99.0);
    let num_gpus = rng.gen_range(1, 8);

    let now = Instant::now();

    for _ in 1..line_count {
        let sleep_length = 200;

        let time = Paint::purple(Local::now().format("%H:%M:%S"));

        if remaining_until_new_job == 0 {
            remaining_until_new_job = new_job_every_n_lines;
            let info = Paint::cyan("ℹ").bold();

            println!("{info:>3}  {time}{separator}{stratum}  Received new job #{jobhex}  seed: #{seedhex}  target: #{targethex}",
                     info=info,
                     time=time,
                     separator=Paint::black("|"),
                     stratum=Paint::blue("stratum"),
                     jobhex=utils::rand_hex_string(8),
                     seedhex=utils::rand_hex_string(32),
                     targethex=utils::rand_hex_string(24));
        } else {
            remaining_until_new_job -= 1;
            let info = Paint::green("m");

            let normal = Normal::new(0.0, 0.2);
            let mut total_mhs = 0.0;
            let mut gpus = String::from("");
            for gpu in 0..num_gpus {
                let actual_mhs_per_gpu = approximate_mhs_per_gpu + normal.ind_sample(&mut rng);
                gpus.push_str(&format!("gpu/{gpu} {mhs:.2} ", gpu=gpu, mhs=Paint::cyan(actual_mhs_per_gpu)));
                total_mhs += actual_mhs_per_gpu;
            }
            let speed = format!("Speed {mhs:>6.2} Mh/s", mhs=Paint::cyan(total_mhs).bold());
            let duration = Duration::from_std(now.elapsed()).expect("Couldn't make chrono::Duration from std::time::Duration!");
            let elapsed = format!("{hours:02}:{minutes:02}", hours=duration.num_hours(), minutes=duration.num_minutes());
            println!("{info:>3}  {time}{separator}{cryptominer}  {speed}    {gpus}  [A0+0:R0+0:F0] Time: {elapsed}",
                     info=info,
                     time=time,
                     separator=Paint::black("|"),
                     cryptominer=Paint::blue("cryptominer"),
                     speed=speed,
                     gpus=gpus,
                     elapsed=elapsed);
        };

            utils::sleep(sleep_length);
    }
}
