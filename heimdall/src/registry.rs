
use moka::sync::Cache;
use std::sync::Arc;
use wasmtime::{Engine, Module};

pub type ModuleRef = Arc<(Engine, Module)>;

pub struct Registry {
    module_dir: String,
    modules: Cache<String, ModuleRef>
}

impl Registry {
    pub fn new(module_dir: String, max_cached_modules: u64) -> Self {
        Registry {
            module_dir,
            modules: Cache::new(max_cached_modules)
        }
    }

    pub fn resolve(&self, module_name: &str) -> Option<ModuleRef> {
        self.modules
            .get(module_name).map(|arc| arc.clone())
            .or_else(|| self.register(module_name))
    }

    fn register(&self, module_name: &str) -> Option<ModuleRef> {
        let mod_path = std::path::Path::new(&self.module_dir).join(module_name);
        let mod_path_str = mod_path.to_str()?;

        let engine = Engine::default();
        let module = Module::from_file(&engine, mod_path_str).ok()?;

        let mod_ref = Arc::new((engine, module));

        self.modules.insert(module_name.to_string(), mod_ref.clone());

        Some(mod_ref)
    }
}
