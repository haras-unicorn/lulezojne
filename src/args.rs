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

    /// The format in which to print
    #[arg(long, short, value_enum, default_value = "list")]
    format: Format,
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
  /// Fast but inaccurate - best for reaaaaly slow devices
  Colorthief,
  /// Slow but highly accurate - best for fast devices without a GPU
  Kmeans,
  /// Slow but highly accurate - best for fast devices with a GPU
  KmeansGpu,
  /// Medium speed and accuracy - pick this if you don't want to deal with other backends
  #[default]
  MedianCut,
  Neoquant,
}

#[derive(Debug, Clone, Default, clap::ValueEnum)]
pub enum Format {
  /// Simple color-coded list displaying all colors in CSS RGBA format
  #[default]
  List,
  /// Grid with names of colors in foreground and color-coded background
  Grid,
}

// NOTE: try_parse triggers anyhow
pub fn parse() -> Args {
  Args::parse()
}
