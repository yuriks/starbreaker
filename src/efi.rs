pub type EFI_STATUS = uint;
pub type EFI_HANDLE = *();

pub struct EFI_TABLE_HEADER {
	Signature: u64,
	Revision: u32,
	HeaderSize: u32,
	CRC32: u32,
	priv Reserved: u32,
}

pub struct SIMPLE_TEXT_OUTPUT_MODE {
	MaxMode: i32,
	Mode: i32,
	Attribute: i32,
	CursorColumn: i32,
	CursorRow: i32,
	CursorVisible: bool,
}

pub type EFI_TEXT_RESET = extern "win64" fn(This: *EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, ExtendedVerification: bool) -> EFI_STATUS;
pub type EFI_TEXT_STRING = extern "win64" fn(This: *EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, String: *u16) -> EFI_STATUS;
pub type EFI_TEXT_TEST_STRING = extern "win64" fn(This: *EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, String: *u16) -> EFI_STATUS;
pub type EFI_TEXT_QUERY_MODE = extern "win64" fn(This: *EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, ModeNumber: uint, Columns: *uint, Rows: *uint) -> EFI_STATUS;
pub type EFI_TEXT_SET_MODE = extern "win64" fn(This: *EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, ModeNumber: uint) -> EFI_STATUS;
pub type EFI_TEXT_SET_ATTRIBUTE = extern "win64" fn(This: *EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, Attribute: uint) -> EFI_STATUS;
pub type EFI_TEXT_CLEAR_SCREEN = extern "win64" fn(This: *EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL) -> EFI_STATUS;
pub type EFI_TEXT_SET_CURSOR_POSITION = extern "win64" fn(This: *EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, Column: uint, Row: uint) -> EFI_STATUS;
pub type EFI_TEXT_ENABLE_CURSOR = extern "win64" fn(This: *EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, Visible: bool) -> EFI_STATUS;

pub struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
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

pub struct EFI_SIMPLE_TEXT_INPUT_PROTOCOL;
pub struct EFI_RUNTIME_SERVICES;
pub struct EFI_CONFIGURATION_TABLE;

pub struct EFI_SYSTEM_TABLE {
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
