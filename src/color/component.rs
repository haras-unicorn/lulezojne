use std::ops::{Add, Div, Mul, Sub};

use num_traits::{One, Zero};

pub type FloatingComponent = f32;
pub type IntegerComponent = u8;

pub trait Component:
  Default
  + Clone
  + Copy
  + PartialOrd
  + Zero
  + One
  + Add<Output = Self>
  + Sub<Output = Self>
  + Mul<Output = Self>
  + Div<Output = Self>
{
  fn from_f32(value: f32) -> Self;

  fn from_u8(value: u8) -> Self;

  fn from_floating_component(floating: FloatingComponent) -> Self;

  fn from_integer_component(integer: IntegerComponent) -> Self;

  fn to_floating_component(self) -> FloatingComponent;

  fn to_integer_component(self) -> IntegerComponent;

  fn median(min: Self, max: Self) -> Self;

  fn min_component_value() -> Self;

  fn max_component_value() -> Self;
}

impl Component for FloatingComponent {
  fn from_f32(value: f32) -> Self {
    Self::from_floating_component(value)
  }

  fn from_u8(value: u8) -> Self {
    Self::from_integer_component(value)
  }

  fn from_floating_component(component: FloatingComponent) -> Self {
    component.to_floating_component()
  }

  fn from_integer_component(component: IntegerComponent) -> Self {
    component.to_floating_component()
  }

  fn to_floating_component(self) -> FloatingComponent {
    self
  }

  fn to_integer_component(self) -> IntegerComponent {
    (self * IntegerComponent::max_value() as FloatingComponent).round()
      as IntegerComponent
  }

  fn median(min: Self, max: Self) -> Self {
    min + (max - min) / (2 as Self)
  }

  fn min_component_value() -> Self {
    Self::zero()
  }

  fn max_component_value() -> Self {
    Self::one()
  }
}

impl Component for IntegerComponent {
  fn from_f32(value: f32) -> Self {
    Self::from_floating_component(value)
  }

  fn from_u8(value: u8) -> Self {
    Self::from_integer_component(value)
  }

  fn from_floating_component(component: FloatingComponent) -> Self {
    component.to_integer_component()
  }

  fn from_integer_component(component: IntegerComponent) -> Self {
    component.to_integer_component()
  }

  fn to_floating_component(self) -> FloatingComponent {
    (self as FloatingComponent) / (Self::max_value() as FloatingComponent)
  }

  fn to_integer_component(self) -> IntegerComponent {
    self
  }

  fn median(min: Self, max: Self) -> Self {
    min + (max - min) / (2 as Self)
  }

  fn min_component_value() -> Self {
    Self::min_value()
  }

  fn max_component_value() -> Self {
    Self::max_value()
  }
}
