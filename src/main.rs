#[no_std];

use efi::EFI_HANDLE;
use efi::EFI_SYSTEM_TABLE;
use efi::EFI_STATUS;

mod efi;

extern "rust-intrinsic" {
	pub fn transmute<T,U>(e: T) -> U;
}

#[no_mangle]
#[no_split_stack]
pub extern "win64" fn efi_main(image_handle: EFI_HANDLE, system_table: *EFI_SYSTEM_TABLE) -> EFI_STATUS {
	unsafe {
		let stdout = (*system_table).ConOut;
		let hello_array = ['H' as u16, 'i' as u16, ' ' as u16, 'W' as u16, 'o' as u16, 'r' as u16, 'l' as u16, 'd' as u16, '!' as u16, '\r' as u16, '\n' as u16, 0u16];
		let hello_ptr: *u16 = transmute(&hello_array);
		((*stdout).OutputString)(stdout, hello_ptr);

		loop { }
	}
}
