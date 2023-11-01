mod ansi;
mod bootstrap;

use std::sync::Arc;

use tokio::sync::Mutex;

use crate::*;

macro_rules! trace_colors {
  ($colors: ident) => {
    tracing::debug! {
      "Extrapolated colors {}",
      serde_json::to_string_pretty(&$colors)
        .unwrap_or_else(|_| "- Failed serializing colors".to_owned())
    };
  };
}
pub(crate) use trace_colors;

#[async_trait::async_trait]
pub trait Extrapolator<
  'a,
  T: core::fmt::Debug + Clone + serde::Serialize + serde::Deserialize<'a>,
>
{
  async fn extrapolate(&self) -> anyhow::Result<T>;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Palette {
  pub bootstrap: bootstrap::Palette,
  pub ansi: ansi::Palette,
}

pub async fn extrapolate(
  config: config::Config,
  extractor: Box<dyn extract::Extractor + Send + Sync>,
) -> anyhow::Result<Palette> {
  let extractor = Arc::new(Mutex::new(extractor));

  let bootstrap_extrapolator =
    bootstrap::Extrapolator::new(config.bootstrap, extractor.clone());
  let bootstrap_task =
    tokio::spawn(async move { bootstrap_extrapolator.extrapolate().await });

  let ansi_extrapolator =
    ansi::Extrapolator::new(config.ansi, extractor.clone());
  let ansi_task =
    tokio::spawn(async move { ansi_extrapolator.extrapolate().await });

  let bootstrap = bootstrap_task.await??;
  let ansi = ansi_task.await??;

  let palette = Palette { bootstrap, ansi };

  Ok(palette)
}
