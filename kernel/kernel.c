#include "stdint.h"

void zero_memory(volatile char *ptr, int size) {
    char* index = (char*) ptr;

    for(int i = 0; i<size; i++) {
        *index++ = 0;
    }
}

void write_string(uint8_t color, size_t size, char string[]){
    volatile char* index = (volatile char*) 0xb8000;
    for(int i = 0; i < size; i++) {
        *index++ = string[i];
        *index++ = color;
    }
}

void _start() {
    volatile char* video_memory = (volatile char*) 0xb8000;
    zero_memory(video_memory, 4000);

    char string[] = "Hello World";

    write_string(15, sizeof(string), string);
}