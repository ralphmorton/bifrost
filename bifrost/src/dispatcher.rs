
use crate::op::Op;
use serde::Serialize;
use serde::de::DeserializeOwned;

pub struct Dispatcher {
    url: String
}

impl Dispatcher {
    pub fn create(url: String) -> Self {
        Dispatcher {
            url
        }
    }

    pub async fn send<T>(&self, op: &T) -> Response<T::Output> where T : Op + Serialize, T::Output : DeserializeOwned {
        let response =
            reqwest::Client::new()
            .post(&self.url)
            .json(&(<T as Op>::id(), op))
            .send().await.map_err(|e| e.to_string());

        match response {
            Err(e) => Response::NetworkError(e),
            Ok(resp) => {
                if resp.status().is_success() {
                    match resp.json::<T::Output>().await {
                        Ok(v) => Response::Success(v),
                        Err(e) => Response::ParseError(e.to_string())
                    }
                } else {
                    Response::RequestError(
                        resp.status(),
                        resp.text().await.ok().unwrap_or("".to_string())
                    )
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum Response<T> {
    Success(T),
    NetworkError(String),
    RequestError(reqwest::StatusCode, String),
    ParseError(String)
}
