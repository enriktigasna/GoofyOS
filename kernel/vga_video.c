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

    for(int i = 0; i < size; i++) {
        // Return on \n
        if(string[i] == '\n'){
            y++;
            x = 0;
            continue;
        }

        // Put char
        VGA_putchar(string[i], x, y);
        x++;

        // Return on end of line
        if (x > VGA_WIDTH) {
            x = 0;
            y++;
        }
    }
}

void VGA_write_string_c(size_t size, char string[], int x, int y, uint8_t color){
    volatile char* index = (volatile char*) 0xb8000;

    for(int i = 0; i < size; i++) {
        // Return on \n
        if(string[i] == '\n'){
            y++;
            x = 0;
            continue;
        }

        // Put char
        VGA_putchar_c(string[i], x, y, color);
        x++;

        // Return on end of line
        if (x > VGA_WIDTH) {
            x = 0;
            y++;
        }
    }
}


void VGA_set_color (uint8_t color){
    volatile char* index = (volatile char*) 0xb8000;

    for(int i = 0; i<2000; i++) {
        *index++ = 0;
        *index++ = color;
    }
}