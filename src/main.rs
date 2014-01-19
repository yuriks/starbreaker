#[no_std];
#[feature(phase)];

extern mod core;
#[phase(syntax)]
extern mod extensions;
mod efi;

static hello_array : [u16, ..15] = ucs2_from_str!("Hello World!\r\n");

#[no_mangle]
#[no_split_stack]
pub extern "win64" fn efi_main(image_handle: efi::HANDLE, system_table: *efi::SYSTEM_TABLE) -> efi::STATUS {
	unsafe {
		let stdout = (*system_table).ConOut;
		let hello_ptr: *u16 = core::mem::transmute(&hello_array);
		((*stdout).OutputString)(stdout, hello_ptr);

		loop { }
	}
}
