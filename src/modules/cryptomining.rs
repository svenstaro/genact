//! Pretend to mine a cryptocurrency
use async_trait::async_trait;
use chrono::prelude::*;
use chrono::Duration;
use instant::Instant;
use rand::{thread_rng, Rng};
use rand_distr::{Distribution, Normal};
use yansi::Paint;

use crate::args::AppConfig;
use crate::generators::gen_hex_string;
use crate::io::{csleep, newline, print};
use crate::modules::Module;

pub struct Crytomining;

#[allow(clippy::format_push_string)]
#[async_trait(?Send)]
impl Module for Crytomining {
    fn name(&self) -> &'static str {
        "cryptomining"
    }

    fn signature(&self) -> String {
        "./cryptominer.sh --gpu all --provider stratum".to_string()
    }

    async fn run(&self, appconfig: &AppConfig) {
        let mut rng = thread_rng();
        let num_lines = rng.gen_range(300..1000);

        // How often to receive a new job.
        let new_job_every_n_lines = rng.gen_range(20..50);
        let mut remaining_until_new_job = new_job_every_n_lines;

        // Base value for how many MH/s a GPU gets.
        let approximate_mhs_per_gpu = rng.gen_range(1.0..99.0);
        let num_gpus = rng.gen_range(1..8);

        // How often a solution will be found.
        let solution_found_every_n_lines = rng.gen_range(80..200);
        let mut remaining_until_next_solution = solution_found_every_n_lines;

        // How many solutions have already been found.
        let mut num_solutions_found = 0;

        let now = Instant::now();

        for _ in 1..num_lines {
            let sleep_length = 300;

            let time = Local::now().format("%H:%M:%S");
            let time = time.magenta();

            if remaining_until_new_job == 0 {
                remaining_until_new_job = new_job_every_n_lines;
                let info = Paint::cyan("ℹ").bold();

                print(format!("{info:>3}  {time}{separator}{source:<13}Received new job #{jobhex}  seed: #{seedhex}  target: #{targethex}",
                     info=info,
                     time=time,
                     separator=Paint::black("|"),
                     source=Paint::blue("stratum"),
                     jobhex=gen_hex_string(&mut rng, 8),
                     seedhex=gen_hex_string(&mut rng, 32),
                     targethex=gen_hex_string(&mut rng, 24))).await;
                newline().await;
            } else if remaining_until_next_solution == 0 {
                remaining_until_next_solution = solution_found_every_n_lines;
                num_solutions_found += 1;
                let info = Paint::cyan("ℹ").bold();

                print(format!("{info:>3}  {time}{separator}{source:<13}Solution found; Submitted to stratum.buttcoin.org",
                     info=info,
                     time=time,
                     separator=Paint::black("|"),
                     source=Paint::blue("CUDA0"))).await;
                newline().await;
                print(format!(
                    "{info:>3}  {time}{separator}{source:<13}Nonce: 0x{noncehex}",
                    info = info,
                    time = time,
                    separator = Paint::black("|"),
                    source = Paint::blue("CUDA0"),
                    noncehex = gen_hex_string(&mut rng, 16)
                ))
                .await;
                newline().await;
                print(format!(
                    "{info:>3}  {time}{separator}{source:<13}{accepted}",
                    info = info,
                    time = time,
                    separator = Paint::black("|"),
                    source = Paint::blue("stratum"),
                    accepted = Paint::green("Accepted.")
                ))
                .await;
                newline().await;
            } else {
                remaining_until_new_job -= 1;
                remaining_until_next_solution -= 1;
                let info = Paint::green("m");

                let normal = Normal::new(0.0, 0.2).unwrap();
                let mut total_mhs = 0.0;
                let mut gpus = String::from("");
                for gpu in 0..num_gpus {
                    let actual_mhs_per_gpu = approximate_mhs_per_gpu + normal.sample(&mut rng);
                    gpus.push_str(&format!(
                        "gpu/{gpu} {mhs:.2} ",
                        gpu = gpu,
                        mhs = actual_mhs_per_gpu.cyan()
                    ));
                    total_mhs += actual_mhs_per_gpu;
                }
                let speed = format!("Speed {mhs:>6.2} Mh/s", mhs = total_mhs.cyan().bold());
                let duration = Duration::from_std(now.elapsed())
                    .expect("Couldn't make chrono::Duration from std::time::Duration!");
                let elapsed = format!(
                    "{hours:02}:{minutes:02}",
                    hours = duration.num_hours(),
                    minutes = duration.num_minutes()
                );
                print(format!("{info:>3}  {time}{separator}{source:<13}{speed}    {gpus}  [A{solutions}+0:R0+0:F0] Time: {elapsed}",
                     info=info,
                     time=time,
                     separator=Paint::black("|"),
                     source=Paint::blue("cryptominer"),
                     speed=speed,
                     gpus=gpus,
                     solutions=num_solutions_found,
                     elapsed=elapsed)).await;
                newline().await;
            }
            csleep(sleep_length).await;

            if appconfig.should_exit() {
                return;
            }
        }
    }
}
