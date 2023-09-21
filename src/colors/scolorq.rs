use colored::Colorize;
use rayon::prelude::{ParallelBridge, ParallelIterator};

#[derive(Debug, Clone)]
pub struct ScolorqConfig {
  pub size: u8,
  pub dither: Option<f64>,
  pub seed: Option<u64>,
  pub filter: ScolorqConfigFilter,
  pub iters: usize,
  pub repeats: usize,
  pub start_temp: f64,
  pub end_temp: f64,
}

#[derive(Debug, Clone)]
pub enum ScolorqConfigFilter {
  One,
  Three,
  Five,
}

type Color = rscolorq::color::Rgb;

// TODO: async image load

#[tracing::instrument]
pub async fn prominent(
  path: String,
  config: ScolorqConfig,
) -> anyhow::Result<super::Colors> {
  tokio::spawn(async move {
    let (width, height) = {
      let (width, height) =
        image::io::Reader::open(path.clone())?.into_dimensions()?;
      (width as usize, height as usize)
    };
    let pixels = image::io::Reader::open(path)?
      .decode()?
      .to_rgb32f()
      .pixels()
      .par_bridge()
      .map(|image::Rgb([red, green, blue])| Color {
        red: (*red).into(),
        green: (*green).into(),
        blue: (*blue).into(),
      })
      .collect::<Vec<Color>>();

    let image = rscolorq::Matrix2d::from_vec(pixels, width, height);
    let mut quantized = rscolorq::Matrix2d::<u8>::new(width, height);
    let mut palette = Vec::<Color>::with_capacity(config.size.into());
    let mut params = rscolorq::Params::<Color>::new();

    params.palette_size(config.size);
    params.initial_temp(config.start_temp);
    params.final_temp(config.end_temp);
    params.iters_per_level(config.iters);
    params.repeats_per_temp(config.repeats);
    params.seed(config.seed);
    params.filter_size(match config.filter {
      ScolorqConfigFilter::One => rscolorq::FilterSize::One,
      ScolorqConfigFilter::Three => rscolorq::FilterSize::Three,
      ScolorqConfigFilter::Five => rscolorq::FilterSize::Five,
    });
    match config.dither {
      Some(level) => params.dithering_level(level),
      None => params.dithering_level_auto(
        width.try_into()?,
        height.try_into()?,
        config.size.into(),
      ),
    };
    params.verify_parameters()?;

    rscolorq::spatial_color_quant(
      &image,
      &mut quantized,
      &mut palette,
      &params,
    )?;

    let palette = palette
      .iter()
      .map(|Color { red, green, blue }| super::Rgba {
        red: (*red * 255.0f64).round() as u8,
        green: (*green * 255.0f64).round() as u8,
        blue: (*blue * 255.0f64).round() as u8,
        alpha: 1.0f32,
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
