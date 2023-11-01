mod helpers;

use itertools::Itertools;

use crate::*;

#[tracing::instrument(skip_all)]
pub async fn many(
  context: serde_json::Value,
  config: Vec<config::Plop>,
) -> anyhow::Result<()> {
  let definitions = {
    let mut definitions = Vec::new();
    for definition in config {
      definitions.push(config::Plop {
        template: if is_path(definition.template.as_str()) {
          expand(definition.template.clone())?
        } else {
          definition.template
        },
        destination: expand(definition.destination.clone())?,
        then: definition.then,
      })
    }
    definitions
  };

  let compilation_tasks = definitions
    .iter()
    .map(|definition| definition.template.clone())
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

  // TODO: parallel file save
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
  definition: config::Plop,
) -> anyhow::Result<()> {
  tracing::debug! {
    "Rendering {} to {}",
    definition.template,
    definition.destination
  };

  let rendered = registry.render_with_context(&definition.template, context)?;
  let dirname = std::path::Path::new(&definition.destination).parent();
  if let Some(dirname) = dirname {
    if !tokio::fs::try_exists(dirname).await? {
      tokio::fs::create_dir_all(dirname).await?;
    }
  }
  tokio::fs::write(definition.destination.as_str(), rendered.into_bytes())
    .await?;

  if let Some(to_exec) = definition.then {
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
