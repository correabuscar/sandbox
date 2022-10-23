const std = @import("std");

pub fn main() void {
    std.debug.print("Hello, {s}!\n", .{"World"});
    std.debug.print("Hello World!\n", .{});
}
