#include <stdint.h>
#include <stddef.h>



void printf(char* str) {
    static uint16_t* video_memory = (uint16_t*) 0xb8000;
    static uint8_t x = 0;
    static uint8_t y = 0;

    
    for(int i = 0; str[i] != '\0'; i++) {
        video_memory[80*y+x] = (video_memory[80*y+x] & 0xFF00) | str[i];
        x++;
    }
}

void kernel_main() {
    char* video_memory = (char*) 0xB8000;
    *video_memory = 'X';
    
    printf("Hello World");

    while(1);
}
