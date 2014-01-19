#[no_std];

extern "rust-intrinsic" {
	pub fn transmute<T,U>(e: T) -> U;
}

type EFI_STATUS = uint;
type EFI_HANDLE = *();

struct EFI_TABLE_HEADER {
	Signature: u64,
	Revision: u32,
	HeaderSize: u32,
	CRC32: u32,
	priv Reserved: u32,
}

struct SIMPLE_TEXT_OUTPUT_MODE {
	MaxMode: i32,
	Mode: i32,
	Attribute: i32,
	CursorColumn: i32,
	CursorRow: i32,
	CursorVisible: bool,
}

type EFI_TEXT_RESET = extern "win64" fn(This: *EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, ExtendedVerification: bool) -> EFI_STATUS;
type EFI_TEXT_STRING = extern "win64" fn(This: *EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, String: *u16) -> EFI_STATUS;
type EFI_TEXT_TEST_STRING = extern "win64" fn(This: *EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, String: *u16) -> EFI_STATUS;
type EFI_TEXT_QUERY_MODE = extern "win64" fn(This: *EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, ModeNumber: uint, Columns: *uint, Rows: *uint) -> EFI_STATUS;
type EFI_TEXT_SET_MODE = extern "win64" fn(This: *EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, ModeNumber: uint) -> EFI_STATUS;
type EFI_TEXT_SET_ATTRIBUTE = extern "win64" fn(This: *EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, Attribute: uint) -> EFI_STATUS;
type EFI_TEXT_CLEAR_SCREEN = extern "win64" fn(This: *EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL) -> EFI_STATUS;
type EFI_TEXT_SET_CURSOR_POSITION = extern "win64" fn(This: *EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, Column: uint, Row: uint) -> EFI_STATUS;
type EFI_TEXT_ENABLE_CURSOR = extern "win64" fn(This: *EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, Visible: bool) -> EFI_STATUS;

struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
	Reset: EFI_TEXT_RESET,
	OutputString: EFI_TEXT_STRING,
	TestString: EFI_TEXT_TEST_STRING,
	QueryMode: EFI_TEXT_QUERY_MODE,
	SetMode: EFI_TEXT_SET_MODE,
	SetAttribute: EFI_TEXT_SET_ATTRIBUTE,
	ClearScreen: EFI_TEXT_CLEAR_SCREEN,
	SetCursorPosition: EFI_TEXT_SET_CURSOR_POSITION,
	EnableCursor: EFI_TEXT_ENABLE_CURSOR,
	Mode: *SIMPLE_TEXT_OUTPUT_MODE,
}

struct EFI_SIMPLE_TEXT_INPUT_PROTOCOL;
struct EFI_RUNTIME_SERVICES;
struct EFI_CONFIGURATION_TABLE;

struct EFI_SYSTEM_TABLE {
	Hdr: EFI_TABLE_HEADER,
	FirmwareVendor: *u16,
	FirmwareRevision: u32,
	ConsoleInHandle: EFI_HANDLE,
	ConIn: *EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
	ConsoleOutHandle: EFI_HANDLE,
	ConOut: *EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
	StandardErrorHandle: EFI_HANDLE,
	StdErr: *EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
	RuntimeServices: *EFI_RUNTIME_SERVICES,
	NumberOfTableEntries: uint,
	ConfigurationTable: *EFI_CONFIGURATION_TABLE,
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
