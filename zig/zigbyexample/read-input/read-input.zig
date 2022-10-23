const std = @import("std");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const stdin = std.io.getStdIn();

    std.debug.print("Enter input: ", .{});
    //std.debug.print("input: ");
    const input = try stdin.reader().readUntilDelimiterAlloc(allocator, '\n', 1);
    defer allocator.free(input);

    std.debug.print("Gotten value: {s}\n", .{input});
}
