use super::prelude::*;

#[derive(Debug)]
pub struct Document {
  scope: Rc<Scope>,
  root: Rc<Node>,
}

impl Document {
  pub fn new(scope: Rc<Scope>, root: Rc<Node>) -> Self { Self { scope, root } }

  pub fn gen_html(&self) -> html::Node { self.root.gen_html() }
}
