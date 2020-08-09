#[cfg(not(target_arch = "wasm32"))]
use structopt::StructOpt;

use crate::ALL_MODULES;

#[cfg(not(target_arch = "wasm32"))]
#[derive(StructOpt)]
#[structopt(
    name = "genact",
    author,
    about,
    global_settings = &[structopt::clap::AppSettings::ColoredHelp],
)]
pub struct AppConfig {
    /// List available modules
    #[structopt(short, long = "list-modules")]
    pub list_modules_and_exit: bool,

    /// Run only these modules
    #[structopt(short, long, possible_values = &ALL_MODULES)]
    pub modules: Vec<String>,

    /// Exit after running for this long (format example: 2h10min)
    #[structopt(short, long, parse(try_from_str = humantime::parse_duration))]
    pub exit_after: Option<instant::Duration>,
}

#[cfg(target_arch = "wasm32")]
pub struct AppConfig {
    /// Run only these modules
    pub modules: Vec<String>,
}

impl AppConfig {
    /// Check whether it's time to stop running.
    pub fn should_exit(&self) -> bool {
        // Check whether CTRL-C was pressed.
        #[cfg(not(target_arch = "wasm32"))]
        {
            use crate::STARTED_AT;

            // Check if maximum running time is exceeded.
            if let Some(ea) = self.exit_after {
                if STARTED_AT.elapsed() > ea {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn parse_args() -> AppConfig {
    let mut args = AppConfig::from_args();

    if args.modules.is_empty() {
        args.modules = ALL_MODULES.iter().map(|x| x.to_string()).collect();
    };
    args
}

#[cfg(target_arch = "wasm32")]
pub fn parse_args() -> AppConfig {
    use url::Url;

    let mut temp_modules = vec![];
    let window = web_sys::window().expect("no global `window` exists");
    let location = window.location();
    let parsed_url = Url::parse(&location.href().unwrap()).unwrap();
    let pairs = parsed_url.query_pairs();
    let filtered = pairs.filter(|&(ref x, _)| x == "module");
    for (_, query_val) in filtered {
        let actual_val = &&*query_val;
        if ALL_MODULES.contains(actual_val) {
            temp_modules.push(actual_val.to_string());
        }
    }
    let modules_to_run = if temp_modules.is_empty() {
        ALL_MODULES.iter().map(|x| x.to_string()).collect()
    } else {
        temp_modules
    };

    AppConfig {
        modules: modules_to_run,
    }
}
