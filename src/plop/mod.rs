mod helpers;

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
  pub white: Rgba,
  pub bright_black: Rgba,
  pub bright_red: Rgba,
  pub bright_green: Rgba,
  pub bright_blue: Rgba,
  pub bright_cyan: Rgba,
  pub bright_yellow: Rgba,
  pub bright_magenta: Rgba,
  pub bright_white: Rgba,
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
  pub template_or_path: String,
  pub destination_path: String,
  pub to_exec: Option<DefinitionExec>,
}

#[derive(Debug, Clone)]
pub struct DefinitionExec {
  pub command: String,
  pub args: Option<Vec<String>>,
}

#[tracing::instrument(skip_all)]
pub async fn many(context: Context, config: Config) -> anyhow::Result<()> {
  let definitions = {
    let mut definitions = Vec::new();
    for definition in config.definitions {
      definitions.push(Definition {
        template_or_path: if is_path(definition.template_or_path.as_str()) {
          expand(definition.template_or_path.clone())?
        } else {
          definition.template_or_path
        },
        destination_path: expand(definition.destination_path.clone())?,
        to_exec: definition.to_exec,
      })
    }
    definitions
  };

  let compilation_tasks = definitions
    .iter()
    .map(|definition| definition.template_or_path.clone())
    .unique()
    .map(|path| {
      tokio::spawn(async move { (path.clone(), compile(path).await) })
    });
  let mut registry = handlebars::Handlebars::new();
  helpers::register(&mut registry);
  for task in compilation_tasks {
    let (path, template_result) = task.await?;
    registry.register_template(path.as_str(), template_result?);
  }

  let context = handlebars::Context::wraps(context)?;
  for definition in definitions {
    plop(&registry, &context, definition).await?;
  }

  // let plop_tasks = definitions.iter().map(|definition| {
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

async fn compile(
  template_or_path: String,
) -> anyhow::Result<handlebars::Template> {
  Ok(handlebars::Template::compile(
    (if is_path(template_or_path.as_str()) {
      tokio::fs::read_to_string(template_or_path).await?
    } else {
      template_or_path
    })
    .as_str(),
  )?)
}

async fn plop<'a>(
  registry: &handlebars::Handlebars<'a>,
  context: &handlebars::Context,
  definition: Definition,
) -> anyhow::Result<()> {
  tracing::debug! {
    "Rendering {} to {}",
    definition.template_or_path,
    definition.destination_path
  };

  let rendered =
    registry.render_with_context(&definition.template_or_path, context)?;
  let dirname = std::path::Path::new(&definition.destination_path).parent();
  if let Some(dirname) = dirname {
    if !tokio::fs::try_exists(dirname).await? {
      tokio::fs::create_dir_all(dirname).await?;
    }
  }
  tokio::fs::write(definition.destination_path.as_str(), rendered.into_bytes())
    .await?;

  if let Some(to_exec) = definition.to_exec {
    let mut command = tokio::process::Command::new(to_exec.command);
    if let Some(args) = to_exec.args {
      command.args(args);
    }
    command.status().await?;
  }

  Ok(())
}

fn is_path(str: &str) -> bool {
  !str.contains('\n')
}

fn expand(path: String) -> anyhow::Result<String> {
  match shellexpand::full_with_context(
    path.as_str(),
    || {
      let base_dirs = directories::BaseDirs::new();
      match base_dirs {
        None => None,
        Some(base_dirs) => base_dirs
          .home_dir()
          .to_str()
          .map(|home_dir| home_dir.to_owned()),
      }
    },
    |_| Result::<_, anyhow::Error>::Ok(Option::<&str>::None),
  ) {
    Ok(result) => Ok(result.to_string()),
    Err(shellexpand::LookupError { var_name, cause }) => Err(anyhow::anyhow!(
      "Failed looking up {} because {}",
      var_name,
      cause
    )),
  }
}
