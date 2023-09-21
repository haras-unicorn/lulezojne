use colored::Colorize;

#[derive(Debug, Clone)]
pub struct MedianCutConfig {
  pub iterations: u8,
}

// TODO: async image load

#[tracing::instrument]
pub async fn prominent(
  path: String,
  config: MedianCutConfig,
) -> anyhow::Result<super::Colors> {
  tokio::spawn(async move {
    let bucket = iris_lib::color_bucket::ColorBucket::from_image(path.as_str());
    match bucket {
      None => Err(anyhow::anyhow!(format!(
        "Failed creating buckets from {path}"
      ))),
      Some(mut buckets) => {
        let palette = buckets.make_palette(config.iterations);
        tracing::debug! {
          "Generated palette of {} colors {}",
          palette.len(),
          palette.iter().fold(
            String::new(),
            |acc, iris_lib::color::Color { r, g, b, a }| {
              acc
                + format!("\nrgba({r}, {g}, {b}, {a})")
                  .truecolor(*r, *g, *b)
                  .to_string()
                  .as_str()
            },
          ) + "\n"
        };

        Ok(super::Colors {
          palette: palette
            .iter()
            .map(|iris_lib::color::Color { r, g, b, a }| super::Rgba {
              red: *r,
              green: *g,
              blue: *b,
              alpha: (Into::<f32>::into(*a) / 255.0f32),
            })
            .collect(),
        })
      }
    }
  })
  .await?
}
