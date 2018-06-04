/// Module that pretends to tail a web server log.
use rand::{thread_rng, Rng};

use parse_args::AppConfig;
use chrono::prelude::*;
use EXTENSIONS_LIST;
use PACKAGES_LIST;
use utils::{csleep, dprint, gen_file_path};
static HTTP_CODES: &'static [u16] = &[200, 201, 400, 401, 403, 404, 500, 502, 503];

pub fn run(appconfig: &AppConfig) {
    let mut rng = thread_rng();
    let num_lines = rng.gen_range(50, 200);
    let mut burst_mode = false;
    let mut count_burst_lines = 0;

    for _ in 1..num_lines {
        let ip = if rng.gen_bool(0.5) {
            fake!(Internet.ipv4)
        } else {
            fake!(Internet.ipv6).to_lowercase()
        };
        let date = Local::now().format("%e/%b/%Y:%T %z");
        let method = "GET";
        let dir_candidates = fake!(Lorem.words(20));
        let path = gen_file_path(&mut rng, &PACKAGES_LIST, &EXTENSIONS_LIST, &dir_candidates);
        let http_code = rng.choose(HTTP_CODES).unwrap_or(&200);
        let size = fake!(Number.between(99, 5_000_000));
        let referrer = "-";
        let user_agent = fake!(Internet.user_agent);
        let line = format!(
            "{ip} - - [{date}] \"{method} {path} HTTP/1.0\" {http_code} {size} \"{referrer}\" \"{user_agent}\"",
            ip=ip,
            date=date,
            method=method,
            path=path,
            http_code=http_code,
            size=size,
            referrer=referrer,
            user_agent=user_agent
        );
        let mut line_sleep_length = rng.gen_range(10, 1000);
        let burst_lines = rng.gen_range(10, 50);

        if burst_mode && count_burst_lines < burst_lines {
            line_sleep_length = 30;
        } else if count_burst_lines == burst_lines {
            burst_mode = false;
            count_burst_lines = 0;
        } else if !burst_mode {
            burst_mode = rng.gen_bool(1.0 / 20.0);
        }

        dprint(line.to_string(), 0);

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
