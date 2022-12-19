//! Pretend to update julia packages
use async_trait::async_trait;
use instant::Instant;
use rand::prelude::*;
use std::fmt::Display;
use yansi::Paint;

use crate::args::AppConfig;
use crate::data::JULIA_PACKAGES_LIST;
use crate::generators::gen_hex_string;
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

        // Choose `num_packages` packages and `num_artifacts` artifacts
        // non-repeating and in random order
        let num_packages = rng.gen_range(10..150);
        let num_artifacts = rng.gen_range(1..10);
        let all_packages: Vec<&Package> = JULIA_PACKAGES_LIST
            .choose_multiple(&mut rng, num_packages + num_artifacts)
            .collect();
        let (packages, artifacts) = all_packages.split_at(num_packages);

        // root project of the julia session
        let project = if rng.gen_bool(0.3) {
            "@v1.7"
        } else {
            JULIA_PACKAGES_LIST.choose(&mut rng).unwrap().name
        };

        // julia startup
        print_banner().await;
        csleep(rng.gen_range(50..150)).await;
        newline().await;
        print_julia_prompt().await;

        // wait user input
        csleep(rng.gen_range(500..2500)).await;

        // enter pkg mode
        erase_line().await;
        print_pkg_prompt(project).await;

        // wait user input
        csleep(rng.gen_range(500..1500)).await;

        // type "update" and press enter
        dprint("up", rng.gen_range(100..500)).await;
        dprint("date", rng.gen_range(100..500)).await;
        csleep(rng.gen_range(500..1500)).await;
        newline().await;

        // wait Pkg.update() startup time
        csleep(rng.gen_range(200..1000)).await;

        log_action("Updating", "registry at `~/.julia/registries/General.toml`").await;
        csleep(rng.gen_range(1500..5000)).await;

        log_action("Resolving", "package versions...").await;
        csleep(rng.gen_range(500..2500)).await;

        install_packages(packages).await;
        csleep(rng.gen_range(250..1000)).await;

        download_artifacts(artifacts).await;
        csleep(rng.gen_range(250..1000)).await;

        update_project_and_manifest(project).await;
        csleep(rng.gen_range(10..100)).await;

        report_packages(packages).await;
        csleep(rng.gen_range(150..500)).await;

        build_artifacts(artifacts).await;
        csleep(rng.gen_range(150..500)).await;

        precompile(packages).await;

        if rng.gen_bool(0.25) {
            gc().await;
        }

        // wait cleanup
        csleep(rng.gen_range(50..250)).await;

        newline().await;
        print_pkg_prompt(project).await;

        // wait user input
        csleep(rng.gen_range(500..5000)).await;

        // exit pkg mode
        erase_line().await;
        print_julia_prompt().await;

        // wait user input
        csleep(rng.gen_range(1000..7000)).await;

        // quit julia
        newline().await;

        if appconfig.should_exit() {
            return;
        }
    }
}

async fn print_banner() {
    print(format!(
        r#"               {gu}{cr}
   {bu}       _ {ru}{g}{mu}     |  Documentation: https://docs.julialang.org{cr}
  {b}     | {r} {m}    |{cr}
   _ _   _| |_  __ _   |  Type "?" for help, "]?" for Pkg help.{cr}
  | | | | | | |/ _` |  |{cr}
  | | |_| | | | (_| |  |  Version 1.7.3 (2022-05-06){cr}
 _/ |\__'_|_|_|\__'_|  |  Fedora 35 build{cr}
|__/                   |{cr}
"#,
        r = Paint::red("(_)").bold(),
        ru = Paint::red("_").bold(),
        g = Paint::green("(_)").bold(),
        gu = Paint::green("_").bold(),
        b = Paint::blue("(_)").bold(),
        bu = Paint::blue("_").bold(),
        m = Paint::magenta("(_)").bold(),
        mu = Paint::magenta("_").bold(),
        cr = '\r',
    ))
    .await;
}

async fn print_julia_prompt() {
    print(format!("{} ", Paint::green("julia>").bold())).await;
}

async fn print_pkg_prompt(project: &str) {
    print(format!(
        "{} ",
        Paint::blue(format!("({project}) pkg>")).bold()
    ))
    .await;
}

async fn log_action(action: impl Display, message: impl Display) {
    print(format!("{:>12} {}", Paint::green(action).bold(), message)).await;
    newline().await;
}

async fn log_progress(bar: &progress_string::Bar) {
    print(format!("{:>10} {}", Paint::cyan("Progress").bold(), bar)).await;
    newline().await;
}

async fn install_packages(packages: &[&Package<'_>]) {
    let mut rng = thread_rng();

    let max_name_length = packages.iter().map(|p| p.name.len()).max().unwrap();
    for package in packages {
        let package_and_version = format!(
            "{name} {empty:─>width$} v{version}",
            name = package.name,
            width = max_name_length - package.name.len() + 1,
            empty = "",
            version = package.versions.last().unwrap()
        );

        if rng.gen_bool(0.1) {
            log_action("Installing", &package_and_version).await;

            csleep(rng.gen_range(250..1000)).await;

            cursor_up(1).await;
            erase_line().await;
        } else {
            csleep(rng.gen_range(10..200)).await;
        }

        log_action("Installed", &package_and_version).await;
    }
}

async fn download_artifacts(artifacts: &[&Package<'_>]) {
    let mut rng = thread_rng();

    for artifact in artifacts {
        csleep(rng.gen_range(50..100)).await;

        log_action("Downloading", format!("artifact: {}", artifact.name)).await;

        csleep(rng.gen_range(100..150)).await;

        let mut bar = progress_string::BarBuilder::new()
            .total(10000)
            .width(41)
            .full_char('=')
            .include_percent()
            .build();

        print(format!("{:>15} {}", Paint::cyan("Downloading").bold(), bar)).await;
        newline().await;

        while bar.current_partial < bar.total {
            let add = rng.gen_range(0..=500.min(bar.total - bar.current_partial));
            bar.update(add);

            cursor_up(1).await;
            erase_line().await;
            print(format!("{:>15} {}", Paint::cyan("Downloading").bold(), bar)).await;
            newline().await;

            csleep(rng.gen_range(50..75)).await;
        }
        cursor_up(1).await;
        erase_line().await;
        csleep(rng.gen_range(100..200)).await;
        cursor_up(1).await;
        erase_line().await;

        log_action("Downloaded", format!("artifact: {}", artifact.name)).await;
    }
}

async fn update_project_and_manifest(project: &str) {
    let mut rng = thread_rng();

    let project_path;
    let manifest_path;

    if let Some(project) = project.strip_prefix('@') {
        project_path = format!("~/.julia/environments/{project}/Project.toml");
        manifest_path = format!("~/.julia/environments/{project}/Manifest.toml");
    } else {
        project_path = format!(
            "~/Documents/code/julia/projects/{project}.jl/Project.toml"
        );
        manifest_path = format!(
            "~/Documents/code/julia/projects/{project}.jl/Manifest.toml"
        );
    }

    let old_format = rng.gen_bool(0.25);

    if old_format {
        print_old_manifest_format_before(&manifest_path).await;
    }

    if rng.gen_bool(0.9) {
        log_action("Updating", format!("`{project_path}`")).await;
    } else {
        log_action("No Changes", format!("to `{project_path}`")).await;
    }

    csleep(rng.gen_range(10..100)).await;

    log_action("Updating", format!("`{manifest_path}`")).await;

    if old_format {
        print_old_manifest_format_after(&manifest_path).await;
    }
}

async fn print_old_manifest_format_before(manifest_path: &str) {
    print(format!(
        "{} The active manifest file at `{}` has an old format that is being maintained.",
        Paint::yellow("┌ Warning:").bold(),
        manifest_path
    ))
    .await;
    newline().await;
    print(format!(
        "{} To update to the new format run `Pkg.upgrade_manifest()` which will upgrade the format without re-resolving.",
        Paint::yellow("│").bold()
    ))
    .await;
    newline().await;
    print(format!(
        "{} {}",
        Paint::yellow("└").bold(),
        Paint::fixed(8, "@ Pkg.Types /builddir/build/BUILD/julia-1.7.3/build/usr/share/julia/stdlib/v1.7/Pkg/src/manifest.jl:287")
    ))
    .await;
    newline().await;
}

async fn print_old_manifest_format_after(manifest_path: &str) {
    print(format!(
        "{} The active manifest file is an older format with no julia version entry. Dependencies may have been resolved with a different julia version.",
        Paint::yellow("┌ Warning:").bold(),
    ))
    .await;
    newline().await;
    print(format!(
        "{} {}",
        Paint::yellow("└").bold(),
        Paint::fixed(8, format!("@ {manifest_path}:0"))
    ))
    .await;
    newline().await;
}

async fn report_packages(packages: &[&Package<'_>]) {
    let mut rng = thread_rng();

    for package in packages {
        if package.versions.len() > 1 && rng.gen_bool(0.75) {
            // update package
            print(format!(
                "  {} {}",
                Paint::fixed(8, format!("[{}]", &package.id[0..8])),
                Paint::fixed(
                    11, // bright yellow
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
        } else if rng.gen_bool(0.9) {
            // add package
            print(format!(
                "  {} {}",
                Paint::fixed(8, format!("[{}]", &package.id[0..8])),
                Paint::fixed(
                    10, // bright green
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
                    9, // bright red
                    format!("- {} v{}", package.name, package.versions.last().unwrap())
                )
            ))
            .await;
            newline().await;
        }

        csleep(rng.gen_range(10..100)).await;
    }
}

async fn build_artifacts(artifacts: &[&Package<'_>]) {
    let mut rng = thread_rng();

    let mut bar = progress_string::BarBuilder::new()
        .total(artifacts.len())
        .width(41)
        .full_char('=')
        .include_numbers()
        .build();

    log_progress(&bar).await;

    let max_name_length = artifacts.iter().map(|p| p.name.len()).max().unwrap();
    for artifact in artifacts {
        bar.update(1);

        cursor_up(1).await;
        erase_line().await;

        log_action(
            "Building",
            format!(
                "{name} {empty:─>width$}→ `~/.julia/scratchspaces/{}-{}-{}-{}-{}/{}/build.log`",
                gen_hex_string(&mut rng, 8),
                gen_hex_string(&mut rng, 4),
                gen_hex_string(&mut rng, 4),
                gen_hex_string(&mut rng, 4),
                gen_hex_string(&mut rng, 12),
                gen_hex_string(&mut rng, 40),
                name = artifact.name,
                width = max_name_length - artifact.name.len() + 1,
                empty = "",
            ),
        )
        .await;

        log_progress(&bar).await;

        csleep(rng.gen_range(500..5000)).await;
    }

    // erase progress bar
    cursor_up(1).await;
    erase_line().await;
}

async fn precompile(packages: &[&Package<'_>]) {
    let mut rng = thread_rng();

    let num_packages = packages.len();

    let now = Instant::now();

    log_action("Precompiling", "project...").await;

    let mut bar = progress_string::BarBuilder::new()
        .total(num_packages)
        .width(41)
        .full_char('=')
        .include_numbers()
        .build();

    log_progress(&bar).await;

    for i in 1..=num_packages {
        bar.replace(i);

        cursor_up(1).await;
        erase_line().await;
        log_progress(&bar).await;

        csleep(rng.gen_range(50..1000)).await;
    }

    let elapsed = now.elapsed();
    let seconds = elapsed.as_secs() as f32;

    cursor_up(1).await;
    erase_line().await;
    print(format!(
            "  {num_packages} dependencies successfully precompiled in {seconds:.0} seconds ({} already precompiled)",
            rng.gen_range(10..500)
        ))
        .await;
    newline().await;
}

async fn gc() {
    let mut rng = thread_rng();

    csleep(rng.gen_range(50..250)).await;

    print(format!(
        "{} We haven't cleaned this depot up for a bit, running Pkg.gc()...",
        Paint::cyan("[ Info:").bold(),
    ))
    .await;
    newline().await;

    csleep(rng.gen_range(100..250)).await;

    log_action(
        "Active",
        format!("manifest files: {} found", rng.gen_range(1..10)),
    )
    .await;

    csleep(rng.gen_range(100..250)).await;

    log_action(
        "Active",
        format!("artifact files: {} found", rng.gen_range(10..200)),
    )
    .await;

    csleep(rng.gen_range(100..250)).await;

    log_action(
        "Active",
        format!("scratchspaces: {} found", rng.gen_range(10..20)),
    )
    .await;

    csleep(rng.gen_range(100..250)).await;

    log_action(
        "Deleted",
        format!(
            "{} package installations ({:.3} MiB)",
            rng.gen_range(2..100),
            rng.gen_range(10f32..250f32)
        ),
    )
    .await;

    csleep(rng.gen_range(100..250)).await;

    log_action(
        "Deleted",
        format!(
            "{} artifact installations ({:.3} MiB)",
            rng.gen_range(2..10),
            rng.gen_range(10f32..250f32)
        ),
    )
    .await;

    csleep(rng.gen_range(100..250)).await;

    log_action(
        "Deleted",
        format!(
            "{} scratchspaces ({:.3} byte)",
            rng.gen_range(2..10),
            rng.gen_range(10f32..1000f32)
        ),
    )
    .await;
}
