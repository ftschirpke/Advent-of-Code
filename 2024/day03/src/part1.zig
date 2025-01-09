const std = @import("std");
const aoc = @import("aoc_lib");
const mecha = @import("mecha");

const ws = mecha.ascii.whitespace.many(.{ .collect = false }).discard();

pub const mul_parser = mecha.combine(.{
    mecha.ascii.char('m').discard(),
    mecha.ascii.char('u').discard(),
    mecha.ascii.char('l').discard(),
    mecha.ascii.char('(').discard(),
    mecha.int(i64, .{}),
    ws,
    mecha.ascii.char(',').discard(),
    ws,
    mecha.int(i64, .{}),
    mecha.ascii.char(')').discard(),
}).map(mecha.toStruct(Multiply));

pub const Multiply = struct {
    a: i64,
    b: i64,
};

fn parse(file: std.fs.File, allocator: std.mem.Allocator) !std.ArrayList(Multiply) {
    var buf_reader = std.io.bufferedReader(file.reader());
    var reader = buf_reader.reader();

    var line = std.ArrayList(u8).init(allocator);
    defer line.deinit();

    var instructions = std.ArrayList(Multiply).init(allocator);

    const writer = line.writer();
    var line_no: usize = 0;
    while (reader.streamUntilDelimiter(writer, '\n', null)) : (line_no += 1) {
        defer line.clearRetainingCapacity();

        var i: usize = 0;
        while (i < line.items.len) {
            const parsed_line_res = (try mul_parser.parse(allocator, line.items[i..]));
            switch (parsed_line_res.value) {
                .ok => |mul| {
                    i += parsed_line_res.index;
                    try instructions.append(mul);
                },
                .err => i += 1,
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
            std.log.debug("[PART 1] There should exist a 'input.txt' file in the current working directory.", .{});
        }
        return err;
    };
    defer file.close();

    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const instructions = try parse(file, allocator);
    defer instructions.deinit();

    try run(&instructions);
}

pub fn run(instructions: *const std.ArrayList(Multiply)) !void {
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    try stdout.print("Successfully parsed {d} mul instructions\n", .{instructions.items.len});

    var sum: i64 = 0;
    for (instructions.items) |inst| {
        sum += inst.a * inst.b;
    }

    try stdout.print("The mul instructions sum up to {d}\n", .{sum});

    try bw.flush();
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
