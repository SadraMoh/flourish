use druid::{Data, im::Vector, Lens, widget::ListIter};

use crate::components::task::TaskState;

#[derive(Clone, Data, Lens, Default)]
pub struct ScrollState {
  pub name: String,
  pub tasks: Vector<TaskState>,
}

impl ScrollState {
  pub fn new() -> Self {
    ScrollState::default()
  }
}
