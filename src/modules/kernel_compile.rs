//! Pretend to build a Linux kernel
use rand::prelude::*;
use regex::Regex;

use crate::args::AppConfig;
use crate::data::CFILES_LIST;
use crate::io::{csleep, newline, print};

pub fn get_signature() -> &'static str {
    "sudo make install"
}

/// Generate a build step for a header file
fn gen_header(arch: &str, rng: &mut ThreadRng) -> String {
    const RARE_CMDS: &[&str] = &["SYSTBL ", "SYSHDR "];

    const CMDS: &[&str] = &["WRAP   ", "CHK    ", "UPD    "];

    let cmd = if rng.gen_bool(1.0 / 15.0) {
        RARE_CMDS.choose(rng).unwrap_or(&"")
    } else {
        CMDS.choose(rng).unwrap_or(&"")
    };

    let cfile = &CFILES_LIST.choose(rng).unwrap_or(&"");
    let mut file = format!("{}h", &cfile[..cfile.len() - 1]);

    if file.starts_with("arch") {
        let re = Regex::new(r"arch/([a-z0-9_])+/").unwrap();
        file = re
            .replace(&file, format!("arch/{}/", arch).as_str())
            .into_owned();
    }

    format!("  {} {}", cmd, file)
}

/// Generate a build step for an object file
fn gen_object(arch: &str, rng: &mut ThreadRng) -> String {
    const RARE_CMDS: &[&str] = &["HOSTCC ", "AS     "];

    let cmd = if rng.gen_bool(1.0 / 15.0) {
        RARE_CMDS.choose(rng).unwrap_or(&"")
    } else if rng.gen_bool(0.33) {
        "AR     "
    } else {
        "CC     "
    };

    let cfile = &CFILES_LIST.choose(rng).unwrap_or(&"");
    let mut file = format!("{}o", cfile[..cfile.len() - 1].to_owned());

    if file.starts_with("arch") {
        let re = Regex::new(r"arch/([a-z0-9_])+/").unwrap();
        file = re
            .replace(&file, format!("arch/{}/", arch).as_str())
            .into_owned();
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

    let special = SPECIALS.choose(rng).unwrap_or(&"").to_string();
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

pub async fn run(appconfig: &AppConfig) {
    let mut rng = thread_rng();
    let num_lines = rng.gen_range(50..500);

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

    let arch = ARCHES.choose(&mut rng).unwrap_or(&"x86");

    for _ in 1..num_lines {
        let line = gen_line(arch, &mut rng);
        let sleep_length = rng.gen_range(10..1000);

        print(line).await;
        newline().await;
        csleep(sleep_length).await;

        if appconfig.should_exit() {
            return;
        }
    }

    print(format!("BUILD   arch/{}/boot/bzImage", arch)).await;
    newline().await;

    newline().await;

    let bytes: u32 = rng.gen_range(9000..1_000_000);
    let padded_bytes: u32 = rng.gen_range(bytes..1_100_000);

    print(format!(
        "Setup is {} bytes (padded to {} bytes).",
        bytes, padded_bytes
    ))
    .await;
    newline().await;

    let system: u32 = rng.gen_range(300..3000);
    print(format!("System is {} kB", system)).await;
    newline().await;

    let crc: u32 = rng.gen_range(0x1000_0000..0xffff_ffff);

    print(format!("CRC {:x}", crc)).await;
    newline().await;

    print(format!("Kernel: arch/{}/boot/bzImage is ready (#1)", arch)).await;
    newline().await;

    newline().await;
}
