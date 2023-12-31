use num_traits::{
  CheckedDiv, One, SaturatingAdd, SaturatingMul, SaturatingSub, Zero,
};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
pub struct FloatingComponent(f32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct IntegerComponent(u8);

pub trait Component:
  Clone
  + Copy
  + PartialEq
  + PartialOrd
  + Zero
  + One
  + SaturatingAdd
  + SaturatingSub
  + SaturatingMul
  + CheckedDiv
{
  fn from_f32(value: f32) -> Self;

  fn from_u8(value: u8) -> Self;

  fn from_floating_component(floating: FloatingComponent) -> Self;

  fn from_integer_component(integer: IntegerComponent) -> Self;

  fn to_floating_component(self) -> FloatingComponent;

  fn to_integer_component(self) -> IntegerComponent;

  fn median(min: Self, max: Self) -> Self;

  fn median_circular(min: Self, max: Self) -> Self;

  fn clamp(self, min: Self, max: Self) -> Self;

  fn clamp_circular(self, min: Self, max: Self) -> Self;

  fn extend(self, range: (Self, Self)) -> (Self, Self);

  fn extend_circular(self, range: (Self, Self)) -> (Self, Self);

  fn min_component() -> Self;

  fn max_component() -> Self;
}

impl Zero for FloatingComponent {
  fn zero() -> Self {
    Self(0.0f32)
  }

  fn is_zero(&self) -> bool {
    self.0 == 0.0f32
  }
}

impl One for FloatingComponent {
  fn one() -> Self {
    Self(1.0f32)
  }
}

impl Add for FloatingComponent {
  type Output = Self;

  fn add(self, v: Self) -> Self {
    Self(self.0.add(v.0))
  }
}

impl Sub for FloatingComponent {
  type Output = Self;

  fn sub(self, v: Self) -> Self {
    Self(self.0.sub(v.0))
  }
}

impl Mul for FloatingComponent {
  type Output = Self;

  fn mul(self, v: Self) -> Self {
    Self(self.0.mul(v.0))
  }
}

impl Div for FloatingComponent {
  type Output = Self;

  fn div(self, v: Self) -> Self {
    Self(self.0.div(v.0))
  }
}

impl SaturatingAdd for FloatingComponent {
  fn saturating_add(&self, v: &Self) -> Self {
    Self(self.0.add(v.0))
  }
}

impl SaturatingSub for FloatingComponent {
  fn saturating_sub(&self, v: &Self) -> Self {
    Self(self.0.add(v.0))
  }
}

impl SaturatingMul for FloatingComponent {
  fn saturating_mul(&self, v: &Self) -> Self {
    Self(self.0.add(v.0))
  }
}

impl CheckedDiv for FloatingComponent {
  fn checked_div(&self, v: &Self) -> Option<Self> {
    Some(Self(self.0.div(v.0)))
  }
}

impl Component for FloatingComponent {
  fn from_f32(value: f32) -> Self {
    Self(value.clamp(Self::min_component().0, Self::max_component().0))
  }

  fn from_u8(value: u8) -> Self {
    Self::from_integer_component(IntegerComponent::from_u8(value))
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
    IntegerComponent(
      (self.0 * (IntegerComponent::max_component().0 as f32)).round() as u8,
    )
  }

  fn min_component() -> Self {
    Self(0.0f32)
  }

  fn max_component() -> Self {
    Self(1.0f32)
  }

  fn median(min: Self, max: Self) -> Self {
    Self(min.0 + (max.0 - min.0) / (2 as f32))
  }

  fn median_circular(min: Self, max: Self) -> Self {
    if max > min {
      Self::median(min, max)
    } else {
      let max = max.saturating_add(&Self::max_component());
      let median = Self::median(min, max);
      if median > Self::max_component() {
        median.saturating_sub(&Self::max_component())
      } else {
        median
      }
    }
  }

  fn clamp(self, min: Self, max: Self) -> Self {
    Self(self.0.clamp(min.0, max.0))
  }

  fn clamp_circular(self, min: Self, max: Self) -> Self {
    if max > min {
      self.clamp(min, max)
    } else {
      let me = self.saturating_add(&Self::max_component());
      let max = max.saturating_add(&Self::max_component());
      let clamped = me.clamp(min, max);
      if clamped > Self::max_component() {
        clamped.saturating_sub(&Self::max_component())
      } else {
        clamped
      }
    }
  }

  fn extend(self, range: (Self, Self)) -> (Self, Self) {
    if self < range.0 {
      (self, range.1)
    } else if self > range.1 {
      (range.0, self)
    } else {
      range
    }
  }

  fn extend_circular(self, range: (Self, Self)) -> (Self, Self) {
    todo!()
  }
}

impl Zero for IntegerComponent {
  fn zero() -> Self {
    Self(0u8)
  }

  fn is_zero(&self) -> bool {
    self.0 == 0u8
  }
}

impl One for IntegerComponent {
  fn one() -> Self {
    Self(1u8)
  }
}

impl Add for IntegerComponent {
  type Output = Self;

  fn add(self, v: Self) -> Self {
    Self(self.0.add(v.0))
  }
}

impl Sub for IntegerComponent {
  type Output = Self;

  fn sub(self, v: Self) -> Self {
    Self(self.0.sub(v.0))
  }
}

impl Mul for IntegerComponent {
  type Output = Self;

  fn mul(self, v: Self) -> Self {
    Self(self.0.mul(v.0))
  }
}

impl Div for IntegerComponent {
  type Output = Self;

  fn div(self, v: Self) -> Self {
    Self(self.0.div(v.0))
  }
}

impl SaturatingAdd for IntegerComponent {
  fn saturating_add(&self, v: &Self) -> Self {
    Self(self.0.saturating_add(v.0))
  }
}

impl SaturatingSub for IntegerComponent {
  fn saturating_sub(&self, v: &Self) -> Self {
    Self(self.0.saturating_sub(v.0))
  }
}

impl SaturatingMul for IntegerComponent {
  fn saturating_mul(&self, v: &Self) -> Self {
    Self(self.0.saturating_mul(v.0))
  }
}

impl CheckedDiv for IntegerComponent {
  fn checked_div(&self, v: &Self) -> Option<Self> {
    self.0.checked_div(v.0).map(|v| Self(v))
  }
}

impl Component for IntegerComponent {
  fn from_f32(value: f32) -> Self {
    Self::from_floating_component(FloatingComponent::from_f32(value))
  }

  fn from_u8(value: u8) -> Self {
    Self(value.clamp(Self::min_component().0, Self::max_component().0))
  }

  fn from_floating_component(component: FloatingComponent) -> Self {
    component.to_integer_component()
  }

  fn from_integer_component(component: IntegerComponent) -> Self {
    component.to_integer_component()
  }

  fn to_floating_component(self) -> FloatingComponent {
    FloatingComponent((self.0 as f32) / (Self::max_component().0 as f32))
  }

  fn to_integer_component(self) -> IntegerComponent {
    self
  }

  fn median(min: Self, max: Self) -> Self {
    Self(
      min
        .0
        .saturating_add(max.0.saturating_sub(min.0).saturating_div(2u8)),
    )
  }

  fn min_component() -> Self {
    Self(u8::MIN)
  }

  fn max_component() -> Self {
    Self(u8::MAX)
  }

  fn median_circular(min: Self, max: Self) -> Self {
    if max > min {
      Self::median(min, max)
    } else {
      let max = max.to_u16().saturating_add(Self::max_component().to_u16());
      let min = min.to_u16();
      let median =
        min.saturating_add(max.saturating_sub(min).saturating_div(2u16));
      if median > Self::max_component().to_u16() {
        Self::from_u16(median.saturating_sub(Self::max_component().to_u16()))
      } else {
        Self::from_u16(median)
      }
    }
  }

  fn clamp(self, min: Self, max: Self) -> Self {
    Self(self.0.clamp(min.0, max.0))
  }

  fn clamp_circular(self, min: Self, max: Self) -> Self {
    if max > min {
      Component::clamp(self, min, max)
    } else {
      let me = self.to_u16().saturating_add(Self::max_component().to_u16());
      let maxx = min.to_u16().saturating_add(Self::max_component().to_u16());
      let minn = max.to_u16();
      let clamped = me.clamp(minn, maxx);
      if clamped > Self::max_component().to_u16() {
        Self::from_u16(clamped.saturating_sub(Self::max_component().to_u16()))
      } else {
        Self::from_u16(clamped)
      }
    }
  }

  fn extend(self, range: (Self, Self)) -> (Self, Self) {
    if self < range.0 {
      (self, range.1)
    } else if self > range.1 {
      (range.0, self)
    } else {
      range
    }
  }

  fn extend_circular(self, range: (Self, Self)) -> (Self, Self) {
    if range.1 > range.1 {
      self.extend(range)
    } else {
      let me = self.to_u16();
      let range = (
        range.1.to_u16(),
        range
          .0
          .to_u16()
          .saturating_add(Self::max_component().to_u16()),
      );
      if range.0 <= range.1 {
        if me < range.0 {
          (self, Self::from_u16(range.1))
        } else if me > range.1 {
          (Self::from_u16(range.0), self)
        } else {
          (Self::from_u16(range.0), Self::from_u16(range.1))
        }
      } else {
        if me < range.1 {
          (Self::from_u16(range.0), self)
        } else if me > range.0 {
          (self, Self::from_u16(range.1))
        } else {
          (Self::from_u16(range.0), Self::from_u16(range.1))
        }
      }
    }
  }
}

impl IntegerComponent {
  fn to_u16(self) -> u16 {
    self.0.into()
  }

  fn from_u16(value: u16) -> Self {
    Self(value.clamp(0u16, 255u16) as u8)
  }
}
