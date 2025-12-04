const std = @import("std");

pub fn part01(input: []const u8) !u64 {
    var sum: u64 = 0;

    var ranges = std.mem.splitScalar(u8, input, ',');
    while (ranges.next()) |range| {
        var rangeBounds = std.mem.splitScalar(u8, range, '-');
        const start = try std.fmt.parseInt(u64, rangeBounds.next().?, 10);
        const end = try std.fmt.parseInt(u64, rangeBounds.next().?, 10);
        for (start..end + 1) |n| {
            var buf: [256]u8 = undefined;
            const numStr = try std.fmt.bufPrint(&buf, "{d}", .{n});
            if (numStr.len % 2 != 0) continue;
            const lenOver2 = numStr.len / 2;
            if (std.mem.eql(u8, numStr[0..lenOver2], numStr[lenOver2..])) {
                sum += n;
            }
        }
    }

    return sum;
}

pub fn part02(input: []const u8) !u64 {
    var sum: u64 = 0;

    var ranges = std.mem.splitScalar(u8, input, ',');
    while (ranges.next()) |range| {
        var rangeBounds = std.mem.splitScalar(u8, range, '-');
        const start = try std.fmt.parseInt(u64, rangeBounds.next().?, 10);
        const end = try std.fmt.parseInt(u64, rangeBounds.next().?, 10);
        for (start..end + 1) |n| {
            var buf: [256]u8 = undefined;
            const numStr = try std.fmt.bufPrint(&buf, "{d}", .{n});
            lenloop: for (2..numStr.len + 1) |l| {
                if (numStr.len % l != 0) continue;
                const sublen = numStr.len / l;
                for (0..l) |offset| {
                    if (!std.mem.eql(u8, numStr[0..sublen], numStr[(sublen * offset) .. (sublen * offset) + sublen])) {
                        continue :lenloop;
                    }
                }
                sum += n;
                break;
            }
        }
    }

    return sum;
}

test "part 01 practice 1" {
    const utils = @import("../utils.zig");
    const input = try utils.loadChallengeInput(std.testing.allocator, 2025, 2, "practice_1");
    defer std.testing.allocator.free(input);
    try std.testing.expectEqual(1227775554, try part01(utils.trimString(input)));
}

test "part 01 final" {
    const utils = @import("../utils.zig");
    const input = try utils.loadChallengeInput(std.testing.allocator, 2025, 2, "final");
    defer std.testing.allocator.free(input);
    try std.testing.expectEqual(31000881061, try part01(utils.trimString(input)));
}

test "part 02 practice 1" {
    const utils = @import("../utils.zig");
    const input = try utils.loadChallengeInput(std.testing.allocator, 2025, 2, "practice_1");
    defer std.testing.allocator.free(input);
    try std.testing.expectEqual(4174379265, try part02(utils.trimString(input)));
}

test "part 02 final" {
    const utils = @import("../utils.zig");
    const input = try utils.loadChallengeInput(std.testing.allocator, 2025, 2, "final");
    defer std.testing.allocator.free(input);
    try std.testing.expectEqual(46769308485, try part02(utils.trimString(input)));
}
