use regex::Regex;

#[derive(Debug)]
pub struct Doc {
  pub res_list: Vec<Res>,
  pub body: Node,
}

#[derive(Debug)]
pub enum Res {
  Prop(Prop),
  Def(Def),
  Style(Style),
}

#[derive(Debug)]
pub enum Def {
  Alias(String, ElemHead),
  Elem(String, ElemBody),
}

#[derive(Debug)]
pub struct Style {
  pub name: String,
  pub base: Vec<String>,
  pub body: ElemBody,
}

#[derive(Debug)]
pub enum Node {
  Text(String),
  Elem(Elem),
}

#[derive(Debug)]
pub struct Elem {
  pub head: ElemHead,
  pub body: ElemBody,
}

#[derive(Debug)]
pub struct ElemHead {
  pub class: String,
  pub base: Vec<String>,
}

#[derive(Debug)]
pub struct ElemBody {
  pub props: Vec<Prop>,
  pub children: Option<Vec<Node>>,
}

#[derive(Debug)]
pub struct Prop(pub String, pub PropVal);

#[derive(Debug)]
pub enum PropVal {
  Default,
  String(String),
  Int(i64),
  Ident(String),
  Elem(Elem),
  Array(Vec<PropVal>),
  Block(Option<Vec<Node>>),
}

pub fn parse_strlit(s: &str) -> String {
  lazy_static! {
    static ref TRIM_RE: Regex = Regex::new(r#"^"|"$"#).unwrap();
    static ref ESCAPE_RE: Regex = Regex::new(r#"\\(.)"#).unwrap();
  }

  let s = TRIM_RE.replace_all(s, "");
  // TODO: add proper escape processing (e.g. \n, \r, \t, \0, \uXXXX, \xXX, etc.)
  let s = ESCAPE_RE.replace_all(&s, "$1");

  s.into_owned()
}
