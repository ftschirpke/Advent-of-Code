const std = @import("std");
const aoc = @import("aoc_lib");
const mecha = @import("mecha");

const part1 = @import("part1.zig");
const Multiply = part1.Multiply;
const mul_parser = part1.mul_parser;

const do_parser = mecha.combine(.{
    mecha.ascii.char('d').discard(),
    mecha.ascii.char('o').discard(),
    mecha.ascii.char('(').discard(),
    mecha.ascii.char(')').discard(),
});

const dont_parser = mecha.combine(.{
    mecha.ascii.char('d').discard(),
    mecha.ascii.char('o').discard(),
    mecha.ascii.char('n').discard(),
    mecha.ascii.char('\'').discard(),
    mecha.ascii.char('t').discard(),
    mecha.ascii.char('(').discard(),
    mecha.ascii.char(')').discard(),
});

fn parse(file: std.fs.File, allocator: std.mem.Allocator) !std.ArrayList(Multiply) {
    var buf_reader = std.io.bufferedReader(file.reader());
    var reader = buf_reader.reader();

    var line = std.ArrayList(u8).init(allocator);
    defer line.deinit();

    var instructions = std.ArrayList(Multiply).init(allocator);

    const writer = line.writer();
    var active: bool = true;

    var line_no: usize = 0;
    while (reader.streamUntilDelimiter(writer, '\n', null)) : (line_no += 1) {
        defer line.clearRetainingCapacity();

        var i: usize = 0;
        while (i < line.items.len) {
            const c = line.items[i];
            if (c == 'm' and active) {
                const parsed_line_res = (try mul_parser.parse(allocator, line.items[i..]));
                switch (parsed_line_res.value) {
                    .ok => |mul| {
                        i += parsed_line_res.index;
                        try instructions.append(mul);
                    },
                    .err => i += 1,
                }
            } else if (c == 'd') {
                const parser = if (active) dont_parser else do_parser;
                const parsed_line_res = (try parser.parse(allocator, line.items[i..]));
                switch (parsed_line_res.value) {
                    .ok => {
                        i += parsed_line_res.index;
                        active = !active;
                    },
                    .err => i += 1,
                }
            } else {
                i += 1;
            }
        }
    } else |err| switch (err) {
        error.EndOfStream => {},
        else => return err,
    }

    return instructions;
}

pub fn main() !void {
    const cwd = std.fs.cwd();
    const file = cwd.openFile("input.txt", .{}) catch |err| {
        if (err == std.fs.File.OpenError.FileNotFound) {
            std.log.debug("[PART 2] There should exist a 'input.txt' file in the current working directory.", .{});
        }
        return err;
    };
    defer file.close();

    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const instructions = try parse(file, allocator);
    defer instructions.deinit();

    try part1.run(&instructions);
}

test "parsing memory leaks" {
    const testing = std.testing;
    const cwd = std.fs.cwd();
    const file = cwd.openFile("input.txt", .{}) catch |err| {
        if (err == std.fs.File.OpenError.FileNotFound) {
            std.log.debug("[TEST] There should exist a 'input.txt' file in the current working directory.", .{});
        }
        return err;
    };
    defer file.close();

    const instructions = try parse(file, testing.allocator);
    defer instructions.deinit();
}
