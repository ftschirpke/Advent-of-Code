const std = @import("std");
const aoc = @import("aoc_lib");
const mecha = @import("mecha");

const number_parser = mecha.int(i64, .{
    .parse_sign = false,
});

const ws = mecha.ascii.whitespace.many(.{ .collect = false }).discard();

const followup_parser = mecha.combine(.{
    ws,
    number_parser,
});

const line_parser = mecha.many(followup_parser, .{});

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

    var buf_reader = std.io.bufferedReader(file.reader());
    var reader = buf_reader.reader();

    var line = std.ArrayList(u8).init(allocator);
    defer line.deinit();

    var safe_reports: u32 = 0;

    const writer = line.writer();
    var line_no: usize = 0;
    while (reader.streamUntilDelimiter(writer, '\n', null)) : (line_no += 1) {
        defer line.clearRetainingCapacity();

        const parsed_line_res = (try line_parser.parse(allocator, line.items));
        const parsed_line = switch (parsed_line_res.value) {
            .ok => |parsed_line| parsed_line,
            .err => |err| return err,
        };
        defer allocator.free(parsed_line_res.value.ok);

        if (validate_report(parsed_line)) {
            safe_reports += 1;
        }
    } else |err| switch (err) {
        error.EndOfStream => {},
        else => return err,
    }

    try run(safe_reports);
}

fn validate_report(report: []i64) bool {
    if (is_valid(report, null)) {
        return true;
    }
    for (0..report.len) |idx| {
        if (is_valid(report, idx)) {
            return true;
        }
    }
    return false;
}

fn is_valid(report: []i64, without_index: ?usize) bool {
    var increasing: ?bool = null;
    var last: ?i64 = null;

    for (0.., report) |i, value| {
        if (without_index != null and without_index.? == i) {
            continue;
        }
        if (last == null) {
            last = value;
            continue;
        }
        const last_value = last.?;
        if (increasing) |inc| {
            const step = is_increasing_diff(last_value, value);
            if (step == null or step.? != inc) {
                return false;
            }
        } else {
            increasing = is_increasing_diff(last_value, value);
            if (increasing == null) {
                return false;
            }
        }
        last = value;
    }
    return true;
}

fn is_increasing_diff(prev: i64, value: i64) ?bool {
    const diff: i64 = value - prev;
    if (-3 <= diff and diff <= -1) {
        return false;
    } else if (1 <= diff and diff <= 3) {
        return true;
    } else {
        return null;
    }
}

fn run(count: u32) !void {
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    try stdout.print("Total number of safe reports: {}\n", .{count});

    try bw.flush();
}

test "parsing memory leaks" {
    const allocator = std.testing.allocator;

    const parsed_line_res = (try line_parser.parse(allocator, " 12 13 14 15"));
    _ = switch (parsed_line_res.value) {
        .ok => |parsed_line| parsed_line,
        .err => |err| return err,
    };
    defer allocator.free(parsed_line_res.value.ok);
}
