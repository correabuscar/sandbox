// code from: https://github.com/gyscos/cursive/issues/776
use cursive::menu::Tree;
use cursive::event::Key;

fn main() {
    let mut siv = cursive::default();
    siv.menubar().add_subtree("foo", Tree::new());
    siv.add_global_callback(Key::Esc, |s| s.select_menubar());
    siv.run();
    // Then press escape, go to foo and press enter.
}
