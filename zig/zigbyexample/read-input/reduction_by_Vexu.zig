fn foo() type {
    return struct {
        fn bar() !void {
            return error.Bar;
        }
    };
}

test {
    const T = foo();
    try T.bar();
}
