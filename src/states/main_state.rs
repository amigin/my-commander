use mountpoints::MountInfo;

use super::PanelState;

pub struct MainState {
    pub mounts: Vec<MountInfo>,
    pub left_panel: PanelState,
    pub right_panel: PanelState,
    pub left_panel_active: bool,
}

impl MainState {
    pub fn new() -> Self {
        let mounts = mountpoints::mountinfos().unwrap();
        let selected_volume = mounts.first().unwrap().path.to_str().unwrap().to_string();
        MainState {
            mounts,
            left_panel: PanelState::new(selected_volume.clone()),
            right_panel: PanelState::new(selected_volume.clone()),
            left_panel_active: true,
        }
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
