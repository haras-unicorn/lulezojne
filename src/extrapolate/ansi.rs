use lazy_static::lazy_static;
use palette::{IntoColor, Mix};

#[derive(Debug, Clone)]
pub struct Config {
  pub main_factor: f32,
  pub gradient_factor: f32,
  pub grayscale_factor: f32,
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
  static ref GREY: Color = opaque(2 * *THIRD, 2 * *THIRD, 2 * *THIRD);
  static ref BGREY: Color = opaque(*THIRD, *THIRD, *THIRD);
  static ref BRED: Color = opaque(*FULL, *EMPTY, *EMPTY);
  static ref BGREEN: Color = opaque(*EMPTY, *FULL, *EMPTY);
  static ref BBLUE: Color = opaque(*EMPTY, *EMPTY, *FULL);
  static ref BCYAN: Color = opaque(*EMPTY, *FULL, *FULL);
  static ref BYELLOW: Color = opaque(*FULL, *FULL, *EMPTY);
  static ref BMAGENTA: Color = opaque(*FULL, *EMPTY, *FULL);
  static ref WHITE: Color = opaque(*FULL, *FULL, *FULL);
  static ref MAIN: Vec<Color> = vec![
    *BLACK, *RED, *GREEN, *YELLOW, *BLUE, *MAGENTA, *CYAN, *GREY, *BGREY,
    *BRED, *BGREEN, *BYELLOW, *BBLUE, *BMAGENTA, *BCYAN, *WHITE
  ];
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

lazy_static! {
  static ref ANSI: Vec<Color> = {
    let mut result = Vec::new();
    result.append(&mut (*MAIN).clone());
    result.append(&mut (*GRADIENT).clone());
    result.append(&mut (*GRAYSCALE).clone());
    result
  };
}

pub fn from(palette: Vec<Rgba>, config: Config) -> Vec<Rgba> {
  let color_palette = from_rgba(&palette);

  let mut result_main = (*MAIN)
    .iter()
    .map(|color| {
      mix(
        *color,
        closest_to(&color_palette, *color).unwrap_or_default(),
        config.main_factor,
      )
    })
    .collect::<Vec<_>>();
  let mut result_gradient = (*GRADIENT)
    .iter()
    .map(|color| {
      mix(
        *color,
        closest_to(&color_palette, *color).unwrap_or_default(),
        config.gradient_factor,
      )
    })
    .collect::<Vec<_>>();
  let mut result_grayscale = (*GRAYSCALE)
    .iter()
    .map(|color| {
      mix(
        *color,
        closest_to(&color_palette, *color).unwrap_or_default(),
        config.grayscale_factor,
      )
    })
    .collect::<Vec<_>>();

  let result = {
    let mut result = Vec::with_capacity(256);
    result.append(&mut result_main);
    result.append(&mut result_gradient);
    result.append(&mut result_grayscale);
    result
  };

  to_rgba(&result)
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

fn to_rgba(palette: &[Color]) -> Vec<Rgba> {
  palette
    .iter()
    .map(|computational| {
      let DiscreteRgba {
        color: DiscreteRgb {
          red, green, blue, ..
        },
        alpha,
      } = IntoColor::<ContinuousRgba>::into_color(*computational)
        .into_linear::<f32, f32>()
        .into_format::<u8, f32>();
      Rgba {
        red,
        green,
        blue,
        alpha,
      }
    })
    .collect()
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
