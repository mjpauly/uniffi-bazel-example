load("@aspect_bazel_lib//lib:copy_file.bzl", "copy_file")
load("@rules_python//python:defs.bzl", "py_library")
load("@rules_rust//rust:defs.bzl", "rust_shared_library")

rust_shared_library(
    name = "arithmetic_rs",
    srcs = ["lib.rs"],
    deps = [
        "@crate_index//:thiserror",
        "@crate_index//:uniffi",
        "//arithmetic/subcrate",
    ],
)

# rename libarithmetic_rs.dylib to libarithmetic.dylib
copy_file(
    name = "arithmetic_dylib",
    src = ":arithmetic_rs",
    out = "libarithmetic.dylib",
    allow_symlink = True,
)

# generate the python bindings with the uniffi CLI and the proc macro metadata
# uniffi left in the dylib
genrule(
    name = "gen_py",
    srcs = [":arithmetic_dylib"],
    outs = [
        "__init__.py",
        "subcrate.py",
    ],
    tools = ["//uniffi_cli"],
    cmd = '''
# generate our bindings in the current directory

$(location //uniffi_cli) generate --library \
    $(location :arithmetic_dylib) \
    --language python \
    --out-dir=.

# copy the generated files to the output locations

cp arithmetic.py $(location __init__.py)
cp subcrate.py $(location subcrate.py)
    ''',
)

# top-level python interface to the arithmetic Rust library
py_library(
    name = "arithmetic",
    srcs = [":gen_py"],
    data = ["arithmetic_dylib"],
    visibility = ["//visibility:public"],
)
