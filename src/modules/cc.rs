//! Pretend to run a C compiler
use std::path::Path;

use async_trait::async_trait;
use rand::rngs::ThreadRng;
use rand::seq::IndexedRandom;
use rand::{Rng, rng};

use crate::args::AppConfig;
use crate::data::{CFILES_LIST, PACKAGES_LIST};
use crate::generators::gen_random_n_from_list_into_string;
use crate::io::{csleep, newline, print};
use crate::modules::Module;

/// Generate a `String` containing all of the `file_list`'s file's parents as -I flags
fn generate_includes(file_list: &[&str], max: u32, rng: &mut ThreadRng) -> String {
    let mut include_flags = vec![];
    for file in file_list {
        let path = Path::new(file);
        if let Some(dir) = path.parent() {
            if let Some(dir_str) = dir.to_str() {
                if !include_flags.contains(&dir_str) {
                    include_flags.push(dir_str);
                }
            }
        }
    }
    let limited_flags = (0..max).map(|_| include_flags.choose(rng).unwrap());
    limited_flags.fold(String::new(), |acc, x| acc + "-I" + x + " ")
}

/// Generate a list of `n` random linker flags given a list of `candidates`.
fn generate_linker_flags(candidates: &[&str], n: usize, rng: &mut ThreadRng) -> String {
    let libraries = candidates.choose_multiple(rng, n);
    libraries.fold(String::new(), |acc, &x| acc + "-l" + x + " ")
}

pub struct Cc;

#[async_trait(?Send)]
impl Module for Cc {
    fn name(&self) -> &'static str {
        "cc"
    }

    fn signature(&self) -> String {
        "gcc app.c".to_string()
    }

    async fn run(&self, appconfig: &AppConfig) {
        const COMPILERS: &[&str] = &["gcc", "clang"];
        const FLAGS_OPT: &[&str] = &["-O0", "-O1", "-O2", "-O3", "-Og", "-Os"];
        const FLAGS_WARN_BASE: &[&str] = &["-Wall", "-Wall -Wextra"];
        const FLAGS_WARN: &[&str] = &[
            "-Wno-unused-variable",
            "-Wno-sign-compare",
            "-Wno-unknown-pragmas",
            "-Wno-parentheses",
            "-Wundef",
            "-Wwrite-strings",
            "-Wold-style-definition",
        ];
        const FLAGS_F: &[&str] = &["-fsigned-char", "-funroll-loops", "-fgnu89-inline", "-fPIC"];
        const FLAGS_ARCH: &[&str] = &["-march=x86-64", "-mtune=generic", "-pipe"];
        const FLAGS_DEF_BASE: &[&str] = &["-DDEBUG", "-DNDEBUG"];
        const FLAGS_DEF: &[&str] = &[
            "-D_REENTRANT",
            "-DMATH_LOOP",
            "-D_LIBS_REENTRANT",
            "-DNAMESPACE=lib",
            "-DMODULE_NAME=lib",
            "-DPIC",
            "-DSHARED",
        ];

        let mut rng = rng();

        // Choose a random package name to be our final linking target.
        let package = &PACKAGES_LIST.choose(&mut rng).unwrap();

        let compiler = COMPILERS.choose(&mut rng).unwrap();

        let num_cfiles = rng.random_range(100..1000);
        let mut chosen_files: Vec<&str> = CFILES_LIST
            .choose_multiple(&mut rng, num_cfiles)
            .cloned()
            .collect();
        chosen_files.sort_unstable();

        let opt = FLAGS_OPT.choose(&mut rng).unwrap();

        // Pick a bunch of warning flags.
        let warn = FLAGS_WARN_BASE.choose(&mut rng).unwrap().to_string();
        let num_additional_warn_flags = rng.random_range(0..FLAGS_WARN.len()) as u64;
        let warn_additional =
            gen_random_n_from_list_into_string(&mut rng, FLAGS_WARN, num_additional_warn_flags);
        let warn_final = warn + &warn_additional;

        // Pick a bunch of f flags
        let num_f_flags = rng.random_range(0..FLAGS_F.len()) as u64;
        let f = gen_random_n_from_list_into_string(&mut rng, FLAGS_F, num_f_flags);

        // Pick a bunch of architecture flags.
        let num_arch_flags = rng.random_range(0..FLAGS_ARCH.len()) as u64;
        let arch = gen_random_n_from_list_into_string(&mut rng, FLAGS_ARCH, num_arch_flags);

        // Get includes for the given files.
        let includes = generate_includes(chosen_files.as_slice(), 20, &mut rng);

        // Get random linker flags.
        let num_linker_flags = rng.random_range(0..10);
        let linker_flags = generate_linker_flags(&PACKAGES_LIST, num_linker_flags, &mut rng);

        // Pick a bunch of defs
        let defs = FLAGS_DEF_BASE.choose(&mut rng).unwrap().to_string();
        let num_def_flags = rng.random_range(0..FLAGS_DEF.len()) as u64;
        let defs_additional =
            gen_random_n_from_list_into_string(&mut rng, FLAGS_DEF, num_def_flags);
        let defs_final = defs + &defs_additional;

        // Compile everything.
        for cfile in &chosen_files {
            print(format!(
                "{compiler} -c {opt} {warn}{f}{arch} {includes}{defs} -o {output_file}",
                compiler = compiler,
                opt = opt,
                warn = warn_final,
                f = f,
                arch = arch,
                includes = includes,
                defs = defs_final,
                output_file = cfile.replace(".c", ".o")
            ))
            .await;
            newline().await;

            let sleep_length = rng.random_range(30..200);
            csleep(sleep_length).await;

            if appconfig.should_exit() {
                return;
            }
        }

        // Link everything together.
        let object_files = chosen_files
            .iter()
            .fold(String::new(), |acc, &x| acc + &x.replace(".c", ".o") + " ");
        print(format!(
            "{compiler} -o {package} {object_files}{linker_flags}"
        ))
        .await;
        newline().await;

        let sleep_length = rng.random_range(300..1000);
        csleep(sleep_length).await;
    }
}
