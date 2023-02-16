# DESK - Dopey Embedded Systems Kernel

A fast multiplatform kernel, written in Rust.

## Building

Here are the requirements for building:
- Go (1.17+)
- Ruby (2.x+)
- Gems (elftools, colorize) => `gem install colorize elftools`
- QEMU (for running)
- Rust (nightly)
- LLVM tools (preview) => `rustup component add llvm-tools-preview`
- Rust aarch64-unknown-none-softfloat target => `rustup target add aarch64-unknown-none-softfloat`
