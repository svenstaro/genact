/// Module that dumps some random memory locations in a slightly cool fashion.

use rand::{thread_rng, Rng};
use yansi::Paint;
use std::io::Write;
use std::io::stdout;

use utils;

pub fn run() {
    let mut rng = thread_rng();

    let mut current_loc = (rng.gen_range(0, 2u64.pow(63)) / 16) * 16;
    let num_lines = rng.gen_range(50, 200);
    for _ in 1..num_lines {
        print!("{loc:016x}  ", loc=current_loc);
        current_loc += 0x10;

        let values = (0..16)
            .map(|_| utils::rand_hex_string(&mut rng, 2))
            .collect::<Vec<String>>();

        // Print the values in two columns.
        for (n, val) in values.iter().enumerate() {
            if n == 8 {
                print!(" ");
            }
            print!("{} ", val);
            let val_delay = rng.gen_range(20, 40);
            stdout().flush().unwrap();
            utils::sleep(val_delay);
        }

        // Print the ascii values.
        let mut ascii_repr = String::new();
        for val in values {
            let ascii_val = u8::from_str_radix(&val, 16).unwrap_or('.' as u8) as char;
            if utils::is_printable_ascii(ascii_val as usize) {
                ascii_repr.push(ascii_val);
            } else {
                ascii_repr.push('.');
            }
        }
        print!(" |{ascii_repr}|", ascii_repr=ascii_repr);

        let row_delay = rng.gen_range(100, 200);
        utils::sleep(row_delay);
        println!();
    }
}
