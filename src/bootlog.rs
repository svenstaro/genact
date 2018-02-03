/// Module that pretends to boot a system.
use rand::{thread_rng, Rng};
use yansi::Paint;

use parse_args::AppConfig;
use utils::{csleep, dprint};
use BOOTLOG_LIST;

pub fn run(appconfig: &AppConfig) {
    let mut rng = thread_rng();
    let num_lines = rng.gen_range(50, 200);

    for _ in 1..num_lines {
        let choice = rng.choose(&BOOTLOG_LIST).unwrap_or(&"");
        let sleep_length = rng.gen_range(10, 1000);

        let is_error = rng.gen_weighted_bool(100);
        if is_error {
            dprint(format!("{}", Paint::red(format!("ERROR: {}", choice))), 10);
        } else {
            let has_bold_word = rng.gen_weighted_bool(10);
            if has_bold_word {
                let mut words: Vec<String> = choice.split_whitespace().map(String::from).collect();
                words[0] = format!("{}", Paint::new(&words[0]).bold());
                dprint(format!("{}", words.join(" ")), 5);
            } else {
                dprint(format!("{}", choice), 5);
            }
        }

        println!();
        csleep(sleep_length);

        if appconfig.should_exit() {
            return;
        }
    }
}
