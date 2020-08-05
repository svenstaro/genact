mod bootlog;
mod botnet;
mod cargo;
mod cc;
mod composer;
mod cryptomining;
mod data;
mod docker;
mod download;
mod generators;
mod io;
mod kernel_compile;
mod memdump;
mod mkinitcpio;
mod parse_args;
mod simcity;
mod weblog;

static ALL_MODULES: &[&str] = &[
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

#[cfg(not(target_arch = "wasm32"))]
#[macro_use]
extern crate clap;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// Called when the wasm module is instantiated
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    use std::panic;
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let appconfig = parse_args::parse_args(ALL_MODULES);
    let mut rng = thread_rng();
    loop {
        let choice: &str = appconfig.modules.choose(&mut rng).unwrap();
        match choice {
            "bootlog" => bootlog::run(&appconfig).await,
            "botnet" => botnet::run(&appconfig).await,
            "cargo" => cargo::run(&appconfig).await,
            "cryptomining" => cryptomining::run(&appconfig).await,
            "simcity" => simcity::run(&appconfig).await,
            "mkinitcpio" => mkinitcpio::run(&appconfig).await,
            "cc" => cc::run(&appconfig).await,
            "download" => download::run(&appconfig).await,
            "docker" => docker::run(&appconfig).await,
            "memdump" => memdump::run(&appconfig).await,
            "composer" => composer::run(&appconfig).await,
            "kernel_compile" => kernel_compile::run(&appconfig).await,
            "weblog" => weblog::run(&appconfig).await,
            _ => panic!("Unknown module '{}'!", choice),
        }
        if appconfig.should_exit() {
            println!("Saving work to disk...");
            std::process::exit(0);
        }
    }
}

use rand::prelude::*;

use std::sync::atomic::AtomicBool;

lazy_static::lazy_static! {
    static ref CTRLC_PRESSED: AtomicBool = AtomicBool::new(false);
}

use anyhow::Result;

#[cfg(not(target_arch = "wasm32"))]
pub async fn main() -> Result<()> {
    use crate::parse_args::parse_args;
    let appconfig = parse_args(&ALL_MODULES);

    if appconfig.list_modules_and_exit {
        println!("Available modules:");
        for module in ALL_MODULES {
            println!("  {}", module);
        }
        std::process::exit(0);
    }

    use std::sync::atomic::Ordering;
    ctrlc::set_handler(move || {
        CTRLC_PRESSED.store(true, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let mut rng = thread_rng();
    loop {
        let choice: &str = appconfig.modules.choose(&mut rng).unwrap();
        match choice {
            "bootlog" => bootlog::run(&appconfig).await,
            "botnet" => botnet::run(&appconfig).await,
            "cargo" => cargo::run(&appconfig).await,
            "cryptomining" => cryptomining::run(&appconfig).await,
            "simcity" => simcity::run(&appconfig).await,
            "mkinitcpio" => mkinitcpio::run(&appconfig).await,
            "cc" => cc::run(&appconfig).await,
            "download" => download::run(&appconfig).await,
            "docker" => docker::run(&appconfig).await,
            "memdump" => memdump::run(&appconfig).await,
            "composer" => composer::run(&appconfig).await,
            "kernel_compile" => kernel_compile::run(&appconfig).await,
            "weblog" => weblog::run(&appconfig).await,
            _ => panic!("Unknown module '{}'!", choice),
        }
        if appconfig.should_exit() {
            println!("Saving work to disk...");
            std::process::exit(0);
        }
    }
}
