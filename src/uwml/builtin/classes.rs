use super::prelude::*;

#[derive(Debug)]
pub struct HtmlElementClass;

#[derive(Debug)]
pub struct NElementClass;

#[derive(Debug)]
pub struct LElementClass;

impl ElementClass for HtmlElementClass {
  fn gen_html(&self, element: &Element) -> html::Node {
    let head = html::Branch::new(html::BranchTag::Head, vec![]);
    let body = html::Branch::new(
      html::BranchTag::Body,
      element.body().iter().map(|n| n.gen_html()),
    );

    html::Branch::new(html::BranchTag::Html, vec![head.into(), body.into()])
      .into()
  }
}

impl ElementClass for NElementClass {
  fn gen_html(&self, element: &Element) -> html::Node {
    // TODO: use the 'tag' property to determine the tag
    html::Branch::new(
      html::BranchTag::Div,
      element.body().iter().map(|n| n.gen_html()),
    )
    .into()
  }
}

impl ElementClass for LElementClass {
  fn gen_html(&self, element: &Element) -> html::Node { unimplemented!() }
}
