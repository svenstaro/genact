/// Module that pretends to delete Docker images
use rand::prelude::*;
use rand::Rng;

use crate::utils::csleep;
use crate::utils::gen_sha256_string;
use crate::DOCKER_LIST;
use crate::parse_args::AppConfig;


pub fn run(appconfig: &AppConfig) {
    let mut rng = thread_rng();
    let num_packages = rng.gen_range(20, 100);
    // Choose `num_packages` packages, non-repeating and in random order
    let chosen_packages: Vec<_> = DOCKER_LIST.choose_multiple(&mut rng, num_packages).collect();

    for _stage in &["Deleting"] {
        for &package_name in &chosen_packages {
            let sleep_length = rng.gen_range(500, 5000);

            println!(
                "Untagged: {package_name}",
                package_name = package_name
            );
            println!(
                "Untagged: {package_name}@sha256:{hash}",
                package_name = package_name,
                hash = gen_sha256_string(&mut rng)
            );

            let num_hashes = rng.gen_range(10, 30);
            let mut index = 0;
            while index < num_hashes {
                println!(
                    "Deleted: sha256:{hash}", 
                    hash = gen_sha256_string(&mut rng)
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
