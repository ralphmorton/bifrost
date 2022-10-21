use crate::store::Store;
use log::{debug, error, warn};

pub struct DiskStore {
    dir: String,
}

impl DiskStore {
    pub fn new(dir: String) -> Self {
        DiskStore { dir }
    }
}

impl Store for DiskStore {
    fn store(&self, module_id: &str, binary: Vec<u8>) -> bool {
        let path = std::path::Path::new(&self.dir).join(module_id);
        debug!("storing module at {:?}", path);

        if path.exists() {
            error!("will not overwrite existing module at {:?}", &path);
            return false
        }

        match std::fs::write(&path, binary) {
            Ok(_) => true,
            Err(e) => {
                error!("failed to store module at {:?}: {}", &path, e);
                false
            },
        }
    }

    fn delete(&self, module_id: &str) -> bool {
        let path = std::path::Path::new(&self.dir).join(module_id);
        debug!("deleting module at {:?}", path);

        match std::fs::remove_file(&path) {
            Ok(_) => true,
            Err(e) => {
                error!("failed to delete module at {:?}: {}", &path, e);
                false
            },
        }
    }

    fn retrieve(&self, module_id: &str) -> Option<Vec<u8>> {
        let path = std::path::Path::new(&self.dir).join(module_id);
        debug!("resolving module at {:?}", path);

        let res = std::fs::read(&path);

        match res {
            Ok(data) => Some(data),
            Err(e) => {
                warn!("unable to load module {}: {}", module_id, e);
                None
            }
        }
    }
}
