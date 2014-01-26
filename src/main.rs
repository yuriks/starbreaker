#[no_std];

extern mod core;

use core::fail;
use core::ptr;

mod efi;

fn str_len<'a>(s: &'a str) -> uint {
	let (_, l) : (*u8, uint) = unsafe { core::mem::transmute(s) };
	l
}

unsafe fn byte_offset<T>(p: *T, offset: int) -> *T {
	ptr::offset(p as *u8, offset) as *T
}

fn unmut<T>(p: *mut T) -> *T {
	p as *T
}

static mut system_table: *efi::SYSTEM_TABLE = 0 as *efi::SYSTEM_TABLE;

unsafe fn allocate_buffer(buffer_size: uint) -> *mut () {
	let boot_services = (*system_table).BootServices;
	let AllocatePool = (*boot_services).AllocatePool;

	let mut buffer = 0 as *mut();

	match AllocatePool(efi::LoaderData, buffer_size, &mut buffer) {
		efi::SUCCESS => buffer,
		efi::OUT_OF_RESOURCES => fail::out_of_memory(),
		_ => fail::abort(),
	}
}

unsafe fn free_buffer(buffer: *mut ()) {
	let boot_services = (*system_table).BootServices;
	let FreePool = (*boot_services).FreePool;

	match FreePool(buffer) {
		efi::SUCCESS => (),
		_ => fail::abort(),
	}
}

fn describe_MEMORY_TYPE(v: efi::MEMORY_TYPE) -> &'static str {
	match v {
		efi::ReservedMemoryType => "ReservedMemoryType",
		efi::LoaderCode => "LoaderCode",
		efi::LoaderData => "LoaderData",
		efi::BootServicesCode => "BootServicesCode",
		efi::BootServicesData => "BootServicesData",
		efi::RuntimeServicesCode => "RuntimeServicesCode",
		efi::RuntimeServicesData => "RuntimeServicesData",
		efi::ConventionalMemory => "ConventionalMemory",
		efi::UnusableMemory => "UnusableMemory",
		efi::ACPIReclaimMemory => "ACPIReclaimMemory",
		efi::ACPIMemoryNVS => "ACPIMemoryNVS",
		efi::MemoryMappedIO => "MemoryMappedIO",
		efi::MemoryMappedIOPortSpace => "MemoryMappedIOPortSpace",
		efi::PalCode => "PalCode",
	}
}

static hex_chars: [&'static str, ..16] = [
	"0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F"];

fn hex_print(val: u64) {
	let mut val = val;

	let mut i = 0;
	while i < 16 {
		let upper_nibble = val >> (64 - 4);
		print(hex_chars[upper_nibble]);
		val <<= 4;
		i += 1;
	}
}

unsafe fn read_memory_map() {
	let boot_services = (*system_table).BootServices;
	let GetMemoryMap = (*boot_services).GetMemoryMap;

	let mut buffer_size = 0;
	let mut buffer = 0u as *mut efi::MEMORY_DESCRIPTOR;
	let mut key = 0u;
	let mut descriptor_size = 0u;
	let mut descriptor_version = 0u32;

	// TODO: This loops logic is confusing
	loop {
		match GetMemoryMap(&mut buffer_size, buffer, &mut key, &mut descriptor_size, &mut descriptor_version) {
			efi::SUCCESS => break,
			efi::BUFFER_TOO_SMALL => {
				// Buffer was too small. EFI sets buffer_size to the required size, but add some headroom in case
				// the required size changes between the two calls.
				buffer_size += 1024;
				buffer = allocate_buffer(buffer_size) as *mut efi::MEMORY_DESCRIPTOR;
				print("Allocated!\r\n");
				continue;
			}
			// Unexpected errors
			_ => {
				print("Error :(\r\n");
				return;
			}
		}
	}

	let mut offset = 0u;
	while offset < buffer_size {
		fail::assert(offset + descriptor_size <= buffer_size);

		let descriptor = *byte_offset(unmut(buffer), offset as int);
		hex_print(descriptor.PhysicalStart);
		print("-");
		hex_print(descriptor.PhysicalStart + descriptor.NumberOfPages * 4096u64);
		print("  ");
		print(describe_MEMORY_TYPE(descriptor.Type));
		print("\r\n");

		offset += descriptor_size;
	}
}

unsafe fn print_utf16(str_utf16: *u16) {
	let conout = (*system_table).ConOut;
	((*conout).OutputString)(conout, str_utf16);
}

fn print(s: &str) {
	static buf_len: uint = 4096;
	let mut buffer = [0u16, ..buf_len];

	let mut src_i = 0;
	let mut dst_i = 0;
	while dst_i < buf_len - 1 && src_i < str_len(s) {
		if s[src_i] & 0x80 == 0 { // ASCII character
			buffer[dst_i] =	s[src_i] as u16;
		} else { // Non-ASCII character
			if s[src_i] & 0x40 != 0 { // Leading byte
				buffer[dst_i] = '?' as u16;
			} else { // Continuation, don't output anything
				src_i += 1;
				continue;
			}
		}
		src_i += 1;
		dst_i += 1;
	}
	buffer[dst_i] = 0u16;

	unsafe {
		print_utf16(core::mem::transmute(&buffer));
	}
}

#[no_mangle]
#[no_split_stack]
pub extern "win64" fn efi_main(_: efi::HANDLE, system_table_: *efi::SYSTEM_TABLE) -> efi::STATUS {
	unsafe {
		system_table = system_table_;
		print("Hello ß World!\r\n");
		read_memory_map();

		loop { }
	}
}
