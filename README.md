# Python Extensions Example with Rust, UniFFI, and Bazel

This repository shows how to compile and run the `arithmetic-procmacro` example
from `uniffi` with Bazel.

The Rust interface is in `arithmetic/lib.rs`, and the Python test script in
`test_arithmetic.py`.

Run the test like so:

```
bazel test :test_arithmetic --test_output=all
```

## Overview

Types and functions are exported to the FFI boundary from the Rust library
using `uniffi` procedural macros. `uniffi` is patched to avoid looking for a
Cargo workspace (which isn't present when building with Bazel). Instead of
querying Cargo, we pass the namespace and module path (which must be the same)
to `uniffi::setup_scaffolding` as a string literal. Since later proc macros use
this value for the module path, we must call `setup_scaffolding` before any
other proc macros.

```rust
// top of file

uniffi::setup_scaffolding!("my_crate_name");

// other uniffi proc macros below

#[uniffi::export]
fn my_ffi_fn() {
...
```

On the Rust side, we build a shared library that we can load from other
languages:

```starlark
rust_shared_library(
    name = "arithmetic_rs",
    srcs = ["lib.rs"],
    deps = [
        "@crate_index//:thiserror",
        "@crate_index//:uniffi",
        "//arithmetic/subcrate",
    ],
)
```

`subcrate` is another crate, defined as a `rust_library`, which itself exports
objects using `uniffi` proc macros. These objects can be passed to python by the
top-level `arithmetic` library.

We rename the library file to remove the `_rs` extension (which just comes from
our naming of the `rust_shared_library`):

```starlark
copy_file(
    name = "arithmetic_dylib",
    src = ":arithmetic_rs",
    out = "libarithmetic.dylib",
    allow_symlink = True,
)
```

Next we use the `uniffi` CLI tool to generate Python language bindings for the
shared library. The shared library contains the necessary metadata in the
dynamic symbols to generate the bindings. We put the generated bindings for the
top-level into an `__init__.py`, and keep bindings for subcrates with their
original names.

The tool is built in `uniffi_cli/`, and we use a Bazel `genrule` to run it on
a specified set of inputs and outputs.

```starlark
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
```

Finally we create a `py_library` using the generated bindings as its source and
the shared library as runtime data.

```starlark
py_library(
    name = "arithmetic",
    srcs = [":gen_py"],
    data = ["arithmetic_dylib"],
    visibility = ["//visibility:public"],
)
```
