use anyhow::Error;
use wasmtime::*;

pub struct Mongo {}

impl Mongo {
    pub const MODULE: &'static str = "bifrost_mongodb";

    pub fn new() -> Self {
        Self {}
    }

    pub fn add_to_linker<T>(&self, linker: &mut Linker<T>) -> Result<(), Error> {
        linker.func_wrap(
            Self::MODULE,
            "query",
            move |i: i32| -> i32 {
                i + 1
            }
        )?;

        Ok(())
    }
}
