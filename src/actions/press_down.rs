use dioxus::prelude::*;

use crate::states::{MainState, PanelFilesStatistics};

pub fn press_down(
    selected_file_index: usize,
    panel_statistics: &PanelFilesStatistics,
    mut main_state: Signal<MainState>,
) {
    if selected_file_index < panel_statistics.total_items - 1 {
        main_state
            .write()
            .get_panel_state_mut(panel_statistics.left_panel)
            .set_selected_file(selected_file_index + 1);
    }
}
