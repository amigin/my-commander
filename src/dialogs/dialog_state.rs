use std::rc::Rc;

use dioxus::prelude::*;

use crate::{volume_path_and_file::VolumePathAndFile, PanelFileItem};

#[derive(Clone, Debug)]
pub enum DialogState {
    ErrorDialog {
        title: &'static str,
        msg: String,
    },
    ViewFile(String),
    DeleteConfirmation {
        volume_and_path: VolumePathAndFile,
        items: Rc<Vec<PanelFileItem>>,
        on_ok: EventHandler<()>,
    },
    CreateDir(Rc<String>),
}
