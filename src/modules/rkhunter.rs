//! Pretend to detect rootkits using rkhunter
use async_trait::async_trait;
use chrono::Utc;
use rand::seq::{IndexedRandom, IteratorRandom};
use rand::{Rng, rng};
use yansi::Paint;

use crate::args::AppConfig;
use crate::data::{RKHUNTER_CHECKS_LIST, RKHUNTER_ROOTKITS_LIST, RKHUNTER_TASKS_LIST};
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
        let mut rng = rng();
        let check_positive_probability = 0.05;

        print(format!(
            "Running Rootkit Hunter version {version} on localhost\r\n",
            version = gen_package_version(&mut rng)
        ))
        .await;
        newline().await;
        csleep(500).await;
        print(format!(
            "Info: Start date is {date}",
            date = Utc::now().format("%a %d %b %Y %I:%M:%S %p %Z\r\n")
        ))
        .await;
        newline().await;
        print("Info: Detected operating system is 'Linux'\r\n").await;
        print(format!(
            "Found O/S name: Ubuntu {version}",
            version = gen_package_version(&mut rng)
        ))
        .await;
        csleep(500).await;
        print("Info: Environment shell is /bin/bash; rkhunter is using dash\r\n").await;
        print("Info: Using configuration file '/etc/rkhunter.conf'\r\n").await;
        print("Info: Installation directory is '/usr'\r\n").await;
        print("Info: Using language 'en'\r\n").await;
        print("Info: Using '/var/lib/rkhunter/db' as the database directory\r\n").await;
        print("Info: Using '/usr/share/rkhunter/scripts' as the support script directory\r\n")
            .await;
        print("Info: Using '/var/lib/rkhunter/db' as the database directory\r\n").await;
        print("Info: Using '/usr/share/rkhunter/scripts' as the support script directory\r\n")
            .await;
        print("Info: Using '/usr/local/sbin /usr/local/bin /usr/sbin /usr/bin /sbin /bin /usr/games /usr/local/games /snap/bin /usr/libexec' as the command directories\r\n").await;
        print("Info: Using '/var/lib/rkhunter/tmp' as the temporary directory\r\n").await;
        print("Info: No mail-on-warning address configured\r\n").await;
        newline().await;
        csleep(500).await;
        print("Checking if the O/S has changed since last time...\r\n").await;
        csleep(500).await;
        print("Info: Nothing seems to have changed.\r\n").await;
        print("Info: Locking is not being used\r\n").await;
        newline().await;
        csleep(500).await;
        print("Starting system checks...\r\n").await;
        newline().await;

        loop {
            let task = RKHUNTER_TASKS_LIST.iter().choose(&mut rng).unwrap();
            print(format!("{task}\r\n")).await;

            let is_rootkit = rng.random_bool(0.5);
            let rk_pad = if is_rootkit { "  " } else { "" };
            let rootkit = RKHUNTER_ROOTKITS_LIST.iter().choose(&mut rng).unwrap();
            if is_rootkit {
                print(format!("  Checking for {rootkit}...\r\n")).await;
            }

            let mut rootkit_found = false;
            let num_checks = rng.random_range(2..30);
            let mut checks: Vec<&&str> = RKHUNTER_CHECKS_LIST
                .choose_multiple(&mut rng, num_checks)
                .collect();

            checks.sort();

            // Calculate the right padding for checks to properly align the check status, with a
            // minimum width of 40 characters
            let mut check_pad = 40;
            for &check in &checks {
                check_pad = if check.len() > check_pad {
                    check.len()
                } else {
                    check_pad
                }
            }

            for &check in &checks {
                csleep(rng.random_range(200..1000)).await;
                // Specify if a check was positive; if yes also set the rootkit to have been found
                let check_positive = rng.random_bool(check_positive_probability);
                if check_positive {
                    rootkit_found = true;
                }

                // Prepare check and status
                let mut check_status = if check_positive {
                    "Found".red()
                } else {
                    "Not found".resetting()
                };
                if rng.random_bool(0.01) {
                    check_status = "Skipped".resetting();
                }

                print(format!(
                    "{rk_pad}  {check:<check_pad$} [ {check_status} ]\r\n"
                ))
                .await;
            }

            if is_rootkit {
                check_pad += 2;
                print(format!(
                    "  {rootkit:<check_pad$} [ {status} ]\r\n",
                    status = if rootkit_found {
                        "Found".red()
                    } else {
                        "Not found".resetting()
                    }
                ))
                .await;
            }

            newline().await;
            csleep(500).await;

            if appconfig.should_exit() {
                return;
            }
        }
    }
}
