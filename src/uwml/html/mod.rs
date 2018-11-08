use std::str::FromStr;

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

#[derive(Debug)]
pub enum LeafTag {
  Link,
}

#[derive(Debug)]
pub enum BranchTag {
  Body,
  Div,
  Em,
  Head,
  Html,
  P,
  Span,
  Strong,
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

impl ToString for LeafTag {
  fn to_string(&self) -> String {
    use self::LeafTag::*;

    match self {
      Link => "link",
    }
    .to_string()
  }
}

impl FromStr for LeafTag {
  type Err = String;

  // TODO

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    use self::LeafTag::*;

    match s {
      "link" => Ok(Link),
      s => Err(format!("unknown tag '{}'", s)),
    }
  }
}

impl ToString for BranchTag {
  fn to_string(&self) -> String {
    use self::BranchTag::*;

    match self {
      Body => "body",
      Div => "div",
      Em => "em",
      Head => "head",
      Html => "html",
      P => "p",
      Span => "span",
      Strong => "strong",
    }
    .to_string()
  }
}

impl FromStr for BranchTag {
  type Err = String;

  // TODO

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    use self::BranchTag::*;

    match s {
      "body" => Ok(Body),
      "div" => Ok(Div),
      "em" => Ok(Em),
      "head" => Ok(Head),
      "html" => Ok(Html),
      "p" => Ok(P),
      "span" => Ok(Span),
      "strong" => Ok(Strong),
      s => Err(format!("unknown tag '{}'", s)),
    }
  }
}

pub enum NodeTag {
  Leaf(LeafTag),
  Branch(BranchTag),
}

// TODO: fix error type
pub fn parse_tag(s: &str) -> Result<NodeTag, String> {
  use self::NodeTag::*;
  s.parse()
    .map(|b| Branch(b))
    .or_else(|_| s.parse().map(|l| Leaf(l)))
    .map_err(|_| format!("unknown tag '{}'", s))
}
