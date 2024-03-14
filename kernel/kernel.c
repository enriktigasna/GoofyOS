// TODO:
// Load FAT32 file system
// Load tty driver from file system
// Start shell
void _start(){
    char* video_memory = (char*) 0xb8000;
    *video_memory = 'E';
}