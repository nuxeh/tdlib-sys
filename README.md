# `tdlib-sys`

[![build](https://github.com/nuxeh/tdlib-sys/workflows/build/badge.svg)](https://github.com/nuxeh/tdlib-sys/actions?query=branch%3Amaster+event%3Apush+workflow%3Abuild)
[![macOS](https://github.com/nuxeh/tdlib-sys/workflows/macOS/badge.svg)](https://github.com/nuxeh/tdlib-sys/actions?query=branch%3Amaster+event%3Apush+workflow%3AmacOS)
[![windows](https://github.com/nuxeh/tdlib-sys/workflows/windows/badge.svg)](https://github.com/nuxeh/tdlib-sys/actions?query=branch%3Amaster+event%3Apush+workflow%3Awindows)
[![crates.io](https://img.shields.io/crates/v/tdlib-sys)](https://crates.io/crates/tdlib-sys)

Rust sys crate for Telegram's [`TDLib`](https://core.telegram.org/tdlib) client
library.

Requires the `gperf` tool to be installed to build, and the following
development libraries:

 - `libssl`
 - `libz`

Build tools required:

 - `Cmake`

## Statically linked builds

The feature `bundled_deps` can be used on any platform to use `zlib` and
`openssl` bundled and build statically by the respective `sys` crate.

This does not require the installation of either of the dependencies on the
host system, so is an easy solution for building on Windows and OSX.

This still requires standard build tools, and additionally the following to be
installed:

 - `perl`
