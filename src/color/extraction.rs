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
        color.$channel::<TComponent>() > min
          && color.$channel::<TComponent>() < max
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
      let channel = color.$channel::<FloatingComponent>();
      let min = channel + min_difference.to_floating_component();
      let max = channel - min_difference.to_floating_component();
      colors.into_iter().filter(move |color| {
        color.$channel::<FloatingComponent>() > min
          && color.$channel::<FloatingComponent>() < max
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
      let channel = color.$channel::<FloatingComponent>();
      let min = channel - min_difference.to_floating_component();
      let max = channel + min_difference.to_floating_component();
      colors.into_iter().filter(move |color| {
        color.$channel::<FloatingComponent>() > min
          && color.$channel::<FloatingComponent>() < max
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
        .min_by_key(move |color| color.$channel::<IntegerComponent>())
    }

    #[allow(dead_code)]
    pub fn $name_max<TColor: Color, TIntoIter: IntoIterator<Item = TColor>>(
      colors: TIntoIter,
    ) -> Option<TColor> {
      colors
        .into_iter()
        .min_by_key(move |color| color.$channel::<IntegerComponent>())
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
      let channel = color.$channel::<FloatingComponent>();
      colors.into_iter().min_by(move |lhs, rhs| {
        let lhs_diff = (lhs.$channel::<FloatingComponent>() - channel).abs();
        let rhs_diff = (rhs.$channel::<FloatingComponent>() - channel).abs();
        lhs_diff.total_cmp(&rhs_diff)
      })
    }
  };
}

impl_filter_max_difference!(lightness, filter_lightness_max_difference);
impl_filter_min_difference!(lightness, filter_lightness_min_difference);
impl_filter_range!(lightness, filter_lightness_range);
impl_closest_by!(lightness, closest_by_lightness);
impl_min_max!(lightness, darkest, brightest);

impl_filter_max_difference!(luminance, filter_luminance_max_difference);
impl_filter_min_difference!(luminance, filter_luminance_min_difference);
impl_filter_range!(luminance, filter_luminance_range);
impl_closest_by!(luminance, closest_by_luminance);
impl_min_max!(luminance, least_luminant, most_luminant);

impl_filter_max_difference!(saturation, filter_saturation_max_difference);
impl_filter_min_difference!(saturation, filter_saturation_min_difference);
impl_filter_range!(saturation, filter_saturation_range);
impl_closest_by!(saturation, closest_by_saturation);
impl_min_max!(saturation, least_saturated, most_saturated);

impl_filter_max_difference!(hue, filter_hue_max_difference);
impl_filter_min_difference!(hue, filter_hue_min_difference);
impl_filter_range!(hue, filter_hue_range);
impl_closest_by!(hue, closest_by_hue);

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
