use rand::{thread_rng, Rng};
use std::{thread, time};

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

pub fn rand_hex_string(length: usize) -> String {
    const HEX_CHARS: &[u8] = b"0123456789abcdef";
    let mut rng = thread_rng();
    let hex_string: String = (0..length).map(|_| (*rng.choose(HEX_CHARS).unwrap() as char)).collect();
    hex_string
}
