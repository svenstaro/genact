use crate::parse_args::AppConfig;
use crate::utils::csleep;
use crate::{BOOT_HOOKS_LIST, CFILES_LIST, COMPRESSION_ALGORITHMS_LIST};
use rand::prelude::*;
use rand::seq::SliceRandom;
use regex::Regex;
use yansi::Paint;

const REQUIRED_HOOKS: &[&str] = &[
    &"base",
    &"udev",
    &"autodetect",
    &"modconf",
    &"block",
    &"fsck",
    &"filesystems",
];

fn warn(msg: &str) {
    println!(
        "{}{}",
        Paint::yellow("==> WARNING: ").bold(),
        Paint::new(msg).bold()
    );
}

fn msg1(msg: &str) {
    println!("{}{}", Paint::green("==> ").bold(), Paint::new(msg).bold());
}

fn msg2(msg: &str) {
    println!("{}{}", Paint::blue("  -> ").bold(), Paint::new(msg).bold());
}

fn build(
    hooks: &[&str],
    preset: &str,
    mode: &str,
    zip: &str,
    drivers: &[&str],
    appconfig: &AppConfig,
) {
    let mut rng = thread_rng();

    msg1(
        format!(
            "Building image from preset: /etc/mkinitcpio.d/{preset}.preset: '{mode}'",
            preset = preset,
            mode = mode
        )
        .as_ref(),
    );

    let image = format!(
        "/boot/initramfs-{preset}{suffix}.img",
        preset = preset,
        suffix = if mode == "default" {
            "".to_string()
        } else {
            format!("-{}", mode)
        }
    );

    msg2(
        format!(
            "-k /boot/vmlinuz-{preset} -c /etc/mkinitcpio.conf -g {image}",
            preset = preset,
            image = image
        )
        .as_ref(),
    );
    msg1("Starting build: 5.0.7-arch1-1-ARCH");

    for hook in hooks {
        msg2(format!("Running build hook: [{}]", hook).as_ref());
        csleep(rng.gen_range(50, 1000));

        if *hook == "block" && mode == "fallback" {
            for driver in drivers {
                warn(format!("Possibly missing firmware for module: {}", driver).as_ref());
            }
        }

        if appconfig.should_exit() {
            return;
        }
    }

    msg1("Generating module dependencies");
    csleep(rng.gen_range(200, 500));

    msg1(
        format!(
            "Creating {zip}-compressed initcpio image: {image}",
            image = image,
            zip = zip
        )
        .as_ref(),
    );
    csleep(rng.gen_range(500, 2500));

    msg1("Image generation successful");
}

pub fn run(appconfig: &AppConfig) {
    let mut rng = thread_rng();

    // Select a few hooks from the list of all hooks (in order). Make sure the required default
    // hooks are also included (also, in order).
    let hooks = {
        let mut ret: Vec<&str> = vec![];
        for hook in BOOT_HOOKS_LIST.iter() {
            if REQUIRED_HOOKS.contains(hook) || rng.gen_range(0, 10) < 3 {
                ret.push(&hook);
            }
        }
        ret
    };

    // Find some "drivers" that cannot find firmware in fallback mode, by identifying files in the
    // kernel under driverr/scsi/**/*.c and use their file name (without extension) as the kernel
    // module name. It may not be 100% what happens, but it's close enough and looks reasonable.
    let drivers = {
        let mut ret: Vec<&str> = vec![];

        let re = Regex::new(r"^drivers/scsi.*/([^/\.]+).c$").unwrap();

        let count = rng.gen_range(0, 5);
        while ret.len() < count {
            if let Some(file) = CFILES_LIST.choose(&mut rng) {
                if let Some(m) = re.captures(file) {
                    ret.push(m.get(1).unwrap().as_str());
                }
            }
        }
        ret
    };

    // For now, the preset is always the same.
    let preset = "linux";

    if let Some(zip) = COMPRESSION_ALGORITHMS_LIST.choose(&mut rng) {
        build(&hooks, preset, "default", zip, &drivers, &appconfig);
        build(&hooks, preset, "fallback", zip, &drivers, &appconfig);
    };
}
