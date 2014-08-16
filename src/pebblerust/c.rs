use pebblerust::types::*;

extern {
  pub fn app_event_loop();
  pub fn window_create() -> *Window;
  pub fn window_stack_push(window: *Window, animated: bool);
  pub fn app_log(level: u8, filename: *u8, line_num: u32, msg: *u8);
  pub fn window_set_click_config_provider(window: *Window, provider: extern fn(*u8));
  pub fn window_set_click_config_provider_with_context(window: *Window, provider: extern fn(*u8), context: *u8);
  pub fn window_single_click_subscribe(button: u8, subscriber: extern fn(ClickRecognizerRef, *u8));
  pub fn window_set_window_handlers(window: *Window, handlers: WindowHandlers);
  pub fn window_get_root_layer(window: *Window) -> *Layer;
  pub fn layer_get_bounds(layer: *Layer) -> GRect;
  pub fn text_layer_create(frame: GRect) -> *TextLayer;
  pub fn text_layer_set_text(layer: *TextLayer, text: &str);
  pub fn layer_add_child(parent: *Layer, child: *Layer);
  pub fn text_layer_get_layer(text_layer: *TextLayer) -> *Layer;
}


