pub type HANDLE = *();
pub type EVENT = *();
pub type TPL = uint;

pub static error_bit : uint = !(!0u >> 1);
#[repr(uint)]
pub enum STATUS {
	SUCCESS = 0u,

	// Warnings
	WARN_UNKNOWN_GLYPH = 1u,
	WARN_DELETE_FAILURE = 2u,
	WARN_WRITE_FAILURE = 3u,
	WARN_BUFFER_TOO_SMALL = 4u,
	WARN_STALE_DATA = 5u,

	// Errors
	LOAD_ERROR = error_bit | 1u,
	INVALID_PARAMETER = error_bit | 2u,
	UNSUPPORTED = error_bit | 3u,
	BAD_BUFFER_SIZE = error_bit | 4u,
	BUFFER_TOO_SMALL = error_bit | 5u,
	NOT_READY= error_bit | 6u,
	DEVICE_ERROR = error_bit | 7u,
	WRITE_PROTECTED = error_bit | 8u,
	OUT_OF_RESOURCES = error_bit | 9u,
	VOLUME_CORRUPTED = error_bit | 10u,
	VOLUME_FULL = error_bit | 11u,
	NO_MEDIA = error_bit | 12u,
	MEDIA_CHANGED = error_bit | 13u,
	NOT_FOUND = error_bit | 14u,
	ACCESS_DENIED = error_bit | 15u,
	NO_RESPONSE = error_bit | 16u,
	NO_MAPPING = error_bit | 17u,
	TIMEOUT = error_bit | 18u,
	NOT_STARTED = error_bit | 19u,
	ALREADY_STARTED = error_bit | 20u,
	ABORTED = error_bit | 21u,
	ICMP_ERROR = error_bit | 22u,
	TFTP_ERROR = error_bit | 23u,
	PROTOCOL_ERROR = error_bit | 24u,
	INCOMPATIBLE_VERSION = error_bit | 25u,
	SECURITY_VIOLATION = error_bit | 26u,
	CRC_ERROR = error_bit | 27u,
	END_OF_MEDIA = error_bit | 28u,

	END_OF_FILE = error_bit | 31u,
	INVALID_LANGUAGE = error_bit | 32u,
	COMPROMISED_DATA = error_bit | 33u,
	IP_ADDRESS_CONFLICT = error_bit | 34u,
}

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
pub type TEXT_QUERY_MODE = extern "win64" fn(This: *SIMPLE_TEXT_OUTPUT_PROTOCOL, ModeNumber: uint, Columns: *mut uint, Rows: *mut uint) -> STATUS;
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

pub enum ALLOCATE_TYPE {
	AllocateAnyPages,
	AllocateMaxAddress,
	AllocateAddress,
}

pub enum MEMORY_TYPE {
	ReservedMemoryType,
	LoaderCode,
	LoaderData,
	BootServicesCode,
	BootServicesData,
	RuntimeServicesCode,
	RuntimeServicesData,
	ConventionalMemory,
	UnusableMemory,
	ACPIReclaimMemory,
	ACPIMemoryNVS,
	MemoryMappedIO,
	MemoryMappedIOPortSpace,
	PalCode,
}

pub struct MEMORY_DESCRIPTOR {
	Type: u32,
	PhysicalStart: PHYSICAL_ADDRESS,
	VirtualStart: VIRTUAL_ADDRESS,
	NumberOfPages: u64,
	Attribute: u64,
}

pub enum TIMER_DELAY {
	TimerCancel,
	TimerPeriodic,
	TimerRelative,
}

pub type PHYSICAL_ADDRESS = u64;
pub type VIRTUAL_ADDRESS = u64;

pub type EVENT_NOTIFY = extern "win64" fn(Event: EVENT, Context: *());

pub type RAISE_TPL = extern "win64" fn(NewTpl: TPL) -> TPL;
pub type RESTORE_TPL = extern "win64" fn(OldTpl: TPL);
pub type ALLOCATE_PAGES = extern "win64" fn(Type: ALLOCATE_TYPE, MemoryType: MEMORY_TYPE, Pages: uint, Memory: *mut PHYSICAL_ADDRESS) -> STATUS;
pub type FREE_PAGES = extern "win64" fn(Memory: PHYSICAL_ADDRESS, Pages: uint) -> STATUS;
pub type GET_MEMORY_MAP = extern "win64" fn(MemoryMapSize: *mut uint, MemoryMap: *mut MEMORY_DESCRIPTOR, MapKey: *mut uint, DescriptorSize: *mut uint, DescriptorVersion: *mut u32) -> STATUS;
pub type ALLOCATE_POOL = extern "win64" fn(PoolType: MEMORY_TYPE, Size: uint, Buffer: *mut *()) -> STATUS;
pub type FREE_POOL = extern "win64" fn(Buffer: *()) -> STATUS;
pub type CREATE_EVENT = extern "win64" fn(Type: uint, NotifyTpl: TPL, NotifyFunction: EVENT_NOTIFY, NotifyContext: *(), Event: *mut EVENT) -> STATUS;
pub type SET_TIMER = extern "win64" fn(Event: EVENT, Type: TIMER_DELAY, TriggerTime: u64) -> STATUS;
pub type WAIT_FOR_EVENT = extern "win64" fn(NumberOfEvents: uint, Event: *EVENT, Index: *mut uint) -> STATUS;
pub type SIGNAL_EVENT = extern "win64" fn(Event: EVENT) -> STATUS;
pub type CLOSE_EVENT = extern "win64" fn(Event: EVENT) -> STATUS;
pub type CHECK_EVENT = extern "win64" fn(Event: EVENT) -> STATUS;
//pub type  = extern "win64" fn(

pub struct BOOT_SERVICES {
	Hdr: TABLE_HEADER,

	// EFI 1.0
	RaiseTPL: RAISE_TPL,
	RestoreTPL: RESTORE_TPL,

	AllocatePages: ALLOCATE_PAGES,
	FreePages: FREE_PAGES,
	GetMemoryMap: GET_MEMORY_MAP,
	AllocatePool: ALLOCATE_POOL,
	FreePool: FREE_POOL,

	CreateEvent: CREATE_EVENT,
	SetTimer: SET_TIMER,
	WaitForEvent: WAIT_FOR_EVENT,
	SignalEvent: SIGNAL_EVENT,
	CloseEvent: CLOSE_EVENT,
	CheckEvent: CHECK_EVENT,

	/*
	InstallProtocolInterface: INSTALL_PROTOCOL_INTERFACE,
	ReinstallProtocolInterface: REINSTALL_PROTOCOL_INTERFACE,
	UninstallProtocolInterface: UNINSTALL_PROTOCOL_INTERFACE,
	HandleProtocol: HANDLE_PROTOCOL,
	priv Reserved: *(),
	RegisterProtocolNotify: REGISTER_PROTOCOL_NOTIFY,
	LocateHandle: LOCATE_HANDLE,
	LocateDevicePath: LOCATE_DEVICE_PATH,
	InstallConfigurationTable: INSTALL_CONFIGURATION_TABLE,

	LoadImage: IMAGE_LOAD,
	StartImage: IMAGE_START,
	Exit: EXIT,
	UnloadImage: IMAGE_UNLOAD,
	ExitBootServices: EXIT_BOOT_SERVICES,

	GetNextMonotonicCount: GET_NEXT_MONOTONIC_COUNT,
	Stall: STALL,
	SetWatchdogTimer: SET_WATCHDOG_TIMER,

	// EFI 1.1
	ConnectController: CONNECT_CONTROLLER,
	DisconnectController: DISCONNECT_CONTROLLER,

	OpenProtocol: OPEN_PROTOCOL,
	CloseProtocol: CLOSE_PROTOCOL,
	OpenProtocolInformation: OPEN_PROTOCOL_INFORMATION,

	ProtocolsPerHandle: PROTOCOLS_PER_HANDLE,
	LocateHandleBuffer: LOCATE_HANDLER_BUFFER,
	LocateProtocol: LOCATE_PROTOCOL,
	InstallMultipleProtocolInterfaces: INSTALL_MULTIPLE_PROTOCOL_INTERFACES,
	UninstallMultipleProtocolInterfaces: UNINSTALL_MULTIPLE_PROTOCOL_INTERFACES,

	CalculateCrc32: CALCULATE_CRC32,

	CopyMem: COPY_MEM,
	SetMem: SET_MEM,

	// UEFI 2.0
	CreateEventEx: CREATE_EVENT_EX,
	*/
}

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
	BootServices: *BOOT_SERVICES,
	NumberOfTableEntries: uint,
	ConfigurationTable: *CONFIGURATION_TABLE,
}
