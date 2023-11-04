# [dyn_import](https://github.com/protibimbok/dyn_import)

`dyn_import` is a Rust macro crate that simplifies the process of dynamically importing modules from a specified directory. This can be especially useful for projects that need to load and use modules at runtime, making your Rust application more flexible and extensible.

## Features

- **Dynamic Module Import:** Easily import modules from a directory at runtime using a simple macro.
- **Flexible:** Allows you to specify the directory and filtering criteria for the modules you want to import.

## Installation

To use `dyn_import` in your Rust project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
dyn_import = "0.1"
```
Or run the following command:
```bash
cargo add dyn_import
```


## Usage

Suppose you have the following directory structure for your Rust project:

```
my_project/
│
├── Cargo.toml
│
└── src/
    ├── main.rs
    |-- some_mod.rs
    └── some_mod/
        ├── module_one.rs
        └── module_two.rs
```


Here's how you can use the import_mods macro from dyn_import in your Rust project:

```rust
extern crate dyn_import;
use dyn_import::import_mods;

// Declare the parent module
mod some_mod;

// All child modules are added here
import_mods!("src/some_mod");

fn main() {
    // Dynamically import modules from the specified directory.
    // The imported modules will be available as if they were part of your project.
    // You can use them just like regular Rust modules.

    // Example:
    module_one::function_from_module_one();
    module_two::function_from_module_two();
}
```