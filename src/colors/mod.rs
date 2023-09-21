pub mod colorthief;
pub mod kmeans;
pub mod kmeans_gpu;
pub mod median_cut;
pub mod neoquant;

#[derive(Debug, Clone)]
pub struct Colors {
  pub palette: Vec<Rgba>,
}

#[derive(Debug, Clone, Default)]
pub struct Rgba {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
  pub alpha: f32,
}
