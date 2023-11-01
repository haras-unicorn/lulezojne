use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(author, version, about)]
#[command(before_help = include_str!("../assets/logo.txt"))]
pub enum Args {
  /// Generate color files from the specified image and templates defined in the configuration
  Plop {
    #[clap(flatten)]
    config: ConfigArgs,

    #[clap(flatten)]
    extraction: ExtractionArgs,
  },
  /// Print colors from the specified image
  Print {
    #[clap(flatten)]
    config: ConfigArgs,

    #[clap(flatten)]
    extraction: ExtractionArgs,

    /// The format in which to print
    #[arg(long, short, value_enum, default_value = "list")]
    format: Format,
  },
}

#[derive(Debug, Clone, clap::Args)]
pub struct ConfigArgs {
  /// Alternative config location
  #[arg(long = "config", short = 'c')]
  pub location: Option<String>,
}

#[derive(Debug, Clone, clap::Args)]
pub struct ExtractionArgs {
  /// Image to take prominent colors from
  pub image: String,

  /// Extractor to use for extraction of prominent colors
  #[arg(long, short, value_enum, default_value = "neoquant")]
  pub extractor: Extractor,
}

#[derive(Debug, Clone, Default, clap::ValueEnum)]
pub enum Extractor {
  /// Fast but inaccurate - best for reaaaaly slow devices
  Colorthief,
  /// Slow but highly accurate - use if you have a fast device without a GPU
  Kmeans,
  /// Slow but highly accurate - use if you have a fast device with a GPU
  KmeansGpu,
  /// Medium speed and accuracy - pick this if you don't want to deal with other backends
  MedianCut,
  /// Medium speed and accuracy - improved version of median-cut
  #[default]
  Neoquant,
  /// Super slow but highly accurate  - use if you have a fast device without a GPU
  Scolorq,
}

#[derive(Debug, Clone, Default, clap::ValueEnum)]
pub enum Format {
  /// Json object of all extrapolated colors
  Json,
  /// Simple naive list of all extrapolated colors
  #[default]
  List,
}

// NOTE: try_parse triggers anyhow
pub fn parse() -> Args {
  Args::parse()
}
