use palette::IntoColor;
use rayon::prelude::*;

// TODO: async image load
// TODO: https://github.com/redwarp/kmeans-gpu

#[derive(Debug, Clone)]
pub struct KmeansGpuConfig {
  pub runs: u64,
  pub k: usize,
  pub converge: f32,
  pub max_iter: usize,
}

#[tracing::instrument]
pub async fn prominent(
  path: String,
  config: KmeansGpuConfig,
) -> anyhow::Result<super::Colors> {
  tokio::spawn(async move {
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

    let seed = rand::random::<u64>();
    let kmeans = (0..config.runs)
      .into_par_iter()
      .map(|i| {
        let kmeans = kmeans_colors::get_kmeans_hamerly(
          config.k,
          config.max_iter,
          config.converge,
          false,
          &pixels,
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

    Ok(super::Colors {
      means: kmeans
        .centroids
        .iter()
        .map(|lab| {
          let intermediary =
            IntoColor::<palette::rgb::Srgba<f32>>::into_color(*lab)
              .into_linear::<f32, f32>()
              .into_format::<u8, f32>();
          super::Rgba {
            red: intermediary.red,
            green: intermediary.green,
            blue: intermediary.blue,
            alpha: intermediary.alpha,
          }
        })
        .collect(),
    })
  })
  .await?
}
