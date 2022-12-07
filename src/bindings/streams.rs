use crate::bindings::core::*;
use std::os::raw::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TobiiGazePoint {
    pub timestamp_us: i64,
    pub validity: TobiiValidity,
    pub position_xy: [f32; 2usize],
}

pub type TobiiGazePointCallback = ::std::option::Option<
    unsafe extern "C" fn(
        gaze_point: *const TobiiGazePoint,
        user_data: *mut c_void,
    ),
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TobiiGazeOrigin {
    pub timestamp_us: i64,
    pub left_validity: TobiiValidity,
    pub left_xyz: [f32; 3usize],
    pub right_validity: TobiiValidity,
    pub right_xyz: [f32; 3usize],
}

pub type TobiiGazeOriginCallback = ::std::option::Option<
    unsafe extern "C" fn(
        gaze_origin: *const TobiiGazeOrigin,
        user_data: *mut c_void,
    ),
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TobiiEyePositionNormalized {
    pub timestamp_us: i64,
    pub left_validity: TobiiValidity,
    pub left_xyz: [f32; 3usize],
    pub right_validity: TobiiValidity,
    pub right_xyz: [f32; 3usize],
}

pub type TobiiEyePositionNormalizedCallback = ::std::option::Option<
    unsafe extern "C" fn(
        eye_position: *const TobiiEyePositionNormalized,
        user_data: *mut c_void,
    ),
>;

pub type TobiiUserPresenceStatus = c_uint;

pub const TOBII_USER_PRESENCE_STATUS_UNKNOWN: TobiiUserPresenceStatus = 0;
pub const TOBII_USER_PRESENCE_STATUS_AWAY: TobiiUserPresenceStatus = 1;
pub const TOBII_USER_PRESENCE_STATUS_PRESENT: TobiiUserPresenceStatus = 2;

pub type TobiiUserPresenceCallback = ::std::option::Option<
    unsafe extern "C" fn(
        status: TobiiUserPresenceStatus,
        timestamp_us: i64,
        user_data: *mut c_void,
    ),
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TobiiHeadPose {
    pub timestamp_us: i64,
    pub position_validity: TobiiValidity,
    pub position_xyz: [f32; 3usize],
    pub rotation_validity_xyz: [TobiiValidity; 3usize],
    pub rotation_xyz: [f32; 3usize],
}

pub type TobiiHeadPoseCallback = ::std::option::Option<
    unsafe extern "C" fn(
        head_pose: *const TobiiHeadPose,
        user_data: *mut c_void,
    ),
>;
pub type TobiiNotificationType = c_int;

pub const TOBII_NOTIFICATION_TYPE_CALIBRATION_STATE_CHANGED: TobiiNotificationType = 0;
pub const TOBII_NOTIFICATION_TYPE_EXCLUSIVE_MODE_STATE_CHANGED: TobiiNotificationType = 1;
pub const TOBII_NOTIFICATION_TYPE_TRACK_BOX_CHANGED: TobiiNotificationType = 2;
pub const TOBII_NOTIFICATION_TYPE_DISPLAY_AREA_CHANGED: TobiiNotificationType = 3;
pub const TOBII_NOTIFICATION_TYPE_FRAMERATE_CHANGED: TobiiNotificationType = 4;
pub const TOBII_NOTIFICATION_TYPE_POWER_SAVE_STATE_CHANGED: TobiiNotificationType = 5;
pub const TOBII_NOTIFICATION_TYPE_DEVICE_PAUSED_STATE_CHANGED: TobiiNotificationType = 6;
pub const TOBII_NOTIFICATION_TYPE_CALIBRATION_ENABLED_EYE_CHANGED: TobiiNotificationType = 7;
pub const TOBII_NOTIFICATION_TYPE_CALIBRATION_ID_CHANGED: TobiiNotificationType = 8;
pub const TOBII_NOTIFICATION_TYPE_COMBINED_GAZE_EYE_SELECTION_CHANGED: TobiiNotificationType = 9;
pub const TOBII_NOTIFICATION_TYPE_FAULTS_CHANGED: TobiiNotificationType = 10;
pub const TOBII_NOTIFICATION_TYPE_WARNINGS_CHANGED: TobiiNotificationType = 11;
pub const TOBII_NOTIFICATION_TYPE_FACE_TYPE_CHANGED: TobiiNotificationType = 12;

pub type TobiiNotificationValueType = c_int;

pub const TOBII_NOTIFICATION_VALUE_TYPE_NONE: TobiiNotificationValueType = 0;
pub const TOBII_NOTIFICATION_VALUE_TYPE_FLOAT: TobiiNotificationValueType = 1;
pub const TOBII_NOTIFICATION_VALUE_TYPE_STATE: TobiiNotificationValueType = 2;
pub const TOBII_NOTIFICATION_VALUE_TYPE_DISPLAY_AREA: TobiiNotificationValueType = 3;
pub const TOBII_NOTIFICATION_VALUE_TYPE_UINT: TobiiNotificationValueType = 4;
pub const TOBII_NOTIFICATION_VALUE_TYPE_ENABLED_EYE: TobiiNotificationValueType = 5;
pub const TOBII_NOTIFICATION_VALUE_TYPE_STRING: TobiiNotificationValueType = 6;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TobiiNotification {
    pub type_: TobiiNotificationType,
    pub value_type: TobiiNotificationValueType,
    pub value: TobiiNotification_Union_Value,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union TobiiNotification_Union_Value {
    pub float_: f32,
    pub state: tobii_state_bool_t,
    pub display_area: tobii_display_area_t,
    pub uint_: u32,
    pub enabled_eye: tobii_enabled_eye_t,
    pub string_: tobii_state_string_t,
}

pub type TobiiNotificationCallback = ::std::option::Option<
    unsafe extern "C" fn(
        notification: *const TobiiNotification,
        user_data: *mut c_void,
    ),
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TobiiUserPositionGuide {
    pub timestamp_us: i64,
    pub left_position_validity: TobiiValidity,
    pub left_position_normalized_xyz: [f32; 3usize],
    pub right_position_validity: TobiiValidity,
    pub right_position_normalized_xyz: [f32; 3usize],
}

pub type TobiiUserPositionGuideCallback = ::std::option::Option<
    unsafe extern "C" fn(
        user_position_guide: *const TobiiUserPositionGuide,
        user_data: *mut c_void,
    ),
>;

extern "C" {
    pub fn tobii_gaze_point_subscribe(
        device: *mut TobiiDevice,
        callback: TobiiGazePointCallback,
        user_data: *mut c_void
    ) -> TobiiError;
    pub fn tobii_gaze_point_unsubscribe(device: *mut TobiiDevice) -> TobiiError;
    pub fn tobii_gaze_origin_subscribe(
        device: *mut TobiiDevice,
        callback: TobiiGazeOriginCallback,
        user_data: *mut c_void
    ) -> TobiiError;
    pub fn tobii_gaze_origin_unsubscribe(device: *mut TobiiDevice) -> TobiiError;
    pub fn tobii_eye_position_normalized_subscribe(
        device: *mut TobiiDevice,
        callback: TobiiEyePositionNormalizedCallback,
        user_data: *mut c_void,
    ) -> TobiiError;
    pub fn tobii_eye_position_normalized_unsubscribe(device: *mut TobiiDevice) -> TobiiError;
    pub fn tobii_user_presence_subscribe(
        device: *mut TobiiDevice,
        callback: TobiiUserPresenceCallback,
        user_data: *mut c_void,
    ) -> TobiiError;
    pub fn tobii_user_presence_unsubscribe(device: *mut TobiiDevice) -> TobiiError;
    pub fn tobii_head_pose_subscribe(
        device: *mut TobiiDevice,
        callback: TobiiHeadPoseCallback,
        user_data: *mut c_void,
    ) -> TobiiError;
    pub fn tobii_head_pose_unsubscribe(device: *mut TobiiDevice) -> TobiiError;
    pub fn tobii_notifications_subscribe(
        device: *mut TobiiDevice,
        callback: TobiiNotificationCallback,
        user_data: *mut c_void,
    ) -> TobiiError;
    pub fn tobii_notifications_unsubscribe(device: *mut TobiiDevice) -> TobiiError;
    pub fn tobii_user_position_guide_subscribe(
        device: *mut TobiiDevice,
        callback: TobiiUserPositionGuideCallback,
        user_data: *mut c_void,
    ) -> TobiiError;
    pub fn tobii_user_position_guide_unsubscribe(device: *mut TobiiDevice) -> TobiiError;
}