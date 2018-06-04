/// Module that pretends to boot a system.
use rand::{thread_rng, Rng};
use yansi::Paint;

use parse_args::AppConfig;
use utils::{csleep, dprint};
use BOOTLOG_LIST;

pub fn run(appconfig: &AppConfig) {
    let mut rng = thread_rng();
    let num_lines = rng.gen_range(50, 200);
    let mut burst_mode = false;
    let mut count_burst_lines = 0;

    for _ in 1..num_lines {
        let choice = rng.choose(&BOOTLOG_LIST).unwrap_or(&"");
        let mut line_sleep_length = rng.gen_range(10, 1000);
        let mut char_sleep_length = 5;
        let burst_lines = rng.gen_range(10, 50);

        if burst_mode && count_burst_lines < burst_lines {
            line_sleep_length = 30;
            char_sleep_length = 0;
        } else if count_burst_lines == burst_lines {
            burst_mode = false;
            count_burst_lines = 0;
        } else if !burst_mode {
            burst_mode = rng.gen_bool(1.0 / 20.0);
        }

        let is_error = rng.gen_bool(0.01);
        if is_error {
            dprint(format!("{}", Paint::red(format!("ERROR: {}", choice))), 10);
        } else {
            let has_bold_word = rng.gen_bool(0.1);
            if has_bold_word {
                let mut words: Vec<String> = choice.split_whitespace().map(String::from).collect();
                words[0] = format!("{}", Paint::new(&words[0]).bold());
                dprint(words.join(" ").to_string(), char_sleep_length);
            } else {
                dprint(choice.to_string(), char_sleep_length);
            }
        }

        println!();
        if burst_mode {
            count_burst_lines += 1;
        }

        csleep(line_sleep_length);

        if appconfig.should_exit() {
            return;
        }
    }
}
