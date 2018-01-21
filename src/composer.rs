/// Module that pretends to install composer packages.

use rand::{thread_rng, Rng, ThreadRng};
use rand::distributions::{ChiSquared, IndependentSample};
use std::collections::HashMap;
use yansi::Paint;

use utils::csleep;
use COMPOSERS_LIST;
use parse_args::AppConfig;

fn gen_package(packages: &[&str], mut rng: &mut ThreadRng) -> (String, String) {
    let chi = ChiSquared::new(1.0);
    let package_version = format!(
        "{major:.0}.{minor:.0}.{patch:.0}",
        major = 10.0 * chi.ind_sample(&mut rng),
        minor = 10.0 * chi.ind_sample(&mut rng),
        patch = 10.0 * chi.ind_sample(&mut rng)
    );
    let package_name = rng.choose(packages).unwrap_or(&"");
    (package_name.to_string(), package_version.to_string())
}

pub fn run(appconfig: &AppConfig) {
    let mut rng = thread_rng();
    let num_packages = rng.gen_range(10, 100);
    let mut chosen_packages = HashMap::new();

    while chosen_packages.len() < num_packages {
        let (package_name, package_version) = gen_package(&COMPOSERS_LIST, &mut rng);
        chosen_packages
            .entry(package_name)
            .or_insert(package_version);
    }

    println!(
        "{text}",
        text = Paint::green("Loading composer repositories with package information")
    );
    println!(
        "{text}",
        text = Paint::green("Updating dependencies (including require-dev)")
    );

    for stage in &["Installing"] {
        for (package_name, package_version) in &chosen_packages {
            let sleep_length = rng.gen_range(100, 2000);

            println!(
                "  - {stage} {package_name} ({package_version}): Loading from cache",
                stage = stage,
                package_name = Paint::green(package_name),
                package_version = Paint::yellow(package_version)
            );

            csleep(sleep_length);

            if appconfig.is_time_to_quit() {
                return;
            }
        }
    }
    println!("{text}", text = Paint::green("Writing lock file"));
    println!("{text}", text = Paint::green("Generating autoload files"))
}
