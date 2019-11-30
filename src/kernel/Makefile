kernel.o: kernel.c
	x86_64-elf-gcc -c kernel.c -o ../../build/kernel.o -std=gnu99 -ffreestanding -O2 -Wall -Wextra

kernel_entry.o: kernel_entry.asm
	nasm kernel_entry.asm -f elf64 -o ../../build/kernel_entry.o

kernel.bin: kernel.o kernel_entry.o
	x86_64-elf-ld -o ../../build/kernel.bin -Ttext 0x1000 ../../build/kernel_entry.o ../../build/kernel.o --oformat binary
