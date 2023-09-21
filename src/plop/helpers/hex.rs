use handlebars::{
  Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext,
};

use super::HelperExtensions;

#[derive(Clone, Copy)]
pub struct HexHelper;

impl HelperDef for HexHelper {
  fn call<'reg: 'rc, 'rc>(
    &self,
    helper: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
  ) -> HelperResult {
    let super::super::Rgba {
      red,
      green,
      blue,
      alpha,
    } = serde_json::value::from_value(helper.get_param(0)?.value().clone())?;
    let alpha = (alpha * 255.0f32).round() as u8;

    out
      .write(format!("#{red:02X}{green:02X}{blue:02X}{alpha:02X}").as_str())?;

    Ok(())
  }
}
