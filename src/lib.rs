pub mod args;
mod data;
mod generators;
mod io;
pub mod modules;

use futures::lock::Mutex;
use instant::Instant;
use rand::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicU32};

use args::AppConfig;
use modules::{Module, ALL_MODULES};

lazy_static::lazy_static! {
    pub static ref CTRLC_PRESSED: AtomicBool = AtomicBool::new(false);
}

lazy_static::lazy_static! {
    pub static ref SPEED_FACTOR: Mutex<f32> = Mutex::new(1.0);
}

lazy_static::lazy_static! {
    pub static ref STARTED_AT: Instant = Instant::now();
}

lazy_static::lazy_static! {
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

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// Called when the wasm module is instantiated
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    use std::panic;
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let appconfig = args::parse_args();
    *SPEED_FACTOR.lock().await = appconfig.speed_factor;

    run(appconfig).await;
    Ok(())
}
