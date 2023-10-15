use colored::Colorize;
use lazy_static::lazy_static;
use palette::IntoColor;

#[derive(Debug, Clone, Copy)]
pub struct Config {
  pub main: AreaConfig,
  pub gradient: AreaConfig,
  pub grayscale: AreaConfig,
}

#[derive(Debug, Clone, Copy)]
pub struct AreaConfig {
  pub saturation_factor: f32,
  pub lightness_factor: f32,
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
  pub white: Rgba,
  pub bright_black: Rgba,
  pub bright_red: Rgba,
  pub bright_green: Rgba,
  pub bright_blue: Rgba,
  pub bright_cyan: Rgba,
  pub bright_yellow: Rgba,
  pub bright_magenta: Rgba,
  pub bright_white: Rgba,
}

#[derive(Debug, Clone)]
pub struct Rgba {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
  pub alpha: f32,
}

type Color = palette::Oklaba<f32>;
type Hsla = palette::Okhsla<f32>;
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
      black: mix_closest_to(&palette, *BLACK, config.main),
      red: mix_closest_to(&palette, *RED, config.main),
      green: mix_closest_to(&palette, *GREEN, config.main),
      blue: mix_closest_to(&palette, *BLUE, config.main),
      cyan: mix_closest_to(&palette, *CYAN, config.main),
      yellow: mix_closest_to(&palette, *YELLOW, config.main),
      magenta: mix_closest_to(&palette, *MAGENTA, config.main),
      white: mix_closest_to(&palette, *GREY, config.main),
      bright_black: mix_closest_to(&palette, *BRIGHT_GREY, config.main),
      bright_red: mix_closest_to(&palette, *BRIGHT_RED, config.main),
      bright_green: mix_closest_to(&palette, *BRIGHT_GREEN, config.main),
      bright_blue: mix_closest_to(&palette, *BRIGHT_BLUE, config.main),
      bright_cyan: mix_closest_to(&palette, *BRIGHT_CYAN, config.main),
      bright_yellow: mix_closest_to(&palette, *BRIGHT_YELLOW, config.main),
      bright_magenta: mix_closest_to(&palette, *BRIGHT_MAGENTA, config.main),
      bright_white: mix_closest_to(&palette, *WHITE, config.main),
    },
    gradient: (*GRADIENT)
      .iter()
      .map(|color| mix_closest_to(&palette, *color, config.gradient))
      .collect(),
    grayscale: (*GRAYSCALE)
      .iter()
      .map(|color| mix_closest_to(&palette, *color, config.grayscale))
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
        DiscreteRgba::new(*red, *green, *blue, *alpha)
          .into_format::<f32, f32>()
          .into_color()
      },
    )
    .collect()
}

fn to_rgba(color: Color) -> Rgba {
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

fn mix_closest_to(palette: &[Color], color: Color, config: AreaConfig) -> Rgba {
  let closest = closest_to(palette, color).unwrap_or_default();
  let mixed = mix(closest, color, config);
  let result = to_rgba(mixed);

  #[cfg(debug_assertions)]
  {
    print(color, closest, mixed, config);
  }

  result
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

fn mix(lhs: Color, rhs: Color, config: AreaConfig) -> Color {
  let lhs_hsla = palette::IntoColor::<Hsla>::into_color(lhs);
  let rhs_hsla = palette::IntoColor::<Hsla>::into_color(rhs);
  let saturation = lhs_hsla.saturation
    + (rhs_hsla.saturation - lhs_hsla.saturation) * config.saturation_factor;
  let lightness = lhs_hsla.lightness
    + (rhs_hsla.lightness - lhs_hsla.lightness) * config.lightness_factor;

  Hsla::new(lhs_hsla.color.hue, saturation, lightness, rhs.alpha).into_color()
}

fn opaque(r: u8, g: u8, b: u8) -> Color {
  ContinuousRgba::from_linear(
    DiscreteRgba::new(r, g, b, 1.0).into_format::<f32, f32>(),
  )
  .into_color()
}

fn print(reference: Color, closest: Color, result: Color, config: AreaConfig) {
  let Rgba {
    red: reference_red,
    green: reference_green,
    blue: reference_blue,
    alpha: reference_alpha,
  } = to_rgba(reference);
  let Rgba {
    red: closest_red,
    green: closest_green,
    blue: closest_blue,
    alpha: closest_alpha,
  } = to_rgba(closest);
  let Rgba {
    red: result_red,
    green: result_green,
    blue: result_blue,
    alpha: result_alpha,
  } = to_rgba(result);

  let reference = format!("rgba({reference_red}, {reference_green}, {reference_blue}, {reference_alpha})") 
     .custom_color(colored::CustomColor {
    r: reference_red,
    g: reference_green,
    b: reference_blue,
  }).to_string();

  let closest = format!(
    "rgba({closest_red}, {closest_green}, {closest_blue}, {closest_alpha})"
  )
  .custom_color(colored::CustomColor {
    r: closest_red,
    g: closest_green,
    b: closest_blue,
  })
  .to_string();

  let result = format!(
    "rgba({result_red}, {result_green}, {result_blue}, {result_alpha})"
  )
  .custom_color(colored::CustomColor {
    r: result_red,
    g: result_green,
    b: result_blue,
  })
  .to_string();

  let lightness_factor = config.lightness_factor;
  let saturation_factor = config.saturation_factor;

  let _ = std::io::Write::write_all(
    &mut std::io::stdout(),
    format!("{reference} -> ({closest}, {lightness_factor}, {saturation_factor}) -> {result}\n").as_bytes(),
  );
}
