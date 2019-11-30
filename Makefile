wsn_os:
	$(MAKE) -C ./src/boot bootloader.bin
	$(MAKE) -C ./src/kernel kernel.bin

	cat build/bootloader.bin build/kernel.bin > build/wsn_os.bin
	dd if=/dev/null of=build/wsn_os.bin bs=1 count=0 seek=1M oflag=append

run:
	qemu-system-x86_64 -drive file=./build/wsn_os.bin,format=raw,index=0,media=disk
