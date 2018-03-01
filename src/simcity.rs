use rand::{thread_rng, Rng};
use parse_args::AppConfig;
use utils::{dprint, csleep};
use yansi::Paint;

use SIMCITY_LIST;

pub fn run(appconfig: &AppConfig) {
    const SPINNERS: &[&str] = &["/", "-", "\\", "|"];
    const SPINNER_SLEEP: u64 = 50;
    const TEXT_SLEEP: u64 = 15;
    const MAX_SPINNER_LOOPS: u8 = 20;

    let mut rng = thread_rng();
    let mut simcity = "";

    for _ in 0..500 {
        let spinner_loops = rng.gen_range(1, MAX_SPINNER_LOOPS);

        // Message chosen from "data/simcity.txt"
        // Thanks https://gist.github.com/erikcox/7e96d031d00d7ecb1a2f
        let last_simcity = simcity;
        simcity = rng.choose(&SIMCITY_LIST).unwrap_or(&"");

        // Don't choose the same message twice in a row
        while simcity == last_simcity {
            // Select another message
            simcity = rng.choose(&SIMCITY_LIST).unwrap_or(&"");
        }

        // Choose a status/resolution per "task"
        let resolution_id = 1 + rng.gen::<u8>() % 100;
        let resolution = match resolution_id {
            1...4 => "FAIL",
            5...9 => "YES",
            10...14 => "SUCCESS",
            _ => "OK",
        };

        // Select a color
        let color_func = if resolution == "FAIL" {
            // Use red for FAIL
            Paint::red
        } else if resolution_id > 50 {
            // Use white most of the time
            Paint::white
        } else {
            let color_id = 1 + rng.gen::<u8>() % 20;
            match color_id {
                1...2 => Paint::red,
                3...4 => Paint::green,
                5...6 => Paint::cyan,
                7...10 => Paint::blue,
                _ => Paint::white,
            }
        };

        // Prepare and color the messages
        let simcity_msg = color_func(simcity);
        let resolution_msg = color_func(resolution);
        let dots = color_func("... ");
        let unchecked_checkbox = "[ ] ";
        let checked_checkbox = "[o] ";

        // Keep track of when the message is first printed
        let mut first = true;
        for i in 0..spinner_loops {
            for spinner in SPINNERS {
                // Output a message, with a checkbox in front and spinner behind
                let msg = simcity_msg.to_string() + "... " + spinner;
                if first {
                    dprint(unchecked_checkbox, 0);
                    dprint(msg, TEXT_SLEEP);
                    first = false;
                } else {
                    dprint(unchecked_checkbox, 0);
                    dprint(msg, 0);
                }
                // Wait a bit, then erase the line
                csleep(SPINNER_SLEEP);
                dprint("\r", 0);
            }
            if i == (spinner_loops - 1) {
                // End of loop, the line has been removed, conclude the status
                dprint(checked_checkbox, 10);
                dprint(simcity_msg.to_string(), 0);
                dprint(dots.to_string(), 0);
                dprint(resolution_msg.to_string(), 0);
            }
        }

        if appconfig.should_exit() {
            dprint("\nALL DONE\n", 0);
            return;
        }

        println!();
    }
}
