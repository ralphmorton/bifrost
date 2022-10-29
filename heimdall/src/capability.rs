use anyhow::Error;
use std::collections::HashMap;
use wasmtime::Linker;

pub enum Capability {
    MongoDB(MongoDB),
}

#[derive(Debug)]
pub enum CapabilityInitError {
    UnknownCapability(String),
    MissingArg(&'static str, &'static str),
}

impl Capability {
    pub fn from_config(
        cap: &str,
        args: &HashMap<String, String>,
    ) -> Result<Self, CapabilityInitError> {
        match cap {
            "mongo" => {
                let mdb = MongoDB::from_args(args)?;
                Ok(Self::MongoDB(mdb))
            }
            _ => Err(CapabilityInitError::UnknownCapability(cap.to_string())),
        }
    }

    pub fn add_to_linker<T: std::marker::Send>(&self, linker: &mut Linker<T>) -> Result<(), Error> {
        match self {
            Self::MongoDB(mdb) => mdb.add_to_linker(linker),
        }
    }
}

pub struct MongoDB {
    connection_string: String,
    database: String,
}

impl MongoDB {
    pub const NAME: &'static str = "mongo";
    pub const CONNECTION_STRING: &'static str = "connection_string";
    pub const DATABASE: &'static str = "database";

    fn from_args(args: &HashMap<String, String>) -> Result<Self, CapabilityInitError> {
        let connection_string = args.get(Self::CONNECTION_STRING).map(String::from).ok_or(
            CapabilityInitError::MissingArg(Self::NAME, Self::CONNECTION_STRING),
        )?;

        let database = args
            .get(Self::DATABASE)
            .map(String::from)
            .ok_or(CapabilityInitError::MissingArg(Self::NAME, Self::DATABASE))?;

        Ok(Self {
            connection_string,
            database,
        })
    }

    fn add_to_linker<T: std::marker::Send>(&self, linker: &mut Linker<T>) -> Result<(), Error> {
        bifrost_mongodb_wasmtime::add_to_linker(
            self.connection_string.clone(),
            self.database.clone(),
            linker,
        )
    }
}
