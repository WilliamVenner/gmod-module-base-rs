# gmod-module-base-rs

A base for developing Garry's Mod binary modules in Rust.

# Getting Started

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. [Download](https://github.com/WilliamVenner/gmod-module-base-rs/archive/refs/heads/master.zip) or `git clone` this repository
3. Open the cloned repository in a command prompt or terminal
4. Type `cargo build`

Your built module can be found in `target/debug`!

### Linux Note

When building a Linux module, you'll find it has the extension `.so`, you can simply change it to `.dll` and it will still work fine in Garry's Mod.

# What next?

* Read the [`gmod` crate documentation](https://docs.rs/gmod/latest/gmod/)
* Read the [Rust book](https://doc.rust-lang.org/book/)
* Build in [release mode](https://doc.rust-lang.org/book/ch14-01-release-profiles.html)!