use handlebars::handlebars_helper;
use serde_json::json;

use crate::color::{Color, Component, FloatingComponent, Rgba};

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
  let red = red.to_integer_component();
  let green = green.to_integer_component();
  let blue = blue.to_integer_component();
  format!("#{red:02X}{green:02X}{blue:02X}")
});

handlebars_helper!(HexaHelper: |color: Rgba| {
  let Rgba {
    red,
    green,
    blue,
    alpha
  } = color;
  let red = red.to_integer_component();
  let green = green.to_integer_component();
  let blue = blue.to_integer_component();
  let alpha = alpha.to_integer_component();
  format!("#{red:02X}{green:02X}{blue:02X}{alpha:02X}")
});

handlebars_helper!(VividHelper: |color: Rgba| {
  let Rgba {
    red,
    green,
    blue,
    ..
  } = color;
  let red = red.to_integer_component();
  let green = green.to_integer_component();
  let blue = blue.to_integer_component();
  format!("{red:02X}{green:02X}{blue:02X}")
});

handlebars_helper!(HyprHelper: |color: Rgba| {
  let Rgba {
    red,
    green,
    blue,
    ..
  } = color;
  let red = red.to_integer_component();
  let green = green.to_integer_component();
  let blue = blue.to_integer_component();
  format!("rgb({red:02X}{green:02X}{blue:02X})")
});

handlebars_helper!(HypraHelper: |color: Rgba| {
  let Rgba {
    red,
    green,
    blue,
    alpha
  } = color;
  let red = red.to_integer_component();
  let green = green.to_integer_component();
  let blue = blue.to_integer_component();
  let alpha = alpha.to_integer_component();
  format!("rgba({red:02X}{green:02X}{blue:02X}{alpha:02X})")
});

handlebars_helper!(SetAlphaHelper: |color: Rgba, alpha: FloatingComponent| {
  json!(color.with_alpha(alpha))
});

handlebars_helper!(SetLightnessHelper: |color: Rgba, lightness: FloatingComponent| {
  json!(color.with_lightness(lightness))
});

handlebars_helper!(SetSaturationHelper: |color: Rgba, saturation: FloatingComponent| {
  json!(color.with_saturation(saturation))
});

pub fn register(handlebars: &mut handlebars::Handlebars) {
  handlebars.register_helper("hex", Box::new(HexHelper));
  handlebars.register_helper("hexa", Box::new(HexaHelper));
  handlebars.register_helper("rgb", Box::new(RgbHelper));
  handlebars.register_helper("rgba", Box::new(RgbaHelper));
  handlebars.register_helper("vivid", Box::new(VividHelper));
  handlebars.register_helper("hypr", Box::new(HyprHelper));
  handlebars.register_helper("hypra", Box::new(HypraHelper));
  handlebars.register_helper("set-alpha", Box::new(SetAlphaHelper));
  handlebars.register_helper("set-lightness", Box::new(SetLightnessHelper));
  handlebars.register_helper("set-saturation", Box::new(SetSaturationHelper));
}
