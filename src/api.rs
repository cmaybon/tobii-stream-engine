use crate::bindings::*;
use crate::helpers;
use std::os::raw::*;
use std::ffi::{CStr, CString};

pub enum FieldOfUse {
    /// Device will be created for interactive use. No special license is required for this type use.
    /// Eye tracking data is only used as a user input for interaction experiences and cannot be
    /// stored, transmitted, nor analyzed or processed for other purposes.
    Interactive = TOBII_FIELD_OF_USE_INTERACTIVE as isize,
    /// Device will be created for analytical use. This requires a special license from Tobii.
    /// Eye tracking data is used to analyze user attention, behavior or decisions in applications
    /// that store, transfer, record or analyze the data.
    Analytical = TOBII_FIELD_OF_USE_ANALYTICAL as isize,
}

#[derive(Debug)]
pub enum Error {
    NoError = TOBII_ERROR_NO_ERROR as isize,
    Internal = TOBII_ERROR_INTERNAL as isize,
    InsufficientLicense = TOBII_ERROR_INSUFFICIENT_LICENSE as isize,
    NotSupported = TOBII_ERROR_NOT_SUPPORTED as isize,
    NotAvailable = TOBII_ERROR_NOT_AVAILABLE as isize,
    ConnectionFailed = TOBII_ERROR_CONNECTION_FAILED as isize,
    TimedOut = TOBII_ERROR_TIMED_OUT as isize,
    AllocationFailed = TOBII_ERROR_ALLOCATION_FAILED as isize,
    InvalidParameter = TOBII_ERROR_INVALID_PARAMETER as isize,
    CalibrationAlreadyStarted = TOBII_ERROR_CALIBRATION_ALREADY_STARTED as isize,
    CalibrationNotStarted = TOBII_ERROR_CALIBRATION_NOT_STARTED as isize,
    AlreadySubscribed = TOBII_ERROR_ALREADY_SUBSCRIBED as isize,
    NotSubscribed = TOBII_ERROR_NOT_SUBSCRIBED as isize,
    OperationFailed = TOBII_ERROR_OPERATION_FAILED as isize,
    ConflictingApiInstances = TOBII_ERROR_CONFLICTING_API_INSTANCES as isize,
    CalibrationBusy = TOBII_ERROR_CALIBRATION_BUSY as isize,
    CallbackInProgress = TOBII_ERROR_CALLBACK_IN_PROGRESS as isize,
}

impl Error {
    fn tobii_error_as_result(tobii_error: TobiiError) -> Result<(), Error> {
        match tobii_error {
            TOBII_ERROR_NO_ERROR => Ok(()),
            TOBII_ERROR_INTERNAL => Err(Error::Internal),
            TOBII_ERROR_INSUFFICIENT_LICENSE => Err(Error::InsufficientLicense),
            TOBII_ERROR_NOT_SUPPORTED => Err(Error::NotSupported),
            TOBII_ERROR_NOT_AVAILABLE => Err(Error::NotAvailable),
            TOBII_ERROR_CONNECTION_FAILED => Err(Error::ConnectionFailed),
            TOBII_ERROR_TIMED_OUT => Err(Error::TimedOut),
            TOBII_ERROR_ALLOCATION_FAILED => Err(Error::AllocationFailed),
            TOBII_ERROR_INVALID_PARAMETER => Err(Error::InvalidParameter),
            TOBII_ERROR_CALIBRATION_ALREADY_STARTED => Err(Error::CalibrationAlreadyStarted),
            TOBII_ERROR_CALIBRATION_NOT_STARTED => Err(Error::CalibrationNotStarted),
            TOBII_ERROR_ALREADY_SUBSCRIBED => Err(Error::AlreadySubscribed),
            TOBII_ERROR_NOT_SUBSCRIBED => Err(Error::NotSubscribed),
            TOBII_ERROR_OPERATION_FAILED => Err(Error::OperationFailed),
            TOBII_ERROR_CONFLICTING_API_INSTANCES => Err(Error::ConflictingApiInstances),
            TOBII_ERROR_CALIBRATION_BUSY => Err(Error::CalibrationBusy),
            TOBII_ERROR_CALLBACK_IN_PROGRESS => Err(Error::CallbackInProgress),
            _ => panic!("Unknown Tobii error \"{}\".", tobii_error),
        }
    }
}

#[derive(Debug)]
pub struct Device {
    ptr: *mut TobiiDevice,
    url: String,
    pub serial_number: String,
    pub model: String,
    pub generation: String,
    pub firmware_version: String,
    pub integration_id: String,
    pub hw_calibration_version: String,
    pub hw_calibration_date: String,
    pub lot_id: String,
    pub integration_type: String,
    pub runtime_build_version: String,
}

impl Device {
    pub fn new(api: &Api, url: String, field_of_use: FieldOfUse) -> Result<Device, Error> {
        unsafe {
            let url_c = CString::new(url.as_bytes()).unwrap();
            let url_c = url_c.as_c_str();
            let mut device_ptr: *mut TobiiDevice = std::mem::zeroed();
            let error = tobii_device_create(api.ptr,
                                            url_c.as_ptr(),
                                            field_of_use as u32,
                                            &mut device_ptr as *mut *mut TobiiDevice);
            match Error::tobii_error_as_result(error) {
                Ok(_) => {
                    Ok(Device {
                        ptr: device_ptr,
                        url,
                        serial_number: "".to_string(),
                        model: "".to_string(),
                        generation: "".to_string(),
                        firmware_version: "".to_string(),
                        integration_id: "".to_string(),
                        hw_calibration_version: "".to_string(),
                        hw_calibration_date: "".to_string(),
                        lot_id: "".to_string(),
                        integration_type: "".to_string(),
                        runtime_build_version: "".to_string()
                    })
                }
                Err(error_type) => Err(error_type)
            }
        }
    }

    pub fn reconnect(&mut self) -> Result<(), Error> {
        let error = unsafe {
            tobii_device_reconnect(self.ptr)
        };
        Error::tobii_error_as_result(error)
    }

    pub fn get_info(&mut self) {
        unsafe {
            let mut tobii_device_info = &mut TobiiDeviceInfo::default() as *mut TobiiDeviceInfo;
            let error = tobii_get_device_info(self.ptr, tobii_device_info);
            assert_eq!(error, TOBII_ERROR_NO_ERROR);
            self.serial_number = helpers::c_char_ptr_to_string((*tobii_device_info).serial_number.as_ptr());
            self.model = helpers::c_char_ptr_to_string((*tobii_device_info).model.as_ptr());
            self.generation = helpers::c_char_ptr_to_string((*tobii_device_info).generation.as_ptr());
            self.firmware_version = helpers::c_char_ptr_to_string((*tobii_device_info).firmware_version.as_ptr());
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        let status: TobiiError = unsafe {
            tobii_device_destroy(self.ptr)
        };
        assert_eq!(status, TOBII_ERROR_NO_ERROR);
        println!("Successfully destroyed Device")
    }
}

pub struct Api {
    ptr: *mut TobiiApi
}

impl Drop for Api {
    fn drop(&mut self) {
        let status: TobiiError = unsafe {
            tobii_api_destroy(self.ptr)
        };
        assert_eq!(status, TOBII_ERROR_NO_ERROR);
        println!("Successfully destroyed API");
    }
}

impl Api {
    unsafe extern "C" fn custom_api_log(_log_context: *mut c_void, level: LogLevel, text: *const c_char) {
        println!("INTERNAL LOG - {}: {}", level, std::ffi::CStr::from_ptr(text).to_str().unwrap());
    }

    pub fn new() -> Api {
        unsafe {
            let mut api_ptr: *mut TobiiApi = std::mem::zeroed();
            let custom_log = TobiiCustomLog {
                log_context: std::ptr::null_mut(),
                log_func: Some(Api::custom_api_log),
            };

            assert_eq!(tobii_api_create(&mut api_ptr as *mut *mut TobiiApi,
                                        std::ptr::null_mut(),
                                        &custom_log as *const TobiiCustomLog),
                       TOBII_ERROR_NO_ERROR);
            println!("Successfully created API");
            Api {
                ptr: api_ptr,
            }
        }
    }

    /// Returns a string of the current API version in the format `major.minor.revision.build`.
    /// - `major` incremented for API changes which are not backward-compatible.
    /// - `minor` incremented for releases which add new, but backward-compatible, API features.
    /// - `revision` incremented for minor changes and bug fixes which do not change the API.
    /// - `build` incremented every time a new build is done, even when there are no changes.
    pub fn get_api_version_string() -> String {
        let v: TobiiVersion = unsafe {
            let mut version = TobiiVersion::default();
            assert_eq!(tobii_get_api_version(&mut version), TOBII_ERROR_NO_ERROR);
            version
        };
        format!("{}.{}.{}.{}", v.major, v.minor, v.revision, v.build)
    }

    /// Returns the current system time in microseconds (us), from the same clock used to time-stamp callback data.
    /// The epoch is undefined.
    /// The timestamps are only useful for calculating the time elapsed between a pair of values.
    /// These timestamps are the same as the ones used in stream engine callbacks.
    pub fn system_clock(&mut self) -> i64 {
        let mut timestamp: i64 = 0;
        unsafe {
            let timestamp_ptr = &mut timestamp as *mut c_longlong;
            assert_eq!(tobii_system_clock(self.ptr, timestamp_ptr), TOBII_ERROR_NO_ERROR);
        }
        timestamp
    }

    /// Will be called for each device found during enumeration.
    ///
    /// Based on the example code in the Tobii documentation C++ example [here](https://developer.tobii.com/product-integration/stream-engine/getting-started/)
    /// # Arguments
    /// * `url` - A zero terminated string for the URL of the device. This pointer will be invalid after
    ///    returning from the function. Make sure to make a copy of the string rather than storing the pointer.
    /// * `user_data` - Custom pointer sent to `tobii_enumerate_local_device_urls
    unsafe extern "C" fn device_url_receiver(url: *const c_char, user_data: *mut c_void) {
        let buffer = &mut *(user_data as *mut Vec<String>);
        let url_str = CStr::from_ptr(url).to_str().unwrap().to_string();
        if url_str.len() < 256 {
            buffer.push(url_str);
        }
    }

    pub fn device_urls(&mut self) -> Vec<String> {
        let mut list: Vec<String> = Vec::new();
        let list_ptr = &mut list as *mut Vec<String>;
        let error = unsafe {
            tobii_enumerate_local_device_urls(self.ptr, Some(Api::device_url_receiver), list_ptr as *mut c_void)
        };

        assert_eq!(error, TOBII_ERROR_NO_ERROR);
        list
    }

    pub fn get_all_interactive_devices(&mut self) -> Vec<Device> {
        let urls = self.device_urls();
        let mut devices: Vec<Device> = vec![];
        for url in urls {
            let result = Device::new(&self, url, FieldOfUse::Interactive);
            match result {
                Ok(device) => devices.push(device),
                Err(_) => {}
            }
        }
        devices
    }

    fn error_message_to_string(error: TobiiError) -> String {
        let message = unsafe {
            let message_ptr = tobii_error_message(error);
            CStr::from_ptr(message_ptr).to_str().unwrap()
        };
        message.to_string()
    }

    pub fn new_device(&self, url: String, field_of_use: FieldOfUse) -> Result<Device, Error> {
        Device::new(&self, url, field_of_use)
    }
}
