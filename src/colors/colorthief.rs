use rayon::prelude::*;

// TODO: async image load

#[derive(Debug, Clone)]
pub struct ColorthiefConfig {
  pub quality: u8,
  pub max_colors: u8,
}

#[tracing::instrument]
pub async fn prominent(
  path: String,
  config: ColorthiefConfig,
) -> anyhow::Result<super::prominent::Colors> {
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

  Ok(super::prominent::Colors {
    means: palette
      .drain(0..)
      .map(
        |color_thief::Color {
           r: red,
           g: green,
           b: blue,
         }| {
          super::prominent::Rgba {
            red,
            green,
            blue,
            alpha: 255,
          }
        },
      )
      .collect(),
  })
}
