use rayon::prelude::{ParallelBridge, ParallelIterator};

use crate::*;

#[derive(Debug, Clone)]
pub struct Scolorq {
  config: config::ScolorqConfig,
  width: usize,
  height: usize,
  pixels: Vec<rscolorq::color::Rgb>,
}

impl Scolorq {
  pub fn new(
    config: config::ScolorqConfig,
    path: String,
  ) -> anyhow::Result<Self> {
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
      .map(|image::Rgb([red, green, blue])| rscolorq::color::Rgb {
        red: (*red).into(),
        green: (*green).into(),
        blue: (*blue).into(),
      })
      .collect::<Vec<_>>();

    Ok(Self {
      config,
      width,
      height,
      pixels,
    })
  }
}

#[async_trait::async_trait]
impl super::Extractor for Scolorq {
  #[tracing::instrument(skip(self))]
  async fn prominent(&self, count: u8) -> anyhow::Result<Vec<color::Rgba>> {
    let image = rscolorq::Matrix2d::from_vec(
      self.pixels.clone(),
      self.width,
      self.height,
    );
    let mut quantized = rscolorq::Matrix2d::<_>::new(self.width, self.height);
    let mut result = Vec::<_>::with_capacity(count.into());
    let mut params = rscolorq::Params::<_>::new();

    params.palette_size(count);
    params.initial_temp(self.config.start_temp);
    params.final_temp(self.config.end_temp);
    params.iters_per_level(self.config.iters);
    params.repeats_per_temp(self.config.repeats);
    params.seed(self.config.seed);
    params.filter_size(match self.config.filter {
      config::ScolorqConfigFilter::One => rscolorq::FilterSize::One,
      config::ScolorqConfigFilter::Three => rscolorq::FilterSize::Three,
      config::ScolorqConfigFilter::Five => rscolorq::FilterSize::Five,
    });
    match self.config.dither {
      Some(level) => params.dithering_level(level),
      None => params.dithering_level_auto(
        self.width.try_into()?,
        self.height.try_into()?,
        count.into(),
      ),
    };
    params.verify_parameters()?;

    rscolorq::spatial_color_quant(
      &image,
      &mut quantized,
      &mut result,
      &params,
    )?;

    let colors = result
      .iter()
      .map(|rscolorq::color::Rgb { red, green, blue }| color::Rgba {
        red: (*red) as color::FloatingComponent,
        green: (*green) as color::FloatingComponent,
        blue: (*blue) as color::FloatingComponent,
        alpha: 1.0f32,
      })
      .collect::<Vec<_>>();

    super::trace_colors!(colors);

    Ok(colors)
  }
}
