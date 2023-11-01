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

#[derive(Debug, Default, Clone, Copy)]
pub struct Rgba {
  pub red: FloatingComponent,
  pub green: FloatingComponent,
  pub blue: FloatingComponent,
  pub alpha: FloatingComponent,
}

type Hsla = palette::Okhsla<FloatingComponent>;
type Lumma = palette::SrgbLumaa<FloatingComponent>;
type Srgba = palette::Srgba<FloatingComponent>;
type ColorImpl = Rgba;

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
  fn luminance<TComponent: Component>(self) -> TComponent;
  fn saturation<TComponent: Component>(self) -> TComponent;
  fn hue<TComponent: Component>(self) -> TComponent;

  fn alpha<TComponent: Component>(self) -> TComponent;

  fn with_lightness<TComponent: Component>(self, lightness: TComponent)
    -> Self;
  fn with_luminance<TComponent: Component>(self, luminance: TComponent)
    -> Self;
  fn with_saturation<TComponent: Component>(
    self,
    saturation: TComponent,
  ) -> Self;
  fn with_alpha<TComponent: Component>(self, alpha: TComponent) -> Self;

  fn multiply_lightness<TFactor: Factor>(self, factor: TFactor) -> Self;
  fn multiply_luminance<TFactor: Factor>(self, factor: TFactor) -> Self;
  fn multiply_saturation<TFactor: Factor>(self, factor: TFactor) -> Self;
  fn multiply_alpha<TFactor: Factor>(self, factor: TFactor) -> Self;

  fn to_colored_string(self) -> String;
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

  fn lightness<TComponent: Component>(self) -> TComponent {
    TComponent::from_floating_component(self.to_hsla().lightness)
  }

  fn saturation<TComponent: Component>(self) -> TComponent {
    TComponent::from_floating_component(self.to_hsla().saturation)
  }

  fn hue<TComponent: Component>(self) -> TComponent {
    TComponent::from_floating_component(
      self.to_hsla().hue.into_positive_radians()
        / FloatingComponent::from_f32(2f32 * std::f32::consts::PI),
    )
  }

  fn luminance<TComponent: Component>(self) -> TComponent {
    TComponent::from_floating_component(self.to_lumma().luma)
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

  fn with_luminance<TComponent: Component>(
    self,
    luminance: TComponent,
  ) -> Self {
    let mut lumma = self.to_lumma();
    lumma.luma = luminance.to_floating_component();
    Self::from_lumma(lumma)
  }

  fn with_saturation<TComponent: Component>(
    self,
    saturation: TComponent,
  ) -> Self {
    let mut hsla = self.to_hsla();
    hsla.saturation = saturation.to_floating_component();
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

  fn multiply_luminance<TFactor: Factor>(self, factor: TFactor) -> Self {
    let mut lumma = self.to_lumma();
    lumma.luma = factor.multiply(lumma.luma);
    Self::from_lumma(lumma)
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

  fn to_colored_string(self) -> String {
    let srgba = self.to_srgba();
    let red = srgba.red.to_integer_component();
    let green = srgba.green.to_integer_component();
    let blue = srgba.blue.to_integer_component();
    let alpha = srgba.alpha.to_floating_component();

    colored::Colorize::truecolor(
      format!("rgba({red}, {green}, {blue}, {alpha})").as_str(),
      red,
      green,
      blue,
    )
    .to_string()
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
  fn to_hsla(self) -> Hsla;
  fn from_hsla(hsla: Hsla) -> Self;

  fn to_lumma(self) -> Lumma;
  fn from_lumma(lumma: Lumma) -> Self;

  fn to_srgba(self) -> Srgba;
  fn from_srgba(srgba: Srgba) -> Self;
}

impl InternalColor for ColorImpl {
  fn to_hsla(self) -> Hsla {
    palette::IntoColor::<Hsla>::into_color(self.to_srgba())
  }

  fn from_hsla(hsla: Hsla) -> Self {
    Self::from_srgba(palette::IntoColor::<Srgba>::into_color(hsla))
  }

  fn to_lumma(self) -> Lumma {
    palette::IntoColor::<Lumma>::into_color(self.to_srgba())
  }

  fn from_lumma(lumma: Lumma) -> Self {
    Self::from_srgba(palette::IntoColor::<Srgba>::into_color(lumma))
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
