#[cfg(feature = "local")]
pub mod dispatcher;
pub mod op;

#[macro_export]
macro_rules! entrypoint {
    ( $( $typ:ty ),* ) => {
        fn main() {
            use bifrost::op::Op;

            let args : Vec<String> = std::env::args().collect();

            let label = &args[0];
            let json = &args[1];

            $(
                if label.as_str() == <$typ>::id() {
                    __bifrost_dispatch::<$typ>(json);
                }
            )*
        }

        fn __bifrost_dispatch<T>(json: &String) where T : Op + DeserializeOwned, T::Output : Serialize {
            match __bifrost_exec::<T>(json) {
                Some(result) => print!("{}", result),
                None => ()
            }
        }

        fn __bifrost_exec<T>(json: &String) -> Option<String> where T : Op + DeserializeOwned, T::Output : Serialize {
            let op : T = serde_json::from_str(json).map_err(|e| e.to_string()).ok()?;
            let result = op.execute();
            serde_json::to_string(&result).ok()
        }
    };
}
