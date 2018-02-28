use humantime::Duration;

#[cfg(not(target_os = "emscripten"))]
use humantime::parse_duration;

use std::time::Instant;

#[cfg(not(target_os = "emscripten"))]
fn is_parse_duration_format(v: String) -> Result<(), String> {
    if parse_duration(&v).is_ok() {
        Ok(())
    } else {
        Err(String::from("Couldn't parse the time format"))
    }
}

pub struct AppConfig {
    pub list_modules_and_exit: bool,
    pub modules: Vec<String>,
    pub exit_after: Option<Duration>,
    pub started_at: Instant,
}

impl AppConfig {
    /// Check whether it's time to stop running.
    pub fn should_exit(&self) -> bool {
        // Check whether CTRL-C was pressed.
        #[cfg(not(target_os = "emscripten"))]
        {
            use std::sync::atomic::Ordering;
            use CTRLC_PRESSED;
            if CTRLC_PRESSED.load(Ordering::SeqCst) {
                return true;
            }
        }

        // Check if maximum running time is exceeded.
        if let Some(ea) = self.exit_after {
            if self.started_at.elapsed() > *ea {
                return true;
            }
        }
        false
    }
}

#[cfg(not(target_os = "emscripten"))]
pub fn parse_args(all_modules: &[&str]) -> AppConfig {
    use clap::{App, Arg};

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list-modules")
                .help("List available modules"),
        )
        .arg(
            Arg::with_name("modules")
                .short("m")
                .long("modules")
                .multiple(true)
                .value_name("MODULE")
                .takes_value(true)
                .possible_values(all_modules)
                .help("Run only these modules"),
        )
        .arg(
            Arg::with_name("exitafter")
                .short("e")
                .long("exitafter")
                .value_name("EXITAFTER")
                .takes_value(true)
                .validator(is_parse_duration_format)
                .help("Exit after running for this long (format example: 2h10min)"),
        )
        .get_matches();

    let list_modules_and_exit = matches.is_present("list");

    let modules_to_run = if matches.is_present("modules") {
        matches
            .values_of("modules")
            .unwrap()
            .map(|x| x.to_string())
            .collect()
    } else {
        all_modules.iter().map(|x| x.to_string()).collect()
    };

    let exit_after = value_t!(matches, "exitafter", Duration).ok();

    AppConfig {
        list_modules_and_exit,
        modules: modules_to_run,
        exit_after,
        started_at: Instant::now(),
    }
}

#[cfg(target_os = "emscripten")]
pub fn parse_args(all_modules: &[&str]) -> AppConfig {
    use stdweb::web;
    use url::Url;

    let mut temp_modules = vec![];
    let location = web::document().location().unwrap();
    let parsed_url = Url::parse(&location.href().unwrap()).unwrap();
    let pairs = parsed_url.query_pairs();
    let filtered = pairs.filter(|&(ref x, _)| x == "module");
    for (_, query_val) in filtered {
        let actual_val = &&*query_val;
        if all_modules.contains(actual_val) {
            temp_modules.push(actual_val.to_string());
        }
    }
    let modules_to_run = if temp_modules.is_empty() {
        all_modules.iter().map(|x| x.to_string()).collect()
    } else {
        temp_modules
    };

    AppConfig {
        list_modules_and_exit: false,
        modules: modules_to_run,
        exit_after: None,
        started_at: Instant::now(),
    }
}
