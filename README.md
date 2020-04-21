# `tdlib-sys`

[![build](https://github.com/nuxeh/tdlib-sys/workflows/build/badge.svg)](https://github.com/nuxeh/tdlib-sys/actions?query=branch%3Amaster+event%3Apush+workflow%3Abuild)
[![macOS](https://github.com/nuxeh/tdlib-sys/workflows/macOS/badge.svg)](https://github.com/nuxeh/tdlib-sys/actions?query=branch%3Amaster+event%3Apush+workflow%3AmacOS)
[![windows](https://github.com/nuxeh/tdlib-sys/workflows/windows/badge.svg)](https://github.com/nuxeh/tdlib-sys/actions?query=branch%3Amaster+event%3Apush+workflow%3Awindows)
[![bindgen](https://github.com/nuxeh/tdlib-sys/workflows/bindgen/badge.svg)](https://github.com/nuxeh/tdlib-sys/actions?query=branch%3Amaster+event%3Apush+workflow%3Abindgen)
[![crates.io](https://img.shields.io/crates/v/tdlib-sys)](https://crates.io/crates/tdlib-sys)

Rust sys crate for Telegram's [`TDLib`](https://core.telegram.org/tdlib) client
library.

Requires the `gperf` tool to be installed to build, and the following
development libraries:

 - `libssl`
 - `zlib`

Standard build tools are required, and additionally:

 - `cmake`

All build instructions below for OSX and Windows are currently only tested on
Github workflows, if you find something more accurate for a normal system,
please feel free to send a PR!

## Dynamically linked builds

### Get source

    git clone https://github.com/nuxeh/tdlib-sys.git
    cd tdlib-sys

### Linux

    sudo apt install gperf perl build-essential cmake libssl-dev libz3-dev
    cargo build

It should be possible to dynamically link on other platforms, but this is
currently untested.

## Statically linked builds

The crate feature `bundled_deps` can be used on any platform to use `zlib` and
`openssl` built statically by their respective `sys` crates.

This does not require the installation of either of the dependencies on the
host system, so is an easy solution for building on Windows and OSX.

This still requires standard build tools, and additionally the following to be
installed:

 - `perl`

### Get source

    git clone https://github.com/nuxeh/tdlib-sys.git
    cd tdlib-sys

### Linux

    sudo apt install gperf perl build-essential cmake
    cargo build --features bundled_deps

### OSX

    brew install gperf perl
    cargo build --features bundled_deps

### Windows

    vcpkg.exe install gperf:x86-windows
    cargo build --features bundled_deps
