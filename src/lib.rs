pub mod args;
mod data;
mod generators;
mod io;
pub mod modules;

use std::sync::LazyLock;
use std::sync::atomic::{AtomicBool, AtomicU32};

use tokio::sync::Mutex;

use instant::Instant;
use rand::rng;
use rand::seq::IndexedRandom;

use crate::args::AppConfig;
use crate::modules::{ALL_MODULES, Module};

pub static CTRLC_PRESSED: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));
pub static SPEED_FACTOR: LazyLock<Mutex<f32>> = LazyLock::new(|| Mutex::new(1.0));
pub static INSTANT_PRINT_LINES: LazyLock<AtomicU32> = LazyLock::new(|| AtomicU32::new(0));
pub static STARTED_AT: LazyLock<Instant> = LazyLock::new(Instant::now);
pub static MODULES_RAN: LazyLock<AtomicU32> = LazyLock::new(|| AtomicU32::new(0));

pub async fn run(appconfig: AppConfig) {
    let mut rng = rng();
    let selected_modules: Vec<&Box<dyn Module + Send + 'static>> = appconfig
        .modules
        .iter()
        .map(|m| &ALL_MODULES[m.as_str()])
        .collect();
    loop {
        let choice = selected_modules.choose(&mut rng).unwrap();

        // platforms supported by `keepawake`
        #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
        let keep_awake = appconfig
            .inhibit
            .then(|| {
                keepawake::Builder::default()
                    .display(true)
                    .app_name("genact")
                    .app_reverse_domain("io.github.svenstaro.genact")
                    .reason(format!("Running `{}`", choice.signature()))
                    .create()
                    .inspect_err(|err| println!("WARN: failed to set up idle inhibition: {err}"))
                    .ok()
            })
            .flatten();

        choice.run(&appconfig).await;

        #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
        drop(keep_awake);

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
