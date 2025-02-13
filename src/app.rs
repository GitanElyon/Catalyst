#![allow(non_snake_case)]

use dioxus::{html::{button, img}, prelude::*};
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
        let result = invoke("open_file", JsValue::NULL)
            .await
            .as_string()
            .unwrap_or_default();
        document_content.set(result);
    };

    // Saves the current content using Tauri
    let save_file = move |_| async move {
        let content = document_content.read();
        let args = serde_wasm_bindgen::to_value(&SaveFileArgs { content: &*content }).unwrap();
        let _ = invoke("save_file", args).await;
    };

    // callbacks for window control actions
    let minimize_window = move |_| async move {
        let _ = invoke("minimize_window", JsValue::NULL).await;
    };
    let maximize_window = move |_| async move {
        let _ = invoke("maximize_window", JsValue::NULL).await;
    };
    let close_window = move |_| async move {
        let _ = invoke("close_window", JsValue::NULL).await;
    };

    // making the window draggable
    

    rsx! {
        link { rel: "stylesheet", href: "assets/styles.css" }
        main {
            class: "container code-editor-container",

            div {
                class: "toolbar top-toolbar",
                img {
                    "data-tauri-drag-region": "", // I don't know why this is how it is
                    class: "toolbar-button logo",
                    id: "corner-logo",
                    src: "assets/images/catalyst.svg",
                    alt: "Logo"
                }
                div {
                    class: "toolbar-button",
                    id: "file-button",
                    "File"
                    button {
                        class: "toolbar-sub-button",
                        onclick: move |_| document_content.set(String::new()),
                        "New File"
                    }
                    button {
                        class: "toolbar-sub-button",
                        onclick: open_file,
                        "Open File"
                    }
                    button {
                        class: "toolbar-sub-button",
                        onclick: save_file,
                        "Save File"
                    }
                }
                div {
                    class: "toolbar-button",
                    id: "edit-button",
                    "Edit"
                    button {
                        class: "toolbar-sub-button",
                        onclick: move |_| document_content.set(String::new()),
                        "Undo"
                    }
                    button {
                        class: "toolbar-sub-button",
                        onclick: move |_| document_content.set(String::new()),
                        "Redo"
                    }
                }
                div {
                    class: "toolbar-button",
                    id: "run-button",
                    "Run"
                }
                div {
                    class: "toolbar-button",
                    id: "help-button",
                    "Help"
                    button {
                        class: "toolbar-sub-button",
                        onclick: move |_| document_content.set(String::new()),
                        "Documentation"
                    }
                    button {
                        class: "toolbar-sub-button",
                        onclick: move |_| document_content.set(String::new()),
                        "About"
                    }
                }

            }

            div {
                class: "toolbar window-toolbar",
                img {
                    class: "toolbar-button window-button",
                    id: "minimize-window-button",
                    src: "assets/icons/minimize.svg",
                    alt: "Logo",
                    onclick: minimize_window,
                    "Minimize"
                }
                img {
                    class: "toolbar-button window-button",
                    id: "maximize-window-button",
                    src: "assets/icons/maximize.svg",
                    alt: "Logo",
                    onclick: maximize_window,
                    "Maximize"
                }
                img {
                    class: "toolbar-button window-button",
                    id: "close-window-button",
                    src: "assets/icons/close.svg",
                    alt: "Logo",
                    onclick: close_window,
                    "Close"
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
