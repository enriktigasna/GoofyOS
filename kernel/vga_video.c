#include "vga_video.h"

void VGA_putchar(char _char, int x, int y){
    volatile char* index = (volatile char*) 0xb8000;

    index += VGA_WIDTH * 2 * y;
    index += x * 2;

    *index++ = _char;
    index++;
}

void VGA_putchar_c(char _char, int x, int y, uint8_t color){
    volatile char* index = (volatile char*) 0xb8000;

    index += VGA_WIDTH * 2 * y;
    index += x * 2;

    *index++ = _char;
    *index++ = color;
}

void VGA_write_string(size_t size, char string[], int x, int y){
    volatile char* index = (volatile char*) 0xb8000;
    index = index + x * 2;
    index = index + y * 2 * VGA_WIDTH;

    for(int i = 0; i < size; i++) {
        if(string[i] == '\n') {
            int offset = (int)(index - (volatile char*)0xb8000);
            int remaining = VGA_WIDTH * 2 - (offset % (VGA_WIDTH * 2));
            index += remaining;
            continue;
        }
        *index++ = string[i];
        index++;
    }
}

void VGA_write_string_c(size_t size, char string[], int x, int y, uint8_t color){
    volatile char* index = (volatile char*) 0xb8000;
    index = index + x * 2;
    index = index + y * 2 * VGA_WIDTH;

    for(int i = 0; i < size; i++) {
        if(string[i] == '\n') {
            int offset = (int)(index - (volatile char*)0xb8000);
            int remaining = VGA_WIDTH * 2 - (offset % (VGA_WIDTH * 2));
            index += remaining;
            continue;
        }
        *index++ = string[i];
        *index++ = color;
    }
}

void VGA_set_color (uint8_t color){
    volatile char* index = (volatile char*) 0xb8000;

    for(int i = 0; i<2000; i++) {
        *index++ = 0;
        *index++ = color;
    }
}