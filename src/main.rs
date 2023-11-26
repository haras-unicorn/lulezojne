#![deny(
    unsafe_code,
    // reason = "Let's just not do it"
  )]
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unreachable,
    clippy::arithmetic_side_effects,
    clippy::as_conversions
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

        stdout.write_all("\nBootstrap:\n".as_bytes())?;
        write_color(&mut stdout, colors.bootstrap.background)?;
        write_color(&mut stdout, colors.bootstrap.foreground)?;
        write_color(&mut stdout, colors.bootstrap.primary)?;
        write_color(&mut stdout, colors.bootstrap.secondary)?;
        write_color(&mut stdout, colors.bootstrap.ternary)?;
        write_color(&mut stdout, colors.bootstrap.accent)?;
        write_color(&mut stdout, colors.bootstrap.debug)?;
        write_color(&mut stdout, colors.bootstrap.info)?;
        write_color(&mut stdout, colors.bootstrap.warning)?;
        write_color(&mut stdout, colors.bootstrap.error)?;

        stdout.write_all("\nAnsi:\n".as_bytes())?;
        write_color(&mut stdout, colors.ansi.black)?;
        write_color(&mut stdout, colors.ansi.red)?;
        write_color(&mut stdout, colors.ansi.green)?;
        write_color(&mut stdout, colors.ansi.blue)?;
        write_color(&mut stdout, colors.ansi.cyan)?;
        write_color(&mut stdout, colors.ansi.yellow)?;
        write_color(&mut stdout, colors.ansi.magenta)?;
        write_color(&mut stdout, colors.ansi.white)?;
        write_color(&mut stdout, colors.ansi.bright_black)?;
        write_color(&mut stdout, colors.ansi.bright_red)?;
        write_color(&mut stdout, colors.ansi.bright_green)?;
        write_color(&mut stdout, colors.ansi.bright_blue)?;
        write_color(&mut stdout, colors.ansi.bright_cyan)?;
        write_color(&mut stdout, colors.ansi.bright_yellow)?;
        write_color(&mut stdout, colors.ansi.bright_magenta)?;
        write_color(&mut stdout, colors.ansi.bright_white)?;
      }
    },
  }

  Ok(())
}

fn write_color(
  write: &mut impl Write,
  color: impl Color,
) -> Result<(), std::io::Error> {
  let color_string = color.to_colored_string();
  let color_square = color.color_square();
  let black = colored::Colorize::truecolor("█", 0, 0, 0);
  let white = colored::Colorize::truecolor("█", 255, 255, 255);
  let contrast_row = format!("{black}{black}{black} {white}{white}{white}");
  let color_row =
    format!("{black}{color_square}{black} {white}{color_square}{white}");
  write.write_all(
    format!("{color_string}\n{contrast_row}\n{color_row}\n{contrast_row}\n\n")
      .as_bytes(),
  )
}
