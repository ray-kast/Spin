use super::prelude::*;

#[derive(Debug)]
pub struct HtmlElementClass;

#[derive(Debug)]
pub struct NElementClass;

impl ElementClass for HtmlElementClass {
  fn gen_html(&self, element: &Element) -> html::Node {
    // TODO: don't panic
    // TODO: should direct control of the <head> tag be allowed?
    // TODO: do actual property validation somewhere
    // TODO: perform proper property lookup
    let head_els: Vec<_> = match element.props().get("head") {
      Some(Value::Block(b)) => {
        b.iter().flat_map(|v| v).map(|n| n.gen_html()).collect()
      },
      Some(_) => panic!("expected block for property 'head'"),
      None => Vec::new(),
    };

    let head = html::Branch::new(html::BranchTag::Head, head_els);
    let body = html::Branch::new(
      html::BranchTag::Body,
      element
        .body()
        .into_iter()
        .flat_map(|v| v)
        .map(|n| n.gen_html()),
    );

    html::Branch::new(html::BranchTag::Html, vec![head.into(), body.into()])
      .into()
  }
}

impl ElementClass for NElementClass {
  fn gen_html(&self, element: &Element) -> html::Node {
    // TODO: don't panic
    // TODO: do actual property validation somewhere
    // TODO: perform proper property lookup
    match if let Some(Value::String(s)) = element.props().get("tag") {
      html::parse_tag(s).unwrap()
    } else {
      panic!("missing string property 'tag'");
    } {
      html::NodeTag::Leaf(l) => {
        if !element.body().map_or(true, |b| b.is_empty()) {
          panic!("leaf element cannot have content");
        }

        html::Leaf::new(l).into()
      },
      html::NodeTag::Branch(b) => html::Branch::new(
        b,
        element
          .body()
          .into_iter()
          .flat_map(|v| v)
          .map(|n| n.gen_html()),
      )
      .into(),
    }
  }
}
