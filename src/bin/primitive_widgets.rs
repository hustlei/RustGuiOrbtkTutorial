//! rust orbtk primitive widgets

use orbtk::properties::Visibility;
use orbtk::styling::vector_graphics::material_font_icons::{CHECK_FONT_ICON, FLOPPY_FONT_ICON};
use orbtk::*;

struct MainView;

impl Widget for MainView {
    type Template = Template;

    fn create() -> self::Template {
        Template::default().child(
            Stack::create()
                .max_width(150.0)
                .child(TextBlock::create().text("textblock"))
                .child(
                    TextBlock::create()
                        .text("textblockhidden")
                        .visibility(Visibility::Hidden),
                )
                .child(TextBox::create().text("textbox").enabled(Enabled(false)))
                .child(
                    TextBox::create()
                        .water_mark(WaterMark::from("watermark..."))
                        .focused(Focused(true)),
                )
                .child(Button::create().text("button"))
                .child(
                    Button::create()
                        .text("btn")
                        .font_icon(FontIcon::from(CHECK_FONT_ICON))
                        .pressed(Pressed(true))
                        .enabled(Enabled(true)),
                )
                .child(ToggleButton::create().text("togglebutton"))
                .child(Switch::create())
                .child(
                    CheckBox::create()
                        .text("checkbox")
                        .font_icon(FontIcon::from(FLOPPY_FONT_ICON))
                        .selected(Selected(true))
                        .enabled(Enabled(true)),
                )
                .child(FontIconBlock::create().font_icon(FontIcon::from(CHECK_FONT_ICON)))
                .child(FontIconBlock::create().font_icon(FontIcon::from("")))
                .child(ImageWidget::create().image("src/bin/img/home.png"))
                .child(
                    WaterMarkTextBlock::create()
                        .text(String::new())
                        .water_mark(WaterMark::from("yyyy...")),
                ),
        )
    }
}

fn main() {
    let mut application = Application::default();

    application
        .create_window()
        .bounds((100.0, 100.0, 320.0, 260.0))
        .title("OrbTk - Primitive Widgets")
        .resizable(true)
        .root(MainView::create())
        //.debug_flag(true)
        .build();
    application.run();
}
