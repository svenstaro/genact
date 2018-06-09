# genact - a nonsense activity generator

[![Build Status](https://travis-ci.org/svenstaro/genact.svg?branch=master)](https://travis-ci.org/svenstaro/genact)
[![Snap Status](https://build.snapcraft.io/badge/svenstaro/genact.svg)](https://build.snapcraft.io/user/svenstaro/genact)
[![AUR](https://img.shields.io/aur/version/genact.svg)](https://aur.archlinux.org/packages/genact/)
[![Crates.io](https://img.shields.io/crates/v/genact.svg)](https://crates.io/crates/genact)
[![dependency status](https://deps.rs/repo/github/svenstaro/genact/status.svg)](https://deps.rs/repo/github/svenstaro/genact)
[![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/svenstaro/genact/blob/master/LICENSE)
[![Stars](https://img.shields.io/github/stars/svenstaro/genact.svg)](https://github.com/svenstaro/genact/stargazers)
[![Downloads](https://img.shields.io/github/downloads/svenstaro/genact/total.svg)](https://github.com/svenstaro/genact/releases)

Pretend to be busy or waiting for your computer when you should actually be doing real work! Impress people with your insane multitasking skills. Just open a few instances of `genact` and watch the show. `genact` has multiple scenes that pretend to be doing something exciting or useful when in reality nothing is happening at all.

![](https://svenstaro.org/genact/cc.gif)
![](https://svenstaro.org/genact/memdump.gif)
![](https://svenstaro.org/genact/cargo.gif)

## Installation

You don't have to install anything! For your convenience, prebuilt binaries for Linux, OSX and Windows are provided [here](https://github.com/svenstaro/genact/releases) that should run without any dependencies. **Additionally, there is a web version at https://svenstaro.github.io/genact/**

It's compatible with FreeBSD, Linux, OSX, Windows 10 (it needs a recent Windows 10 to get ANSI support) and most modern web browsers that support WebAssembly.

**On FreeBSD**: You don't have to do anything special here. Just run

    pkg install genact
    genact

**On Linux**: Download `genact-linux` from [the releases page](https://github.com/svenstaro/genact/releases) and run

    chmod +x genact-linux
    ./genact-linux

**On OSX**: Download `genact-osx` from [the releases page](https://github.com/svenstaro/genact/releases) and run

    chmod +x genact-osx
    ./genact-osx

**On Windows**: Download `genact-win.exe` from [the releases page](https://github.com/svenstaro/genact/releases) and double click it.

**With Cargo**: If you have a somewhat recent version of Rust and Cargo installed, you can run

    cargo install genact
    genact

**With snap**: If you'd like to use [snapcraft](https://snapcraft.io/), you can just get it from the [snap store](https://snapcraft.io/store/):

    snap install genact

To build it yourself using snap, run:

    snapcraft cleanbuild
    snap install genact_*.snap --devmode --dangerous

## Running

To see a list of all available options, you can run

    ./genact -h

or

    cargo run -- -h

or (on Docker)

    docker run -it --rm svenstaro/genact -h

The help:

    genact 0.6.0
    Sven-Hendrik Haase <svenstaro@gmail.com>
    A nonsense activity generator

    USAGE:
        genact [FLAGS] [OPTIONS]

    FLAGS:
        -h, --help            Prints help information
        -l, --list-modules    List available modules
        -V, --version         Prints version information

    OPTIONS:
        -e, --exitafter <EXITAFTER>    Exit after running for this long (format example: 2h10min)
        -m, --modules <MODULE>...      Run only these modules [values: bootlog, botnet, cargo, cc, composer,
                                       cryptomining, simcity, download, memdump, kernel_compile, weblog]

In the web version, you can run specific modules by providing them as `?module`
parameters like this: https://svenstaro.github.io/genact?module=cc&module=memdump

## Compiling

You should have a recent version of rust and cargo installed. You don't need nightly. Then, just clone it like usual and `cargo run` to get output:

    git clone https://github.com/svenstaro/genact.git
    cd genact
    cargo run

## Releasing

This is mostly a note for me on how to release this thing:

- Update versions in `README.md`, `static/index.html`, `Cargo.toml`, `snapcraft.yaml`.
- `git commit` and `git tag -s`, `git push`.
- `cargo publish`
- Releases will automatically be deployed by Travis.
- Update AUR package.
