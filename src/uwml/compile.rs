use super::prelude::*;
use crate::ast;

pub fn compile(doc: ast::Doc, root_scope: &Rc<Scope>) -> Document {
  let scope = compile_scope(doc.res_list, &root_scope);
  let root = compile_node(doc.body, &scope);

  Document::new(scope, root)
}

fn compile_scope(res_list: Vec<ast::Res>, parent: &Rc<Scope>) -> Rc<Scope> {
  let (scope, builder) = Scope::new(vec![Rc::downgrade(parent)]);

  for res in res_list {
    let (name, itm) = compile_res(res, &scope);
    builder.define(name, itm);
  }

  scope
}

fn compile_res(res: ast::Res, scope: &Rc<Scope>) -> (String, ScopeItem) {
  match res {
    ast::Res::Prop(ast::Prop(n, v)) => (n, compile_propval(v, scope).into()),
    ast::Res::Def(d) => compile_def(d, scope),
    ast::Res::Style(s) => compile_style(s, scope),
  }
}

fn compile_style(style: ast::Style, scope: &Rc<Scope>) -> (String, ScopeItem) {
  (
    style.name,
    create_style(style.base, style.body, scope).into(),
  )
}

fn compile_def(def: ast::Def, scope: &Rc<Scope>) -> (String, ScopeItem) {
  match def {
    ast::Def::Alias(n, h) => (n, compile_def_alias(h, scope).into()),
    ast::Def::Elem(n, e) => (n, compile_def_elem(e, scope).into()),
  }
}

fn compile_def_alias(
  head: ast::ElemHead,
  scope: &Rc<Scope>,
) -> AliasElementClass {
  AliasElementClass::new(
    ScopeRef::new(Rc::downgrade(scope), head.class),
    compile_base(head.base, scope),
  )
}

fn compile_def_elem(
  _elem: ast::ElemBody,
  _scope: &Rc<Scope>,
) -> CustomElementClass {
  CustomElementClass::new("div".to_string()) // TODO
}

fn compile_node(node: ast::Node, scope: &Rc<Scope>) -> Node {
  match node {
    ast::Node::Text(s) => Node::Text(s),
    ast::Node::Elem(e) => Node::Element(compile_elem(e, scope)),
  }
}

fn compile_elem(elem: ast::Elem, scope: &Rc<Scope>) -> Element {
  Element::new(
    ScopeRef::new(Rc::downgrade(scope), elem.head.class),
    create_style(elem.head.base, elem.body, scope),
  )
}

fn create_style(
  base: Vec<String>,
  body: ast::ElemBody,
  scope: &Rc<Scope>,
) -> Style {
  let (newscope, props, body) = compile_elembody(body, scope);

  Style::new(newscope, compile_base(base, scope), props, body)
}

// TODO: create some kind of style-base object to abstract lookup
fn compile_base<'a>(
  base: Vec<String>,
  scope: &'a Rc<Scope>,
) -> impl Iterator<Item = ScopeRef<Rc<Style>>> + 'a {
  base
    .into_iter()
    .rev()
    .map(move |b| ScopeRef::new(Rc::downgrade(scope), b))
}

fn compile_elembody(
  body: ast::ElemBody,
  scope: &Rc<Scope>,
) -> (Rc<Scope>, Vec<(String, Rc<Value>)>, Option<Vec<Node>>) {
  let newscope = compile_scope(body.res, scope);

  let newscope_1 = newscope.clone();
  let newscope_2 = newscope.clone();

  (
    newscope,
    body
      .props
      .into_iter()
      .map(move |ast::Prop(k, v)| (k, Rc::new(compile_propval(v, &newscope_1))))
      .collect(),
    body.children.map(move |c| {
      c.into_iter()
        .map(|n| compile_node(n, &newscope_2))
        .collect()
    }),
  )
}

fn compile_propval(val: ast::PropVal, scope: &Rc<Scope>) -> Value {
  match val {
    ast::PropVal::Default => unimplemented!(),
    ast::PropVal::String(s) => s.into(),
    ast::PropVal::Int(i) => i.into(),
    ast::PropVal::Ident(_i) => unimplemented!(),
    ast::PropVal::Elem(e) => compile_elem(e, scope).into(),
    ast::PropVal::Array(a) => a
      .into_iter()
      .map(|v| compile_propval(v, scope))
      .collect::<Vec<_>>()
      .into(),
    ast::PropVal::Block(b) => b
      .map(|c| c.into_iter().map(|n| compile_node(n, scope)).collect())
      .into(),
  }
}
