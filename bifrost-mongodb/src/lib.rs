use std::collections::HashMap;
use std::mem::MaybeUninit;

mod raw {
    #[link(wasm_import_module = "bifrost_mongodb")]
    extern "C" {
        pub fn find(query_ptr: u32, query_len: u32, handle_ptr: u32) -> u32;
        pub fn insert(query_ptr: u32, query_len: u32, handle_ptr: u32) -> u32;
        pub fn delete(query_ptr: u32, query_len: u32) -> u32;
        pub fn update(query_ptr: u32, query_len: u32) -> u32;
        pub fn read(handle: u32, buf_ptr: u32, buf_len: u32, cont_ptr: u32) -> u32;
        pub fn close(handle: u32) -> u32;
    }
}

pub fn find(collection: &str, doc: &bson::Document) -> Result<Vec<bson::Document>, u32> {
    let data = rmp_serde::to_vec(&(collection, doc)).unwrap();
    let mut handle : MaybeUninit<u32> = MaybeUninit::uninit();

    unsafe {
        let res = raw::find(
            data.as_ptr() as u32,
            data.len() as u32,
            handle.as_mut_ptr() as u32
        );

        if res != 0 {
            return Err(res);
        }

        let mut cont = true;
        let mut raw = Vec::new();
        let handle = handle.assume_init();

        while cont {
            let (mut res_raw, res_cont) = read_handle(handle)?;

            raw.append(&mut res_raw);
            cont = res_cont;
        }

        raw::close(handle);

        rmp_serde::from_slice(&raw).or(Err(10))
    }
}

pub fn insert(collection: &str, docs: &Vec<bson::Document>) -> Result<HashMap<u32, bson::Bson>, u32> {
  let data = rmp_serde::to_vec(&(collection, docs)).unwrap();
  let mut handle : MaybeUninit<u32> = MaybeUninit::uninit();

  unsafe {
      let res = raw::insert(
          data.as_ptr() as u32,
          data.len() as u32,
          handle.as_mut_ptr() as u32
      );

      if res != 0 {
          return Err(res);
      }

      let mut cont = true;
      let mut raw = Vec::new();
      let handle = handle.assume_init();

      while cont {
          let (mut res_raw, res_cont) = read_handle(handle)?;

          raw.append(&mut res_raw);
          cont = res_cont;
      }

      raw::close(handle);

      rmp_serde::from_slice(&raw).or(Err(10))
  }
}

pub fn delete(collection: &str, doc: bson::Document) -> Result<(), u32> {
  let data = rmp_serde::to_vec(&(collection, doc)).unwrap();

  unsafe {
      let res = raw::delete(
          data.as_ptr() as u32,
          data.len() as u32
      );

      if res != 0 {
          return Err(res);
      }

      Ok(())
  }
}

pub fn update(collection: &str, filter: bson::Document, upd: bson::Document) -> Result<(), u32> {
  let data = rmp_serde::to_vec(&(collection, filter, upd)).unwrap();

  unsafe {
      let res = raw::update(
          data.as_ptr() as u32,
          data.len() as u32
      );

      if res != 0 {
          return Err(res);
      }

      Ok(())
  }
}

fn read_handle(handle: u32) -> Result<(Vec<u8>, bool), u32> {
    let mut cont_ptr : MaybeUninit<u32> = MaybeUninit::uninit();

    const BUF_LEN : usize = 4096;
    let buf = [0u8; BUF_LEN];

    unsafe {
        let res = raw::read(
            handle,
            buf.as_ptr() as u32,
            BUF_LEN as u32,
            cont_ptr.as_mut_ptr() as u32
        );

        if res == 0 {
            Ok((Vec::from(buf), cont_ptr.assume_init() == 1))
        } else {
            Err(res)
        }
    }
}
