use druid::{widget::Painter, Color, RenderContext};

pub fn interactive_bg<T>(bg: Color, hover: Color, active: Color) -> Painter<T> {
    Painter::new(move |ctx, _, _| {
        let bounds = ctx.size().to_rect();

        // background
        ctx.fill(bounds, &bg);

        if ctx.is_hot() {
            // hover
            ctx.fill(bounds, &hover);
        }

        if ctx.is_active() {
            // click
            ctx.fill(bounds, &active);
        }
    })
}
