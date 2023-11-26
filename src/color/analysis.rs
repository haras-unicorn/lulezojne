use num_traits::*;

use super::{Color, Component, FloatingComponent};

// TODO: hue min, max, median, average

#[derive(Debug, Clone, Copy)]
pub struct Metrics<TComponent: Component> {
  pub lightness: ChannelMetrics<TComponent>,
  pub saturation: ChannelMetrics<TComponent>,
  pub luminance: ChannelMetrics<TComponent>,
  pub hue: ChannelMetrics<TComponent>,
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
  let mut count: FloatingComponent = Zero::zero();

  let mut lightness_average: FloatingComponent = Zero::zero();
  let mut saturation_average: FloatingComponent = Zero::zero();
  let mut luminance_average: FloatingComponent = Zero::zero();
  for color in colors.into_iter() {
    min!(metrics, lightness, color);
    min!(metrics, saturation, color);
    min!(metrics, luminance, color);

    max!(metrics, lightness, color);
    max!(metrics, saturation, color);
    max!(metrics, luminance, color);

    lightness_average = lightness_average.saturating_add(&color.lightness());
    saturation_average = saturation_average.saturating_add(&color.saturation());
    luminance_average = luminance_average.saturating_add(&color.luminance());

    count = count + One::one();
  }

  metrics.lightness.average = TComponent::from_floating_component(
    lightness_average
      .checked_div(&count)
      .unwrap_or(Zero::zero()),
  );
  metrics.saturation.average = TComponent::from_floating_component(
    saturation_average
      .checked_div(&count)
      .unwrap_or(Zero::zero()),
  );

  metrics.lightness.median =
    TComponent::median(metrics.lightness.min, metrics.lightness.max);
  metrics.saturation.median =
    TComponent::median(metrics.saturation.min, metrics.saturation.max);

  metrics
}

impl<TComponent: Component> Default for Metrics<TComponent> {
  fn default() -> Self {
    Self {
      hue: Default::default(),
      luminance: Default::default(),
      lightness: Default::default(),
      saturation: Default::default(),
    }
  }
}

impl<TComponent: Component> Default for ChannelMetrics<TComponent> {
  fn default() -> Self {
    Self {
      min: TComponent::max_component(),
      max: TComponent::min_component(),
      average: TComponent::zero(),
      median: TComponent::zero(),
    }
  }
}
