use bifrost_example_yew::Increment;

#[cfg(feature = "local")]
use bifrost_example_yew::client::App;

#[cfg(feature = "remote")]
use bifrost::op::Op;
#[cfg(feature = "remote")]
use serde::Serialize;
use serde::de::DeserializeOwned;

#[cfg(feature = "local")]
fn main() {
  yew::start_app::<App>();
}

#[cfg(feature = "remote")]
bifrost::entrypoint!(Increment);
