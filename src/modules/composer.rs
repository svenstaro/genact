//! Pretend to install composer packages
use rand::prelude::*;
use rand_distr::ChiSquared;
use yansi::Paint;

use crate::args::AppConfig;
use crate::data::COMPOSERS_LIST;
use crate::io::{csleep, newline, print};

pub fn get_signature() -> &'static str {
    "composer install"
}

fn gen_package_version(rng: &mut ThreadRng) -> String {
    let chi = ChiSquared::new(1.0).unwrap();
    format!(
        "{major:.0}.{minor:.0}.{patch:.0}",
        major = 10.0 * chi.sample(rng),
        minor = 10.0 * chi.sample(rng),
        patch = 10.0 * chi.sample(rng)
    )
}

pub async fn run(appconfig: &AppConfig) {
    let mut rng = thread_rng();
    let num_packages = rng.gen_range(10..100);
    // Choose `num_packages` packages, non-repeating and in random order
    let chosen_names: Vec<_> = COMPOSERS_LIST
        .choose_multiple(&mut rng, num_packages)
        .collect();
    let chosen_packages: Vec<_> = chosen_names
        .iter()
        .map(|name| (name, gen_package_version(&mut rng)))
        .collect();

    print(format!(
        "{text}",
        text = Paint::green("Loading composer repositories with package information")
    ))
    .await;
    newline().await;
    print(format!(
        "{text}",
        text = Paint::green("Updating dependencies (including require-dev)")
    ))
    .await;
    newline().await;

    for stage in &["Installing"] {
        for &(package_name, ref package_version) in &chosen_packages {
            let sleep_length = rng.gen_range(100..2000);

            print(format!(
                "  - {stage} {package_name} ({package_version}): Loading from cache",
                stage = stage,
                package_name = Paint::green(package_name),
                package_version = Paint::yellow(package_version)
            ))
            .await;
            newline().await;

            csleep(sleep_length).await;

            if appconfig.should_exit() {
                return;
            }
        }
    }
    print(format!("{text}", text = Paint::green("Writing lock file"))).await;
    newline().await;
    print(format!(
        "{text}",
        text = Paint::green("Generating autoload files")
    ))
    .await;
    newline().await;
}
