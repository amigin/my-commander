use std::{collections::HashMap, rc::Rc, sync::Arc};

use dioxus::prelude::*;

use crate::{
    background_tasks::DirSizeCalculationHandler, volume_path_and_file::VolumePathAndFile,
    BackgroundTask,
};

use super::{DataState, PanelFileItem, PanelFilesStatistics};

pub enum AutoSelectElement {
    None,
    Dir(String),
    File(String),
}

impl AutoSelectElement {
    pub fn set_dir_to_auto_select(name: String) -> Self {
        Self::Dir(name.into())
    }

    pub fn set_auto_select(el: Option<&PanelFileItem>) -> Self {
        if el.is_none() {
            return Self::None;
        }

        let el = el.unwrap();
        match el.tp {
            super::FileLineType::Dir => Self::Dir(el.name.clone().into()),
            super::FileLineType::File => Self::File(el.name.clone().into()),
            super::FileLineType::Back => Self::None,
        }
    }
}

pub struct FilesState {
    pub items: Vec<PanelFileItem>,
    pub files_amount: usize,
    pub folders_amount: usize,
}

pub struct PanelState {
    pub files: DataState<Vec<PanelFileItem>>,
    pub volume_and_path: VolumePathAndFile,
    pub selected_file_index: usize,
    pub auto_select_after_load: AutoSelectElement,
    pub show_hidden: bool,
    pub search: String,
    pub left_panel: bool,
    calculations: HashMap<String, Arc<DirSizeCalculationHandler>>,

    pub background_tasks: Rc<Coroutine<BackgroundTask>>,
    pub statistics: PanelFilesStatistics,
}

impl PanelState {
    pub fn new(
        background_tasks: Rc<Coroutine<BackgroundTask>>,
        volume_and_path: VolumePathAndFile,
        left_panel: bool,
        show_hidden: bool,
    ) -> Self {
        PanelState {
            files: DataState::None,
            volume_and_path,
            selected_file_index: 0,
            auto_select_after_load: AutoSelectElement::None,
            show_hidden,
            search: String::new(),
            left_panel,
            calculations: HashMap::new(),
            background_tasks,
            statistics: PanelFilesStatistics::new(left_panel),
        }
    }

    pub fn is_file_selected(&self, index: usize) -> bool {
        self.selected_file_index == index
    }

    pub fn set_selected_file(&mut self, no: usize) {
        self.selected_file_index = no;
    }

    fn mark_file_or_dir(&mut self, no: usize) -> PressSpaceActionResult {
        let files_state = self.files.unwrap_loaded_mut();

        let file = files_state.get_mut(no);

        if let Some(file) = file {
            match file.tp {
                super::FileLineType::Dir => {
                    file.marked = !file.marked;

                    if file.marked {
                        if file.size.is_unknown() {
                            file.size = super::FileItemSize::Calculating(0);

                            let size_calculator_handler = Arc::new(DirSizeCalculationHandler::new(
                                self.volume_and_path.new_with_segment(&file.name),
                                self.left_panel,
                            ));

                            println!("Sending size calculator to thread");
                            self.background_tasks
                                .send(size_calculator_handler.clone().into());
                            return PressSpaceActionResult::StartCalculation(
                                size_calculator_handler,
                            );
                        }
                    } else {
                        if !file.size.is_known() {
                            let dir = self.volume_and_path.new_with_segment(&file.name);
                            file.size = super::FileItemSize::Unknown;
                            return PressSpaceActionResult::StopCalculation(dir);
                        }
                    }
                }
                super::FileLineType::File => {
                    file.marked = !file.marked;
                }
                super::FileLineType::Back => {}
            }
        }

        PressSpaceActionResult::DoNothing
    }

    pub fn space_pressed(&mut self, no: usize) {
        match self.mark_file_or_dir(no) {
            PressSpaceActionResult::StartCalculation(handler) => {
                let key = handler.dir.to_string();
                println!("Inserting calculation with key: {}", key);
                self.calculations.insert(key, handler);
            }
            PressSpaceActionResult::StopCalculation(file_and_path) => {
                let key = file_and_path.into_string();
                println!("Removing calculation with key: {}", key);
                if let Some(item) = self.calculations.remove(&key) {
                    println!("Cancelling calculation for dir: {}", key);
                    item.cancel();
                }
            }
            PressSpaceActionResult::DoNothing => {}
        }
    }

    pub fn set_files(&mut self, files: FilesState) {
        match &self.auto_select_after_load {
            AutoSelectElement::None => {}
            AutoSelectElement::Dir(name_to_auto_select) => {
                for (index, itm) in files.items.iter().enumerate() {
                    if itm.tp.is_dir() {
                        if itm.name.eq_ignore_ascii_case(name_to_auto_select) {
                            self.selected_file_index = index;
                            break;
                        }
                    }
                }
            }
            AutoSelectElement::File(name_to_auto_select) => {
                for (index, itm) in files.items.iter().enumerate() {
                    if !itm.tp.is_dir() {
                        if itm.name.eq_ignore_ascii_case(name_to_auto_select) {
                            self.selected_file_index = index;
                            break;
                        }
                    }
                }
            }
        }
        self.statistics.files_amount = files.files_amount;
        self.statistics.folders_amount = files.folders_amount;
        self.statistics.total_items = files.items.len();
        self.files.set_loaded(files.items);
    }

    fn cancel_dir_size_calculation(&mut self) {
        for (_, calc) in self.calculations.drain() {
            calc.cancel();
        }
    }

    pub fn go_to_folder(&mut self, no: usize) {
        let item = self.files.unwrap_loaded_mut().remove(no);
        self.volume_and_path.append_segment(&item.name);

        self.reset_files();
        self.search.clear();
        self.cancel_dir_size_calculation();
    }

    pub fn go_back(&mut self) {
        let item = self.volume_and_path.go_back().unwrap();

        self.search.clear();

        self.reset_files();
        self.auto_select_after_load = AutoSelectElement::set_dir_to_auto_select(item);
        self.cancel_dir_size_calculation();
    }

    fn reset_files(&mut self) {
        self.files.set_none();
        self.selected_file_index = 0;
    }

    pub fn set_selected_volume(&mut self, volume: String) {
        self.volume_and_path = VolumePathAndFile::new(volume);
        self.files.set_none();
    }

    pub fn click_show_hidden(&mut self) -> bool {
        match &self.files {
            DataState::Loaded(files) => {
                let item = files.get(self.selected_file_index);
                self.auto_select_after_load = AutoSelectElement::set_auto_select(item);
            }
            _ => {
                self.auto_select_after_load = AutoSelectElement::None;
            }
        }
        self.files.set_none();
        self.show_hidden = !self.show_hidden;
        self.show_hidden
    }

    pub fn press_enter(&mut self) -> bool {
        let tp = self
            .files
            .unwrap_loaded_mut()
            .get(self.selected_file_index)
            .unwrap()
            .tp;
        match tp {
            super::FileLineType::Dir => {
                self.go_to_folder(self.selected_file_index);
                true
            }
            super::FileLineType::File => false,
            super::FileLineType::Back => {
                self.go_back();
                true
            }
        }
    }

    fn find_calculated_dir(
        &mut self,
        dir: &VolumePathAndFile,
        found_dir: impl Fn(&mut PanelFileItem),
    ) {
        let (volume_and_path, dir) = dir.get_last_segment().unwrap();

        if self.volume_and_path.as_str() != volume_and_path {
            return;
        }

        if let DataState::Loaded(files) = &mut self.files {
            for itm in files.iter_mut() {
                if itm.tp.is_dir() {
                    if itm.name.eq_ignore_ascii_case(dir) {
                        found_dir(itm);
                        break;
                    }
                }
            }
        }
    }

    pub fn set_dir_size_unknown(&mut self, dir: &VolumePathAndFile) {
        self.find_calculated_dir(dir, |itm| {
            itm.size = super::FileItemSize::Unknown;
        });
    }
    pub fn set_dir_size(&mut self, dir: &VolumePathAndFile, size: u64, known: bool) {
        self.find_calculated_dir(dir, |itm| {
            if known {
                itm.size = super::FileItemSize::Known(size);
            } else {
                itm.size = super::FileItemSize::Calculating(size);
            }
        });
    }

    pub fn get_selected_item(&self) -> &PanelFileItem {
        self.files
            .unwrap_loaded()
            .get(self.selected_file_index)
            .unwrap()
    }

    pub fn try_get_selected_item(&self) -> Option<&PanelFileItem> {
        match self.files {
            DataState::Loaded(ref files) => files.get(self.selected_file_index),
            _ => None,
        }
    }

    pub fn select_last_file(&mut self) {
        let last_index = if let DataState::Loaded(files) = &self.files {
            files.len()
        } else {
            0
        };
        self.selected_file_index = last_index - 1;
    }

    pub fn refresh_files(&mut self, set_selected_item: AutoSelectElement) {
        self.auto_select_after_load = set_selected_item;
        self.files.set_none();
        self.statistics.reset();
    }
}

pub enum PressSpaceActionResult {
    StartCalculation(Arc<DirSizeCalculationHandler>),
    StopCalculation(VolumePathAndFile),
    DoNothing,
}
