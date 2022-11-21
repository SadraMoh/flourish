use druid::{
    self,
    piet::RoundInto,
    widget::{Button, Either, Flex, Label, TextBox},
    widget::{List, Painter},
    Color, KeyOrValue, RenderContext, Widget, WidgetExt,
};

use super::ScrollState;
use crate::{
    components::task::{build_task, TaskState},
    shared::styles::{
        BORDER_COLOR, BORDER_RADIUS, CARD_BACKGROUND, PRIMARY, SM, TEXTBOX_BACKGROUND, XS,
    },
};

use crate::shared::styles::LIST_WIDTH;

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

    const HEAD_HEIGHT: f64 = 24.;

    // label
    let label = Label::new(|scroll: &ScrollState, _env: &_| scroll.name.clone())
        .on_click(|_ctx, data: &mut ScrollState, _env| data.is_editing_name = true)
        .fix_height(HEAD_HEIGHT)
        .align_left();

    let painter = Painter::new(|ctx, _, env| {
        let bounds = ctx.size().to_rect();

        ctx.fill(bounds, &PRIMARY);

        if ctx.is_hot() {
            // ctx.stroke(bounds.inset(-0.5), &Color::WHITE, 1.0);
            ctx.fill(bounds, &Color::RED);
        }

        if ctx.is_active() {
            ctx.fill(bounds, &Color::GRAY);
        }
    });

    // label-editor
    let label_editor = Flex::row()
        .with_flex_child(
            TextBox::new()
                .lens(ScrollState::name)
                .background(TEXTBOX_BACKGROUND)
                .fix_height(HEAD_HEIGHT)
                .expand_width(),
            1.,
        )
        .with_spacer(XS)
        .with_child(
            Label::new("✓")
                .background(painter)
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
        .fix_width(LIST_WIDTH)
        .background(CARD_BACKGROUND)
        .border(BORDER_COLOR, 2.)
        .rounded(BORDER_RADIUS)
}
