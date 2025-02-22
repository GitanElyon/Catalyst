#![allow(non_snake_case)]

use dioxus::{
    html::{button, img},
    prelude::*, web,
};
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
    let mut file_dropdown_open = use_signal(|| false);
    let mut edit_dropdown_open = use_signal(|| false);
    let mut run_dropdown_open = use_signal(|| false);
    let mut help_dropdown_open = use_signal(|| false);

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

    // toolbar dropdown logic
    let toggle_file_dropdown = move |_| {
        let current = *file_dropdown_open.read();
        let new_state = !current;
        file_dropdown_open.set(new_state);
    };

    let toggle_edit_dropdown = move |_| {
        let current = *edit_dropdown_open.read();
        let new_state = !current;
        edit_dropdown_open.set(new_state);
    };

    let toggle_run_dropdown = move |_| {
        let current = *run_dropdown_open.read();
        let new_state = !current;
        run_dropdown_open.set(new_state);
    };

    let toggle_help_dropdown = move |_| {
        let current = *help_dropdown_open.read();
        let new_state = !current;
        help_dropdown_open.set(new_state);
    };

    let close_dropdowns = move |_| {
        file_dropdown_open.set(false);
        edit_dropdown_open.set(false);
        run_dropdown_open.set(false);
        help_dropdown_open.set(false);
        // print to console for debugging
        
    };

    rsx! {
        link { rel: "stylesheet", href: "assets/styles.css" }
        main {
            class: "container code-editor-container",
            onmousedown: close_dropdowns,

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
                    onclick: toggle_file_dropdown,
                    "File"
                    div {
                        onmousedown: |e| e.stop_propagation(),
                        class: if *file_dropdown_open.read() { "dropdown-menu open" } else { "dropdown-menu" },
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
                }
                div {
                    class: "toolbar-button",
                    id: "edit-button",
                    onclick: toggle_edit_dropdown,
                    "Edit"
                    div {
                        onmousedown: |e| e.stop_propagation(),
                        class: if *edit_dropdown_open.read() { "dropdown-menu open" } else { "dropdown-menu" },
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
                        button {
                            class: "toolbar-sub-button",
                            onclick: move |_| document_content.set(String::new()),
                            "Cut"
                        }
                        button {
                            class: "toolbar-sub-button",
                            onclick: move |_| document_content.set(String::new()),
                            "Copy"
                        }
                        button {
                            class: "toolbar-sub-button",
                            onclick: move |_| document_content.set(String::new()),
                            "Paste"
                        }
                    }
                }
                div {
                    class: "toolbar-button",
                    id: "run-button",
                    onclick: toggle_run_dropdown,
                    "Run"
                    div {
                        onmousedown: |e| e.stop_propagation(),
                        class: if *run_dropdown_open.read() { "dropdown-menu open" } else { "dropdown-menu" },
                        button {
                            class: "toolbar-sub-button",
                            onclick: move |_| document_content.set(String::new()),
                            "Run"
                        }
                        button {
                            class: "toolbar-sub-button",
                            onclick: move |_| document_content.set(String::new()),
                            "Debug"
                        }
                        button {
                            class: "toolbar-sub-button",
                            onclick: move |_| document_content.set(String::new()),
                            "Stop"
                        }
                    }
                }
                div {
                    class: "toolbar-button",
                    id: "help-button",
                    onclick: toggle_help_dropdown,
                    "Help"
                    div {
                        onmousedown: |e| e.stop_propagation(),
                        class: if *help_dropdown_open.read() { "dropdown-menu open" } else { "dropdown-menu" },
                        button {
                            class: "toolbar-sub-button",
                            onclick: move |_| document_content.set(String::new()),
                            "Documentation"
                        }
                        button {
                            class: "toolbar-sub-button",
                            onclick: move |_| document_content.set(String::new()),
                            "About Catalyst"
                        }
                        button {
                            class: "toolbar-sub-button",
                            onclick: move |_| document_content.set(String::new()),
                            "Preferences"
                        }
                        button {
                            class: "toolbar-sub-button",
                            onclick: move |_| document_content.set(String::new()),
                            "Check for Updates"
                        }
                    }
                }
            }

            div {
                class: "toolbar search-toolbar",
                input {
                    class: "search-bar",
                    type: "text",
                    placeholder: "Search...",
                    oninput: |event| {
                        let value = event.value();
                        // print to console for debugging
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
                class: "toolbar bottom-toolbar",
                div {
                    class: "toolbar-button",
                    id: "problem-button",
                    onclick: move |_| document_content.set(String::new()),
                    "Problems"
                }
                div {
                    class: "toolbar-button",
                    id: "output-button",
                    onclick: move |_| document_content.set(String::new()),
                    "Output"
                }
                div {
                    class: "toolbar-button",
                    id: "terminal-button",
                    onclick: move |_| document_content.set(String::new()),
                    "Terminal"
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
