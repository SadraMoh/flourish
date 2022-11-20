use druid::im::Vector;
use druid::widget::{Button, Flex, List};
use druid::{Widget, WidgetExt};

use crate::components::scroll::{build_scroll, ScrollState};
use crate::components::task::TaskState;
use crate::shared::app_state::AppState;
use crate::shared::styles::LIST_WIDTH;

pub fn build_home() -> impl Widget<AppState> {
    let mut template = Flex::row();

    let button = Button::new("new list")
        .on_click(|_ctx, data: &mut Vector<ScrollState>, _env| data.push_front(ScrollState::new()))
        .lens(AppState::scrolls)
        .padding(5.0);

    // let list = List::new(|| Label::new("HELLo"))
    //     .horizontal()
    //     .lens(AppState::scrolls);

    // let list = List::new(build_scroll)
    //     .horizontal()
    //     .lens(AppState::scrolls);

    // list of scrolls
    let scrolls = List::<ScrollState>::new(build_scroll)
        .horizontal()
        .lens(AppState::scrolls);

    template.add_child(scrolls);
    template.add_child(button);

    template
}
