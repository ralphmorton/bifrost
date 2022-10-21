
use crate::types::Resolver;
use log::{debug,warn};

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

        debug!("resolving module at {:?}", path);

        let res = std::fs::read(path);

        match res {
            Ok(data) => Some(data),
            Err(err) => {
                warn!("unable to load module {}: {}", module_id, err);
                None
            }
        }
    }
}
