/// Module that dumps some random memory locations in a slightly cool fashion.
use rand::{thread_rng, Rng};
use std::io::Write;
use std::io::stdout;

use utils::{csleep, dprint, is_printable_ascii, gen_hex_string};
use parse_args::AppConfig;

pub fn run(appconfig: &AppConfig) {
    let mut rng = thread_rng();

    let mut current_loc = (rng.gen_range(0, 2u64.pow(63)) / 16) * 16;
    let num_lines = rng.gen_range(50, 200);
    for _ in 1..num_lines {
        dprint(format!("{loc:016x}  ", loc = current_loc), 0);
        current_loc += 0x10;

        let values = (0..16)
            .map(|_| gen_hex_string(&mut rng, 2))
            .collect::<Vec<String>>();

        // Print the values in two columns.
        for (n, val) in values.iter().enumerate() {
            if n == 8 {
                dprint(" ", 0);
            }
            dprint(format!("{} ", val), 0);
            let val_delay = rng.gen_range(0, 2);
            stdout().flush().unwrap();
            csleep(val_delay);
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
        dprint(format!(" |{ascii_repr}|", ascii_repr = ascii_repr), 0);

        let row_delay = rng.gen_range(10, 200);
        csleep(row_delay);

        if appconfig.should_exit() {
            return;
        }
        println!();
    }
}
