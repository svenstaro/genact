//! Pretend to install composer packages
use async_trait::async_trait;
use chrono::Utc;
use rand::prelude::*;
use yansi::Paint;

use crate::args::AppConfig;
use crate::data::RKHUNTER_CHECKS_LIST;
use crate::data::RKHUNTER_CHECKS_SHORT_LIST;
use crate::data::RKHUNTER_INFOS_LIST;
use crate::data::RKHUNTER_ROOTKITS_LIST;
use crate::data::RKHUNTER_TASKS_LIST;
use crate::generators::gen_package_version;
use crate::io::{csleep, newline, print};
use crate::modules::Module;

pub struct RkHunter;

#[async_trait(? Send)]
impl Module for RkHunter {
    fn name(&self) -> &'static str {
        "rkhunter"
    }

    fn signature(&self) -> String {
        "rkhunter --check".to_string()
    }

    async fn run(&self, appconfig: &AppConfig) {
        let mut rng = thread_rng();
        let check_positive_probability = 0.05;
        let short_check_probability = 0.15;
        let info_probability = 0.05;

        print(format!(
            "Running Rootkit Hunter version {version} on localhost\r\n",
            version = gen_package_version(&mut rng)
        )).await;
        newline().await;
        print(format!(
            "Info: Start date is {date}",
            date = Utc::now().format("%a %d %b %Y %I:%M:%S %p %Z\r\n")
        )).await;
        newline().await;
        print("Info: Detected operating system is 'Linux'\r\n").await;
        print(format!(
            "Found O/S name: Ubuntu {version}",
            version = gen_package_version(&mut rng)
        )).await;
        print("Info: Environment shell is /bin/bash; rkhunter is using dash\r\n").await;
        print("Info: Using configuration file '/etc/rkhunter.conf'\r\n").await;
        print("Info: Installation directory is '/usr'\r\n").await;
        print("Info: Using language 'en'\r\n").await;
        print("Info: Using '/var/lib/rkhunter/db' as the database directory\r\n").await;
        print("Info: Using '/usr/share/rkhunter/scripts' as the support script directory\r\n").await;
        print("Info: Using language 'en'\r\n").await;

        newline().await;

        loop {
            let task = RKHUNTER_TASKS_LIST.iter().choose(&mut rng).unwrap();
            print(format!("{task}\r\n")).await;

            let is_rootkit = rng.gen_bool(0.5);
            let rk_pad = if is_rootkit { "  " } else { "" };
            let rootkit = RKHUNTER_ROOTKITS_LIST.iter().choose(&mut rng).unwrap();
            if is_rootkit {
                print(format!("  Checking for {rootkit}...\r\n")).await;
            }

            let mut rootkit_found = false;
            let num_checks = rng.gen_range(2..30);
            let checks: Vec<&&str> = RKHUNTER_CHECKS_LIST
                .choose_multiple(&mut rng, num_checks)
                .collect();

            // Calculate the right padding for checks to properly align the check status, with a
            // minimum width of 20 characters
            let mut check_pad = 20;
            for &check in &checks {
                check_pad = if check.len() > check_pad { check.len() } else { check_pad }
            }

            for &check in &checks {
                csleep(rng.gen_range(200..1000)).await;
                // Specify if a check was positive; if yes also set the rootkit to have been found
                let check_positive = rng.gen_bool(check_positive_probability);
                if check_positive {
                    rootkit_found = true;
                }

                // Prepare check and status
                let check_status = if check_positive { Paint::red("Found") } else { Paint::default("Not found") };
                print(format!("{rk_pad}  {check:<check_pad$} [ {check_status} ]\r\n")).await;
            }

            // Generate additional short checks with a probability of 15%
            if rng.gen_bool(short_check_probability) {

                let num_short_checks = 1..rng.gen_range(1..5);
                for _ in num_short_checks {
                    csleep(rng.gen_range(300..1000)).await;

                    // Specify if a check was positive with a chance of 10%
                    let check_positive = rng.gen_bool(check_positive_probability);
                    if check_positive {
                        rootkit_found = true;
                    }

                    // Prepare check and status
                    let check = RKHUNTER_CHECKS_SHORT_LIST.iter().choose(&mut rng).unwrap();
                    let mut check_status = if check_positive { Paint::red("Warning") } else { Paint::default("Ok") };
                    if rng.gen_bool(0.1) {
                        check_status = Paint::default("Skipped");
                    }

                    print(format!("{rk_pad}  {check:<check_pad$} [ {check_status} ]\r\n")).await;
                }
            }

            // Display final info line with probability of 5%
            if rng.gen_bool(info_probability) {
                csleep(rng.gen_range(300..600)).await;
                let info = RKHUNTER_INFOS_LIST.iter().choose(&mut rng).unwrap();
                print(format!("{rk_pad}  {info}\r\n")).await;
            }

            if is_rootkit {
                check_pad = check_pad + 2;
                print(format!(
                    "  {rootkit:<check_pad$} [ {status} ]\r\n",
                    status = if rootkit_found { Paint::red("Found") } else { Paint::default("Not found") })
                ).await;
            }

            newline().await;
            csleep(500).await;

            if appconfig.should_exit() {
                return;
            }
        }
    }
}
