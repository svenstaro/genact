/// Module containing random utilities.
use rand::{ThreadRng, Rng};
use rand::distributions::Uniform;
use std::path::{Path, PathBuf};
use std::time;
#[cfg(not(target_os = "emscripten"))]
use std::thread;
use std::cmp;
#[cfg(target_os = "emscripten")]
use std::io;
use std::io::Write;
#[cfg(not(target_os = "emscripten"))]
use std::io::stdout;
use std::str;

#[cfg(target_os = "emscripten")]
use emscripten_sys;

/// A cross-platform sleep for `length` milliseconds that also handles web stuff.
pub fn csleep(length: u64) {
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
            self::csleep(delay);
        }
    }
}

/// Generate a string of `length` with characters randomly sampled
/// from `string`.
pub fn gen_string_with_chars(rng: &mut ThreadRng, char_set: &str, length: u64) -> String {
    let chars: Vec<_> = char_set.chars().collect();
    let range = Uniform::new(0, chars.len());

    let string: String = (0..length)
        .map(|_| chars[rng.sample(range)])
        .collect();
    string
}

pub fn gen_hex_string(rng: &mut ThreadRng, length: u64) -> String {
    gen_string_with_chars(rng, "0123456789abcdef", length)
}

/// Return a String containing `n` random concatenated elements from `list`.
///
/// If `n` >= `list.len()` then `list.len()` will be used instead of `n`.
pub fn gen_random_n_from_list_into_string(rng: &mut ThreadRng, list: &[&str], n: u64) -> String {
    let range = Uniform::new(0, list.len());
    (0..cmp::min(n, list.len() as u64))
        .fold(String::new(), |acc, _| acc + " " + list[rng.sample(range)])
}

pub fn gen_file_name_with_ext(rng: &mut ThreadRng, files: &[&str], extension: &str) -> String {
    let chosen_file = rng.choose(files).unwrap_or(&"");
    let path = Path::new(&chosen_file).with_extension(extension);
    path.file_name().unwrap().to_str().unwrap().to_string()
}

pub fn gen_file_name(rng: &mut ThreadRng, files: &[&str], extensions: &[&str]) -> String {
    let chosen_file = rng.choose(files).unwrap_or(&"");
    let chosen_extension = rng.choose(extensions).unwrap_or(&"");
    let path = Path::new(&chosen_file).with_extension(chosen_extension);
    path.file_name().unwrap().to_str().unwrap().to_string()
}

pub fn gen_file_path(rng: &mut ThreadRng, files: &[&str], extensions: &[&str], dir_candidates: &[&str]) -> String {
    let path_length = rng.gen_range(1, 5);
    let mut path = PathBuf::from("/");
    let range = Uniform::new(0, dir_candidates.len());
    for _ in 0..path_length {
        path.push(dir_candidates[rng.sample(range)]);
    }
    path.push(gen_file_name(rng, files, extensions));
    path.to_string_lossy().to_string()
}

/// Return `true` if the given `a` is printable ASCII and `false` if it isn't.
pub fn is_printable_ascii(a: u64) -> bool {
    a >= 0x21 && a <= 0x7e
}

pub fn cursor_up(n: u64) {
    dprint(format!("\x1b[{}A", n), 0);
}

pub fn erase_line() {
    dprint("\x1b[2K", 0);
}

#[cfg(target_os = "emscripten")]
pub struct TermWriter;

#[cfg(target_os = "emscripten")]
impl Write for TermWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let n = buf.len();
        let s = str::from_utf8(buf).unwrap();
        js! {
            window.term.write(@{s});
        }
        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
