pub type STATUS = uint;
pub type HANDLE = *();

pub struct TABLE_HEADER {
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

pub type TEXT_RESET = extern "win64" fn(This: *SIMPLE_TEXT_OUTPUT_PROTOCOL, ExtendedVerification: bool) -> STATUS;
pub type TEXT_STRING = extern "win64" fn(This: *SIMPLE_TEXT_OUTPUT_PROTOCOL, String: *u16) -> STATUS;
pub type TEXT_TEST_STRING = extern "win64" fn(This: *SIMPLE_TEXT_OUTPUT_PROTOCOL, String: *u16) -> STATUS;
pub type TEXT_QUERY_MODE = extern "win64" fn(This: *SIMPLE_TEXT_OUTPUT_PROTOCOL, ModeNumber: uint, Columns: *uint, Rows: *uint) -> STATUS;
pub type TEXT_SET_MODE = extern "win64" fn(This: *SIMPLE_TEXT_OUTPUT_PROTOCOL, ModeNumber: uint) -> STATUS;
pub type TEXT_SET_ATTRIBUTE = extern "win64" fn(This: *SIMPLE_TEXT_OUTPUT_PROTOCOL, Attribute: uint) -> STATUS;
pub type TEXT_CLEAR_SCREEN = extern "win64" fn(This: *SIMPLE_TEXT_OUTPUT_PROTOCOL) -> STATUS;
pub type TEXT_SET_CURSOR_POSITION = extern "win64" fn(This: *SIMPLE_TEXT_OUTPUT_PROTOCOL, Column: uint, Row: uint) -> STATUS;
pub type TEXT_ENABLE_CURSOR = extern "win64" fn(This: *SIMPLE_TEXT_OUTPUT_PROTOCOL, Visible: bool) -> STATUS;

pub struct SIMPLE_TEXT_OUTPUT_PROTOCOL {
	Reset: TEXT_RESET,
	OutputString: TEXT_STRING,
	TestString: TEXT_TEST_STRING,
	QueryMode: TEXT_QUERY_MODE,
	SetMode: TEXT_SET_MODE,
	SetAttribute: TEXT_SET_ATTRIBUTE,
	ClearScreen: TEXT_CLEAR_SCREEN,
	SetCursorPosition: TEXT_SET_CURSOR_POSITION,
	EnableCursor: TEXT_ENABLE_CURSOR,
	Mode: *SIMPLE_TEXT_OUTPUT_MODE,
}

pub struct SIMPLE_TEXT_INPUT_PROTOCOL;
pub struct RUNTIME_SERVICES;
pub struct CONFIGURATION_TABLE;

pub struct SYSTEM_TABLE {
	Hdr: TABLE_HEADER,
	FirmwareVendor: *u16,
	FirmwareRevision: u32,
	ConsoleInHandle: HANDLE,
	ConIn: *SIMPLE_TEXT_INPUT_PROTOCOL,
	ConsoleOutHandle: HANDLE,
	ConOut: *SIMPLE_TEXT_OUTPUT_PROTOCOL,
	StandardErrorHandle: HANDLE,
	StdErr: *SIMPLE_TEXT_OUTPUT_PROTOCOL,
	RuntimeServices: *RUNTIME_SERVICES,
	NumberOfTableEntries: uint,
	ConfigurationTable: *CONFIGURATION_TABLE,
}
