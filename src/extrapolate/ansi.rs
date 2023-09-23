use lazy_static::lazy_static;
use palette::{IntoColor, Mix};

#[derive(Debug, Clone)]
pub struct Config {
  pub main: MainConfig,
  pub gradient_mix_factor: f32,
  pub grayscale_mix_factor: f32,
}

#[derive(Debug, Clone)]
pub struct MainConfig {
  pub mix_factor: f32,
  pub lightness_range: (f32, f32),
  pub saturation_range: (f32, f32),
}

#[derive(Debug, Clone)]
pub struct Result {
  pub main: ResultMain,
  pub gradient: Vec<Rgba>,
  pub grayscale: Vec<Rgba>,
}

#[derive(Debug, Clone)]
pub struct ResultMain {
  pub black: Rgba,
  pub red: Rgba,
  pub green: Rgba,
  pub blue: Rgba,
  pub cyan: Rgba,
  pub yellow: Rgba,
  pub magenta: Rgba,
  pub grey: Rgba,
  pub bright_grey: Rgba,
  pub bright_red: Rgba,
  pub bright_green: Rgba,
  pub bright_blue: Rgba,
  pub bright_cyan: Rgba,
  pub bright_yellow: Rgba,
  pub bright_magenta: Rgba,
  pub white: Rgba,
}

#[derive(Debug, Clone)]
pub struct Rgba {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
  pub alpha: f32,
}

type Color = palette::Okhsla<f32>;
type Hue = palette::OklabHue<f32>;
type ContinuonsRgb = palette::Srgb<f32>;
type ContinuousRgba = palette::Alpha<ContinuonsRgb, f32>;
type DiscreteRgb = palette::LinSrgb<u8>;
type DiscreteRgba = palette::Alpha<DiscreteRgb, f32>;

lazy_static! {
  static ref EMPTY: u8 = 0;
  static ref FULL: u8 = 255;
  static ref PART32: u8 = (Into::<f32>::into(*FULL) / 32.0f32).floor() as u8;
  static ref SIXTH: u8 = (Into::<f32>::into(*FULL) / 6.0f32).floor() as u8;
  static ref THIRD: u8 = (Into::<f32>::into(*FULL) / 3.0f32).floor() as u8;
  static ref HALF: u8 = (Into::<f32>::into(*FULL) / 2.0f32).floor() as u8;
}

lazy_static! {
  static ref BLACK: Color = opaque(*EMPTY, *EMPTY, *EMPTY);
  static ref RED: Color = opaque(*HALF, *EMPTY, *EMPTY);
  static ref GREEN: Color = opaque(*EMPTY, *HALF, *EMPTY);
  static ref BLUE: Color = opaque(*EMPTY, *EMPTY, *HALF);
  static ref CYAN: Color = opaque(*EMPTY, *HALF, *HALF);
  static ref YELLOW: Color = opaque(*HALF, *HALF, *EMPTY);
  static ref MAGENTA: Color = opaque(*HALF, *EMPTY, *HALF);
  static ref BRIGHT_GREY: Color = opaque(2 * *THIRD, 2 * *THIRD, 2 * *THIRD);
  static ref GREY: Color = opaque(*THIRD, *THIRD, *THIRD);
  static ref BRIGHT_RED: Color = opaque(*FULL, *EMPTY, *EMPTY);
  static ref BRIGHT_GREEN: Color = opaque(*EMPTY, *FULL, *EMPTY);
  static ref BRIGHT_BLUE: Color = opaque(*EMPTY, *EMPTY, *FULL);
  static ref BRIGHT_CYAN: Color = opaque(*EMPTY, *FULL, *FULL);
  static ref BRIGHT_YELLOW: Color = opaque(*FULL, *FULL, *EMPTY);
  static ref BRIGHT_MAGENTA: Color = opaque(*FULL, *EMPTY, *FULL);
  static ref WHITE: Color = opaque(*FULL, *FULL, *FULL);
}

lazy_static! {
  static ref GRADIENT: Vec<Color> = (0..6)
    .flat_map(move |r| (0..6).map(move |g| (0..6).map(move |b| opaque(
      r * *SIXTH,
      g * *SIXTH,
      b * *SIXTH
    ))))
    .flatten()
    .collect();
}

lazy_static! {
  static ref GRAYSCALE: Vec<Color> = (0..32)
    .map(|i| opaque(i * *PART32, i * *PART32, i * *PART32))
    .collect();
}

pub fn from(palette: Vec<Rgba>, config: Config) -> Result {
  let palette = from_rgba(palette);

  Result {
    main: ResultMain {
      black: make_main_color(&palette, *BLACK, &config),
      red: make_main_color(&palette, *RED, &config),
      green: make_main_color(&palette, *GREEN, &config),
      blue: make_main_color(&palette, *BLUE, &config),
      cyan: make_main_color(&palette, *CYAN, &config),
      yellow: make_main_color(&palette, *YELLOW, &config),
      magenta: make_main_color(&palette, *MAGENTA, &config),
      grey: make_main_color(&palette, *GREY, &config),
      bright_grey: make_main_color(&palette, *BRIGHT_GREY, &config),
      bright_red: make_main_color(&palette, *BRIGHT_RED, &config),
      bright_green: make_main_color(&palette, *BRIGHT_GREEN, &config),
      bright_blue: make_main_color(&palette, *BRIGHT_BLUE, &config),
      bright_cyan: make_main_color(&palette, *BRIGHT_CYAN, &config),
      bright_yellow: make_main_color(&palette, *BRIGHT_YELLOW, &config),
      bright_magenta: make_main_color(&palette, *BRIGHT_MAGENTA, &config),
      white: make_main_color(&palette, *WHITE, &config),
    },
    gradient: (*GRADIENT)
      .iter()
      .map(|color| make_gradient_color(&palette, *color, &config))
      .collect(),
    grayscale: (*GRAYSCALE)
      .iter()
      .map(|color| make_grayscale_color(&palette, *color, &config))
      .collect(),
  }
}

fn from_rgba(mut palette: Vec<Rgba>) -> Vec<Color> {
  palette
    .drain(0..)
    .map(
      |Rgba {
         red,
         green,
         blue,
         alpha,
       }| {
        ContinuousRgba::from_linear(
          DiscreteRgba::new(red, green, blue, alpha).into_format::<f32, f32>(),
        )
        .into_color()
      },
    )
    .collect()
}

fn make_main_color(palette: &[Color], color: Color, config: &Config) -> Rgba {
  to_rgba(clamp_saturation_lightness(
    mix_hue_closest_to(palette, color, config.main.mix_factor),
    config.main.saturation_range,
    config.main.lightness_range,
  ))
}

fn make_gradient_color(
  palette: &[Color],
  color: Color,
  config: &Config,
) -> Rgba {
  to_rgba(mix_color_closest_to(
    palette,
    color,
    config.gradient_mix_factor,
  ))
}

fn make_grayscale_color(
  palette: &[Color],
  color: Color,
  config: &Config,
) -> Rgba {
  to_rgba(mix_color_closest_to(
    palette,
    color,
    config.grayscale_mix_factor,
  ))
}

fn clamp_saturation_lightness(
  color: Color,
  saturation_range: (f32, f32),
  lightness_range: (f32, f32),
) -> Color {
  Color::new(
    color.hue,
    color
      .saturation
      .clamp(saturation_range.0, saturation_range.1),
    color.lightness.clamp(lightness_range.0, lightness_range.1),
    color.alpha,
  )
}

fn mix_color_closest_to(palette: &[Color], color: Color, factor: f32) -> Color {
  mix_color(
    color,
    closest_color(palette, color).unwrap_or_default(),
    factor,
  )
}

fn mix_hue_closest_to(palette: &[Color], color: Color, factor: f32) -> Color {
  mix_hue(
    color,
    closest_hue(palette, color).unwrap_or_default(),
    factor,
  )
}

fn to_rgba(color: Color) -> Rgba {
  let DiscreteRgba {
    color: DiscreteRgb {
      red, green, blue, ..
    },
    alpha,
  } = IntoColor::<ContinuousRgba>::into_color(color)
    .into_linear::<f32, f32>()
    .into_format::<u8, f32>();
  Rgba {
    red,
    green,
    blue,
    alpha,
  }
}

fn closest_color(palette: &[Color], reference: Color) -> Option<Color> {
  let ref_lab: palette::Oklaba<f32> = reference.into_color();
  palette
    .iter()
    .min_by(|x, y| {
      let x_lab: palette::Oklaba<f32> = (**x).into_color();
      let y_lab: palette::Oklaba<f32> = (**y).into_color();
      let dist_x = palette::color_difference::HyAb::hybrid_distance(
        x_lab.color,
        ref_lab.color,
      );
      let dist_y = palette::color_difference::HyAb::hybrid_distance(
        y_lab.color,
        ref_lab.color,
      );
      dist_x.total_cmp(&dist_y)
    })
    .cloned()
}

fn mix_color(base: Color, color: Color, factor: f32) -> Color {
  base.mix(color, factor)
}

fn closest_hue(palette: &[Color], reference: Color) -> Option<Color> {
  palette
    .iter()
    .min_by(|x, y| {
      let dist_x = (x.hue.into_radians() - reference.hue.into_radians()).abs();
      let dist_y = (y.hue.into_radians() - reference.hue.into_radians()).abs();
      dist_x.total_cmp(&dist_y)
    })
    .cloned()
}

fn mix_hue(base: Color, color: Color, factor: f32) -> Color {
  let hue = Hue::from_radians(
    (1.0 - factor) * base.hue.into_radians()
      + factor * color.hue.into_radians(),
  );
  Color::new(hue, base.saturation, base.lightness, base.alpha)
}

fn opaque(r: u8, g: u8, b: u8) -> Color {
  ContinuousRgba::from_linear(
    DiscreteRgba::new(r, g, b, 1.0).into_format::<f32, f32>(),
  )
  .into_color()
}
