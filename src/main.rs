#[cfg(not(target_os = "emscripten"))]
extern crate clap;

extern crate rand;
// extern crate console;

use rand::{thread_rng, Rng};

#[cfg(not(target_os = "emscripten"))]
use clap::{Arg, App};

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

mod bootlog;
mod cargo;

fn main() {
    let all_modules = vec![
        "bootlog",
        "cargo",
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
        let choice: &str = &rng.choose(&modules_to_run).unwrap();
        match choice {
            "bootlog" => bootlog::run(),
            "cargo" => cargo::run(),
            _ => panic!("Unknown module!"),
        }
    }
}
