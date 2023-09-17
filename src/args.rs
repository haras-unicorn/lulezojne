use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None)]
pub enum Args {
  /// Generate color files from the specified image and templates defined in the configuration
  #[command(author, version, about, long_about = None)]
  Plop {
    #[clap(flatten)]
    generation: GenerationArgs,
  },
  /// Print colors from the specified image
  #[command(author, version, about, long_about = None)]
  Print {
    #[clap(flatten)]
    generation: GenerationArgs,
  },
}

#[derive(Debug, Clone, clap::Args)]
pub struct GenerationArgs {
  pub image: String,

  #[arg(long, short, value_enum)]
  pub backend: Backend,
}

#[derive(Debug, Clone, Default, clap::ValueEnum)]
pub enum Backend {
  #[default]
  Colorthief,
  Kmeans,
}

pub fn parse() -> anyhow::Result<Args> {
  Ok(Args::try_parse()?)
}
