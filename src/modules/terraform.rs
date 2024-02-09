//! Pretend to run Terraform
use async_trait::async_trait;
use rand::prelude::*;
use yansi::Paint;

use crate::args::AppConfig;
use crate::data::{TERRAFORM_AWS_RESOURCES_LIST, TERRAFORM_IDS_LIST};
use crate::io::{csleep, newline, print};
use crate::modules::Module;

pub struct Terraform;

async fn bold(msg: &str) {
    print(format!(
        "{}",
        Paint::new(msg).bold()
    ))
    .await;
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

        print(format!(
            "Acquiring state lock. This may take a few moments...\r\n",
        ))
        .await;
        csleep(500).await;

        let mut added = 0;
        let mut changed = 0;
        let mut destroyed = 0;

        loop {
            let resource = TERRAFORM_AWS_RESOURCES_LIST.iter().choose(&mut rng).unwrap();
            let id = TERRAFORM_IDS_LIST.iter().choose(&mut rng).unwrap();
            let secs = rng.gen_range(1..99);

            match rng.gen_range(0..9) {
                0 => { bold(format!("{resource}.{id}: Refreshing state... [id={id}]").as_ref()).await; },
                1 => { bold(format!("{resource}.{id}: Creating...").as_ref()).await; },
                2 => { bold(format!("{resource}.{id}: Creation complete after {secs}s [id={id}]").as_ref()).await; added += 1; },
                3 => { bold(format!("{resource}.{id}: Still creating... [{secs}0s elapsed]").as_ref()).await; },
                4 => { bold(format!("{resource}.{id}: Modifying... [id={id}]").as_ref()).await; },
                5 => { bold(format!("{resource}.{id}: Still modifying... [id={id}, {secs}0s elapsed]").as_ref()).await; },
                6 => { bold(format!("{resource}.{id}: Modifications complete after {secs}s [id={id}]").as_ref()).await; changed += 1; },
                7 => { bold(format!("{resource}.{id}: Destroying... [id={id}]").as_ref()).await; },
                8 => { bold(format!("{resource}.{id}: Destruction complete after {secs}s").as_ref()).await; destroyed += 1; },
                _ => { unreachable!(); },
            };
            csleep(rng.gen_range(100..2000)).await;

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
