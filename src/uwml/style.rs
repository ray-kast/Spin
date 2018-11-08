use super::prelude::*;

#[derive(Debug)]
pub struct Style {
  props: HashMap<String, Value>,
}

impl Style {
  pub fn new<P>(props: P) -> Self
  where
    P: IntoIterator<Item = (String, Value)>,
  {
    Self {
      props: props.into_iter().collect(),
    }
  }
}
