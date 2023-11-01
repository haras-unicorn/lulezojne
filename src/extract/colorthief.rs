use rayon::prelude::*;

use crate::{color::Component, *};

#[derive(Debug, Clone)]
pub struct Colorthief {
  config: config::ColorthiefConfig,
  bytes: Vec<u8>,
}

impl Colorthief {
  pub fn new(
    config: config::ColorthiefConfig,
    path: String,
  ) -> anyhow::Result<Self> {
    let bytes = image::io::Reader::open(path)?
      .decode()?
      .to_rgb8()
      .par_iter()
      .map(|byte| *byte)
      .collect::<Vec<u8>>();

    Ok(Self { bytes, config })
  }
}

#[async_trait::async_trait]
impl super::Extractor for Colorthief {
  #[tracing::instrument(skip(self))]
  async fn prominent(&self, count: u8) -> anyhow::Result<Vec<color::Rgba>> {
    let mut result = color_thief::get_palette(
      &self.bytes,
      color_thief::ColorFormat::Rgb,
      self.config.quality,
      count,
    )?;

    let colors = result
      .drain(0..)
      .map(|color_thief::Color { r, g, b }| color::Rgba {
        red: r.to_floating_component(),
        green: g.to_floating_component(),
        blue: b.to_floating_component(),
        alpha: 1.0,
      })
      .collect::<Vec<_>>();

    super::trace_colors!(colors);

    Ok(colors)
  }
}
