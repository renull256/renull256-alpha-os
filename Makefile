TARGET = x86_64-unknown-uefi
EFI_FILE = target/x86_64-unknown-uefi/debug/BOOTX64.efi
OVMF = /usr/share/ovmf/OVMF.fd

run:
	cargo build --target $(TARGET)
	mkdir -p esp/EFI/BOOT
	cp $(EFI_FILE) esp/EFI/BOOT/BOOTX64.EFI
	qemu-system-x86_64 -bios $(OVMF) -drive format=raw,file=fat:rw:esp -net none