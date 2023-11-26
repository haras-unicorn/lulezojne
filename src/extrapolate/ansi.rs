use std::sync::Arc;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::{
  color::{Component, FloatingComponent, Rgba},
  *,
};

#[derive(Clone)]
pub struct Extrapolator {
  #[allow(unused)]
  config: config::AnsiConfig,
  extractor: Arc<Mutex<Box<dyn extract::Extractor + Send + Sync>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Palette {
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
  pub gradient: Vec<Rgba>,
  pub grayscale: Vec<Rgba>,
}

impl Extrapolator {
  pub fn new(
    config: config::AnsiConfig,
    extractor: Arc<Mutex<Box<dyn extract::Extractor + Send + Sync>>>,
  ) -> Self {
    Self { config, extractor }
  }
}

#[async_trait::async_trait]
impl<'a> super::Extrapolator<'a, Palette> for Extrapolator {
  #[tracing::instrument(skip(self))]
  async fn extrapolate(&self) -> anyhow::Result<Palette> {
    let extracted256 = {
      let extractor = self.extractor.clone().lock_owned().await;
      extractor.prominent(255).await?
    };
    let extracted256 =
      color::filter_saturation_range(0.1f32, 1.0f32, extracted256)
        .collect::<Vec<_>>();

    let colors = Palette {
      black: color::darkest(extracted256.iter().cloned())
        .ok_or_else(|| anyhow::anyhow!("Failed to find ansi black"))?,
      red: color::closest_by_hue(*RED, extracted256.iter().cloned())
        .ok_or_else(|| anyhow::anyhow!("Failed to find ansi red"))?,
      green: color::closest_by_hue(*GREEN, extracted256.iter().cloned())
        .ok_or_else(|| anyhow::anyhow!("Failed to find ansi green"))?,
      blue: color::closest_by_hue(*BLUE, extracted256.iter().cloned())
        .ok_or_else(|| anyhow::anyhow!("Failed to find ansi blue"))?,
      cyan: color::closest_by_hue(*CYAN, extracted256.iter().cloned())
        .ok_or_else(|| anyhow::anyhow!("Failed to find ansi cyan"))?,
      yellow: color::closest_by_hue(*YELLOW, extracted256.iter().cloned())
        .ok_or_else(|| anyhow::anyhow!("Failed to find ansi yellow"))?,
      magenta: color::closest_by_hue(*MAGENTA, extracted256.iter().cloned())
        .ok_or_else(|| anyhow::anyhow!("Failed to find ansi magenta"))?,
      white: color::closest_by_hue(*WHITE, extracted256.iter().cloned())
        .ok_or_else(|| anyhow::anyhow!("Failed to find ansi white"))?,
      bright_black: color::closest_by_hue(
        *BRIGHT_BLACK,
        extracted256.iter().cloned(),
      )
      .ok_or_else(|| anyhow::anyhow!("Failed to find ansi bright_black"))?,
      bright_red: color::closest_by_hue(
        *BRIGHT_RED,
        extracted256.iter().cloned(),
      )
      .ok_or_else(|| anyhow::anyhow!("Failed to find ansi bright_red"))?,
      bright_green: color::closest_by_hue(
        *BRIGHT_GREEN,
        extracted256.iter().cloned(),
      )
      .ok_or_else(|| anyhow::anyhow!("Failed to find ansi bright_green"))?,
      bright_blue: color::closest_by_hue(
        *BRIGHT_BLUE,
        extracted256.iter().cloned(),
      )
      .ok_or_else(|| anyhow::anyhow!("Failed to find ansi bright_blue"))?,
      bright_cyan: color::closest_by_hue(
        *BRIGHT_CYAN,
        extracted256.iter().cloned(),
      )
      .ok_or_else(|| anyhow::anyhow!("Failed to find ansi bright_cyan"))?,
      bright_yellow: color::closest_by_hue(
        *BRIGHT_YELLOW,
        extracted256.iter().cloned(),
      )
      .ok_or_else(|| anyhow::anyhow!("Failed to find ansi bright_yellow"))?,
      bright_magenta: color::closest_by_hue(
        *BRIGHT_MAGENTA,
        extracted256.iter().cloned(),
      )
      .ok_or_else(|| anyhow::anyhow!("Failed to find ansi bright_magenta"))?,
      bright_white: color::closest_by_hue(
        *BRIGHT_WHITE,
        extracted256.iter().cloned(),
      )
      .ok_or_else(|| anyhow::anyhow!("Failed to find ansi bright_white"))?,
      gradient: (*GRADIENT)
        .iter()
        .map(|color| {
          color::closest_by_hue(*color, extracted256.iter().cloned())
        })
        .flatten()
        .collect::<Vec<_>>(),
      grayscale: (*GRAYSCALE)
        .iter()
        .map(|color| {
          color::closest_by_hue(*color, extracted256.iter().cloned())
        })
        .flatten()
        .collect::<Vec<_>>(),
    }
    .correct_contrast(
      |x, y| color::correct_text_contrast(x, y).to_rgba(),
      |x, y| color::correct_graphics_contrast(x, y).to_rgba(),
    );

    super::trace_colors!(colors);

    Ok(colors)
  }
}

impl Palette {
  fn correct_contrast<
    TCorrectText: FnMut(Rgba, Rgba) -> Rgba,
    TCorrectElement: FnMut(Rgba, Rgba) -> Rgba,
  >(
    self,
    mut correct_text: TCorrectText,
    mut correct_graphics: TCorrectElement,
  ) -> Self {
    Self {
      black: self.black,
      red: correct_graphics(self.black, self.red),
      green: correct_graphics(self.black, self.green),
      blue: correct_graphics(self.black, self.blue),
      cyan: correct_graphics(self.black, self.cyan),
      yellow: correct_graphics(self.black, self.yellow),
      magenta: correct_graphics(self.black, self.magenta),
      white: correct_text(self.black, self.white),
      bright_black: correct_graphics(self.black, self.bright_black),
      bright_red: correct_graphics(self.black, self.bright_red),
      bright_green: correct_graphics(self.black, self.bright_green),
      bright_blue: correct_graphics(self.black, self.bright_blue),
      bright_cyan: correct_graphics(self.black, self.bright_cyan),
      bright_yellow: correct_graphics(self.black, self.bright_yellow),
      bright_magenta: correct_graphics(self.black, self.bright_magenta),
      bright_white: correct_text(self.black, self.bright_white),
      gradient: self
        .gradient
        .into_iter()
        .map(|gradient| correct_graphics(self.black, gradient))
        .collect::<Vec<_>>(),
      grayscale: self
        .grayscale
        .into_iter()
        .map(|gradient| correct_graphics(self.black, gradient))
        .collect::<Vec<_>>(),
    }
  }
}

lazy_static! {
  static ref EMPTY: FloatingComponent =
    FloatingComponent::min_component_value();
  static ref FULL: FloatingComponent = FloatingComponent::max_component_value();
  static ref PART32: FloatingComponent =
    *FULL / FloatingComponent::from_f32(32f32);
  static ref SIXTH: FloatingComponent =
    *FULL / FloatingComponent::from_f32(6f32);
  static ref THIRD: FloatingComponent =
    *FULL / FloatingComponent::from_f32(3f32);
  static ref HALF: FloatingComponent =
    *FULL / FloatingComponent::from_f32(2f32);
}

lazy_static! {
  static ref BLACK: Rgba = Rgba::opaque(*EMPTY, *EMPTY, *EMPTY);
  static ref RED: Rgba = Rgba::opaque(*HALF, *EMPTY, *EMPTY);
  static ref GREEN: Rgba = Rgba::opaque(*EMPTY, *HALF, *EMPTY);
  static ref BLUE: Rgba = Rgba::opaque(*EMPTY, *EMPTY, *HALF);
  static ref CYAN: Rgba = Rgba::opaque(*EMPTY, *HALF, *HALF);
  static ref YELLOW: Rgba = Rgba::opaque(*HALF, *HALF, *EMPTY);
  static ref MAGENTA: Rgba = Rgba::opaque(*HALF, *EMPTY, *HALF);
  static ref WHITE: Rgba = Rgba::opaque(
    FloatingComponent::from_f32(2f32) * *THIRD,
    FloatingComponent::from_f32(2f32) * *THIRD,
    FloatingComponent::from_f32(2f32) * *THIRD
  );
  static ref BRIGHT_BLACK: Rgba = Rgba::opaque(*THIRD, *THIRD, *THIRD);
  static ref BRIGHT_RED: Rgba = Rgba::opaque(*FULL, *EMPTY, *EMPTY);
  static ref BRIGHT_GREEN: Rgba = Rgba::opaque(*EMPTY, *FULL, *EMPTY);
  static ref BRIGHT_BLUE: Rgba = Rgba::opaque(*EMPTY, *EMPTY, *FULL);
  static ref BRIGHT_CYAN: Rgba = Rgba::opaque(*EMPTY, *FULL, *FULL);
  static ref BRIGHT_YELLOW: Rgba = Rgba::opaque(*FULL, *FULL, *EMPTY);
  static ref BRIGHT_MAGENTA: Rgba = Rgba::opaque(*FULL, *EMPTY, *FULL);
  static ref BRIGHT_WHITE: Rgba = Rgba::opaque(*FULL, *FULL, *FULL);
}

lazy_static! {
  static ref GRADIENT: Vec<Rgba> = (0..6)
    .flat_map(
      move |r| (0..6).map(move |g| (0..6).map(move |b| Rgba::opaque(
        r as FloatingComponent * *SIXTH,
        g as FloatingComponent * *SIXTH,
        b as FloatingComponent * *SIXTH
      )))
    )
    .flatten()
    .collect();
}

lazy_static! {
  static ref GRAYSCALE: Vec<Rgba> = (0..32)
    .map(|i| Rgba::opaque(
      i as FloatingComponent * *PART32,
      i as FloatingComponent * *PART32,
      i as FloatingComponent * *PART32
    ))
    .collect();
}
