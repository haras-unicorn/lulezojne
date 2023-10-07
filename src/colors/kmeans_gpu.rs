#[derive(Debug, Clone)]
pub struct KmeansGpuConfig {
  pub count: u32,
  pub algorithm: KmeansGpuAlgorithm,
}

#[derive(Debug, Clone)]
pub enum KmeansGpuAlgorithm {
  Kmeans,
  Octree,
}

// TODO: async image load
// TODO: https://github.com/redwarp/kmeans-gpu

#[tracing::instrument]
pub async fn prominent(
  path: String,
  config: KmeansGpuConfig,
) -> anyhow::Result<super::Colors> {
  tokio::spawn(async move {
    let image_processor = kmeans_color_gpu::ImageProcessor::new().await?;

    let dimensions =
      image::io::Reader::open(path.clone())?.into_dimensions()?;
    let pixels = image::io::Reader::open(path.clone())?
      .decode()?
      .into_bytes();
    let image =
      kmeans_color_gpu::image::borrowed_pixel(dimensions, pixels.as_slice());

    let palette = image_processor
      .palette(
        config.count,
        &image,
        match config.algorithm {
          KmeansGpuAlgorithm::Kmeans => kmeans_color_gpu::Algorithm::Kmeans,
          KmeansGpuAlgorithm::Octree => kmeans_color_gpu::Algorithm::Octree,
        },
      )
      .await?;

    Ok(super::Colors {
      palette: palette
        .iter()
        .map(|rgba| super::Rgba {
          red: rgba.r,
          green: rgba.g,
          blue: rgba.b,
          alpha: (rgba.a as f32) / 255.0f32,
        })
        .collect(),
    })
  })
  .await?
}
