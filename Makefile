ARCH            = $(shell uname -m | sed s,i[3456789]86,ia32,)

CC = gcc
CLINK = -Ignu-efi-dir/inc
CFLAGS = -fpic -ffreestanding -fno-stack-protector -fno-stack-check -fshort-wchar -mno-red-zone -maccumulate-outgoing-args

OBJS            = build/hello.o
TARGET          = build/hello.efi

EFIINC          = /usr/include/efi
EFIINCS         = -I$(EFIINC) -I$(EFIINC)/$(ARCH) -I$(EFIINC)/protocol
LIB             = /usr/local/lib
EFILIB          = /usr/local/lib
EFI_CRT_OBJS    = $(EFILIB)/crt0-efi-$(ARCH).o
EFI_LDS         = $(EFILIB)/elf_$(ARCH)_efi.lds

CFLAGS          = $(EFIINCS) -fno-stack-protector -fpic -fshort-wchar -mno-red-zone -Wall 

ifeq ($(ARCH),x86_64)
	CFLAGS += -DEFI_FUNCTION_WRAPPER
endif

LDFLAGS         = -nostdlib -znocombreloc -T $(EFI_LDS) -shared -Bsymbolic -L $(EFILIB) -L $(LIB) $(EFI_CRT_OBJS) 

all: $(TARGET)

build/hello.o: src/helloworld/main.c
	$(CC) $(CFLAGS) $(CLINK) -c src/helloworld/main.c -o build/hello.o

build/hello.so: $(OBJS)
	ld $(LDFLAGS) $(OBJS) -o $@ -lefi -lgnuefi

%.efi: %.so
	objcopy -j .text -j .sdata -j .data -j .dynamic -j .dynsym  -j .rel -j .rela -j .reloc --target=efi-app-$(ARCH) $^ $@

build/uefi.img: build/hello.efi
	dd if=/dev/zero of=build/uefi.img bs=512 count=93750

	parted build/uefi.img -s -a minimal mklabel gpt
	parted build/uefi.img -s -a minimal mkpart EFI FAT16 2048s 93716s
	parted build/uefi.img -s -a minimal toggle 1 boot

	dd if=/dev/zero of=build/part.img bs=512 count=91669

	mformat -i build/part.img -h 32 -t 32 -n 64 -c 1
	mcopy -i build/part.img build/hello.efi ::hello.efi
	
	dd if=build/part.img of=build/uefi.img bs=512 count=91669 seek=2048 conv=notrunc

run: build/uefi.img
	qemu-system-x86_64 -cpu qemu64 \
		-bios /usr/share/ovmf/OVMF.fd \
		-drive file=build/uefi.img,if=ide,format=raw \
		-net none
clean:
	rm -rf build/
	mkdir -p build/