use super::prelude::*;

pub trait StyleInfo {
  fn get_prop(&self, name: &str) -> Option<Rc<Value>>;

  fn get_body(&self) -> Option<Rc<Vec<Rc<Node>>>>;
}

#[derive(Debug)]
pub struct Style {
  base: Vec<ScopeRef<Rc<Style>>>,
  props: HashMap<String, Rc<Value>>,
  body: Option<Rc<Vec<Rc<Node>>>>,
}

impl Style {
  pub fn new<B, P, O>(base: B, props: P, body: Option<O>) -> Self
  where
    B: IntoIterator<Item = ScopeRef<Rc<Style>>>,
    P: IntoIterator<Item = (String, Rc<Value>)>,
    O: IntoIterator<Item = Rc<Node>>,
  {
    Self {
      base: base.into_iter().collect(),
      props: props.into_iter().collect(),
      body: body.map(|b| Rc::new(b.into_iter().collect())),
    }
  }
}

impl StyleInfo for Style {
  fn get_prop(&self, name: &str) -> Option<Rc<Value>> {
    self.props.get(name).map_or_else(
      || {
        for base in &self.base {
          // TODO: probably shouldn't panic
          if let Some(v) = base.get().unwrap().get_prop(name) {
            return Some(v);
          }
        }

        None
      },
      |v| Some(v.clone()),
    )
  }

  fn get_body(&self) -> Option<Rc<Vec<Rc<Node>>>> {
    self.body.as_ref().map_or_else(
      || {
        for base in &self.base {
          // TODO: probably shouldn't panic
          if let Some(b) = base.get().unwrap().get_body() {
            return Some(b);
          }
        }
        None
      },
      |r| Some(r.clone()),
    )
  }
}
