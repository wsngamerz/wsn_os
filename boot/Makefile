bootloader:main.asm printf.asm printh.asm read_disk.asm testA20.asm enableA20.asm lm_check.asm gdt.asm
	nasm -fbin main.asm -o bootloader.bin
	dd if=nul of=bootloader.bin bs=1 count=0 seek=4k oflag=append
	cp bootloader.bin ../build/bootloader.bin

clean:
	rm bootloader.bin

run:
	qemu-system-x86_64 -drive file=./bootloader.bin,format=raw,index=0,media=disk