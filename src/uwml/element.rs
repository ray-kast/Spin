use super::prelude::*;
use std::fmt::Debug;

pub trait ElementClass: StyleInfo + Debug {
  fn gen_html(&self, element: &Element) -> html::Node;
}

#[derive(Debug)]
pub struct AliasElementClass {
  class: ScopeRef<Rc<ElementClass>>,
  base: Vec<ScopeRef<Rc<Style>>>,
}

impl AliasElementClass {
  pub fn new<B>(class: ScopeRef<Rc<ElementClass>>, base: B) -> Self
  where
    B: IntoIterator<Item = ScopeRef<Rc<Style>>>,
  {
    Self {
      class,
      base: base.into_iter().collect(),
    }
  }
}

impl StyleInfo for AliasElementClass {
  fn get_prop(&self, name: &str) -> Option<Rc<Value>> {
    for base in &self.base {
      // TODO: probably shouldn't panic
      if let Some(v) = base.get().unwrap().get_prop(name) {
        return Some(v);
      }
    }

    None
  }

  fn get_body(&self) -> Option<Rc<Vec<Node>>> {
    for base in &self.base {
      // TODO: probably shouldn't panic
      if let Some(b) = base.get().unwrap().get_body() {
        return Some(b);
      }
    }

    None
  }
}

impl ElementClass for AliasElementClass {
  fn gen_html(&self, element: &Element) -> html::Node {
    // TODO: don't panic
    // TODO: check for infinite lööps
    self.class.get().unwrap().gen_html(element)
  }
}

#[derive(Debug)]
pub struct CustomElementClass {
  tag: String,
}

impl CustomElementClass {
  pub fn new(tag: String) -> Self { Self { tag } }
}

impl StyleInfo for CustomElementClass {
  fn get_prop(&self, _name: &str) -> Option<Rc<Value>> { None }

  fn get_body(&self) -> Option<Rc<Vec<Node>>> { None }
}

impl ElementClass for CustomElementClass {
  fn gen_html(&self, _element: &Element) -> html::Node { unimplemented!() }
}

#[derive(Debug)]
pub struct Element {
  class: ScopeRef<Rc<ElementClass>>,
  style: Style,
}

impl Element {
  pub fn new(class: ScopeRef<Rc<ElementClass>>, style: Style) -> Self {
    Self { class, style }
  }

  pub fn gen_html(&self) -> html::Node {
    // TODO: don't panic
    self.class.get().unwrap().gen_html(self)
  }
}

impl StyleInfo for Element {
  // TODO: probably shouldn't panic
  fn get_prop(&self, name: &str) -> Option<Rc<Value>> {
    self
      .style
      .get_prop(name)
      .or_else(|| self.class.get().unwrap().get_prop(name))
  }

  fn get_body(&self) -> Option<Rc<Vec<Node>>> {
    self
      .style
      .get_body()
      .or_else(|| self.class.get().unwrap().get_body())
  }
}
