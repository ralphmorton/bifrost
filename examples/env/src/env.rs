use bifrost::op::Op;
use serde::{Deserialize, Serialize};
#[cfg(feature = "remote")]
use serde::de::DeserializeOwned;

#[derive(Debug, Deserialize, Serialize)]
struct GetEnvVar {
    var: String,
}

impl Op for GetEnvVar {
    type Output = Option<String>;

    fn id() -> &'static str {
        "get_env_var"
    }

    #[cfg(feature = "remote")]
    fn execute(&self) -> Self::Output {
        std::env::var(&self.var).ok()
    }
}

#[cfg(feature = "local")]
#[tokio::main]
async fn main() {
    use bifrost::dispatcher::Dispatcher;

    let heimdall_execute_url = String::from("http://localhost:8081/env-example/execute");
    let dispatcher = Dispatcher::create(heimdall_execute_url);

    println!("GetEnvVar:");
    let op = GetEnvVar {
        var: String::from("TEST_VARIABLE"),
    };
    let result = dispatcher.send(&op).await;
    println!("Got result: {:?}", result);
}

#[cfg(feature = "remote")]
bifrost::entrypoint!(GetEnvVar);
