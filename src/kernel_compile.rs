/// Module that pretends to build a Linux kernel
use rand::{thread_rng, Rng, ThreadRng};
use regex::Regex;

use utils::csleep;
use CFILES_LIST;
use parse_args::AppConfig;

/// Generate a build step for a header file
fn gen_header(arch: &str, rng: &mut ThreadRng) -> String {
    const RARE_CMDS: &[&str] = &[
        "SYSTBL ",
        "SYSHDR ",
    ];

    const CMDS: &[&str] = &[
        "WRAP   ",
        "CHK    ",
        "UPD    ",
    ];

    let cmd = if rng.gen_bool(1.0 / 15.0) {
        rng.choose(RARE_CMDS).unwrap_or(&"")
    } else {
        rng.choose(CMDS).unwrap_or(&"")
    };

    let cfile = rng.choose(&CFILES_LIST).unwrap_or(&"");
    let mut file = format!("{}h", &cfile[..cfile.len() - 1]);

    if file.starts_with("arch") {
        let re = Regex::new(r"arch/([a-z0-9_])+/").unwrap();
        file = re.replace(&file, format!("arch/{}/", arch).as_str()).into_owned();
    }

    format!("  {} {}", cmd, file)
}

/// Generate a build step for an object file
fn gen_object(arch: &str, rng: &mut ThreadRng) -> String {
    const RARE_CMDS: &[&str] = &[
        "HOSTCC ",
        "AS     ",
    ];

    let cmd = if rng.gen_bool(1.0 / 15.0) {
        rng.choose(RARE_CMDS).unwrap_or(&"")
    } else if rng.gen_bool(0.33) {
        "AR     "
    } else {
        "CC     "
    };

    let cfile = rng.choose(&CFILES_LIST).unwrap_or(&"");
    let mut file = format!("{}o", cfile[..cfile.len() - 1].to_owned());

    if file.starts_with("arch") {
        let re = Regex::new(r"arch/([a-z0-9_])+/").unwrap();
        file = re.replace(&file, format!("arch/{}/", arch).as_str()).into_owned();
    }

    format!("  {} {}", cmd, file)
}

/// Generate a 'special' build step
fn gen_special(arch: &str, rng: &mut ThreadRng) -> String {
    const SPECIALS: &[&str] = &[
        "HOSTLD  arch/ARCH/tools/relocs",
        "HOSTLD  scripts/mod/modpost",
        "MKELF   scripts/mod/elfconfig.h",
        "LDS     arch/ARCH/entry/vdso/vdso32/vdso32.lds",
        "LDS     arch/ARCH/kernel/vmlinux.lds",
        "LDS     arch/ARCH/realmode/rm/realmode.lds",
        "LDS     arch/ARCH/boot/compressed/vmlinux.lds",
        "EXPORTS arch/ARCH/lib/lib-ksyms.o",
        "EXPORTS lib/lib-ksyms.o",
        "MODPOST vmlinux.o",
        "SORTEX  vmlinux",
        "SYSMAP  System.map",
        "VOFFSET arch/ARCH/boot/compressed/../voffset.h",
        "OBJCOPY arch/ARCH/entry/vdso/vdso32.so",
        "OBJCOPY arch/ARCH/realmode/rm/realmode.bin",
        "OBJCOPY arch/ARCH/boot/compressed/vmlinux.bin",
        "OBJCOPY arch/ARCH/boot/vmlinux.bin",
        "VDSO2C  arch/ARCH/entry/vdso/vdso-image-32.c",
        "VDSO    arch/ARCH/entry/vdso/vdso32.so.dbg",
        "RELOCS  arch/ARCH/realmode/rm/realmode.relocs",
        "PASYMS  arch/ARCH/realmode/rm/pasyms.h",
        "XZKERN  arch/ARCH/boot/compressed/vmlinux.bin.xz",
        "MKPIGGY arch/ARCH/boot/compressed/piggy.S",
        "DATAREL arch/ARCH/boot/compressed/vmlinux",
        "ZOFFSET arch/ARCH/boot/zoffset.h",
    ];

    let special = rng.choose(SPECIALS).unwrap_or(&"").to_string();
    let special = special.replace("ARCH", arch);

    format!("  {}", special)
}

/// Generates a line from `make` output
fn gen_line(arch: &str, rng: &mut ThreadRng) -> String {
    if rng.gen_bool(1.0 / 50.0) {
        gen_special(arch, rng)
    } else if rng.gen_bool(0.1) {
        gen_header(arch, rng)
    } else {
        gen_object(arch, rng)
    }
}

pub fn run(appconfig: &AppConfig) {
    let mut rng = thread_rng();
    let num_lines = rng.gen_range(50, 500);

    const ARCHES: &[&str] = &[
        "alpha",
        "arc",
        "arm",
        "arm64",
        "blackfin",
        "c6x",
        "cris",
        "frv",
        "h8300",
        "hexagon",
        "ia64",
        "m32r",
        "m68k",
        "metag",
        "microblaze",
        "mips",
        "mn10300",
        "nios2",
        "openrisc",
        "parisc",
        "powerpc",
        "s390",
        "score",
        "sh",
        "sparc",
        "tile",
        "um",
        "unicore32",
        "x86",
        "xtensa",
    ];

    let arch = rng.choose(ARCHES).unwrap_or(&"x86");

    for _ in 1..num_lines {
        let line = gen_line(arch, &mut rng);
        let sleep_length = rng.gen_range(10, 1000);

        println!("{}", line);
        csleep(sleep_length);

        if appconfig.should_exit() {
            return;
        }
    }

    println!("BUILD   arch/{}/boot/bzImage", arch);

    println!();

    let bytes: u32 = rng.gen_range(9000, 1_000_000);
    let padded_bytes: u32 = rng.gen_range(bytes, 1_100_000);

    println!("Setup is {} bytes (padded to {} bytes).",
             bytes,
             padded_bytes);

    let system: u32 = rng.gen_range(300, 3000);
    println!("System is {} kB", system);

    let crc: u32 = rng.gen_range(0x1000_0000, 0xffff_ffff);

    println!("CRC {:x}", crc);

    println!("Kernel: arch/{}/boot/bzImage is ready (#1)", arch);

    println!();
}
