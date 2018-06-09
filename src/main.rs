/// genact - A nonsense activity generator
///
/// Main module.
#[cfg(not(target_os = "emscripten"))]
#[macro_use]
extern crate clap;

#[cfg(not(target_os = "emscripten"))]
extern crate ctrlc;

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
extern crate url;
extern crate rand;
extern crate regex;
extern crate yansi;
#[macro_use]
extern crate fake;

mod bootlog;
mod botnet;
mod cargo;
mod composer;
mod cc;
mod cryptomining;
mod simcity;
mod download;
mod kernel_compile;
mod weblog;
mod memdump;
mod utils;
mod parse_args;

use rand::{thread_rng, Rng};
use yansi::Paint;
use parse_args::parse_args;

static BOOTLOG: &str = include_str!("../data/bootlog.txt");
static CFILES: &str = include_str!("../data/cfiles.txt");
static PACKAGES: &str = include_str!("../data/packages.txt");
static COMPOSERS: &str = include_str!("../data/composer.txt");
static SIMCITY: &str = include_str!("../data/simcity.txt");

lazy_static! {
    static ref BOOTLOG_LIST: Vec<&'static str> = BOOTLOG.lines().collect();
    static ref CFILES_LIST: Vec<&'static str> = CFILES.lines().collect();
    static ref PACKAGES_LIST: Vec<&'static str> = PACKAGES.lines().collect();
    static ref COMPOSERS_LIST: Vec<&'static str> = COMPOSERS.lines().collect();
    static ref SIMCITY_LIST: Vec<&'static str> = SIMCITY.lines().collect();
}

static EXTENSIONS_LIST: &'static [&str] = &["gif", "webm", "mp4", "html", "php", "md",
                                            "png", "jpg", "ogg", "mp3", "flac", "iso",
                                            "zip", "rar", "tar.gz", "tar.bz2", "tar.xz",
                                            "deb", "rpm", "exe"];

#[cfg(not(target_os = "emscripten"))]
use std::sync::atomic::AtomicBool;

#[cfg(not(target_os = "emscripten"))]
lazy_static! {
    static ref CTRLC_PRESSED: AtomicBool = AtomicBool::new(false);
}

fn main() {
    Paint::enable_windows_ascii();

    let all_modules = [
        "bootlog",
        "botnet",
        "cargo",
        "cc",
        "composer",
        "cryptomining",
        "simcity",
        "download",
        "memdump",
        "kernel_compile",
        "weblog",
        // "bruteforce",
        // "initialize",
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
            for module in &all_modules {
                println!("  {}", module);
            }
            process::exit(0);
        }

        use std::sync::atomic::Ordering;
        ctrlc::set_handler(move || {
            CTRLC_PRESSED.store(true, Ordering::SeqCst);
        }).expect("Error setting Ctrl-C handler");
    }

    let mut rng = thread_rng();
    loop {
        let choice: &str = rng.choose(&appconfig.modules).unwrap();
        match choice {
            "bootlog" => bootlog::run(&appconfig),
            "botnet" => botnet::run(&appconfig),
            "cargo" => cargo::run(&appconfig),
            "cryptomining" => cryptomining::run(&appconfig),
            "simcity" => simcity::run(&appconfig),
            "cc" => cc::run(&appconfig),
            "download" => download::run(&appconfig),
            "memdump" => memdump::run(&appconfig),
            "composer" => composer::run(&appconfig),
            "kernel_compile" => kernel_compile::run(&appconfig),
            "weblog" => weblog::run(&appconfig),
            _ => panic!("Unknown module!"),
        }
        #[cfg(not(target_os = "emscripten"))]
        {
            use std::process;
            if appconfig.should_exit() {
                println!("Saving work to disk...");
                process::exit(0);
            }
        }
    }
}
