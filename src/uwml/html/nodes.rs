pub use super::prelude::*;

#[derive(Debug)]
pub enum Node {
  Text(String),
  Leaf(Leaf),
  Branch(Branch),
}

impl From<String> for Node {
  fn from(text: String) -> Self { Node::Text(text) }
}

impl From<Leaf> for Node {
  fn from(leaf: Leaf) -> Self { Node::Leaf(leaf) }
}

impl From<Branch> for Node {
  fn from(branch: Branch) -> Self { Node::Branch(branch) }
}

// TODO: maybe write this to a stream instead?
impl ToString for Node {
  fn to_string(&self) -> String {
    match self {
      Node::Text(s) => s.clone(), // TODO: ADD HTML ESCAPING
      Node::Leaf(l) => l.to_string(),
      Node::Branch(b) => b.to_string(),
    }
  }
}

#[derive(Debug)]
pub struct Leaf {
  tag: LeafTag,
}

#[derive(Debug)]
pub struct Branch {
  tag: BranchTag,
  children: Vec<Node>,
}

impl Leaf {
  pub fn new(tag: LeafTag) -> Self { Self { tag } }
}

impl ToString for Leaf {
  fn to_string(&self) -> String {
    // TODO: handle attributes
    format!("<{} />", self.tag.to_string())
  }
}

impl Branch {
  pub fn new<C>(tag: BranchTag, children: C) -> Self
  where
    C: IntoIterator<Item = Node>,
  {
    Self {
      tag,
      children: children.into_iter().collect(),
    }
  }
}

impl ToString for Branch {
  fn to_string(&self) -> String {
    // TODO: handle attributes
    format!(
      "<{0}>{1}</{0}>",
      self.tag.to_string(),
      itertools::join(self.children.iter().map(|c| c.to_string()), "")
    )
  }
}
