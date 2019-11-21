use rand::distributions::{ChiSquared, Distribution, Exp};
/// Module that pretends to run cargo to install rust packages.
use rand::prelude::*;
use std::time::Instant;
use yansi::Paint;

use crate::parse_args::AppConfig;
use crate::utils::csleep;
use crate::PACKAGES_LIST;

fn gen_package_version(rng: &mut ThreadRng) -> String {
    let chi = ChiSquared::new(1.0);
    let exp = Exp::new(2.0);
    format!(
        "{major:.0}.{minor:.0}.{patch:.0}",
        major = exp.sample(rng),
        minor = 10.0 * chi.sample(rng),
        patch = 10.0 * chi.sample(rng)
    )
}

pub fn run(appconfig: &AppConfig) {
    let mut rng = thread_rng();
    let num_packages = rng.gen_range(10, 100);
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
            let sleep_length = rng.gen_range(100, 2000);

            println!(
                "{stage:>12} {package_name} v{package_version}",
                stage = Paint::green(stage).bold(),
                package_name = package_name,
                package_version = package_version
            );

            csleep(sleep_length);

            if appconfig.should_exit() {
                return;
            }
        }
    }
    let elapsed = now.elapsed();
    let seconds = elapsed.as_secs() as f32 + elapsed.subsec_nanos() as f32 / 1_000_000_000.0;
    println!(
        "{stage:>12} release [optimized] target(s) in {seconds:.2} secs",
        stage = Paint::green("Finished").bold(),
        seconds = seconds
    );
}
