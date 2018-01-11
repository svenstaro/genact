#[cfg(not(target_os = "emscripten"))]
extern crate clap;

#[cfg(target_os = "emscripten")]
extern crate emscripten_sys;

extern crate chrono;
extern crate pbr;
extern crate rand;
extern crate yansi;

use pbr::ProgressBar;

use rand::{thread_rng, Rng};
use yansi::Paint;

#[cfg(not(target_os = "emscripten"))]
use clap::{Arg, App};

mod bootlog;
mod cargo;
mod cryptomining;
mod utils;

#[cfg(not(target_os = "emscripten"))]
fn parse_args(all_modules: Vec<&str>) -> Vec<String> {
    use std::process;

    let app = App::new("genact")
        .version("0.1")
        .author("Sven-Hendrik Haase <svenstaro@gmail.com>")
        .about("A nonsense activity generator")
        .arg(
            Arg::with_name("list")
            .short("l")
            .long("list-modules")
            .help("List available modules"),
            )
        .arg(
            Arg::with_name("modules")
            .short("m")
            .long("modules")
            .multiple(true)
            .value_name("MODULE")
            .takes_value(true)
            .possible_values(&all_modules)
            .help("Run only these modules"),
            )
        .get_matches();

    if app.is_present("list") {
        println!("Available modules:");
        for module in all_modules {
            println!("  {}", module);
        }
        process::exit(0);
    }

    if app.is_present("modules") {
        app.values_of("modules")
            .unwrap()
            .map(|x| x.to_string())
            .collect()
    } else {
        all_modules.iter().map(|x| x.to_string()).collect()
    }
}

fn main() {
    Paint::enable_windows_ascii();

    let all_modules = vec![
        "bootlog",
        "cargo",
        "cryptomining",
        // "bruteforce",
        // "download",
        // "dump",
        // "initialize",
        // "botnet",
        // "heartbeat",
    ];

    #[cfg(not(target_os = "emscripten"))]
    let modules_to_run = parse_args(all_modules);

    #[cfg(target_os = "emscripten")]
    let modules_to_run: Vec<String> = all_modules.iter().map(|x| x.to_string()).collect();

    let mut rng = thread_rng();
    loop {
        // let count = 1000;
        // let mut pb = ProgressBar::new(count);
        // pb.format("╢▌▌░╟");
        // for _ in 0..count {
        //     pb.inc();
        //     #[cfg(not(target_os = "emscripten"))]
        //     thread::sleep(time::Duration::from_millis(200));
        //
        //     #[cfg(target_os = "emscripten")]
        //     unsafe {
        //         emscripten_sys::emscripten_sleep(200u32);
        //         // For some reason, we actually have to manually print a newline here even if we
        //         // flush it manually in order to get it to draw anything at all. This is really
        //         // weird but I'll figure it out some other time.
        //         println!();
        //     }
        // }
        // pb.finish_print("done");

        let choice: &str = &rng.choose(&modules_to_run).unwrap();
        match choice {
            "bootlog" => bootlog::run(),
            "cargo" => cargo::run(),
            "cryptomining" => cryptomining::run(),
            _ => panic!("Unknown module!"),
        }
    }
}
