use bifrost::op::Op;
use serde::{Deserialize, Serialize};
#[cfg(feature = "remote")]
use serde::de::DeserializeOwned;

#[derive(Debug, Deserialize, Serialize)]
struct Query {
    i: i32,
}

impl Op for Query {
    type Output = Result<(std::collections::HashMap<u32, bson::Bson>, Vec<bson::Document>), u32>;

    fn id() -> &'static str {
        "query"
    }

    #[cfg(any(feature = "remote"))]
    fn execute(&self) -> Self::Output {
        let mut insert1 = bson::Document::new();
        insert1.insert("foo", 1);
        insert1.insert("bar", "inserted via bifrost");
        let mut insert2 = bson::Document::new();
        insert2.insert("foo", 2);
        insert2.insert("bar", "inserted via bifrost");
        let inserts = vec![insert1, insert2];

        let insert_res = bifrost_mongodb::insert("test", &inserts)?;

        let doc = bson::Document::new();
        let find_res = bifrost_mongodb::find("test", &doc)?;

        Ok((insert_res, find_res))
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
