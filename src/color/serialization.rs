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
        let red = match red.parse::<FloatingComponent>() {
          Ok(color) => color,
          Err(error) => {
            return Err(E::invalid_type(
              serde::de::Unexpected::Other(error.to_string().as_str()),
              &"a float",
            ))
          }
        };
        let green = match green.parse::<FloatingComponent>() {
          Ok(color) => color,
          Err(error) => {
            return Err(E::invalid_type(
              serde::de::Unexpected::Other(error.to_string().as_str()),
              &"a float",
            ))
          }
        };
        let blue = match blue.parse::<FloatingComponent>() {
          Ok(color) => color,
          Err(error) => {
            return Err(E::invalid_type(
              serde::de::Unexpected::Other(error.to_string().as_str()),
              &"a float",
            ))
          }
        };
        let alpha = match alpha.parse::<FloatingComponent>() {
          Ok(color) => color,
          Err(error) => {
            return Err(E::invalid_type(
              serde::de::Unexpected::Other(error.to_string().as_str()),
              &"a float",
            ))
          }
        };

        Ok(ColorImpl {
          red,
          green,
          blue,
          alpha,
        })
      }

      _ => {
        return Err(E::invalid_value(
          serde::de::Unexpected::Other("unmatched regex"),
          &format!("a string matching {RGBA_PATTERN_STR}").as_str(),
        ))
      }
    }
  }
}

const RGBA_PATTERN_STR: &'static str =
  r"rgba\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*,\s*([0-9.]+)\s*\)";

lazy_static::lazy_static! {
  #[allow(clippy::unwrap_used)] // NOTE: it is a valid regex
  static ref RGBA_PATTERN: regex::Regex =
    regex::Regex::new(RGBA_PATTERN_STR).unwrap();
}
