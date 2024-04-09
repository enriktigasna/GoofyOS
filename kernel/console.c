#include "stdint.h"
#include "vga_video.h"
#include "console.h"


// Basic console for debugging kernel-mode code 
void console_clear(volatile char *ptr, int size) {
    char* index = (char*) ptr;

    for(int i = 0; i<size; i++) {
        *index++ = 0;
    }
}

void console_init(struct CONSOLE* csl) {
    console_clear((volatile char*) 0xb8000, 4000);

    VGA_set_color(csl->CONSOLE_color);
    VGA_putchar('>', 0, 0);
    VGA_putchar_c('_', 2, 0, csl->CONSOLE_color);
}
