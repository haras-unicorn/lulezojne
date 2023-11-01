use palette::IntoColor;
use rayon::prelude::*;

use crate::*;

#[derive(Debug, Clone)]
pub struct Kmeans {
  config: config::KmeansConfig,
  pixels: Vec<palette::Lab>,
}

impl Kmeans {
  pub fn new(
    config: config::KmeansConfig,
    path: String,
  ) -> anyhow::Result<Self> {
    let pixels = image::io::Reader::open(path)?
      .decode()?
      .to_rgb32f()
      .pixels()
      .par_bridge()
      .map(|image::Rgb([r, g, b])| {
        palette::Srgb::from_components((*r, *g, *b))
          .into_format()
          .into_color()
      })
      .collect::<Vec<palette::Lab>>();

    Ok(Self { config, pixels })
  }
}

#[async_trait::async_trait]
impl super::Extractor for Kmeans {
  #[tracing::instrument(skip(self))]
  async fn prominent(&self, count: u8) -> anyhow::Result<Vec<color::Rgba>> {
    let seed = rand::random::<u64>();
    let result = (0..self.config.runs)
      .into_par_iter()
      .map(|i| {
        let kmeans = kmeans_colors::get_kmeans_hamerly(
          count.into(),
          self.config.max_iter,
          self.config.converge,
          false,
          &self.pixels,
          seed + i,
        );

        tracing::debug! {
          "Kmeans {} scored {}",
          i,
          kmeans.score
        };

        kmeans
      })
      .min_by(|x, y| {
        x.score
          .partial_cmp(&y.score)
          .unwrap_or(std::cmp::Ordering::Equal)
      })
      .unwrap_or_default();

    let colors = result
      .centroids
      .iter()
      .map(|lab| {
        let intermediary = IntoColor::<palette::rgb::Srgba>::into_color(*lab);
        color::Rgba {
          red: intermediary.red,
          green: intermediary.green,
          blue: intermediary.blue,
          alpha: intermediary.alpha,
        }
      })
      .collect::<Vec<_>>();

    super::trace_colors!(colors);

    Ok(colors)
  }
}
