const std = @import("std");

pub fn part01(input: []const u8) !u64 {
    var num: i32 = 50;
    var count: u64 = 0;

    var lines = std.mem.splitSequence(u8, input, "\n");
    while (lines.next()) |line| {
        const sign: i32 = if (line[0] == 'L') -1 else 1;
        const step: i32 = try std.fmt.parseInt(i32, line[1..], 10);
        num = @mod(num + (sign * step), 100);
        if (num == 0) {
            count += 1;
        }
    }

    return count;
}

pub fn part02(input: []const u8) !u64 {
    var num: i32 = 50;
    var count: u64 = 0;

    var lines = std.mem.splitSequence(u8, input, "\n");
    while (lines.next()) |line| {
        const sign: i32 = if (line[0] == 'L') -1 else 1;
        const step: i32 = try std.fmt.parseInt(i32, line[1..], 10);
        const prevDiff: i32 = if (sign > 0) num else @mod(100 - num, 100);
        count += @intCast(try std.math.divFloor(i32, step + prevDiff, 100));
        num = @mod(num + (sign * step), 100);
    }

    return count;
}

test "part 01 practice 1" {
    const gpa = std.testing.allocator;
    const input = try readFile(gpa, "../rsrc/inputs/year_2025/day_01/tests/practice_1.txt");
    defer gpa.free(input);

    const trimmed = std.mem.trim(u8, input, &[_]u8{ ' ', '\t', '\n', '\r' });
    try std.testing.expectEqual(3, try part01(trimmed));
}

test "part 01 final" {
    const gpa = std.testing.allocator;
    const input = try readFile(gpa, "../rsrc/inputs/year_2025/day_01/tests/final.txt");
    defer gpa.free(input);

    const trimmed = std.mem.trim(u8, input, &[_]u8{ ' ', '\t', '\n', '\r' });
    try std.testing.expectEqual(1129, try part01(trimmed));
}

test "part 02 practice 1" {
    const gpa = std.testing.allocator;
    const input = try readFile(gpa, "../rsrc/inputs/year_2025/day_01/tests/practice_1.txt");
    defer gpa.free(input);

    const trimmed = std.mem.trim(u8, input, &[_]u8{ ' ', '\t', '\n', '\r' });
    try std.testing.expectEqual(6, try part02(trimmed));
}

test "part 02 final" {
    const gpa = std.testing.allocator;
    const input = try readFile(gpa, "../rsrc/inputs/year_2025/day_01/tests/final.txt");
    defer gpa.free(input);

    const trimmed = std.mem.trim(u8, input, &[_]u8{ ' ', '\t', '\n', '\r' });
    try std.testing.expectEqual(6638, try part02(trimmed));
}

pub fn readFile(allocator: std.mem.Allocator, path: []const u8) ![]u8 {
    const file = try std.fs.cwd().openFile(path, .{ .mode = .read_only });
    const stat = try file.stat();
    file.close();

    return try std.fs.cwd().readFileAlloc(allocator, path, stat.size);
}
