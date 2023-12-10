#[cfg(not(target_arch = "wasm32"))]
use anyhow::Result;

use genact::args::parse_args;
use genact::{run, INSTANT_PRINT_LINES, SPEED_FACTOR};

use std::sync::atomic::Ordering;

#[cfg(not(target_arch = "wasm32"))]
use genact::exit_handler;

#[cfg(not(target_arch = "wasm32"))]
#[async_std::main]
async fn main() -> Result<()> {
    use clap::CommandFactory;
    use genact::args::AppConfig;

    yansi::Paint::enable_windows_ascii();

    let appconfig = parse_args();

    if let Some(shell) = appconfig.print_completions {
        let mut clap_app = AppConfig::command();
        let app_name = clap_app.get_name().to_string();
        clap_complete::generate(shell, &mut clap_app, app_name, &mut std::io::stdout());
        return Ok(());
    }

    if appconfig.print_manpage {
        let clap_app = AppConfig::command();
        let man = clap_mangen::Man::new(clap_app);
        man.render(&mut std::io::stdout())?;
        return Ok(());
    }

    *SPEED_FACTOR.lock().await = appconfig.speed_factor;
    INSTANT_PRINT_LINES.store(appconfig.instant_print_lines, Ordering::SeqCst);

    if appconfig.list_modules_and_exit {
        println!("Available modules:");
        for module in genact::modules::ALL_MODULES.keys() {
            println!("  {module}");
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
    INSTANT_PRINT_LINES.store(appconfig.instant_print_lines, Ordering::SeqCst);

    run(appconfig).await;
}
