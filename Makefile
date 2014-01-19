LD := x86_64-w64-mingw32-ld

SRCS := $(wildcard src/*.rs)
OBJS := $(SRCS:src/%.rs=obj/%.o)

.PHONY : clean

starbreaker.iso : starbreaker.efi
	rm -rf obj/img_tmp
	mkdir -p obj/img_tmp/efi/boot/
	cp starbreaker.efi obj/img_tmp/efi/boot/bootx64.efi
	mkisofs -o $@ obj/img_tmp

starbreaker.efi: src/main.rs $(SRCS)
	@mkdir -p obj/
	rustc -c -O --lib --out-dir obj/ $<
	$(LD) --oformat pei-x86-64 --subsystem 10 -pie -e efi_main $(<:src/%.rs=obj/%.o) -o $@

clean:
	rm -f starbreaker.iso starbreaker.efi
	rm -rf obj
