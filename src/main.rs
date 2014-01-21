#[no_std];
#[feature(phase)];

extern mod core;
#[phase(syntax)]
extern mod extensions;
mod efi;

fn str_len<'a>(s: &'a str) -> uint {
	let (_, l) : (*u8, uint) = unsafe { core::mem::transmute(s) };
	l
}

static mut system_table: *efi::SYSTEM_TABLE = 0 as *efi::SYSTEM_TABLE;

unsafe fn print_utf16(str_utf16: *u16) {
	let conout = (*system_table).ConOut;
	((*conout).OutputString)(conout, str_utf16);
}

fn print(s: &str) {
	static buf_len: uint = 4096;
	let mut buffer = [0u16, ..buf_len];

	let mut i = 0;
	while i < buf_len - 1 && i < str_len(s) {
		buffer[i] =
			if (s[i] & 0x80 == 0) { // ASCII character
				s[i] as u16
			} else { // Non-ASCII character
				if (s[i] & 0x40 != 0) { // Leading byte
					'?' as u16
				} else { // Continuation, don't output anything
					continue;
				}
			};
		i += 1;
	}
	buffer[i] = 0u16;

	unsafe {
		print_utf16(core::mem::transmute(&buffer));
	}
}

#[no_mangle]
#[no_split_stack]
pub extern "win64" fn efi_main(image_handle: efi::HANDLE, system_table_: *efi::SYSTEM_TABLE) -> efi::STATUS {
	unsafe {
		system_table = system_table_;
		print("Hello World!\r\n");

		loop { }
	}
}
