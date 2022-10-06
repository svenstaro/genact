//! Choose a password, hash it, and pretend to decrypt that hash
use std::str::from_utf8;

use async_trait::async_trait;
use rand::seq::SliceRandom;
use yansi::Paint;

use crate::args::AppConfig;
use crate::data::PASSWORDS_AND_HASHES_LIST;
use crate::generators::gen_hex_string;
use crate::io::{csleep, newline, print};
use crate::modules::Module;

pub struct Bruteforce;

#[async_trait(?Send)]
impl Module for Bruteforce {
    fn name(&self) -> &'static str {
        "bruteforce"
    }
    fn signature(&self) -> String {
        "./bruteforce.sh".to_string()
    }
    async fn run(&self, app_config: &AppConfig) {
        let mut rng = rand::thread_rng();
        let (password, hash_str) = *PASSWORDS_AND_HASHES_LIST.choose(&mut rng).unwrap();

        print(format!("SHA256 value: {hash_str}",)).await;
        newline().await;
        csleep(500).await;

        // Wait for "extraction" with a rainbow progress bar
        {
            let message = "Extracting Rainbow Table";
            let width = 30;
            let millis_wait = 2500;
            let fill_char = "=";

            let mut spinner_statuses = ["|", "/", "-", "\\"].iter().cycle();

            for i in 0..width {
                let spinner = if i == (width - 1) {
                    // Make sure to get a fill_char at the end of our cycle when done.
                    fill_char
                } else {
                    spinner_statuses.next().unwrap()
                };

                let progress = fill_char.repeat(i);
                let spaces = " ".repeat(width - i - 1);
                let progress_content = rainbow(&format!("{progress}{spinner}{spaces}"));

                print(format!("\r{message} [{progress_content}]",)).await;

                csleep((millis_wait / width) as u64).await;

                if app_config.should_exit() {
                    return;
                }
            }

            newline().await;
        }

        print("Begin matching").await;
        newline().await;
        csleep(500).await;

        // Show the progress of "decryption"
        {
            let mut progress: usize = 0;

            let hash_bytes = hash_str.bytes().collect::<Vec<_>>();
            let l = hash_bytes.len();

            while progress < l {
                let guesses: Vec<_> = gen_hex_string(&mut rng, l as u64).bytes().collect();

                while progress < l && guesses[progress] == hash_bytes[progress] {
                    progress += 1;
                }

                let (done, undone) = if progress < l {
                    let done = from_utf8(&hash_bytes[0..progress]).unwrap();
                    let undone = from_utf8(&guesses[progress..]).unwrap();
                    (done, undone)
                } else {
                    (hash_str, "")
                };

                let (done, undone) = (Paint::green(done), Paint::red(undone));

                print(format!("\r :: {done}{undone} ::")).await;
                csleep(10).await;

                if app_config.should_exit() {
                    return;
                }
            }
            newline().await;
        }

        print(format!("+ Match found -- the password is \"{password}\"")).await;
        newline().await;
    }
}

// color a string rainbow
fn rainbow(s: &str) -> String {
    use std::fmt::Write;

    // Need to split chars by bytes
    debug_assert!(s.is_ascii());

    static RAINBOW_COLORS: &[yansi::Color] = &[
        yansi::Color::Red,
        yansi::Color::Yellow,
        yansi::Color::Green,
        yansi::Color::Cyan,
        yansi::Color::Blue,
        yansi::Color::Magenta,
    ];

    let len = s.len();
    let colors = RAINBOW_COLORS.len();

    let bytes: Vec<_> = s.bytes().collect();
    let mut ret = String::new();

    // split the string and apply colors
    for (i, c) in RAINBOW_COLORS.iter().enumerate() {
        let start = i * len / colors;
        let end = (i + 1) * len / colors;
        let s = from_utf8(&bytes[start..end]).unwrap();
        write!(ret, "{}", Paint::new(s).fg(*c)).unwrap();
    }

    ret
}
