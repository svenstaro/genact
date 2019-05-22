/// Module that pretends to delete Docker images
use rand::prelude::*;
use rand::distributions::{Distribution};
use rand::Rng;
use rand::distributions::Alphanumeric;
use sha2::{Sha256, Digest};

use crate::utils::csleep;
use crate::DOCKER_LIST;
use crate::parse_args::AppConfig;

fn gen_hash() -> String {
    let mut rng = rand::thread_rng();
    let mut sha256 = Sha256::new();
    sha256.input(
        rng.sample_iter(&Alphanumeric)
        .take(30)
        .collect::<String>()
    );
    return sha256.result();
}

pub fn run(appconfig: &AppConfig) {
    let mut rng = thread_rng();
    let num_packages = rng.gen_range(20, 50);
    // Choose `num_packages` packages, non-repeating and in random order
    let chosen_packages: Vec<_> = DOCKER_LIST.choose_multiple(&mut rng, num_packages).collect();

    for stage in &["Deleting"] {
        for &package_name in &chosen_packages {
            let sleep_length = rng.gen_range(100, 2000);

            println!(
                "Untagged: {package_name}",
                package_name = package_name
            );
            println!(
                "Untagged: {package_name}@sha256:{hash}",
                package_name = package_name,
                hash = gen_hash()
            );

            let num_hashes = rng.gen_range(10, 30);
            let mut index = 0;
            while index < num_hashes {
                println!(
                    "Deleted: sha256:{hash}", 
                    hash = gen_hash()
                );

                index = index + 1;
            }

            csleep(sleep_length);

            if appconfig.should_exit() {
                return;
            }
        }
    }
}
