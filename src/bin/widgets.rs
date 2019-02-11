//! rust orbtk primitive widgets

// primitive widgets分解步骤版demo
/*
use orbtk::*;
//use orbtk::styling::vector_graphics::material_font_icons::CHECK_FONT_ICON;

fn main() {
    let mut application = Application::default();

    //创建控件
    let text_block = TextBlock::create().text("textblock");
    let text_box = TextBox::create().text("textbox");
    let water_mark_text_box = TextBox::create().water_mark(WaterMark::from("watermark..."));
    let btn = Button::create().text("button");
    let toggle_btn = ToggleButton::create().text("togglebutton");
    let switch = Switch::create();
    let check_box = CheckBox::create().text("checkbox");
    let font_icon_block = FontIconBlock::create().font_icon(FontIcon::from(CHECK_FONT_ICON));
    let image_widget = ImageWidget::create().image("src/bin/img/home.png");
    let water_mark_text_block = WaterMarkTextBlock::create()
        .text(String::new())
        .water_mark(WaterMark::from("yyyy..."));

    let layout = Stack::create()
        .width(150.0)
        .child(text_block)
        .child(text_box)
        .child(water_mark_text_box)
        .child(btn)
        .child(toggle_btn)
        .child(switch)
        .child(check_box)
        .child(font_icon_block)
        .child(image_widget)
        .child(water_mark_text_block);

    let root = Template::default().child(layout);

    application
        .create_window()
        .bounds(Bounds::new(100.0, 100.0, 320.0, 260.0))
        .title("OrbTk - Widgets")
        .resizable(true)
        .root(root)
        //.debug_flag(true)
        .build();

    application.run();
}
*/
//primitive widgets 最终版demo

use orbtk::*;

struct MainView;

impl Widget for MainView {
    type Template = Template;

    fn create() -> self::Template {
        Template::default().child(
            Stack::create()
                .max_width(150.0)
                .child(TextBlock::create().text("textblock"))
                .child(TextBox::create().text("textbox"))
                .child(TextBox::create().water_mark(WaterMark::from("watermark...")))
                .child(Button::create().text("button"))
                .child(ToggleButton::create().text("togglebutton"))
                .child(Switch::create())
                .child(CheckBox::create().text("checkbox"))
                .child(FontIconBlock::create().font_icon(FontIcon::from(CHECK_FONT_ICON)))
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
        .title("OrbTk - Widgets")
        .resizable(true)
        .root(MainView::create())
        //.debug_flag(true)
        .build();
    application.run();
}
