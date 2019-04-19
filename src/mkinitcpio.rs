use crate::parse_args::AppConfig;
use crate::utils::csleep;
use rand::prelude::*;
use rand::seq::SliceRandom;
use yansi::Paint;

const ALL_HOOKS: &'static [&str] = &[
    &"base",
    &"udev",
    &"usr",
    &"resume",
    &"autodetect",
    &"modconf",
    &"block",
    &"net",
    &"mdadm",
    &"mdadm_udev",
    &"keyboard",
    &"keymap",
    &"consolefont",
    &"encrypt",
    &"lvm2",
    &"fsck",
    &"filesystems",
];

const REQUIRED_HOOKS: &'static [&str] = &[
    &"base",
    &"udev",
    &"autodetect",
    &"modconf",
    &"block",
    &"fsck",
    &"filesystems",
];

const ZIP_FORMATS: &'static [&str] = &[&"gzip", &"bzip2", &"lzma", &"xz", &"lzop", &"lz4"];

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

fn build(hooks: &[&str], preset: &str, mode: &str, zip: &str, appconfig: &AppConfig) {
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
            warn("Possibly missing firmware for module: aic94xx");
            warn("Possibly missing firmware for module: wd719x");
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

    let hooks = {
        let mut ret: Vec<&str> = vec![];
        for hook in ALL_HOOKS {
            if REQUIRED_HOOKS.contains(hook) || rng.gen_range(0, 10) < 3 {
                ret.push(&hook);
            }
        }
        ret
    };

    let preset = "linux";

    match ZIP_FORMATS.choose(&mut rng) {
        Some(zip) => {
            build(&hooks, preset, "default", zip, &appconfig);
            build(&hooks, preset, "fallback", zip, &appconfig);
        }
        _ => {}
    };
}
