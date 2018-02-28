/// Module that pretends to run a C compiler.
use rand::{thread_rng, Rng, ThreadRng};
use std::path::Path;

use utils::{csleep, get_random_n_from_list_into_string};
use CFILES_LIST;
use parse_args::AppConfig;

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
    let limited_flags = (0..max).map(|_| *rng.choose(&include_flags).unwrap());
    limited_flags.fold(String::new(), |acc, x| acc + "-I" + x + " ")
}

/// Generate a list of `n` random linker flags given a list of `candidates`.
fn generate_linker_flags(candidates: &[&str], n: u64, rng: &mut ThreadRng) -> String {
    let mut libraries = vec![];
    for _ in 0..n {
        let candidate = rng.choose(candidates).unwrap();
        if !libraries.contains(candidate) {
            libraries.push(candidate);
        }
    }
    libraries
        .iter()
        .fold(String::new(), |acc, &x| acc + "-l" + x + " ")
}

pub fn run(appconfig: &AppConfig) {
    let packages = include_str!("../data/packages.txt");
    let packages_list: Vec<&str> = packages.lines().collect();

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

    let mut rng = thread_rng();
    let num_cfiles = rng.gen_range(100, 1000);
    let mut chosen_files: Vec<&str> = vec![];

    // Choose a random package name to be our final linking target.
    let package = rng.choose(&packages_list).unwrap();

    let compiler = rng.choose(COMPILERS).unwrap();

    while chosen_files.len() < num_cfiles {
        let cfile = rng.choose(&CFILES_LIST).unwrap();
        if !chosen_files.contains(cfile) {
            chosen_files.push(cfile);
        }
    }
    chosen_files.sort();

    let opt = rng.choose(FLAGS_OPT).unwrap();

    // Pick a bunch of warning flags.
    let warn = rng.choose(FLAGS_WARN_BASE).unwrap().to_string();
    let num_additional_warn_flags = rng.gen_range(0, FLAGS_WARN.len()) as u64;
    let warn_additional =
        get_random_n_from_list_into_string(&mut rng, FLAGS_WARN, num_additional_warn_flags);
    let warn_final = warn + &warn_additional;

    // Pick a bunch of f flags
    let num_f_flags = rng.gen_range(0, FLAGS_F.len()) as u64;
    let f = get_random_n_from_list_into_string(&mut rng, FLAGS_F, num_f_flags);

    // Pick a bunch of architecture flags.
    let num_arch_flags = rng.gen_range(0, FLAGS_ARCH.len()) as u64;
    let arch = get_random_n_from_list_into_string(&mut rng, FLAGS_ARCH, num_arch_flags);

    // Get includes for the given files.
    let includes = generate_includes(&chosen_files, 20, &mut rng);

    // Get random linker flags.
    let num_linker_flags = rng.gen_range(0, 10);
    let linker_flags = generate_linker_flags(&packages_list, num_linker_flags, &mut rng);

    // Pick a bunch of defs
    let defs = rng.choose(FLAGS_DEF_BASE).unwrap().to_string();
    let num_def_flags = rng.gen_range(0, FLAGS_DEF.len()) as u64;
    let defs_additional = get_random_n_from_list_into_string(&mut rng, FLAGS_DEF, num_def_flags);
    let defs_final = defs + &defs_additional;

    // Compile everything.
    for cfile in &chosen_files {
        println!(
            "{compiler} -c {opt} {warn}{f}{arch} {includes}{defs} -o {output_file}",
            compiler = compiler,
            opt = opt,
            warn = warn_final,
            f = f,
            arch = arch,
            includes = includes,
            defs = defs_final,
            output_file = cfile.replace(".c", ".o")
        );

        let sleep_length = rng.gen_range(30, 200);
        csleep(sleep_length);

        if appconfig.should_exit() {
            return;
        }
    }

    // Link everything together.
    let object_files = chosen_files
        .iter()
        .fold(String::new(), |acc, &x| acc + &x.replace(".c", ".o") + " ");
    println!(
        "{compiler} -o {output_file} {object_files}{linker_flags}",
        compiler = compiler,
        output_file = package,
        object_files = object_files,
        linker_flags = linker_flags
    );

    let sleep_length = rng.gen_range(300, 1000);
    csleep(sleep_length);
}
