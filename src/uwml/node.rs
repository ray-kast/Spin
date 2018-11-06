use super::prelude::*;

#[derive(Debug)]
pub enum Node {
  Text(String),
  Element(Rc<Element>),
}
