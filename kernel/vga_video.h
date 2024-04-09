#include "stdint.h"

#define VGA_HEIGHT 25
#define VGA_WIDTH 80

void VGA_putchar(char _char, int x, int y);
void VGA_putchar_c(char _char, int x, int y, uint8_t color);
void VGA_write_string(size_t size, char string[], int x, int y);
void VGA_write_string_c(size_t size, char string[], int x, int y, uint8_t color);
void VGA_set_color (uint8_t color);

enum VGA_colors {
    VGA_black_f = 0x00,
    VGA_blue_f = 0x01,
    VGA_green_f = 0x02,
    VGA_cyan_f = 0x03,
    VGA_red_f = 0x04,
    VGA_magenta_f = 0x05,
    VGA_brown_f = 0x06,
    VGA_lgray_f = 0x07,
    VGA_black_b = 0x00,
    VGA_blue_b = 0x10,
    VGA_green_b = 0x20,
    VGA_cyan_b = 0x30,
    VGA_red_b = 0x40,
    VGA_magenta_b = 0x50,
    VGA_brown_b = 0x60,
    VGA_lgray_b = 0x70,
    VGA_light_f = 0x08,
    VGA_light_b = 0x80,
};