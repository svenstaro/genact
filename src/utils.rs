/// Module containing random utilities.

use rand::{ThreadRng, Rng};
use std::{thread, time};
use std::cmp;

#[cfg(target_os = "emscripten")]
use emscripten_sys;

pub fn sleep(length: u64) {
    let sleep_length = time::Duration::from_millis(length);

    #[cfg(not(target_os = "emscripten"))]
    thread::sleep(sleep_length);

    #[cfg(target_os = "emscripten")]
    unsafe {
        let sleep_millis: u32 = sleep_length.subsec_nanos() / 1_000_000;
        emscripten_sys::emscripten_sleep(sleep_millis);
    }
}

pub fn rand_hex_string(rng: &mut ThreadRng, length: usize) -> String {
    const HEX_CHARS: &[u8] = b"0123456789abcdef";
    let hex_string: String = (0..length).map(|_| (*rng.choose(HEX_CHARS).unwrap() as char)).collect();
    hex_string
}

/// Return a String containing `n` random concatenated elements from `list`.
///
/// If `n` >= `list.len()` then `list.len()` will be used instead of `n`.
pub fn get_random_n_from_list_into_string(rng: &mut ThreadRng, list: Vec<&str>, n: usize) -> String {
    (0..cmp::min(n, list.len()))
        .fold(String::new(), |acc, _| acc + " " + &rng.choose(&list).unwrap())
}

/// Return `true` if the given `a` is printable ASCII and `false` if it isn't.
pub fn is_printable_ascii(a: usize) -> bool {
    a >= 0x21 && a <= 0x7e
}
