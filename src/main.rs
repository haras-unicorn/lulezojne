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
  let config_location = match &args {
    args::Args::Plop { config, .. } => config.location.clone(),
    args::Args::Print { config, .. } => config.location.clone(),
  };
  let config = config::read(config_location).await?;

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
    args::Backend::Neoquant => {
      let neoquant_config = colors::neoquant::NeoquantConfig {
        sample_faction: config.neoquant.sample_faction,
        colors: config.neoquant.colors,
      };
      colors::neoquant::prominent(generation.image, neoquant_config).await?
    }
    args::Backend::Scolorq => {
      let scolorq_config = colors::scolorq::ScolorqConfig {
        size: config.scolorq.size,
        dither: config.scolorq.dither,
        seed: config.scolorq.seed,
        filter: match config.scolorq.filter {
          config::ScolorqConfigFilter::One => {
            colors::scolorq::ScolorqConfigFilter::One
          }
          config::ScolorqConfigFilter::Three => {
            colors::scolorq::ScolorqConfigFilter::Three
          }
          config::ScolorqConfigFilter::Five => {
            colors::scolorq::ScolorqConfigFilter::Five
          }
        },
        iters: config.scolorq.iters,
        repeats: config.scolorq.repeats,
        start_temp: config.scolorq.start_temp,
        end_temp: config.scolorq.end_temp,
      };
      colors::scolorq::prominent(generation.image, scolorq_config).await?
    }
  };

  let mut ansi = extrapolate::ansi::from(
    palette
      .palette
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
      main_factor: config.ansi.main_factor,
      gradient_factor: config.ansi.gradient_factor,
      grayscale_factor: config.ansi.grayscale_factor,
    },
  );

  match args {
    args::Args::Plop { .. } => {
      plop::many(
        plop::Context {
          ansi: plop::Ansi {
            main: plop::AnsiMain {
              black: ansi_to_plop(ansi.main.black),
              red: ansi_to_plop(ansi.main.red),
              green: ansi_to_plop(ansi.main.green),
              blue: ansi_to_plop(ansi.main.blue),
              cyan: ansi_to_plop(ansi.main.cyan),
              yellow: ansi_to_plop(ansi.main.yellow),
              magenta: ansi_to_plop(ansi.main.magenta),
              grey: ansi_to_plop(ansi.main.grey),
              bright_grey: ansi_to_plop(ansi.main.bright_grey),
              bright_red: ansi_to_plop(ansi.main.bright_red),
              bright_green: ansi_to_plop(ansi.main.bright_green),
              bright_blue: ansi_to_plop(ansi.main.bright_blue),
              bright_cyan: ansi_to_plop(ansi.main.bright_cyan),
              bright_yellow: ansi_to_plop(ansi.main.bright_yellow),
              bright_magenta: ansi_to_plop(ansi.main.bright_magenta),
              white: ansi_to_plop(ansi.main.white),
            },
            gradient: ansi.gradient.drain(0..).map(ansi_to_plop).collect(),
            grayscale: ansi.grayscale.drain(0..).map(ansi_to_plop).collect(),
          },
        },
        plop::Config {
          definitions: config
            .plop_definitions
            .clone()
            .drain(0..)
            .map(
              |config::PlopDefinition {
                 template_or_path,
                 destination_path,
               }| plop::Definition {
                template_or_path,
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
        ansi: print::Ansi {
          main: print::AnsiMain {
            black: ansi_to_print(ansi.main.black),
            red: ansi_to_print(ansi.main.red),
            green: ansi_to_print(ansi.main.green),
            blue: ansi_to_print(ansi.main.blue),
            cyan: ansi_to_print(ansi.main.cyan),
            yellow: ansi_to_print(ansi.main.yellow),
            magenta: ansi_to_print(ansi.main.magenta),
            grey: ansi_to_print(ansi.main.grey),
            bright_grey: ansi_to_print(ansi.main.bright_grey),
            bright_red: ansi_to_print(ansi.main.bright_red),
            bright_green: ansi_to_print(ansi.main.bright_green),
            bright_blue: ansi_to_print(ansi.main.bright_blue),
            bright_cyan: ansi_to_print(ansi.main.bright_cyan),
            bright_yellow: ansi_to_print(ansi.main.bright_yellow),
            bright_magenta: ansi_to_print(ansi.main.bright_magenta),
            white: ansi_to_print(ansi.main.white),
          },
          gradient: ansi.gradient.drain(0..).map(ansi_to_print).collect(),
          grayscale: ansi.grayscale.drain(0..).map(ansi_to_print).collect(),
        },
      };

      match format {
        args::Format::List => print::list::from(colors).await?,
        args::Format::Grid => print::grid::from(colors).await?,
      }
    }
  }

  Ok(())
}

fn ansi_to_plop(color: extrapolate::ansi::Rgba) -> plop::Rgba {
  let extrapolate::ansi::Rgba {
    red,
    green,
    blue,
    alpha,
  } = color;

  plop::Rgba {
    red,
    green,
    blue,
    alpha,
  }
}

fn ansi_to_print(color: extrapolate::ansi::Rgba) -> print::Rgba {
  let extrapolate::ansi::Rgba {
    red,
    green,
    blue,
    alpha,
  } = color;

  print::Rgba {
    red,
    green,
    blue,
    alpha,
  }
}
