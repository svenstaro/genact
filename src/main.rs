#[cfg(not(target_arch = "wasm32"))]
use anyhow::Result;

use genact::args::parse_args;
use genact::{run, SPEED_FACTOR};

#[cfg(not(target_arch = "wasm32"))]
use genact::exit_handler;

#[cfg(not(target_arch = "wasm32"))]
#[async_std::main]
async fn main() -> Result<()> {
    yansi::Paint::enable_windows_ascii();

    let appconfig = parse_args();
    *SPEED_FACTOR.lock().await = appconfig.speed_factor;

    if appconfig.list_modules_and_exit {
        println!("Available modules:");
        for module in genact::modules::ALL_MODULES.keys() {
            println!("  {}", module);
        }
        std::process::exit(0);
    }

    ctrlc::set_handler(exit_handler)?;

    run(appconfig).await;

    Ok(())
}

// Called when the wasm module is instantiated
#[cfg(target_arch = "wasm32")]
#[async_std::main]
async fn main() {
    use std::panic;
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let appconfig = parse_args();
    *SPEED_FACTOR.lock().await = appconfig.speed_factor;

    run(appconfig).await;
}
