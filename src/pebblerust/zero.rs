// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// Copied from
// https://github.com/rust-lang/rust/blob/master/src/test/run-pass/smallest-hello-world.rs

#![feature(intrinsics)]

extern { fn puts(s: *const u8); }
extern "rust-intrinsic" { pub fn transmute<T, U>(t: T) -> U; }

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }

// Additions

// If we don't implement this here the linker will link against libunwind
// which will then require kill, getpid()... which isn't available.
#[no_mangle]
pub unsafe fn __aeabi_unwind_cpp_pr0() -> () { loop {} }
