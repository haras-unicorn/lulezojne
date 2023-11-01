#![deny(
    unsafe_code,
    // reason = "Let's just not do it"
  )]
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unreachable,
    // reason = "We have to handle errors properly"
  )]
#![deny(
  clippy::dbg_macro,
  // reason = "Use tracing instead"
)]

use std::io::Write;

use color::Color;

mod args;
mod color;
mod config;
mod extract;
mod extrapolate;
mod plop;

#[tokio::main]
#[tracing::instrument]
async fn main() -> anyhow::Result<()> {
  tracing::subscriber::set_global_default({
    let mut builder = tracing_subscriber::FmtSubscriber::builder();
    #[cfg(debug_assertions)]
    {
      builder =
        builder.with_max_level(tracing::level_filters::LevelFilter::DEBUG);
    }
    #[cfg(not(debug_assertions))]
    {
      builder =
        builder.with_max_level(tracing::level_filters::LevelFilter::WARN);
    }
    builder.finish()
  })?;

  let args = args::parse();
  let config_location = match &args {
    args::Args::Plop { config, .. } => config.location.clone(),
    args::Args::Print { config, .. } => config.location.clone(),
  };
  let config = config::read(config_location).await?;

  let extraction = match &args {
    args::Args::Plop { extraction, .. } => extraction.clone(),
    args::Args::Print { extraction, .. } => extraction.clone(),
  };

  let extractor =
    extract::new(extraction.extractor, config.clone(), extraction.image)
      .await?;
  let colors = extrapolate::extrapolate(config.clone(), extractor).await?;

  match args {
    args::Args::Plop { .. } => {
      plop::many(serde_json::json!(colors), config.plop.clone()).await?
    }
    args::Args::Print { format, .. } => match format {
      args::Format::Json => {
        let stdout = std::io::stdout();
        serde_json::to_writer_pretty(stdout, &colors)?;
      }
      args::Format::List => {
        let mut stdout = std::io::stdout();

        stdout.write_all("Bootstrap:\n".as_bytes())?;
        stdout.write_all(
          colors.bootstrap.background.to_colored_string().as_bytes(),
        )?;
        stdout.write_all(
          colors.bootstrap.foreground.to_colored_string().as_bytes(),
        )?;
        stdout
          .write_all(colors.bootstrap.primary.to_colored_string().as_bytes())?;
        stdout.write_all(
          colors.bootstrap.secondary.to_colored_string().as_bytes(),
        )?;
        stdout
          .write_all(colors.bootstrap.ternary.to_colored_string().as_bytes())?;
        stdout
          .write_all(colors.bootstrap.accent.to_colored_string().as_bytes())?;
        stdout
          .write_all(colors.bootstrap.debug.to_colored_string().as_bytes())?;
        stdout
          .write_all(colors.bootstrap.info.to_colored_string().as_bytes())?;
        stdout
          .write_all(colors.bootstrap.warning.to_colored_string().as_bytes())?;
        stdout
          .write_all(colors.bootstrap.error.to_colored_string().as_bytes())?;

        stdout.write_all("Ansi:\n".as_bytes())?;
        stdout.write_all(colors.ansi.black.to_colored_string().as_bytes())?;
        stdout.write_all(colors.ansi.red.to_colored_string().as_bytes())?;
        stdout.write_all(colors.ansi.green.to_colored_string().as_bytes())?;
        stdout.write_all(colors.ansi.blue.to_colored_string().as_bytes())?;
        stdout.write_all(colors.ansi.cyan.to_colored_string().as_bytes())?;
        stdout.write_all(colors.ansi.yellow.to_colored_string().as_bytes())?;
        stdout.write_all(colors.ansi.magenta.to_colored_string().as_bytes())?;
        stdout.write_all(colors.ansi.white.to_colored_string().as_bytes())?;
        stdout
          .write_all(colors.ansi.bright_black.to_colored_string().as_bytes())?;
        stdout
          .write_all(colors.ansi.bright_red.to_colored_string().as_bytes())?;
        stdout
          .write_all(colors.ansi.bright_green.to_colored_string().as_bytes())?;
        stdout
          .write_all(colors.ansi.bright_blue.to_colored_string().as_bytes())?;
        stdout
          .write_all(colors.ansi.bright_cyan.to_colored_string().as_bytes())?;
        stdout.write_all(
          colors.ansi.bright_yellow.to_colored_string().as_bytes(),
        )?;
        stdout.write_all(
          colors.ansi.bright_magenta.to_colored_string().as_bytes(),
        )?;
        stdout
          .write_all(colors.ansi.bright_white.to_colored_string().as_bytes())?;
      }
    },
  }

  Ok(())
}
