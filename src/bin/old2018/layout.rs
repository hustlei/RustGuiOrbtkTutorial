use orbtk::*;

struct MainView;

impl Widget for MainView {
    fn create() -> Template {
        Template::default()
            .as_parent_type(ParentType::Single)
            .with_child(
                Row::create()
                    .with_child(
                        Column::create()
                            .with_child(TextBlock::create())
                            .with_child(TextBlock::create())
                            .with_child(TextBlock::create())
                            .with_child(TextBlock::create()),
                    )
                    .with_child(Spacer::create())
                    .with_child(
                        Column::create()
                            .with_child(
                                Container::create()
                                    .with_child(TextBlock::create())
                                    .with_property(Padding::default().with(20)),
                            )
                            .with_child(
                                Container::create()
                                    .with_child(TextBlock::create())
                                    .with_property(Padding::default().with_left(20)),
                            ),
                    )
                    .with_child(Spacer::create())
                    .with_child(
                        Stack::create()
                            .with_child(TextBox::create())
                            .with_child(TextBlock::create())
                            .with_child(TextBlock::create().with_property(Label::from("~~~~~~~"))),
                    ),
            )
    }
}

fn main() {
    let mut app = Application::default();

    app.create_window()
        .with_bounds(Bounds::new(30, 30, 400, 300))
        .with_title("Orbtk-layout")
        .with_root(MainView::create())
        .with_resizable(true)
        .with_debug_flag(true)
        .build();

    app.run();
}
