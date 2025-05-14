//! Pretend to run mkinitcpio
use async_trait::async_trait;
use rand::seq::IndexedRandom;
use rand::{Rng, rng};
use regex::Regex;
use yansi::Paint;

use crate::args::AppConfig;
use crate::data::{BOOT_HOOKS_LIST, CFILES_LIST, COMPRESSION_FORMATS_LIST, OS_RELEASES_LIST};
use crate::io::{csleep, newline, print};
use crate::modules::Module;

const REQUIRED_HOOKS: &[&str] = &[
    "base",
    "udev",
    "autodetect",
    "modconf",
    "block",
    "fsck",
    "filesystems",
];

async fn warn(msg: &str) {
    print(format!(
        "{}{}",
        Paint::yellow("==> WARNING: ").bold(),
        Paint::new(msg).bold()
    ))
    .await;
    newline().await;
}

async fn msg1(msg: &str) {
    print(format!(
        "{}{}",
        Paint::green("==> ").bold(),
        Paint::new(msg).bold()
    ))
    .await;
    newline().await;
}

async fn msg2(msg: &str) {
    print(format!(
        "{}{}",
        Paint::blue("  -> ").bold(),
        Paint::new(msg).bold()
    ))
    .await;
    newline().await;
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
    let mut rng = rng();

    msg1(
        format!("Building image from preset: /etc/mkinitcpio.d/{preset}.preset: '{mode}'",)
            .as_ref(),
    )
    .await;

    let image = format!(
        "/boot/initramfs-{preset}{suffix}.img",
        preset = preset,
        suffix = if mode == "default" {
            "".to_string()
        } else {
            format!("-{mode}")
        }
    );

    msg2(format!("-k /boot/vmlinuz-{preset} -c /etc/mkinitcpio.conf -g {image}",).as_ref()).await;
    msg1(format!("Starting build: {os_release}").as_ref()).await;

    for hook in hooks {
        msg2(format!("Running build hook: [{hook}]").as_ref()).await;
        csleep(rng.random_range(50..1000)).await;

        if *hook == "block" && mode == "fallback" {
            for driver in drivers {
                warn(format!("Possibly missing firmware for module: {driver}").as_ref()).await;
            }
        }

        if appconfig.should_exit() {
            return;
        }
    }

    msg1("Generating module dependencies").await;
    csleep(rng.random_range(200..500)).await;

    msg1(format!("Creating {zip}-compressed initcpio image: {image}",).as_ref()).await;
    csleep(rng.random_range(500..2500)).await;

    msg1("Image generation successful").await;
}

pub struct Mkinitcpio;

#[async_trait(?Send)]
impl Module for Mkinitcpio {
    fn name(&self) -> &'static str {
        "mkinitcpio"
    }

    fn signature(&self) -> String {
        "mkinitcpio --generate /boot/initramfs-custom2.img --kernel 5.7.12-arch1-1".to_string()
    }

    async fn run(&self, appconfig: &AppConfig) {
        let mut rng = rng();

        // Select a few hooks from the list of all hooks (in order). Make sure the required default
        // hooks are also included (also, in order).
        let hooks = {
            let mut ret: Vec<&str> = vec![];
            for hook in BOOT_HOOKS_LIST.iter() {
                if REQUIRED_HOOKS.contains(hook) || rng.random_range(0..10) < 3 {
                    ret.push(hook);
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

            let count = rng.random_range(0..5);
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
        let zip = COMPRESSION_FORMATS_LIST.choose(&mut rng).unwrap();

        build(
            &hooks, preset, "default", zip, &drivers, os_release, appconfig,
        )
        .await;
        build(
            &hooks, preset, "fallback", zip, &drivers, os_release, appconfig,
        )
        .await;
    }
}
