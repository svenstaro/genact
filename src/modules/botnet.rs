//! Pretend to run and orchestrate a botnet
use async_trait::async_trait;
use rand::prelude::*;
use yansi::Paint;

use crate::args::AppConfig;
use crate::io::{csleep, cursor_up, dprint, erase_line, newline, print};
use crate::modules::Module;

pub struct Botnet;

#[async_trait(?Send)]
impl Module for Botnet {
    fn name(&self) -> &'static str {
        "botnet"
    }

    fn signature(&self) -> String {
        "./botnet.sh".to_string()
    }

    async fn run(&self, appconfig: &AppConfig) {
        let mut rng = thread_rng();
        let clusters = {
            let mut ret = vec![];
            let num_clusters = rng.gen_range(8..16 + 1);
            for _ in 0..num_clusters {
                let num_nodes = rng.gen_range(100..200 + 1);
                ret.push(num_nodes);
            }
            ret
        };
        let mut onlines = vec![false; clusters.len()];
        let size: usize = clusters.iter().sum();

        let mut connected = 0;

        while connected <= size {
            print(format!(
                "\rEstablishing connections: {connected:4}/{size:4}",
                connected = connected,
                size = size
            ))
            .await;
            connected += 1;
            csleep((rng.gen_range(0f64..1.).powi(50) * 50.) as u64).await;
        }
        dprint("\r\n", 0).await;

        csleep(300).await;

        for (i, nodes) in clusters.iter().enumerate() {
            dprint(
                format!("  Cluster #{i:02} ({nodes:3} nodes)", i = i, nodes = nodes),
                10,
            )
            .await;
            newline().await;
            csleep(100).await;
            if appconfig.should_exit() {
                return;
            }
        }

        loop {
            cursor_up(onlines.len() as u64).await;
            {
                let nodes_with_status = clusters.iter().zip(onlines.iter());
                for (i, (nodes, online)) in nodes_with_status.enumerate() {
                    erase_line().await;
                    print(format!(
                        "  Cluster #{i:02} ({nodes:3} nodes) [{status:}]",
                        i = i,
                        nodes = nodes,
                        status = if *online {
                            Paint::green("online")
                        } else {
                            Paint::yellow("booting")
                        }
                        .bold(),
                    ))
                    .await;
                    newline().await;
                }
            }
            if onlines.iter().all(|x| *x) {
                break;
            }
            for o in &mut onlines {
                let success_rate = 0.05;
                if rng.gen_bool(success_rate) {
                    *o = true;
                }
            }
            csleep(100).await;
            if appconfig.should_exit() {
                return;
            }
        }

        let tasks = [
            "Synchronizing clocks...",
            "Sending login information...",
            "Sending command...",
        ];

        for task in &tasks {
            csleep(300).await;
            dprint(format!("+ {} ", task), 10).await;
            csleep(600).await;
            dprint("[done]", 10).await;
            newline().await;
            if appconfig.should_exit() {
                return;
            }
        }

        dprint(">> Botnet update complete.", 10).await;
        newline().await;
    }
}
