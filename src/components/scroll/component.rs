use druid::{
    self,
    widget::List,
    widget::{Either, Flex, Label, TextBox},
    Color, Insets, TextAlignment, Widget, WidgetExt,
};

use super::ScrollState;
use crate::{
    components::{
        core::util::interactive_bg,
        task::{build_task, TaskState},
    },
    shared::styles::{
        ACTIVE, BORDER_COLOR, BORDER_RADIUS, CARD_BACKGROUND, HIGHLIGHT, PRIMARY, SCROLL_WIDTH, SM,
        TEXTBOX_BACKGROUND, TRANSPARENT, XS,
    },
};

pub fn build_scroll() -> impl Widget<ScrollState> {
    let mut template = Flex::column();

    // list of tasks
    let tasks = List::<TaskState>::new(build_task)
        // Label::new(|item: &TaskState, _env: &_| format!("New List {}", item.description))
        //     .padding(10.0)
        //     .expand()
        //     .height(50.0)
        //     .background(Color::rgb(0.5, 0.5, 0.5))
        .lens(ScrollState::tasks);

    const HEAD_HEIGHT: f64 = 28.;

    // label
    let label = Label::new(|scroll: &ScrollState, _env: &_| scroll.name.clone())
        .padding((4., 4.))
        .background(interactive_bg(TRANSPARENT, HIGHLIGHT, ACTIVE))
        .rounded(4.)
        .on_click(|_ctx, data: &mut ScrollState, _env| data.is_editing_name = true)
        .fix_height(HEAD_HEIGHT)
        .align_left();

    // label-editor
    let label_editor = Flex::row()
        .with_flex_child(
            TextBox::new()
                .lens(ScrollState::name)
                .fix_height(HEAD_HEIGHT)
                .expand_width(),
            1.,
        )
        .with_spacer(XS)
        .with_child(
            Label::new("✓")
                .background(interactive_bg(PRIMARY, Color::RED, Color::BLUE))
                .rounded(8.)
                .on_click(|_ctx, data: &mut ScrollState, _env| data.is_editing_name = false)
                .fix_size(24., 24.),
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
        .fix_width(SCROLL_WIDTH)
        .background(CARD_BACKGROUND)
        .border(BORDER_COLOR, 2.)
        .rounded(BORDER_RADIUS)
}
