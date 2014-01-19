LD := x86_64-w64-mingw32-ld

SRCS := $(wildcard src/*.rs)
OBJS := $(SRCS:src/%.rs=obj/%.o)

.PHONY : clean

starbreaker.iso : starbreaker.efi
	rm -rf obj/img_tmp
	mkdir -p obj/img_tmp/efi/boot/
	cp starbreaker.efi obj/img_tmp/efi/boot/bootx64.efi
	mkisofs -o $@ obj/img_tmp

starbreaker.efi: obj/main.o obj/libcore-*.rlib
	$(LD) --oformat pei-x86-64 --subsystem 10 -pie -e efi_main obj/main.o obj/libcore*.rlib -o $@

obj/%.o : obj/%.bc
	clang -c -O2 -o $@ $<

obj/main.bc : src/main.rs $(SRCS) obj/libcore-*.rlib
	rustc --emit-llvm -O --staticlib --out-dir obj/ -L obj/ -Z lto -Z no-landing-pads $<

obj/libcore-*.rlib :
	@mkdir -p obj/
	( cd externals/rust-core && rustc core/lib.rs --out-dir ../../obj/ -O -Z no-landing-pads )

clean:
	rm -f starbreaker.iso starbreaker.efi
	rm -rf obj
