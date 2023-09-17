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
