#[cfg(not(target_arch = "wasm32"))]
use structopt::StructOpt;

use crate::modules::ALL_MODULES;

#[cfg(not(target_arch = "wasm32"))]
fn parse_speed_factor(s: &str) -> Result<f32, String> {
    let value_as_float = s.parse::<f32>().map_err(|e| e.to_string())?;
    if value_as_float < 0.01 {
        return Err("Speed factor must be larger than 0.01".to_string());
    }
    Ok(value_as_float)
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_min_1(s: &str) -> Result<u32, String> {
    let value_as_u32 = s.parse::<u32>().map_err(|e| e.to_string())?;
    if value_as_u32 == 0 {
        return Err("Must be larger than 0".to_string());
    }
    Ok(value_as_u32)
}

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
    #[structopt(short, long, possible_values = &ALL_MODULES.keys().cloned().collect::<Vec<_>>())]
    pub modules: Vec<String>,

    /// Global speed factor
    #[structopt(short, long, default_value = "1", parse(try_from_str = parse_speed_factor))]
    pub speed_factor: f32,

    /// Exit after running for this long (format example: 2h10min)
    #[structopt(long, parse(try_from_str = humantime::parse_duration))]
    pub exit_after_time: Option<instant::Duration>,

    /// Exit after running this many modules
    #[structopt(long, parse(try_from_str = parse_min_1))]
    pub exit_after_modules: Option<u32>,
}

#[cfg(target_arch = "wasm32")]
pub struct AppConfig {
    /// Run only these modules
    pub modules: Vec<String>,

    /// Global speed factor
    pub speed_factor: f32,
}

impl AppConfig {
    /// Check whether it's time to stop running.
    pub fn should_exit(&self) -> bool {
        // Check whether CTRL-C was pressed.
        #[cfg(not(target_arch = "wasm32"))]
        {
            use crate::{MODULES_RAN, STARTED_AT};
            use std::sync::atomic::Ordering;

            // Check if maximum running time is exceeded.
            if let Some(eat) = self.exit_after_time {
                if STARTED_AT.elapsed() > eat {
                    return true;
                }
            }

            // Check if maximum number of module runs has been reached.
            if let Some(eam) = self.exit_after_modules {
                if MODULES_RAN.load(Ordering::SeqCst) >= eam {
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
        args.modules = ALL_MODULES.keys().map(|m| m.to_string()).collect();
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
    let mut pairs = parsed_url.query_pairs();
    let filtered_modules = pairs.filter(|&(ref k, _)| k == "module");
    for (_, query_val) in filtered_modules {
        let actual_val = &&*query_val;
        if ALL_MODULES.keys().any(|x| x == actual_val) {
            temp_modules.push(actual_val.to_string());
        }
    }
    let speed_factor: f32 = pairs
        .find(|&(ref k, _)| k == "speed-factor")
        .map(|(_, v)| v.parse::<f32>().unwrap_or(1.0))
        .unwrap_or(1.0);

    let modules_to_run = if temp_modules.is_empty() {
        ALL_MODULES.keys().map(|m| m.to_string()).collect()
    } else {
        temp_modules
    };

    AppConfig {
        modules: modules_to_run,
        speed_factor,
    }
}
