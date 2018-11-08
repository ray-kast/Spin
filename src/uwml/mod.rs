pub mod builtin;
mod compile;
mod document;
mod element;
pub mod html;
mod node;
mod scope;
mod style;
mod value;

pub use self::{
  compile::*, document::*, element::*, node::*, scope::*, style::*, value::*,
};

pub mod prelude {
  pub use super::*;
  pub use std::{
    collections::{hash_map::Entry as HashEntry, HashMap},
    rc::{Rc, Weak},
  };
}
