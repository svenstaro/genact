extern crate clap;
extern crate rand;
extern crate console;

use clap::{Arg, App};
use rand::{thread_rng, Rng};
use std::process;

mod bootlog;
mod cargo;

fn main() {
    let modules = vec![
        "bootlog",
        "cargo",
        // "bruteforce",
        // "download",
        // "dump",
        // "initialize",
        // "botnet",
        // "heartbeat",
    ];
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
                .possible_values(&modules)
                .help("Run only these modules"),
        )
        .get_matches();

    if app.is_present("list") {
        println!("Available modules:");
        for module in &modules {
            println!("  {}", module);
        }
        process::exit(0);
    }

    let modules_to_run: Vec<&str> = if app.is_present("modules") {
        app.values_of("modules")
            .unwrap()
            .collect()
    } else {
        modules
    };

    let mut rng = thread_rng();
    loop {
        match rng.choose(&modules_to_run).unwrap() {
            &"bootlog" => bootlog::run(),
            &"cargo" => cargo::run(),
            _ => panic!("Unknown module!"),
        }
    }
}
