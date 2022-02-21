//! Pretend to run cargo to install rust packages
use async_trait::async_trait;
use instant::Instant;
use rand::prelude::*;
use yansi::Paint;

use crate::args::AppConfig;
use crate::data::PACKAGES_LIST;
use crate::generators::gen_package_version;
use crate::io::{csleep, dprint, newline, print};
use crate::modules::Module;

pub struct Cargo;

#[async_trait(?Send)]
impl Module for Cargo {
    fn name(&self) -> &'static str {
        "cargo"
    }

    fn signature(&self) -> String {
        "cargo run".to_string()
    }

    async fn run(&self, appconfig: &AppConfig) {
        let mut rng = thread_rng();
        let num_packages = rng.gen_range(10..100);
        // Choose `num_packages` packages, non-repeating and in random order
        let chosen_names: Vec<_> = PACKAGES_LIST
            .choose_multiple(&mut rng, num_packages)
            .collect();
        let chosen_packages: Vec<_> = chosen_names
            .iter()
            .map(|name| (name, gen_package_version(&mut rng)))
            .collect();

        let now = Instant::now();
        for stage in &["Downloading", "Compiling"] {
            for &(package_name, ref package_version) in &chosen_packages {
                let sleep_length = rng.gen_range(100..2000);

                print(format!(
                    "{stage:>12} {package_name} v{package_version}",
                    stage = Paint::green(stage).bold(),
                ))
                .await;
                newline().await;

                csleep(sleep_length).await;

                if appconfig.should_exit() {
                    return;
                }
            }
        }
        let elapsed = now.elapsed();
        let seconds = elapsed.as_secs() as f32 + elapsed.subsec_nanos() as f32 / 1_000_000_000.0;
        dprint(
            format!(
                "{stage:>12} release [optimized] target(s) in {seconds:.2} secs",
                stage = Paint::green("Finished").bold(),
            ),
            0,
        )
        .await;
        newline().await;
    }
}
