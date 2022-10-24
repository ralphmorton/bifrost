use crate::Increment;
use bifrost::dispatcher::{Dispatcher, Response};
use std::sync::Arc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

struct State {
    value: Increment,
    error: Option<String>
}

#[function_component(App)]
pub fn app() -> Html {
    let dispatcher = Arc::new(Dispatcher::create("http://localhost:8081/yew-example/execute".to_string()));

    let initial_state = State {
        value: Increment { i: 0 },
        error: None
    };

    let state = use_state(|| initial_state);

    let increment = {
        let state = state.clone();

        Callback::once(move |_| inc(dispatcher, state.clone()))
    };

    html! {
        <div>
            <h1>{state.value.i}</h1>
            <div style="mb-3">{format!("{:?}", &state.error)}</div>
            <button onclick={increment}>
                { "+" }
            </button>
        </div>
    }
}

fn inc(dispatcher: Arc<Dispatcher>, state: UseStateHandle<State>) {
    spawn_local(async move {
        let response = dispatcher.send(&state.value).await;

        match response {
            Response::Success(v) => {
                state.set(
                    State {
                        value: Increment { i: v },
                        error: None
                    }
                )
            },
            Response::NetworkError(e) => {
                state.set(
                    State {
                        value: Increment { i: state.value.i },
                        error: Some(format!("Network error: {}", e))
                    }
                )
            },
            Response::RequestError(status_code, e) => {
                state.set(
                    State {
                        value: Increment { i: state.value.i },
                        error: Some(format!("Request error: {}/{}", status_code, e))
                    }
                )
            },
            Response::ParseError(e) => {
                state.set(
                    State {
                        value: Increment { i: state.value.i },
                        error: Some(format!("Parse error: {}", e))
                    }
                )
            }
        }
    });
}
