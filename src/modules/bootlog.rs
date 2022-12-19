//! Pretend to boot a system
use async_trait::async_trait;
use rand::prelude::*;
use yansi::Paint;

use crate::args::AppConfig;
use crate::data::BOOTLOG_LIST;
use crate::io::{csleep, dprint, newline};
use crate::modules::Module;

pub struct Bootlog;

#[async_trait(?Send)]
impl Module for Bootlog {
    fn name(&self) -> &'static str {
        "bootlog"
    }

    fn signature(&self) -> String {
        "bcdedit /set {current} bootlog Yes && shutdown /r".to_string()
    }

    async fn run(&self, appconfig: &AppConfig) {
        let mut rng = thread_rng();
        let num_lines = rng.gen_range(50..200);
        let mut burst_mode = false;
        let mut count_burst_lines = 0;

        for _ in 1..num_lines {
            let choice = BOOTLOG_LIST.choose(&mut rng).unwrap_or(&"");
            let mut line_sleep_length = rng.gen_range(10..1000);
            let mut char_sleep_length = 5;
            let burst_lines = rng.gen_range(10..50);

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
                dprint(format!("{}", Paint::red(format!("ERROR: {choice}"))), 10).await;
            } else {
                let has_bold_word = rng.gen_bool(0.1);
                if has_bold_word {
                    let mut words: Vec<String> =
                        choice.split_whitespace().map(String::from).collect();
                    words[0] = format!("{}", Paint::new(&words[0]).bold());
                    dprint(words.join(" ").to_string(), char_sleep_length).await;
                } else {
                    dprint(choice.to_string(), char_sleep_length).await;
                }
            }

            newline().await;
            if burst_mode {
                count_burst_lines += 1;
            }

            csleep(line_sleep_length).await;

            if appconfig.should_exit() {
                return;
            }
        }
    }
}
