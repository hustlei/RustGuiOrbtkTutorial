//! rust orbtk primitive widgets with properties
use orbtk::properties::Visibility;
use orbtk::*;

struct MainView;

impl Widget for MainView {
    type Template = Template;

    fn create() -> self::Template {
        Template::default().child(
            Stack::create()
                .max_width(140.0)
                .child(
                    TextBlock::create()
                        .text("textblock")
                        .visibility(Visibility::Hidden),
                )
                .child(TextBox::create().text("textboxfocused").focused(true))
                .child(TextBox::create().text("textboxdisabled").enabled(false))
                .child(TextBox::create().water_mark("textboxwatermark..."))
                .child(Button::create().text("button").margin((0.0, 2.0)))
                .child(
                    Button::create()
                        .text("primary button")
                        .margin((0.0, 2.0))
                        .selector(Selector::from("button").class("primary")),
                )
                .child(
                    Button::create()
                        .text("button with icon")
                        .margin((0.0, 2.0))
                        .font_icon(FontIcon::from(CHECK_FONT_ICON)),
                )
                .child(
                    ToggleButton::create()
                        .text("togglebuttontoggled")
                        .margin((0.0, 2.0))
                        .pressed(Pressed(true)),
                )
                .child(Switch::create().selected(true))
                .child(
                    CheckBox::create()
                        .text("checkbox with icon")
                        .font_icon(FontIcon::from(FLOPPY_FONT_ICON)),
                )
                .child(FontIconBlock::create().font_icon(FontIcon::from(CHECK_FONT_ICON)))
                .child(FontIconBlock::create().font_icon(FontIcon::from("")))
                .child(ImageWidget::create().image("src/bin/img/home.png"))
                .child(
                    WaterMarkTextBlock::create()
                        .text(String::new())
                        .water_mark(WaterMark::from("WaterMarkTextBlock...")),
                ),
        )
    }
}

fn main() {
    let mut application = Application::default();

    application
        .create_window()
        .bounds((100.0, 100.0, 300.0, 400.0))
        .title("OrbTk - Primitive Widgets with properties")
        .resizable(true)
        .root(MainView::create())
        //.debug_flag(true)
        .build();
    application.run();
}
