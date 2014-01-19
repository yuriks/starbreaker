LD := x86_64-w64-mingw32-ld

SRCS := $(wildcard src/*.rs)
OBJS := $(SRCS:src/%.rs=obj/%.o)

.PHONY : clean

starbreaker.iso : starbreaker.efi
	rm -rf obj/img_tmp
	mkdir -p obj/img_tmp/efi/boot/
	cp starbreaker.efi obj/img_tmp/efi/boot/bootx64.efi
	mkisofs -o $@ obj/img_tmp

starbreaker.efi: $(OBJS)
	$(LD) --oformat pei-x86-64 --subsystem 10 -pie -e efi_main $? -o $@

obj/%.o obj/%.metadata.o : src/%.rs
	@mkdir -p "$(dir $@)"
	rustc -c -O --lib --out-dir $(dir $@) $?

clean:
	rm -f starbreaker.iso starbreaker.efi
	rm -rf obj

