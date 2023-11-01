use crate::{color::Component, *};

#[derive(Debug, Clone)]
pub struct KmeansGpu {
  config: config::KmeansGpuConfig,
  dimensions: (u32, u32),
  pixels: Vec<u8>,
}

impl KmeansGpu {
  pub fn new(
    config: config::KmeansGpuConfig,
    path: String,
  ) -> anyhow::Result<Self> {
    let dimensions =
      image::io::Reader::open(path.clone())?.into_dimensions()?;
    let pixels = image::io::Reader::open(path.clone())?
      .decode()?
      .into_bytes();

    Ok(Self {
      config,
      dimensions,
      pixels,
    })
  }
}

#[async_trait::async_trait]
impl super::Extractor for KmeansGpu {
  #[tracing::instrument(skip(self))]
  async fn prominent(&self, count: u8) -> anyhow::Result<Vec<color::Rgba>> {
    let image = kmeans_color_gpu::image::borrowed_pixel(
      self.dimensions,
      self.pixels.as_slice(),
    );

    let image_processor = kmeans_color_gpu::ImageProcessor::new().await?;

    let result = image_processor
      .palette(
        count.into(),
        &image,
        match self.config.algorithm {
          config::KmeansGpuAlgorithm::Kmeans => {
            kmeans_color_gpu::Algorithm::Kmeans
          }
          config::KmeansGpuAlgorithm::Octree => {
            kmeans_color_gpu::Algorithm::Octree
          }
        },
      )
      .await?;

    let colors = result
      .into_iter()
      .map(|rgba| color::Rgba {
        red: rgba.r.to_floating_component(),
        green: rgba.g.to_floating_component(),
        blue: rgba.b.to_floating_component(),
        alpha: rgba.a.to_floating_component(),
      })
      .collect::<Vec<_>>();

    super::trace_colors!(colors);

    Ok(colors)
  }
}
