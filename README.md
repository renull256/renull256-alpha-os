# renull256-alpha-os
This repository contains an experimental operating system developed for learning and prototyping purposes.

## Get started
### Prerequisites
 - Rust
 - QEMU
 - OVMF  
Executing the folloing command, you can install Rust, QEMU and OVMF in Linux.  
```
  rustup target add x86_64-unknown-uefi  
  sudo apt install qemu-system-x86 ovmf
```

### Execute
 1. clone this repository.
 2. move the root directory in this project.
 3. type the ``` make ``` command in terminal.

## Current Features
- Booting from UEFI to kernel
- Framebuffer pixel drawing
- Written in Rust (no_std)

## Tech Stack
- Rust
- UEFI
- x86_64

## Future Work
- Memory management
- Process management
- Privilege separation
