#[derive(Debug)]
pub enum Value {
  String(String),
  Int(i64),
}

impl From<String> for Value {
  fn from(s: String) -> Self { Value::String(s) }
}

impl From<i64> for Value {
  fn from(i: i64) -> Self { Value::Int(i) }
}