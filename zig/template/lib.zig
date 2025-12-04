const std = @import("std");

pub fn part01(input: []const u8) !u64 {
    @panic("todo");
}

pub fn part02(input: []const u8) !u64 {
    @panic("todo");
}

test "part 01 practice 1" {
    const utils = @import("../utils.zig");
    const input = try utils.loadChallengeInput(std.testing.allocator, 2025, 1, "practice_1");
    defer std.testing.allocator.free(input);
    try std.testing.expectEqual(0, try part01(utils.trimString(input)));
}

test "part 01 final" {
    const utils = @import("../utils.zig");
    const input = try utils.loadChallengeInput(std.testing.allocator, 2025, 1, "final");
    defer std.testing.allocator.free(input);
    try std.testing.expectEqual(0, try part01(utils.trimString(input)));
}

test "part 02 practice 1" {
    const utils = @import("../utils.zig");
    const input = try utils.loadChallengeInput(std.testing.allocator, 2025, 1, "practice_1");
    defer std.testing.allocator.free(input);
    try std.testing.expectEqual(0, try part02(utils.trimString(input)));
}

test "part 02 final" {
    const utils = @import("../utils.zig");
    const input = try utils.loadChallengeInput(std.testing.allocator, 2025, 1, "final");
    defer std.testing.allocator.free(input);
    try std.testing.expectEqual(0, try part02(utils.trimString(input)));
}
