RustyPebble
===========

This is a proof of concept Pebble application written in Rust (http://rust-lang.org).

It does not yet make use of Rust's core library (need to cross compile it first). Instead, it relies on the FFI and implements a few required language items.

rusty.rs contains the app itself. pebblerust contains a rust version of the pebble library.

Building & Installing
---------------------
In order to build it, you will need installed on your system:
* An llvm toolchain compiled for all targets
* A recent rust compiler (0.12 works fine as of this writing).

Otherwise, you can build & install it just like you would any pebble app
  $ pebble build
  $ pebble install

