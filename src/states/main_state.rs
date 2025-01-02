use std::{rc::Rc, sync::Arc};

use crate::{scripts::DirSizeCalculationHandler, volume_path_and_file::VolumePathAndFile};

use super::{DisksState, PanelState, PersistenceState};
use dioxus::prelude::*;

pub struct MainState {
    pub disks: DisksState,
    pub left_panel: PanelState,
    pub right_panel: PanelState,
    pub left_panel_active: bool,
    pub persistence_state: PersistenceState,
}

impl MainState {
    pub fn new(
        persistence_state: PersistenceState,
        size_calculator: Coroutine<Arc<DirSizeCalculationHandler>>,
    ) -> Self {
        let disks = DisksState::new();
        let volume_and_path = {
            let item = disks.iter().next().unwrap();

            VolumePathAndFile::new_with_path(item.path.to_string(), item.default_path.as_str())
        };

        let size_calculator = Rc::new(size_calculator);

        MainState {
            disks,
            left_panel: PanelState::new(size_calculator.clone(), volume_and_path.clone(), true),
            right_panel: PanelState::new(size_calculator, volume_and_path, false),
            left_panel_active: true,
            persistence_state,
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
            self.left_panel.volume_and_path.get_path()
        } else {
            self.right_panel.volume_and_path.get_path()
        }
    }
}
