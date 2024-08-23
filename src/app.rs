use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GeneratorArgs {
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
}

#[derive(Serialize, Deserialize)]
struct CopyArgs {
    label: String,
    text: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let password_input = use_node_ref();

    let length_input = use_node_ref();
    let length_label = use_node_ref();

    let upper_input = use_node_ref();
    let lower_input = use_node_ref();
    let number_input = use_node_ref();
    let symbol_input = use_node_ref();

    let history_input = use_node_ref();

    // Listener for "context menu" event
    // prevent default to stop it from showing

    let generate_password = {
        let password_input = password_input.clone();

        let length_input = length_input.clone();

        let upper_input = upper_input.clone();
        let lower_input = lower_input.clone();
        let number_input = number_input.clone();
        let symbol_input = symbol_input.clone();

        let history_input = history_input.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            let password_input = password_input.clone();

            let length_input = length_input.clone();

            let upper_input = upper_input.clone();
            let lower_input = lower_input.clone();
            let number_input = number_input.clone();
            let symbol_input = symbol_input.clone();

            let history_input = history_input.clone();

            spawn_local(async move {
                let args = GeneratorArgs {
                    length: length_input
                        .cast::<web_sys::HtmlInputElement>()
                        .unwrap()
                        .value()
                        .parse::<u8>()
                        .unwrap(),

                    upper: upper_input
                        .cast::<web_sys::HtmlInputElement>()
                        .unwrap()
                        .checked(),

                    lower: lower_input
                        .cast::<web_sys::HtmlInputElement>()
                        .unwrap()
                        .checked(),

                    number: number_input
                        .cast::<web_sys::HtmlInputElement>()
                        .unwrap()
                        .checked(),

                    symbol: symbol_input
                        .cast::<web_sys::HtmlInputElement>()
                        .unwrap()
                        .checked(),
                };

                let password = invoke("generate_password", to_value(&args).unwrap())
                    .await
                    .as_string()
                    .unwrap();

                password_input
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .set_value(&password);

                let history = history_input
                    .cast::<web_sys::HtmlTextAreaElement>()
                    .unwrap()
                    .value()
                    .trim()
                    .to_string();

                let new_history = format!("{}\n{}", password, history);

                history_input
                    .cast::<web_sys::HtmlTextAreaElement>()
                    .unwrap()
                    .set_value(&new_history);
            });
        })
    };

    let copy_to_clipboard = {
        let password_input = password_input.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            let password_input = password_input.clone();

            spawn_local(async move {
                let password = password_input
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value();

                let args = CopyArgs {
                    label: "Test".to_string(),
                    text: password.clone(),
                };

                invoke(
                    "plugin:clipboard-manager|write_text",
                    to_value(&args).unwrap(),
                )
                .await;
            });
        })
    };

    let open_github = {
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            spawn_local(async move {
                invoke("open_github", JsValue::NULL).await;
            });
        })
    };

    let update_length = {
        let length_label = length_label.clone();
        let length_input = length_input.clone();

        Callback::from(move |e: InputEvent| {
            e.prevent_default();

            let length = length_input
                .cast::<web_sys::HtmlInputElement>()
                .unwrap()
                .value()
                .parse::<u8>()
                .unwrap();

            length_label
                .cast::<web_sys::HtmlInputElement>()
                .unwrap()
                .set_inner_text(&format!("Password Length: {}", length));
        })
    };

    html! {
        <>
            <input type="text" ref={password_input} id="Password" placeholder="Generated password" readonly=true />

            <div>
                <input type="button" onclick={generate_password} value="Generate" />
                <input type="button" onclick={copy_to_clipboard} value="Copy" />
            </div>

            <label for="length" ref={length_label}>{"Password Length: 30"}</label>
            <input type="range" id="length" min="10" max="50" value="30" class="slider" ref={length_input} oninput={update_length}/>

            <div>
                <label class="container">{"A-Z"}
                    <input type="checkbox" id="Upper" ref={upper_input} checked=true />
                    <span class="checkmark"></span>
                </label>

                <label class="container">{"a-z"}
                    <input type="checkbox" id="Lower" ref={lower_input} checked=true />
                    <span class="checkmark"></span>
                </label>

                <label class="container">{"0-9"}
                    <input type="checkbox" id="Number" ref={number_input} checked=true />
                    <span class="checkmark"></span>
                </label>

                <label class="container">{"!@#$%^&*"}
                    <input type="checkbox" id="Symbol" ref={symbol_input} checked=true />
                    <span class="checkmark"></span>
                </label>
            </div>

            <label for="Notes">{"Password history"}</label>
            <textarea id="Notes" ref={history_input} rows="19" readonly=true></textarea>

            <input type="image" src="public/github.svg" id="Github" onclick={open_github} />
        </>
    }
}
