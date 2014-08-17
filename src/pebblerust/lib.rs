use pebblerust::zero::*;
use pebblerust::types::*;
use pebblerust::c;

pub enum AppLogLevel {
  AppLogLevelError = 1,
  AppLogLevelWarning = 50,
  AppLogLevelInfo = 100,
  AppLogLevelDebug = 200,
  AppLogLevelDebugVerbose = 255,
}

pub fn app_event_loop() {
  unsafe {
    c::app_event_loop();
  }
}

pub fn window_create() -> *mut Window {
  unsafe {
    c::window_create()
  }
}

pub fn window_set_window_handlers(window: *mut Window, handlers: WindowHandlers) {
  unsafe {
    c::window_set_window_handlers(window, handlers);
  }
}

pub fn window_stack_push(window: *mut Window, animated: bool) {
  unsafe {
    c::window_stack_push(window, animated);
  }
}

pub fn app_log(level: AppLogLevel, msg: &str) {
  unsafe {
    let (filename, _): (*const u8, uint) = transmute("rusty");
    let (message, _): (*const u8, uint) = transmute(msg);
    c::app_log(level as u8, filename, 0, message);
  }
}

pub fn window_single_click_subscribe<T>(button: u8, subscriber: extern fn(ClickRecognizerRef, *mut T)) {
  unsafe {
    let subscriber_ptr: extern fn(ClickRecognizerRef, *mut u8) = transmute(subscriber);
    c::window_single_click_subscribe(button, subscriber_ptr);
  }
}

pub fn window_set_click_config_provider<T>(window: *mut Window, provider: extern fn(*mut T)) {
  unsafe {
    let provider_ptr: extern fn(*mut u8) = transmute(provider);
    c::window_set_click_config_provider(window, provider_ptr);
  }
}

pub fn window_get_root_layer(window: *mut Window) -> *mut Layer {
  unsafe {
    c::window_get_root_layer(window)
  }
}

pub fn layer_get_bounds(layer: *mut Layer) -> GRect {
  unsafe {
    c::layer_get_bounds(layer)
  }
}

pub fn text_layer_create(frame: GRect) -> *mut TextLayer {
  unsafe {
    c::text_layer_create(frame)
  }
}

pub fn text_layer_set_text(layer: *mut TextLayer, text: &str) {
  unsafe {
    c::text_layer_set_text(layer, text);
  }
}

pub fn layer_add_child(parent: *mut Layer, child: *mut Layer) {
  unsafe {
    c::layer_add_child(parent, child);
  }
}

pub fn text_layer_get_layer(text_layer: *mut TextLayer) -> *mut Layer {
  unsafe {
    c::text_layer_get_layer(text_layer)
  }
}

pub fn window_set_click_config_provider_with_context<T>(window: *mut Window,
    provider: extern fn(*mut T), context: *mut T) {
  unsafe {
    let context_ptr: *mut u8 = transmute(context);
    let fn_ptr: extern fn(*mut u8) = transmute(provider);
    c::window_set_click_config_provider_with_context(window, fn_ptr, context_ptr);
  }
}
