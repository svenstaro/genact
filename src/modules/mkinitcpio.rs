//! Pretend to run mkinitcpio
use crate::args::AppConfig;
use crate::data::{BOOT_HOOKS_LIST, CFILES_LIST, COMPRESSION_ALGORITHMS_LIST, OS_RELEASES_LIST};
use crate::io::{csleep, print};
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

async fn warn(msg: &str) {
    print(format!(
        "{}{}",
        Paint::yellow("==> WARNING: ").bold(),
        Paint::new(msg).bold()
    ))
    .await;
    print("\r\n").await;
}

async fn msg1(msg: &str) {
    print(format!(
        "{}{}",
        Paint::green("==> ").bold(),
        Paint::new(msg).bold()
    ))
    .await;
    print("\r\n").await;
}

async fn msg2(msg: &str) {
    print(format!(
        "{}{}",
        Paint::blue("  -> ").bold(),
        Paint::new(msg).bold()
    ))
    .await;
    print("\r\n").await;
}

async fn build(
    hooks: &[&str],
    preset: &str,
    mode: &str,
    zip: &str,
    drivers: &[&str],
    os_release: &str,
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
    )
    .await;

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
    )
    .await;
    msg1(format!("Starting build: {}", os_release).as_ref()).await;

    for hook in hooks {
        msg2(format!("Running build hook: [{}]", hook).as_ref()).await;
        csleep(rng.gen_range(50, 1000)).await;

        if *hook == "block" && mode == "fallback" {
            for driver in drivers {
                warn(format!("Possibly missing firmware for module: {}", driver).as_ref()).await;
            }
        }

        if appconfig.should_exit() {
            return;
        }
    }

    msg1("Generating module dependencies").await;
    csleep(rng.gen_range(200, 500)).await;

    msg1(
        format!(
            "Creating {zip}-compressed initcpio image: {image}",
            image = image,
            zip = zip
        )
        .as_ref(),
    )
    .await;
    csleep(rng.gen_range(500, 2500)).await;

    msg1("Image generation successful").await;
}

pub async fn run(appconfig: &AppConfig) {
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
            let file = CFILES_LIST.choose(&mut rng).unwrap();

            if let Some(m) = re.captures(file) {
                ret.push(m.get(1).unwrap().as_str());
            }
        }
        ret
    };

    // For now, the preset is always the same.
    let preset = "linux";
    let os_release = OS_RELEASES_LIST.choose(&mut rng).unwrap();
    let zip = COMPRESSION_ALGORITHMS_LIST.choose(&mut rng).unwrap();

    build(
        &hooks, preset, "default", zip, &drivers, os_release, &appconfig,
    )
    .await;
    build(
        &hooks, preset, "fallback", zip, &drivers, os_release, &appconfig,
    )
    .await;
}
