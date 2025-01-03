use dioxus::prelude::*;
use tokio::io::{AsyncReadExt, AsyncSeekExt};

use crate::DataState;

const PAGE_SIZE: u64 = 1024;

#[component]
pub fn ViewHex(file_name: String, file_size: u64) -> Element {
    let mut state = use_signal(|| ViewHexState::new(file_size));

    let state_read_access = state.read();

    let data = match &state_read_access.page_content {
        DataState::None => {
            let page_no = state_read_access.page_no;

            spawn(async move {
                state.write().page_content = DataState::Loading;

                let result = load_page(page_no, file_name.as_str()).await;
                match result {
                    Ok(data) => {
                        state.write().page_content = DataState::Loaded(data);
                    }
                    Err(err) => {
                        state.write().page_content = DataState::Error(err);
                    }
                }
            });
            return rsx! { "Loading page... {state_read_access.page_no}" };
        }
        DataState::Loading => return rsx! { "Loading page... {state_read_access.page_no}" },
        DataState::Loaded(data) => data,
        DataState::Error(err) => {
            return rsx! {
                div { "Error loading page. Err: {err}" }
            }
        }
    };

    let mut to_render = Vec::new();

    let mut no = 0;
    let mut line = Vec::new();
    let mut text_line = Vec::new();
    let mut address = state_read_access.page_no * PAGE_SIZE;
    let mut line_no = address / 16;
    let mut b_no = 0;
    for b in data {
        if state_read_access.is_selected(b_no) {
            line.push(rsx! {
                span {
                    style: "cursor:pointer",
                    onclick: move |_| {
                        state.write().selected_no = Some(b_no);
                    },
                    class: "hex-symb-selected",
                    {format!("{:02x} ", b).as_str()}
                }
            });
        } else {
            line.push(rsx! {
                span {
                    style: "cursor:pointer",
                    onclick: move |_| {
                        state.write().selected_no = Some(b_no);
                    },
                    {format!("{:02x} ", b).as_str()}
                }
            });
        }

        text_line.push(generate_char(
            b_no,
            *b,
            state_read_access.is_selected(b_no),
            state,
        ));

        no += 1;

        if no == 16 {
            no = 0;
            to_render.push(render_line(line_no, address, line, text_line));

            line = Vec::new();
            text_line = Vec::new();
            address += 16;
            line_no += 1;
        }

        b_no += 1;
    }

    if line.len() > 0 {
        to_render.push(render_line(line_no, address, line, text_line));
    }

    rsx! {
        table { style: "margin: auto",
            tr {
                td {
                    button {
                        class: "btn btn-sm btn-light",
                        onclick: move |_| {
                            state.write().prev_page();
                        },
                        "<"
                    }
                }
                td {
                    input {
                        class: "from-control-sm",
                        style: "width:150px",
                        oninput: move |ctx| {
                            state.write().set_page_str(ctx.value());
                        },
                        onkeypress: move |ctx| {
                            match ctx.key() {
                                Key::Enter => {
                                    state.write().apply_page_str();
                                }
                                Key::ArrowUp => {
                                    state.write().prev_page();
                                }
                                Key::ArrowDown => {
                                    state.write().next_page();
                                }
                                _ => {}
                            }
                        },
                        value: state_read_access.page_no_str.as_str(),
                    }
                }
                td {
                    button {
                        class: "btn btn-sm btn-light",
                        onclick: move |_| {
                            state.write().next_page();
                        },
                        ">"
                    }
                }
            }
        }
        div {
            class: "view-file-pad",
            style: "font-family: monospace; font-size:14px; overflow-y: auto;",
            table { {to_render.into_iter()} }
        }
    }
}

fn render_line(
    line_no: u64,
    address: u64,
    hex_line: Vec<Element>,
    text_line: Vec<Element>,
) -> Element {
    rsx! {
        tr { class: "hex-view-line",
            td { {line_no.to_string()} }
            td {
                div { style: "width:10px" }
            }
            td {
                div { style: "cursor:default", title: "{address}", {format!("{:08x} ", address)} }
            }
            td {
                div { style: "width:10px" }
            }
            td { {hex_line.into_iter()} }
            td {
                div { style: "width:10px" }
            }
            {text_line.into_iter()}
        }
    }
}

pub struct ViewHexState {
    file_size: u64,
    page_no: u64,
    page_content: DataState<Vec<u8>>,
    page_no_str: String,
    selected_no: Option<usize>,
}

impl ViewHexState {
    pub fn new(file_size: u64) -> Self {
        Self {
            file_size,
            page_no: 0,
            page_content: DataState::None,
            page_no_str: "0".to_string(),
            selected_no: None,
        }
    }

    pub fn is_selected(&self, no: usize) -> bool {
        match self.selected_no {
            Some(selected_no) => selected_no == no,
            None => false,
        }
    }

    pub fn prev_page(&mut self) {
        if self.page_no > 0 {
            self.page_no -= 1;
            self.page_no_str = self.page_no.to_string();
            self.page_content = DataState::None;
        }
    }

    pub fn next_page(&mut self) {
        if self.page_no * PAGE_SIZE < self.file_size {
            self.page_no += 1;
            self.page_no_str = self.page_no.to_string();
            self.page_content = DataState::None;
        }
    }

    pub fn set_page_str(&mut self, value: String) {
        self.page_no_str = value;
    }

    pub fn apply_page_str(&mut self) {
        self.page_no = self.page_no_str.parse().unwrap_or(0);
        if self.page_no * PAGE_SIZE > self.file_size {
            self.page_no = self.file_size / PAGE_SIZE;
            self.page_no_str = self.page_no.to_string();
        }
        self.page_content = DataState::None;
    }
}

async fn load_page(page_no: u64, file_name: &str) -> Result<Vec<u8>, String> {
    let page_size = PAGE_SIZE as usize;
    let mut buffer = Vec::with_capacity(page_size);
    unsafe { buffer.set_len(page_size) };

    let mut file = tokio::fs::File::open(file_name)
        .await
        .map_err(|err| format!("Error opening file: {err}"))?;

    let offset = page_no * PAGE_SIZE;
    file.seek(tokio::io::SeekFrom::Start(offset))
        .await
        .map_err(|err| format!("Error seeking file: {err}"))?;

    let read = file
        .read(&mut buffer)
        .await
        .map_err(|err| format!("Error reading file: {err}"))?;

    println!("Hex ReadSize: {read}");

    buffer.truncate(read);

    Ok(buffer)
}

fn generate_char(no: usize, b: u8, selected: bool, mut state: Signal<ViewHexState>) -> Element {
    let selected_class = if selected { "hex-symb-selected" } else { "" };

    if b.is_ascii_graphic() {
        let char = &[b];
        unsafe {
            let str = std::str::from_utf8_unchecked(char);
            return rsx! {
                td {
                    div {
                        class: "hex_symb {selected_class}",
                        onclick: move |_| {
                            state.write().selected_no = Some(no);
                        },
                        {str}
                    }
                }
            };
        }
    }

    match b {
        0 => {
            rsx! {
                td {
                    div {
                        class: "hex_symb asci-symb {selected_class}",
                        onclick: move |_| {
                            state.write().selected_no = Some(no);
                        },
                        {"NL"}
                    }
                }
            }
        }
        1 => {
            rsx! {
                td {
                    div {
                        class: "hex_symb asci-symb {selected_class}",
                        onclick: move |_| {
                            state.write().selected_no = Some(no);
                        },
                        {"SH"}
                    }
                }
            }
        }
        2 => {
            rsx! {
                td {
                    div {
                        class: "hex_symb asci-symb {selected_class}",
                        onclick: move |_| {
                            state.write().selected_no = Some(no);
                        },
                        {"ST"}
                    }
                }
            }
        }
        3 => {
            rsx! {
                td {
                    div {
                        class: "hex_symb asci-symb {selected_class}",
                        onclick: move |_| {
                            state.write().selected_no = Some(no);
                        },
                        {"ET"}
                    }
                }
            }
        }
        10 => {
            rsx! {
                td {
                    div {
                        class: "hex_symb asci-symb {selected_class}",
                        onclick: move |_| {
                            state.write().selected_no = Some(no);
                        },
                        {"CR"}
                    }
                }
            }
        }

        13 => {
            rsx! {
                td {
                    div {
                        class: "hex_symb asci-symb {selected_class}",
                        onclick: move |_| {
                            state.write().selected_no = Some(no);
                        },
                        {"CL"}
                    }
                }
            }
        }

        _ => {
            rsx! {
                td {
                    div {
                        class: "hex_symb {selected_class}",
                        onclick: move |_| {
                            state.write().selected_no = Some(no);
                        },
                        {"."}
                    }
                }
            }
        }
    }
}
