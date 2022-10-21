
use crate::types::Resolver;
use moka::sync::Cache;
use std::sync::Arc;
use wasmtime::{Engine, Module};

pub type ModuleRef = Arc<(Engine, Module)>;

pub struct Registry {
    resolver: Box<dyn Resolver + Send + Sync>,
    modules: Cache<String, ModuleRef>
}

impl Registry {
    pub fn new(resolver: Box<dyn Resolver + Send + Sync>, max_cached_modules: u64) -> Self {
        Registry {
            resolver: resolver,
            modules: Cache::new(max_cached_modules)
        }
    }

    pub fn resolve(&self, module_id: &str) -> Option<ModuleRef> {
        self.modules
            .get(module_id).map(|arc| arc.clone())
            .or_else(|| self.register(module_id))
    }

    fn register(&self, module_id: &str) -> Option<ModuleRef> {
        let binary = self.resolver.resolve(module_id)?;

        let engine = Engine::default();
        let module = Module::from_binary(&engine, &binary).ok()?;

        let mod_ref = Arc::new((engine, module));

        self.modules.insert(module_id.to_string(), mod_ref.clone());

        Some(mod_ref)
    }
}
