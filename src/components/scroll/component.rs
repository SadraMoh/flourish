use druid::{
    self,
    widget::List,
    widget::{Button, Either, Flex, Label, TextBox},
    Color, Widget, WidgetExt,
};

use super::ScrollState;
use crate::{
    components::task::TaskState,
    shared::styles::{BORDER_RADIUS, SM, TEXTBOX_BACKGROUND, XS},
};

use crate::shared::styles::LIST_WIDTH;

pub fn build_scroll() -> impl Widget<ScrollState> {
    let mut template = Flex::column();

    // list of tasks
    let tasks = List::<TaskState>::new(|| {
        Label::new(|item: &TaskState, _env: &_| format!("New List {}", item.description))
            .padding(10.0)
            .expand()
            .height(50.0)
            .background(Color::rgb(0.5, 0.5, 0.5))
    })
    .lens(ScrollState::tasks);

    const HEAD_HEIGHT: f64 = 24.;

    // label
    let label = Label::new(|scroll: &ScrollState, _env: &_| scroll.name.clone())
        .on_click(|_ctx, data: &mut ScrollState, _env| data.is_editing_name = true)
        .fix_height(HEAD_HEIGHT)
        .align_left();

    // label-editor
    let label_editor = Flex::row()
        .with_flex_child(
            TextBox::new()
                .lens(ScrollState::name)
                .background(TEXTBOX_BACKGROUND)
                .rounded(BORDER_RADIUS)
                .fix_height(HEAD_HEIGHT)
                .expand_width(),
            1.,
        )
        .with_spacer(XS)
        .with_child(
            Button::new("✓")
                .on_click(|_ctx, data: &mut ScrollState, _env| data.is_editing_name = false),
        )
        .expand_width();

    // scroll header
    let header = Either::new(
        |item: &ScrollState, _env: &_| item.is_editing_name,
        label_editor,
        label,
    )
    .padding(SM);

    template.add_child(header);
    template.add_child(tasks);

    template
        .fix_width(LIST_WIDTH)
        .background(Color::RED)
        .rounded(BORDER_RADIUS)
}
