use std::io::Write;

use colored::Colorize;

// TODO: actual grid

#[tracing::instrument]
pub async fn from(colors: super::Colors) -> anyhow::Result<()> {
  for super::Rgba {
    red: r,
    green: g,
    blue: b,
    alpha: a,
  } in colors.ansi
  {
    std::io::stdout().write_all(
      format!("rgba({r}, {g}, {b}, {a})\n")
        .custom_color(colored::CustomColor { r, g, b })
        .to_string()
        .as_bytes(),
    )?;
  }

  Ok(())
}
