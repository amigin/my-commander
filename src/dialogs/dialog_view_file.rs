use dioxus::prelude::*;

use crate::{dialogs::*, DataState};

#[component]
pub fn DialogViewFile(file_name: String) -> Element {
    let mut state = use_signal(|| ViewFileState::new(file_name.as_str()));
    let state_read_access = state.read();

    let mut image_style = "btn-light";
    let mut text_style = "btn-light";
    let mut hex_style = "btn-light";

    let content = match &state_read_access.content_type {
        ViewContentType::Image => {
            image_style = "btn-primary";

            rsx! {
                div { class: "view-file-pad", style: "overflow-y: auto;",
                    img { src: file_name.as_str() }
                }
            }
        }
        ViewContentType::Text => {
            let (content, err) = match &state_read_access.data {
                DataState::None => {
                    let file_name = file_name.clone();

                    spawn(async move {
                        state.write().data = DataState::Loading;
                        let content = load_file(file_name).await;
                        state.write().data = DataState::Loaded(content);
                    });

                    (None, rsx! { "Loading..." })
                }

                DataState::Loading => (None, rsx! { "Loading..." }),
                DataState::Loaded(content) => (Some(content), rsx! {}),
                DataState::Error(_) => (None, rsx! { "Error loading file" }),
            };

            text_style = "btn-primary";
            if let Some(content) = content {
                let text = std::str::from_utf8(&content);
                match text {
                    Ok(text) => {
                        rsx! {
                            div { class: "view-file-pad",
                                textarea { style: "width:100%; height:100%; border: none;resize: none; border-radius: 5px;font-family: monospace;",
                                    {text}
                                }
                            }
                        }
                    }
                    Err(err) => {
                        rsx! { "Error showing as text content. {err}" }
                    }
                }
            } else {
                err
            }
        }
        ViewContentType::Hex => {
            hex_style = "btn-primary";
            rsx! { "Showing hex content" }
        }
    };
    rsx! {
        DialogTemplate {
            window_size_style: "width: 95vw; height: 95vh;",
            title: "View file {file_name}",

            content: rsx! {
                div { style: "padding: 5px; text-align: right;",
                    div { class: "btn-group",
                        button {
                            class: "btn {image_style} btn-sm",
                            onclick: move |_| {
                                state.write().content_type = ViewContentType::Image;
                            },
                            {"Image"}
                        }
                        button {
                            class: "btn {text_style} btn-sm",
                            onclick: move |_| {
                                state.write().content_type = ViewContentType::Text;
                            },
                            {"Text"}
                        }
                        button {
                            class: "btn {hex_style} btn-sm",
                            onclick: move |_| {
                                state.write().content_type = ViewContentType::Hex;
                            },
                            {"Hex"}
                        }
                    }
                }
                {content}
            },
        }
    }
}

async fn load_file(file_name: String) -> Vec<u8> {
    tokio::fs::read(file_name).await.unwrap()
}

pub struct ViewFileState {
    pub content_type: ViewContentType,
    pub data: DataState<Vec<u8>>,
}

impl ViewFileState {
    pub fn new(file: &str) -> Self {
        let ext = file.split('.').last().unwrap_or("");
        Self {
            content_type: ViewContentType::detect_from_ext(ext.to_lowercase().as_str()),
            data: DataState::None,
        }
    }
}

pub enum ViewContentType {
    Image,
    Text,
    Hex,
}

impl ViewContentType {
    pub fn detect_from_ext(ext: &str) -> Self {
        match ext {
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "svg" | "mp4" | "pdf" => Self::Image,
            "txt" | "md" | "rs" | "toml" | "json" | "xml" | "html" | "htm" | "lock" | "yaml" => {
                Self::Text
            }
            _ => Self::Hex,
        }
    }
}
