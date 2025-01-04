#[derive(Clone, Copy)]
pub struct PanelFilesStatistics {
    pub left_panel: bool,
    pub folders_amount: usize,
    pub files_amount: usize,
    pub total_items: usize,
    pub selected_amount: usize,
}

impl PanelFilesStatistics {
    pub fn new(left_panel: bool) -> Self {
        Self {
            left_panel,
            folders_amount: 0,
            files_amount: 0,
            total_items: 0,
            selected_amount: 0,
        }
    }

    pub fn reset(&mut self) {
        self.folders_amount = 0;
        self.files_amount = 0;
        self.total_items = 0;
        self.selected_amount = 0;
    }
}
