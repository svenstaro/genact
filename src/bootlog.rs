use rand::{thread_rng, Rng};
use rand::distributions::{IndependentSample, Range};
use std::{thread, time};
use console::style;

pub fn run() {
    let bootlog = include_str!("../data/bootlog.txt");

    let mut rng = thread_rng();
    let between = Range::new(10, 30);
    let line_count = between.ind_sample(&mut rng);
    let lines: Vec<&str> = bootlog.lines().collect();

    for _ in 1..line_count {
        let choice = rng.choose(&lines).unwrap_or(&"");
        let sleep_range = Range::new(10, 100);
        let sleep_length = time::Duration::from_millis(sleep_range.ind_sample(&mut rng));

        let is_error = rng.gen_weighted_bool(100);
        if is_error {
            println!("{}", style(format!("ERROR: {}", choice)).red());
        } else {
            let has_bold_word = rng.gen_weighted_bool(10);
            if has_bold_word {
                let mut words: Vec<String> = choice.split_whitespace().map(|x| String::from(x)).collect();
                words[0] = format!("{}", style(&words[0]).bold());
                println!("{}", words.join(" "));
            } else {
                println!("{}", choice);
            }
        }
        thread::sleep(sleep_length);
    }
}
