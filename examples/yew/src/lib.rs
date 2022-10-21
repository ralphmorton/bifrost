#[cfg(feature = "local")]
pub mod client;

use bifrost::op::Op;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Increment {
    i: i64
}

impl Op for Increment {
    type Output = i64;

    fn id() -> &'static str {
      "increment"
    }

    #[cfg(feature = "remote")]
    fn execute(&self) -> Self::Output {
        self.i + 1
    }
}
