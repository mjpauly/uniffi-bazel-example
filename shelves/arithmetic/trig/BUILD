load("@rules_rust//rust:defs.bzl", "rust_library")

rust_library(
    name = "trig",
    srcs = ["lib.rs"],
    deps = [
        "@crate_index//:thiserror",
        "@crate_index//:uniffi",
    ],
    visibility = ["//visibility:public"],
)
