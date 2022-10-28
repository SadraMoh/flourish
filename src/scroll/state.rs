use druid::{Data, im::Vector, Lens};

use crate::task::TaskState;

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