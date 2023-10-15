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
    alpha,
  })
});

handlebars_helper!(SetLightnessHelper: |color: Rgba, lightness: f32| {
  let Rgba {
    red,
    green,
    blue,
    alpha
  } = color;
  let mut hsla = palette::IntoColor::<palette::Okhsla>::into_color(
    palette::Alpha::<palette::LinSrgb<u8>, f32>::new(red, green, blue, alpha)
      .into_format::<f32, f32>(),
  );
  hsla.lightness = lightness;
  let processed = palette::IntoColor::<palette::LinSrgba>::into_color(hsla).into_format::<u8, f32>();
  let palette::Alpha::<palette::LinSrgb::<u8>, f32> {
    color: palette::LinSrgb::<u8> {
      red,
      green,
      blue,
      ..
    },
    alpha
  } = processed;
  json!(Rgba {
    red,
    green,
    blue,
    alpha
  })
});

handlebars_helper!(SetSaturationHelper: |color: Rgba, saturation: f32| {
  let Rgba {
    red,
    green,
    blue,
    alpha
  } = color;
  let mut hsla = palette::IntoColor::<palette::Okhsla>::into_color(
    palette::Alpha::<palette::LinSrgb<u8>, f32>::new(red, green, blue, alpha)
      .into_format::<f32, f32>(),
  );
  hsla.saturation = saturation;
  let processed = palette::IntoColor::<palette::LinSrgba>::into_color(hsla).into_format::<u8, f32>();
  let palette::Alpha::<palette::LinSrgb::<u8>, f32> {
    color: palette::LinSrgb::<u8> {
      red,
      green,
      blue,
      ..
    },
    alpha
  } = processed;
  json!(Rgba {
    red,
    green,
    blue,
    alpha
  })
});

pub fn register(handlebars: &mut handlebars::Handlebars) {
  handlebars.register_helper("hex", Box::new(HexHelper));
  handlebars.register_helper("hexa", Box::new(HexaHelper));
  handlebars.register_helper("rgb", Box::new(RgbHelper));
  handlebars.register_helper("rgba", Box::new(RgbaHelper));
  handlebars.register_helper("set-alpha", Box::new(SetAlphaHelper));
  handlebars.register_helper("set-lightness", Box::new(SetLightnessHelper));
  handlebars.register_helper("set-saturation", Box::new(SetSaturationHelper));
}
