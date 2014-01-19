#[no_std];

mod efi;

extern "rust-intrinsic" {
	pub fn transmute<T,U>(e: T) -> U;
}

#[no_mangle]
#[no_split_stack]
pub extern "win64" fn efi_main(image_handle: efi::HANDLE, system_table: *efi::SYSTEM_TABLE) -> efi::STATUS {
	unsafe {
		let stdout = (*system_table).ConOut;
		let hello_array = ['H' as u16, 'i' as u16, ' ' as u16, 'W' as u16, 'o' as u16, 'r' as u16, 'l' as u16, 'd' as u16, '!' as u16, '\r' as u16, '\n' as u16, 0u16];
		let hello_ptr: *u16 = transmute(&hello_array);
		((*stdout).OutputString)(stdout, hello_ptr);

		loop { }
	}
}
