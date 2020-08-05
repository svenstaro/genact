use anyhow::Result;
use yansi::Paint;

#[async_std::main]
async fn main() -> Result<()> {
    Paint::enable_windows_ascii();

    use genact::parse_args::parse_args;
    use genact::{run, ALL_MODULES};
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
        genact::CTRLC_PRESSED.store(true, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    run(appconfig).await;

    Ok(())
}
