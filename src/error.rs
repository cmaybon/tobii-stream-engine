use crate::bindings::core::*;

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
    pub fn tobii_error_as_result(tobii_error: TobiiError) -> Result<(), Error> {
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

