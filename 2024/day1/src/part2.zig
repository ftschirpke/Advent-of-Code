const std = @import("std");
const aoc = @import("aoc_lib");
const assert = std.debug.assert;

fn less_than(context: void, a: i32, b: i32) std.math.Order {
    _ = context;
    return std.math.order(a, b);
}

const NumPQ = std.PriorityQueue(i32, void, less_than);

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

    var left_nums = NumPQ.init(allocator, {});
    var right_nums = NumPQ.init(allocator, {});
    defer left_nums.deinit();
    defer right_nums.deinit();

    while (true) {
        var line_buffer: [14]u8 = undefined;
        const buffer_slice = line_buffer[0..];
        const bytes_read = try file.read(buffer_slice);

        if (bytes_read == 0) {
            break;
        }

        assert(bytes_read == 14);
        assert(buffer_slice[13] == '\n');
        assert(buffer_slice[5] == ' ');
        assert(buffer_slice[6] == ' ');
        assert(buffer_slice[7] == ' ');

        const left = try std.fmt.parseInt(i32, buffer_slice[0..5], 10);
        const right = try std.fmt.parseInt(i32, buffer_slice[8..13], 10);

        try left_nums.add(left);
        try right_nums.add(right);
    }

    assert(left_nums.count() == right_nums.count());

    try run(&left_nums, &right_nums);
}

fn run(left_nums: *NumPQ, right_nums: *NumPQ) !void {
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    var sum: u32 = 0;
    while (left_nums.count() > 0 and right_nums.count() > 0) {
        const left = left_nums.peek() orelse 0;
        const right = right_nums.peek() orelse 0;
        if (left < right) {
            _ = left_nums.remove();
        } else if (right < left) {
            _ = right_nums.remove();
        } else {
            const common_value = left;
            var left_occurrences: u32 = 0;
            while (left_nums.peek() == common_value) : (left_occurrences += 1) {
                _ = left_nums.remove();
            }
            var right_occurrences: u32 = 0;
            while (right_nums.peek() == common_value) : (right_occurrences += 1) {
                _ = right_nums.remove();
            }
            sum += @abs(common_value) * left_occurrences * right_occurrences;
        }
    }

    try stdout.print("Total sum of multiplied occurrences: {d}\n", .{sum});

    try bw.flush();
}
