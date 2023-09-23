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
  let palette = from_rgba(&palette);

  Result {
    main: ResultMain {
      black: mix_hue_closest_to(&palette, *BLACK, config.main.mix_factor),
      red: mix_hue_closest_to(&palette, *RED, config.main.mix_factor),
      green: mix_hue_closest_to(&palette, *GREEN, config.main.mix_factor),
      blue: mix_hue_closest_to(&palette, *BLUE, config.main.mix_factor),
      cyan: mix_hue_closest_to(&palette, *CYAN, config.main.mix_factor),
      yellow: mix_hue_closest_to(&palette, *YELLOW, config.main.mix_factor),
      magenta: mix_hue_closest_to(&palette, *MAGENTA, config.main.mix_factor),
      grey: mix_hue_closest_to(&palette, *GREY, config.main.mix_factor),
      bright_grey: mix_hue_closest_to(
        &palette,
        *BRIGHT_GREY,
        config.main.mix_factor,
      ),
      bright_red: mix_hue_closest_to(
        &palette,
        *BRIGHT_RED,
        config.main.mix_factor,
      ),
      bright_green: mix_hue_closest_to(
        &palette,
        *BRIGHT_GREEN,
        config.main.mix_factor,
      ),
      bright_blue: mix_hue_closest_to(
        &palette,
        *BRIGHT_BLUE,
        config.main.mix_factor,
      ),
      bright_cyan: mix_hue_closest_to(
        &palette,
        *BRIGHT_CYAN,
        config.main.mix_factor,
      ),
      bright_yellow: mix_hue_closest_to(
        &palette,
        *BRIGHT_YELLOW,
        config.main.mix_factor,
      ),
      bright_magenta: mix_hue_closest_to(
        &palette,
        *BRIGHT_MAGENTA,
        config.main.mix_factor,
      ),
      white: mix_hue_closest_to(&palette, *WHITE, config.main.mix_factor),
    },
    gradient: (*GRADIENT)
      .iter()
      .map(|color| {
        mix_color_closest_to(&palette, *color, config.gradient_mix_factor)
      })
      .collect(),
    grayscale: (*GRAYSCALE)
      .iter()
      .map(|color| {
        mix_color_closest_to(&palette, *color, config.grayscale_mix_factor)
      })
      .collect(),
  }
}

fn from_rgba(palette: &[Rgba]) -> Vec<Color> {
  palette
    .iter()
    .map(
      |Rgba {
         red,
         green,
         blue,
         alpha,
       }| {
        ContinuousRgba::from_linear(
          DiscreteRgba::new(*red, *green, *blue, *alpha)
            .into_format::<f32, f32>(),
        )
        .into_color()
      },
    )
    .collect()
}

fn mix_color_closest_to(palette: &[Color], color: Color, factor: f32) -> Rgba {
  to_rgba(&mix_color(
    color,
    closest_color(palette, color).unwrap_or_default(),
    factor,
  ))
}

fn mix_hue_closest_to(palette: &[Color], color: Color, factor: f32) -> Rgba {
  to_rgba(&mix_hue(
    color,
    closest_hue(palette, color).unwrap_or_default(),
    factor,
  ))
}

fn to_rgba(color: &Color) -> Rgba {
  let DiscreteRgba {
    color: DiscreteRgb {
      red, green, blue, ..
    },
    alpha,
  } = IntoColor::<ContinuousRgba>::into_color(*color)
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
