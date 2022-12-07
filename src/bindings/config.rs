use crate::bindings::core::*;
use std::os::raw::*;

pub type TobiiCalibrationPointStatus = c_int;

pub const TOBII_CALIBRATION_POINT_STATUS_FAILED_OR_INVALID: TobiiCalibrationPointStatus = 0;
pub const TOBII_CALIBRATION_POINT_STATUS_VALID_BUT_NOT_USED_IN_CALIBRATION: TobiiCalibrationPointStatus = 1;
pub const TOBII_CALIBRATION_POINT_STATUS_VALID_AND_USED_IN_CALIBRATION: TobiiCalibrationPointStatus = 2;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TobiiCalibrationPointData {
    pub point_xy: [f32; 2usize],
    pub left_status: TobiiCalibrationPointStatus,
    pub left_mapping_xy: [f32; 2usize],
    pub right_status: TobiiCalibrationPointStatus,
    pub right_mapping_xy: [f32; 2usize],
}

pub type TobiiCalibrationPointDataReceiver = ::std::option::Option<
    unsafe extern "C" fn(
        point_data: *const TobiiCalibrationPointData,
        user_data: *mut c_void,
    ),
>;

pub type TobiiDataReceiver = ::std::option::Option<
    unsafe extern "C" fn(
        data: *const c_void,
        size: usize,
        user_data: *mut c_void,
    ),
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TobiiGeometryMounting {
    pub guides: c_int,
    pub width_mm: f32,
    pub angle_deg: f32,
    pub external_offset_mm_xyz: [f32; 3usize],
    pub internal_offset_mm_xyz: [f32; 3usize],
}

pub type TobiiDeviceName = [c_char; 64usize];
pub type TobiiOutputFrequencyReceiver = ::std::option::Option<
    unsafe extern "C" fn(
        output_frequency:
        f32, user_data:
        *mut c_void,
    ),
>;

#[link(name = "third_party/tobii/tobii_stream_engine")]
extern "C" {
    pub fn tobii_set_enabled_eye(
        device: *mut TobiiDevice,
        enabled_eye: TobiiEnabledEye,
    ) -> TobiiError;
    pub fn tobii_get_enabled_eye(
        device: *mut TobiiDevice,
        enabled_eye: *mut TobiiEnabledEye,
    ) -> TobiiError;
    pub fn tobii_calibration_start(
        device: *mut TobiiDevice,
        enabled_eye: TobiiEnabledEye,
    ) -> TobiiError;
    pub fn tobii_calibration_stop(device: *mut TobiiDevice) -> TobiiError;
    pub fn tobii_calibration_collect_data_2d(
        device: *mut TobiiDevice,
        x: f32,
        y: f32,
    ) -> TobiiError;
    pub fn tobii_calibration_collect_data_3d(
        device: *mut TobiiDevice,
        x: f32,
        y: f32,
        z: f32,
    ) -> TobiiError;
    pub fn tobii_calibration_collect_data_per_eye_2d(
        device: *mut TobiiDevice,
        x: f32,
        y: f32,
        requested_eyes: TobiiEnabledEye,
        collected_eyes: *mut TobiiEnabledEye,
    ) -> TobiiError;
    pub fn tobii_calibration_discard_data_2d(
        device: *mut TobiiDevice,
        x: f32,
        y: f32,
    ) -> TobiiError;
    pub fn tobii_calibration_discard_data_3d(
        device: *mut TobiiDevice,
        x: f32,
        y: f32,
        z: f32,
    ) -> TobiiError;
    pub fn tobii_calibration_discard_data_per_eye_2d(
        device: *mut TobiiDevice,
        x: f32,
        y: f32,
        eyes: TobiiEnabledEye,
    ) -> TobiiError;
    pub fn tobii_calibration_clear(device: *mut TobiiDevice) -> TobiiError;
    pub fn tobii_calibration_compute_and_apply(device: *mut TobiiDevice) -> TobiiError;
    pub fn tobii_calibration_compute_and_apply_per_eye(
        device: *mut TobiiDevice,
        calibrated_eyes: *mut TobiiEnabledEye,
    ) -> TobiiError;
    pub fn tobii_calibration_retrieve(
        device: *mut TobiiDevice,
        receiver: TobiiDataReceiver,
        user_data: *mut c_void,
    ) -> TobiiError;
    pub fn tobii_calibration_parse(
        api: *mut TobiiApi,
        data: *const c_void,
        data_size: usize,
        receiver: TobiiCalibrationPointDataReceiver,
        user_data: *mut c_void,
    ) -> TobiiError;
    pub fn tobii_calibration_apply(
        device: *mut TobiiDevice,
        data: *const c_void,
        size: usize,
    ) -> TobiiError;
    pub fn tobii_get_geometry_mounting(
        device: *mut TobiiDevice,
        geometry_mounting: *mut TobiiGeometryMounting,
    ) -> TobiiError;
    pub fn tobii_get_display_area(
        device: *mut TobiiDevice,
        display_area: *mut TobiiDisplayArea,
    ) -> TobiiError;
    pub fn tobii_set_display_area(
        device: *mut TobiiDevice,
        display_area: *const TobiiDisplayArea,
    ) -> TobiiError;
    pub fn tobii_calculate_display_area_basic(
        api: *mut TobiiApi,
        width_mm: f32,
        height_mm: f32,
        offset_x_mm: f32,
        geometry_mounting: *const TobiiGeometryMounting,
        display_area: *mut TobiiDisplayArea,
    ) -> TobiiError;
    pub fn tobii_get_device_name(
        device: *mut TobiiDevice,
        device_name: *mut TobiiDeviceName,
    ) -> TobiiError;
    pub fn tobii_set_device_name(
        device: *mut TobiiDevice,
        device_name: *mut c_char,
    ) -> TobiiError;
    pub fn tobii_enumerate_output_frequencies(
        device: *mut TobiiDevice,
        receiver: TobiiOutputFrequencyReceiver,
        user_data: *mut c_void,
    ) -> TobiiError;
    pub fn tobii_set_output_frequency(
        device: *mut TobiiDevice,
        output_frequency: f32,
    ) -> TobiiError;
    pub fn tobii_get_output_frequency(
        device: *mut TobiiDevice,
        output_frequency: *mut f32,
    ) -> TobiiError;
}
