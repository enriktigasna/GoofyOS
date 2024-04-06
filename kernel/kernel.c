#include "stdint.h"

#define VGA_HEIGHT 25
#define VGA_WIDTH 80

void zero_memory(volatile char *ptr, int size) {
    char* index = (char*) ptr;

    for(int i = 0; i<size; i++) {
        *index++ = 0;
    }
}

void set_background(uint8_t color){
    volatile char* index = (volatile char*) 0xb8000;

    for(int i = 0; i<2000; i++) {
        *index++ = 0;
        *index++ = color;
    }
}

void write_string(uint8_t color, size_t size, char string[], int x, int y){
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

void _start() {
    volatile char* video_memory = (volatile char*) 0xb8000;
    zero_memory(video_memory, 4000);
    set_background(0x70);

    char string[] = "GOOFYOS\nWelcome to GoofyOS\n";
    write_string(0x70, sizeof(string), string, 0, 0);

}