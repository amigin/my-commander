use std::collections::BTreeMap;

use dioxus::prelude::*;


use crate::{consts::*, dialogs::*, states::*, views::*};

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

    let mut selected_amount = 0;
    let mut selected_size = 0;

    let files = match &panel_state.files {
        DataState::None => {
            let volume_and_path = panel_state.volume_and_path.clone();
            let show_hidden = panel_state.show_hidden;
            spawn(async move {
                main_state
                    .write()
                    .get_panel_state_mut(left_panel)
                    .files
                    .set_loading();
                let path = volume_and_path.get_path();
                
                let files = load_files(volume_and_path.as_str(), path.len() == 0, show_hidden).await;
                match files {
                    Ok(files) => main_state
                        .write()
                        .get_panel_state_mut(left_panel)
                        .set_files(files),
                    Err(err) => main_state
                        .write()
                        .get_panel_state_mut(left_panel)
                        .files
                        .set_error(format!("Path: {path}. Error loading files: {err}")),
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
         

            let files = value.files.iter().enumerate().filter(|itm| {

                if panel_state.search.len()>0{
                    if !itm.1.name.to_lowercase().contains(&panel_state.search){
                        return false;
                    }
                }

                true


            }). map(|(no, file_info)| {

                if file_info.marked{
                    selected_amount += 1;
                    selected_size += file_info.size.get_size();
                }

                let item_selected = panel_state.is_file_selected(no);
                let class_selected = if main_state_read_access.left_panel_active == left_panel{
                    if item_selected {
                        Some("selected-line")
                    } else {
                        None
                    }
                }else{
                    if item_selected {
                        Some("selected-line-not-focused")
                    } else {
                        None
                    }
                };

                if item_selected  {
                    selected_file_type = Some(file_info.tp);
                }

                let (icon, created, modified) = match file_info.tp{
                    FileLineType::Dir => {
                        let icon = rsx! {
                            img {
                                class: "file-ico",
                                src: asset!("/assets/ico/folder.svg"),
                            }
                        };

                        let created = file_info.created.to_rfc5322();
                        let modified = file_info.modified.to_rfc5322();

                        (icon, created, modified)
                    },
                    FileLineType::File => 
                    {
                      let icon = render_icon(file_info.name.as_str());
                      let created = file_info.created.to_rfc5322();
                      let modified = file_info.modified.to_rfc5322();

                      (icon, created, modified)
                    },
                    FileLineType::Back => {
                        let icon = rsx! {
                            img {
                                class: "file-ico",
                                src: asset!("/assets/ico/back.svg"),
                            }
                        } ;


                        (icon, String::new(), String::new())
                    },
                };
           
                let marked_file_class = if file_info.marked { "file-marked" } else { "" };

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

                let calc_icon = if file_info.size.is_calculating(){
                    rsx!{
                        div { style: "width:16px; position: relative; top:3px;",
                            span { class: "loader" }
                        }
                    }

                }else{
                    rsx!{
                        div { style: "width:16px;" }
                    }
                };

                let item_file_type = file_info.tp;

                let style = if no == 0{
                    "margin-top: 10px;"
                }else{
                    ""
                };

                  rsx! {
                    tr {
                        id: class_selected,
                        style,
                        class: "file-line {class_selected.unwrap_or_default()} {hidden_attr} {marked_file_class}",
                        onclick: move |_| {
                            main_state.write().get_panel_state_mut(left_panel).set_selected_file(no);
                        },

                        ondoubleclick: move |_| {
                            match item_file_type {
                                FileLineType::Dir => {
                                    main_state.write().press_enter(left_panel);
                                }
                                FileLineType::Back => {
                                    main_state.write().press_enter(left_panel);
                                }
                                FileLineType::File => {}
                            }
                        },
                        td { {icon} }
                        td {
                            div {
                                class: "file-item file-name",
                                title: file_info.name.as_str(),
                                {file_info.name.as_str()}
                            }
                        }
                        td { {calc_icon} }
                        td {
                            div { class: "file-item file",
                                {file_info.size.get_formatted_size_as_string()}
                            }
                        }
                        td {
                            div { class: "file-item file-date", {created} }
                        }
                        td {
                            div { class: "file-item file-date", {modified} }
                        }
                    }
                }
                
            });

            rsx! {
                {files}
            }
        }
        DataState::Error(err) => rsx! {
            div { style: "text-align:center",
                div { "{err}" }
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


    crate::utils::scroll_to_active_element();

    let select_disk = render_select_disk(&main_state_read_access, &panel_state, left_panel);

    let show_hidden_style = if panel_state.show_hidden{
        "btn-secondary"
    }else{
        "btn-light"
    };

    let panel_id = if left_panel {LEFT_PANEL_ID } else { RIGHT_PANEL_ID };


    let search_ico =asset!("/assets/ico/search.svg");

    let selected_content = if selected_amount > 0 {
        let size = crate::utils::format_bytes(selected_size);
      rsx!{
        span { style: "color:red", "Selected: {selected_amount} items, {size} bytes" }
    }  
    }else{
        rsx!{}
    };

    rsx! {
        div { class: "top-panel",
            table { style: "width:100%",
                tr {
                    td { {select_disk} }
                    td { style: "text-align:right;",
                        div { style: "display:inline-block",
                            input {
                                class: "search-input",
                                style: "background-image: url(\"{search_ico}\")",
                                oninput: move |ctx| {
                                    main_state.write().get_panel_state_mut(left_panel).search = ctx
                                        .value()
                                        .to_lowercase();
                                },
                                value: panel_state.search.as_str(),
                            }
                        }
                        button {
                            class: "btn {show_hidden_style} btn-sm",
                            onclick: move |_| {
                                main_state.write().click_show_hidden(left_panel);
                            },
                            img {
                                class: "top-panel-ico",
                                src: asset!("/assets/ico/hidden.svg"),
                            }
                        }
                    }
                }
            }
        }

        div {
            id: panel_id,
            class: "files-panel",
            style: "  overflow-anchor: none;",
            tabindex: 1,

            onkeypress: move |ctx| {
                ctx.prevent_default();
            },

            onkeydown: move |ctx| {
                ctx.prevent_default();
            },


            onkeyup: move |ctx| {
                match ctx.key() {
                    Key::Tab => {
                        main_state.write().tab_pressed();
                    }
                    Key::Enter => {
                        if let Some(selected_file_type) = selected_file_type {
                            match selected_file_type {
                                FileLineType::Dir => {
                                    main_state.write().press_enter(left_panel);
                                }
                                FileLineType::Back => {
                                    main_state.write().press_enter(left_panel);
                                }
                                FileLineType::File => {}
                            }
                        }
                    }
                    Key::ArrowLeft => {
                        main_state.write().get_panel_state_mut(left_panel).selected_file_index = 0;
                    }
                    Key::ArrowRight => {
                        main_state.write().get_panel_state_mut(left_panel).select_last_file();
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
                    Key::F3 => {
                        let file_path = {
                            let write_access = main_state.read();
                            let panel_state = write_access.get_panel_state(left_panel);
                            let item = panel_state.get_selected_file();
                            if item.tp.is_file() {
                                Some(
                                    panel_state
                                        .volume_and_path
                                        .new_with_segment(item.name.as_str())
                                        .into_string(),
                                )
                            } else {
                                None
                            }
                        };
                        if let Some(file_path) = file_path {
                            main_state.write().dialog = Some(DialogState::ViewFile(file_path));
                        }
                    }
                    Key::Escape => {
                        if main_state.read().dialog.is_some() {
                            main_state.write().dialog = None;
                        }
                    }
                    _ => {
                        match ctx.code() {
                            Code::Space => {
                                main_state
                                    .write()
                                    .get_panel_state_mut(left_panel)
                                    .space_pressed(selected_file_index);
                            }
                            _ => {}
                        }
                    }
                }
            },
            table { class: "files-table",

                thead {
                    tr { style: " border-bottom:1px solid var(--line-separator-color)",
                        th {}
                        th { "Name" }
                        th {}
                        th { "Size" }
                        th { "Created" }
                        th { "Modified" }
                    }
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
            " bytes  "
            {selected_content}
        }
    }
}

async fn load_files(path:&str, root_path: bool, show_hidden: bool) -> Result<FilesState, String> {

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

        if !show_hidden && file_info.hidden {
            continue;
        }

        result.total_size += file_info.size.get_size();

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

    if !root_path{
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
