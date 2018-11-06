use super::prelude::*;

pub fn gen_scope() -> Rc<Scope> {
  let mut scope = Scope::new(Vec::new());

  Rc::new(scope)
}
