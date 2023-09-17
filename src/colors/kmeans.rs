use palette::IntoColor;
use rayon::prelude::*;

// TODO: async image load

#[derive(Debug, Clone)]
pub struct KmeansConfig {
  pub runs: u64,
  pub k: usize,
  pub converge: f32,
  pub max_iter: usize,
}

#[tracing::instrument]
pub async fn prominent(
  path: String,
  kmeans_config: KmeansConfig,
) -> anyhow::Result<super::prominent::Colors> {
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
    let kmeans = (0..kmeans_config.runs)
      .into_par_iter()
      .map(|i| {
        let kmeans = kmeans_colors::get_kmeans_hamerly(
          kmeans_config.k,
          kmeans_config.max_iter,
          kmeans_config.converge,
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

    Ok(super::prominent::Colors {
      means: kmeans
        .centroids
        .iter()
        .map(|lab| {
          let float = IntoColor::<
            palette::rgb::Rgba<palette::encoding::Srgb, f32>,
          >::into_color(*lab);
          let palette::rgb::Rgba::<palette::encoding::Srgb, u8> {
            color:
              palette::Srgb::<u8> {
                red, green, blue, ..
              },
            alpha,
            ..
          } = float.into_format::<u8, u8>();
          super::prominent::Rgba {
            red,
            green,
            blue,
            alpha,
          }
        })
        .collect(),
    })
  })
  .await?
}
