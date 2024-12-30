use std::mem;

use super::{DataState, PanelFileItem};

pub enum AutoSelectElement {
    None,
    Dir(String),
    File(String),
}

impl AutoSelectElement {
    pub fn set_dir_to_auto_select(name: Option<String>) -> Self {
        match name {
            Some(name) => Self::Dir(name),
            None => Self::None,
        }
    }

    pub fn set_auto_select(el: Option<&PanelFileItem>) -> Self {
        if el.is_none() {
            return Self::None;
        }

        let el = el.unwrap();
        match el.tp {
            super::FileLineType::Dir => Self::Dir(el.name.clone()),
            super::FileLineType::File => Self::File(el.name.clone()),
            super::FileLineType::Back => Self::None,
        }
    }
}

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
    pub auto_select_after_load: AutoSelectElement,
    pub show_hidden: bool,
    pub search: String,
}

impl PanelState {
    pub fn new(selected_volume: String, selected_path: String) -> Self {
        PanelState {
            files: DataState::None,
            selected_volume,
            selected_path,
            selected_file_index: 0,
            auto_select_after_load: AutoSelectElement::None,
            show_hidden: false,
            search: String::new(),
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
        match &self.auto_select_after_load {
            AutoSelectElement::None => {}
            AutoSelectElement::Dir(name_to_auto_select) => {
                for (index, itm) in files.files.iter().enumerate() {
                    if itm.tp.is_dir() {
                        if itm.name.eq_ignore_ascii_case(name_to_auto_select) {
                            self.selected_file_index = index;
                            break;
                        }
                    }
                }
            }
            AutoSelectElement::File(name_to_auto_select) => {
                for (index, itm) in files.files.iter().enumerate() {
                    if !itm.tp.is_dir() {
                        if itm.name.eq_ignore_ascii_case(name_to_auto_select) {
                            self.selected_file_index = index;
                            break;
                        }
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
        self.search.clear();
    }

    pub fn go_back(&mut self) {
        let mut path = String::new();
        mem::swap(&mut self.selected_path, &mut path);
        let mut path_segments: Vec<_> = path.split("/").collect();
        let last_segment = path_segments.pop();
        self.search.clear();

        for segment in path_segments {
            if segment.is_empty() {
                continue;
            }
            self.selected_path.push(std::path::MAIN_SEPARATOR);
            self.selected_path.push_str(segment);
        }

        self.reset_files();
        self.auto_select_after_load =
            AutoSelectElement::set_dir_to_auto_select(last_segment.map(|s| s.to_string()));
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
        match &self.files {
            DataState::Loaded(files) => {
                let item = files.files.get(self.selected_file_index);
                self.auto_select_after_load = AutoSelectElement::set_auto_select(item);
            }
            _ => {
                self.auto_select_after_load = AutoSelectElement::None;
            }
        }
        self.files.set_none();
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
