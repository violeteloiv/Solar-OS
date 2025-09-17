set -ex

cargo build

rm -fr isodir
mkdir -p isodir/boot/grub
cp target/target/debug/kernel isodir/boot/solar_os.bin
cp grub.cfg isodir/boot/grub/grub.cfg
grub-mkrescue -o solar_os.iso isodir