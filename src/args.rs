use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(author, version, about)]
#[command(before_help = include_str!("../assets/logo.txt"))]
pub enum Args {
  /// Generate color files from the specified image and templates defined in the configuration
  Plop {
    #[clap(flatten)]
    generation: GenerationArgs,
  },
  /// Print colors from the specified image
  Print {
    #[clap(flatten)]
    generation: GenerationArgs,
  },
}

#[derive(Debug, Clone, clap::Args)]
pub struct GenerationArgs {
  /// Image to take prominent colors from
  pub image: String,

  /// Backend to use for generation of prominent colors
  #[arg(long, short, value_enum, default_value = "colorthief")]
  pub backend: Backend,
}

#[derive(Debug, Clone, Default, clap::ValueEnum)]
pub enum Backend {
  #[default]
  Colorthief,
  Kmeans,
}

// NOTE: try_parse triggers anyhow
pub fn parse() -> Args {
  Args::parse()
}
