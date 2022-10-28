use crate::store::Store;
use log::{debug, error, warn};
use std::collections::HashMap;
use std::string::ToString;

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
            return false;
        }

        if !std::fs::create_dir(&path).is_ok() {
            error!("failed to create module directory at {:?}", &path);
            return false;
        }

        let mod_path = path.join("module.wasm");

        match std::fs::write(&mod_path, binary) {
            Ok(_) => true,
            Err(e) => {
                error!("failed to store module at {:?}: {}", &mod_path, e);
                false
            }
        }
    }

    fn attach_variables(&self, module_id: &str, variables: &Vec<(String, String)>) -> bool {
        let path = std::path::Path::new(&self.dir).join(module_id);
        debug!("attaching env vars to module at {:?}", path);

        if !path.exists() {
            error!("cannot attach env var to missing module at {:?}", &path);
            return false;
        }

        let env_path = path.join("env.json");

        let result = serde_json::to_string(variables)
            .map_err(|e| e.to_string())
            .and_then(|json| std::fs::write(&env_path, json).map_err(|e| e.to_string()));

        match result {
            Ok(_) => true,
            Err(e) => {
                error!(
                    "failed to attach env var to module at {:?}: {}",
                    &env_path, e
                );
                false
            }
        }
    }

    fn attach_capabilities(
        &self,
        module_id: &str,
        capabilities: &HashMap<String, HashMap<String, String>>,
    ) -> bool {
        let path = std::path::Path::new(&self.dir).join(module_id);
        debug!("attaching capabilities to module at {:?}", path);

        if !path.exists() {
            error!(
                "cannot attach capabilities to missing module at {:?}",
                &path
            );
            return false;
        }

        let caps_path = path.join("caps.json");

        let result = serde_json::to_string(capabilities)
            .map_err(|e| e.to_string())
            .and_then(|json| std::fs::write(&caps_path, json).map_err(|e| e.to_string()));

        match result {
            Ok(_) => true,
            Err(e) => {
                error!(
                    "failed to attach capabilities to module at {:?}: {}",
                    &caps_path, e
                );
                false
            }
        }
    }

    fn delete(&self, module_id: &str) -> bool {
        let path = std::path::Path::new(&self.dir).join(module_id);
        debug!("deleting module at {:?}", path);

        match std::fs::remove_dir_all(&path) {
            Ok(_) => true,
            Err(e) => {
                error!("failed to delete module at {:?}: {}", &path, e);
                false
            }
        }
    }

    fn retrieve(
        &self,
        module_id: &str,
    ) -> Option<(
        Vec<u8>,
        Vec<(String, String)>,
        HashMap<String, HashMap<String, String>>,
    )> {
        let path = std::path::Path::new(&self.dir).join(module_id);
        debug!("resolving module at {:?}", path);

        let mod_path = path.join("module.wasm");
        let mod_binary = or_warn(std::fs::read(&mod_path), "unable to load module")?;

        let env_path = path.join("env.json");
        let env_vars = if env_path.exists() {
            or_warn(
                std::fs::read_to_string(&env_path)
                    .map_err(|e| e.to_string())
                    .and_then(|json| serde_json::from_str(&json).map_err(|e| e.to_string())),
                "unable to load env vars",
            )?
        } else {
            Vec::new()
        };

        let caps_path = path.join("caps.json");
        let caps = if caps_path.exists() {
            or_warn(
                std::fs::read_to_string(&caps_path)
                    .map_err(|e| e.to_string())
                    .and_then(|json| serde_json::from_str(&json).map_err(|e| e.to_string())),
                "unable to load capabilities",
            )?
        } else {
            HashMap::new()
        };

        Some((mod_binary, env_vars, caps))
    }
}

fn or_warn<T, E>(res: Result<T, E>, prefix: &'static str) -> Option<T>
where
    E: ToString,
{
    match res {
        Ok(v) => Some(v),
        Err(e) => {
            warn!("{}: {}", prefix, e.to_string());
            None
        }
    }
}
