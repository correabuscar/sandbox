//code from: https://github.com/gyscos/cursive/issues/780
use cursive::{views::TextView, Cursive, CursiveExt};

fn main() {
    let mut app = Cursive::new();
    app.add_layer(TextView::new("Hello Null\0byte: \0and after\0\0\0\0\0\0\0\0\0\0"));
    //app.add_layer(TextView::new("Hello non-Null"));
    app.add_global_callback('q', |a| a.quit());
    app.run();
}
