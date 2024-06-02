void print_string(const char* str, char color) {
    volatile short* video_memory = (short*)0xb8000;
    while (*str) {
        *video_memory++ = (color << 8) | *str++;
    }
}

void _start() {
    const char* message = "Hello, World!";
    char color = 0x0F; // White text on black background
    print_string(message, color);
}