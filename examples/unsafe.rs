use tobii_stream_engine::bindings::core::*;
use std::os::raw::{c_longlong};

fn main() {
    unsafe {
        let mut api_ptr: *mut TobiiApi = std::mem::zeroed();
        assert_eq!(tobii_api_create(&mut api_ptr as *mut *mut TobiiApi,
                                    std::ptr::null_mut(),
                                    std::ptr::null_mut()),
                   TOBII_ERROR_NO_ERROR);
        let mut timestamp: i64 = 0;
        let timestamp_ptr = &mut timestamp as *mut c_longlong;
        assert_eq!(tobii_system_clock(api_ptr, timestamp_ptr), TOBII_ERROR_NO_ERROR);
        println!("{}", timestamp);
    }
}
