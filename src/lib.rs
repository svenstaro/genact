pub mod args;
mod data;
mod generators;
mod io;
mod modules;

pub static ALL_MODULES: &[&str] = &[
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

use instant::Instant;
use rand::prelude::*;
use std::sync::atomic::AtomicBool;

use args::AppConfig;

lazy_static::lazy_static! {
    pub static ref CTRLC_PRESSED: AtomicBool = AtomicBool::new(false);
}

lazy_static::lazy_static! {
    pub static ref STARTED_AT: Instant = Instant::now();
}

pub async fn run(appconfig: AppConfig) {
    let mut rng = thread_rng();
    loop {
        let choice: &str = appconfig.modules.choose(&mut rng).unwrap();
        match choice {
            "bootlog" => modules::bootlog::run(&appconfig).await,
            "botnet" => modules::botnet::run(&appconfig).await,
            "cargo" => modules::cargo::run(&appconfig).await,
            "cryptomining" => modules::cryptomining::run(&appconfig).await,
            "simcity" => modules::simcity::run(&appconfig).await,
            "mkinitcpio" => modules::mkinitcpio::run(&appconfig).await,
            "cc" => modules::cc::run(&appconfig).await,
            "download" => modules::download::run(&appconfig).await,
            "docker" => modules::docker::run(&appconfig).await,
            "memdump" => modules::memdump::run(&appconfig).await,
            "composer" => modules::composer::run(&appconfig).await,
            "kernel_compile" => modules::kernel_compile::run(&appconfig).await,
            "weblog" => modules::weblog::run(&appconfig).await,
            _ => panic!("Unknown module '{}'!", choice),
        }

        #[cfg(not(target_arch = "wasm32"))]
        if appconfig.should_exit() {
            exit_handler();
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn exit_handler() {
    println!("Saving work to disk...");
    std::process::exit(0);
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// Called when the wasm module is instantiated
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    use std::panic;
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let appconfig = args::parse_args();
    run(appconfig).await;
    Ok(())
}
