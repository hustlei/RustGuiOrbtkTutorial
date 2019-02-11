//! helloworld for rust orbtk

use orbtk::*;

// 分解步骤版helloworld
/*
fn main() {
    let mut application = Application::default();
    //创建一个textblock，添加一个Label类型的属性
    let text_block = TextBlock::create().text("Hello, World!");
    //创建root控件，把textBlock设置为子控件
    let root = Template::default().child(text_block);

    application
        .create_window()
        .bounds(Bounds::new(100.0, 100., 320., 210.))
        .title("OrbTk - helloworld")
        .resizable(true) //可以调整窗口大小
        .root(root)
        .build();
    application.run();
}
*/

fn main() {
    let mut application = Application::default();

    let root = Template::default().child(TextBlock::create().text("Hello, World!"));

    application
        .create_window()
        .bounds((100.0, 100., 320., 210.))
        .title("OrbTk - helloworld")
        .resizable(true)
        .root(root)
        .build();
    application.run();
}
