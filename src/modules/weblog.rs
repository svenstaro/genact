//! Pretend to tail a web server log
use async_trait::async_trait;
use chrono::prelude::*;
use fake::faker::internet::en::*;
use fake::faker::lorem::en::*;
use fake::Fake;
use rand::prelude::*;

use crate::args::AppConfig;
use crate::data::{EXTENSIONS_LIST, PACKAGES_LIST};
use crate::generators::gen_file_path;
use crate::io::{csleep, newline, print};
use crate::modules::Module;

static HTTP_CODES: &[u16] = &[200, 201, 400, 401, 403, 404, 500, 502, 503];

pub struct Weblog;

#[async_trait(?Send)]
impl Module for Weblog {
    fn name(&self) -> &'static str {
        "weblog"
    }

    fn signature(&self) -> String {
        "tail -f /var/log/nginx/access.log".to_string()
    }

    async fn run(&self, appconfig: &AppConfig) {
        let mut rng = thread_rng();
        let num_lines = rng.gen_range(50..200);
        let mut burst_mode = false;
        let mut count_burst_lines = 0;

        for _ in 1..num_lines {
            let ip = if rng.gen_bool(0.5) {
                IPv4().fake()
            } else {
                IPv6().fake::<String>().to_lowercase()
            };
            let date = Local::now().format("%e/%b/%Y:%T %z");
            let method = "GET";
            let dir_candidates: Vec<String> = Words(20..21).fake();
            let path = gen_file_path(&mut rng, &PACKAGES_LIST, EXTENSIONS_LIST, &dir_candidates);
            let http_code = HTTP_CODES.choose(&mut rng).unwrap_or(&200);
            let size = rng.gen_range(99..5_000_000);
            let referrer = "-";
            let user_agent: String = UserAgent().fake();
            let line = format!(
                "{ip} - - [{date}] \"{method} {path} HTTP/1.0\" {http_code} {size} \"{referrer}\" \"{user_agent}\"",
            );
            let mut line_sleep_length = rng.gen_range(10..1000);
            let burst_lines = rng.gen_range(10..50);

            if burst_mode && count_burst_lines < burst_lines {
                line_sleep_length = 30;
            } else if count_burst_lines == burst_lines {
                burst_mode = false;
                count_burst_lines = 0;
            } else if !burst_mode {
                burst_mode = rng.gen_bool(1.0 / 20.0);
            }

            print(line.to_string()).await;

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
