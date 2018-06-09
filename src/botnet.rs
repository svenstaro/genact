use parse_args::AppConfig;
use rand::{thread_rng, Rng};
use utils::{csleep, cursor_up, dprint, erase_line};
use yansi::Paint;

pub fn run(appconfig: &AppConfig) {
    let mut rng = thread_rng();
    let clusters = {
        let mut ret = vec![];
        let num_clusters = rng.gen_range(8, 16 + 1);
        for _ in 0..num_clusters {
            let num_nodes = rng.gen_range(100, 200 + 1);
            ret.push(num_nodes);
        }
        ret
    };
    let mut onlines = vec![false; clusters.len()];
    let size: usize = clusters.iter().sum();

    let mut connected = 0;

    while connected <= size {
        dprint(
            format!(
                "\rEstablishing connections: {connected:4}/{size:4}",
                connected = connected,
                size = size
            ),
            0,
        );
        connected += 1;
        csleep((rng.gen_range(0f64, 1.).powi(50) * 50.) as u64);
    }
    println!();

    csleep(300);

    for (i, nodes) in clusters.iter().enumerate() {
        dprint(
            format!(
                "  Cluster #{i:02} ({nodes:3} nodes)",
                i = i,
                nodes = nodes
            ),
            10,
        );
        println!();
        csleep(100);
        if appconfig.should_exit() {
            return;
        }
    }

    loop {
        cursor_up(onlines.len() as u64);
        {
            let nodes_with_status = clusters.iter().zip(onlines.iter());
            for (i, (nodes, online)) in nodes_with_status.enumerate() {
                erase_line();
                println!(
                        "  Cluster #{i:02} ({nodes:3} nodes) [{status:}]",
                        i = i,
                        nodes = nodes,
                        status = if *online {
                            Paint::green("online")
                        } else {
                            Paint::yellow("booting")
                        }.bold(),
                );
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
        csleep(100);
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
        csleep(300);
        dprint(format!("+ {} ", task), 10);
        csleep(600);
        dprint("[done]", 10);
        println!();
        if appconfig.should_exit() {
            return;
        }
    }

    dprint(">> Botnet update complete.", 10);
    println!();
}
