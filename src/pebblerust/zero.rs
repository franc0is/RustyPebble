#[lang="fail_bounds_check"]
pub fn fail_bounds_check(_: *i8, _: uint, _: uint, _: uint) {
  unsafe {
    abort();
  }
}

extern "rust-intrinsic" {
    pub fn transmute<T,U>(val: T) -> U;
}

pub unsafe fn abort() {
  let null_func: fn() = transmute(&0);
  null_func();
}

