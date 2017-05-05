# retdec-rust

A Rust library and tools providing easy access to the
[retdec.com](https://retdec.com) decompilation service through their public
[REST API](https://retdec.com/api/).

You can either incorporate the library in your own tools:

```rust
extern crate retdec;

use retdec::{Decompiler, DecompilationArguments, File, Settings};

let decompiler = Decompiler::new(
    Settings::new()
        .with_api_key("YOUR-API-KEY")
);
let mut decompilation = decompiler.start_decompilation(
    DecompilationArguments::new()
        .with_input_file(File::from_path("hello.exe")?)
)?;
decompilation.wait_until_finished()?;
let output_code = decompilation.get_output_hll_code()?;
print!("{}", output_code);
```

or you can use the provided tool for stand-alone decompilations:

```text
$ decompiler -k YOUR-API-KEY hello.exe
```

Either way, you get the decompiled C code:

```text
//
// This file was generated by the Retargetable Decompiler
// Website: https://retdec.com
// Copyright (c) 2017 Retargetable Decompiler <info@retdec.com>
//

int main(int argc, char ** argv) {
    printf("Hello, world!\n");
    return 0;
}
```

## Status

The library is **at the beginning of its development** and its state is
**pre-alpha** (**highly experimental**).

A summary of the supported parts of the [retdec.com's
API](https://retdec.com/api/docs/index.html) is available
[here](https://github.com/s3rvac/retdec-rust/tree/master/STATUS.md).

## Installation

Currently, the only way of using the library and tools is to specify the
dependency from this git repository. To do that, add the following lines into
your `Cargo.toml` file:

```text
[dependencies]
retdec = { git = "https://github.com/s3rvac/retdec-rust" }
```

As soon as the first version is released, you will be able to add a dependency
directly from [crates.io](https://crates.io/).

## Documentation

An automatically generated API documentation is available here:

* [master](https://projects.petrzemek.net/retdec-rust/doc/master/retdec/index.html)

## License

Licensed under either of

* Apache License, Version 2.0,
  ([LICENSE-APACHE](https://github.com/s3rvac/retdec-rust/tree/master/LICENSE-APACHE)
  or http://www.apache.org/licenses/LICENSE-2.0)
* MIT License
  ([LICENSE-MIT](https://github.com/s3rvac/retdec-rust/tree/master/LICENSE-APACHE)
  or http://opensource.org/licenses/MIT)

at your option.

## Access from Other Languages

If you want to access the [retdec.com](https://retdec.com) decompilation
service from other languages, check out the following projects:

* [retdec-cpp](https://github.com/s3rvac/retdec-cpp) - A library and tools for
  accessing the service from C++.
* [retdec-python](https://github.com/s3rvac/retdec-python) - A library and
  tools for accessing the service from Python.
* [retdec-sh](https://github.com/s3rvac/retdec-sh) - Scripts for accessing the
  service from shell.
