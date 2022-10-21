use crate::store::Store;
use log::debug;
use moka::sync::Cache;
use std::sync::Arc;
use wasmtime::{Engine, Module};

pub type ModuleRef = Arc<(Engine, Module)>;

pub struct Registry {
    store: Box<dyn Store + Send + Sync>,
    modules: Cache<String, ModuleRef>,
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

    pub fn delete(&self, module_id: &str) -> bool {
        debug!("deleting module from registry: {}", module_id);

        self.store.delete(module_id)
    }

    pub fn resolve(&self, module_id: &str) -> Option<ModuleRef> {
        debug!("retrieving module from registry: {}", module_id);

        self.modules
            .get(module_id)
            .map(|arc| arc.clone())
            .or_else(|| self.register(module_id))
    }

    fn register(&self, module_id: &str) -> Option<ModuleRef> {
        let binary = self.store.retrieve(module_id)?;

        let engine = Engine::default();
        let module = Module::from_binary(&engine, &binary).ok()?;

        let mod_ref = Arc::new((engine, module));

        self.modules.insert(module_id.to_string(), mod_ref.clone());

        Some(mod_ref)
    }
}
