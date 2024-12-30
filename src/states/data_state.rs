pub enum DataState<T> {
    None,
    Loading,
    Loaded(T),
    Error(String),
}

impl<T> DataState<T> {
    pub fn set_none(&mut self) {
        *self = DataState::None;
    }
    pub fn set_error(&mut self, err: String) {
        *self = DataState::Error(err);
    }
    pub fn set_loading(&mut self) {
        *self = DataState::Loading;
    }
    pub fn set_loaded(&mut self, value: T) {
        *self = DataState::Loaded(value);
    }

    pub fn unwrap_loaded_mut(&mut self) -> &mut T {
        match self {
            DataState::Loaded(value) => value,
            _ => panic!("DataState is not loaded"),
        }
    }

    pub fn as_ref(&self) -> &Self {
        self
    }
}
