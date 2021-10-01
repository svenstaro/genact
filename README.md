# genact - a nonsense activity generator

[![CI](https://github.com/svenstaro/genact/workflows/CI/badge.svg)](https://github.com/svenstaro/genact/actions)
[![DockerHub](https://img.shields.io/docker/cloud/build/svenstaro/genact.svg?style=flat)](https://cloud.docker.com/repository/docker/svenstaro/genact)
[![Crates.io](https://img.shields.io/crates/v/genact.svg)](https://crates.io/crates/genact)
[![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/svenstaro/genact/blob/master/LICENSE)
[![Stars](https://img.shields.io/github/stars/svenstaro/genact.svg)](https://github.com/svenstaro/genact/stargazers)
[![Downloads](https://img.shields.io/github/downloads/svenstaro/genact/total.svg)](https://github.com/svenstaro/genact/releases)

**Pretend to be busy or waiting for your computer when you should actually be doing real work!** Impress people with your insane multitasking skills. Just open a few instances of `genact` and watch the show. `genact` has multiple scenes that pretend to be doing something exciting or useful when in reality nothing is happening at all.

![](gifs/cc.gif)
![](gifs/memdump.gif)
![](gifs/cargo.gif)

## Installation

<a href="https://repology.org/project/genact/versions"><img align="right" src="https://repology.org/badge/vertical-allrepos/genact.svg" alt="Packaging status"></a>

You don't have to install anything! For your convenience, prebuilt binaries for Linux, OSX and Windows are provided [here](https://github.com/svenstaro/genact/releases) that should run without any dependencies. **Additionally, there is a web version at https://svenstaro.github.io/genact/**

It's compatible with FreeBSD, Linux, macOS, Windows 10 (it needs a recent Windows 10 to get ANSI support) and most modern web browsers that support WebAssembly.

**On FreeBSD**: You don't have to do anything special here. Just run

    pkg install genact
    genact

**On Linux**: Download `genact-linux` from [the releases page](https://github.com/svenstaro/genact/releases) and run

    chmod +x genact-linux
    ./genact-linux

**On macOS**: Download `genact-osx` from [the releases page](https://github.com/svenstaro/genact/releases) and run

    chmod +x genact-osx
    ./genact-osx

A homebrew package is also available. To install it this way, run

    brew install genact
    
On macOS, you can also install via MacPorts:

    sudo port install genact

**On Windows**: Download `genact-win.exe` from [the releases page](https://github.com/svenstaro/genact/releases) and double click it.

**With Cargo**: If you have a somewhat recent version of Rust and Cargo installed, you can run

    cargo install genact
    genact

## Running

To see a list of all available options, you can run

    ./genact -h

or

    cargo run -- -h

or (on Docker)

    docker run -it --rm svenstaro/genact -h

### Usage

    genact 0.11.0
    Sven-Hendrik Haase <svenstaro@gmail.com>
    A nonsense activity generator

    USAGE:
        genact [FLAGS] [OPTIONS]

    FLAGS:
        -h, --help            Prints help information
        -l, --list-modules    List available modules
        -V, --version         Prints version information

    OPTIONS:
            --exit-after-modules <exit-after-modules>    Exit after running this many modules
            --exit-after-time <exit-after-time>          Exit after running for this long (format example: 2h10min)
        -m, --modules <modules>...
                Run only these modules [possible values: botnet, bootlog, kernel_compile,
                memdump, cargo, cc, composer, docker_build, cryptomining,
                download, mkinitcpio, weblog, docker_image_rm, simcity]
        -s, --speed-factor <speed-factor>                Global speed factor [default: 1]

### Web usage

In the web version, you can run specific modules by providing them as `?module`
parameters like this: https://svenstaro.github.io/genact?module=cc&module=memdump

You can also provide a `?speed-factor` like this:
https://svenstaro.github.io/genact?speed-factor=5

## Building

You should have a recent version of rust and cargo installed.

Then, just clone it like usual and `cargo run` to get output:

    git clone https://github.com/svenstaro/genact.git
    cd genact
    cargo run --release

## Releasing

This is mostly a note for me on how to release this thing:

- Make sure `CHANGELOG.md` is up to date.
- `cargo release --dry-run`
- `cargo release`
- Releases will automatically be deployed by Github Actions.
- Docker images will automatically be built by Docker Hub.
- Update Arch package.
