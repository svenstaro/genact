//! Print some Sim City loading screen status quips
use async_trait::async_trait;
use rand::prelude::*;
use yansi::Paint;

use crate::args::AppConfig;
use crate::data::SIMCITY_LIST;
use crate::io::{csleep, dprint, newline, print};
use crate::modules::Module;

pub struct Simcity;

#[async_trait(?Send)]
impl Module for Simcity {
    fn name(&self) -> &'static str {
        "mkinitcpio"
    }

    fn signature(&self) -> String {
        "./start-sumcity.sh".to_string()
    }

    async fn run(&self, appconfig: &AppConfig) {
        const SPINNERS: &[&str] = &["/", "-", "\\", "|"];
        const SPINNER_SLEEP: u64 = 50;
        const TEXT_SLEEP: u64 = 15;
        const MAX_SPINNER_LOOPS: u8 = 20;

        let mut rng = thread_rng();
        let mut simcity = "";

        for _ in 0..500 {
            let spinner_loops = rng.gen_range(1..MAX_SPINNER_LOOPS);

            // Message chosen from "data/simcity.txt"
            // Thanks https://gist.github.com/erikcox/7e96d031d00d7ecb1a2f
            let last_simcity = simcity;
            simcity = SIMCITY_LIST.choose(&mut rng).unwrap_or(&"");

            // Don't choose the same message twice in a row
            while simcity == last_simcity {
                // Select another message
                simcity = SIMCITY_LIST.choose(&mut rng).unwrap_or(&"");
            }

            // Choose a status/resolution per "task"
            let resolution_id = 1 + rng.gen::<u8>() % 100;
            let mut resolution = match resolution_id {
                1..=4 => "FAIL",
                5..=9 => "YES",
                10..=14 => "SUCCESS",
                _ => "OK",
            };

            // Prepare and color the messages
            let unchecked_checkbox = "[ ] ";
            let checked_checkbox = "[o] ";

            // Keep track of when the message is first printed
            let mut first = true;

            'outer: for _ in 0..spinner_loops {
                for spinner in SPINNERS {
                    // Output a message, with a checkbox in front and spinner behind
                    let msg = format!("{}... {}", simcity, spinner);

                    // on first print, text appears letter by letter
                    if first {
                        print(unchecked_checkbox).await;
                        dprint(msg, TEXT_SLEEP).await;
                        first = false;
                    } else {
                        print(unchecked_checkbox).await;
                        print(msg).await;
                    }
                    // Wait a bit, then erase the line
                    csleep(SPINNER_SLEEP).await;
                    print("\r").await;

                    // Don't wait until finished, exit both loops if that is requested
                    if appconfig.should_exit() {
                        resolution = "ABORTED";
                        break 'outer;
                    }
                }
            }

            // Select the color
            let color_func = if resolution == "FAIL" || resolution == "ABORTED" {
                // Use red for FAIL
                Paint::red
            } else if resolution_id > 50 {
                // Use white most of the time
                Paint::white
            } else {
                let color_id = 1 + rng.gen::<u8>() % 20;
                match color_id {
                    1..=2 => Paint::red,
                    3..=4 => Paint::green,
                    5..=6 => Paint::cyan,
                    7..=10 => Paint::blue,
                    _ => Paint::white,
                }
            };

            // End of loop, the line has been removed, conclude the status
            dprint(checked_checkbox, 10).await;
            print(color_func(format!("{}... {}", simcity, resolution)).to_string()).await;

            if appconfig.should_exit() {
                print("\nALL DONE\n").await;
                return;
            }

            newline().await;
        }
    }
}
