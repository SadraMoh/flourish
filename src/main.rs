use druid::{AppLauncher, Color, PlatformError, Widget, WidgetExt, WindowDesc};

use flourish::components::home::build_home;
use flourish::shared::app_state::AppState;
use flourish::shared::styles::BORDER_COLOR;

fn main() -> Result<(), PlatformError> {
    let data = AppState::default();
    let main_window = WindowDesc::new(build_ui)
        .window_size((900., 600.))
        .set_position((500., 500.));

    AppLauncher::with_window(main_window).launch(data)
}

fn build_ui() -> impl Widget<AppState> {
    build_home().border(BORDER_COLOR, 1.)
}
