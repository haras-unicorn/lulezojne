use super::{Color, ColorImpl, FloatingComponent, IntegerComponent};

impl<'de> serde::Deserialize<'de> for ColorImpl {
  fn deserialize<TDeserializer>(
    deserializer: TDeserializer,
  ) -> Result<Self, TDeserializer::Error>
  where
    TDeserializer: serde::Deserializer<'de>,
  {
    deserializer.deserialize_string(ColorImplVisitor)
  }
}

impl serde::Serialize for ColorImpl {
  fn serialize<TSerializer>(
    &self,
    serializer: TSerializer,
  ) -> Result<TSerializer::Ok, TSerializer::Error>
  where
    TSerializer: serde::Serializer,
  {
    let red = self.red::<IntegerComponent>();
    let green = self.green::<IntegerComponent>();
    let blue = self.blue::<IntegerComponent>();
    let alpha = self.alpha::<FloatingComponent>();
    serializer
      .serialize_str(format!("rgba({red}, {green}, {blue}, {alpha})").as_str())
  }
}

struct ColorImplVisitor;

impl<'de> serde::de::Visitor<'de> for ColorImplVisitor {
  type Value = ColorImpl;

  fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
    formatter
      .write_str("a string in format rgba({red}, {green}, {blue}, {alpha})")
  }

  fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    match RGBA_PATTERN.captures(value) {
      Some(captures) => {
        let (_, [red, green, blue, alpha]) = captures.extract();
        // FIXME: this panics!
        let red = red.parse::<FloatingComponent>().unwrap();
        // FIXME: this panics!
        let green = green.parse::<FloatingComponent>().unwrap();
        // FIXME: this panics!
        let blue = blue.parse::<FloatingComponent>().unwrap();
        // FIXME: this panics!
        let alpha = alpha.parse::<FloatingComponent>().unwrap();

        Ok(ColorImpl {
          red,
          green,
          blue,
          alpha,
        })
      }
      // FIXME: this panics!
      _ => panic!("yea"),
    }
  }
}

lazy_static::lazy_static! {
  #[allow(clippy::unwrap_used)]
  static ref RGBA_PATTERN: regex::Regex = regex::Regex::new(
    r"rgba\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*,\s*([0-9.]+)\s*\)"
  ).unwrap();
}
