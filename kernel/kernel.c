#include "stdint.h"
#include "vga_video.h"
#include "console.h"

void zero_memory(volatile char *ptr, int size) {
    char* index = (char*) ptr;

    for(int i = 0; i<size; i++) {
        *index++ = 0;
    }
}



void _start() {
    struct CONSOLE csl;
    csl.CONSOLE_color = VGA_black_b | VGA_lgray_f | VGA_light_f | VGA_light_b;
    console_init(&csl);
}