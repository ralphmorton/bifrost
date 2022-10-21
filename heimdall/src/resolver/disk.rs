
use crate::types::Resolver;

pub struct DiskResolver {
    dir: String
}

impl DiskResolver {
    pub fn new(dir: String) -> Self {
        DiskResolver {
            dir
        }
    }
}

impl Resolver for DiskResolver {
    fn resolve(&self, module_id: &str) -> Option<Vec<u8>> {
        let path = std::path::Path::new(&self.dir).join(module_id);
        std::fs::read(path).ok()
    }
}
