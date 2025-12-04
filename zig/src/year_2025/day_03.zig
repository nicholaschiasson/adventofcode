const std = @import("std");

pub fn part01(input: []const u8) !u64 {
    var sum: u64 = 0;
    var banks = std.mem.splitScalar(u8, input, '\n');
    while (banks.next()) |bank| {
        var joltages: [2]u64 = .{ 0, 0 };
        for (bank, 1..) |battery, i| {
            const joltage = battery - '0';
            if (i < bank.len and joltage > joltages[0]) {
                joltages[1] = 0;
                joltages[0] = joltage;
            } else if (joltage > joltages[1]) {
                joltages[1] = joltage;
            }
        }
        sum += @as(u64, (10 * joltages[0]) + joltages[1]);
    }
    return sum;
}

pub fn part02(input: []const u8) !u64 {
    var sum: u64 = 0;
    var banks = std.mem.splitScalar(u8, input, '\n');
    while (banks.next()) |bank| {
        var joltages: [12]u64 = .{ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 };
        for (bank, 0..) |battery, i| {
            const joltage = battery - '0';
            for (joltages, 0..) |_, j| {
                if (i + joltages.len - j - 1 < bank.len and joltage > joltages[j]) {
                    joltages[j] = joltage;
                    for (j + 1..joltages.len) |k| {
                        joltages[k] = 0;
                    }
                    break;
                }
            }
        }
        for (joltages, 0..) |_, j| {
            sum += @as(u64, joltages[joltages.len - 1 - j]) * std.math.pow(u64, 10, j);
        }
    }
    return sum;
}

test "part 01 practice 1" {
    const utils = @import("../utils.zig");
    const input = try utils.loadChallengeInput(std.testing.allocator, 2025, 3, "practice_1");
    defer std.testing.allocator.free(input);
    try std.testing.expectEqual(357, try part01(utils.trimString(input)));
}

test "part 01 final" {
    const utils = @import("../utils.zig");
    const input = try utils.loadChallengeInput(std.testing.allocator, 2025, 3, "final");
    defer std.testing.allocator.free(input);
    try std.testing.expectEqual(16812, try part01(utils.trimString(input)));
}

test "part 02 practice 1" {
    const utils = @import("../utils.zig");
    const input = try utils.loadChallengeInput(std.testing.allocator, 2025, 3, "practice_1");
    defer std.testing.allocator.free(input);
    try std.testing.expectEqual(3121910778619, try part02(utils.trimString(input)));
}

test "part 02 final" {
    const utils = @import("../utils.zig");
    const input = try utils.loadChallengeInput(std.testing.allocator, 2025, 3, "final");
    defer std.testing.allocator.free(input);
    try std.testing.expectEqual(166345822896410, try part02(utils.trimString(input)));
}
