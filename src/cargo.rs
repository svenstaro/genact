/// Module that pretends to run cargo to install rust packages.

use rand::{thread_rng, Rng, ThreadRng};
use rand::distributions::{ChiSquared, IndependentSample};
use std::time::Instant;
use std::collections::HashMap;
use yansi::Paint;

use utils::csleep;
use PACKAGES_LIST;
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
        let (package_name, package_version) = gen_package(&PACKAGES_LIST, &mut rng);
        chosen_packages
            .entry(package_name)
            .or_insert(package_version);
    }

    let now = Instant::now();
    for stage in &["Downloading", "Compiling"] {
        for (package_name, package_version) in &chosen_packages {
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
