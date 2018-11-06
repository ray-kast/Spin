use std::collections::HashMap;

#[derive(Debug)]
pub struct ResourceDict {
  items: HashMap<String, Resource>,
}

impl ResourceDict {
  pub fn new(items: HashMap<String, Resource>) -> Self { Self { items } }
}

#[derive(Debug)]
pub enum Resource {}
