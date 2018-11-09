pub mod classes;

mod prelude {
  pub use super::super::prelude::*;
}

use super::prelude::*;

pub fn gen_scope() -> Rc<Scope> {
  use self::classes::*;

  let (scope, builder) = Scope::new(Vec::new());

  builder.define("Html".to_string(), HtmlElementClass.into());
  builder.define("N".to_string(), NElementClass.into());

  scope
}
