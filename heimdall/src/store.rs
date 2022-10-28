pub mod disk;

use std::collections::HashMap;

pub trait Store {
    fn store(&self, module_id: &str, binary: Vec<u8>) -> bool;

    fn attach_variables(&self, module_id: &str, variables: &Vec<(String, String)>) -> bool;

    fn attach_capabilities(
        &self,
        module_id: &str,
        capabilities: &HashMap<String, HashMap<String, String>>,
    ) -> bool;

    fn delete(&self, module_id: &str) -> bool;

    fn retrieve(
        &self,
        module_id: &str,
    ) -> Option<(
        Vec<u8>,
        Vec<(String, String)>,
        HashMap<String, HashMap<String, String>>,
    )>;
}
