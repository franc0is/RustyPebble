pub type WindowHandler = extern fn(*mut Window);

pub struct Window;
pub struct Layer;
pub struct TextLayer;
pub struct ClickRecognizer;

pub struct GPoint {
  pub x: u16,
  pub y: u16,
}

pub struct GSize {
  pub w: u16,
  pub h: u16,
}

pub struct GRect {
  pub origin: GPoint,
  pub size: GSize,
}

pub struct WindowHandlers {
  pub load: WindowHandler,
  pub appear: WindowHandler,
  pub disappear: WindowHandler,
  pub unload: WindowHandler
}

pub type ClickRecognizerRef = *mut ClickRecognizer;


