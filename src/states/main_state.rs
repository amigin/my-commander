use std::rc::Rc;

use crate::{dialogs::DialogState, volume_path_and_file::VolumePathAndFile, BackgroundTask};

use super::*;
use dioxus::prelude::*;

pub struct MainState {
    pub disks: DisksState,
    pub left_panel: PanelState,
    pub right_panel: PanelState,
    pub left_panel_active: bool,
    pub persistence_state: PersistenceState,
    background_tasks: Rc<Coroutine<BackgroundTask>>,
    dialog: Option<DialogState>,
}

impl MainState {
    pub fn new(
        persistence_state: PersistenceState,
        background_tasks: Coroutine<BackgroundTask>,
    ) -> Self {
        let disks = DisksState::new();
        let disk_info = disks.iter().next().unwrap();
        let left_volume_and_path = {
            if let Some(persistence) = persistence_state.left_panel.active.as_ref() {
                VolumePathAndFile::new_with_path(persistence.volume.to_string(), &persistence.path)
            } else {
                VolumePathAndFile::new_with_path(
                    disk_info.path.to_string(),
                    disk_info.default_path.as_str(),
                )
            }
        };

        let right_volume_and_path = {
            if let Some(persistence) = persistence_state.right_panel.active.as_ref() {
                VolumePathAndFile::new_with_path(
                    persistence.volume.to_string(),
                    persistence.path.as_str(),
                )
            } else {
                VolumePathAndFile::new_with_path(
                    disk_info.path.to_string(),
                    disk_info.default_path.as_str(),
                )
            }
        };

        let background_tasks = Rc::new(background_tasks);

        MainState {
            disks,
            left_panel: PanelState::new(
                background_tasks.clone(),
                left_volume_and_path,
                true,
                persistence_state.left_panel.show_hidden_files,
            ),
            right_panel: PanelState::new(
                background_tasks.clone(),
                right_volume_and_path,
                false,
                persistence_state.right_panel.show_hidden_files,
            ),
            left_panel_active: true,
            persistence_state,
            background_tasks,
            dialog: None,
        }
    }

    pub fn tab_pressed(&mut self) {
        self.left_panel_active = !self.left_panel_active;
        crate::utils::set_panel_focus(self.left_panel_active);
    }

    pub fn get_panel_state(&self, left_panel: Option<bool>) -> &PanelState {
        match left_panel {
            Some(left_panel) => {
                if left_panel {
                    &self.left_panel
                } else {
                    &self.right_panel
                }
            }

            None => self.get_active_panel(),
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

    pub fn press_enter(&mut self, left_panel: Option<bool>) {
        let has_update = {
            let active_panel = if let Some(left_panel) = left_panel {
                self.get_panel_state_mut(left_panel)
            } else {
                self.get_active_panel_mut()
            };
            active_panel.press_enter()
        };

        if has_update {
            let persistence_state = {
                let active_panel = if let Some(left_panel) = left_panel {
                    self.get_panel_state_mut(left_panel)
                } else {
                    self.get_active_panel()
                };

                VolumeAndPathPersistenceState {
                    volume: active_panel.volume_and_path.get_volume().to_string(),
                    path: active_panel.volume_and_path.get_path().to_string(),
                }
            };

            if self.left_panel_active {
                self.persistence_state.left_panel.active = Some(persistence_state);
            } else {
                self.persistence_state.right_panel.active = Some(persistence_state);
            }

            self.background_tasks
                .send(BackgroundTask::SaveState(self.persistence_state.clone()));
        }
    }

    pub fn click_show_hidden(&mut self, left_panel: bool) {
        let value = {
            let active_panel = self.get_panel_state_mut(left_panel);
            active_panel.click_show_hidden()
        };

        if left_panel {
            self.persistence_state.left_panel.show_hidden_files = value;
        } else {
            self.persistence_state.right_panel.show_hidden_files = value;
        }

        self.background_tasks
            .send(BackgroundTask::SaveState(self.persistence_state.clone()));
    }

    pub fn get_active_panel(&self) -> &PanelState {
        if self.left_panel_active {
            &self.left_panel
        } else {
            &self.right_panel
        }
    }

    pub fn get_active_panel_mut(&mut self) -> &mut PanelState {
        if self.left_panel_active {
            &mut self.left_panel
        } else {
            &mut self.right_panel
        }
    }

    pub fn set_focus_to_active_panel(&self) {
        crate::utils::set_panel_focus(self.left_panel_active);
    }

    pub fn show_dialog(&mut self, dialog: DialogState) {
        self.dialog = Some(dialog);
    }

    pub fn hide_dialog(&mut self) {
        self.dialog = None;
        self.set_focus_to_active_panel();
    }

    pub fn dialog_is_opened(&self) -> bool {
        self.dialog.is_some()
    }

    pub fn get_dialog(&self) -> &Option<DialogState> {
        &self.dialog
    }
}
