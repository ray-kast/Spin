use super::prelude::*;

#[derive(Debug)]
pub struct HtmlElementClass;

#[derive(Debug)]
pub struct NElementClass;

impl StyleInfo for HtmlElementClass {
  fn get_prop(&self, _name: &str) -> Option<Rc<Value>> { None }

  fn get_body(&self) -> Option<Rc<Vec<Node>>> { None }
}

impl ElementClass for HtmlElementClass {
  fn gen_html(&self, element: &Element) -> html::Node {
    // TODO: don't panic
    // TODO: should direct control of the <head> tag be allowed?
    // TODO: do actual property validation somewhere
    let head_els: Vec<_> = match element.get_prop("head") {
      Some(v) => match &*v {
        Value::Block(b) => {
          b.iter().flat_map(|v| v).map(|n| n.gen_html()).collect()
        },
        _ => panic!("expected block for property 'head'"),
      },
      None => Vec::new(),
    };

    let head = html::Branch::new(html::BranchTag::Head, head_els);
    let body = html::Branch::new(
      html::BranchTag::Body,
      element.get_body().map_or_else(
        || Vec::new(),
        |v| v.iter().map(|n| n.gen_html()).collect(),
      ),
    );

    html::Branch::new(html::BranchTag::Html, vec![head.into(), body.into()])
      .into()
  }
}

impl StyleInfo for NElementClass {
  fn get_prop(&self, _name: &str) -> Option<Rc<Value>> { None }

  fn get_body(&self) -> Option<Rc<Vec<Node>>> { None }
}

impl ElementClass for NElementClass {
  fn gen_html(&self, element: &Element) -> html::Node {
    // TODO: don't panic
    // TODO: do actual property validation somewhere
    match if let Some(v) = element.get_prop("tag") {
      if let Value::String(s) = &*v {
        html::parse_tag(s).unwrap()
      } else {
        panic!("expecting string for property 'tag'");
      }
    } else {
      panic!("missing string property 'tag'");
    } {
      html::NodeTag::Leaf(l) => {
        if !element.get_body().map_or(true, |b| b.is_empty()) {
          panic!("leaf element cannot have content");
        }

        html::Leaf::new(l).into()
      },
      html::NodeTag::Branch(b) => html::Branch::new(
        b,
        element.get_body().map_or_else(
          || Vec::new(),
          |v| v.iter().map(|n| n.gen_html()).collect(),
        ),
      )
      .into(),
    }
  }
}
