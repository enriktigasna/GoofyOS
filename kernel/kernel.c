#include "stdint.h"
#include "vga_video.h"

void zero_memory(volatile char *ptr, int size) {
    char* index = (char*) ptr;

    for(int i = 0; i<size; i++) {
        *index++ = 0;
    }
}



void _start() {
    volatile char* video_memory = (volatile char*) 0xb8000;
    zero_memory(video_memory, 4000);
    VGA_set_color(0x70);

    char string[] = "GOOFYOS\nWelcome to GoofyOS\n";
    VGA_write_string(sizeof(string), string, 0, 0);
}