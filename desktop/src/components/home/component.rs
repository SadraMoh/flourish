use druid::im::Vector;
use druid::widget::{Button, CrossAxisAlignment, Flex, List};
use druid::{KeyOrValue, Widget, WidgetExt};

use crate::components::scroll::{build_scroll, ScrollState};
use crate::shared::app_state::AppState;
use crate::shared::styles::XL;

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
    let mut scrolls = List::<ScrollState>::new(build_scroll)
        .horizontal()
        .with_spacing(KeyOrValue::Concrete(XL))
        .lens(AppState::scrolls);

    template.add_child(scrolls);
    template.add_child(button);

    template
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .expand()
        .padding(XL)
}
