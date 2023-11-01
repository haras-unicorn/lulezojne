use rayon::prelude::{ParallelBridge, ParallelIterator};

use crate::{color::Component, *};

#[derive(Debug, Clone)]
pub struct MedianCut {
  #[allow(dead_code)]
  config: config::MedianCutConfig,
  pixels: Vec<iris_lib::color::Color>,
}

impl MedianCut {
  pub fn new(
    config: config::MedianCutConfig,
    path: String,
  ) -> anyhow::Result<Self> {
    let pixels = image::io::Reader::open(path)?
      .decode()?
      .to_rgb8()
      .pixels()
      .par_bridge()
      .map(|image::Rgb([red, green, blue])| iris_lib::color::Color {
        r: *red,
        g: *green,
        b: *blue,
        a: 255u8,
      })
      .collect::<Vec<_>>();

    Ok(Self { config, pixels })
  }
}

#[async_trait::async_trait]
impl super::Extractor for MedianCut {
  #[tracing::instrument(skip(self))]
  async fn prominent(&self, count: u8) -> anyhow::Result<Vec<color::Rgba>> {
    let iterations = (count as f32).sqrt().ceil() as u8;

    let mut bucket =
      iris_lib::color_bucket::ColorBucket::from_pixels(self.pixels.clone())
        .ok_or_else(|| anyhow::anyhow!("Empty image"))?;
    let result = bucket.make_palette(iterations);

    let colors = result
      .iter()
      .map(|iris_lib::color::Color { r, g, b, a }| color::Rgba {
        red: (*r).to_floating_component(),
        green: (*g).to_floating_component(),
        blue: (*b).to_floating_component(),
        alpha: (*a).to_floating_component(),
      })
      .collect::<Vec<_>>();

    super::trace_colors!(colors);

    Ok(colors)
  }
}
