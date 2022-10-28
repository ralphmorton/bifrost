use crate::store::Store;
use log::{debug, error};
use moka::sync::Cache;
use std::sync::Arc;
use wasmtime::{Config, Engine, Module};

pub type EnvironmentRef = Arc<Environment>;

pub struct Environment {
    pub engine: Engine,
    pub module: Module,
    pub variables: Vec<(String, String)>
}

pub struct Registry {
    store: Box<dyn Store + Send + Sync>,
    modules: Cache<String, EnvironmentRef>,
}

impl Registry {
    pub fn new(store: Box<dyn Store + Send + Sync>, max_cached_modules: u64) -> Self {
        Registry {
            store: store,
            modules: Cache::new(max_cached_modules),
        }
    }

    pub fn add(&self, module_id: &str, binary: Vec<u8>) -> bool {
        debug!("adding module to registry: {}", module_id);
        self.store.store(module_id, binary)
    }

    pub fn attach_variables(&self, module_id: &str, variables: Vec<(String, String)>) -> bool {
        debug!("attaching env vars to registered module: {}", module_id);
        let result = self.store.attach_variables(module_id, variables);
        self.modules.invalidate(module_id);
        result
    }

    pub fn delete(&self, module_id: &str) -> bool {
        debug!("deleting module from registry: {}", module_id);
        self.store.delete(module_id)
    }

    pub fn resolve(&self, module_id: &str) -> Option<EnvironmentRef> {
        debug!("retrieving module from registry: {}", module_id);

        self.modules
            .get(module_id)
            .map(|arc| arc.clone())
            .or_else(|| self.register(module_id))
    }

    fn register(&self, module_id: &str) -> Option<EnvironmentRef> {
        let (binary, vars) = self.store.retrieve(module_id)?;

        let mut config = Config::new();
        config.async_support(true);
        let engine = Engine::new(&config).ok()?;

        match Module::from_binary(&engine, &binary) {
            Err(e) => {
                error!("unable to initialize module from store: {:?}", e);
                None
            },
            Ok(module) => {
                let env_ref = Arc::new(
                    Environment {
                        engine,
                        module,
                        variables: vars
                    }
                );
                self.modules.insert(module_id.to_string(), env_ref.clone());
                Some(env_ref)
            },
        }
    }
}
