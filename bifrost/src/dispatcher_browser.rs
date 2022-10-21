use crate::op::Op;
use gloo_net::http;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct Dispatcher {
    url: String,
}

impl Dispatcher {
    pub fn create(url: String) -> Self {
        Dispatcher { url }
    }

    pub async fn send<T>(&self, op: &T) -> Response<T::Output>
    where
        T: Op + Serialize,
        T::Output: DeserializeOwned,
    {
        let response = http::Request::post(&self.url)
            .json(&(<T as Op>::id(), op))
            .expect("can serialize payload")
            .send()
            .await;

        match response {
            Err(e) => Response::NetworkError(e.to_string()),
            Ok(resp) => {
                if resp.status() == 200 {
                    match resp.json::<T::Output>().await {
                        Ok(v) => Response::Success(v),
                        Err(e) => Response::ParseError(e.to_string()),
                    }
                } else {
                    Response::RequestError(
                        resp.status(),
                        resp.text().await.ok().unwrap_or("".to_string()),
                    )
                }
            },
        }
    }
}

#[derive(Debug)]
pub enum Response<T> {
    Success(T),
    NetworkError(String),
    RequestError(u16, String),
    ParseError(String),
}
