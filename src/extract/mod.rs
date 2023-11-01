mod colorthief;
mod kmeans;
mod kmeans_gpu;
mod median_cut;
mod neoquant;
mod scolorq;

use crate::*;

macro_rules! trace_colors {
  ($colors: ident) => {
    tracing::debug! {
      "Extraced {} colors {}",
      $colors.len(),
      $colors.iter().fold(
        String::new(),
        |acc, color| acc + "\n" + color::Color::to_colored_string(*color).as_str(),
      ) + "\n"
    };
  };
}

pub(crate) use trace_colors;

#[async_trait::async_trait]
pub trait Extractor {
  async fn prominent(&self, count: u8) -> anyhow::Result<Vec<color::Rgba>>;
}

// TODO: async image load

pub async fn new(
  kind: args::Extractor,
  config: config::Config,
  path: String,
) -> anyhow::Result<Box<dyn Extractor + Send + Sync>> {
  Ok(match kind {
    args::Extractor::Colorthief => {
      Box::new(colorthief::Colorthief::new(config.colorthief, path)?)
    }
    args::Extractor::Kmeans => {
      Box::new(kmeans::Kmeans::new(config.kmeans, path)?)
    }
    args::Extractor::KmeansGpu => {
      Box::new(kmeans_gpu::KmeansGpu::new(config.kmeans_gpu, path)?)
    }
    args::Extractor::MedianCut => {
      Box::new(median_cut::MedianCut::new(config.median_cut, path)?)
    }
    args::Extractor::Neoquant => {
      Box::new(neoquant::Neoquant::new(config.neoquant, path)?)
    }
    args::Extractor::Scolorq => {
      Box::new(scolorq::Scolorq::new(config.scolorq, path)?)
    }
  })
}
