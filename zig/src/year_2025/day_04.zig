const std = @import("std");

const Grid = @import("../utils.zig").types.Grid;

const PAPER: u8 = '@';

const Cell = union(enum) {
    paper: u4,
    empty,
};

pub fn stringDimensions(input: []const u8) struct { usize, usize } {
    var newline_reached = false;
    var columns: usize = 0;
    var rows: usize = 0;
    var i: usize = 0;
    var step: usize = 1;
    while (i < input.len) : (i += step) {
        if (!newline_reached and input[i] == '\n') {
            newline_reached = true;
            step = @max(columns, 1);
        }
        if (newline_reached) {
            rows += 1;
        } else {
            columns += 1;
        }
    }
    return .{ rows, columns };
}

pub fn part01(input: []const u8) !u64 {
    const dimensions = stringDimensions(input);

    const griddy = try Grid(Cell).init(std.testing.allocator, dimensions[0], dimensions[1], .empty);
    defer griddy.deinit();

    {
        var lines = std.mem.splitScalar(u8, input, '\n');
        var row: usize = 0;
        while (lines.next()) |line| : (row += 1) {
            for (line, 0..) |char, column| {
                if (char == PAPER) {
                    if (griddy.get(row, column)) |cell| {
                        cell.* = .{ .paper = 0 };
                    }
                }
            }
        }
    }

    for (0..griddy.rows) |row| {
        for (0..griddy.columns) |column| {
            if (griddy.get(row, column)) |cell| {
                const cell_neighbors = switch (cell.*) {
                    .paper => |*neighbors| neighbors,
                    .empty => continue,
                };
                for (0..4) |offset| {
                    const rowOffset = @divFloor(offset, 3);
                    const columnOffset = offset % 3;
                    const r = row + rowOffset -% 1;
                    const c = column + columnOffset -% 1;
                    if (griddy.get(r, c)) |neighbor| {
                        const neighbor_neighbors = switch (neighbor.*) {
                            .paper => |*neighbors| neighbors,
                            .empty => continue,
                        };
                        cell_neighbors.* += 1;
                        neighbor_neighbors.* += 1;
                    }
                }
            }
        }
    }

    var accessible: u64 = 0;

    for (0..griddy.rows) |row| {
        for (0..griddy.columns) |column| {
            if (griddy.get(row, column)) |cell| {
                accessible += switch (cell.*) {
                    .paper => |neighbors| if (neighbors < 4) 1 else 0,
                    .empty => 0,
                };
            }
        }
    }

    return accessible;
}

pub fn part02(input: []const u8) !u64 {
    const dimensions = stringDimensions(input);

    const griddy = try Grid(Cell).init(std.testing.allocator, dimensions[0], dimensions[1], .empty);
    defer griddy.deinit();

    {
        var lines = std.mem.splitScalar(u8, input, '\n');
        var row: usize = 0;
        while (lines.next()) |line| : (row += 1) {
            for (line, 0..) |char, column| {
                if (char == PAPER) {
                    if (griddy.get(row, column)) |cell| {
                        cell.* = .{ .paper = 0 };
                    }
                }
            }
        }
    }

    for (0..griddy.rows) |row| {
        for (0..griddy.columns) |column| {
            if (griddy.get(row, column)) |cell| {
                const cell_neighbors = switch (cell.*) {
                    .paper => |*neighbors| neighbors,
                    .empty => continue,
                };
                for (0..4) |offset| {
                    const rowOffset = @divFloor(offset, 3);
                    const columnOffset = offset % 3;
                    const r = row + rowOffset -% 1;
                    const c = column + columnOffset -% 1;
                    if (griddy.get(r, c)) |neighbor| {
                        const neighbor_neighbors = switch (neighbor.*) {
                            .paper => |*neighbors| neighbors,
                            .empty => continue,
                        };
                        cell_neighbors.* += 1;
                        neighbor_neighbors.* += 1;
                    }
                }
            }
        }
    }

    var can_remove = true;
    var removed: u64 = 0;

    while (can_remove) {
        can_remove = false;
        for (0..griddy.rows) |row| {
            for (0..griddy.columns) |column| {
                if (griddy.get(row, column)) |cell| {
                    const cell_neighbors = switch (cell.*) {
                        .paper => |neighbors| neighbors,
                        .empty => continue,
                    };
                    if (cell_neighbors < 4) {
                        can_remove = true;
                        removed += 1;
                        cell.* = .empty;
                        for (0..9) |offset| {
                            const rowOffset = @divFloor(offset, 3);
                            const columnOffset = offset % 3;
                            const r = row + rowOffset -% 1;
                            const c = column + columnOffset -% 1;
                            if (row == r and column == c) continue;
                            if (griddy.get(r, c)) |neighbor| {
                                switch (neighbor.*) {
                                    .paper => |*neighbors| neighbors.* -= 1,
                                    .empty => {},
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    return removed;
}

test "part 01 practice 1" {
    const utils = @import("../utils.zig");
    const input = try utils.loadChallengeInput(std.testing.allocator, 2025, 4, "practice_1");
    defer std.testing.allocator.free(input);
    try std.testing.expectEqual(13, try part01(utils.trimString(input)));
}

test "part 01 final" {
    const utils = @import("../utils.zig");
    const input = try utils.loadChallengeInput(std.testing.allocator, 2025, 4, "final");
    defer std.testing.allocator.free(input);
    try std.testing.expectEqual(1344, try part01(utils.trimString(input)));
}

test "part 02 practice 1" {
    const utils = @import("../utils.zig");
    const input = try utils.loadChallengeInput(std.testing.allocator, 2025, 4, "practice_1");
    defer std.testing.allocator.free(input);
    try std.testing.expectEqual(43, try part02(utils.trimString(input)));
}

test "part 02 final" {
    const utils = @import("../utils.zig");
    const input = try utils.loadChallengeInput(std.testing.allocator, 2025, 4, "final");
    defer std.testing.allocator.free(input);
    try std.testing.expectEqual(8112, try part02(utils.trimString(input)));
}
