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

  Ok(super::Colors {
    means: palette
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
      .collect(),
  })
}
