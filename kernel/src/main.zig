const limine = @import("limine");
const std = @import("std");
const font = @import("font.zig");
const framebuffer = @import("framebuffer.zig");
const terminal = @import("terminal.zig");
const gdt = @import("gdt.zig");

pub export var base_revision: limine.BaseRevision = .{ .revision = 2 };
inline fn done() noreturn {
    while (true) {
        asm volatile ("hlt");
    }
}

export fn _start() callconv(.C) noreturn {
    if (!base_revision.is_supported()) {
        done();
    }

    framebuffer.init() catch done();
    terminal.init();
    gdt.init();
    terminal.write_fmt(
        \\ d888b   .d88b.   .d88b.  d88888b db    db       .d88b.  .d8888.
        \\88' Y8b .8P  Y8. .8P  Y8. 88'     `8b  d8'      .8P  Y8. 88'  YP
        \\88      88    88 88    88 88ooo    `8bd8'       88    88 `8bo.
        \\88  ooo 88    88 88    88 88~~~      88         88    88   `Y8b.
        \\88. ~8~ `8b  d8' `8b  d8' 88         88         `8b  d8' db   8D
        \\ Y888P   `Y88P'   `Y88P'  YP         YP          `Y88P'  `8888Y'
        \\
        \\
    , .{});

    terminal.write_fmt("Welcome to GoofyOS v0.0.1\n", .{});
    terminal.write_fmt("Detected resolution {d}x{d}\n", .{ framebuffer.fb.width, framebuffer.fb.height });
    terminal.write_fmt("Text mode {d}x{d}\n", .{ terminal.term.cols, terminal.term.rows });
    done();
}
