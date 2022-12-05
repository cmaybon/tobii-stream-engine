extern crate tobii_stream_engine;
use tobii_stream_engine::*;

#[derive(Debug,Clone,Copy)]
pub enum TobiiError {
    Internal,
    InsufficientLicense,
    NotSupported,
    NotAvailable,
    ConnectionFailed,
    TimedOut,
    AllocationFailed,
    InvalidParameter,
    CalibrationAlreadyStarted,
    CalibrationNotStarted,
    AlreadySubscribed,
    NotSubscribed,
    OperationFailed,
    ConflictingApiInstances,
    CalibrationBusy,
    CallbackInProgress,
    Unknown(Error),
}

pub fn status_to_result(error: Error) -> Result<(),TobiiError> {
    match error {
        TOBII_ERROR_NO_ERROR => Ok(()),
        TOBII_ERROR_INTERNAL => Err(TobiiError::Internal),
        TOBII_ERROR_INSUFFICIENT_LICENSE => Err(TobiiError::InsufficientLicense),
        TOBII_ERROR_NOT_SUPPORTED => Err(TobiiError::NotSupported),
        TOBII_ERROR_NOT_AVAILABLE => Err(TobiiError::NotAvailable),
        TOBII_ERROR_CONNECTION_FAILED => Err(TobiiError::ConnectionFailed),
        TOBII_ERROR_TIMED_OUT => Err(TobiiError::TimedOut),
        TOBII_ERROR_ALLOCATION_FAILED => Err(TobiiError::AllocationFailed),
        TOBII_ERROR_INVALID_PARAMETER => Err(TobiiError::InvalidParameter),
        TOBII_ERROR_CALIBRATION_ALREADY_STARTED => Err(TobiiError::CalibrationAlreadyStarted),
        TOBII_ERROR_CALIBRATION_NOT_STARTED => Err(TobiiError::CalibrationNotStarted),
        TOBII_ERROR_ALREADY_SUBSCRIBED => Err(TobiiError::AlreadySubscribed),
        TOBII_ERROR_NOT_SUBSCRIBED => Err(TobiiError::NotSubscribed),
        TOBII_ERROR_OPERATION_FAILED => Err(TobiiError::OperationFailed),
        TOBII_ERROR_CONFLICTING_API_INSTANCES => Err(TobiiError::ConflictingApiInstances),
        TOBII_ERROR_CALIBRATION_BUSY => Err(TobiiError::CalibrationBusy),
        TOBII_ERROR_CALLBACK_IN_PROGRESS => Err(TobiiError::CallbackInProgress),
        _ => Err(TobiiError::Unknown(error))
    }
}

fn safe_get_api_version() -> Error {
    unsafe {
        let mut version = ApiVersion {
            major: 0,
            minor: 0,
            revision: 0,
            build: 0
        };
        let error = tobii_get_api_version(&mut version);
        println!("{:?}", version);
        error
    }
}

fn main() {
    println!("{:?}", status_to_result(safe_get_api_version()));
}
