struct Point {
    x: i32,
    y: i32,
}

#[allow(clippy::many_single_char_names)]
fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    #[allow(non_shorthand_field_patterns)]
    {
        let Point { x: x, y: y } = p;
        assert_eq!(0, x);
        assert_eq!(7, y);
    }

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}
