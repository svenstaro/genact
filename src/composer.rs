/// Module that pretends to install composer packages.
use rand::prelude::*;
use rand_distr::{ChiSquared, Distribution};
use yansi::Paint;

use crate::utils::csleep;
use crate::COMPOSERS_LIST;
use crate::parse_args::AppConfig;

fn gen_package_version(rng: &mut ThreadRng) -> String {
    let chi = ChiSquared::new(1.0).unwrap();
    format!(
        "{major:.0}.{minor:.0}.{patch:.0}",
        major = 10.0 * chi.sample(rng),
        minor = 10.0 * chi.sample(rng),
        patch = 10.0 * chi.sample(rng)
    )
}

pub fn run(appconfig: &AppConfig) {
    let mut rng = thread_rng();
    let num_packages = rng.gen_range(10, 100);
    // Choose `num_packages` packages, non-repeating and in random order
    let chosen_names: Vec<_> = COMPOSERS_LIST.choose_multiple(&mut rng, num_packages).collect();
    let chosen_packages: Vec<_> = chosen_names
        .iter()
        .map(|name| (name, gen_package_version(&mut rng)))
        .collect();

    println!(
        "{text}",
        text = Paint::green("Loading composer repositories with package information")
    );
    println!(
        "{text}",
        text = Paint::green("Updating dependencies (including require-dev)")
    );

    for stage in &["Installing"] {
        for &(package_name, ref package_version) in &chosen_packages {
            let sleep_length = rng.gen_range(100, 2000);

            println!(
                "  - {stage} {package_name} ({package_version}): Loading from cache",
                stage = stage,
                package_name = Paint::green(package_name),
                package_version = Paint::yellow(package_version)
            );

            csleep(sleep_length);

            if appconfig.should_exit() {
                return;
            }
        }
    }
    println!("{text}", text = Paint::green("Writing lock file"));
    println!("{text}", text = Paint::green("Generating autoload files"))
}
