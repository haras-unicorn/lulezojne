mod accessibility;
mod analysis;
mod component;
mod extraction;
mod factor;
mod serialization;

pub use accessibility::*;
pub use analysis::*;
pub use component::*;
pub use extraction::*;
pub use factor::*;

// TODO: use https://git.apcacontrast.com/
// TODO: cymk

#[derive(Debug, Default, Clone, Copy)]
pub struct Rgba {
  pub red: FloatingComponent,
  pub green: FloatingComponent,
  pub blue: FloatingComponent,
  pub alpha: FloatingComponent,
}

type ColorImpl = Rgba;

type LabImpl = palette::Oklaba<FloatingComponent>;
type HslaImpl = palette::Okhsla<FloatingComponent>;
type RgbaImpl = palette::Srgba<FloatingComponent>;

type HueImpl = palette::hues::OklabHue<FloatingComponent>;
type LuminanceImpl =
  palette::LinLuma<palette::white_point::D65, FloatingComponent>;

impl Rgba {
  #[allow(dead_code)]
  pub fn opaque<TComponent: Component>(
    red: TComponent,
    green: TComponent,
    blue: TComponent,
  ) -> Self {
    Self {
      red: red.to_floating_component(),
      green: green.to_floating_component(),
      blue: blue.to_floating_component(),
      alpha: FloatingComponent::max_component(),
    }
  }

  #[allow(dead_code)]
  pub fn transparent<TComponent: Component>(
    red: TComponent,
    green: TComponent,
    blue: TComponent,
    alpha: TComponent,
  ) -> Self {
    Self {
      red: red.to_floating_component(),
      green: green.to_floating_component(),
      blue: blue.to_floating_component(),
      alpha: alpha.to_floating_component(),
    }
  }
}

pub trait Color: Clone + Copy {
  fn red<TComponent: Component>(self) -> TComponent;
  fn green<TComponent: Component>(self) -> TComponent;
  fn blue<TComponent: Component>(self) -> TComponent;

  fn lightness<TComponent: Component>(self) -> TComponent;
  fn saturation<TComponent: Component>(self) -> TComponent;
  fn alpha<TComponent: Component>(self) -> TComponent;

  fn luminance<TComponent: Component>(self) -> TComponent;
  fn hue<TComponent: Component>(self) -> TComponent;

  fn with_lightness<TComponent: Component>(self, lightness: TComponent)
    -> Self;
  fn with_saturation<TComponent: Component>(
    self,
    saturation: TComponent,
  ) -> Self;
  fn with_hue<TComponent: Component>(self, hue: TComponent) -> Self;
  fn with_alpha<TComponent: Component>(self, alpha: TComponent) -> Self;

  fn multiply_lightness<TFactor: Factor>(self, factor: TFactor) -> Self;
  fn multiply_saturation<TFactor: Factor>(self, factor: TFactor) -> Self;
  fn multiply_alpha<TFactor: Factor>(self, factor: TFactor) -> Self;

  fn add_lightness<TFactor: Factor>(self, factor: TFactor) -> Self;
  fn add_saturation<TFactor: Factor>(self, factor: TFactor) -> Self;
  fn add_alpha<TFactor: Factor>(self, factor: TFactor) -> Self;

  fn distance<TComponent: Component>(self, other: Self) -> TComponent;

  fn to_colored_string(self) -> String;
  fn color_square(self) -> String;
  fn to_rgba(self) -> Rgba;
}

impl Color for ColorImpl {
  fn red<TComponent: Component>(self) -> TComponent {
    TComponent::from_floating_component(self.to_rgba().red)
  }

  fn green<TComponent: Component>(self) -> TComponent {
    TComponent::from_floating_component(self.to_rgba().green)
  }

  fn blue<TComponent: Component>(self) -> TComponent {
    TComponent::from_floating_component(self.to_rgba().blue)
  }

  fn luminance<TComponent: Component>(self) -> TComponent {
    TComponent::from_floating_component(self.to_lumma().luma)
  }

  fn hue<TComponent: Component>(self) -> TComponent {
    TComponent::from_floating_component(
      self.to_hsla().color.hue.into_positive_radians()
        / (2.0f32 * std::f32::consts::PI),
    )
  }

  fn lightness<TComponent: Component>(self) -> TComponent {
    TComponent::from_floating_component(self.to_hsla().lightness)
  }

  fn saturation<TComponent: Component>(self) -> TComponent {
    TComponent::from_floating_component(self.to_hsla().saturation)
  }

  fn alpha<TComponent: Component>(self) -> TComponent {
    TComponent::from_floating_component(self.to_rgba().alpha)
  }

  fn with_lightness<TComponent: Component>(
    self,
    lightness: TComponent,
  ) -> Self {
    let mut hsla = self.to_hsla();
    hsla.lightness = lightness.to_floating_component();
    Self::from_hsla(hsla)
  }

  fn with_saturation<TComponent: Component>(
    self,
    saturation: TComponent,
  ) -> Self {
    let mut hsla = self.to_hsla();
    hsla.saturation = saturation.to_floating_component();
    Self::from_hsla(hsla)
  }

  fn with_hue<TComponent: Component>(self, hue: TComponent) -> Self {
    let mut hsla = self.to_hsla();
    hsla.color.hue = HueImpl::from_radians(hue.to_floating_component());
    Self::from_hsla(hsla)
  }

  fn with_alpha<TComponent: Component>(self, alpha: TComponent) -> Self {
    let mut srgba = self.to_srgba();
    srgba.alpha = alpha.to_floating_component();
    Self::from_srgba(srgba)
  }

  fn multiply_lightness<TFactor: Factor>(self, factor: TFactor) -> Self {
    let mut hsla = self.to_hsla();
    hsla.lightness = factor.multiply(hsla.lightness);
    Self::from_hsla(hsla)
  }

  fn multiply_saturation<TFactor: Factor>(self, factor: TFactor) -> Self {
    let mut hsla = self.to_hsla();
    hsla.saturation = factor.multiply(hsla.saturation);
    Self::from_hsla(hsla)
  }

  fn multiply_alpha<TFactor: Factor>(self, factor: TFactor) -> Self {
    let mut srgba = self.to_srgba();
    srgba.alpha = factor.multiply(srgba.alpha);
    Self::from_srgba(srgba)
  }

  fn add_lightness<TFactor: Factor>(self, factor: TFactor) -> Self {
    let mut hsla = self.to_hsla();
    hsla.lightness = factor.add(hsla.lightness);
    Self::from_hsla(hsla)
  }

  fn add_saturation<TFactor: Factor>(self, factor: TFactor) -> Self {
    let mut hsla = self.to_hsla();
    hsla.saturation = factor.add(hsla.saturation);
    Self::from_hsla(hsla)
  }

  fn add_alpha<TFactor: Factor>(self, factor: TFactor) -> Self {
    let mut srgba = self.to_srgba();
    srgba.alpha = factor.add(srgba.alpha);
    Self::from_srgba(srgba)
  }

  fn distance<TComponent: Component>(self, other: Self) -> TComponent {
    TComponent::from_f32(palette::color_difference::HyAb::hybrid_distance(
      self.to_lab().color,
      other.to_lab().color,
    ))
  }

  fn to_colored_string(self) -> String {
    let srgba = self.to_srgba();
    let red = srgba.red.to_integer_component();
    let green = srgba.green.to_integer_component();
    let blue = srgba.blue.to_integer_component();
    let alpha = srgba.alpha.to_floating_component();

    let foreground = colored::Colorize::truecolor(
      format!("rgba({red}, {green}, {blue}, {alpha})").as_str(),
      red,
      green,
      blue,
    )
    .to_string();

    if self.lightness::<FloatingComponent>()
      < FloatingComponent::median(
        FloatingComponent::min_component(),
        FloatingComponent::max_component(),
      )
    {
      colored::Colorize::on_truecolor(foreground.as_str(), 255, 255, 255)
        .to_string()
    } else {
      colored::Colorize::on_truecolor(foreground.as_str(), 0, 0, 0).to_string()
    }
  }

  fn color_square(self) -> String {
    let srgba = self.to_srgba();
    let red = srgba.red.to_integer_component();
    let green = srgba.green.to_integer_component();
    let blue = srgba.blue.to_integer_component();

    colored::Colorize::truecolor("█", red, green, blue).to_string()
  }

  fn to_rgba(self) -> Rgba {
    let srgba = self.to_srgba();

    Rgba {
      red: srgba.red,
      green: srgba.green,
      blue: srgba.blue,
      alpha: srgba.alpha,
    }
  }
}

trait InternalColor {
  fn to_lab(self) -> LabImpl;
  fn from_lab(lab: LabImpl) -> Self;

  fn to_hsla(self) -> HslaImpl;
  fn from_hsla(hsla: HslaImpl) -> Self;

  fn to_lumma(self) -> LuminanceImpl;

  fn to_srgba(self) -> RgbaImpl;
  fn from_srgba(srgba: RgbaImpl) -> Self;
}

use palette::IntoColor;

impl InternalColor for ColorImpl {
  fn to_lab(self) -> LabImpl {
    self.to_srgba().into_color()
  }

  fn from_lab(lab: LabImpl) -> Self {
    Self::from_srgba(lab.into_color())
  }

  fn to_hsla(self) -> HslaImpl {
    self.to_srgba().into_color()
  }

  fn from_hsla(hsla: HslaImpl) -> Self {
    Self::from_srgba(hsla.into_color())
  }

  fn to_lumma(self) -> LuminanceImpl {
    let srgba = self.to_srgba();
    let encoded: LuminanceImpl = srgba.into_color();
    encoded.into_linear()
  }

  fn to_srgba(self) -> RgbaImpl {
    RgbaImpl::new(self.red, self.green, self.blue, self.alpha)
  }

  fn from_srgba(srgba: RgbaImpl) -> Self {
    Self {
      red: srgba.red,
      green: srgba.green,
      blue: srgba.blue,
      alpha: srgba.alpha,
    }
  }
}

mod contrast {
  use super::{Color, FloatingComponent, LuminanceImpl};

  macro_rules! impl_contrast_check {
    ( $check: ident) => {
      pub fn $check<TForeground: Color, TBackground: Color>(
        foreground: TForeground,
        background: TBackground,
      ) -> bool {
        palette::color_difference::Wcag21RelativeContrast::$check(
          LuminanceImpl::new(foreground.luminance::<FloatingComponent>()),
          LuminanceImpl::new(background.luminance::<FloatingComponent>()),
        )
      }
    };
  }

  impl_contrast_check!(has_min_contrast_text);
  impl_contrast_check!(has_min_contrast_large_text);
  impl_contrast_check!(has_enhanced_contrast_text);
  impl_contrast_check!(has_enhanced_contrast_large_text);
  impl_contrast_check!(has_min_contrast_graphics);
}

mod channel {
  use super::{Color, Component};

  pub fn saturation_channel<TComponent: Component>(
    color: impl Color,
  ) -> TComponent {
    color.saturation::<TComponent>()
  }

  pub fn lightness_channel<TComponent: Component>(
    color: impl Color,
  ) -> TComponent {
    color.lightness::<TComponent>()
  }

  pub fn hue_channel<TComponent: Component>(color: impl Color) -> TComponent {
    color.hue::<TComponent>()
  }
}

#[allow(dead_code)]
pub fn closest<TColor: Color, TIntoIter: IntoIterator<Item = TColor>>(
  color: TColor,
  colors: TIntoIter,
) -> Option<TColor> {
  colors.into_iter().min_by(move |lhs, rhs| {
    let lhs_dist = (lhs.distance::<FloatingComponent>(color)).abs();
    let rhs_dist = (rhs.distance::<FloatingComponent>(color)).abs();
    lhs_dist.total_cmp(&rhs_dist)
  })
}

#[allow(dead_code)]
pub fn clamp_hue_around<
  TColor: Color,
  TReference: Color,
  TTolerance: Component,
>(
  reference: TReference,
  tolerance: TTolerance,
  color: TColor,
) -> TColor {
  let half_tolerance = tolerance / TTolerance::from_f32(2.0);
  let reference_hue = reference.hue::<TTolerance>();
  let min_hue = reference_hue - half_tolerance;
  let max_hue = reference_hue + half_tolerance;
  let color_hue = color.hue::<TTolerance>();
  let hue = if color_hue < min_hue {
    min_hue
  } else if color_hue > max_hue {
    max_hue
  } else {
    color_hue
  };
  color.with_hue(hue)
}
