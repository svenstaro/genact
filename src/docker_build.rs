/// Module that pretends to build Docker images
use rand::prelude::*;
use rand::Rng;
use crate::utils::{csleep, dprint, gen_hex_string};
use crate::DOCKER_PACKAGES_LIST;
use crate::DOCKER_TAGS_LIST;
use crate::parse_args::AppConfig;


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
        let instruction = "COPY ./config /app/config";

        // Print the current step with the instruction to run
        println!(
            "Step {current_step}/{total_steps} : {instruction}",
            current_step = current_step,
            total_steps = total_steps,
            instruction = instruction
        );

        if rand::random() {
            println!(
                " ---> Using cache"
            );
        } else {
            // Print the instruction output here
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
