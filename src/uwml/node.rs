use super::prelude::*;

#[derive(Debug)]
pub enum Node {
  Text(String),
  Element(Element),
}

impl Node {
  pub fn gen_html(&self) -> html::Node {
    match self {
      Node::Text(s) => s.clone().into(),
      Node::Element(e) => e.gen_html(),
    }
  }
}
