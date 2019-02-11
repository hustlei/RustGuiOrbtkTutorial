use orbtk::*;

struct MainView;

impl Widget for MainView {
    type Template = Template;
    fn create() -> self::Template {
        Template::default().child(
            Stack::create()
                .child(
                    Grid::create()
                        .margin(10.0)
                        .max_height(80.0)
                        .columns(Columns::create().column(100.0).column("Auto").build())
                        .rows(Rows::create().row(20.0).row(20.0).row("Auto").build())
                        .child(
                            TextBlock::create()
                                .attach_property(GridColumn(0))
                                .attach_property(GridRow(0)),
                        )
                        .child(
                            TextBlock::create()
                                .attach_property(GridColumn(0))
                                .attach_property(GridRow(1)),
                        )
                        .child(
                            TextBlock::create()
                                .attach_property(GridColumn(0))
                                .attach_property(GridRow(2)),
                        )
                        .child(
                            TextBlock::create()
                                .attach_property(GridColumn(1))
                                .attach_property(GridRow(0)),
                        )
                        .child(
                            TextBlock::create()
                                .attach_property(GridColumn(1))
                                .attach_property(GridRow(1)),
                        )
                        .child(
                            TextBlock::create()
                                .attach_property(GridColumn(1))
                                .attach_property(GridRow(2)),
                        ),
                )
                .child(
                    Stack::create()
                        .margin(10.0)
                        .max_height(40.0)
                        .orientation("Horizontol")
                        .child(TextBlock::create())
                        .child(TextBlock::create())
                        .child(TextBlock::create()),
                )
                .child(
                    Stack::create()
                        .child(
                            Container::create()
                                .padding(Padding::default())
                                .child(TextBlock::create().text("~~~~~~~")),
                        )
                        .child(
                            Container::create()
                                .padding((5.0, 20.0))
                                .child(TextBlock::create().text("+++++++")),
                        )
                        .child(
                            Container::create()
                                .padding(10.0)
                                .child(TextBlock::create().text("///////")),
                        ),
                ),
        )
    }
}

fn main() {
    let mut app = Application::default();

    app.create_window()
        .bounds((30.0, 30.0, 400.0, 300.0))
        .title("Orbtk-layout")
        .root(MainView::create())
        .resizable(true)
        .debug_flag(true)
        .build();

    app.run();
}
