//! Module for defining different status errors.
#![allow(non_camel_case_types)]

use firebase_auth_sdk::Error;
use std::fmt::Display;

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

/// Application error related to storage manipulations.
///
/// Basically flags if the read or write operation was done successfully or not. It is used with
/// both local storage and cloud alternatives.
#[repr(u8)]
pub enum StorageError {
    /// if trying to read from non-existing file.
    FILE_NOT_EXIST = 40,
    /// Either read or write is taking too much time and caused an IO timeout. 
    TIME_OUT = 41,
    /// The data's content does not match it's destination.
    BAD_DATA = 42,
    /// No data is provided for writing, or the file is empty.
    NO_DATA = 43,
    /// This can only occur if serde's Serialize/Deserialize macro will somehow fail. Not very likely. 
    SERIALIZATION_ERROR = 44,
    /// Not enough memory to perform read or write. This can be handled from the software side and
    /// fixed right away.
    OUT_OF_MEMORY = 45,
    /// An IO operation was interrupted.
    INTERRUPTED = 46,
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
    /// Token is expired and user must login once more to renew it.
    TOKEN_EXPIRED = 32,
    /// Token is not valid yet.
    TOKEN_NOT_VALID = 33,
    /// Either token's header is malformed or it is missing kid property.
    MALFORMED_TOKEN_HEADER = 34,
    /// Unable to parse keys from token validation request.
    KEY_PARSING_ERROR = 35,
}

impl Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TOKIO_THREAD_ERROR => write!(f, "Tokio Thread did not exit successfully"),
            Self::NETWORK_ERROR => write!(f, "Network error. No internet connection."),
        }
    }
}

impl Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FILE_NOT_EXIST => write!(f, "IO operation failed: File does not exist."),
            Self::BAD_DATA => write!(f, "IO operation failed: data's content does not match it's destination."),
            Self::NO_DATA => write!(f, "IO operation failed: No data."),
            Self::SERIALIZATION_ERROR => write!(f, "IO operation failed: Serde serialization/deserialization failed."),
            Self::TIME_OUT => write!(f, "IO operation failed: Operation timed out."),
            Self::INTERRUPTED => write!(f, "IO operation failed: Interrupted, retrying..."),
            Self::OUT_OF_MEMORY => write!(f, "IO opertation failed: Out of memory, retrying..."),
        }
    }
}

impl Display for SignupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::INVALID_EMAIL => write!(f, "Invalid email regex was provided."),
            Self::INVALID_PASS => write!(f, "Provided password is too weak."),
            Self::EMAIL_EXISTS => write!(f, "Email exists, login required."),
            Self::TOO_MANY_ATTEMPTS => write!(f, "Too many attepts. Waiting on CD."),
        }
    }
}

impl Display for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::INVALID_EMAIL => write!(f, "Invalid email regex was provided."),
            Self::INVALID_PASS => write!(f, "Provided password is too weak."),
            Self::INVALID_LOGIN_CREDENTIALS => write!(f, "Either password or email is wrong. Retry is needed."),
            Self::USER_DISABLED => write!(f, "User is disabled by application administrator."),
            Self::TOKEN_EXPIRED => write!(f, "Token is expired."),
            Self::TOKEN_NOT_VALID => write!(f, "Token is not valid yet."),
            Self::MALFORMED_TOKEN_HEADER => write!(f, "Malformed token header."),
            Self::KEY_PARSING_ERROR => write!(f, "Unable to parse keys from token validation requerst."),
        }
    }
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
                if s.contains("INVALID_LOGIN_CREDENTIALS") {
                    return LoginError::INVALID_LOGIN_CREDENTIALS
                }
                if s.contains("USER_DISABLED") {
                    return LoginError::USER_DISABLED

                }
            
                log::error!("Fatal error: {}", s);
                panic!();   
            },
            Self::Token(s) => {
                if s.contains("Invalid ID token") || s.contains("Token isn't valid yet!") {
                    return LoginError::TOKEN_NOT_VALID
                }
                if s.contains("Token is expired") {
                    return LoginError::TOKEN_EXPIRED
                }
                if s.contains("token header") {
                    return LoginError::MALFORMED_TOKEN_HEADER
                }

                log::error!("Fatal error: {}", s);
                panic!();  
            }
            _ => unreachable!(),
        }
    }
}
