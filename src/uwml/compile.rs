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
    ast::Res::Prop(p) => unreachable!(),
    ast::Res::Def(d) => compile_def(d, scope),
    ast::Res::Style(s) => unreachable!(),
  }
}

fn compile_def(def: ast::Def, scope: &Scope) -> (String, ScopeItem) {
  match def {
    ast::Def::Alias(n, v) => unreachable!(),
    ast::Def::Elem(n, e) => (n, compile_def_elem(e, scope).into()),
  }
}

fn compile_def_elem(elem: ast::ElemBody, scope: &Scope) -> Rc<ElementClass> {
  Rc::new(CustomElementClass::new("div".to_string())) // TODO
}

fn compile_node(node: ast::Node, scope: &Rc<Scope>) -> Rc<Node> {
  Rc::new(match node {
    ast::Node::Text(s) => Node::Text(s),
    ast::Node::Elem(e) => Node::Element(compile_element(e, scope)),
  })
}

fn compile_element(elem: ast::Elem, scope: &Rc<Scope>) -> Rc<Element> {
  let body = elem
    .body
    .children
    .into_iter()
    .map(|n| compile_node(n, scope));

  Rc::new(Element::new(scope.lookup_class(&elem.name), body))
}
