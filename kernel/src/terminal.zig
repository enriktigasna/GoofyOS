const std = @import("std");
const framebuffer = @import("framebuffer.zig");
const font = @import("font.zig");

pub const Terminal = struct {
    rows: u16, //
    cols: u16,
    cursor_x: u16,
    cursor_y: u16,
    foreground: u4,
    background: u4,
    colors: [16]u24,
};

pub var term: Terminal = undefined;

pub const ANSIColors: [16]u24 = [16]u24{
    0x000000, // Black
    0x555555, // Black Light
    0xFF0000, // Red
    0xFF5555, // Red Light
    0x00FF00, // Green
    0x55FF55, // Green Light
    0xFFFF00, // Yellow
    0xFFFF55, // Yello Light
    0x0000FF, // Blue
    0x5555FF, // Blue Light
    0xFF00FF, // Purple
    0xFF55FF, // Purple Light
    0x00FFFF, // Cyan
    0x55FFFF, // Cyan Light
    0xAAAAAA, // White
    0xFFFFFF, // White Light
};

pub fn write(char: u8) void {
    if (char == 10) {
        term.cursor_y +%= 1;
        term.cursor_x = 0;
    }

    if (char == 32) {
        if (term.cursor_x > term.cols - 2) {
            term.cursor_y +%= 1;
            term.cursor_x = 0;
        } else {
            term.cursor_x +%= 1;
        }
    }
    if (char >= 33 and char <= 126) {
        const x_offset: u16 = term.cursor_x * 8;
        const y_offset: u16 = term.cursor_y * 16;

        const foreground_color: u24 = term.colors[term.foreground];
        const background_color: u24 = term.colors[term.background];

        const bitmap_char: []u8 = @constCast(&font.font[char]);

        framebuffer.drawBitmap(x_offset, y_offset, foreground_color, background_color, 16, 8, bitmap_char);
        if (term.cursor_x > term.cols - 2) {
            term.cursor_y +%= 1;
            term.cursor_x = 0;
        } else {
            term.cursor_x +%= 1;
        }
    }
}

pub fn write_string(_: @TypeOf(.{}), string: []const u8) !usize {
    var counter: usize = 0;
    for (string) |char| {
        write(char);
        counter +%= 1;
    }
    return counter;
}

const Writer = std.io.Writer(@TypeOf(.{}), error{}, write_string);
const writer = Writer{ .context = .{} };

pub fn write_fmt(comptime format: []const u8, args: anytype) void {
    try writer.print(format, args);
}

pub fn init() void {
    term = Terminal{
        .rows = @intCast(framebuffer.fb.height / 16),
        .cols = @intCast(framebuffer.fb.width / 8),
        .cursor_x = 0,
        .cursor_y = 0,
        .foreground = 15, // Indexes of ANSIColors
        .background = 0,
        .colors = ANSIColors,
    };
}
