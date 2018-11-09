use super::prelude::*;
use std::{cell::RefCell, convert::TryInto, marker::PhantomData};

#[derive(Debug)]
pub struct Scope {
  parents: Vec<Weak<Scope>>,
  items: RefCell<HashMap<String, ScopeItem>>,
}

pub struct ScopeBuilder(Rc<Scope>);

impl Scope {
  pub fn new<P>(parents: P) -> (Rc<Self>, ScopeBuilder)
  where
    P: IntoIterator<Item = Weak<Scope>>,
  {
    let scope = Rc::new(Self {
      parents: parents.into_iter().collect(),
      items: RefCell::new(HashMap::new()),
    });

    (scope.clone(), ScopeBuilder(scope))
  }

  // TODO: add a predicate
  fn lookup_all(&self, name: &str, results: &mut Vec<ScopeItem>) {
    if let Some(itm) = self.items.borrow().get(name) {
      results.push(itm.clone());
    }

    for parent in &self.parents {
      parent.upgrade().unwrap().lookup_all(name, results);
    }
  }

  fn lookup(&self, name: &str) -> ScopeItem {
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
}

impl ScopeBuilder {
  pub fn define(&self, name: String, itm: ScopeItem) {
    match self.0.items.borrow_mut().entry(name) {
      HashEntry::Vacant(v) => {
        v.insert(itm);
      },
      HashEntry::Occupied(_) => panic!(""),
    }
  }
}

#[derive(Clone, Debug)]
pub enum ScopeItem {
  Value(Rc<Value>),
  Style(Rc<Style>),
  Class(Rc<ElementClass>),
}

impl From<Rc<Value>> for ScopeItem {
  fn from(val: Rc<Value>) -> Self { ScopeItem::Value(val) }
}

impl From<Value> for ScopeItem {
  fn from(val: Value) -> Self { Rc::new(val).into() }
}

impl From<Rc<Style>> for ScopeItem {
  fn from(style: Rc<Style>) -> Self { ScopeItem::Style(style) }
}

impl From<Style> for ScopeItem {
  fn from(style: Style) -> Self { Rc::new(style).into() }
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

impl TryInto<Rc<Value>> for ScopeItem {
  // TODO: fix this
  type Error = ();

  fn try_into(self) -> Result<Rc<Value>, Self::Error> {
    match self {
      ScopeItem::Value(v) => Ok(v),
      _ => Err(()),
    }
  }
}

impl TryInto<Rc<Style>> for ScopeItem {
  // TODO: fix this
  type Error = ();

  fn try_into(self) -> Result<Rc<Style>, Self::Error> {
    match self {
      ScopeItem::Style(s) => Ok(s),
      _ => Err(()),
    }
  }
}

impl TryInto<Rc<ElementClass>> for ScopeItem {
  // TODO: fix this
  type Error = ();

  fn try_into(self) -> Result<Rc<ElementClass>, Self::Error> {
    match self {
      ScopeItem::Class(c) => Ok(c),
      _ => Err(()),
    }
  }
}

// TODO: make sure all ScopeRefs are valid, even if they're not used
#[derive(Debug)]
pub struct ScopeRef<T>
where
  ScopeItem: TryInto<T>,
  T: Clone,
{
  scope: Weak<Scope>,
  name: String,
  _x: PhantomData<T>,
}

impl<T> ScopeRef<T>
where
  ScopeItem: TryInto<T>,
  T: Clone,
{
  pub fn new(scope: Weak<Scope>, name: String) -> Self {
    Self {
      scope,
      name,
      _x: PhantomData,
    }
  }

  pub fn get(&self) -> Result<T, <ScopeItem as TryInto<T>>::Error> {
    // TODO: probably shouldn't panic
    self.scope.upgrade().unwrap().lookup(&self.name).try_into()
  }
}
