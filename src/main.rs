use anyhow::Result;
use yansi::Paint;

use genact::args::parse_args;
use genact::{exit_handler, run, ALL_MODULES, SPEED_FACTOR};

#[async_std::main]
async fn main() -> Result<()> {
    Paint::enable_windows_ascii();

    let appconfig = parse_args();
    *SPEED_FACTOR.lock().await = appconfig.speed_factor;

    if appconfig.list_modules_and_exit {
        println!("Available modules:");
        for module in ALL_MODULES {
            println!("  {}", module);
        }
        std::process::exit(0);
    }

    ctrlc::set_handler(exit_handler)?;

    run(appconfig).await;

    Ok(())
}
