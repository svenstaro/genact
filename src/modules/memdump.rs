//! Pretend to dump some random memory locations
use async_trait::async_trait;
use rand::prelude::*;
use std::io::stdout;
use std::io::Write;

use crate::args::AppConfig;
use crate::generators::gen_hex_string;
use crate::io::{csleep, is_printable_ascii, newline, print};
use crate::modules::Module;

pub struct Memdump;

#[async_trait(?Send)]
impl Module for Memdump {
    fn name(&self) -> &'static str {
        "memdump"
    }

    fn signature(&self) -> String {
        "memdump -k -v".to_string()
    }

    async fn run(&self, appconfig: &AppConfig) {
        let mut rng = thread_rng();

        let mut current_loc = (rng.gen_range(0..2u64.pow(63)) / 16) * 16;
        let num_lines = rng.gen_range(50..200);
        for _ in 1..num_lines {
            print(format!("{current_loc:016x}  ")).await;
            current_loc += 0x10;

            let values = (0..16)
                .map(|_| gen_hex_string(&mut rng, 2))
                .collect::<Vec<String>>();

            // Print the values in two columns.
            for (n, val) in values.iter().enumerate() {
                if n == 8 {
                    print(" ").await;
                }
                print(format!("{val} ")).await;
                let val_delay = rng.gen_range(0..2);
                stdout().flush().unwrap();
                csleep(val_delay).await;
            }

            // Print the ascii values.
            let mut ascii_repr = String::with_capacity(values.len());
            for val in values {
                let ascii_val = u8::from_str_radix(&val, 16).unwrap_or(b'.') as char;
                if is_printable_ascii(ascii_val as u64) {
                    ascii_repr.push(ascii_val);
                } else {
                    ascii_repr.push('.');
                }
            }
            print(format!(" |{ascii_repr}|")).await;

            let row_delay = rng.gen_range(10..200);
            csleep(row_delay).await;

            if appconfig.should_exit() {
                return;
            }
            newline().await;
        }
    }
}
