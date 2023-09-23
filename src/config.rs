use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
  #[serde(default, rename = "plop")]
  pub plop_definitions: Vec<PlopDefinition>,

  #[serde(default)]
  pub kmeans: KmeansConfig,

  #[serde(default)]
  pub kmeans_gpu: KmeansGpuConfig,

  #[serde(default)]
  pub colorthief: ColorthiefConfig,

  #[serde(default)]
  pub median_cut: MedianCutConfig,

  #[serde(default)]
  pub neoquant: NeoquantConfig,

  #[serde(default)]
  pub scolorq: ScolorqConfig,

  #[serde(default)]
  pub ansi: AnsiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KmeansConfig {
  #[serde(default = "KmeansConfig::default_runs")]
  pub runs: u64,

  #[serde(default = "KmeansConfig::default_k")]
  pub k: usize,

  #[serde(default = "KmeansConfig::default_max_iter")]
  pub max_iter: usize,

  #[serde(default = "KmeansConfig::default_converge")]
  pub converge: f32,
}

impl KmeansConfig {
  fn default_runs() -> u64 {
    num_cpus::get().try_into().unwrap_or_default()
  }
  fn default_k() -> usize {
    256
  }
  fn default_max_iter() -> usize {
    30
  }
  fn default_converge() -> f32 {
    5.0
  }
}

impl Default for KmeansConfig {
  fn default() -> Self {
    Self {
      runs: KmeansConfig::default_runs(),
      k: KmeansConfig::default_k(),
      max_iter: KmeansConfig::default_max_iter(),
      converge: KmeansConfig::default_converge(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KmeansGpuConfig {
  #[serde(default = "KmeansGpuConfig::default_count")]
  pub count: u32,

  #[serde(default = "KmeansGpuConfig::default_algorithm")]
  pub algorithm: KmeansGpuAlgorithm,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum KmeansGpuAlgorithm {
  #[default]
  Kmeans,
  Octree,
}

impl KmeansGpuConfig {
  fn default_count() -> u32 {
    256
  }
  fn default_algorithm() -> KmeansGpuAlgorithm {
    KmeansGpuAlgorithm::Kmeans
  }
}

impl Default for KmeansGpuConfig {
  fn default() -> Self {
    Self {
      count: Self::default_count(),
      algorithm: Self::default_algorithm(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorthiefConfig {
  #[serde(default = "ColorthiefConfig::default_quality")]
  pub quality: u8,

  #[serde(default = "ColorthiefConfig::default_max_colors")]
  pub max_colors: u8,
}

impl ColorthiefConfig {
  fn default_quality() -> u8 {
    10
  }
  fn default_max_colors() -> u8 {
    16
  }
}

impl Default for ColorthiefConfig {
  fn default() -> Self {
    Self {
      quality: Self::default_quality(),
      max_colors: Self::default_max_colors(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedianCutConfig {
  #[serde(default = "MedianCutConfig::default_iterations")]
  pub iterations: u8,
}

impl MedianCutConfig {
  fn default_iterations() -> u8 {
    8 // 2 ^ 8 = 256 for ANSI
  }
}

impl Default for MedianCutConfig {
  fn default() -> Self {
    Self {
      iterations: Self::default_iterations(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeoquantConfig {
  #[serde(default = "NeoquantConfig::default_sample_faction")]
  pub sample_faction: i32,

  #[serde(default = "NeoquantConfig::default_colors")]
  pub colors: usize,
}

impl NeoquantConfig {
  pub fn default_sample_faction() -> i32 {
    10
  }

  pub fn default_colors() -> usize {
    256
  }
}

impl Default for NeoquantConfig {
  fn default() -> Self {
    Self {
      sample_faction: Self::default_sample_faction(),
      colors: Self::default_colors(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScolorqConfig {
  #[serde(default = "ScolorqConfig::default_size")]
  pub size: u8,

  #[serde(default = "ScolorqConfig::default_dither")]
  pub dither: Option<f64>,

  #[serde(default = "ScolorqConfig::default_seed")]
  pub seed: Option<u64>,

  #[serde(default = "ScolorqConfig::default_filter")]
  pub filter: ScolorqConfigFilter,

  #[serde(default = "ScolorqConfig::default_iters")]
  pub iters: usize,

  #[serde(default = "ScolorqConfig::default_repeats")]
  pub repeats: usize,

  #[serde(default = "ScolorqConfig::default_start_temp")]
  pub start_temp: f64,

  #[serde(default = "ScolorqConfig::default_end_temp")]
  pub end_temp: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum ScolorqConfigFilter {
  One,
  #[default]
  Three,
  Five,
}

impl ScolorqConfig {
  fn default_size() -> u8 {
    // NOTE: higher lasts LONGER
    32
  }

  fn default_dither() -> Option<f64> {
    None
  }

  fn default_seed() -> Option<u64> {
    None
  }

  fn default_filter() -> ScolorqConfigFilter {
    ScolorqConfigFilter::Three
  }

  fn default_iters() -> usize {
    3
  }

  fn default_repeats() -> usize {
    1
  }

  fn default_start_temp() -> f64 {
    1.0
  }

  fn default_end_temp() -> f64 {
    0.001
  }
}

impl Default for ScolorqConfig {
  fn default() -> Self {
    Self {
      size: Self::default_size(),
      dither: Self::default_dither(),
      seed: Self::default_seed(),
      filter: Self::default_filter(),
      iters: Self::default_iters(),
      repeats: Self::default_repeats(),
      start_temp: Self::default_start_temp(),
      end_temp: Self::default_end_temp(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnsiConfig {
  #[serde(default)]
  pub main: AnsiMainConfig,

  #[serde(default = "AnsiConfig::default_gradient_mix_factor")]
  pub gradient_mix_factor: f32,

  #[serde(default = "AnsiConfig::default_grayscale_mix_factor")]
  pub grayscale_mix_factor: f32,
}

impl AnsiConfig {
  fn default_gradient_mix_factor() -> f32 {
    0.7
  }

  fn default_grayscale_mix_factor() -> f32 {
    0.4
  }
}

impl Default for AnsiConfig {
  fn default() -> Self {
    Self {
      main: Default::default(),
      gradient_mix_factor: Self::default_gradient_mix_factor(),
      grayscale_mix_factor: Self::default_grayscale_mix_factor(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnsiMainConfig {
  #[serde(default = "AnsiMainConfig::default_mix_factor")]
  pub mix_factor: f32,

  #[serde(default = "AnsiMainConfig::default_saturation_range")]
  pub saturation_range: (f32, f32),

  #[serde(default = "AnsiMainConfig::default_lightness_range")]
  pub lightness_range: (f32, f32),
}

impl AnsiMainConfig {
  pub fn default_mix_factor() -> f32 {
    0.8
  }

  pub fn default_saturation_range() -> (f32, f32) {
    (0.6, 0.9)
  }

  pub fn default_lightness_range() -> (f32, f32) {
    (0.7, 1.0)
  }
}

impl Default for AnsiMainConfig {
  fn default() -> Self {
    Self {
      mix_factor: Self::default_mix_factor(),
      saturation_range: Self::default_saturation_range(),
      lightness_range: Self::default_lightness_range(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlopDefinition {
  #[serde(rename = "template")]
  pub template_or_path: String,

  #[serde(rename = "in")]
  pub destination_path: String,
}

pub async fn read(config_location: Option<String>) -> anyhow::Result<Config> {
  let config_location = {
    match config_location {
      Some(config_location) => config_location.into(),
      None => {
        let project_dirs =
          match directories::ProjectDirs::from("com", "Lulezojne", "lulezojne")
          {
            None => return Ok(Config::default()),
            Some(project_dirs) => project_dirs,
          };
        let mut location = project_dirs.config_dir().to_path_buf();
        location.push("config.toml");
        location
      }
    }
  };
  if !tokio::fs::try_exists(&config_location).await? {
    return Ok(Config::default());
  }

  let config_string = tokio::fs::read_to_string(&config_location).await?;
  let config = toml::from_str::<Config>(&config_string)?;

  Ok(config)
}
