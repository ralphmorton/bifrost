use bifrost::op::Op;
use serde::{Deserialize, Serialize};
#[cfg(feature = "remote")]
use serde::de::DeserializeOwned;

#[derive(Debug, Deserialize, Serialize)]
struct Greet {
    name: String,
}

impl Op for Greet {
    type Output = String;

    fn id() -> &'static str {
        "greet"
    }

    #[cfg(any(feature = "remote", feature = "debug"))]
    fn execute(&self) -> Self::Output {
        format!("Hi there, {}", self.name)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct AddOne {
    i: i32,
}

impl Op for AddOne {
    type Output = i32;

    fn id() -> &'static str {
        "add_one"
    }

    #[cfg(any(feature = "remote", feature = "debug"))]
    fn execute(&self) -> Self::Output {
        self.i + 1
    }
}

#[cfg(any(feature = "local", feature = "debug"))]
#[tokio::main]
async fn main() {
    use bifrost::dispatcher::Dispatcher;

    let heimdall_execute_url = String::from("http://localhost:8080/greet-example/execute");
    let dispatcher = Dispatcher::create(heimdall_execute_url);

    println!("Greet:");
    let op = Greet {
        name: String::from("Bifrost"),
    };
    let result = dispatcher.send(&op).await;
    println!("Got result: {:?}", result);

    println!("Add one:");
    let op = AddOne { i: 41 };
    let result = dispatcher.send(&op).await;
    println!("Got result: {:?}", result);
}

#[cfg(feature = "remote")]
bifrost::entrypoint!(Greet, AddOne);
