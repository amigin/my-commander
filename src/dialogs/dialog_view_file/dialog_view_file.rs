use dioxus::prelude::*;
use tokio::io::AsyncReadExt;

use super::*;
use crate::{dialogs::*, DataState, MainState};

#[component]
pub fn DialogViewFile(file_name: String) -> Element {
    let mut state = use_signal(|| ViewFileState::new());
    let state_read_access = state.read();

    let file_size = state_read_access.file_size;

    let content_type = match state_read_access.content_type.as_ref() {
        DataState::None => {
            let file_name_spawned = file_name.clone();
            spawn(async move {
                state.write().content_type.set_loading();
                match detect_file_content(file_name_spawned.as_str()).await {
                    Ok((content_type, file_size)) => {
                        state.write().content_type = content_type.into();
                        state.write().file_size = file_size;
                    }
                    Err(err) => {
                        state.write().content_type = DataState::Error(err);
                    }
                }
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
                div {
                    class: "view-file-pad square-pattern",
                    style: "overflow-y: auto;",
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
            rsx! {
                ViewHex { file_name: file_name.as_str(), file_size }
            }
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
    pub file_size: u64,
}

impl ViewFileState {
    pub fn new() -> Self {
        Self {
            content_type: DataState::None,
            data: DataState::None,
            file_size: 0,
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

async fn detect_file_content(file_name: &str) -> Result<(ViewContentType, u64), String> {
    let file_size = get_file_size(file_name).await?;
    let file_ext = file_name.split('.').last().unwrap_or("");
    let result = ViewContentType::detect_from_ext(file_ext.to_lowercase().as_str());

    if !result.is_hex() {
        return Ok((result, file_size));
    }

    let mut to_download = Vec::with_capacity(1024);

    unsafe {
        to_download.set_len(1024);
    }

    let mut file = tokio::fs::File::open(file_name)
        .await
        .map_err(|err| format!("Error opening file: {err}"))?;

    let read_size = file
        .read(&mut to_download)
        .await
        .map_err(|err| format!("Error reading file: {err}"))?;

    if read_size == 0 {
        return Ok((ViewContentType::Text, file_size));
    }

    let text = std::str::from_utf8(&to_download[0..read_size]);

    if text.is_ok() {
        return Ok((ViewContentType::Text, file_size));
    }

    Ok((ViewContentType::Hex, file_size))
}

async fn get_file_size(file_name: &str) -> Result<u64, String> {
    let meta_data = tokio::fs::metadata(file_name)
        .await
        .map_err(|err| format!("Error reading file: {err}"))?;
    Ok(meta_data.len())
}
