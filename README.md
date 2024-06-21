# Python Extensions Example with Rust, UniFFI, and Bazel

This repository shows how to compile and run the `arithmetic-procmacro` example
from `uniffi` with Bazel.

Run the test like so:

```
bazel test :test_arithmetic --test_output=all
```

# Overview

Types and functions are exported to the FFI boundary from the Rust library
using `uniffi` procedural macros. `uniffi` is patched to avoid looking for a
Cargo workspace (which isn't present when building with Bazel). Instead of
querying Cargo, we pass the namespace and module path (which must be the same)
to `uniffi::setup_scaffolding` as a string literal. Since later proc macros use
this value, we must call `setup_scaffolding` before any other proc macros.

```rust
// top of file

uniffi::setup_scaffolding!("arithmetic");

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
    ],
)
```

Then we rename the library file to remove the `_rs` extension:

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
dynamic symbols to generate the bindings. We put the generated bindings into an
`__init__.py` for our python module.

```starlark
genrule(
    name = "gen_py",
    srcs = [":arithmetic_dylib"],
    outs = ["__init__.py"],
    cmd = '''
$(location //uniffi_cli) generate --library \
    $(location :arithmetic_dylib) \
    --language python \
    --out-dir=.
cp arithmetic.py $(location __init__.py)
    ''',
    tools = ["//uniffi_cli"],
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
