use handlebars::handlebars_helper;
use serde_json::json;

use super::Rgba;

handlebars_helper!(RgbHelper: |color: Rgba| {
  let Rgba {
    red,
    green,
    blue,
    ..
  } = color;
  format!("rgb({red}, {green}, {blue})")
});

handlebars_helper!(RgbaHelper: |color: Rgba| {
  let Rgba {
    red,
    green,
    blue,
    alpha
  } = color;
  format!("rgba({red}, {green}, {blue}, {alpha})")
});

handlebars_helper!(HexHelper: |color: Rgba| {
  let Rgba {
    red,
    green,
    blue,
    ..
  } = color;
  format!("#{red:02X}{green:02X}{blue:02X}")
});

handlebars_helper!(HexaHelper: |color: Rgba| {
  let Rgba {
    red,
    green,
    blue,
    alpha
  } = color;
  let alpha = (alpha * 255.0f32).round() as u8;
  format!("#{red:02X}{green:02X}{blue:02X}{alpha:02X}")
});

handlebars_helper!(SetAlphaHelper: |color: Rgba, alpha: f32| {
  json!(Rgba {
    red: color.red,
    green: color.green,
    blue: color.blue,
    alpha: alpha,
  })
});

pub fn register(handlebars: &mut handlebars::Handlebars) {
  handlebars.register_helper("hex", Box::new(HexHelper));
  handlebars.register_helper("hexa", Box::new(HexaHelper));
  handlebars.register_helper("rgb", Box::new(RgbHelper));
  handlebars.register_helper("rgba", Box::new(RgbaHelper));
  handlebars.register_helper("set-alpha", Box::new(SetAlphaHelper));
}
