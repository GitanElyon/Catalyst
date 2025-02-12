#![allow(non_snake_case)]

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Generic Tauri invoke to interact with native commands
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

// Arguments for saving a file
#[derive(Serialize, Deserialize)]
struct SaveFileArgs<'a> {
    content: &'a str,
}

pub fn App() -> Element {
    let mut document_content = use_signal(|| String::new());

    // Opens a file using Tauri, receives its contents and updates the editor
    let open_file = move |_| async move {
        // Call the Tauri command "open_file"
        let result = invoke("open_file", JsValue::NULL)
            .await
            .as_string()
            .unwrap_or_default();
        document_content.set(result);
    };

    // Saves the current content using Tauri
    let save_file = move |_| async move {
        let content = document_content.read();
        let args =
            serde_wasm_bindgen::to_value(&SaveFileArgs { content: &*content }).unwrap();
        // Call the Tauri command "save_file" (native side implementation required)
        let _ = invoke("save_file", args).await;
    };

    rsx! {
        link { rel: "stylesheet", href: "assets/styles.css" }
        main {
            class: "container code-editor-container",
    
            div {
                class: "toolbar",
                button {
                    class: "toolbar-button",
                    onclick: open_file,
                    "Open File"
                }
                button {
                    class: "toolbar-button",
                    onclick: save_file,
                    "Save File"
                }
            }
    
            div {
                class: "editor",
                textarea {
                    id: "document-editor",
                    class: "editor-textarea",
                    placeholder: "Edit your code here...",
                    value: "{document_content}",
                    oninput: move |event| document_content.set(event.value())
                }
            }
        }
    }
}