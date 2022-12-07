use crate::bindings::core::*;
use std::os::raw::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TobiiLicenseKey {
    pub license_key: *const u16,
    pub size_in_bytes: usize,
}

pub type TobiiLicenseValidationResult = c_int;
pub const TOBII_LICENSE_VALIDATION_RESULT_OK: TobiiLicenseValidationResult = 0;
pub const TOBII_LICENSE_VALIDATION_RESULT_TAMPERED: TobiiLicenseValidationResult = 1;
pub const TOBII_LICENSE_VALIDATION_RESULT_INVALID_APPLICATION_SIGNATURE : TobiiLicenseValidationResult = 2 ;
pub const TOBII_LICENSE_VALIDATION_RESULT_NONSIGNED_APPLICATION : TobiiLicenseValidationResult = 3 ;
pub const TOBII_LICENSE_VALIDATION_RESULT_EXPIRED: TobiiLicenseValidationResult = 4;
pub const TOBII_LICENSE_VALIDATION_RESULT_PREMATURE: TobiiLicenseValidationResult = 5;
pub const TOBII_LICENSE_VALIDATION_RESULT_INVALID_PROCESS_NAME: TobiiLicenseValidationResult = 6;
pub const TOBII_LICENSE_VALIDATION_RESULT_INVALID_SERIAL_NUMBER : TobiiLicenseValidationResult = 7 ;
pub const TOBII_LICENSE_VALIDATION_RESULT_INVALID_MODEL: TobiiLicenseValidationResult = 8;
pub const TOBII_LICENSE_VALIDATION_RESULT_INVALID_PLATFORM_TYPE : TobiiLicenseValidationResult = 9 ;

pub type TobiiFeatureGroup = c_int;
pub const TOBII_FEATURE_GROUP_BLOCKED: TobiiFeatureGroup = 0;
pub const TOBII_FEATURE_GROUP_CONSUMER: TobiiFeatureGroup = 1;
pub const TOBII_FEATURE_GROUP_CONFIG: TobiiFeatureGroup = 2;
pub const TOBII_FEATURE_GROUP_PROFESSIONAL: TobiiFeatureGroup = 3;
pub const TOBII_FEATURE_GROUP_INTERNAL: TobiiFeatureGroup = 4;

#[link(name = "third_party/tobii/tobii_stream_engine")]
extern "C" {
    pub fn tobii_device_create_ex(
        api: *mut TobiiApi,
        url: *const c_char,
        field_of_use: TobiiFieldOfUse,
        license_keys: *const TobiiLicenseKey,
        license_count: c_int,
        license_results: *mut TobiiLicenseValidationResult,
        device: *mut *mut TobiiDevice,
    ) -> TobiiError;
    pub fn tobii_license_key_store(
        device: *mut TobiiDevice,
        data: *mut c_void,
        size: usize,
    ) -> TobiiError;
    pub fn tobii_license_key_retrieve(
        device: *mut TobiiDevice,
        receiver: TobiiDataReceiver,
        user_data: *mut c_void,
    ) -> TobiiError;
    pub fn tobii_get_feature_group(
        device: *mut TobiiDevice,
        feature_group: *mut TobiiFeatureGroup,
    ) -> TobiiError;
}
