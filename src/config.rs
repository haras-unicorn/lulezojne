use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
  #[serde(default, rename = "plop")]
  pub plop_definitions: Vec<PlopDefinition>,

  #[serde(default)]
  pub kmeans: KmeansConfig,

  #[serde(default)]
  pub colorthief: ColorthiefConfig,
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
    16
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
