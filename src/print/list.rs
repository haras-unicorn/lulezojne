use std::io::Write;

use colored::Colorize;

#[tracing::instrument(skip_all)]
pub async fn from(colors: super::Colors) -> anyhow::Result<()> {
  std::io::stdout().write_all("MAIN:\n".as_bytes())?;
  one(colors.ansi.main.black)?;
  one(colors.ansi.main.red)?;
  one(colors.ansi.main.green)?;
  one(colors.ansi.main.blue)?;
  one(colors.ansi.main.cyan)?;
  one(colors.ansi.main.yellow)?;
  one(colors.ansi.main.magenta)?;
  one(colors.ansi.main.grey)?;
  one(colors.ansi.main.bright_grey)?;
  one(colors.ansi.main.bright_red)?;
  one(colors.ansi.main.bright_green)?;
  one(colors.ansi.main.bright_blue)?;
  one(colors.ansi.main.bright_cyan)?;
  one(colors.ansi.main.bright_yellow)?;
  one(colors.ansi.main.bright_magenta)?;
  one(colors.ansi.main.white)?;

  std::io::stdout().write_all("\nGRADIENT:\n".as_bytes())?;
  for color in colors.ansi.gradient {
    one(color)?;
  }

  std::io::stdout().write_all("\nGRAYSCALE:\n".as_bytes())?;
  for color in colors.ansi.grayscale {
    one(color)?;
  }

  std::io::stdout().write_all("\n".as_bytes())?;

  Ok(())
}

fn one(color: super::Rgba) -> anyhow::Result<()> {
  let super::Rgba {
    red: r,
    green: g,
    blue: b,
    alpha: a,
  } = color;

  std::io::stdout().write_all(
    format!("rgba({r}, {g}, {b}, {a})\n")
      .custom_color(colored::CustomColor { r, g, b })
      .to_string()
      .as_bytes(),
  )?;

  Ok(())
}
