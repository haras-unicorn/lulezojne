#![deny(
    unsafe_code,
    // reason = "Let's just not do it"
  )]
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    // reason = "We have to handle errors properly"
  )]

use colored::Colorize;

mod args;
mod colors;
mod config;

#[tokio::main]
#[tracing::instrument]
async fn main() -> anyhow::Result<()> {
  tracing::subscriber::set_global_default(
    tracing_subscriber::FmtSubscriber::builder().finish(),
  )?;

  let args = args::parse()?;
  let generation = match args {
    args::Args::Plop { generation, .. } => generation,
    args::Args::Print { generation, .. } => generation,
  };

  let config = config::read().await?;
  let kmeans_config = colors::kmeans::KmeansConfig {
    runs: config.kmeans.runs,
    k: config.kmeans.k,
    converge: config.kmeans.converge,
    max_iter: config.kmeans.max_iter,
  };
  let colorthief_config = colors::colorthief::ColorthiefConfig {
    quality: config.colorthief.quality,
    max_colors: config.colorthief.max_colors,
  };

  let colors = match generation.backend {
    args::Backend::Kmeans => {
      colors::kmeans::prominent(generation.image, kmeans_config).await?
    }
    args::Backend::Colorthief => {
      colors::colorthief::prominent(generation.image, colorthief_config).await?
    }
  };

  for color in colors.means {
    let red = color.red;
    let green = color.green;
    let blue = color.blue;
    println!(
      "{}",
      format!("rgb({red}, {green}, {blue})").truecolor(red, green, blue)
    )
  }

  Ok(())
}
