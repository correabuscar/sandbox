const print = std.debug.print;
const std = @import("std");

pub fn main() !void {
    var u: u8 = undefined; //XXX: if this becomes 'var' then no error below:
    var w: u8 = u + 1; // undef-tst.zig:7:17: error: use of undefined value here causes undefined behavior
    //var x: u8 = undefined / 1;
    //var xx:Undefined=undefined;
    print("Hi! {}\n", .{u});
    print("{}\n", .{u == undefined}); // false
    print("{}\n", .{undefined == u}); // true
    // XXX: ^ wtf?
    //print("Hi! {}\n", .{undefined == undefined}); // error: operator == not allowed for type '@TypeOf(undefined)'  //This makes some sense.
    u = u + 1;
    print("Hi! {}\n", .{w});
    //print("Hi! {}\n", .{x});
    print("Hi! {}\n", .{@TypeOf(undefined)});
    const r: @TypeOf(undefined) = undefined;
    _ = r;
    print("{u8}\n", .{r});
}
