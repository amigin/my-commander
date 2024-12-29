use std::mem;

use super::{DataState, PanelFileItem};

pub struct FilesState {
    pub files: Vec<PanelFileItem>,
    pub total_size: u64,
    pub files_amount: usize,
    pub folders_amount: usize,
}

pub struct PanelState {
    pub files: DataState<FilesState>,
    pub selected_volume: String,
    pub selected_path: String,
    pub selected_file_index: usize,
    pub auto_select_dir: Option<String>,
    pub show_hidden: bool,
}

impl PanelState {
    pub fn new(selected_volume: String, selected_path: String) -> Self {
        PanelState {
            files: DataState::None,
            selected_volume,
            selected_path,
            selected_file_index: 0,
            auto_select_dir: None,
            show_hidden: false,
        }
    }

    pub fn is_file_selected(&self, index: usize) -> bool {
        self.selected_file_index == index
    }

    pub fn set_selected_file(&mut self, no: usize) {
        self.selected_file_index = no;
    }

    pub fn mark_file(&mut self, no: usize) {
        let files_state = self.files.unwrap_loaded_mut();

        let file = files_state.files.get_mut(no);

        if let Some(file) = file {
            match file.tp {
                super::FileLineType::Dir => {
                    file.marked = !file.marked;
                }
                super::FileLineType::File => {
                    file.marked = !file.marked;
                }
                super::FileLineType::Back => {}
            }
        }
    }

    pub fn set_files(&mut self, files: FilesState) {
        if let Some(auto_select_dir) = &self.auto_select_dir {
            for (index, itm) in files.files.iter().enumerate() {
                if itm.tp.is_dir() {
                    if itm.name.eq_ignore_ascii_case(auto_select_dir) {
                        self.selected_file_index = index;
                        break;
                    }
                }
            }
        }

        self.files.set_loaded(files);
    }

    pub fn go_to_folder(&mut self, no: usize) {
        let item = self.files.unwrap_loaded_mut().files.remove(no);
        self.selected_path.push(std::path::MAIN_SEPARATOR);

        self.selected_path.push_str(&item.name);
        self.reset_files();
    }

    pub fn go_back(&mut self) {
        let mut path = String::new();
        mem::swap(&mut self.selected_path, &mut path);
        let mut path_segments: Vec<_> = path.split("/").collect();
        let last_segment = path_segments.pop();

        for segment in path_segments {
            if segment.is_empty() {
                continue;
            }
            self.selected_path.push(std::path::MAIN_SEPARATOR);
            self.selected_path.push_str(segment);
        }

        self.reset_files();
        self.auto_select_dir = last_segment.map(|s| s.to_string());
    }

    fn reset_files(&mut self) {
        self.files.set_none();
        self.selected_file_index = 0;
    }

    pub fn set_selected_volume(&mut self, volume: String) {
        self.selected_volume = volume;
        self.selected_path.clear();
        self.files.set_none();
    }

    pub fn click_show_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;
    }

    pub fn press_enter(&mut self) {
        let tp = self
            .files
            .unwrap_loaded_mut()
            .files
            .get(self.selected_file_index)
            .unwrap()
            .tp;
        match tp {
            super::FileLineType::Dir => {
                self.go_to_folder(self.selected_file_index);
            }
            super::FileLineType::File => {}
            super::FileLineType::Back => {
                self.go_back();
            }
        }
    }
}
