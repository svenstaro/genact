//! Module that pretends to build Docker images
use rand::prelude::*;
use rand::Rng;

use crate::io::{csleep, dprint};
use crate::generators::gen_hex_string;
use crate::data::DOCKER_PACKAGES_LIST;
use crate::data::DOCKER_TAGS_LIST;
use crate::args::AppConfig;

use crate::modules::bootlog;
use crate::modules::botnet;
use crate::modules::cargo;
use crate::modules::cc;
use crate::modules::composer;
use crate::modules::cryptomining;
use crate::modules::download;
use crate::modules::kernel_compile;
use crate::modules::memdump;
use crate::modules::mkinitcpio;
use crate::modules::simcity;
use crate::modules::weblog;

pub fn get_signature() -> &'static str {
    return &"docker build -f Dockerfile";
}

pub async fn run(appconfig: &AppConfig) {
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
        ).await;

        let remaining_size = target_size - current_size;
        if remaining_size <= 5.0 {
            current_size += 5.0;
        } else {
            current_size += rng.gen_range(5.0, 30.0);
        }

        if appconfig.should_exit() {
            return;
        }

        csleep(200).await;
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
            instruction = ["RUN", get_module_signature(command)].join(" ")
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
                "bootlog" => bootlog::run(&docker_appconfig).await,
                "botnet" => botnet::run(&docker_appconfig).await,
                "cargo" => cargo::run(&docker_appconfig).await,
                "cryptomining" => cryptomining::run(&docker_appconfig).await,
                "simcity" => simcity::run(&docker_appconfig).await,
                "mkinitcpio" => mkinitcpio::run(&docker_appconfig).await,
                "cc" => cc::run(&docker_appconfig).await,
                "download" => download::run(&docker_appconfig).await,
                "memdump" => memdump::run(&docker_appconfig).await,
                "composer" => composer::run(&docker_appconfig).await,
                "kernel_compile" => kernel_compile::run(&docker_appconfig).await,
                "weblog" => weblog::run(&docker_appconfig).await,
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
        csleep(rng.gen_range(300, 1000)).await;
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
        "bootlog",
        "botnet",
        "cargo",
        "cc",
        "composer",
        "cryptomining",
        "simcity",
        "download",
        "memdump",
        "mkinitcpio",
        "kernel_compile",
        "weblog",
    ];

    let rand_choice: &str = available_modules.choose(&mut rng).unwrap();
    return rand_choice;
}

fn get_module_signature(choice: &str) -> &str {
    return match choice {
        "bootlog" => bootlog::get_signature(),
        "botnet" => botnet::get_signature(),
        "cargo" => cargo::get_signature(),
        "cryptomining" => cryptomining::get_signature(),
        "simcity" => simcity::get_signature(),
        "mkinitcpio" => mkinitcpio::get_signature(),
        "cc" => cc::get_signature(),
        "download" => download::get_signature(),
        "memdump" => memdump::get_signature(),
        "composer" => composer::get_signature(),
        "kernel_compile" => kernel_compile::get_signature(),
        "weblog" => weblog::get_signature(),
        _ => panic!("Unknown module!"),
    }
}
