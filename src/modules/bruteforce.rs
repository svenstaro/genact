//! Choose a password, hash it, and pretend to decrypt that hash
use std::str::from_utf8;

use async_trait::async_trait;
use fake::{faker::name::raw::FirstName, locales::EN, Fake};
use rand::Rng;
use sha2::{Digest, Sha256};
use yansi::Paint;

use crate::args::AppConfig;
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
        let password = &format!(
            "{}{:02}",
            FirstName(EN).fake::<&str>().to_lowercase(),
            rng.gen_range(0..99)
        );
        let hash_str: &str = &sha256(password);

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

// Get the SHA256 string for a str
fn sha256(s: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(&s);
    let result_bytes = hasher.finalize();
    format!("{:x}", result_bytes)
}

// Color a string rainbow
fn rainbow(s: &str) -> String {
    use std::fmt::Write;

    let len = s.len();
    let colors = colorgrad::sinebow().colors(len);
    let mut ret = String::new();

    // apply colors to each characters
    for (color, ch) in colors.into_iter().zip(s.chars()) {
        let approx = approx_color(color);
        write!(ret, "{}", Paint::new(ch).fg(approx)).unwrap();
    }

    ret
}

// Approximate RGB with ANSI 216 colors
fn approx_color(c: colorgrad::Color) -> yansi::Color {
    // 6 × 6 × 6 cube (216 colors): 16 + 36 × r + 6 × g + b (0 ≤ r, g, b ≤ 5)

    let r = (c.r * 5.).round() as u8;
    let g = (c.g * 5.).round() as u8;
    let b = (c.b * 5.).round() as u8;

    yansi::Color::Fixed(16 + 36 * r + 6 * g + b)
}
