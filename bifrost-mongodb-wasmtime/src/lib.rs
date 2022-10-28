use anyhow::Error;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::sync::RwLock;
use wasmtime::*;

pub const MODULE: &'static str = "bifrost_mongodb";

const MEMORY: &str = "memory";

const SUCCESS: u32 = 0;
const ERR_MEMORYACCESSFAILED: u32 = 1;
const ERR_QUERYREADFAILED: u32 = 2;
const ERR_QUERYDECODEFAILED: u32 = 3;
const ERR_CONNECTFAILED: u32 = 4;
const ERR_QUERYFAILED: u32 = 5;
const ERR_SERIALIZEFAILED: u32 = 6;
const ERR_PUSHRESPONSEFAILED: u32 = 7;
const ERR_DOCREADFAILED: u32 = 8;
const ERR_READRESPONSEFAILED: u32 = 9;
const ERR_INVALIDHANDLE: u32 = 10;

type Handle = u32;

struct Data {
    bytes: Vec<u8>,
    pos: usize,
}

struct State {
    connection_string: String,
    database: String,
    responses: RwLock<HashMap<Handle, Data>>,
    handle_gen: AtomicU32
}

pub fn add_to_linker<T : std::marker::Send>(connection_string: String, database: String, linker: &mut Linker<T>) -> Result<(), Error> {
    let state = Arc::new(
        State {
            connection_string,
            database,
            responses: RwLock::new(HashMap::new()),
            handle_gen: AtomicU32::new(0)
        }
    );

    let state_find = state.clone();
    let state_read = state.clone();
    let state_close = state.clone();

    linker.func_wrap3_async(
        MODULE,
        "find",
        move |mut caller: Caller<'_, T>, query_ptr: u32, query_len: u32, handle_ptr: u32| {
            let state = state_find.clone();
            Box::new(async move {
                let memory = match caller.get_export(MEMORY) {
                    Some(Extern::Memory(mem)) => mem,
                    _ => return ERR_MEMORYACCESSFAILED,
                };
                let mut ctx = caller.as_context_mut();

                let mut buf = vec![0u8; query_len as usize];
                let read_res = memory.read(&mut ctx, query_ptr as usize, buf.as_mut_slice()).ok();

                if read_res.is_none() {
                    return ERR_QUERYREADFAILED;
                }

                match find(&state, &buf).await {
                    Ok(handle) => {
                        match memory.write(&mut ctx, handle_ptr as usize, &handle.to_le_bytes()) {
                            Ok(_) => return SUCCESS,
                            Err(_) => return ERR_MEMORYACCESSFAILED,
                        }
                    },
                    Err(e) => {
                        return e
                    },
                }
            })
        }
    )?;

    linker.func_wrap4_async(
        MODULE,
        "read",
        move |mut caller: Caller<'_, T>, handle: u32, buf_ptr: u32, buf_len: u32, cont_ptr: u32| {
            let state = state_read.clone();
            Box::new(async move {
                let memory = match caller.get_export(MEMORY) {
                    Some(Extern::Memory(mem)) => mem,
                    _ => return ERR_MEMORYACCESSFAILED,
                };
                let mut ctx = caller.as_context_mut();

                match read(&state, handle, buf_len as usize) {
                    Ok(bytes) => {
                        match memory.write(&mut ctx, buf_ptr as usize, &bytes) {
                            Ok(_) => {
                                let cont : u32 = if bytes.len() == buf_len as usize { 1 } else { 0 };
                                match memory.write(&mut ctx, cont_ptr as usize, &cont.to_le_bytes()) {
                                    Ok(_) => return SUCCESS,
                                    Err(_) => return ERR_MEMORYACCESSFAILED,
                                }
                            },
                            Err(_) => return ERR_MEMORYACCESSFAILED,
                        }
                    },
                    Err(e) => {
                        return e
                    },
                }
            })
        }
    )?;

    linker.func_wrap1_async(
        MODULE,
        "close",
        move |mut _caller: Caller<'_, T>, handle: u32| {
            let state = state_close.clone();
            Box::new(async move {
                match close(&state, handle) {
                    Ok(_) => return SUCCESS,
                    Err(e) => return e,
                }
            })
        }
    )?;

    Ok(())
}

async fn find(state: &Arc<State>, raw: &Vec<u8>) -> Result<Handle, u32> {
    let q : (String, bson::Document) = rmp_serde::from_slice(&raw).ok().ok_or(ERR_QUERYDECODEFAILED)?;

    let collection = q.0;
    let filter = q.1;

    let client = mongodb::Client::with_uri_str(&state.connection_string).await.or(Err(ERR_CONNECTFAILED))?;
    let db = client.database(&state.database);
    let coll = db.collection::<bson::Document>(&collection);
    let mut cursor = coll.find(Some(filter), None).await.or(Err(ERR_QUERYFAILED))?;

    let mut docs = Vec::new();

    while cursor.advance().await.or(Err(ERR_DOCREADFAILED))? {
        let doc = cursor.deserialize_current().or(Err(ERR_DOCREADFAILED))?;
        docs.push(doc);
    }

    let raw = rmp_serde::to_vec(&docs).or(Err(ERR_SERIALIZEFAILED))?;

    let handle = state.handle_gen.fetch_add(1, Ordering::SeqCst);
    let mut responses = state.responses.write().or(Err(ERR_PUSHRESPONSEFAILED))?;
    responses.insert(
        handle,
        Data {
            bytes: raw,
            pos: 0
        }
    );

    Ok(handle)
}

fn read(state: &Arc<State>, handle: Handle, buf_len: usize) -> Result<Vec<u8>, u32> {
    let mut responses = state.responses.write().or(Err(ERR_READRESPONSEFAILED))?;
    let data = responses.remove(&handle).ok_or(ERR_INVALIDHANDLE)?;

    let slice_end = if (data.pos + buf_len) <= data.bytes.len() {
        data.pos + buf_len
    } else {
        data.bytes.len()
    };

    let res = Vec::from(&data.bytes[data.pos..slice_end]);

    responses.insert(
        handle,
        Data {
            bytes: data.bytes,
            pos: (slice_end - data.pos) + data.pos
        }
    );

    Ok(res)
}

fn close(state: &Arc<State>, handle: Handle) -> Result<(), u32> {
    let mut responses = state.responses.write().or(Err(ERR_READRESPONSEFAILED))?;
    responses.remove(&handle).ok_or(ERR_INVALIDHANDLE)?;
    Ok(())
}
