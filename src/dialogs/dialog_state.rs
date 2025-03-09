use std::rc::Rc;

use dioxus::prelude::*;

use crate::{volume_path_and_file::VolumePathAndFile, PanelFileItem};
#[derive(Clone, Debug, PartialEq)]
pub enum SelectedItem {
    Single(PanelFileItem),
    MultiSelect(usize),
}

#[derive(Clone, Debug)]
pub enum DialogState {
    ErrorDialog {
        title: &'static str,
        msg: String,
    },
    ViewFile(String),
    DeleteConfirmation {
        volume_and_path: VolumePathAndFile,
        selected_item: SelectedItem,
        on_ok: EventHandler<()>,
    },
    CreateDir(Rc<String>),
}
