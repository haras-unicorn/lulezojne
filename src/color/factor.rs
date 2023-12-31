use num_traits::{SaturatingAdd, SaturatingMul};

use super::{component::Component, FloatingComponent};

pub trait Factor {
  fn multiply<TComponent: Component>(self, component: TComponent)
    -> TComponent;

  fn add<TComponent: Component>(self, component: TComponent) -> TComponent;
}

impl Factor for f32 {
  fn multiply<TComponent: Component>(
    self,
    component: TComponent,
  ) -> TComponent {
    TComponent::from_floating_component(
      component
        .to_floating_component()
        .saturating_mul(&FloatingComponent::from_f32(self.to_owned()))
        .clamp(0.0f32, 1.0f32),
    )
  }

  fn add<TComponent: Component>(self, component: TComponent) -> TComponent {
    TComponent::from_floating_component(
      component
        .to_floating_component()
        .saturating_add(self)
        .clamp(0.0f32, 1.0f32),
    )
  }
}
