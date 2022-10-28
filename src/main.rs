use druid::im::Vector;
use druid::widget::{Button, Flex, Label, List};
use druid::{AppLauncher, Color, Data, Lens, PlatformError, Widget, WidgetExt, WindowDesc};

use flourish::scroll::*;
use flourish::task::TaskState;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub scrolls: Vector<ScrollState>,
}

fn main() -> Result<(), PlatformError> {
    let data = AppState {
        scrolls: Vector::new(),
    };
    let main_window = WindowDesc::new(ui_builder);
    AppLauncher::with_window(main_window).launch(data)
}

fn ui_builder() -> impl Widget<AppState> {
    let mut template = Flex::column();

    let button = Button::new("new list")
        .on_click(|_ctx, data: &mut Vector<ScrollState>, _env| data.push_front(ScrollState::new()))
        .lens(AppState::scrolls)
        .padding(5.0);

    let list = List::new(|| {
        Label::new(|item: &ScrollState, _env: &_| format!("New List {}", item.name))
            .padding(10.0)
            .expand()
            .height(50.0)
            .background(Color::rgb(0.5, 0.5, 0.5))
    })
    .lens(AppState::scrolls);
    
    template.add_child(list);
    template.add_child(button);

    template
}
