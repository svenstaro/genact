/// genact - A nonsense activity generator
///
/// Main module.

#[cfg(not(target_os = "emscripten"))]
#[macro_use]
extern crate clap;

#[cfg(target_os = "emscripten")]
extern crate emscripten_sys;

#[cfg(target_os = "emscripten")]
#[macro_use]
extern crate stdweb;

extern crate chrono;
extern crate pbr;
extern crate rand;
extern crate yansi;
extern crate url;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use rand::{thread_rng, Rng};
use yansi::Paint;

#[cfg(not(target_os = "emscripten"))]
use clap::{Arg, App};

#[cfg(target_os = "emscripten")]
use stdweb::web;

#[cfg(target_os = "emscripten")]
use url::Url;

mod bootlog;
mod cargo;
mod composer;
mod cc;
mod cryptomining;
mod download;
mod kernel_compile;
mod memdump;
mod utils;

static BOOTLOG: &str = include_str!("../data/bootlog.txt");
static CFILES: &str = include_str!("../data/cfiles.txt");
static PACKAGES: &str = include_str!("../data/packages.txt");
static COMPOSERS: &str = include_str!("../data/composer.txt");

lazy_static! {
    static ref BOOTLOG_LIST: Vec<&'static str> = BOOTLOG.lines().collect();
    static ref CFILES_LIST: Vec<&'static str> = CFILES.lines().collect();
    static ref PACKAGES_LIST: Vec<&'static str> = PACKAGES.lines().collect();
    static ref COMPOSERS_LIST: Vec<&'static str> = COMPOSERS.lines().collect();
}

#[cfg(not(target_os = "emscripten"))]
fn parse_args(all_modules: Vec<&str>) -> Vec<String> {
    use std::process;

    let app = App::new(crate_name!())
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
            .possible_values(&all_modules)
            .help("Run only these modules"),
            )
        .get_matches();

    if app.is_present("list") {
        println!("Available modules:");
        for module in all_modules {
            println!("  {}", module);
        }
        process::exit(0);
    }

    if app.is_present("modules") {
        app.values_of("modules")
            .unwrap()
            .map(|x| x.to_string())
            .collect()
    } else {
        all_modules.iter().map(|x| x.to_string()).collect()
    }
}

fn main() {
    Paint::enable_windows_ascii();

    let all_modules = vec![
        "bootlog",
        "cargo",
        "cc",
        "composer",
        "cryptomining",
        "download",
        "memdump",
        "kernel_compile",
        // "bruteforce",
        // "initialize",
        // "botnet",
        // "heartbeat",
    ];

    let modules_to_run: Vec<String>;

    #[cfg(not(target_os = "emscripten"))]
    {
        modules_to_run = parse_args(all_modules);
    }

    #[cfg(target_os = "emscripten")]
    {
        stdweb::initialize();
        let mut temp_modules = vec![];
        let location = web::document().location().unwrap();
        let parsed_url = Url::parse(&location.href()).unwrap();
        let pairs = parsed_url.query_pairs();
        let filtered = pairs.filter(|&(ref x, _)| x == "module");
        for (_, query_val) in filtered {
            let actual_val = &&*query_val;
            if all_modules.contains(actual_val) {
                temp_modules.push(actual_val.to_string());
            }
        }
        if temp_modules.is_empty() {
            modules_to_run = all_modules.iter().map(|x| x.to_string()).collect();
        } else {
            modules_to_run = temp_modules;
        }
    }

    #[cfg(target_os = "emscripten")]
    {
        utils::csleep(10);
    }

    let mut rng = thread_rng();
    loop {
        let choice: &str = rng.choose(&modules_to_run).unwrap();
        match choice {
            "bootlog" => bootlog::run(),
            "cargo" => cargo::run(),
            "cryptomining" => cryptomining::run(),
            "cc" => cc::run(),
            "download" => download::run(),
            "memdump" => memdump::run(),
            "composer" => composer::run(),
            "kernel_compile" => kernel_compile::run(),
            _ => panic!("Unknown module!"),
        }
    }
}
