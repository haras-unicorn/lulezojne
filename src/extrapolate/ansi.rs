use lazy_static::lazy_static;
use palette::{IntoColor, Mix};

#[derive(Debug, Clone)]
pub struct Config {
  pub main_factor: f32,
  pub gradient_factor: f32,
  pub grayscale_factor: f32,
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

type Color = palette::Oklaba<f32>;
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
      black: mix_closest_to(&palette, *BLACK, config.main_factor),
      red: mix_closest_to(&palette, *RED, config.main_factor),
      green: mix_closest_to(&palette, *GREEN, config.main_factor),
      blue: mix_closest_to(&palette, *BLUE, config.main_factor),
      cyan: mix_closest_to(&palette, *CYAN, config.main_factor),
      yellow: mix_closest_to(&palette, *YELLOW, config.main_factor),
      magenta: mix_closest_to(&palette, *MAGENTA, config.main_factor),
      grey: mix_closest_to(&palette, *GREY, config.main_factor),
      bright_grey: mix_closest_to(&palette, *BRIGHT_GREY, config.main_factor),
      bright_red: mix_closest_to(&palette, *BRIGHT_RED, config.main_factor),
      bright_green: mix_closest_to(&palette, *BRIGHT_GREEN, config.main_factor),
      bright_blue: mix_closest_to(&palette, *BRIGHT_BLUE, config.main_factor),
      bright_cyan: mix_closest_to(&palette, *BRIGHT_CYAN, config.main_factor),
      bright_yellow: mix_closest_to(
        &palette,
        *BRIGHT_YELLOW,
        config.main_factor,
      ),
      bright_magenta: mix_closest_to(
        &palette,
        *BRIGHT_MAGENTA,
        config.main_factor,
      ),
      white: mix_closest_to(&palette, *WHITE, config.main_factor),
    },
    gradient: (*GRADIENT)
      .iter()
      .map(|color| mix_closest_to(&palette, *color, config.gradient_factor))
      .collect(),
    grayscale: (*GRAYSCALE)
      .iter()
      .map(|color| mix_closest_to(&palette, *color, config.grayscale_factor))
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
          DiscreteRgba::from_components((*red, *green, *blue, *alpha))
            .into_format::<f32, f32>(),
        )
        .into_color()
      },
    )
    .collect()
}

fn mix_closest_to(palette: &[Color], color: Color, factor: f32) -> Rgba {
  to_rgba(&mix(
    color,
    closest_to(palette, color).unwrap_or_default(),
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

fn closest_to(palette: &[Color], reference: Color) -> Option<Color> {
  palette
    .iter()
    .min_by(|x, y| {
      let dist_x = palette::color_difference::HyAb::hybrid_distance(
        x.color,
        reference.color,
      );
      let dist_y = palette::color_difference::HyAb::hybrid_distance(
        y.color,
        reference.color,
      );
      dist_x.total_cmp(&dist_y)
    })
    .cloned()
}

fn mix(lhs: Color, rhs: Color, factor: f32) -> Color {
  lhs.mix(rhs, factor)
}

fn opaque(r: u8, g: u8, b: u8) -> Color {
  ContinuousRgba::from_linear(
    DiscreteRgba::from_components((r, g, b, 1.0)).into_format::<f32, f32>(),
  )
  .into_color()
}
