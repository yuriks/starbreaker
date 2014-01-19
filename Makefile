LD := x86_64-w64-mingw32-ld

starbreaker.iso : starbreaker.efi
	rm -rf obj/img_tmp
	mkdir -p obj/img_tmp/efi/boot/
	cp starbreaker.efi obj/img_tmp/efi/boot/bootx64.efi
	mkisofs -o $@ obj/img_tmp

starbreaker.efi: uefi.rs
	rustc -c -O --lib $?
	$(LD) --oformat pei-x86-64 --subsystem 10 -pie -e efi_main uefi.o -o $@
