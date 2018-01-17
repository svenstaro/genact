/// Module containing random utilities.

use rand::{ThreadRng, Rng};
use std::{thread, time};
use std::cmp;
use std::io::Write;
use std::io::stdout;

#[cfg(target_os = "emscripten")]
use emscripten_sys;

/// Sleep for `length` milliseconds.
pub fn sleep(length: u64) {
    let sleep_length = time::Duration::from_millis(length as u64);

    #[cfg(not(target_os = "emscripten"))]
    thread::sleep(sleep_length);

    #[cfg(target_os = "emscripten")]
    unsafe {
        let sleep_millis: u32 = sleep_length.subsec_nanos() / 1_000_000;
        emscripten_sys::emscripten_sleep(sleep_millis);
    }
}

/// Print `s` with each letter delayed by `delay` milliseconds.
pub fn dprint<S: Into<String>>(s: S, delay: u64) {
    // Construct a `Vec` of single characters converted to `String`s.
    let string_arr = s
        .into()
        .chars()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    for c in string_arr {
        #[cfg(target_os = "emscripten")]
        {
            js! {
                window.term.write(@{c});
            }
        }

        #[cfg(not(target_os = "emscripten"))]
        {
            print!("{}", c);
            stdout().flush().unwrap();
        }

        if delay > 0 {
            self::sleep(delay);
        }
    }
}

pub fn rand_hex_string(rng: &mut ThreadRng, length: u64) -> String {
    const HEX_CHARS: &[u8] = b"0123456789abcdef";
    let hex_string: String = (0..length)
        .map(|_| (*rng.choose(HEX_CHARS).unwrap() as char))
        .collect();
    hex_string
}

/// Return a String containing `n` random concatenated elements from `list`.
///
/// If `n` >= `list.len()` then `list.len()` will be used instead of `n`.
pub fn get_random_n_from_list_into_string(rng: &mut ThreadRng, list: Vec<&str>, n: u64) -> String {
    (0..cmp::min(n, list.len() as u64))
        .fold(String::new(), |acc, _| acc + " " + &rng.choose(&list).unwrap())
}

/// Return `true` if the given `a` is printable ASCII and `false` if it isn't.
pub fn is_printable_ascii(a: u64) -> bool {
    a >= 0x21 && a <= 0x7e
}
