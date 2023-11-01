use super::{Color, Component, FloatingComponent};

// TODO: use https://git.apcacontrast.com/

macro_rules! impl_correct_contrast {
  ($name: ident, $ratio: expr) => {
    #[allow(dead_code)]
    pub fn $name<TBackground: Color, TForeground: Color>(
      background: TBackground,
      foreground: TForeground,
    ) -> impl Color {
      if background.lightness::<FloatingComponent>()
        < foreground.lightness::<FloatingComponent>()
      {
        let min_lightness =
          background.lightness::<FloatingComponent>() * $ratio;
        foreground.with_lightness(
          foreground
            .lightness::<FloatingComponent>()
            .clamp(min_lightness, FloatingComponent::max_component_value()),
        )
      } else {
        let max_lightness =
          background.lightness::<FloatingComponent>() / $ratio;
        foreground.with_lightness(
          foreground
            .lightness::<FloatingComponent>()
            .clamp(FloatingComponent::min_component_value(), max_lightness),
        )
      }
    }
  };
}

// NOTE: https://www.w3.org/WAI/WCAG22/quickref/?versions=2.1&showtechniques=143%2C146#contrast-minimum
impl_correct_contrast!(
  correct_text_foreground_contrast,
  FloatingComponent::from_f32(4.5f32)
);
impl_correct_contrast!(
  correct_element_foreground_contrast,
  FloatingComponent::from_f32(3f32)
);

// NOTE: https://www.w3.org/WAI/WCAG22/quickref/?versions=2.1&showtechniques=143%2C146#contrast-enhanced
impl_correct_contrast!(
  correct_high_contrast_text_foreground_contrast,
  FloatingComponent::from_f32(7f32)
);
impl_correct_contrast!(
  correct_high_contrast_element_foreground_contrast,
  FloatingComponent::from_f32(4.5f32)
);
