const std = @import("std");

pub fn Grid(comptime T: type) type {
    return struct {
        allocator: std.mem.Allocator,
        rows: usize,
        columns: usize,
        data: []T,

        pub fn init(
            allocator: std.mem.Allocator,
            rows: usize,
            columns: usize,
            initial_value: T,
        ) !Grid(T) {
            const data = try allocator.alloc(T, rows * columns);
            for (0..rows * columns) |i| {
                data[i] = initial_value;
            }
            return .{
                .allocator = allocator,
                .rows = rows,
                .columns = columns,
                .data = data,
            };
        }

        pub fn deinit(self: Grid(T)) void {
            self.allocator.free(self.data);
        }

        pub fn get(self: Grid(T), row: usize, column: usize) ?*T {
            if (row >= self.rows or column >= self.columns) {
                return null;
            }
            return &self.data[row * self.columns + column];
        }
    };
}
