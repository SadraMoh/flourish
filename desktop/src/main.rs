use druid::{AppLauncher, PlatformError, Widget, WidgetExt, WindowDesc, MenuDesc, LocalizedString};

use flourish::components::home::build_home;
use flourish::shared::app_state::AppState;
use flourish::shared::styles::BORDER_COLOR;

const IMPORTANT_LABEL_COLOR: Key<Color> = Key::new("important-label-color");
const RED: Color = Color::rgb8(0xFF, 0, 0);

fn main() -> Result<(), PlatformError> {
    let data = AppState::default();
    let main_window = WindowDesc::new(build_ui)
        .window_size((900., 600.))
        .set_position((500., 500.))
        .menu(build_menu());

    AppLauncher::with_window(main_window).launch(data)
}

fn build_ui() -> impl Widget<AppState> {
    build_home().border(BORDER_COLOR, 1.)
}

fn build_menu() -> MenuDesc<AppState> {
    MenuDesc::new(LocalizedString::new("menu"))
        .append(druid::platform_menus::common::undo())
        .append(druid::platform_menus::common::redo())
        .append(druid::platform_menus::common::cut())
        .append(druid::platform_menus::common::copy())
        .append(druid::platform_menus::common::paste())
}
