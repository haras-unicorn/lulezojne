use itertools::Itertools;
use serde::{Deserialize, Serialize};

// TODO: parallel file save

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
  pub ansi: Ansi,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ansi {
  pub main: AnsiMain,
  pub gradient: Vec<Rgba>,
  pub grayscale: Vec<Rgba>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnsiMain {
  pub black: Rgba,
  pub red: Rgba,
  pub green: Rgba,
  pub blue: Rgba,
  pub cyan: Rgba,
  pub yellow: Rgba,
  pub magenta: Rgba,
  pub grey: Rgba,
  pub bright_grey: Rgba,
  pub bright_red: Rgba,
  pub bright_green: Rgba,
  pub bright_blue: Rgba,
  pub bright_cyan: Rgba,
  pub bright_yellow: Rgba,
  pub bright_magenta: Rgba,
  pub white: Rgba,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rgba {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
  pub alpha: f32,
}

#[derive(Debug, Clone)]
pub struct Config {
  pub definitions: Vec<Definition>,
}

#[derive(Debug, Clone)]
pub struct Definition {
  pub template_path: String,
  pub destination_path: String,
}

#[tracing::instrument]
pub async fn many(context: Context, config: Config) -> anyhow::Result<()> {
  let compilation_tasks = config
    .definitions
    .iter()
    .map(|definition| definition.template_path.clone())
    .unique()
    .map(|path| {
      tokio::spawn({
        let path = path.clone();
        async move { (path.clone(), compile(path).await) }
      })
    });
  let mut registry = handlebars::Handlebars::new();
  for task in compilation_tasks {
    let (path, template_result) = task.await?;
    registry.register_template(path.as_str(), template_result?);
  }

  let context = handlebars::Context::wraps(context)?;
  for definition in config.definitions {
    plop(&registry, &context, definition.clone()).await?;
  }

  // let plop_tasks = config.definitions.iter().map(|definition| {
  //   tokio::spawn({
  //     let definition = definition.clone();
  //     async { plop(&registry, &context, definition).await }
  //   })
  // });
  // for task in plop_tasks {
  //   task.await?;
  // }

  Ok(())
}

async fn compile(path: String) -> anyhow::Result<handlebars::Template> {
  Ok(handlebars::Template::compile(
    tokio::fs::read_to_string(path).await?.as_str(),
  )?)
}

async fn plop<'a>(
  registry: &handlebars::Handlebars<'a>,
  context: &handlebars::Context,
  definition: Definition,
) -> anyhow::Result<()> {
  let rendered =
    registry.render_with_context(&definition.template_path, context)?;
  tokio::fs::write(definition.destination_path.as_str(), rendered.into_bytes())
    .await?;

  Ok(())
}
