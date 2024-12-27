const std = @import("std");
const aoc = @import("aoc_lib");

pub fn main() !void {
    const cwd = std.fs.cwd();
    const file = cwd.openFile("input.txt", .{}) catch |err| {
        if (err == std.fs.File.OpenError.FileNotFound) {
            std.log.debug("There should exist a 'input.txt' file in the current working directory.", .{});
        }
        return err;
    };
    defer file.close();

    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();

    var buffer: [10]u8 = undefined;
    const buffer_slice = buffer[0..];
    const bytes_read = try file.read(buffer_slice);
    std.log.debug("{}", .{bytes_read});
    // TODO: implement parsing

    try run();
}

fn run() !void {
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    aoc.say_hello();

    try stdout.print("Hi, welcome to part 1.\n", .{});
    // TODO: implement part 1

    try bw.flush();
}
