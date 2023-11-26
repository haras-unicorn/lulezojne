use super::{contrast::*, Color, Component, FloatingComponent};

// TODO: use https://git.apcacontrast.com/

macro_rules! impl_correct_contrast {
  ($name: ident, $check: ident) => {
    #[allow(dead_code)]
    pub fn $name<TBackground: Color, TForeground: Color>(
      background: TBackground,
      mut foreground: TForeground,
    ) -> impl Color {
      let factor = if background.lightness::<FloatingComponent>()
        < foreground.lightness::<FloatingComponent>()
        || background.lightness::<FloatingComponent>()
          == foreground.lightness::<FloatingComponent>()
          && background.lightness::<FloatingComponent>()
            < FloatingComponent::median(
              FloatingComponent::min_component(),
              FloatingComponent::max_component(),
            ) {
        0.05f32
      } else {
        -0.05f32
      };

      while foreground.lightness::<FloatingComponent>()
        < FloatingComponent::max_component()
        && !$check(foreground, background)
      {
        foreground = foreground.add_lightness(factor);
      }

      foreground
    }
  };
}

impl_correct_contrast!(correct_text_contrast, has_min_contrast_text);
impl_correct_contrast!(
  correct_large_text_contrast,
  has_min_contrast_large_text
);
impl_correct_contrast!(
  correct_text_enhanced_contrast,
  has_enhanced_contrast_text
);
impl_correct_contrast!(
  correct_large_text_enhanced_contrast,
  has_enhanced_contrast_large_text
);
impl_correct_contrast!(correct_graphics_contrast, has_min_contrast_graphics);
