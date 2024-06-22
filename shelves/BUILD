load("@aspect_bazel_lib//lib:copy_file.bzl", "copy_file")
load("@rules_python//python:defs.bzl", "py_library")
load("@rules_rust//rust:defs.bzl", "rust_shared_library")

rust_shared_library(
    name = "shelves_rs",
    srcs = ["lib.rs"],
    deps = [
        "@crate_index//:thiserror",
        "@crate_index//:uniffi",
        "//shelves/arithmetic",
    ],
)

# rename libshelves_rs.dylib to libshelves.dylib
copy_file(
    name = "shelves_dylib",
    src = ":shelves_rs",
    out = "libshelves.dylib",
    allow_symlink = True,
)

# generate the python bindings with the uniffi CLI and the proc macro metadata
# uniffi left in the dylib
genrule(
    name = "gen_py",
    srcs = [":shelves_dylib"],
    outs = [
        "__init__.py",
        "arithmetic.py",
        "trig.py",
        "subcrate.py",
    ],
    tools = ["//uniffi_cli"],
    cmd = '''
# generate our bindings in the current directory

$(location //uniffi_cli) generate --library \
    $(location :shelves_dylib) \
    --language python \
    --out-dir=.

# copy the generated files to the output locations

cp shelves.py $(location __init__.py)
cp arithmetic.py $(location arithmetic.py)
cp trig.py $(location trig.py)
cp subcrate.py $(location subcrate.py)
    ''',
)

# top-level python interface to the shelves Rust library
py_library(
    name = "shelves",
    srcs = [":gen_py"],
    data = ["shelves_dylib"],
    visibility = ["//visibility:public"],
)
