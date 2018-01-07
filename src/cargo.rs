use rand::{thread_rng, Rng, ThreadRng};
use rand::distributions::{ChiSquared, IndependentSample};
use std::{thread, time};
use std::time::Instant;
use std::collections::HashMap;
use console::style;

fn gen_package(packages: &Vec<&str>, mut rng: &mut ThreadRng) -> (String, String) {
    let chi = ChiSquared::new(1.0);
    let package_version = format!("{major:.0}.{minor:.0}.{patch:.0}",
                          major=10.0 * chi.ind_sample(&mut rng),
                          minor=10.0 * chi.ind_sample(&mut rng),
                          patch=10.0 * chi.ind_sample(&mut rng));
    let package_name = rng.choose(&packages).unwrap_or(&"");
    (package_name.to_string(), package_version.to_string())
}

pub fn run() {
    let packages = include_str!("../data/packages.txt");
    let package_list: Vec<&str> = packages.lines().collect();

    let mut rng = thread_rng();
    let packages_count = rng.gen_range(10, 100);
    let mut chosen_packages = HashMap::new();

    while chosen_packages.len() < packages_count {
        let (package_name, package_version) = gen_package(&package_list, &mut rng);
        if !chosen_packages.contains_key(&package_name) {
            chosen_packages.insert(package_name, package_version);
        }
    }

    let now = Instant::now();
    for stage in vec!["Downloading", "Compiling"] {
        for (package_name, package_version) in &chosen_packages {
            let sleep_length = time::Duration::from_millis(rng.gen_range(100, 2000));

            println!("{stage:>12} {package_name} v{package_version}",
                     stage=style(stage).green().bold(),
                     package_name=package_name,
                     package_version=package_version);
            thread::sleep(sleep_length);
        }
    }
    let elapsed = now.elapsed();
    let seconds = elapsed.as_secs() as f32 + elapsed.subsec_nanos() as f32 / 1_000_000_000.0;
    println!("{stage:>12} release [optimized] target(s) in {seconds:.2} secs",
             stage=style("Finished").green().bold(),
             seconds=seconds);
}
