load("@rules_rust//rust:defs.bzl", "rust_library")

# a crate that exports things with uniffi, but is a dependency of the top-level
# library
rust_library(
    name = "subcrate",
    srcs = ["lib.rs"],
    deps = [
        "@crate_index//:uniffi",
    ],
    visibility = ["//arithmetic:__subpackages__"],
)
