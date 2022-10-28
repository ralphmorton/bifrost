use bifrost::op::Op;
use serde::{Deserialize, Serialize};
#[cfg(feature = "remote")]
use serde::de::DeserializeOwned;

#[derive(Debug, Deserialize, Serialize)]
struct Query {
    i: i32,
}

impl Op for Query {
    type Output = Result<Vec<bson::Document>, u32>;

    fn id() -> &'static str {
        "query"
    }

    #[cfg(any(feature = "remote"))]
    fn execute(&self) -> Self::Output {
        let doc = bson::Document::new();
        bifrost_mongodb::find("test", &doc)
    }
}

#[cfg(any(feature = "local"))]
#[tokio::main]
async fn main() {
    use bifrost::dispatcher::Dispatcher;

    let heimdall_execute_url = String::from("http://localhost:8081/mongo-example/execute");
    let dispatcher = Dispatcher::create(heimdall_execute_url);

    println!("Query:");
    let op = Query {
        i: 3,
    };
    let result = dispatcher.send(&op).await;
    println!("Got result: {:?}", result);
}

#[cfg(feature = "remote")]
bifrost::entrypoint!(Query);
