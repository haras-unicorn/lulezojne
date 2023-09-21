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
  pub extrapolate: ExtrapolateConfig,
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
  #[serde(default = "KmeansGpuConfig::default_runs")]
  pub runs: u64,

  #[serde(default = "KmeansGpuConfig::default_k")]
  pub k: usize,

  #[serde(default = "KmeansGpuConfig::default_max_iter")]
  pub max_iter: usize,

  #[serde(default = "KmeansGpuConfig::default_converge")]
  pub converge: f32,
}

impl KmeansGpuConfig {
  fn default_runs() -> u64 {
    num_cpus::get().try_into().unwrap_or_default()
  }
  fn default_k() -> usize {
    256
  }
  fn default_max_iter() -> usize {
    300
  }
  fn default_converge() -> f32 {
    0.2
  }
}

impl Default for KmeansGpuConfig {
  fn default() -> Self {
    Self {
      runs: KmeansGpuConfig::default_runs(),
      k: KmeansGpuConfig::default_k(),
      max_iter: KmeansGpuConfig::default_max_iter(),
      converge: KmeansGpuConfig::default_converge(),
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
pub struct ExtrapolateConfig {
  #[serde(default = "ExtrapolateConfig::default_main_factor")]
  pub main_factor: f32,

  #[serde(default = "ExtrapolateConfig::default_gradient_factor")]
  pub gradient_factor: f32,

  #[serde(default = "ExtrapolateConfig::default_grayscale_factor")]
  pub grayscale_factor: f32,
}

impl ExtrapolateConfig {
  fn default_main_factor() -> f32 {
    0.8
  }
  fn default_gradient_factor() -> f32 {
    0.7
  }
  fn default_grayscale_factor() -> f32 {
    0.4
  }
}

impl Default for ExtrapolateConfig {
  fn default() -> Self {
    Self {
      main_factor: Self::default_main_factor(),
      gradient_factor: Self::default_gradient_factor(),
      grayscale_factor: Self::default_grayscale_factor(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlopDefinition {
  #[serde(rename = "template")]
  pub template_path: String,

  #[serde(rename = "in")]
  pub destination_path: String,
}

pub async fn read() -> anyhow::Result<Config> {
  let project_dirs =
    match directories::ProjectDirs::from("com", "Lulezojne", "lulezojne") {
      None => return Ok(Config::default()),
      Some(project_dirs) => project_dirs,
    };
  let config_location = {
    let mut location = project_dirs.config_dir().to_path_buf();
    location.push("config.toml");
    location
  };
  if !tokio::fs::try_exists(&config_location).await? {
    return Ok(Config::default());
  }

  let config_string = tokio::fs::read_to_string(&config_location).await?;
  let config = toml::from_str::<Config>(&config_string)?;

  Ok(config)
}
