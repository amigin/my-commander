use super::{DisksState, PanelState};

pub struct MainState {
    pub disks: DisksState,
    pub left_panel: PanelState,
    pub right_panel: PanelState,
    pub left_panel_active: bool,
}

impl MainState {
    pub fn new() -> Self {
        let disks = DisksState::new();
        let (selected_volume, selected_path) = {
            let item = disks.iter().next().unwrap();

            (item.path.to_string(), item.default_path.to_string())
        };
        MainState {
            disks,
            left_panel: PanelState::new(selected_volume.clone(), selected_path.to_string()),
            right_panel: PanelState::new(selected_volume, selected_path),
            left_panel_active: true,
        }
    }

    pub fn tab_pressed(&mut self) {
        self.left_panel_active = !self.left_panel_active;
        crate::utils::set_panel_focus(self.left_panel_active);
    }

    pub fn get_panel_state(&self, left_panel: bool) -> &PanelState {
        if left_panel {
            &self.left_panel
        } else {
            &self.right_panel
        }
    }

    pub fn get_panel_state_mut(&mut self, left_panel: bool) -> &mut PanelState {
        self.left_panel_active = left_panel;
        if left_panel {
            &mut self.left_panel
        } else {
            &mut self.right_panel
        }
    }

    pub fn get_active_path(&self) -> &str {
        if self.left_panel_active {
            self.left_panel.selected_path.as_str()
        } else {
            self.right_panel.selected_path.as_str()
        }
    }
}
