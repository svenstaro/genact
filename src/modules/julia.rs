//! Pretend to update julia packages
use async_trait::async_trait;
use instant::Instant;
use rand::prelude::*;
use yansi::Paint;

use crate::args::AppConfig;
use crate::data::JULIA_PACKAGES_LIST;
// use crate::generators::gen_package_version;
use crate::io::{csleep, cursor_up, dprint, erase_line, newline, print};
use crate::modules::Module;

#[derive(Debug)]
pub struct Package<'a> {
    pub name: &'a str,
    pub id: &'a str,
    pub versions: Vec<&'a str>,
}

pub struct Julia;

#[async_trait(?Send)]
impl Module for Julia {
    fn name(&self) -> &'static str {
        "julia"
    }

    fn signature(&self) -> String {
        "julia --threads auto --project .".to_string()
    }

    async fn run(&self, appconfig: &AppConfig) {
        let mut rng = thread_rng();

        // Choose `num_packages` packages, non-repeating and in random order
        let num_packages = rng.gen_range(10..150);
        let chosen_packages: Vec<_> = JULIA_PACKAGES_LIST
            .choose_multiple(&mut rng, num_packages)
            .collect();
        // root project of the julia session
        let project = if rng.gen::<f32>() < 0.3f32 {
            "@v1.7"
        } else {
            &JULIA_PACKAGES_LIST.choose(&mut rng).unwrap().name
        };

        print(format!(
            r#"
               {gu}
   {bu}       _ {ru}{g}{mu}     |  Documentation: https://docs.julialang.org
  {b}     | {r} {m}    |
   _ _   _| |_  __ _   |  Type "?" for help, "]?" for Pkg help.
  | | | | | | |/ _` |  |
  | | |_| | | | (_| |  |  Version 1.7.3 (2022-05-06)
 _/ |\__'_|_|_|\__'_|  |  Fedora 35 build
|__/                   |

"#,
            r = Paint::red("(_)").bold(),
            ru = Paint::red("_").bold(),
            g = Paint::green("(_)").bold(),
            gu = Paint::green("_").bold(),
            b = Paint::blue("(_)").bold(),
            bu = Paint::blue("_").bold(),
            m = Paint::magenta("(_)").bold(),
            mu = Paint::magenta("_").bold(),
        ))
        .await;
        print(format!("{}", Paint::green("julia> ").bold())).await;

        csleep(rng.gen_range(500..2500)).await;

        erase_line().await;
        print(format!(
            "{} ",
            Paint::blue(format!("({}) pkg>", project)).bold()
        ))
        .await;
        csleep(rng.gen_range(500..1500)).await;
        dprint("update", rng.gen_range(100..500)).await;
        newline().await;

        csleep(rng.gen_range(200..1000)).await;

        print(format!(
            "{:>12} registry at `~/.julia/registries/General.toml`",
            Paint::green("Updating").bold()
        ))
        .await;
        newline().await;

        csleep(rng.gen_range(1500..5000)).await;

        print(format!(
            "{:>12} package versions...",
            Paint::green("Resolving").bold()
        ))
        .await;
        newline().await;

        csleep(rng.gen_range(500..2500)).await;

        let max_name_length = chosen_packages.iter().map(|p| p.name.len()).max().unwrap();
        for package in &chosen_packages {
            if rng.gen::<f32>() < 0.1f32 {
                print(format!(
                    "{:>12} {name} {empty:─>width$} v{version}",
                    Paint::green("Installing").bold(),
                    name = package.name,
                    width = max_name_length - package.name.len() + 1,
                    empty = "",
                    version = package.versions.last().unwrap()
                ))
                .await;
                newline().await;

                csleep(rng.gen_range(250..1000)).await;

                cursor_up(1).await;
                erase_line().await;
            } else {
                csleep(rng.gen_range(10..100)).await;
            }

            print(format!(
                "{:>12} {name} {empty:─>width$} v{version}",
                Paint::green("Installed").bold(),
                name = package.name,
                width = max_name_length - package.name.len() + 1,
                empty = "",
                version = package.versions.last().unwrap()
            ))
            .await;
            newline().await;
        }

        csleep(rng.gen_range(250..1000)).await;

        if project.starts_with('@') {
            if rng.gen::<f32>() < 0.9f32 {
                print(format!(
                    "{:>12} `~/.julia/environments/{}/Project.toml`",
                    Paint::green("Updating").bold(),
                    &project[1..]
                ))
                .await;
                newline().await;
            } else {
                print(format!(
                    "{:>12} to `~/.julia/environments/{}/Project.toml`",
                    Paint::green("No Changes").bold(),
                    &project[1..]
                ))
                .await;
                newline().await;
            }

            csleep(rng.gen_range(10..100)).await;

            print(format!(
                "{:>12} `~/.julia/environments/{}/Manifest.toml`",
                Paint::green("Updating").bold(),
                &project[1..]
            ))
            .await;
            newline().await;
        } else {
            if rng.gen::<f32>() < 0.9f32 {
                print(format!(
                    "{:>12} `~/Documents/code/julia/projects/{}.jl/Project.toml`",
                    Paint::green("Updating").bold(),
                    project
                ))
                .await;
                newline().await;
            } else {
                print(format!(
                    "{:>12} to `~/Documents/code/julia/projects/{}.jl/Project.toml`",
                    Paint::green("No Changes").bold(),
                    project
                ))
                .await;
                newline().await;
            }

            csleep(rng.gen_range(10..100)).await;

            print(format!(
                "{:>12} `~/Documents/code/julia/projects/{}.jl/Manifest.toml`",
                Paint::green("Updating").bold(),
                project
            ))
            .await;
            newline().await;
        }

        csleep(rng.gen_range(10..100)).await;

        for package in &chosen_packages {
            if package.versions.len() > 1 && rng.gen::<f32>() < 0.75f32 {
                // update package
                print(format!(
                    "  {} {}",
                    Paint::fixed(8, format!("[{}]", &package.id[0..8])),
                    Paint::fixed(
                        11,
                        format!(
                            "↑ {} v{} ⇒ v{}",
                            package.name,
                            package.versions[rng.gen_range(0..package.versions.len() - 1)],
                            package.versions.last().unwrap()
                        )
                    )
                ))
                .await;
                newline().await;
            } else if rng.gen::<f32>() < 0.9f32 {
                // add package
                print(format!(
                    "  {} {}",
                    Paint::fixed(8, format!("[{}]", &package.id[0..8])),
                    Paint::fixed(
                        10,
                        format!("+ {} v{}", package.name, package.versions.last().unwrap())
                    )
                ))
                .await;
                newline().await;
            } else {
                // remove package
                print(format!(
                    "  {} {}",
                    Paint::fixed(8, format!("[{}]", &package.id[0..8])),
                    Paint::fixed(
                        9,
                        format!("- {} v{}", package.name, package.versions.last().unwrap())
                    )
                ))
                .await;
                newline().await;
            }

            csleep(rng.gen_range(10..100)).await;
        }

        csleep(rng.gen_range(150..500)).await;

        let now = Instant::now();
        print(format!(
            "{:>12} project...",
            Paint::green("Precompiling").bold()
        ))
        .await;
        newline().await;

        print(format!(
            "  {} [====================================>    ]  30/34",
            Paint::cyan("Progress").bold()
        ))
        .await;
        newline().await;

        csleep(rng.gen_range(1000..10000)).await;

        let elapsed = now.elapsed();
        let seconds = elapsed.as_secs() as f32;

        print(format!(
            "  {num_packages} dependencies successfully precompiled in {seconds:.0} seconds ({} already precompiled)",
            rng.gen_range(10..500)
        ))
        .await;
        newline().await;

        csleep(rng.gen_range(50..250)).await;

        newline().await;
        print(format!(
            "{} ",
            Paint::blue(format!("({}) pkg>", project)).bold()
        ))
        .await;

        csleep(rng.gen_range(500..5000)).await;

        erase_line().await;
        print(format!("{}", Paint::green("julia> ").bold())).await;

        csleep(rng.gen_range(1000..7000)).await;

        newline().await;

        if appconfig.should_exit() {
            return;
        }
    }
}
