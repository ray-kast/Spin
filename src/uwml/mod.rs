pub mod builtin;
mod compile;
mod document;
mod element;
pub mod html;
mod ir_tags;
mod node;
mod scope;
mod style;
mod value;

pub use self::compile::*;

pub mod prelude {
  pub use super::{
    document::*, element::*, ir_tags::*, node::*, scope::*, style::*, value::*,
    *,
  };
  pub use std::{
    collections::{hash_map::Entry as HashEntry, HashMap},
    rc::{Rc, Weak},
  };
}
