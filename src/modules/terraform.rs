//! Pretend to run Terraform
use async_trait::async_trait;
use rand::distributions::{Bernoulli, Distribution};
use rand::prelude::*;
use yansi::Paint;

use crate::args::AppConfig;
use crate::data::TERRAFORM_AWS_RESOURCES_LIST;
use crate::data::TERRAFORM_AZURE_RESOURCES_LIST;
use crate::data::TERRAFORM_GCP_RESOURCES_LIST;
use crate::data::TERRAFORM_IDS_LIST;
use crate::io::{csleep, newline, print};
use crate::modules::Module;

pub struct Terraform;

async fn bold(msg: &str) {
    print(format!("{}", Paint::new(msg).bold())).await;
    newline().await;
}

#[async_trait(? Send)]
impl Module for Terraform {
    fn name(&self) -> &'static str {
        "terraform"
    }

    fn signature(&self) -> String {
        "terraform --check".to_string()
    }

    async fn run(&self, appconfig: &AppConfig) {
        let mut rng = thread_rng();

        print("Acquiring state lock. This may take a few moments...\r\n").await;
        csleep(500).await;

        // Count how many resources has been added, changed, and destroyed
        let mut added = 0;
        let mut changed = 0;
        let mut destroyed = 0;

        // Start counting time
        let start = std::time::Instant::now();

        // Randomize the cloud provider
        let cloud = match rng.gen_range(0..3) {
            0 => "AWS",
            1 => "AZURE",
            2 => "GCP",
            _ => unreachable!(),
        };

        loop {
            // Based on the cloud provider, randomize a resource name
            let resource = match cloud {
                "AWS" => TERRAFORM_AWS_RESOURCES_LIST
                    .iter()
                    .choose(&mut rng)
                    .unwrap(),
                "AZURE" => TERRAFORM_AZURE_RESOURCES_LIST
                    .iter()
                    .choose(&mut rng)
                    .unwrap(),
                "GCP" => TERRAFORM_GCP_RESOURCES_LIST
                    .iter()
                    .choose(&mut rng)
                    .unwrap(),
                _ => unreachable!(),
            };

            // Randomize a resource ID
            let id = TERRAFORM_IDS_LIST.iter().choose(&mut rng).unwrap();

            // Count the time elapsed
            let secs = start.elapsed().as_secs();

            // Randomize a Terraform message to print
            match rng.gen_range(0..9) {
                0 => {
                    bold(format!("{resource}.{id}: Refreshing state... [id={id}]").as_ref()).await;
                }
                1 => {
                    bold(format!("{resource}.{id}: Creating...").as_ref()).await;
                }
                2 => {
                    bold(
                        format!("{resource}.{id}: Creation complete after {secs}s [id={id}]")
                            .as_ref(),
                    )
                    .await;
                    added += 1;
                }
                3 => {
                    bold(format!("{resource}.{id}: Still creating... [{secs}s elapsed]").as_ref())
                        .await;
                }
                4 => {
                    bold(format!("{resource}.{id}: Modifying... [id={id}]").as_ref()).await;
                }
                5 => {
                    bold(
                        format!("{resource}.{id}: Still modifying... [id={id}, {secs}s elapsed]")
                            .as_ref(),
                    )
                    .await;
                }
                6 => {
                    bold(
                        format!("{resource}.{id}: Modifications complete after {secs}s [id={id}]")
                            .as_ref(),
                    )
                    .await;
                    changed += 1;
                }
                7 => {
                    bold(format!("{resource}.{id}: Destroying... [id={id}]").as_ref()).await;
                }
                8 => {
                    bold(format!("{resource}.{id}: Destruction complete after {secs}s").as_ref())
                        .await;
                    destroyed += 1;
                }
                _ => {
                    unreachable!();
                }
            };

            // Sleep for a random time between 100ms and 2s based on Bernoulli distribution
            let dist = Bernoulli::new(0.5).unwrap();
            if dist.sample(&mut rng) {
                csleep(rng.gen_range(100..3000)).await;
            }

            // Check if program wants to exit and print the final message
            if appconfig.should_exit() {
                print(format!(
                    "\r\nApply complete! Resources: {added} added, {changed} changed, {destroyed} destroyed.\r\n"
                ))
                .await;
                break;
            }
        }
        return;
    }
}
