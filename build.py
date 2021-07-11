#!/usr/bin/env python3

import argparse
import os
import shutil
import sys
import subprocess as sp
from pathlib import Path

ARCH = "x86_64"
BOOTLOADER_TARGET = ARCH + "-unknown-uefi"
KERNEL_TARGET = ARCH + "-wsnos"
CONFIG = "debug"
QEMU = "qemu-system-" + ARCH

WORKSPACE_DIR = Path(__file__).resolve().parents[0]
BUILD_DIR = WORKSPACE_DIR / "build"
BOOTLOADER_DIR = WORKSPACE_DIR / "bootloader"
KERNEL_DIR = WORKSPACE_DIR / "kernel"

BOOTLOADER_CARGO_BUILD_DIR = WORKSPACE_DIR / \
    "bootloader" / "target" / BOOTLOADER_TARGET / CONFIG
KERNEL_CARGO_BUILD_DIR = WORKSPACE_DIR / \
    "kernel" / "target" / KERNEL_TARGET / CONFIG

OVMF_CODE = WORKSPACE_DIR / "bootloader" / "ovmf" / "OVMF_CODE-pure-efi.fd"
OVMF_VARS = WORKSPACE_DIR / "bootloader" / "ovmf" / "OVMF_VARS-pure-efi.fd"


def main(args):
    print("==================== Directories ====================")
    print(f"             Workspace Directory: {WORKSPACE_DIR}")
    print(f"                 Build Directory: {BUILD_DIR}")
    print("")
    print(f"            Bootloader Directory: {BOOTLOADER_DIR}")
    print(f"Bootloader Cargo Build Directory: {BOOTLOADER_CARGO_BUILD_DIR}")
    print(f"                Kernel Directory: {KERNEL_DIR}")
    print(f"    Kernel Cargo Build Directory: {KERNEL_CARGO_BUILD_DIR}")
    print("=====================================================")

    # clear any args before build and set only what we need
    os.environ["RUSTFLAGS"] = ""
    os.environ["RUST_TARGET_PATH"] = str(WORKSPACE_DIR)

    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="verb")
    subparsers.add_parser("build")
    subparsers.add_parser("build-bootloader")
    subparsers.add_parser("build-kernel")
    subparsers.add_parser("run")

    options = parser.parse_args()
    if options.verb == "build":
        build_command()

    elif options.verb == "build-bootloader":
        build_bootloader_command()

    elif options.verb == "build-kernel":
        build_kernel_command()

    elif options.verb == "run":
        run_command()

    else:
        print(f"Unknown command {options.verb}")


def build_command():
    """Build wsnOS"""
    build_bootloader_command()
    build_kernel_command()

    # Create build folders
    boot_dir = BUILD_DIR / "EFI" / "BOOT"
    boot_dir.mkdir(parents=True, exist_ok=True)

    kernel_dir = BUILD_DIR / "EFI" / "WSNOS"
    kernel_dir.mkdir(parents=True, exist_ok=True)

    # Copy the bootloader EFI application to the build directory
    bootloader_build = BOOTLOADER_CARGO_BUILD_DIR / "bootloader.efi"
    bootloader_output = boot_dir / "BootX64.efi"
    shutil.copy2(bootloader_build, bootloader_output)

    # Write a startup script to make UEFI Shell load into
    # the application automatically
    startup_file = open(BUILD_DIR / "startup.nsh", "w")
    startup_file.write("\EFI\BOOT\BOOTX64.EFI")
    startup_file.close()

    # Copy the kernel accross into the correct folders
    kernel_build = KERNEL_CARGO_BUILD_DIR / "kernel"
    kernel_output = kernel_dir / "kernel.bin"
    shutil.copy2(kernel_build, kernel_output)


def build_bootloader_command():
    """Build the bootloader"""
    print("Build wsnOS Bootloader")

    cmd = ["cargo", "+nightly", "build",
           "-Z", "build-std=core,compiler_builtins,alloc",
           "-Z", "build-std-features=compiler-builtins-mem",
           "--target", BOOTLOADER_TARGET,
           "--package", "bootloader"]
    sp.run(cmd, cwd=BOOTLOADER_DIR).check_returncode()
    return


def build_kernel_command():
    """Build wsnOS Kernel"""
    print("Build wsnOS Kernel")

    cmd = ["cargo", "+nightly", "build",
           "-Z", "build-std=core,compiler_builtins",
           "-Z", "build-std-features=compiler-builtins-mem",
           "--target", f"{KERNEL_TARGET}.json",
           "--package", "kernel"]
    sp.run(cmd, cwd=KERNEL_DIR).check_returncode()
    return


def run_command():
    print("Run")

    if shutil.which(QEMU) is None:
        print("QEMU is either not installed or not on the path")
        return

    if not os.path.isfile(OVMF_CODE):
        print(OVMF_CODE)
        return

    if not os.path.isfile(OVMF_VARS):
        print(OVMF_VARS)
        return

    qemu_flags = [
        # Disable default devices
        # QEMU by default enables a ton of devices which slow down boot.
        "-nodefaults",

        # Use a standard VGA for graphics
        "-vga", "std",

        # Use a modern machine, with acceleration if possible.
        "-machine", "q35,accel=kvm:tcg",

        # Allocate some memory
        "-m", "256M",

        # Set up OVMF
        "-drive", f"if=pflash,format=raw,readonly=on,file={OVMF_CODE}",
        "-drive", f"if=pflash,format=raw,file={OVMF_VARS}",

        # Mount a local directory as a FAT partition
        "-drive", f"format=raw,file=fat:rw:{BUILD_DIR}",

        # Enable serial
        #
        # Connect the serial port to the host. OVMF is kind enough to connect
        # the UEFI stdout and stdin to that port too.
        "-serial", "stdio",

        # Setup monitor
        "-monitor", "vc:1280x720",
    ]

    sp.run([QEMU] + qemu_flags).check_returncode()
    return


if __name__ == "__main__":
    main(sys.argv)
