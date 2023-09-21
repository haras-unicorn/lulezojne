#![deny(
    unsafe_code,
    // reason = "Let's just not do it"
  )]
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    // reason = "We have to handle errors properly"
  )]
#![deny(
  clippy::dbg_macro,
  // reason = "Use tracing instead"
)]

mod args;
mod colors;
mod config;
mod extrapolate;
mod plop;
mod print;

#[tokio::main]
#[tracing::instrument]
async fn main() -> anyhow::Result<()> {
  tracing::subscriber::set_global_default({
    let mut builder = tracing_subscriber::FmtSubscriber::builder();
    #[cfg(debug_assertions)]
    {
      builder =
        builder.with_max_level(tracing::level_filters::LevelFilter::DEBUG);
    }
    #[cfg(not(debug_assertions))]
    {
      builder =
        builder.with_max_level(tracing::level_filters::LevelFilter::WARN);
    }
    builder.finish()
  })?;

  let args = args::parse();
  let config = config::read().await?;

  let generation = match &args {
    args::Args::Plop { generation, .. } => generation.clone(),
    args::Args::Print { generation, .. } => generation.clone(),
  };
  let mut palette = match generation.backend {
    args::Backend::Kmeans => {
      let kmeans_config = colors::kmeans::KmeansConfig {
        runs: config.kmeans.runs,
        k: config.kmeans.k,
        converge: config.kmeans.converge,
        max_iter: config.kmeans.max_iter,
      };
      colors::kmeans::prominent(generation.image, kmeans_config).await?
    }
    args::Backend::Colorthief => {
      let colorthief_config = colors::colorthief::ColorthiefConfig {
        quality: config.colorthief.quality,
        max_colors: config.colorthief.max_colors,
      };
      colors::colorthief::prominent(generation.image, colorthief_config).await?
    }
    args::Backend::KmeansGpu => {
      let kmeans_gpu_config = colors::kmeans_gpu::KmeansGpuConfig {
        runs: config.kmeans_gpu.runs,
        k: config.kmeans_gpu.k,
        converge: config.kmeans_gpu.converge,
        max_iter: config.kmeans_gpu.max_iter,
      };
      colors::kmeans_gpu::prominent(generation.image, kmeans_gpu_config).await?
    }
    args::Backend::MedianCut => {
      let median_cut_config = colors::median_cut::MedianCutConfig {
        iterations: config.median_cut.iterations,
      };
      colors::median_cut::prominent(generation.image, median_cut_config).await?
    }
  };

  let mut ansi = extrapolate::ansi::from(
    palette
      .means
      .drain(0..)
      .map(
        |colors::Rgba {
           red,
           green,
           blue,
           alpha,
         }| extrapolate::ansi::Rgba {
          red,
          green,
          blue,
          alpha,
        },
      )
      .collect(),
    extrapolate::ansi::Config {
      main_factor: config.extrapolate.main_factor,
      gradient_factor: config.extrapolate.gradient_factor,
      grayscale_factor: config.extrapolate.grayscale_factor,
    },
  );

  match args {
    args::Args::Plop { .. } => {
      plop::many(
        plop::Context {
          means: ansi
            .drain(0..)
            .map(
              |extrapolate::ansi::Rgba {
                 red,
                 green,
                 blue,
                 alpha,
               }| plop::Rgba {
                red,
                green,
                blue,
                alpha,
              },
            )
            .collect(),
        },
        plop::Config {
          definitions: config
            .plop_definitions
            .clone()
            .drain(0..)
            .map(
              |config::PlopDefinition {
                 template_path,
                 destination_path,
               }| plop::Definition {
                template_path,
                destination_path,
              },
            )
            .collect(),
        },
      )
      .await?;
    }
    args::Args::Print { format, .. } => {
      let colors = print::Colors {
        ansi: ansi
          .drain(0..)
          .map(
            |extrapolate::ansi::Rgba {
               red,
               green,
               blue,
               alpha,
             }| print::Rgba {
              red,
              green,
              blue,
              alpha,
            },
          )
          .collect(),
      };

      match format {
        args::Format::List => print::list::from(colors).await?,
        args::Format::Grid => print::grid::from(colors).await?,
      }
    }
  }

  Ok(())
}
