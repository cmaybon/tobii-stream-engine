use crate::core::*;
use std::os::raw::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TobiiWearableConsumerEye {
    pub pupil_position_in_sensor_area_validity: TobiiValidity,
    pub pupil_position_in_sensor_area_xy: [f32; 2usize],
    pub position_guide_validity: TobiiValidity,
    pub position_guide_xy: [f32; 2usize],
    pub blink_validity: TobiiValidity,
    pub blink: TobiiStateBool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TobiiWearableConsumerData {
    pub timestamp_us: i64,
    pub left: TobiiWearableConsumerEye,
    pub right: TobiiWearableConsumerEye,
    pub gaze_origin_combined_validity: TobiiValidity,
    pub gaze_origin_combined_mm_xyz: [f32; 3usize],
    pub gaze_direction_combined_validity: TobiiValidity,
    pub gaze_direction_combined_normalized_xyz: [f32; 3usize],
    pub convergence_distance_validity: TobiiValidity,
    pub convergence_distance_mm: f32,
    pub improve_user_position_hmd: TobiiStateBool,
    pub increase_eye_relief: TobiiStateBool,
}

pub type TobiiWearableConsumerDataCallback = ::std::option::Option<
    unsafe extern "C" fn(
        data: *const TobiiWearableConsumerData,
        user_data: *mut c_void,
    ),
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TobiiWearableAdvancedEye {
    pub gaze_origin_validity: TobiiValidity,
    pub gaze_origin_mm_xyz: [f32; 3usize],
    pub gaze_direction_validity: TobiiValidity,
    pub gaze_direction_normalized_xyz: [f32; 3usize],
    pub pupil_diameter_validity: TobiiValidity,
    pub pupil_diameter_mm: f32,
    pub pupil_position_in_sensor_area_validity: TobiiValidity,
    pub pupil_position_in_sensor_area_xy: [f32; 2usize],
    pub position_guide_validity: TobiiValidity,
    pub position_guide_xy: [f32; 2usize],
    pub blink_validity: TobiiValidity,
    pub blink: TobiiStateBool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TobiiWearableAdvancedData {
    pub timestamp_tracker_us: i64,
    pub timestamp_system_us: i64,
    pub left: TobiiWearableAdvancedEye,
    pub right: TobiiWearableAdvancedEye,
    pub gaze_origin_combined_validity: TobiiValidity,
    pub gaze_origin_combined_mm_xyz: [f32; 3usize],
    pub gaze_direction_combined_validity: TobiiValidity,
    pub gaze_direction_combined_normalized_xyz: [f32; 3usize],
    pub convergence_distance_validity: TobiiValidity,
    pub convergence_distance_mm: f32,
    pub improve_user_position_hmd: TobiiStateBool,
    pub increase_eye_relief: TobiiStateBool,
}

pub type TobiiWearableAdvancedDataCallback = ::std::option::Option<
    unsafe extern "C" fn(
        data: *const TobiiWearableAdvancedData,
        user_data: *mut c_void,
    ),
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TobiiLensConfiguration {
    pub left_xyz: [f32; 3usize],
    pub right_xyz: [f32; 3usize],
}

pub type TobiiLensConfigurationWritable = c_int;

pub const TOBII_LENS_CONFIGURATION_NOT_WRITABLE: TobiiLensConfigurationWritable = 0;
pub const TOBII_LENS_CONFIGURATION_WRITABLE: TobiiLensConfigurationWritable = 1;

pub type TobiiWearableFoveatedTrackingState = c_int;

pub const TOBII_WEARABLE_FOVEATED_TRACKING_STATE_TRACKING: TobiiWearableFoveatedTrackingState = 0;
pub const TOBII_WEARABLE_FOVEATED_TRACKING_STATE_EXTRAPOLATED: TobiiWearableFoveatedTrackingState = 1;
pub const TOBII_WEARABLE_FOVEATED_TRACKING_STATE_LAST_KNOWN: TobiiWearableFoveatedTrackingState = 2;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TobiiWearableFoveatedGaze {
    pub timestamp_us: i64,
    pub tracking_state: TobiiWearableFoveatedTrackingState,
    pub gaze_direction_combined_normalized_xyz: [f32; 3usize],
}

pub type TobiiWearableFoveatedGazeCallback = ::std::option::Option<
    unsafe extern "C" fn(
        data: *const TobiiWearableFoveatedGaze,
        user_data: *mut c_void,
    ),
>;

#[link(name = "third_party/tobii/tobii_stream_engine")]
extern "C" {
    pub fn tobii_wearable_consumer_data_subscribe(
        device: *mut TobiiDevice,
        callback: TobiiWearableConsumerDataCallback,
        user_data: *mut c_void,
    ) -> TobiiError;
    pub fn tobii_wearable_consumer_data_unsubscribe(device: *mut TobiiDevice) -> TobiiError;
    pub fn tobii_wearable_advanced_data_subscribe(
        device: *mut TobiiDevice,
        callback: TobiiWearableAdvancedDataCallback,
        user_data: *mut c_void,
    ) -> TobiiError;
    pub fn tobii_wearable_advanced_data_unsubscribe(device: *mut TobiiDevice) -> TobiiError;
    pub fn tobii_get_lens_configuration(
        device: *mut TobiiDevice,
        lens_config: *mut TobiiLensConfiguration,
    ) -> TobiiError;
    pub fn tobii_set_lens_configuration(
        device: *mut TobiiDevice,
        lens_config: *const TobiiLensConfiguration,
    ) -> TobiiError;
    pub fn tobii_lens_configuration_writable(
        device: *mut TobiiDevice,
        writable: *mut TobiiLensConfigurationWritable,
    ) -> TobiiError;
    pub fn tobii_wearable_foveated_gaze_subscribe(
        device: *mut TobiiDevice,
        callback: TobiiWearableFoveatedGazeCallback,
        user_data: *mut c_void,
    ) -> TobiiError;
    pub fn tobii_wearable_foveated_gaze_unsubscribe(device: *mut TobiiDevice) -> TobiiError;
}
