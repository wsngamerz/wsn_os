kernel:kernel.cpp
	x86_64-elf-as boot.s -o boot.o
	x86_64-elf-g++ -m64 kernel.cpp -o kernel.o -nostdlib -ffreestanding -std=c++11 -mno-red-zone -fno-exceptions -nostdlib -fno-rtti -Wall -Wextra -Werror
	x86_64-elf-g++ -T linker.ld -o wsn_os.bin -ffreestanding -O2 -nostdlib boot.o kernel.o -lgcc

	cp wsn_os.bin ../build/wsn_os.bin