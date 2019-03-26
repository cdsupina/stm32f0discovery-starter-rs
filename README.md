# STM32F0DISCOVERY Starter Rust

This repository is meant to serve as a starting point for creating and running programs on the STM32F0DISCOVERY board. I found it difficult to find a straightforward beginners guide for embedded rust. This is my attempt to put all of the information I gathered into one place to hopefully make getting started easier.

Take note that I'm still a beginner to rust as a language, so I encourage you for the time being to pursue other sources if you are looking to learn the rust language.

# Installation

Install rust: https://www.rust-lang.org/tools/install

Add the thumbv6m target for Cortex M0: `rustup target add thumbv6m-none-eabi`

Install cargo-generate and cargo-binutils: `cargo install cargo-generate cargo-binutils`

Add llvm-tools-preview component: `rustup component add llvm-tools-preview`

Install **OpenOCD** and **gdb-multiarch** using your OS's package manager (pacman, dnf, brew, apt, etc).

# To Build

To build main.rs located in src, use the command: `cargo build`

To build an example located in examples, use the command `cargo build --example [example_name]`

For example, to build the double_blinky.rs example, use the command `cargo build --example double_blinky`

# To Flash

Plug in your STM32F0DISCOVERY. Use the flash_device.sh shell script to flash your program, giving the path to the compiled program as an argument. The compiled src program is located in target/thumbv6m-none-eabi/debug and the compiled example programs are located in target/thumbv6m-none-eabi/debug/examples. See examples of flashing below.

Flash src:

./flash_device.sh target/thumbv6m-none-eabi/debug/stm32f0discovery-starter-rs

Flash example adc_serial:

./flash_device.sh target/thumbv6m-none-eabi/debug/examples/adc_serial
