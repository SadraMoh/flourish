use druid::{
    self,
    im::Vector,
    widget::Label,
    widget::{LensWrap, List},
    Color, Widget, WidgetExt,
};

use crate::{
    components::task::TaskState,
    shared::app_state::{app_state_derived_lenses::scrolls, AppState},
};

use super::ScrollState;

pub fn build_scroll() -> impl Widget<ScrollState> {
    // list of tasks
    let tasks = List::<TaskState>::new(|| {
        Label::new(|item: &TaskState, _env: &_| format!("New List {}", item.description))
            .padding(10.0)
            .expand()
            .height(50.0)
            .background(Color::rgb(0.5, 0.5, 0.5))
    })
    .lens(ScrollState::tasks);

    tasks
}
