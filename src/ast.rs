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
  Alias(String, String),
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
  pub name: String,
  pub base: Vec<String>,
  pub body: ElemBody,
}

#[derive(Debug)]
pub struct ElemBody {
  pub props: Vec<Prop>,
  pub children: Vec<Node>,
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
}
