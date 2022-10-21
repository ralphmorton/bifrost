pub mod disk;

pub trait Store {
  fn store(&self, module_id: &str, binary: Vec<u8>) -> bool;

  fn delete(&self, module_id: &str) -> bool;

  fn retrieve(&self, module_id: &str) -> Option<Vec<u8>>;
}
