//tst
//
//
//
//src: https://github.com/ziglang/zig/issues/12725#issuecomment-1236139872
var a: i32 = undefined;
fn foo() type {
    var b: i32 = undefined;
    _ = b;
    return struct {
        var a: i32 = undefined;
        fn bar() !void {
            return error.Bar;
        }
    };
}
fn foo2() type {
    {
        var m: i32 = undefined;
        _ = m;
    }
    return struct {
        var b: i32 = undefined;
        var c: struct {
            var d: i32 = undefined;
        } = undefined;
    };
}

test {
    const T = foo();
    try T.bar();
}
