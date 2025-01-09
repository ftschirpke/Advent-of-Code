const std = @import("std");
const aoc = @import("aoc_lib");

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

    var buf_reader = std.io.bufferedReader(file.reader());
    var reader = buf_reader.reader();

    var line = std.ArrayList(u8).init(allocator);
    defer line.deinit();

    var valid_reports: u64 = 0;

    const writer = line.writer();
    var line_no: usize = 0;
    while (reader.streamUntilDelimiter(writer, '\n', null)) : (line_no += 1) {
        defer line.clearRetainingCapacity();

        var increasing: ?bool = null;
        var last: ?i64 = null;
        var last_delimiter: ?usize = null;
        var valid: bool = true;

        for (0.., line.items) |i, c| {
            if (std.ascii.isDigit(c)) {
                continue;
            }
            var num_start: usize = undefined;
            if (last_delimiter) |last_delim| {
                num_start = last_delim + 1;
            } else {
                num_start = 0;
            }

            last_delimiter = i;
            if (i <= num_start) {
                continue;
            }
            const value = try std.fmt.parseInt(i64, line.items[num_start..i], 10);
            if (last == null) {
                last = value;
                continue;
            }
            const last_value = last.?;
            if (increasing) |inc| {
                const step = is_increasing_diff(last_value, value);
                if (step == null or step.? != inc) {
                    valid = false;
                    break;
                }
            } else {
                increasing = is_increasing_diff(last_value, value);
                if (increasing == null) {
                    valid = false;
                    break;
                }
            }
            last = value;
        }

        if (valid) {
            var num_start: usize = undefined;
            if (last_delimiter) |last_delim| {
                num_start = last_delim + 1;
            } else {
                num_start = 0;
            }

            const value = try std.fmt.parseInt(i64, line.items[num_start..], 10);
            const last_value = last.?;
            if (increasing) |inc| {
                const step = is_increasing_diff(last_value, value);
                if (step == null or step.? != inc) {
                    valid = false;
                }
            } else {
                increasing = is_increasing_diff(last_value, value);
                if (increasing == null) {
                    valid = false;
                }
            }
        }
        // std.log.debug("[{d:>4}] {s} {}", .{ line_no, line.items, valid });
        if (valid) {
            valid_reports += 1;
        }
    } else |err| switch (err) {
        error.EndOfStream => {},
        else => return err,
    }

    std.log.debug("Number of valid reports: {d}", .{valid_reports});
}
