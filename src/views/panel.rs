use std::collections::BTreeMap;

use dioxus::prelude::*;
use rust_extensions::StrOrString;

use crate::{states::*, views::*};

#[component]
pub fn Panel(left_panel: bool) -> Element {
    let mut main_state = use_context::<Signal<MainState>>();

    let main_state_read_access = main_state.read();

    let panel_state = main_state_read_access.get_panel_state(left_panel);

    let mut total_folders = 0;
    let mut total_files = 0;
    let mut total_size = 0;

    let selected_file_index  = panel_state.selected_file_index;
    let mut total_items = 0;

    let mut selected_file_type:Option<FileLineType> = None;

    let files = match &panel_state.files {
        DataState::None => {
            let volume = panel_state.selected_volume.to_string();
            let selected_path = panel_state.selected_path.clone();
            spawn(async move {
                main_state
                    .write()
                    .get_panel_state_mut(left_panel)
                    .files
                    .set_loading();
                let files = load_files(volume.as_str(), selected_path.as_str()).await;
                match files {
                    Ok(files) => main_state
                        .write()
                        .get_panel_state_mut(left_panel)
                        .files
                        .set_loaded(files),
                    Err(err) => main_state
                        .write()
                        .get_panel_state_mut(left_panel)
                        .files
                        .set_error(err),
                }
            });
            rsx! { "Loading files..." }
        }
        DataState::Loading => rsx! { "Loading files..." },
        DataState::Loaded(value) => {
            total_files = value.files_amount;
            total_size = value.total_size;
            total_folders = value.folders_amount;
            total_items = value.files.len();
         
            let mut no = 0;
            let files = value.files.iter().map(|file_info| {

                let item_selected = panel_state.is_file_selected(no);
                let class_selected = if main_state_read_access.left_panel_active == left_panel{
                    if item_selected {
                        "selected-line"
                    } else {
                        ""
                    }
                }else{
                    if item_selected {
                        "selected-line-not-focused"
                    } else {
                        ""
                    }
                };

                if item_selected  {
                    selected_file_type = Some(file_info.tp);
                }

                let (icon, created, modified, file_size) = match file_info.tp{
                    FileLineType::Dir => {
                        let icon = rsx! {
                            img {
                                class: "file-ico",
                                src: asset!("/assets/ico/folder.svg"),
                            }
                        };

                        let created = file_info.created.to_rfc5322();
                        let modified = file_info.modified.to_rfc5322();
                        (icon, created, modified, crate::utils::format_bytes(file_info.size))
                    },
                    FileLineType::File => 
                    {
                      let icon = render_icon(file_info.name.as_str());
                      let created = file_info.created.to_rfc5322();
                      let modified = file_info.modified.to_rfc5322();

                      (icon, created, modified, crate::utils::format_bytes(file_info.size))
                    },
                    FileLineType::Back => {
                        let icon = rsx! {
                            img {
                                class: "file-ico",
                                src: asset!("/assets/ico/back.svg"),
                            }
                        } ;


                        (icon, String::new(), String::new(), String::new())
                    },
                };
           

                let hidden_attr = if file_info.hidden { "hidden" } else { "" };

                let created =if created.len()>10 {
                    &created[..20]
                }else{
                    created.as_str()
                };


                let modified =if modified.len()>10 {
                    &modified[..20]
                }else{
                    modified.as_str()
                };

                let item_file_type = file_info.tp;

                    let result = rsx! {
                        tr {
                            class: "{class_selected} {hidden_attr}",
                            onclick: move |_| {
                                main_state.write().get_panel_state_mut(left_panel).set_selected_file(no);
                            },

                            ondoubleclick: move |_| {
                                match item_file_type {
                                    FileLineType::Dir => {
                                        main_state.write().get_panel_state_mut(left_panel).press_enter();
                                    }
                                    FileLineType::Back => {
                                        main_state.write().get_panel_state_mut(left_panel).press_enter();
                                    }
                                    FileLineType::File => {}
                                }
                            },
                            td { {icon} }
                            td {
                                div { class: "file", {file_info.name.as_str()} }
                            }
                            td {
                                div { class: "file", {file_size} }
                            }
                            td {
                                div { class: "file-date", {created} }
                            }
                            td {
                                div { class: "file-date", {modified} }
                            }
                        }
                    };
                    no+=1;

                    result

                
            });

            rsx! {
                {files}
            }
        }
        DataState::Error(err) => rsx! {
            div { style: "text-align:center",
                div { "Error loading files: {err}" }
                button {
                    class: "btn btn-secondary btn-sm",
                    onclick: move |_| {
                        main_state.write().get_panel_state_mut(left_panel).go_back();
                    },
                    "Back"
                }
            }
        },
    };

    let disks = main_state_read_access.mounts.iter().map(|disk| {
        let name = match disk.name.as_ref() {
            Some(name) =>  name,
            None => disk.path.to_str().unwrap_or(""),
        };

        let name_to_show: StrOrString<'_> = if let Some(avail) = disk.avail{
            if let Some(size) = disk.size{
                format!("{} ({} of {})", name, crate::utils::format_bytes(avail) , crate::utils::format_bytes(size)).into()
            }else{
                name.into()
            }
        }else{
            name.into()
        };

        rsx! {
            option { selected: panel_state.selected_volume.as_str() == name, {name_to_show.as_str()} }
        }
    });

    rsx! {
        div { class: "top-panel",
            select {
                class: "form-select",
                oninput: move |ctx| {
                    main_state
                        .write()
                        .get_panel_state_mut(left_panel)
                        .set_selected_volume(ctx.value());
                },
                style: "width:300px",
                {disks}
            }
        }

        div {
            class: "files-panel",
            style: "  overflow-anchor: none;",
            tabindex: 1,

            onkeypress: move |ctx| {
                ctx.stop_propagation();
            },

            onkeydown: move |ctx| {
                ctx.stop_propagation();
            },


            onkeyup: move |ctx| {
                match ctx.key() {
                    Key::Enter => {
                        if let Some(selected_file_type) = selected_file_type {
                            match selected_file_type {
                                FileLineType::Dir => {
                                    main_state
                                        .write()
                                        .get_panel_state_mut(left_panel)
                                        .press_enter();
                                }
                                FileLineType::Back => {
                                    main_state
                                        .write()
                                        .get_panel_state_mut(left_panel)
                                        .press_enter();
                                }
                                FileLineType::File => {}
                            }
                        }
                    }
                    Key::ArrowDown => {
                        if selected_file_index < total_items - 1 {
                            main_state
                                .write()
                                .get_panel_state_mut(left_panel)
                                .set_selected_file(selected_file_index + 1);
                        }
                    }
                    Key::ArrowUp => {
                        if selected_file_index > 0 {
                            main_state
                                .write()
                                .get_panel_state_mut(left_panel)
                                .set_selected_file(selected_file_index - 1);
                        }
                    }
                    _ => {}
                }
                println!("Key pressed: {:?}", ctx);
            },
            table { class: "files-table",

                tr { style: "position:sticky; border-bottom:1px solid var(--line-separator-color)",
                    th {}
                    th { "Name" }
                    th { "Size" }
                    th { "Created" }
                    th { "Modified" }
                }
                {files}
            }
        }
        div { class: "file-panel-footer",
            "Total "
            b { {total_folders.to_string()} }
            " folders and "
            b { {total_files.to_string()} }
            " files sized "
            b { {total_size.to_string()} }
            " bytes"
        }
    }
}

async fn load_files(volume: &str, selected_path: &str) -> Result<FilesState, String> {
    let path = format!("{}{}", volume, selected_path);
    let mut read_dir = tokio::fs::read_dir(path)
        .await
        .map_err(|err| err.to_string())?;

    let mut result = FilesState {
        files: Vec::new(),
        total_size: 0,
        files_amount: 0,
        folders_amount: 0,
    };

    let mut folders = BTreeMap::new();

    let mut files = BTreeMap::new();

    loop {
        let next_entry = read_dir.next_entry().await.map_err(|err| err.to_string())?;
        if next_entry.is_none() {
            break;
        }

        let next_entry = next_entry.unwrap();

        let name = next_entry.file_name().to_string_lossy().to_string();
        let metadata = next_entry.metadata().await.map_err(|err| err.to_string())?;
        let file_info = PanelFileItem::new(metadata, name);

        result.total_size += file_info.size;

        match file_info.tp{
            FileLineType::Dir => {
                result.folders_amount += 1;
                folders.insert(file_info.name.to_lowercase(), file_info);
            },
            FileLineType::File => {
                result.files_amount += 1;
                files.insert(file_info.name.to_lowercase(), file_info);
            },
            FileLineType::Back => {},
        }
    }

    if selected_path.len()>0{
        let back = PanelFileItem::new_back();
        folders.insert(back.name.to_lowercase(), back);
    }

    for dir in folders {
        result.files.push(dir.1);
    }

    for dir in files {
        result.files.push(dir.1);
    }

    Ok(result)
}
