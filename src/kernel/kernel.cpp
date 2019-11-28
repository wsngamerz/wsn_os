
void printf(char* str) {
    // basic print function
    static unsigned short* VideoMemory = (unsigned short*) 0xb8000;
    
    for(int i = 0; str[i] != '\0'; ++i) {
        VideoMemory[i] = (VideoMemory[i] & 0xFF00) | str[i];
    }
}

extern "C" void kernel_main() {
    // main kernel method
    printf("Welcome to WSN OS!");
    while(1);
}
