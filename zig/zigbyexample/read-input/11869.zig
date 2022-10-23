const std = @import("std");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{ .stack_trace_frames = 8 }){};
    defer std.debug.assert(!gpa.deinit());
    const alloc = gpa.allocator();

    const xdd = std.os.getenv("XDG_DATA_DIRS").?;
    var iter = std.mem.split(u8, xdd, ":");
    var paths = std.ArrayList([]const u8).init(alloc);
    errdefer paths.deinit();
    while (iter.next()) |entry| try paths.append(entry);

    const list = paths.toOwnedSlice();
    defer alloc.free(list);
    const file: std.fs.File = for (list) |item| {
        const fpath = try std.fmt.allocPrint(alloc, "{s}/icons/Adwaita/scalable/places/folder-symbolic.svg", .{item});
        defer alloc.free(fpath);
        break std.fs.cwd().openFile(fpath, .{}) catch |err| switch (err) {
            error.FileNotFound => continue,
            else => |e| return e,
        };
    } else unreachable;
    defer file.close();
}
