# Linux syscall macros for Rust

This library defines syscall numbers and a `syscall!` macro for directly
invoking Linux system calls.

The `arch` modules document available syscall numbers for all supported
architectures, and the top-level module re-exports syscall numbers for the
current target platform.

Supported architectures:
- `aarch64`
- `arm`
- `loongarch64`
- `riscv64`
- `s390x`
- `x86`
- `x86_64`

To be supported by this library, an architecture must:
- Have a `*-linux-*` target supported by the Rust toolchain at Tier 2 or better
  (https://doc.rust-lang.org/rustc/platform-support.html), and
- Have a stabilised `asm!` macro (https://github.com/rust-lang/rust/issues/93335).
