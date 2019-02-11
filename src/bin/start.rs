//! 第一个orbtk GUI：一个空白窗口

use orbtk::*;

fn main() {
    let mut application = Application::default();

    application
        .create_window()
        .title("OrbTk - simple gui")
        .root(Template::default())
        .bounds(Bounds::new(100f64, 100f64, 320.0, 210.0))
        .build();

    application.run();
}
