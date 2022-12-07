use crate::bindings::core::*;
use std::os::raw::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TobiiGazeDataEye {
    pub gaze_origin_validity: TobiiValidity,
    pub gaze_origin_from_eye_tracker_mm_xyz: [f32; 3usize],
    pub eye_position_validity: TobiiValidity,
    pub eye_position_in_track_box_normalized_xyz: [f32; 3usize],
    pub gaze_point_validity: TobiiValidity,
    pub gaze_point_from_eye_tracker_mm_xyz: [f32; 3usize],
    pub gaze_point_on_display_normalized_xy: [f32; 2usize],
    pub eyeball_center_validity: TobiiValidity,
    pub eyeball_center_from_eye_tracker_mm_xyz: [f32; 3usize],
    pub pupil_validity: TobiiValidity,
    pub pupil_diameter_mm: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TobiiGazeData {
    pub timestamp_tracker_us: i64,
    pub timestamp_system_us: i64,
    pub left: TobiiGazeDataEye,
    pub right: TobiiGazeDataEye,
}

pub type TobiiGazeDataCallback = ::std::option::Option<
    unsafe extern "C" fn(
        gaze_data: *const TobiiGazeData,
        user_data: *mut c_void,
    ),
>;

pub type TobiiDigitalSyncportUnsubscribe = ::std::option::Option<
    unsafe extern "C" fn(
        signal: u32,
        timestamp_tracker_us: i64,
        timestamp_system_us: i64,
        user_data: *mut c_void,
    ),
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TobiiTimesyncData {
    pub system_start_us: i64,
    pub system_end_us: i64,
    pub tracker_us: i64,
}

pub type TobiiIlluminationMode = [c_char; 64usize];
pub type TobiiIlluminationModeReceiver = ::std::option::Option<
    unsafe extern "C" fn(
        illumination_mode: *mut c_char,
        user_data: *mut c_void,
    ),
>;

pub type TobiiFaceType = [::std::os::raw::c_char; 64usize];
pub type TobiiFaceTypeReceiver = ::std::option::Option<
    unsafe extern "C" fn(
        face_type: *mut c_char,
        user_data: *mut c_void,
    ),
>;

extern "C" {
    pub fn tobii_gaze_data_subscribe(
        device: *mut TobiiDevice,
        callback: TobiiGazeDataCallback,
        user_data: *mut c_void,
    ) -> TobiiError;
    pub fn tobii_gaze_data_unsubscribe(device: *mut TobiiDevice) -> TobiiError;
    pub fn tobii_digital_syncport_subscribe(
        device: *mut TobiiDevice,
        callback: TobiiDigitalSyncportUnsubscribe,
        user_data: *mut c_void,
    ) -> TobiiError;
    pub fn tobii_digital_syncport_unsubscribe(device: *mut TobiiDevice) -> TobiiError;
    pub fn tobii_timesync(
        device: *mut TobiiDevice,
        timesync: *mut TobiiTimesyncData,
    ) -> TobiiError;
    pub fn tobii_enumerate_illumination_modes(
        device: *mut TobiiDevice,
        receiver: TobiiIlluminationModeReceiver,
        user_data: *mut c_void,
    ) -> TobiiError;
    pub fn tobii_set_illumination_mode(
        device: *mut TobiiDevice,
        illumination_mode: *mut c_char,
    ) -> TobiiError;
    pub fn tobii_get_illumination_mode(
        device: *mut TobiiDevice,
        illumination_mode: *mut TobiiIlluminationMode,
    ) -> TobiiError;
    pub fn tobii_enumerate_face_types(
        device: *mut TobiiDevice,
        receiver: TobiiFaceTypeReceiver,
        user_data: *mut ::std::os::raw::c_void,
    ) -> TobiiError;
    pub fn tobii_set_face_type(
        device: *mut TobiiDevice,
        face_type: *mut ::std::os::raw::c_char,
    ) -> TobiiError;
    pub fn tobii_get_face_type(
        device: *mut TobiiDevice,
        face_type: *mut TobiiFaceType,
    ) -> TobiiError;
}
