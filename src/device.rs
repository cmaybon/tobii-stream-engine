use crate::bindings::core::*;
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