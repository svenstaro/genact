/// Module that pretends to build a Linux kernel

use rand::{thread_rng, Rng};

use utils::csleep;
use KERNEL_LIST;

pub fn run() {
    let mut rng = thread_rng();
    let num_lines = rng.gen_range(50, 500);

    for _ in 1..num_lines {
        let choice = rng.choose(&KERNEL_LIST).unwrap_or(&"");
        let sleep_length = rng.gen_range(10, 1000);

        println!("{}", choice);
        csleep(sleep_length);
    }

    let bytes: u32 = rng.gen_range(9000, 1000000);
    let padded_bytes: u32 = rng.gen_range(bytes, 1100000);

    println!("Setup is {} bytes (padded to {} bytes).",
             bytes,
             padded_bytes);

    let system: u32 = rng.gen_range(300, 3000);
    println!("System is {} kB", system);

    let crc: u32 = rng.gen_range(0x10000000, 0xffffffff);

    println!("CRC {:x}", crc);

    println!("Kernel: arch/x86/boot/bzImage is ready (#1)");

    println!();
}
