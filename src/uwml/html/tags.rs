use std::str::FromStr;

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
  // TODO: fix this
  type Err = String;

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
  // TODO: fix this
  type Err = String;

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
