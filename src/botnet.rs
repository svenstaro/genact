use parse_args::AppConfig;
use rand::{thread_rng, Rng};
use utils::{csleep, dprint};
use yansi::Paint;

pub fn run(appconfig: &AppConfig) {
    let mut rng = thread_rng();
    let clusters = {
        let mut ret = vec![];
        for _ in 0..rng.gen_range(8, 16 + 1) {
            ret.push(rng.gen_range(100, 200 + 1));
        }
        ret
    };

    let mut onlines = vec![false; clusters.len()];
    let size: usize = clusters.iter().sum();

    let mut connected = 0;

    while connected <= size {
        dprint(
            format!("\rEstablishing connections: {:4}/{:4}", connected, size),
            0,
        );
        connected += 1;
        csleep((rng.gen_range(0_f64, 1.).powi(50) * 50.) as u64);
    }
    dprint("\n", 0);

    csleep(300);

    for (i, nodes) in clusters.iter().enumerate() {
        dprint(format!("  Cluster #{:02} ({:3} nodes)\n", i, nodes), 10);
        csleep(100);
        if appconfig.should_exit() {
            return;
        }
    }

    loop {
        dprint(format!("\u{001b}[{}A\n", onlines.len() + 1), 0);
        for (i, (nodes, online)) in clusters.iter().zip(onlines.iter()).enumerate() {
            dprint(
                format!(
                    "\u{001b}[2K  Cluster #{:02} ({:3} nodes) [{}]\n",
                    i,
                    nodes,
                    if *online {
                        Paint::green("online")
                    } else {
                        Paint::yellow("booting")
                    }.bold(),
                ),
                0,
            );
        }
        if onlines.iter().all(|x| *x) {
            break;
        }
        for o in &mut onlines {
            if rng.gen_range(0., 1.) > 0.95_f64 {
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
        dprint("[done]\n", 10);
        if appconfig.should_exit() {
            return;
        }
    }

    dprint(">> Botnet update complete.\n", 10);
}
