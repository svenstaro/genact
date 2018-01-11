use rand::{thread_rng, Rng};
use yansi::Paint;

use utils;

pub fn run() {
    let bootlog = include_str!("../data/bootlog.txt");

    let mut rng = thread_rng();
    let line_count = rng.gen_range(20, 100);
    let lines: Vec<&str> = bootlog.lines().collect();

    for _ in 1..line_count {
        let choice = rng.choose(&lines).unwrap_or(&"");
        let sleep_length = rng.gen_range(10, 1000);

        let is_error = rng.gen_weighted_bool(100);
        if is_error {
            println!("{}", Paint::red(format!("ERROR: {}", choice)));
        } else {
            let has_bold_word = rng.gen_weighted_bool(10);
            if has_bold_word {
                let mut words: Vec<String> = choice.split_whitespace().map(|x| String::from(x)).collect();
                words[0] = format!("{}", Paint::new((&words[0])).bold());
                println!("{}", words.join(" "));
            } else {
                println!("{}", choice);
            }
        }

        utils::sleep(sleep_length);
    }
}
