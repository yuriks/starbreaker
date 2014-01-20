LD := x86_64-w64-mingw32-ld

LIBCORE := libcore-2e829c2f-0.0.rlib
LIBEXTENSIONS := libextensions-d5bbce2f-0.0.so

.PHONY : clean

starbreaker.iso : starbreaker.efi
	rm -rf obj/img_tmp
	mkdir -p obj/img_tmp/efi/boot/
	cp starbreaker.efi obj/img_tmp/efi/boot/bootx64.efi
	mkisofs -o $@ obj/img_tmp

starbreaker.efi : obj/main.o obj/$(LIBCORE)
	$(LD) --oformat pei-x86-64 --subsystem 10 -pie -e efi_main $? -o $@

obj/%.o : obj/%.bc
	clang -c -O2 -o $@ $<

obj/$(LIBEXTENSIONS) : src/extensions.rs
	rustc --lib $< --out-dir $(dir $@)

obj/main.bc : src/main.rs src/efi.rs obj/$(LIBCORE) obj/$(LIBEXTENSIONS)
	rustc --emit-llvm -O --staticlib --out-dir $(dir $@) -L obj/ -Z lto -Z no-landing-pads $<

obj/$(LIBCORE) :
	@mkdir -p obj/
	( cd externals/rust-core && rustc core/lib.rs --out-dir ../../$(dir $@) -O -Z no-landing-pads )

clean:
	rm -f starbreaker.iso starbreaker.efi
	rm -rf obj
