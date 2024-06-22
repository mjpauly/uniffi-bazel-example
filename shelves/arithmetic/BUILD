load("@rules_rust//rust:defs.bzl", "rust_library")

rust_library(
    name = "arithmetic",
    srcs = ["lib.rs"],
    deps = [
        "@crate_index//:thiserror",
        "@crate_index//:uniffi",
        "//shelves/arithmetic/trig",
        "//shelves/subcrate",
    ],
    visibility = ["//visibility:public"],
)
