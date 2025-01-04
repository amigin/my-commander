use dioxus::prelude::*;

use crate::{volume_path_and_file::VolumePathAndFile, PanelFileItem};

#[derive(Clone, Debug)]
pub enum DialogState {
    ViewFile(String),
    DeleteConfirmation {
        amount: usize,
        volume_and_path: VolumePathAndFile,
        selected_item: PanelFileItem,
        on_ok: EventHandler<()>,
    },
}
