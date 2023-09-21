pub mod colorthief;
pub mod kmeans;
pub mod kmeans_gpu;
pub mod median_cut;

#[derive(Debug, Clone)]
pub struct Colors {
  pub means: Vec<Rgba>,
}

#[derive(Debug, Clone)]
pub struct Rgba {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
  pub alpha: f32,
}
