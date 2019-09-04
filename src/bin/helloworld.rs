// author: lileilei
// lastedited: 2019.9

//! helloworld for rust orbtk

use orbtk::prelude::*;

fn main() {
    Application::new()
        .window(|ctx| {
            Window::create()
                .title("OrbTk - helloworld")
                .position((100.0, 100.0))
                .size(300.0, 200.0)
                .child(TextBlock::create().text("Hello World!").build(ctx))
                .build(ctx)
        })
        .run();
}