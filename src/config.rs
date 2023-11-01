use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
  #[serde(default)]
  pub accessibility: AccessibilityConfig,

  #[serde(default)]
  pub plop: Vec<Plop>,

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

  #[serde(default)]
  pub bootstrap: BootstrapConfig,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum AccessibilityConfig {
  #[default]
  Normal,

  HighContrast,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KmeansConfig {
  #[serde(default = "KmeansConfig::default_runs")]
  pub runs: u64,

  #[serde(default = "KmeansConfig::default_max_iter")]
  pub max_iter: usize,

  #[serde(default = "KmeansConfig::default_converge")]
  pub converge: f32,
}

impl KmeansConfig {
  fn default_runs() -> u64 {
    num_cpus::get().try_into().unwrap_or_default()
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
      max_iter: KmeansConfig::default_max_iter(),
      converge: KmeansConfig::default_converge(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KmeansGpuConfig {
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
  fn default_algorithm() -> KmeansGpuAlgorithm {
    KmeansGpuAlgorithm::Kmeans
  }
}

impl Default for KmeansGpuConfig {
  fn default() -> Self {
    Self {
      algorithm: Self::default_algorithm(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorthiefConfig {
  #[serde(default = "ColorthiefConfig::default_quality")]
  pub quality: u8,
}

impl ColorthiefConfig {
  fn default_quality() -> u8 {
    10
  }
}

impl Default for ColorthiefConfig {
  fn default() -> Self {
    Self {
      quality: Self::default_quality(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MedianCutConfig {}

impl MedianCutConfig {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeoquantConfig {
  #[serde(default = "NeoquantConfig::default_sample_faction")]
  pub sample_faction: i32,
}

impl NeoquantConfig {
  pub fn default_sample_faction() -> i32 {
    10
  }
}

impl Default for NeoquantConfig {
  fn default() -> Self {
    Self {
      sample_faction: Self::default_sample_faction(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScolorqConfig {
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
  #[serde(default = "AnsiConfig::default_main")]
  pub main: AnsiAreaConfig,

  #[serde(default = "AnsiConfig::default_gradient")]
  pub gradient: AnsiAreaConfig,

  #[serde(default = "AnsiConfig::default_grayscale")]
  pub grayscale: AnsiAreaConfig,
}

impl AnsiConfig {
  fn default_main() -> AnsiAreaConfig {
    AnsiAreaConfig {
      lightness_factor: 0.6,
      saturation_factor: 0.6,
    }
  }
  fn default_gradient() -> AnsiAreaConfig {
    AnsiAreaConfig {
      lightness_factor: 0.2,
      saturation_factor: 0.8,
    }
  }
  fn default_grayscale() -> AnsiAreaConfig {
    AnsiAreaConfig {
      lightness_factor: 0.8,
      saturation_factor: 0.2,
    }
  }
}

impl Default for AnsiConfig {
  fn default() -> Self {
    Self {
      main: Self::default_main(),
      gradient: Self::default_gradient(),
      grayscale: Self::default_grayscale(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnsiAreaConfig {
  pub lightness_factor: f32,

  pub saturation_factor: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BootstrapConfig {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plop {
  #[serde(rename = "template")]
  pub template: String,

  #[serde(rename = "in")]
  pub destination: String,

  #[serde(rename = "then")]
  pub then: Option<PlopExec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlopExec {
  #[serde(rename = "command")]
  pub command: String,

  #[serde(rename = "args")]
  pub args: Option<Vec<String>>,
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
