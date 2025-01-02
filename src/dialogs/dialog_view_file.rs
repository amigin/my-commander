use std::io::Read;

use dioxus::prelude::*;

use crate::{dialogs::*, DataState, MainState};

#[component]
pub fn DialogViewFile(file_name: String) -> Element {
    let mut state = use_signal(|| ViewFileState::new());
    let state_read_access = state.read();

    let content_type = match state_read_access.content_type.as_ref() {
        DataState::None => {
            let file_name_spawned = file_name.clone();
            spawn(async move {
                state.write().content_type.set_loading();
                let content_type = detect_file_content(file_name_spawned.as_str());
                state.write().content_type = content_type.into();
            });
            return render_view(&file_name, rsx! { "Detecting content type..." });
        }
        DataState::Loading => {
            return render_view(&file_name, rsx! { "Detecting content type..." });
        }
        DataState::Loaded(content_type) => content_type,
        DataState::Error(err) => {
            return render_view(&file_name, rsx! { "Error detecting content type: {err}" });
        }
    };

    let mut image_style = "btn-light";
    let mut text_style = "btn-light";
    let mut hex_style = "btn-light";

    let content = match content_type {
        ViewContentType::Image => {
            image_style = "btn-primary";

            rsx! {
                div { class: "view-file-pad", style: "overflow-y: auto;",
                    img { src: file_name.as_str() }
                }
            }
        }
        ViewContentType::Text => {
            let content = match &state_read_access.data {
                DataState::None => {
                    let file_name_spawned = file_name.clone();

                    spawn(async move {
                        state.write().data = DataState::Loading;
                        match load_file(file_name_spawned).await {
                            Ok(content) => {
                                state.write().data = DataState::Loaded(content);
                            }
                            Err(err) => {
                                state.write().data = DataState::Error(err);
                            }
                        }
                    });

                    return render_view(&file_name, rsx! { "Loading..." });
                }

                DataState::Loading => {
                    return render_view(&file_name, rsx! { "Loading..." });
                }
                DataState::Loaded(content) => content,
                DataState::Error(err) => {
                    return render_view(&file_name, rsx! { "Error loading file: {err}" });
                }
            };

            text_style = "btn-primary";

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
        }
        ViewContentType::Hex => {
            hex_style = "btn-primary";
            rsx! { "Showing hex content" }
        }
    };

    let content = rsx! {
        div { style: "padding: 5px; text-align: right;",
            div { class: "btn-group",
                button {
                    class: "btn {image_style} btn-sm",
                    onclick: move |_| {
                        state.write().content_type = ViewContentType::Image.into();
                    },
                    onkeypress: |ctx| {
                        if ctx.key() == Key::Escape {
                            consume_context::<Signal<MainState>>().write().dialog = None;
                        }
                    },
                    {"Image"}
                }
                button {
                    class: "btn {text_style} btn-sm",
                    onclick: move |_| {
                        state.write().content_type = ViewContentType::Text.into();
                    },
                    onkeypress: |ctx| {
                        if ctx.key() == Key::Escape {
                            consume_context::<Signal<MainState>>().write().dialog = None;
                        }
                    },
                    {"Text"}
                }
                button {
                    class: "btn {hex_style} btn-sm",
                    onclick: move |_| {
                        state.write().content_type = ViewContentType::Hex.into();
                    },
                    onkeypress: |ctx| {
                        if ctx.key() == Key::Escape {
                            consume_context::<Signal<MainState>>().write().dialog = None;
                        }
                    },
                    {"Hex"}
                }
            }
        }
        {content}
    };

    render_view(&file_name, content)
}

fn render_view(file_name: &str, content: Element) -> Element {
    rsx! {
        DialogTemplate {
            window_size_style: "width: 95vw; height: 95vh;",
            title: "View file {file_name}",
            content,
        }
    }
}

async fn load_file(file_name: String) -> Result<Vec<u8>, String> {
    {
        let meta_data = tokio::fs::metadata(file_name.as_str())
            .await
            .map_err(|err| format!("Error reading file: {err}"))?;

        if meta_data.len() > 1024 * 1024 * 5 {
            return Err("File is too big to load".to_string());
        }
    }
    tokio::fs::read(file_name)
        .await
        .map_err(|err| format!("Error reading file: {err}"))
}

pub struct ViewFileState {
    pub content_type: DataState<ViewContentType>,
    pub data: DataState<Vec<u8>>,
}

impl ViewFileState {
    pub fn new() -> Self {
        Self {
            content_type: DataState::None,
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
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "svg" | "mp4" | "pdf" | "mov" => {
                Self::Image
            }
            "txt" | "md" | "rs" | "toml" | "json" | "xml" | "html" | "htm" | "lock" | "yaml" => {
                Self::Text
            }
            _ => Self::Hex,
        }
    }

    pub fn is_hex(&self) -> bool {
        matches!(self, Self::Hex)
    }
}

fn detect_file_content(file_name: &str) -> ViewContentType {
    let file_ext = file_name.split('.').last().unwrap_or("");
    let result = ViewContentType::detect_from_ext(file_ext.to_lowercase().as_str());

    if !result.is_hex() {
        return result;
    }

    let mut to_download = Vec::with_capacity(1024);

    unsafe {
        to_download.set_len(1024);
    }

    let file = std::fs::File::open(file_name);

    if file.is_err() {
        return ViewContentType::Hex;
    }

    let mut file = file.unwrap();

    let size = file.read(&mut to_download);

    if size.is_err() {
        return ViewContentType::Hex;
    }

    let size = size.unwrap();

    println!("Size: {size}");

    if size == 0 {
        return ViewContentType::Text;
    }

    let text = std::str::from_utf8(&to_download[0..size]);

    if text.is_ok() {
        return ViewContentType::Text;
    }

    ViewContentType::Hex
}
