//! Module containing random utilities.
use std::cmp;
use std::path::{Path, PathBuf};
use std::str;

use rand::distr::Uniform;
use rand::rngs::ThreadRng;
use rand::seq::IndexedRandom;
use rand::Rng;

/// Generate a string of `length` with characters randomly sampled
/// from `string`.
pub fn gen_string_with_chars(rng: &mut ThreadRng, char_set: &str, length: u64) -> String {
    let chars: Vec<_> = char_set.chars().collect();
    let range = Uniform::new(0, chars.len()).unwrap();

    let string: String = (0..length).map(|_| chars[rng.sample(range)]).collect();
    string
}

pub fn gen_hex_string(rng: &mut ThreadRng, length: u64) -> String {
    gen_string_with_chars(rng, "0123456789abcdef", length)
}

/// Return a String containing `n` random concatenated elements from `list`.
///
/// If `n` >= `list.len()` then `list.len()` will be used instead of `n`.
pub fn gen_random_n_from_list_into_string(rng: &mut ThreadRng, list: &[&str], n: u64) -> String {
    let range = Uniform::new(0, list.len()).unwrap();
    (0..cmp::min(n, list.len() as u64))
        .fold(String::new(), |acc, _| acc + " " + list[rng.sample(range)])
}

pub fn gen_file_name_with_ext(rng: &mut ThreadRng, files: &[&str], extension: &str) -> String {
    let chosen_file = files.choose(rng).unwrap_or(&"");
    let path = Path::new(&chosen_file).with_extension(extension);
    path.file_name().unwrap().to_str().unwrap().to_string()
}

pub fn gen_file_name(rng: &mut ThreadRng, files: &[&str], extensions: &[&str]) -> String {
    let chosen_file = files.choose(rng).unwrap_or(&"");
    let chosen_extension = extensions.choose(rng).unwrap_or(&"");
    let path = Path::new(&chosen_file).with_extension(chosen_extension);
    path.file_name().unwrap().to_str().unwrap().to_string()
}

pub fn gen_file_path<T: std::clone::Clone + AsRef<str> + std::convert::AsRef<std::path::Path>>(
    rng: &mut ThreadRng,
    files: &[&str],
    extensions: &[&str],
    dir_candidates: &[T],
) -> String {
    let path_length = rng.random_range(1..5);
    let mut path = PathBuf::from("/");
    let range = Uniform::new(0, dir_candidates.len()).unwrap();
    for _ in 0..path_length {
        path.push(dir_candidates[rng.sample(range)].clone());
    }
    path.push(gen_file_name(rng, files, extensions));
    path.to_string_lossy().to_string()
}

pub fn gen_package_version(rng: &mut ThreadRng) -> String {
    let major = rng.random_range(0..10);    // Major versions typically 0-9
    let minor = rng.random_range(0..100);   // Minor versions typically 0-99
    let patch = rng.random_range(0..1000);  // Patch versions typically 0-999
    
    format!("{major}.{minor}.{patch}")
}
