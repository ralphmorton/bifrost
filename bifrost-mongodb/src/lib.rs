pub fn query(i: i32) -> i32 {
    #[link(wasm_import_module = "bifrost_mongodb")]
    extern "C" {
        fn query(i: i32) -> i32;
    }

    unsafe {
      query(i)
    }
}
