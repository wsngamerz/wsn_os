kernel.o: kernel.cpp
	x86_64-elf-g++ -c kernel.cpp -o ../../build/kernel.o -O2 -Wall -Wextra -fno-exceptions -fno-rtti -ffreestanding

kernel_entry.o: kernel_entry.asm
	nasm kernel_entry.asm -f elf64 -o ../../build/kernel_entry.o

kernel.bin: kernel.o kernel_entry.o
	x86_64-elf-ld -o ../../build/kernel.bin -Ttext 0x1000 ../../build/kernel_entry.o ../../build/kernel.o --oformat binary
