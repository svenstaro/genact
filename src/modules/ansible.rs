//! Pretend to run Ansible to set up some systems
use async_trait::async_trait;
use fake::faker::internet::en::*;
use fake::Fake;
use rand::prelude::*;
use rand_distr::Normal;
use yansi::Paint;

use crate::args::AppConfig;
use crate::data::ANSIBLE_ROLES_LIST;
use crate::data::ANSIBLE_TASKS_LIST;
use crate::io::{csleep, get_terminal_width, newline, print};
use crate::modules::Module;

pub struct Ansible;

async fn do_for_all_hosts(hosts: &[String], is_gather: bool) {
    let mut rng = thread_rng();

    let latency_distr = Normal::new(500.0, 100.0).unwrap();

    // To spice things up, add a mode where everything shows up as either failed, changed, or
    // skipped.
    let global_outcome = rng.gen_range(1..20);
    for host in hosts {
        let host_outcome = rng.gen_range(1..50);

        // If this is the gather task, we always want to return all ok.
        let text = if is_gather {
            Paint::green(format!("ok: [{host}]")).to_string()
        } else {
            match global_outcome {
                1 => Paint::cyan(format!("skipping: [{host}]")).to_string(),
                2 => Paint::red(format!("failed: [{host}]")).to_string(),
                3 => Paint::yellow(format!("changed: [{host}]")).to_string(),
                _ => match host_outcome {
                    1 => Paint::cyan(format!("skipping: [{host}]")).to_string(),
                    2 => Paint::red(format!("failed: [{host}]")).to_string(),
                    3..=5 => Paint::yellow(format!("changed: [{host}]")).to_string(),
                    _ => Paint::green(format!("ok: [{host}]")).to_string(),
                },
            }
        };
        print(text).await;
        newline().await;
        let sleep: f64 = latency_distr.sample(&mut rng);
        csleep(sleep.round() as u64).await;
    }
}

#[async_trait(?Send)]
impl Module for Ansible {
    fn name(&self) -> &'static str {
        "ansible"
    }

    fn signature(&self) -> String {
        "ansible-playbook".to_string()
    }

    async fn run(&self, appconfig: &AppConfig) {
        let mut rng = thread_rng();

        let term_width = get_terminal_width();
        let play_text = format!(
            "PLAY [setup {server}] ",
            server = Username().fake::<String>()
        );

        print(format!("{play_text:*<term_width$}",)).await;
        newline().await;
        csleep(rng.gen_range(1000..3000)).await;
        newline().await;

        let num_ipv4_hosts = 1..rng.gen_range(1..20);
        let num_ipv6_hosts = 1..rng.gen_range(1..20);
        let ipv4_hosts = num_ipv4_hosts
            .map(|_| IPv4().fake())
            .collect::<Vec<String>>();
        let ipv6_hosts = num_ipv6_hosts
            .map(|_| IPv6().fake::<String>().to_lowercase())
            .collect::<Vec<String>>();
        let mut hosts = [ipv4_hosts, ipv6_hosts].concat();
        hosts.shuffle(&mut rng);

        let gathering_text = "TASK [Gathering Facts] ";
        csleep(rng.gen_range(1000..3000)).await;
        print(format!("{gathering_text:*<term_width$}",)).await;
        do_for_all_hosts(&hosts, true).await;
        csleep(rng.gen_range(1000..3000)).await;

        let num_roles = rng.gen_range(3..10);
        for _ in 1..num_roles {
            let role = ANSIBLE_ROLES_LIST.choose(&mut rng).unwrap_or(&"unknown");
            let num_tasks = rng.gen_range(3..10);
            for _ in 1..num_tasks {
                newline().await;
                let task = ANSIBLE_TASKS_LIST.choose(&mut rng).unwrap_or(&"unknown");
                let task_text = format!("TASK [{role} : {task}] ");
                print(format!("{task_text:*<term_width$}")).await;
                csleep(rng.gen_range(1000..3000)).await;
                do_for_all_hosts(&hosts, false).await;

                if appconfig.should_exit() {
                    return;
                }
            }
        }
    }
}
