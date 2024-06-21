load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "uniffi_cli",
    srcs = ["main.rs"],
    deps = [
        "@crate_index//:uniffi",
    ],
    visibility = ["//visibility:public"],
)
