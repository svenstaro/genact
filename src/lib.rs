pub mod args;
mod data;
mod generators;
mod io;
pub mod modules;

use std::sync::atomic::{AtomicBool, AtomicU32};

use async_std::sync::Mutex;
use instant::Instant;
use rand::prelude::*;

use args::AppConfig;
use modules::{Module, ALL_MODULES};

lazy_static::lazy_static! {
    pub static ref CTRLC_PRESSED: AtomicBool = AtomicBool::new(false);
    pub static ref SPEED_FACTOR: Mutex<f32> = Mutex::new(1.0);
    pub static ref STARTED_AT: Instant = Instant::now();
    pub static ref MODULES_RAN: AtomicU32 = AtomicU32::new(0);
}

pub async fn run(appconfig: AppConfig) {
    let mut rng = thread_rng();
    let selected_modules: Vec<&Box<dyn Module>> = appconfig
        .modules
        .iter()
        .map(|m| &ALL_MODULES[m.as_str()])
        .collect();
    loop {
        let choice = selected_modules.choose(&mut rng).unwrap();
        choice.run(&appconfig).await;

        #[cfg(not(target_arch = "wasm32"))]
        {
            use std::sync::atomic::Ordering;
            MODULES_RAN.fetch_add(1, Ordering::SeqCst);

            if appconfig.should_exit() {
                exit_handler();
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn exit_handler() {
    println!("Saving work to disk...");
    std::process::exit(0);
}
