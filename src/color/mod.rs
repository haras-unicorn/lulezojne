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

// TODO: no leaky abstractions

#[derive(Debug, Default, Clone, Copy)]
pub struct Rgba {
  pub red: FloatingComponent,
  pub green: FloatingComponent,
  pub blue: FloatingComponent,
  pub alpha: FloatingComponent,
}

type Lab = palette::Oklaba<FloatingComponent>;
type Hsla = palette::Okhsla<FloatingComponent>;
type Srgba = palette::Srgba<FloatingComponent>;
type ColorImpl = Rgba;

type Hue = palette::hues::OklabHue<FloatingComponent>;
type Lumma = palette::LinLumaa<palette::white_point::D65, FloatingComponent>;

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
      alpha: FloatingComponent::max_component_value(),
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

  fn luminance(self) -> Lumma;
  fn hue(self) -> Hue;

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

  fn luminance(self) -> Lumma {
    self.to_lumma()
  }

  fn hue(self) -> Hue {
    self.to_hsla().hue
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
    hsla.color.hue = Hue::from_radians(hue.to_floating_component());
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
        FloatingComponent::min_component_value(),
        FloatingComponent::max_component_value(),
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
  fn to_lab(self) -> Lab;
  fn from_lab(lab: Lab) -> Self;

  fn to_hsla(self) -> Hsla;
  fn from_hsla(hsla: Hsla) -> Self;

  fn to_lumma(self) -> Lumma;

  fn to_srgba(self) -> Srgba;
  fn from_srgba(srgba: Srgba) -> Self;
}

use palette::IntoColor;

impl InternalColor for ColorImpl {
  fn to_lab(self) -> Lab {
    self.to_srgba().into_color()
  }

  fn from_lab(lab: Lab) -> Self {
    Self::from_srgba(lab.into_color())
  }

  fn to_hsla(self) -> Hsla {
    self.to_srgba().into_color()
  }

  fn from_hsla(hsla: Hsla) -> Self {
    Self::from_srgba(hsla.into_color())
  }

  fn to_lumma(self) -> Lumma {
    let srgba = self.to_srgba();
    let encoded: Lumma = srgba.into_color();
    encoded.into_linear()
  }

  fn to_srgba(self) -> Srgba {
    Srgba::new(self.red, self.green, self.blue, self.alpha)
  }

  fn from_srgba(srgba: Srgba) -> Self {
    Self {
      red: srgba.red,
      green: srgba.green,
      blue: srgba.blue,
      alpha: srgba.alpha,
    }
  }
}
