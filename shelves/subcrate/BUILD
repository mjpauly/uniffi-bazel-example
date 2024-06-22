load("@rules_rust//rust:defs.bzl", "rust_library")

rust_library(
    name = "subcrate",
    srcs = ["lib.rs"],
    deps = [
        "@crate_index//:uniffi",
    ],
    visibility = ["//visibility:public"],
)
