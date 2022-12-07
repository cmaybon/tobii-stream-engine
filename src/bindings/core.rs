use std::os::raw::*;
use std::fmt::{Debug, Formatter};

pub type TobiiError = c_uint;

pub const TOBII_ERROR_NO_ERROR: TobiiError = 0;
pub const TOBII_ERROR_INTERNAL: TobiiError = 1;
pub const TOBII_ERROR_INSUFFICIENT_LICENSE: TobiiError = 2;
pub const TOBII_ERROR_NOT_SUPPORTED: TobiiError = 3;
pub const TOBII_ERROR_NOT_AVAILABLE: TobiiError = 4;
pub const TOBII_ERROR_CONNECTION_FAILED: TobiiError = 5;
pub const TOBII_ERROR_TIMED_OUT: TobiiError = 6;
pub const TOBII_ERROR_ALLOCATION_FAILED: TobiiError = 7;
pub const TOBII_ERROR_INVALID_PARAMETER: TobiiError = 8;
pub const TOBII_ERROR_CALIBRATION_ALREADY_STARTED: TobiiError = 9;
pub const TOBII_ERROR_CALIBRATION_NOT_STARTED: TobiiError = 10;
pub const TOBII_ERROR_ALREADY_SUBSCRIBED: TobiiError = 11;
pub const TOBII_ERROR_NOT_SUBSCRIBED: TobiiError = 12;
pub const TOBII_ERROR_OPERATION_FAILED: TobiiError = 13;
pub const TOBII_ERROR_CONFLICTING_API_INSTANCES: TobiiError = 14;
pub const TOBII_ERROR_CALIBRATION_BUSY: TobiiError = 15;
pub const TOBII_ERROR_CALLBACK_IN_PROGRESS: TobiiError = 16;

#[repr(C)]
#[derive(Debug, Default)]
pub struct TobiiVersion {
    pub major: c_int,
    pub minor: c_int,
    pub revision: c_int,
    pub build: c_int,
}

pub type LogLevel = c_uint;

pub const TOBII_LOG_LEVEL_ERROR: LogLevel = 0;
pub const TOBII_LOG_LEVEL_WARN: LogLevel = 1;
pub const TOBII_LOG_LEVEL_INFO: LogLevel = 2;
pub const TOBII_LOG_LEVEL_DEBUG: LogLevel = 3;
pub const TOBII_LOG_LEVEL_TRACE: LogLevel = 4;

pub type TobiiLogFunc = std::option::Option<
    unsafe extern "C" fn(log_context: *mut c_void,
                         level: LogLevel,
                         text: *const c_char
    )
>;

#[repr(C)]
pub struct TobiiCustomLog {
    pub log_context: *mut c_void,
    pub log_func: TobiiLogFunc,
}

pub type TobiiMallocFunc = std::option::Option<
    unsafe extern "C" fn(mem_context: *mut c_void,
                         size: usize,
    ) -> *mut c_void
>;
pub type TobiiFreeFunc = std::option::Option<
    unsafe extern "C" fn(mem_context: *mut c_void,
                         ptr: *mut c_void,
    )
>;

#[repr(C)]
pub struct TobiiCustomAlloc {
    pub mem_context: *mut c_void,
    pub malloc_func: TobiiMallocFunc,
    pub free_func: TobiiFreeFunc,
}

#[repr(C)]
pub struct TobiiApi {
    _unused: [u8; 0],
}

pub type TobiiDeviceUrlReceiver = std::option::Option<
    unsafe extern "C" fn(url: *const c_char,
                         user_data: *mut c_void,
    )
>;

// TODO don't know what types these are meant to be
pub const TOBII_DEVICE_GENERATION_G5: c_uint = 0x00000002;
pub const TOBII_DEVICE_GENERATION_IS3: c_uint = 0x00000004;
pub const TOBII_DEVICE_GENERATION_IS4: c_uint = 0x00000008;

#[repr(C)]
pub struct TobiiDevice {
    _unused: [u8; 0],
}

pub type TobiiFieldOfUse = c_uint;

pub const TOBII_FIELD_OF_USE_INTERACTIVE: TobiiFieldOfUse = 1;
pub const TOBII_FIELD_OF_USE_ANALYTICAL: TobiiFieldOfUse = 2;

#[repr(C)]
pub struct TobiiDeviceInfo {
    pub serial_number: [c_char; 256],
    pub model: [c_char; 256],
    pub generation: [c_char; 256],
    pub firmware_version: [c_char; 256],
    pub integration_id: [c_char; 128],
    pub hw_calibration_version: [c_char; 128],
    pub hw_calibration_date: [c_char; 128],
    pub lot_id: [c_char; 128],
    pub integration_type: [c_char; 256],
    pub runtime_build_version: [c_char; 256],
}

impl Default for TobiiDeviceInfo {
    fn default() -> Self {
        TobiiDeviceInfo {
            serial_number: [c_char::default(); 256],
            model: [c_char::default(); 256],
            generation: [c_char::default(); 256],
            firmware_version: [c_char::default(); 256],
            integration_id: [c_char::default(); 128],
            hw_calibration_version: [c_char::default(); 128],
            hw_calibration_date: [c_char::default(); 128],
            lot_id: [c_char::default(); 128],
            integration_type: [c_char::default(); 256],
            runtime_build_version:[c_char::default(); 256],
        }
    }
}

#[repr(C)]
pub struct TobiiTrackBox {
    pub front_upper_right_xyz: [c_float; 3],
    pub front_upper_left_xyz: [c_float; 3],
    pub front_lower_left_xyz: [c_float; 3],
    pub front_lower_right_xyz: [c_float; 3],
    pub back_upper_right_xyz: [c_float; 3],
    pub back_upper_left_xyz: [c_float; 3],
    pub back_lower_left_xyz: [c_float; 3],
    pub back_lower_right_xyz: [c_float; 3],
}

pub type TobiiState = c_uint;

pub const TOBII_STATE_POWER_SAVE_ACTIVE: TobiiState = 0;
pub const TOBII_STATE_REMOTE_WAKE_ACTIVE: TobiiState = 1;
pub const TOBII_STATE_DEVICE_PAUSED: TobiiState = 2;
pub const TOBII_STATE_EXCLUSIVE_MODE: TobiiState = 3;
pub const TOBII_STATE_FAULT: TobiiState = 4;
pub const TOBII_STATE_WARNING: TobiiState = 5;
pub const TOBII_STATE_CALIBRATION_ID: TobiiState = 6;
pub const TOBII_STATE_CALIBRATION_ACTIVE: TobiiState = 7;

pub type TobiiStateBool = c_uint;

pub const TOBII_STATE_BOOL_FALSE: TobiiStateBool = 0;
pub const TOBII_STATE_BOOL_TRUE: TobiiStateBool = 1;

pub type TobiiStateString = [c_char; 512];

pub type TobiiSupported = c_uint;

pub const TOBII_NOT_SUPPORTED: TobiiSupported = 0;
pub const TOBII_SUPPORTED: TobiiSupported = 1;

pub type TobiiCapability = c_uint;

pub const TOBII_CAPABILITY_DISPLAY_AREA_WRITABLE: TobiiCapability = 0;
pub const TOBII_CAPABILITY_CALIBRATION_2D: TobiiCapability = 1;
pub const TOBII_CAPABILITY_CALIBRATION_3D: TobiiCapability = 2;
pub const TOBII_CAPABILITY_PERSISTENT_STORAGE: TobiiCapability = 3;
pub const TOBII_CAPABILITY_CALIBRATION_PER_EYE: TobiiCapability = 4;
pub const TOBII_CAPABILITY_COMPOUND_STREAM_WEARABLE_3D_GAZE_COMBINED: TobiiCapability = 5;
pub const TOBII_CAPABILITY_FACE_TYPE: TobiiCapability = 6;
pub const TOBII_CAPABILITY_COMPOUND_STREAM_USER_POSITION_GUIDE_XY: TobiiCapability = 7;
pub const TOBII_CAPABILITY_COMPOUND_STREAM_USER_POSITION_GUIDE_Z: TobiiCapability = 8;
pub const TOBII_CAPABILITY_COMPOUND_STREAM_WEARABLE_LIMITED_IMAGE: TobiiCapability = 9;
pub const TOBII_CAPABILITY_COMPOUND_STREAM_WEARABLE_PUPIL_DIAMETER: TobiiCapability = 10;
pub const TOBII_CAPABILITY_COMPOUND_STREAM_WEARABLE_PUPIL_POSITION: TobiiCapability = 11;
pub const TOBII_CAPABILITY_COMPOUND_STREAM_WEARABLE_EYE_OPENNESS: TobiiCapability = 12;
pub const TOBII_CAPABILITY_COMPOUND_STREAM_WEARABLE_3D_GAZE_PER_EYE: TobiiCapability = 13;
pub const TOBII_CAPABILITY_COMPOUND_STREAM_WEARABLE_USER_POSITION_GUIDE_XY: TobiiCapability = 14;
pub const TOBII_CAPABILITY_COMPOUND_STREAM_WEARABLE_TRACKING_IMPROVEMENTS: TobiiCapability = 15;
pub const TOBII_CAPABILITY_COMPOUND_STREAM_WEARABLE_CONVERGENCE_DISTANCE: TobiiCapability = 16;
pub const TOBII_CAPABILITY_COMPOUND_STREAM_WEARABLE_IMPROVE_USER_POSITION_HMD: TobiiCapability = 17;
pub const TOBII_CAPABILITY_COMPOUND_STREAM_WEARABLE_INCREASE_EYE_RELIEF: TobiiCapability = 18;

pub type TobiiStream = c_uint;

pub const TOBII_STREAM_GAZE_POINT: TobiiStream = 0;
pub const TOBII_STREAM_GAZE_ORIGIN: TobiiStream = 1;
pub const TOBII_STREAM_EYE_POSITION_NORMALIZED: TobiiStream = 2;
pub const TOBII_STREAM_USER_PRESENCE: TobiiStream = 3;
pub const TOBII_STREAM_HEAD_POSE: TobiiStream = 4;
pub const TOBII_STREAM_GAZE_DATA: TobiiStream = 5;
pub const TOBII_STREAM_DIGITAL_SYNCPORT: TobiiStream = 6;
pub const TOBII_STREAM_DIAGNOSTICS_IMAGE: TobiiStream = 7;
pub const TOBII_STREAM_USER_POSITION_GUIDE: TobiiStream = 8;
pub const TOBII_STREAM_WEARABLE_CONSUMER: TobiiStream = 9;
pub const TOBII_STREAM_WEARABLE_ADVANCED: TobiiStream = 10;
pub const TOBII_STREAM_WEARABLE_FOVEATED_GAZE: TobiiStream = 11;

pub type TobiiDataReceiver = std::option::Option<
    unsafe extern "C" fn(data: *const c_void,
                         size: usize,
                         user_data: *mut c_void
    )
>;

pub type TobiiValidity = c_uint;

pub const TOBII_VALIDITY_INVALID: TobiiValidity = 0;
pub const TOBII_VALIDITY_VALID: TobiiValidity = 1;

#[repr(C)]
pub struct TobiiDisplayArea {
    pub top_left_mm_xyz: [c_float; 3],
    pub top_right_mm_xyz: [c_float; 3],
    pub bottom_left_mm_xyz: [c_float; 3],
}

pub type TobiiEnabledEye = c_uint;

pub const TOBII_ENABLED_EYE_LEFT: TobiiEnabledEye = 0;
pub const TOBII_ENABLED_EYE_RIGHT: TobiiEnabledEye = 1;
pub const TOBII_ENABLED_EYE_BOTH: TobiiEnabledEye = 2;

#[link(name = "third_party/tobii/tobii_stream_engine")]
extern {
    pub fn tobii_error_message(error: TobiiError) -> *const c_char;
    pub fn tobii_get_api_version(version: *mut TobiiVersion) -> TobiiError;
    pub fn tobii_api_create(api: *mut *mut TobiiApi,
                            custom_alloc: *const TobiiCustomAlloc,
                            custom_log: *const TobiiCustomLog) -> TobiiError;
    pub fn tobii_api_destroy(api: *mut TobiiApi) -> TobiiError;
    pub fn tobii_system_clock(api: *mut TobiiApi,
                              timestamp_us: *mut c_longlong) -> TobiiError;
    pub fn tobii_enumerate_local_device_urls(api: *mut TobiiApi,
                                             receiver: TobiiDeviceUrlReceiver,
                                             user_data: *mut c_void) -> TobiiError;
    pub fn tobii_enumerate_local_device_urls_ex(api: *mut TobiiApi,
                                                receiver: TobiiDeviceUrlReceiver,
                                                user_data: *mut c_void,
                                                device_generations: c_uint) -> TobiiError;
    pub fn tobii_wait_for_callbacks(device_count: c_int,
                                    devices: *const *mut TobiiDevice) -> TobiiError;
    pub fn tobii_device_create(api: *mut TobiiApi,
                               url: *const c_char,
                               field_of_use: TobiiFieldOfUse,
                               device: *mut *mut TobiiDevice) -> TobiiError;
    pub fn tobii_device_destroy(device: *mut TobiiDevice) -> TobiiError;
    pub fn tobii_device_reconnect(device: *mut TobiiDevice) -> TobiiError;
    pub fn tobii_device_process_callbacks(device: *mut TobiiDevice) -> TobiiError;
    pub fn tobii_device_clear_callback_buffers(device: *mut TobiiDevice) -> TobiiError;
    pub fn tobii_update_timesync(device: *mut TobiiDevice) -> TobiiError;
    pub fn tobii_get_device_info(device: *mut TobiiDevice,
                                 device_info: *mut TobiiDeviceInfo) -> TobiiError;
    pub fn tobii_get_track_box(device: *mut TobiiDevice,
                               track_box: *mut TobiiTrackBox) -> TobiiError;
    pub fn tobii_get_state_bool(device: *mut TobiiDevice,
                                state: TobiiState, value: *mut TobiiStateBool) -> TobiiError;
    pub fn tobii_get_state_uint32(device: *mut TobiiDevice,
                                  state: TobiiState, value: *mut c_uint) -> TobiiError;
    pub fn tobii_get_state_string(device: *mut TobiiDevice,
                                  state: TobiiState, value: *mut TobiiStateString) -> TobiiError; // TODO trying *mut instead of by direct value
    pub fn tobii_capability_supported(device: *mut TobiiDevice,
                                      capability: TobiiCapability,
                                      supported: *mut TobiiSupported) -> TobiiError;
    pub fn tobii_stream_supported(device: *mut TobiiDevice,
                                  stream: TobiiStream,
                                  supported: *mut TobiiSupported) -> TobiiError;
}
