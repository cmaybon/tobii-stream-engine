use crate::bindings::core::*;
use crate::helpers;
use crate::device::{Device, FieldOfUse};
use crate::error::*;
use std::os::raw::*;
use std::ffi::{CStr, CString};

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
