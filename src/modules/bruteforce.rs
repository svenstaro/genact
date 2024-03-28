//! Choose a password, hash it, and pretend to decrypt that hash
use std::str::from_utf8;

use async_trait::async_trait;
use fake::{faker::name::raw::FirstName, locales::EN, Fake};
use rand::{rngs::ThreadRng, Rng};
use sha2::{Digest, Sha256};
use yansi::Paint;

use crate::args::AppConfig;
use crate::generators::gen_hex_string;
use crate::io::{csleep, cursor_up, newline, print};
use crate::modules::Module;

pub struct Bruteforce;

#[async_trait(?Send)]
impl Module for Bruteforce {
    fn name(&self) -> &'static str {
        "bruteforce"
    }

    fn signature(&self) -> String {
        "./bruteforce.sh ./hashes.txt".to_string()
    }

    async fn run(&self, app_config: &AppConfig) {
        let mut rng = rand::thread_rng();

        let n_parallel = rng.gen_range(2..10);
        let pass_hash_pairs: Vec<_> = std::iter::repeat_with(|| gen_pass_and_hash(&mut rng))
            .take(n_parallel)
            .collect();

        print("=> Hashes to decrypt").await;
        newline().await;
        csleep(500).await;

        for (_, hash) in &pass_hash_pairs {
            print(format!("  {hash}")).await;
            newline().await;
        }
        csleep(500).await;

        // Wait for "extraction" with a rainbow progress bar
        {
            let message = "=> Extracting Rainbow Table";
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

        print("=> Begin matching").await;
        newline().await;
        csleep(500).await;

        // Show the progress of "decryption"
        {
            let mut guessers: Vec<_> = pass_hash_pairs
                .iter()
                .map(|(_, hash)| HashGuesser::new(hash))
                .collect();
            let mut first = true;
            while !guessers.iter().all(|g| g.completed()) {
                // Do not cursor-up at the first time to avoid messing up
                if first {
                    first = false;
                } else {
                    cursor_up(n_parallel as u64).await;
                }

                for (i, a_guesser) in guessers.iter_mut().enumerate() {
                    a_guesser.tick_guess();
                    print(format!("\r :: {a_guesser} ::")).await;

                    // Do not append new line to the final line
                    if i != n_parallel {
                        newline().await;
                    }
                }

                csleep(10).await;

                if app_config.should_exit() {
                    return;
                }
            }
        }

        print("=> Match found").await;
        newline().await;

        for (pass, hash) in pass_hash_pairs {
            print(format!("  {hash}:{}", Paint::new(pass).bold())).await;
            newline().await;
        }
    }
}

// Get the SHA256 string for a str
fn sha256(s: &str) -> String {
    let mut hasher = Sha256::default();
    hasher.update(s);
    let result_bytes = hasher.finalize();
    format!("{result_bytes:x}")
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

fn gen_pass_and_hash<T: Rng>(rng: &mut T) -> (String, String) {
    let pass = format!(
        "{}{:02}",
        FirstName(EN).fake::<&str>().to_lowercase(),
        rng.gen_range(0..99)
    );
    let hash = sha256(&pass);
    (pass, hash)
}

struct HashGuesser {
    hash: Vec<u8>,
    guesses: Vec<u8>,
    progress: usize,
    len: usize,
    rng: ThreadRng,
}

impl HashGuesser {
    fn new(hash: &str) -> Self {
        // NOTE: the hash string must only contains lowercase hex characters

        Self {
            hash: hash.bytes().collect(),
            guesses: vec![], // will be set by tick_guess
            progress: 0,
            len: hash.len(),
            rng: rand::thread_rng(),
        }
    }

    fn tick_guess(&mut self) {
        if !self.completed() {
            self.guesses = gen_hex_string(&mut self.rng, self.len as u64)
                .bytes()
                .collect();

            while !self.completed() && self.guesses[self.progress] == self.hash[self.progress] {
                self.progress += 1;
            }
        }
    }

    fn completed(&self) -> bool {
        self.progress == self.len
    }
}

impl std::fmt::Display for HashGuesser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let progress = self.progress;

        let (done, undone) = if progress < self.len {
            let done = from_utf8(&self.hash[0..progress]).unwrap();
            let undone = from_utf8(&self.guesses[progress..]).unwrap();
            (done, undone)
        } else {
            (from_utf8(&self.hash).unwrap(), "")
        };

        let (done, undone) = (Paint::green(done), Paint::red(undone));

        write!(f, "{done}{undone}")
    }
}
