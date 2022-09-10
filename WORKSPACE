workspace(name = "rust_linux_syscall")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rules_rust",
    sha256 = "0cc7e6b39e492710b819e00d48f2210ae626b717a3ab96e048c43ab57e61d204",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.10.0/rules_rust-v0.10.0.tar.gz"],
)

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

load(
    "@rules_rust//rust:repositories.bzl",
    "rules_rust_dependencies",
    "rust_register_toolchains",
)

rules_rust_dependencies()

rust_register_toolchains(
    edition = "2018",
    version = "1.63.0",
)
