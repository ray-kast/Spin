use super::prelude::*;

#[derive(Debug)]
pub struct Scope {
  parents: Vec<Weak<Scope>>,
  items: HashMap<String, ScopeItem>,
}

impl Scope {
  pub fn new<P>(parents: P) -> Self
  where
    P: IntoIterator<Item = Weak<Scope>>,
  {
    Self {
      parents: parents.into_iter().collect(),
      items: HashMap::new(),
    }
  }

  // TODO: add a predicate
  fn lookup_all(&self, name: &str, results: &mut Vec<ScopeItem>) {
    if let Some(itm) = self.items.get(name) {
      results.push(itm.clone());
    }

    for parent in &self.parents {
      parent.upgrade().unwrap().lookup_all(name, results);
    }
  }

  pub fn lookup(&self, name: &str) -> ScopeItem {
    let items = {
      let mut items = Vec::new();
      self.lookup_all(name, &mut items);
      items
    };

    // TODO: don't panic
    match items.len() {
      0 => panic!("use of undefined scope item '{}'", name),
      1 => items.into_iter().next().unwrap(),
      _ => panic!("use of ambiguous name '{}'", name),
    }
  }

  pub fn lookup_class(&self, name: &str) -> Rc<ElementClass> {
    match self.lookup(name) {
      ScopeItem::Class(c) => c,
      _ => panic!("expected element class"), // TODO: don't panic
    }
  }

  pub fn define(&mut self, name: String, itm: ScopeItem) {
    match self.items.entry(name) {
      HashEntry::Vacant(v) => {
        v.insert(itm);
      },
      HashEntry::Occupied(_) => panic!(""),
    }
  }
}

#[derive(Clone, Debug)]
pub enum ScopeItem {
  Const(f64),
  Class(Rc<ElementClass>),
}

impl From<Rc<ElementClass>> for ScopeItem {
  fn from(class: Rc<ElementClass>) -> Self { ScopeItem::Class(class) }
}

impl<T> From<Rc<T>> for ScopeItem
where
  T: ElementClass + 'static,
{
  fn from(class: Rc<T>) -> Self { (class as Rc<ElementClass>).into() }
}

impl<T> From<T> for ScopeItem
where
  T: ElementClass + 'static,
{
  fn from(class: T) -> Self { Rc::new(class).into() }
}
