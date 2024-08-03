const std = @import("std");
const term = @import("terminal.zig");

const GDT_DESCRIPTORS: u16 = 5;

const GDTEntry = packed struct {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access: u8,
    limit: u4,
    flags: u4,
    base_high: u8,
};

const GDTPointer = packed struct {
    limit: u16,
    base: u64,
};

const KERNEL_CODE: u16 = 0x8;
const KERNEL_DATA: u16 = 0x10;
var gdt_entries: [GDT_DESCRIPTORS]GDTEntry = [GDT_DESCRIPTORS]GDTEntry{
    // Null Descriptor
    GDTEntry{
        .limit_low = 0,
        .base_low = 0,
        .base_mid = 0,
        .access = 0,
        .limit = 0,
        .flags = 0,
        .base_high = 0,
    },
    // Kernel Code Segment
    GDTEntry{
        .limit_low = 0xFFFF,
        .base_low = 0,
        .base_mid = 0,
        .access = 0x9A,
        .limit = 0xF,
        .flags = 0xA,
        .base_high = 0,
    },
    // Kernel Data Segment
    GDTEntry{
        .limit_low = 0xFFFF,
        .base_low = 0,
        .base_mid = 0,
        .access = 0x92,
        .limit = 0xF,
        .flags = 0xC,
        .base_high = 0,
    },
    // Usermode Code Segment
    GDTEntry{
        .limit_low = 0xFFFF,
        .base_low = 0,
        .base_mid = 0,
        .access = 0xFA,
        .limit = 0xF,
        .flags = 0xA,
        .base_high = 0,
    },
    // Usermode Data Segment
    GDTEntry{
        .limit_low = 0xFFFF,
        .base_low = 0,
        .base_mid = 0,
        .access = 0xFA,
        .limit = 0xF,
        .flags = 0xC,
        .base_high = 0,
    },
};

const TSS = packed struct {
    reserved1: u32,
    rsp: [3]u64,
    reserved2: u64,
    ist: [7]u64,
    reserved3: u64,
    reserved4: u16,
    iopb: u16,
};

fn lgdt(gdt_ptr: *const GDTPointer) void {
    asm volatile (
        \\  lgdt %[ptr]
        :
        : [ptr] "*p" (&gdt_ptr),
    );
    asm volatile (
        \\  mov %%bx, %%ds
        \\  mov %%bx, %%es
        \\  mov %%bx, %%fs
        \\  mov %%bx, %%gs
        \\  mov %%bx, %%ss
        :
        : [KERNEL_DATA] "{bx}" (KERNEL_DATA),
    );
}

pub fn init() void {
    const gdt_ptr: *const GDTPointer = &GDTPointer{
        .limit = (GDT_DESCRIPTORS * 8) - 1,
        .base = (@intFromPtr(&gdt_entries)),
    };

    term.write_fmt("GDTPointer: 0x{x}\n", .{@intFromPtr(gdt_ptr)});
    term.write_fmt("GDTPointer.limit: 0x{x}\n", .{gdt_ptr.limit});
    term.write_fmt("GDTPointer.base: 0x{x}\n\n", .{gdt_ptr.base});

    term.write_fmt("Pointer to gdt_entries[0] 0x{x}\n", .{@intFromPtr(&gdt_entries[0])});
    term.write_fmt("Pointer to gdt_entries 0x{x}\n\n", .{@intFromPtr(&gdt_entries)});
    lgdt(gdt_ptr);
}
