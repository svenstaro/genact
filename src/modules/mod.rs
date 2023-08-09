pub mod ansible;
pub mod bootlog;
pub mod botnet;
pub mod bruteforce;
pub mod cargo;
pub mod cc;
pub mod composer;
pub mod cryptomining;
pub mod docker_build;
pub mod docker_image_rm;
pub mod download;
pub mod julia;
pub mod kernel_compile;
pub mod memdump;
pub mod mkinitcpio;
pub mod rkhunter;
pub mod simcity;
pub mod weblog;

use async_trait::async_trait;
use std::collections::BTreeMap;

use crate::args::AppConfig;

#[async_trait(?Send)]
pub trait Module: Sync {
    fn name(&self) -> &'static str;
    fn signature(&self) -> String;
    async fn run(&self, app_config: &AppConfig);
}

lazy_static::lazy_static! {
    pub static ref ALL_MODULES: BTreeMap<&'static str, Box<dyn Module>> = {
        let mut all_modules: BTreeMap<&'static str, Box<dyn Module>> = BTreeMap::new();
        all_modules.insert("ansible", Box::new(ansible::Ansible));
        all_modules.insert("bootlog", Box::new(bootlog::Bootlog));
        all_modules.insert("botnet", Box::new(botnet::Botnet));
        all_modules.insert("bruteforce", Box::new(bruteforce::Bruteforce));
        all_modules.insert("cargo", Box::new(cargo::Cargo));
        all_modules.insert("cc", Box::new(cc::Cc));
        all_modules.insert("composer", Box::new(composer::Composer));
        all_modules.insert("cryptomining", Box::new(cryptomining::Crytomining));
        all_modules.insert("docker_build", Box::new(docker_build::DockerBuild));
        all_modules.insert("docker_image_rm", Box::new(docker_image_rm::DockerImageRm));
        all_modules.insert("download", Box::new(download::Download));
        all_modules.insert("julia", Box::new(julia::Julia));
        all_modules.insert("kernel_compile", Box::new(kernel_compile::KernelCompile));
        all_modules.insert("memdump", Box::new(memdump::Memdump));
        all_modules.insert("mkinitcpio", Box::new(mkinitcpio::Mkinitcpio));
        all_modules.insert("rkhunter", Box::new(rkhunter::RkHunter));
        all_modules.insert("simcity", Box::new(simcity::Simcity));
        all_modules.insert("weblog", Box::new(weblog::Weblog));
        all_modules
    };
}
