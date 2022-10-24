use crate::op::Op;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct Dispatcher {
    url: String,
}

#[derive(Debug)]
pub enum Response<T> {
    Success(T),
    NetworkError(String),
    RequestError(u16, String),
    ParseError(String),
}

#[cfg(feature = "local-native")]
impl Dispatcher {
    pub fn create(url: String) -> Self {
        Dispatcher { url }
    }

    pub async fn send<T>(&self, op: &T) -> Response<T::Output>
    where
        T: Op + Serialize,
        T::Output: DeserializeOwned,
    {
        let response = reqwest::Client::new()
            .post(&self.url)
            .json(&(<T as Op>::id(), op))
            .send()
            .await
            .map_err(|e| e.to_string());

        match response {
            Err(e) => Response::NetworkError(e),
            Ok(resp) => {
                if resp.status().is_success() {
                    match resp.json::<T::Output>().await {
                        Ok(v) => Response::Success(v),
                        Err(e) => Response::ParseError(e.to_string()),
                    }
                } else {
                    Response::RequestError(
                        resp.status().as_u16(),
                        resp.text().await.ok().unwrap_or("".to_string()),
                    )
                }
            }
        }
    }
}

#[cfg(feature = "local-browser")]
impl Dispatcher {
    pub fn create(url: String) -> Self {
        Dispatcher { url }
    }

    pub async fn send<T>(&self, op: &T) -> Response<T::Output>
    where
        T: Op + Serialize,
        T::Output: DeserializeOwned,
    {
        let response = gloo_net::http::Request::post(&self.url)
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

#[cfg(feature = "debug")]
impl Dispatcher {
    pub fn create(url: String) -> Self {
        Dispatcher { url }
    }

    pub async fn send<T>(&self, op: &T) -> Response<T::Output>
    where
        T: Op + Serialize,
        T::Output: DeserializeOwned,
    {
        Response::Success(op.execute())
    }
}
