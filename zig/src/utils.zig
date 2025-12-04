const std = @import("std");

pub fn readFile(allocator: std.mem.Allocator, path: []const u8) ![]u8 {
    const file = try std.fs.cwd().openFile(path, .{ .mode = .read_only });
    const stat = try file.stat();
    file.close();

    return try std.fs.cwd().readFileAlloc(allocator, path, stat.size);
}

pub fn loadChallengeInput(allocator: std.mem.Allocator, year: u16, day: u8, testName: []const u8) ![]const u8 {
    const path = try std.fmt.allocPrint(allocator, "../rsrc/inputs/year_{d}/day_{d:0>2}/tests/{s}.txt", .{ year, day, testName });
    defer allocator.free(path);
    return try readFile(allocator, path);
}

pub fn trimString(input: []const u8) []const u8 {
    return std.mem.trim(u8, input, &[_]u8{ ' ', '\t', '\n', '\r' });
}
