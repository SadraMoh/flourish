use druid::{im::Vector, Data, Lens};

use crate::components::scroll::ScrollState;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub scrolls: Vector<ScrollState>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            scrolls: Default::default(),
        }
    }
}
