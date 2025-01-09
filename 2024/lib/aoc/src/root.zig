//! By convention, root.zig is the root source file when making a library. If
//! you are making an executable, the convention is to delete this file and
//! start with main.zig instead.
const std = @import("std");
const testing = std.testing;

pub export fn say_hello() void {
    std.log.debug("Hi there from the my Zig helper library for Advent of Code 2024\n", .{});
}
