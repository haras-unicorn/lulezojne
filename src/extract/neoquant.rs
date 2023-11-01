use rayon::prelude::{ParallelBridge, ParallelIterator};

use crate::{color::Component, *};

#[derive(Debug, Clone)]
pub struct Neoquant {
  config: config::NeoquantConfig,
  pixels: Vec<u8>,
}

impl Neoquant {
  pub fn new(
    config: config::NeoquantConfig,
    path: String,
  ) -> anyhow::Result<Self> {
    let pixels = image::io::Reader::open(path)?
      .decode()?
      .to_rgba8()
      .pixels()
      .par_bridge()
      .map(|image::Rgba([r, g, b, a])| vec![*r, *g, *b, *a])
      .flatten()
      .collect::<Vec<_>>();

    Ok(Self { config, pixels })
  }
}

#[async_trait::async_trait]
impl super::Extractor for Neoquant {
  #[tracing::instrument(skip(self))]
  async fn prominent(&self, count: u8) -> anyhow::Result<Vec<color::Rgba>> {
    let result = color_quant::NeuQuant::new(
      self.config.sample_faction,
      count.into(),
      &self.pixels,
    );

    let colors = result
      .color_map_rgba()
      .chunks(4)
      .filter_map(|pixel| match pixel {
        [red, green, blue, alpha] => Some(color::Rgba {
          red: (*red).to_floating_component(),
          green: (*green).to_floating_component(),
          blue: (*blue).to_floating_component(),
          alpha: Into::<f32>::into(*alpha) / 255.0f32,
        }),
        _ => None,
      })
      .collect::<Vec<_>>();

    super::trace_colors!(colors);

    Ok(colors)
  }
}
