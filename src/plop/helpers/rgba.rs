use handlebars::{
  Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext,
};

use super::HelperExtensions;

#[derive(Clone, Copy)]
pub struct RgbaHelper;

impl HelperDef for RgbaHelper {
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

    out.write(format!("rgba({red}, {green}, {blue}, {alpha})").as_str())?;

    Ok(())
  }
}
