pub const year_2025 = struct {
    pub const day_01 = @import("year_2025/day_01.zig");
    pub const day_02 = @import("year_2025/day_02.zig");
};

pub const utils = @import("utils.zig");

test {
    @import("std").testing.refAllDeclsRecursive(@This());
}
