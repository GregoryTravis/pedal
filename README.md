# Pedal

Code for a guitar pedal using the Daisy Seed.

## Quickstart

### Clean & Build everything
    ./all

### Build & push board binary
    ./push-board

### Run sim
    cd host && cargo run --bin sim --features for_host

## Project Structure

### Overview

This repo contains a generic rust interface to libDaisy, a Rust/C++ runtime,
and code for various pedals. In the following, 'board' means 'runs on the
Daisy', and 'host' means 'runs locally'. 'shared' means 'can be used by both
board and host code. 'shim' means C++ code needed to connect everything else to
libDaisy.

Inside Rust source, code can be enabled/disabled for different contexts:

    #[cfg(feature = "for_host")]
    #[cfg(not(feature = "for_host"))]

The coded is divided up this way because (1) I am a rust beginner and this is
how I could make it work, and (2) it builds for two architectures.

Daisy Seed code is built with target `thumbv7em-none-eabihf`.
Host code is built with the default target for your machine.
In addition to having two different targets, board code runs `no_std`.

Thus:
* `board` code can only build for `thumbv7em-none-eabihf`
* `host` code can only build for the default target, as it requires `std`
* `shared` code can build on either
* `shim` code can only build for the Daisy, using the libDaisy makefile

It contains 6 packages:
* Generic library
  * `lib/board`: Rust runtime
  * `lib/shared`: Generic dsp code, suitable for running on either the Daisy or the host
  * `lib/host`: Utilities for local development testing, including a Patch runner
  * `lib/shim`: The C++ glue between Rust and libDaisy
* Pedal implementations
  * `board` (package name: 'pedalboard'): `main` entrypoints for pedal patches
  * `host` (package name: 'pedalhost'): local development and testing of pedal patches

### Building a Board Binary

This code is based on libDaisy. Linking the final binary that is pushed to the
Daisy is done with the libDaisy Makefile, not with Rust/Cargo. Thus, the Rust
"binary", containing the entrypoint and all other code it requires is actually
the library crate built in `board`. This is then linked into a executable by
the shim makefile.

`board/src/lib.rs` is, in a sense, the root of the board binary. The `main`
defined in the C++ shim in `lib/shim/pedal.cpp` simply calls `PEDAL_MAIN`,
which is set by the Makefile, which gets it from the make invocation command
line in `push-board`:

    make PEDAL_MAIN=reso_main

`reso_main()` is defined in `board/src/lib.rs`. You can deploy different
patches simply by changing the definition of `PEDAL_MAIN` to a different main
routine in `board/src/lib.rs`.

### Features

**for_host**: enabled when building for the host.

## Developing Using The Library

_TODO_
