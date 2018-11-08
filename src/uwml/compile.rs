use super::prelude::*;
use crate::ast;

pub fn compile(doc: ast::Doc) -> Document {
  let root_scope = builtin::gen_scope();

  let scope = compile_scope(doc.res_list, &root_scope);
  let root = compile_node(doc.body, &scope);

  Document::new(scope, root)
}

fn compile_scope(res_list: Vec<ast::Res>, parent: &Rc<Scope>) -> Rc<Scope> {
  let mut scope = Scope::new(vec![Rc::downgrade(parent)]);

  for res in res_list {
    let (name, itm) = compile_res(res, &scope);
    scope.define(name, itm);
  }

  Rc::new(scope)
}

fn compile_res(res: ast::Res, scope: &Scope) -> (String, ScopeItem) {
  match res {
    ast::Res::Prop(ast::Prop(n, v)) => (n, compile_propval(v, scope).into()),
    ast::Res::Def(d) => compile_def(d, scope),
    ast::Res::Style(s) => compile_style(s, scope),
  }
}

fn compile_style(style: ast::Style, scope: &Scope) -> (String, ScopeItem) {
  (style.name, Style::new(vec![]).into())
}

fn compile_def(def: ast::Def, scope: &Scope) -> (String, ScopeItem) {
  match def {
    ast::Def::Alias(n, v) => unimplemented!(),
    ast::Def::Elem(n, e) => unimplemented!(),
  }
}

// fn compile_def_elem(_elem: ast::ElemBody, _scope: &Scope) -> Rc<ElementClass> {
//   Rc::new(CustomElementClass::new("div".to_string())) // TODO
// }

fn compile_node(node: ast::Node, scope: &Scope) -> Rc<Node> {
  Rc::new(match node {
    ast::Node::Text(s) => Node::Text(s),
    ast::Node::Elem(e) => Node::Element(compile_elem(e, scope)),
  })
}

fn compile_elem(elem: ast::Elem, scope: &Scope) -> Rc<Element> {
  let (props, body) = compile_elembody(elem.body, scope);

  Rc::new(Element::new(scope.lookup_class(&elem.head.class), props, body))
}

// TODO: why does scope have to be static?
fn compile_elembody(
  body: ast::ElemBody,
  scope: &Scope,
) -> (Vec<(String, Value)>, Option<Vec<Rc<Node>>>) {
  (
    body
      .props
      .into_iter()
      .map(|ast::Prop(k, v)| (k, compile_propval(v, scope)))
      .collect(),
    body
      .children
      .map(|c| c.into_iter().map(|n| compile_node(n, scope)).collect()),
  )
}

fn compile_propval(val: ast::PropVal, scope: &Scope) -> Value {
  match val {
    ast::PropVal::Default => unimplemented!(),
    ast::PropVal::String(s) => s.into(),
    ast::PropVal::Int(i) => i.into(),
    ast::PropVal::Ident(i) => Value::Ident(i),
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
