/// Module that pretends to delete Docker images
use rand::prelude::*;
use rand::Rng;

use crate::parse_args::AppConfig;
use crate::utils::csleep;
use crate::utils::gen_hex_string;
use crate::DOCKER_PACKAGES_LIST;
use crate::DOCKER_TAGS_LIST;

pub fn run(appconfig: &AppConfig) {
    let mut rng = thread_rng();
    let num_packages = rng.gen_range(20, 100);
    // Choose `num_packages` packages, non-repeating and in random order
    let chosen_packages: Vec<_> = DOCKER_PACKAGES_LIST
        .choose_multiple(&mut rng, num_packages)
        .collect();

    for &package_name in &chosen_packages {
        let sleep_length = rng.gen_range(500, 5000);
        let package_tag: &&str = DOCKER_TAGS_LIST.choose(&mut rng).unwrap();

        println!(
            "Untagged: {package_name}:{package_tag}",
            package_name = package_name,
            package_tag = package_tag,
        );
        println!(
            "Untagged: {package_name}:{package_tag}@sha256:{hash}",
            package_name = package_name,
            package_tag = package_tag,
            hash = gen_hex_string(&mut rng, 64)
        );

        let num_hashes = rng.gen_range(10, 30);
        let mut index = 0;
        while index < num_hashes {
            println!(
                "Deleted: sha256:{hash}",
                hash = gen_hex_string(&mut rng, 64)
            );

            index += 1;
        }

        csleep(sleep_length);

        if appconfig.should_exit() {
            return;
        }
    }
}
