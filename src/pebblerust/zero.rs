#![feature(intrinsics)]

extern crate core;

use pebblerust::c;

extern "rust-intrinsic" {
    pub fn transmute<T,U>(val: T) -> U;
}

#[lang="stack_exhausted"] extern fn stack_exhausted() {}

#[lang="eh_personality"] extern fn eh_personality() {}

#[lang="panic_fmt"]
pub fn panic_fmt(_fmt: &core::fmt::Arguments, _file_line: &(&'static str, usize)) -> ! {
  // TODO: logging

  unsafe {
    c::window_stack_pop_all();
  }

  loop {}
}

// If we don't implement this here the linker will link against libunwind
// which will then require kill, getpid()... which isn't available.
#[no_mangle]
pub unsafe fn __aeabi_unwind_cpp_pr0() -> () {
  unsafe {
    c::window_stack_pop_all();
  }

  loop {}
}
