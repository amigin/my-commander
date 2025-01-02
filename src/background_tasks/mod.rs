mod calc_dir_size;
use std::sync::Arc;
mod save_state;
pub use save_state::*;

pub use calc_dir_size::*;

use crate::PersistenceState;

pub enum BackgroundTask {
    CalcDirSize(Arc<DirSizeCalculationHandler>),
    SaveState(PersistenceState),
}

impl Into<BackgroundTask> for Arc<DirSizeCalculationHandler> {
    fn into(self) -> BackgroundTask {
        BackgroundTask::CalcDirSize(self)
    }
}
