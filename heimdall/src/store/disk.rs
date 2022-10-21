use crate::types::Store;
use log::{debug, warn};

pub struct DiskStore {
    dir: String,
}

impl DiskStore {
    pub fn new(dir: String) -> Self {
        DiskStore { dir }
    }
}

impl Store for DiskStore {
    fn retrieve(&self, module_id: &str) -> Option<Vec<u8>> {
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
