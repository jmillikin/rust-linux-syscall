workspace(name = "rust_linux_syscall")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rules_rust",
    sha256 = "0cc7e6b39e492710b819e00d48f2210ae626b717a3ab96e048c43ab57e61d204",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.10.0/rules_rust-v0.10.0.tar.gz"],
)

http_archive(
    name = "rust_posix_errno",
    sha256 = "e3f1c80383975b3f2decce96a799181392ea90fbb4540aa7b6bfd97608c5e570",
    strip_prefix = "posix-errno-1.0.2",
    urls = ["https://github.com/jmillikin/rust-posix-errno/releases/download/v1.0.1/posix-errno-1.0.2.tar.xz"],
)

http_archive(
    name = "rust_linux_errno",
    sha256 = "3c235f30c7081b4d1647d8d03e4dd83a88833e32c6db444c699b9ed07ecb14ee",
    strip_prefix = "linux-errno-1.1.0",
    urls = ["https://github.com/jmillikin/rust-linux-errno/releases/download/v1.1.0/linux-errno-1.1.0.tar.xz"],
)

load(
    "@rules_rust//rust:repositories.bzl",
    "rules_rust_dependencies",
    "rust_register_toolchains",
)

rules_rust_dependencies()

rust_register_toolchains(
    version = "1.63.0",
)
