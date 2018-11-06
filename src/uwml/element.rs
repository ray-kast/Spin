use super::prelude::*;
use std::fmt::Debug;

pub trait ElementClass: Debug {
  fn get_tag(&self) -> &str;

  fn get_attrs(&self) -> () {} // TODO
}

#[derive(Debug)]
pub struct CustomElementClass {
  tag: String,
}

impl ElementClass for CustomElementClass {
  fn get_tag(&self) -> &str { &self.tag }
}

impl CustomElementClass {
  pub fn new(tag: String) -> Self { Self { tag } }
}

#[derive(Debug)]
pub struct Element {
  class: Rc<ElementClass>,
}

impl Element {
  pub fn new(class: Rc<ElementClass>) -> Self { Self { class } }
}
