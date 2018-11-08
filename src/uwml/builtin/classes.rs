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
    // TODO: don't panic
    // TODO: do actual property validation somewhere
    match if let Some(Value::String(s)) = element.props().get("tag") {
      html::parse_tag(s).unwrap()
    } else {
      panic!("missing string property 'tag'");
    } {
      html::NodeTag::Leaf(l) => {
        if !element.body().is_empty() {
          panic!("leaf element cannot have content");
        }

        html::Leaf::new(l).into()
      },
      html::NodeTag::Branch(b) => {
        html::Branch::new(b, element.body().iter().map(|n| n.gen_html())).into()
      },
    }
  }
}

impl ElementClass for LElementClass {
  fn gen_html(&self, _element: &Element) -> html::Node { unimplemented!() }
}
