use std::io::Write;

use colored::Colorize;

#[derive(Debug, Clone)]
pub struct Colors {
  pub means: Vec<Rgba>,
}

#[derive(Debug, Clone)]
pub struct Rgba {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
  pub alpha: u8,
}

#[tracing::instrument]
pub async fn pretty(colors: Colors) -> anyhow::Result<()> {
  for Rgba {
    red: r,
    green: g,
    blue: b,
    alpha: a,
  } in colors.means
  {
    std::io::stdout().write(
      format!("rgba({r}, {g}, {b}, {a})\n")
        .custom_color(colored::CustomColor { r, g, b })
        .to_string()
        .as_bytes(),
    )?;
  }

  Ok(())
}
