use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::{
  color::{Color, Component, FloatingComponent, Rgba},
  *,
};

#[derive(Clone)]
pub struct Extrapolator {
  #[allow(unused)]
  config: config::BootstrapConfig,
  extractor: Arc<Mutex<Box<dyn extract::Extractor + Send + Sync>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Palette {
  pub background: Rgba,
  pub foreground: Rgba,
  pub primary: Rgba,
  pub secondary: Rgba,
  pub ternary: Rgba,
  pub accent: Rgba,
  pub debug: Rgba,
  pub info: Rgba,
  pub warning: Rgba,
  pub error: Rgba,
}

impl Extrapolator {
  pub fn new(
    config: config::BootstrapConfig,
    extractor: Arc<Mutex<Box<dyn extract::Extractor + Send + Sync>>>,
  ) -> Self {
    Self { config, extractor }
  }
}

#[async_trait::async_trait]
impl<'a> super::Extrapolator<'a, Palette> for Extrapolator {
  #[tracing::instrument(skip(self))]
  async fn extrapolate(&self) -> anyhow::Result<Palette> {
    let extracted16 = {
      let extractor = self.extractor.clone().lock_owned().await;
      extractor.prominent(16).await?
    };
    let extracted4 = {
      let extractor = self.extractor.clone().lock_owned().await;
      extractor.prominent(4).await?
    };

    let _metrics: color::Metrics<FloatingComponent> =
      color::analyze(extracted16.iter().cloned());

    let background = color::darkest(extracted16.iter().cloned())
      .ok_or_else(|| anyhow::anyhow!("Failed finding background"))?;

    let foreground = color::brightest(extracted16.iter().cloned())
      .ok_or_else(|| anyhow::anyhow!("Failed finding foreground"))?;

    let primary = extracted4[0];
    let secondary = extracted4[1];
    let ternary = extracted4[2];
    let accent = extracted4[3];

    let debug = color::closest_by_hue(extracted16.iter().cloned(), *CYAN)
      .ok_or(anyhow::anyhow!("Couldn't find debug color"))?;
    let info = color::closest_by_hue(extracted16.iter().cloned(), *GREEN)
      .ok_or(anyhow::anyhow!("Couldn't find info color"))?;
    let warning = color::closest_by_hue(extracted16.iter().cloned(), *YELLOW)
      .ok_or(anyhow::anyhow!("Couldn't find warning color"))?;
    let error = color::closest_by_hue(extracted16.iter().cloned(), *RED)
      .ok_or(anyhow::anyhow!("Couldn't find error color"))?;

    let colors = Palette {
      background,
      foreground,
      primary,
      secondary,
      ternary,
      accent,
      debug,
      info,
      warning,
      error,
    }
    .correct_contrast(
      |x, y| color::correct_text_foreground_contrast(x, y).to_rgba(),
      |x, y| color::correct_element_foreground_contrast(x, y).to_rgba(),
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
    mut correct_element: TCorrectElement,
  ) -> Self {
    Self {
      background: self.background,
      foreground: correct_text(self.background, self.foreground),
      primary: correct_element(self.background, self.primary),
      secondary: correct_element(self.background, self.secondary),
      ternary: correct_element(self.background, self.ternary),
      accent: correct_element(self.background, self.accent),
      debug: correct_text(self.background, self.debug),
      info: correct_text(self.background, self.info),
      warning: correct_text(self.background, self.warning),
      error: correct_text(self.background, self.error),
    }
  }
}

lazy_static::lazy_static! {
  static ref EMPTY: FloatingComponent = FloatingComponent::max_component_value();
  static ref FULL: FloatingComponent = FloatingComponent::min_component_value();
  static ref HALF: FloatingComponent = *FULL / FloatingComponent::from_f32(2f32);
}

lazy_static::lazy_static! {
  static ref GREEN: Rgba = Rgba::opaque(*EMPTY, *HALF, *EMPTY);
  static ref CYAN: Rgba = Rgba::opaque(*EMPTY, *HALF, *HALF);
  static ref YELLOW: Rgba = Rgba::opaque(*HALF, *HALF, *EMPTY);
  static ref RED: Rgba = Rgba::opaque(*HALF, *EMPTY, *EMPTY);
}
