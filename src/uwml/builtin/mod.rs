pub mod classes;

mod prelude {
  pub use super::super::prelude::*;
}

use super::prelude::*;

pub fn gen_scope() -> Rc<Scope> {
  use self::classes::*;

  let mut scope = Scope::new(Vec::new());

  scope.define("Html".to_string(), HtmlElementClass.into());
  scope.define("N".to_string(), NElementClass.into());
  scope.define("L".to_string(), LElementClass.into());

  Rc::new(scope)
}
