#![crate_type="lib"]
#![no_std]
#![feature(globs)]
#![feature(lang_items)]
#![feature(intrinsics)]


use pebblerust::lib::*;
use pebblerust::types::*;


mod pebblerust {
  pub mod lib;
  pub mod c;
  pub mod types;
  pub mod zero;
}

extern fn select_click_handler(_: ClickRecognizerRef, text_layer: *mut TextLayer) {
  text_layer_set_text(text_layer, "Select!\0");
}

extern fn up_click_handler(_: ClickRecognizerRef, text_layer: *mut TextLayer) {
  text_layer_set_text(text_layer, "Up!\0");
}

extern fn down_click_handler(_: ClickRecognizerRef, text_layer: *mut TextLayer) {
  text_layer_set_text(text_layer, "Down!\0");
}

extern fn click_config_provider(_: *mut TextLayer) {
  window_single_click_subscribe(1, up_click_handler);
  window_single_click_subscribe(2, select_click_handler);
  window_single_click_subscribe(3, down_click_handler);
}

extern fn window_load_handler(window: *mut Window) {
  app_log(AppLogLevelDebug, "window loaded!\0");
  let window_layer = window_get_root_layer(window);
  let window_bounds = layer_get_bounds(window_layer);

  let text_bounds = GRect {
    origin: GPoint { x: 0, y: 72 },
    size: GSize { w: window_bounds.size.w, h: 20 }
  };
  let text_layer = text_layer_create(text_bounds);

  text_layer_set_text(text_layer, "Press a button\0");
  layer_add_child(window_layer, text_layer_get_layer(text_layer));

  window_set_click_config_provider_with_context(window, click_config_provider, text_layer);
}

extern fn window_unload_handler(window: *mut Window) {
}
extern fn window_appear_handler(window: *mut Window) {
}
extern fn window_disappear_handler(window: *mut Window) {
}

#[no_mangle]
pub extern fn main() -> int {
  app_log(AppLogLevelDebug, "Pebble-y Rust, Rust-y Pebble\0");
  let window = window_create();
  let window_handlers = WindowHandlers {
    load: window_load_handler,
    unload: window_unload_handler,
    appear: window_appear_handler,
    disappear: window_disappear_handler
  };
  window_set_window_handlers(window, window_handlers);
  window_stack_push(window, true);
  app_event_loop();
  0
}