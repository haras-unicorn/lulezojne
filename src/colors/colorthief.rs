use rayon::prelude::*;

pub struct ColorthiefConfig {
  pub quality: u8,
  pub max_colors: u8,
}

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

  let palette = color_thief::get_palette(
    &bytes,
    color_thief::ColorFormat::Rgb,
    config.quality,
    config.max_colors,
  )?;

  Ok(super::prominent::Colors {
    means: palette
      .iter()
      .map(|color_thief::Color { r, g, b }| {
        palette::Srgb::from_components((*r, *g, *b))
      })
      .collect(),
  })
}
