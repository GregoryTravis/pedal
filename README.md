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

A cross-platform audio effects environment for the Daisy Seed processor, for
use on 125b form-factor pedals.

The codebase is Rust, with a small shim layer for interfacing with `libDaisy`.
Using `libDaisy` for basic hardware interaction ensures this will work across
multiple revisions of the Daisy Seed.

### Cross-Platform

All effects and tools run mostly unchanged embedded or on a POSIX-compliant
host machine.

The platform-specific code is as small as possible to make the tests worth
running on the host; where possible, embedded libraries (such as memory
allocators) are the same between embedded and host. This means that a few of
the hardware features are simulated for the host. While not quite comprising a
complete HAL, the following aspects of the embedded environment are stubbed or
simulated locally on the host machine:

* Realtime callback invocation
* Hardware UI (foot switches, knobs)
* Timing (for benchmarking)
* Safe installation and removal of real-time callback
* Panic recovery
* Interrupt-safe logging
* Special memory regions (e.g. the SDRAM section)
* Canned-input effect unit tests

### Unit Tests

Canned-input unit tests are implemented for most effects. This passes a fixed
input block through the effect and compares the result to a golden output.

For testing within the realtime architecture (within the interrupt), a special
override test intervenes before each interrupt invocation. It replaces the
hardware live audio stream with the canned data and checks the effect output,
thus overriding the real audio stream with the canned data. This allows for
sample-exact testing in real-world production conditions.

## Features

### EDSL

The library also contains a prototype EDSL for eliminating runtime overhead.

Without the EDSL, patches can be composed (see: Mixer, Seq), but this incurs
some dynamic dispatch overhead, and composition options are limited.

With the EDSL, a general DAG structure is easily supported, allowing more
general routing of audio data. Processing nodes can serve as source or sink to
any number of other nodes.

The EDSL is implemented as a Rust code generator, producing a struct containing
intermediate node output buffers, and translating the node DAG in-order into a
single function.

I had hoped this approach would allow the Rust inliner to inline harder, but
it turns out the Rust inliner already inlines very hard, so the speed
improvements will likely be slight. But it also allows substantial reuse of
intermediate buffers, which is useful for allocated limited fast RAM.

### Effects

* Chorus
* Delay
* Fuzz
* Vibrato
* Gain
* High / Low pass filter
* Resonant filter
* Waveshaper
* Guitar synth (uses FFT; based on a charming misinterpretation of the original
  GR-300 hardware, so it's strange but kind of interesting)
* Eventide Harmonizer (based on the [notes in the Eventide
  blog](https://www.eventideaudio.com/blog/50th-flashback-4-3-h910-harmonizer-minds-blown/))

### Utility Nodes

* FFT
* Envelope follower

### FFTs

The codebase supports both ARM and MicroFFT implementations of FFT. The two
libraries do not provide the same options for blocksize, so generally one must
pick a single implementation.

## Code Structure

This repo contains a generic rust interface to libDaisy, a Rust/C++ runtime,
and code for various effects. In the following, 'board' means 'runs on the
Daisy', and 'host' means 'runs locally', i.e. on the host machine. 'shared'
means 'can be used by both board and host code. 'shim' means C++ code needed to
connect everything else to libDaisy.

Inside Rust source, code can be enabled/disabled for different contexts:

    #[cfg(feature = "for_host")]
    #[cfg(not(feature = "for_host"))]

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
