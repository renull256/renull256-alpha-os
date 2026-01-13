# renull256-alpha-os
This repository contains an experimental operating system developed for learning and prototyping purposes.

## Getting started
### Prerequisites
 - Rust
 - QEMU
 - OVMF  
You can install the required tools on Linux by running the following commands:

```sh
rustup target add x86_64-unknown-uefi
sudo apt install qemu-system-x86 ovmf
```

### Build and Run
 1. Clone this repository.
 2. Move to the root directory of this project.
 3. Run the following command:
 ```sh
 make
 ```

 ### Notes
- The paths to QEMU and OVMF may differ depending on the environment.
- If the build or execution fails, please adjust the paths in the Makefile accordingly.

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
