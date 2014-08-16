use pebblerust::types::*; 
extern {
  pub fn app_event_loop();
  pub fn window_create() -> *mut Window;
  pub fn window_stack_push(window: *mut Window, animated: bool);
  pub fn app_log(level: u8, filename: &'static [u8], line_num: u32, msg: &'static [u8]);
  pub fn window_set_click_config_provider(window: *mut Window, provider: extern fn(*mut u8));
  pub fn window_set_click_config_provider_with_context(window: *mut Window, provider: extern fn(*mut u8), context: *mut u8);
  pub fn window_single_click_subscribe(button: u8, subscriber: extern fn(ClickRecognizerRef, *mut u8));
  pub fn window_set_window_handlers(window: *mut Window, handlers: WindowHandlers);
  pub fn window_get_root_layer(window: *mut Window) -> *mut Layer;
  pub fn layer_get_bounds(layer: *mut Layer) -> GRect;
  pub fn text_layer_create(frame: GRect) -> *mut TextLayer;
  pub fn text_layer_set_text(layer: *mut TextLayer, text: &str);
  pub fn layer_add_child(parent: *mut Layer, child: *mut Layer);
  pub fn text_layer_get_layer(text_layer: *mut TextLayer) -> *mut Layer;
}


