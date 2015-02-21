RustyPebble
===========

This is a proof of concept Pebble application written in Rust (http://rust-lang.org).

~~It does not yet make use of Rust's core library (need to cross compile it first). Instead, it relies on the FFI and implements a few required language items.~~

rusty.rs contains the app itself. pebblerust contains a rust version of the pebble library.

Prerequisites
-------------

Make sure you a recent version of Rust (1.0-beta).

    brew tap caskroom/cask
    brew install brew-cask
    brew tap caskroom/versions
    brew cask install rust-nightly

Rust has forked LLVM and is loosely based on LLVM 3.6 which isn't released yet.
In LLVM 3.5 you might get weird `error: expected type` messages.

You can try to build Rust's custom LLVM yourself.

    git clone --depth 1 -b rust-llvm-2015-02-19 https://github.com/rust-lang/llvm
    (cd llvm && ./configure --enable-optimized && make)

Building & Installing
---------------------
In order to build it, you will need installed on your system:
* An llvm toolchain compiled for all targets
* A recent rust compiler (0.12 works fine as of this writing).

Otherwise, you can build & install it just like you would any pebble app

    pebble build
    pebble install

