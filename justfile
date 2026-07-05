build:
    cargo build

build-iso:
    cargo build
    mkdir -vp target/iso/boot/grub
    cp -v target/x86_64-unknown-none/debug/micro-os target/iso/boot/micro-os.bin
    cp -v meta/grub.cfg target/iso/boot/grub/grub.cfg
    grub-mkrescue -o target/iso/micro-os.iso target/iso/boot/

run:
    qemu-system-x86_64 -cdrom target/iso/micro-os.iso