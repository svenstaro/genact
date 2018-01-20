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
extern crate humantime;
#[macro_use]
extern crate lazy_static;
extern crate pbr;
extern crate rand;
extern crate url;
extern crate yansi;

mod bootlog;
mod cargo;
mod composer;
mod cc;
mod cryptomining;
mod download;
mod memdump;
mod utils;
mod parse_args;

use rand::{thread_rng, Rng};
use yansi::Paint;

#[cfg(target_os = "emscripten")]
use utils::csleep;

use parse_args::parse_args;

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
        // "bruteforce",
        // "initialize",
        // "botnet",
        // "heartbeat",
    ];

    #[cfg(target_os = "emscripten")]
    {
        stdweb::initialize();
    }

    let appconfig = parse_args(&all_modules);

    #[cfg(not(target_os = "emscripten"))]
    {
        use std::process;
        if appconfig.list_modules_and_exit {
            println!("Available modules:");
            for module in all_modules {
                println!("  {}", module);
            }
            process::exit(0);
        }
    }

    let mut rng = thread_rng();
    loop {
        let choice: &str = rng.choose(&appconfig.modules).unwrap();
        match choice {
            "bootlog" => bootlog::run(&appconfig),
            "cargo" => cargo::run(&appconfig),
            "cryptomining" => cryptomining::run(&appconfig),
            "cc" => cc::run(&appconfig),
            "download" => download::run(&appconfig),
            "memdump" => memdump::run(&appconfig),
            "composer" => composer::run(&appconfig),
            _ => panic!("Unknown module!"),
        }
        #[cfg(not(target_os = "emscripten"))]
        {
            use std::process;
            if appconfig.is_time_to_quit() {
                process::exit(0);
            }
        }
    }
}
