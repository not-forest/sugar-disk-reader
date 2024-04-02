//! Module for defining different status errors.
#![allow(non_camel_case_types)]

use firebase_auth_sdk::Error;

/// Internal application errors.
///
/// Errors, which occur due to some internal application faults.
#[repr(u8)]
pub enum InternalError {
    /// Can occur if some of tokio threads did not exit successfully or panicked for some reason.
    /// This is a inner application bug.
    TOKIO_THREAD_ERROR = 1,
    /// Will occur if unable to connect to some service. This is most likely a networking issue.
    NETWORK_ERROR = 2,
}

/// Errors which occur during signup.
#[repr(u8)]
pub enum SignupError {
    /// Email is in wrong format.
    INVALID_EMAIL = 10,
    /// Password is too short.
    INVALID_PASS = 11,
    /// User with provided email already exist.
    EMAIL_EXISTS = 20,
    /// Too many attempts. This will happen, if the device was doing too many requests and
    /// firebase counter it as an unusual activity.
    TOO_MANY_ATTEMPTS = 21,
}

/// Errors which occur during login.
#[repr(u8)]
pub enum LoginError {
    /// Email is in wrong format.
    INVALID_EMAIL = 10,
    /// Password is too short.
    INVALID_PASS = 11,
    /// There is no user in firebase auth system with provided email.
    INVALID_LOGIN_CREDENTIALS = 30,
    /// User was disabled by an administrator.
    USER_DISABLED = 31,
}

impl Into<SignupError> for Error {
    fn into(self) -> SignupError {
        match self {
            Self::SignUp(s) => {
                if s.contains("INVALID_EMAIL") {
                    return SignupError::INVALID_EMAIL
                }
                if s.contains("WEAK_PASSWORD") {
                    return SignupError::INVALID_PASS

                }
                if s.contains("EMAIL_EXISTS") {
                    return SignupError::EMAIL_EXISTS

                }
                if s.contains("TOO_MANY_ATTEMPTS_TRY_LATER") {
                    return SignupError::TOO_MANY_ATTEMPTS

                }

                log::error!("Fatal error: {}", s);
                panic!();            
            }
            _ => unreachable!(),
        }
    }
}

impl Into<LoginError> for Error {
    fn into(self) -> LoginError {
        match self {
            Self::SignIn(s) => {
                if s.contains("INVALID_EMAIL") {
                    return LoginError::INVALID_EMAIL
                }
                if s.contains("WEAK_PASSWORD") {
                    return LoginError::INVALID_PASS

                }
                if s.contains("INVALID_LOGIN_CREDENTIALS") {
                    return LoginError::INVALID_LOGIN_CREDENTIALS

                }
                if s.contains("USER_DISABLED") {
                    return LoginError::USER_DISABLED

                }

                log::error!("Fatal error: {}", s);
                panic!();   
            },
            _ => unreachable!(),
        }
    }
}
