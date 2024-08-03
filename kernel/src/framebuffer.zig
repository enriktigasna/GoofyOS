const limine = @import("limine");

pub export var framebuffer_request: limine.FramebufferRequest = .{};

pub const Framebuffer = struct {
    address: [*]u8,
    width: u64,
    height: u64,
    pitch: u64,
    bpp: u16,
};

pub var fb: Framebuffer = undefined;

pub fn init() !void {
    if (framebuffer_request.response) |framebuffer_response| {
        if (framebuffer_response.framebuffer_count < 1) {
            return error.FramebufferNotFound;
        }
        const limine_framebuffer = framebuffer_response.framebuffers()[0];
        fb = Framebuffer{
            .address = limine_framebuffer.address,
            .width = limine_framebuffer.width,
            .height = limine_framebuffer.height,
            .pitch = limine_framebuffer.pitch,
            .bpp = limine_framebuffer.bpp,
        };
        return;
    }

    return error.FramebufferResponseNotFound;
}

pub fn setPixel(x: usize, y: usize, c: u24) void {
    const pixel_offset = (y * fb.pitch) + x * 4;
    @as(*u32, @ptrCast(@alignCast(fb.address + pixel_offset))).* = c;
    return;
}

pub fn drawBitmap(x: usize, y: usize, fg: u24, bg: u24, h: usize, w: usize, buf: []u8) void {
    for (0..h) |row| {
        for (0..w) |col| {
            const bit_index = row * w + col;
            const byte_index = bit_index / 8;
            const bit_offset = 7 - (bit_index % 8);
            switch ((buf[byte_index] >> @intCast(bit_offset)) & 1) {
                1 => setPixel(x + col, y + row, fg),
                else => setPixel(x + col, y + row, bg),
            }
        }
    }
}
