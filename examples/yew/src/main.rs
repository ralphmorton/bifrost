use bifrost::op::Op;

#[cfg(feature = "local")]
use bifrost_example_yew::client::App;

#[cfg(feature = "remote")]
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[cfg(feature = "local")]
fn main() {
  yew::start_app::<App>();
}

#[cfg(feature = "remote")]
bifrost::entrypoint!();
