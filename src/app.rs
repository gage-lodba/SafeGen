use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

const MAX_HISTORY: usize = 20;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
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
struct ClipboardArgs {
    text: String,
}

fn do_generate(
    password_input: NodeRef,
    length_input: NodeRef,
    upper_input: NodeRef,
    lower_input: NodeRef,
    number_input: NodeRef,
    symbol_input: NodeRef,
    history_input: NodeRef,
) {
    spawn_local(async move {
        let Ok(length) = length_input
            .cast::<web_sys::HtmlInputElement>()
            .unwrap()
            .value()
            .parse::<u8>()
        else {
            return;
        };

        let args = GeneratorArgs {
            length,
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

        let password_el = password_input.cast::<web_sys::HtmlInputElement>().unwrap();

        match invoke("generate_password", to_value(&args).unwrap()).await {
            Ok(value) => {
                let Some(password) = value.as_string() else {
                    return;
                };
                password_el.set_value(&password);

                let history_el = history_input
                    .cast::<web_sys::HtmlTextAreaElement>()
                    .unwrap();
                let existing = history_el.value();
                let new_history: String = std::iter::once(password.as_str())
                    .chain(existing.lines().take(MAX_HISTORY - 1))
                    .collect::<Vec<_>>()
                    .join("\n");
                history_el.set_value(&new_history);
            }
            Err(err) => {
                let msg = err
                    .as_string()
                    .unwrap_or_else(|| "Failed to generate password.".into());
                password_el.set_value(&msg);
            }
        }
    });
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
            do_generate(
                password_input.clone(),
                length_input.clone(),
                upper_input.clone(),
                lower_input.clone(),
                number_input.clone(),
                symbol_input.clone(),
                history_input.clone(),
            );
        })
    };

    {
        let password_input = password_input.clone();
        let length_input = length_input.clone();
        let upper_input = upper_input.clone();
        let lower_input = lower_input.clone();
        let number_input = number_input.clone();
        let symbol_input = symbol_input.clone();
        let history_input = history_input.clone();

        use_effect_with((), move |_| {
            let cb = Closure::<dyn FnMut(_)>::new(move |e: web_sys::KeyboardEvent| {
                if e.key() == "Enter" {
                    e.prevent_default();
                    do_generate(
                        password_input.clone(),
                        length_input.clone(),
                        upper_input.clone(),
                        lower_input.clone(),
                        number_input.clone(),
                        symbol_input.clone(),
                        history_input.clone(),
                    );
                }
            });
            let window = web_sys::window().unwrap();
            window
                .add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref())
                .unwrap();
            move || {
                let _ = window
                    .remove_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
                drop(cb);
            }
        });
    }

    let clear_history = {
        let history_input = history_input.clone();
        let password_input = password_input.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            password_input
                .cast::<web_sys::HtmlInputElement>()
                .unwrap()
                .set_value("");

            history_input
                .cast::<web_sys::HtmlTextAreaElement>()
                .unwrap()
                .set_value("");
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

                if password.is_empty() {
                    return;
                }

                let args = ClipboardArgs { text: password };

                let _ = invoke(
                    "plugin:clipboard-manager|write_text",
                    to_value(&args).unwrap(),
                )
                .await;
            });
        })
    };

    let open_github = Callback::from(move |e: MouseEvent| {
        e.prevent_default();

        spawn_local(async move {
            let _ = invoke("open_github", JsValue::NULL).await;
        });
    });

    let update_length = {
        let length_label = length_label.clone();
        let length_input = length_input.clone();

        Callback::from(move |e: InputEvent| {
            e.prevent_default();

            let Ok(length) = length_input
                .cast::<web_sys::HtmlInputElement>()
                .unwrap()
                .value()
                .parse::<u8>()
            else {
                return;
            };

            length_label
                .cast::<web_sys::HtmlElement>()
                .unwrap()
                .set_inner_text(&format!("Password Length: {length}"));
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
            <textarea id="Notes" ref={history_input} readonly=true></textarea>

            <div>
                <button type="button" id="Github" onclick={open_github}>
                    <img src="public/github.svg" alt="" />
                    {"Github"}
                </button>
                <input type="button" id="Clear" value="Clear history" onclick={clear_history}/>
            </div>
        </>
    }
}
