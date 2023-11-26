use super::{Color, Component, FloatingComponent, IntegerComponent};

macro_rules! impl_filter_range {
  ($channel: ident, $name: ident) => {
    #[allow(dead_code)]
    pub fn $name<
      TComponent: Component,
      TColor: Color,
      TIntoIter: IntoIterator<Item = TColor>,
    >(
      min: TComponent,
      max: TComponent,
      colors: TIntoIter,
    ) -> impl Iterator<Item = TColor> {
      colors.into_iter().filter(move |color| {
        $channel::<FloatingComponent>(*color) > min.to_floating_component()
          && $channel::<FloatingComponent>(*color) < max.to_floating_component()
      })
    }
  };
}

macro_rules! impl_filter_min_difference {
  ($channel: ident, $name: ident) => {
    #[allow(dead_code)]
    pub fn $name<
      TComponent: Component,
      TColor: Color,
      TIntoIter: IntoIterator<Item = TColor>,
    >(
      color: TColor,
      min_difference: TComponent,
      colors: TIntoIter,
    ) -> impl Iterator<Item = TColor> {
      let channel = $channel::<FloatingComponent>(color);
      let min = channel + min_difference.to_floating_component();
      let max = channel - min_difference.to_floating_component();
      colors.into_iter().filter(move |color| {
        $channel::<FloatingComponent>(*color) > min
          && $channel::<FloatingComponent>(*color) < max
      })
    }
  };
}

macro_rules! impl_filter_max_difference {
  ($channel: ident, $name: ident) => {
    #[allow(dead_code)]
    pub fn $name<
      TComponent: Component,
      TColor: Color,
      TIntoIter: IntoIterator<Item = TColor>,
    >(
      color: TColor,
      min_difference: TComponent,
      colors: TIntoIter,
    ) -> impl Iterator<Item = TColor> {
      let channel = $channel::<FloatingComponent>(color);
      let min = channel - min_difference.to_floating_component();
      let max = channel + min_difference.to_floating_component();
      colors.into_iter().filter(move |color| {
        $channel::<FloatingComponent>(*color) > min
          && $channel::<FloatingComponent>(*color) < max
      })
    }
  };
}

macro_rules! impl_min_max {
  ($channel: ident, $name_min: ident, $name_max: ident) => {
    #[allow(dead_code)]
    pub fn $name_min<TColor: Color, TIntoIter: IntoIterator<Item = TColor>>(
      colors: TIntoIter,
    ) -> Option<TColor> {
      colors
        .into_iter()
        .min_by_key(move |color| $channel::<IntegerComponent>(*color))
    }

    #[allow(dead_code)]
    pub fn $name_max<TColor: Color, TIntoIter: IntoIterator<Item = TColor>>(
      colors: TIntoIter,
    ) -> Option<TColor> {
      colors
        .into_iter()
        .max_by_key(move |color| $channel::<IntegerComponent>(*color))
    }
  };
}

macro_rules! impl_closest_by {
  ($channel: ident, $name: ident) => {
    #[allow(dead_code)]
    pub fn $name<TColor: Color, TIntoIter: IntoIterator<Item = TColor>>(
      color: TColor,
      colors: TIntoIter,
    ) -> Option<TColor> {
      let channel = $channel::<FloatingComponent>(color);
      colors.into_iter().min_by(move |lhs, rhs| {
        let lhs_diff = ($channel::<FloatingComponent>(color) - channel).abs();
        let rhs_diff = ($channel::<FloatingComponent>(color) - channel).abs();
        lhs_diff.total_cmp(&rhs_diff)
      })
    }
  };
}

fn lightness_channel<TComponent: Component>(color: impl Color) -> TComponent {
  color.lightness::<TComponent>()
}
impl_filter_max_difference!(lightness_channel, filter_lightness_max_difference);
impl_filter_min_difference!(lightness_channel, filter_lightness_min_difference);
impl_filter_range!(lightness_channel, filter_lightness_range);
impl_closest_by!(lightness_channel, closest_by_lightness);
impl_min_max!(lightness_channel, darkest, brightest);

fn saturation_channel<TComponent: Component>(color: impl Color) -> TComponent {
  color.saturation::<TComponent>()
}
impl_filter_max_difference!(
  saturation_channel,
  filter_saturation_max_difference
);
impl_filter_min_difference!(
  saturation_channel,
  filter_saturation_min_difference
);
impl_filter_range!(saturation_channel, filter_saturation_range);
impl_closest_by!(saturation_channel, closest_by_saturation);
impl_min_max!(saturation_channel, least_saturated, most_saturated);

fn hue_channel<TComponent: Component>(color: impl Color) -> TComponent {
  TComponent::from_f32(color.hue().into_inner())
}
impl_filter_max_difference!(hue_channel, filter_hue_max_difference);
impl_filter_min_difference!(hue_channel, filter_hue_min_difference);
impl_filter_range!(hue_channel, filter_hue_range);
impl_closest_by!(hue_channel, closest_by_hue);

#[allow(dead_code)]
pub fn closest<TColor: Color, TIntoIter: IntoIterator<Item = TColor>>(
  color: TColor,
  colors: TIntoIter,
) -> Option<TColor> {
  colors.into_iter().min_by(move |lhs, rhs| {
    let lhs_dist = (lhs.distance::<FloatingComponent>(color)).abs();
    let rhs_dist = (rhs.distance::<FloatingComponent>(color)).abs();
    lhs_dist.total_cmp(&rhs_dist)
  })
}

#[allow(dead_code)]
pub fn clamp_hue_around<
  TColor: Color,
  TReference: Color,
  TTolerance: Component,
>(
  color: TColor,
  reference: TReference,
  tolerance: TTolerance,
) -> TColor {
  let half_tolerance = tolerance / TTolerance::from_f32(2.0);
  let min_hue = reference.hue().saturating_sub(half_tolerance);
  let max_hue = reference.hue().saturating_add(half_tolerance);
}
