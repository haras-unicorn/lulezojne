pub mod grid;
pub mod list;

#[derive(Debug, Clone)]
pub struct Colors {
  pub ansi: Vec<Rgba>,
}

#[derive(Debug, Clone)]
pub struct Rgba {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
  pub alpha: f32,
}
