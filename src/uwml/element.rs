use super::prelude::*;
use std::fmt::Debug;

pub trait ElementClass: Debug {
  fn gen_html(&self, element: &Element) -> html::Node;
}

#[derive(Debug)]
pub struct CustomElementClass {
  tag: String,
}

impl ElementClass for CustomElementClass {
  fn gen_html(&self, _element: &Element) -> html::Node { unimplemented!() }
}

impl CustomElementClass {
  pub fn new(tag: String) -> Self { Self { tag } }
}

#[derive(Debug)]
pub struct Element {
  class: Rc<ElementClass>,
  props: HashMap<String, Value>,
  body: Vec<Rc<Node>>,
}

impl Element {
  pub fn new<P, B>(class: Rc<ElementClass>, props: P, body: B) -> Self
  where
    P: IntoIterator<Item = (String, Value)>,
    B: IntoIterator<Item = Rc<Node>>,
  {
    Self {
      class,
      props: props.into_iter().collect(),
      body: body.into_iter().collect(),
    }
  }

  pub fn props(&self) -> &HashMap<String, Value> { &self.props }

  pub fn body(&self) -> &Vec<Rc<Node>> { &self.body }

  pub fn gen_html(&self) -> html::Node { self.class.gen_html(self) }
}
