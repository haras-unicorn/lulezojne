mod hex;
mod rgba;

use handlebars::{Helper, JsonValue, PathAndJson, RenderError};

pub fn register(handlebars: &mut handlebars::Handlebars) {
  handlebars.register_helper("hex", Box::new(hex::HexHelper));
  handlebars.register_helper("rgba", Box::new(rgba::RgbaHelper));
}

trait HelperExtensions<'reg, 'rc> {
  fn get_param<'a>(
    &'a self,
    index: usize,
  ) -> Result<&'a PathAndJson<'reg, 'rc>, RenderError>;
}

impl<'reg, 'rc> HelperExtensions<'reg, 'rc> for Helper<'reg, 'rc> {
  fn get_param<'a>(
    &'a self,
    index: usize,
  ) -> Result<&'a PathAndJson<'reg, 'rc>, RenderError> {
    match self.param(index) {
      Some(value) => Ok(value),
      None => Err(RenderError::new(format!("Param {index} missing"))),
    }
  }
}

#[allow(unused)]
trait JsonValueExtensions {
  fn at<'a>(&'a self, key: &str) -> Result<&'a JsonValue, RenderError>;
}

#[allow(unused)]
impl JsonValueExtensions for JsonValue {
  fn at<'a>(&'a self, key: &str) -> Result<&'a JsonValue, RenderError> {
    match self.get(key) {
      None => Err(RenderError::new("red missing")),
      Some(red) => Ok(red),
    }
  }
}
