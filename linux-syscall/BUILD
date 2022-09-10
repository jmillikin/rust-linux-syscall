load("@rules_rust//rust:defs.bzl", "rust_doc", "rust_doc_test", "rust_library")

package(default_visibility = ["//visibility:public"])

rust_library(
    name = "linux-syscall",
    srcs = glob([
        "linux-syscall.rs",
        "arch/**/*.rs",
    ]),
    target_compatible_with = [
        "@platforms//os:linux",
    ],
    deps = [
        "@rust_linux_errno//linux-errno",
    ],
)

rust_doc(
    name = "linux-syscall_doc",
    crate = ":linux-syscall",
)

rust_doc_test(
    name = "linux-syscall_doc_test",
    crate = ":linux-syscall",
)
