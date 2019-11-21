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
extern crate rand;
extern crate regex;
extern crate url;
extern crate yansi;
#[macro_use]
extern crate fake;

mod bootlog;
mod botnet;
mod cargo;
mod cc;
mod composer;
mod cryptomining;
mod docker;
mod download;
mod kernel_compile;
mod memdump;
mod mkinitcpio;
mod simcity;
mod weblog;

mod parse_args;
mod utils;

use crate::parse_args::parse_args;
use rand::prelude::*;
use yansi::Paint;

static BOOTLOG: &str = include_str!("../data/bootlog.txt");
static CFILES: &str = include_str!("../data/cfiles.txt");
static PACKAGES: &str = include_str!("../data/packages.txt");
static COMPOSERS: &str = include_str!("../data/composer.txt");
static SIMCITY: &str = include_str!("../data/simcity.txt");
static BOOT_HOOKS: &str = include_str!("../data/boot_hooks.txt");
static OS_RELEASES: &str = include_str!("../data/os_releases.txt");
static DOCKER_PACKAGES: &str = include_str!("../data/docker_packages.txt");
static DOCKER_TAGS: &str = include_str!("../data/docker_tags.txt");

lazy_static! {
    static ref BOOTLOG_LIST: Vec<&'static str> = BOOTLOG.lines().collect();
    static ref CFILES_LIST: Vec<&'static str> = CFILES.lines().collect();
    static ref PACKAGES_LIST: Vec<&'static str> = PACKAGES.lines().collect();
    static ref COMPOSERS_LIST: Vec<&'static str> = COMPOSERS.lines().collect();
    static ref SIMCITY_LIST: Vec<&'static str> = SIMCITY.lines().collect();
    static ref BOOT_HOOKS_LIST: Vec<&'static str> = BOOT_HOOKS.lines().collect();
    static ref OS_RELEASES_LIST: Vec<&'static str> = OS_RELEASES.lines().collect();
    static ref DOCKER_PACKAGES_LIST: Vec<&'static str> = DOCKER_PACKAGES.lines().collect();
    static ref DOCKER_TAGS_LIST: Vec<&'static str> = DOCKER_TAGS.lines().collect();
}

static EXTENSIONS_LIST: &'static [&str] = &[
    "gif", "webm", "mp4", "html", "php", "md", "png", "jpg", "ogg", "mp3", "flac", "iso", "zip",
    "rar", "tar.gz", "tar.bz2", "tar.xz", "deb", "rpm", "exe",
];

static COMPRESSION_ALGORITHMS_LIST: &'static [&str] =
    &["gzip", "bzip2", "lzma", "xz", "lzop", "lz4"];

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
        "docker",
        "memdump",
        "mkinitcpio",
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
        })
        .expect("Error setting Ctrl-C handler");
    }

    let mut rng = thread_rng();
    loop {
        let choice: &str = appconfig.modules.choose(&mut rng).unwrap();
        match choice {
            "bootlog" => bootlog::run(&appconfig),
            "botnet" => botnet::run(&appconfig),
            "cargo" => cargo::run(&appconfig),
            "cryptomining" => cryptomining::run(&appconfig),
            "simcity" => simcity::run(&appconfig),
            "mkinitcpio" => mkinitcpio::run(&appconfig),
            "cc" => cc::run(&appconfig),
            "download" => download::run(&appconfig),
            "docker" => docker::run(&appconfig),
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
