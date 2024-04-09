#include "stdint.h"

#define VGA_HEIGHT 25
#define VGA_WIDTH 80

void VGA_putchar(char _char, int x, int y);
void VGA_putchar_c(char _char, int x, int y, uint8_t color);
void VGA_write_string(size_t size, char string[], int x, int y);
void VGA_write_string_c(size_t size, char string[], int x, int y, uint8_t color);
void VGA_set_color (uint8_t color);