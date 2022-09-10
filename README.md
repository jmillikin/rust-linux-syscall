# Linux syscall macros for Rust

This library defines syscall numbers and a `syscall!` macro for directly
invoking Linux system calls.

The `arch` modules document available syscall numbers for all supported
architectures, and the top-level module re-exports syscall numbers for the
current target platform.

Supported architectures:
- `aarch64`
- `arm`
- `riscv64`
- `x86`
- `x86_64`

To be supported by this library, an architecture must:
- Have a `*-linux-*` target supported by the Rust toolchain at Tier 2 or better
  (https://doc.rust-lang.org/rustc/platform-support.html), and
- Have a stabilised `asm!` macro (https://github.com/rust-lang/rust/issues/93335).

To depend on `linux-syscall` from a Bazel workspace:

```python
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rust_posix_errno",
    sha256 = "0c86c849ff673372fe6415d4004a233565b57b2884ea49d3b725dd1296cc2529",
    strip_prefix = "posix-errno-1.0.1",
    urls = ["https://github.com/jmillikin/rust-posix-errno/releases/download/v1.0.1/posix-errno-1.0.1.tar.xz"],
)

http_archive(
    name = "rust_linux_errno",
    sha256 = "009d58c93c806f178004a4cd30af211860bc44f8ce7d02eb4f544821add7ca99",
    strip_prefix = "linux-errno-1.0.1",
    urls = ["https://github.com/jmillikin/rust-linux-errno/releases/download/v1.0.1/linux-errno-1.0.1.tar.xz"],
)

http_archive(
    name = "rust_linux_syscall",
    # Obtain the package checksum from the release page:
    # https://github.com/jmillikin/rust-linux-syscall/releases/tag/v1.0.0
    sha256 = "",
    strip_prefix = "linux-syscall-1.0.0",
    urls = ["https://github.com/jmillikin/rust-linux-syscall/releases/download/v1.0.0/linux-syscall-1.0.0.tar.xz"],
)
```

To depend on `linux-syscall` from a Cargo workspace:

```
[dependencies]
linux-syscall = { version = "1.0.0" }
```
