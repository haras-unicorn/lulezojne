use super::{Color, Component};

#[derive(Debug, Clone, Copy)]
pub struct Metrics<TComponent: Component> {
  pub lightness: ChannelMetrics<TComponent>,
  pub saturation: ChannelMetrics<TComponent>,
}

#[derive(Debug, Clone, Copy)]
pub struct ChannelMetrics<TComponent: Component> {
  pub min: TComponent,
  pub max: TComponent,
  pub average: TComponent,
  pub median: TComponent,
}

macro_rules! min {
  ($metrics: ident, $channel: ident, $color: ident) => {
    $metrics.$channel.min = if $metrics.$channel.min < $color.$channel() {
      $metrics.$channel.min
    } else {
      $color.$channel()
    }
  };
}

macro_rules! max {
  ($metrics: ident, $channel: ident, $color: ident) => {
    $metrics.$channel.max = if $metrics.$channel.max > $color.$channel() {
      $metrics.$channel.max
    } else {
      $color.$channel()
    }
  };
}

pub fn analyze<
  TComponent: Component,
  TColor: Color,
  TIter: IntoIterator<Item = TColor>,
>(
  colors: TIter,
) -> Metrics<TComponent> {
  let mut metrics = Metrics::<TComponent>::default();
  let mut count: TComponent = TComponent::zero();

  for color in colors.into_iter() {
    min!(metrics, lightness, color);
    min!(metrics, saturation, color);

    max!(metrics, lightness, color);
    max!(metrics, saturation, color);

    metrics.lightness.average = metrics.lightness.average + color.lightness();
    metrics.saturation.average =
      metrics.saturation.average + color.saturation();

    count = count + TComponent::one();
  }

  metrics.lightness.average = metrics.lightness.average / count;
  metrics.saturation.average = metrics.saturation.average / count;

  metrics.lightness.median =
    TComponent::median(metrics.lightness.min, metrics.lightness.max);
  metrics.saturation.median =
    TComponent::median(metrics.saturation.min, metrics.saturation.max);

  metrics
}

impl<TComponent: Component> Default for Metrics<TComponent> {
  fn default() -> Self {
    Self {
      lightness: Default::default(),
      saturation: Default::default(),
    }
  }
}

impl<TComponent: Component> Default for ChannelMetrics<TComponent> {
  fn default() -> Self {
    Self {
      min: TComponent::max_component_value(),
      max: TComponent::min_component_value(),
      average: TComponent::zero(),
      median: TComponent::zero(),
    }
  }
}
