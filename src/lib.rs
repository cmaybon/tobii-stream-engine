use std::fmt::Formatter;

pub type Error = std::os::raw::c_uint;
pub const TOBII_ERROR_NO_ERROR: Error = 0;
pub const TOBII_ERROR_INTERNAL: Error = 1;
pub const TOBII_ERROR_INSUFFICIENT_LICENSE: Error = 2;
pub const TOBII_ERROR_NOT_SUPPORTED: Error = 3;
pub const TOBII_ERROR_NOT_AVAILABLE: Error = 4;
pub const TOBII_ERROR_CONNECTION_FAILED: Error = 5;
pub const TOBII_ERROR_TIMED_OUT: Error = 6;
pub const TOBII_ERROR_ALLOCATION_FAILED: Error = 7;
pub const TOBII_ERROR_INVALID_PARAMETER: Error = 8;
pub const TOBII_ERROR_CALIBRATION_ALREADY_STARTED: Error = 9;
pub const TOBII_ERROR_CALIBRATION_NOT_STARTED: Error = 10;
pub const TOBII_ERROR_ALREADY_SUBSCRIBED: Error = 11;
pub const TOBII_ERROR_NOT_SUBSCRIBED: Error = 12;
pub const TOBII_ERROR_OPERATION_FAILED: Error = 13;
pub const TOBII_ERROR_CONFLICTING_API_INSTANCES: Error = 14;
pub const TOBII_ERROR_CALIBRATION_BUSY: Error = 15;
pub const TOBII_ERROR_CALLBACK_IN_PROGRESS: Error = 16;


#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct ApiVersion {
    pub major: std::os::raw::c_int,
    pub minor: std::os::raw::c_int,
    pub revision: std::os::raw::c_int,
    pub build: std::os::raw::c_int,
}

pub type MallocFn = std::option::Option<unsafe extern "C" fn(mem_context: *mut std::os::raw::c_void, size: usize) -> *mut std::os::raw::c_void>;
pub type FreeFn = std::option::Option<unsafe extern "C" fn(mem_context: *mut ::std::os::raw::c_void, ptr: *mut ::std::os::raw::c_void)>;

pub const TOBII_LOG_LEVEL_ERROR: LogLevel = 0;
pub const TOBII_LOG_LEVEL_WARN: LogLevel = 1;
pub const TOBII_LOG_LEVEL_INFO: LogLevel = 2;
pub const TOBII_LOG_LEVEL_DEBUG: LogLevel = 3;
pub const TOBII_LOG_LEVEL_TRACE: LogLevel = 4;
pub type LogLevel = std::os::raw::c_uint;
pub type LogFn = std::option::Option<unsafe extern "C" fn(log_context: *mut std::os::raw::c_void, level: LogLevel, text: *const std::os::raw::c_char)>;

#[repr(C)]
pub struct Api {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct CustomAlloc {
    pub mem_context: *mut std::os::raw::c_void,
    pub malloc_func: MallocFn,
    pub free_func: FreeFn,
}

#[repr(C)]
pub struct CustomLog {
    pub log_context: *mut std::os::raw::c_void,
    pub log_func: LogFn,
}

#[link(name = "lib/tobii_stream_engine")]
extern {
    pub fn tobii_api_create(api: *mut *mut Api, custom_alloc: *const CustomAlloc, custom_log: *const CustomLog) -> Error;
    pub fn tobii_error_message(error: Error) -> *const std::os::raw::c_char;
    pub fn tobii_get_api_version(version: *mut ApiVersion) -> Error;
}
