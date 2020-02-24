/// Module that pretends to build Docker images
use rand::prelude::*;
use rand::Rng;
use humantime::Duration;

use crate::utils::{csleep, dprint, gen_hex_string};
use crate::DOCKER_PACKAGES_LIST;
use crate::DOCKER_TAGS_LIST;
use crate::parse_args::AppConfig;
use crate::parse_args::parse_args;

use crate::bootlog;
use crate::botnet;
use crate::cargo;
use crate::cc;
use crate::composer;
use crate::cryptomining;
use crate::docker;
use crate::docker_build;
use crate::download;
use crate::kernel_compile;
use crate::memdump;
use crate::mkinitcpio;
use crate::simcity;
use crate::weblog;

pub fn run(appconfig: &AppConfig) {
    let mut rng = thread_rng();

    // Output the sending of the context to Docker
    let target_size = rng.gen_range(100.0, 1000.0);
    let mut current_size = 0.0;

    while current_size <= target_size {
        dprint(
            format!(
                "\rSending build context to Docker daemon  {current_size:>4.2}MB",
                current_size = current_size
            ),
            0,
        );

        let remaining_size = target_size - current_size;
        if remaining_size <= 5.0 {
            current_size += 5.0;
        } else {
            current_size += rng.gen_range(5.0, 30.0);
        }

        if appconfig.should_exit() {
            return;
        }

        csleep(200);
    }

    // Loop trough a set number of steps
    let total_steps = rng.gen_range(30, 100);
    let mut current_step = 1;

    while current_step <= total_steps {
        // Choose a random instruction
        let command = select_command();

        // Print the current step with the instruction to run
        println!(
            "\rStep {current_step}/{total_steps} : {instruction}",
            current_step = current_step,
            total_steps = total_steps,
            instruction = get_signature(command)
        );

        if rand::random() {
            println!(
                " ---> Using cache"
            );
        } else {
            println!(
                " ---> Running in {step_hash}",
                step_hash = gen_hex_string(&mut rng, 12),
            );

            let docker_appconfig = appconfig;
            match command {
                "docker_build_copy" => docker_build_copy(),
                "bootlog" => bootlog::run(&docker_appconfig),
                "botnet" => botnet::run(&docker_appconfig),
                "cargo" => cargo::run(&docker_appconfig),
                "cryptomining" => cryptomining::run(&docker_appconfig),
                "simcity" => simcity::run(&docker_appconfig),
                "mkinitcpio" => mkinitcpio::run(&docker_appconfig),
                "cc" => cc::run(&docker_appconfig),
                "download" => download::run(&docker_appconfig),
                // "docker" => docker::run(&docker_appconfig),
                // "docker_build" => docker_build::run(&docker_appconfig),
                "memdump" => memdump::run(&docker_appconfig),
                "composer" => composer::run(&docker_appconfig),
                "kernel_compile" => kernel_compile::run(&docker_appconfig),
                "weblog" => weblog::run(&docker_appconfig),
                _ => panic!("Unknown module!"),
            }
        }

        println!(
            " ---> {step_hash}",
            step_hash = gen_hex_string(&mut rng, 12),
        );

        if appconfig.should_exit() {
            return;
        }

        current_step += 1;
        csleep(rng.gen_range(300, 1000));
    }

    // Print the final lines
    let hash = gen_hex_string(&mut rng, 12);
    let image: &&str = DOCKER_PACKAGES_LIST.choose(&mut rng).unwrap();
    let image_tag: &&str = DOCKER_TAGS_LIST.choose(&mut rng).unwrap();

    println!(
        "Successfully built {hash}",
        hash = hash
    );

    println!(
        "Successfully tagged {image}:{tag}",
        image = image,
        tag = image_tag
    );

    if appconfig.should_exit() {
        return;
    }
}

fn select_command() -> &'static str {
    let mut rng = thread_rng();

    let available_modules = [
        "docker_build_copy",
        "bootlog",
        "botnet",
        "cargo",
        "cc",
        "composer",
        "cryptomining",
        "simcity",
        "download",
        // "docker",
        // "docker_build",
        "memdump",
        "mkinitcpio",
        "kernel_compile",
        "weblog",
    ];

    let rand_choice: &str = available_modules.choose(&mut rng).unwrap();
    return rand_choice;
}

fn get_signature(choice: &str) -> &str {
    return match choice {
        "docker_build_copy" => "COPY ./config /app/config",
        "bootlog" => "RUN ",
        "botnet" => "RUN ",
        "cargo" => "RUN ",
        "cryptomining" => "RUN ",
        "simcity" => "RUN ",
        "mkinitcpio" => "RUN ",
        "cc" => "RUN ",
        "download" => "RUN ",
        // "docker" => "RUN ",
        // "docker_build" => "RUN ",
        "memdump" => "RUN ",
        "composer" => "RUN ",
        "kernel_compile" => "RUN ",
        "weblog" => "RUN ",
        _ => panic!("Unknown module!"),
    }
}

fn docker_build_copy() {
    return
}
