use colored::Colorize;
use rayon::prelude::{ParallelBridge, ParallelIterator};

#[derive(Debug, Clone)]
pub struct NeoquantConfig {
  pub sample_faction: i32,
  pub colors: usize,
}

// TODO: async image load

#[tracing::instrument]
pub async fn prominent(
  path: String,
  config: NeoquantConfig,
) -> anyhow::Result<super::Colors> {
  tokio::spawn(async move {
    let pixels = image::io::Reader::open(path)?
      .decode()?
      .to_rgba8()
      .pixels()
      .par_bridge()
      .map(|image::Rgba([r, g, b, a])| vec![*r, *g, *b, *a])
      .flatten()
      .collect::<Vec<_>>();

    let nq =
      color_quant::NeuQuant::new(config.sample_faction, config.colors, &pixels);
    let palette = nq
      .color_map_rgba()
      .chunks(4)
      .map(|pixel| match pixel {
        [red, green, blue, alpha] => super::Rgba {
          red: *red,
          green: *green,
          blue: *blue,
          alpha: Into::<f32>::into(*alpha) / 255.0f32,
        },
        _ => Default::default(),
      })
      .collect::<Vec<_>>();
    tracing::debug! {
      "Generated palette of {} colors {}",
      palette.len(),
      palette.iter().fold(
        String::new(),
        |acc, super::Rgba { red, green, blue, alpha }| {
          acc
            + format!("\nrgba({red}, {green}, {blue}, {alpha})")
              .truecolor(*red, *green, *blue)
              .to_string()
              .as_str()
        },
      ) + "\n"
    };

    Ok(super::Colors { palette })
  })
  .await?
}
