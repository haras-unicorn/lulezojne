use colored::Colorize;
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct ColorthiefConfig {
  pub quality: u8,
  pub max_colors: u8,
}

// TODO: async image load

#[tracing::instrument]
pub async fn prominent(
  path: String,
  config: ColorthiefConfig,
) -> anyhow::Result<super::Colors> {
  let bytes = image::io::Reader::open(path)?
    .decode()?
    .to_rgb8()
    .par_iter()
    .map(|byte| *byte)
    .collect::<Vec<u8>>();

  let mut palette = color_thief::get_palette(
    &bytes,
    color_thief::ColorFormat::Rgb,
    config.quality,
    config.max_colors,
  )?;

  let palette = palette
    .drain(0..)
    .map(
      |color_thief::Color {
         r: red,
         g: green,
         b: blue,
       }| {
        super::Rgba {
          red,
          green,
          blue,
          alpha: 1.0,
        }
      },
    )
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
}
