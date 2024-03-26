//! Additional modules for custom errors definitions.

use std::error::Error;
use std::fmt::Display;

/// Error type for user service.
///
/// This error could be handled on higher level of function calls. 
#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UserServiceError {
    /// Unabling to create new user. Usually because user already exists.
    UserCreationError =     1,
    /// Unabling to get user from firebase.
    UserGetError =          2,
    /// Unabling to set data fields for user.
    UserSetError =          3,
    /// Unabling to delete user.
    UserDeleteError =       4,
    /// Unable to parse the data, due to inner bug.
    InputParseError =       5,
    /// Unable to connect to the firebase.
    DatabaseConnectionError = 6,
}

impl Error for UserServiceError {}
impl Display for UserServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use UserServiceError::*;
        match self {
            UserCreationError =>         write!(f, "Unable to create new user."),
            UserGetError =>              write!(f, "Unable to get user from the database."),
            UserSetError =>              write!(f, "Unable to set data for user."),
            UserDeleteError =>           write!(f, "Unable to delete user."),
            InputParseError =>           write!(f, "Unable to parse data due to inner bug."),
            DatabaseConnectionError =>   write!(f, "Failed to establish connection to Firebase's database.")
        }
    }
}

/// Error type from user's info parsing.
///
/// This error could be handler from the highest front-end level of functions calls.
#[derive(Debug, PartialEq, Eq)]
pub enum DataParseError {
    /// Will be provided if the mail is not in right format.
    MailRegexError =            10,
    /// Will be provided if the password conditions are not met.
    PasswordConditionsError =   11,
}

impl Error for DataParseError {}
impl Display for DataParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use DataParseError::*;
        match self {
            MailRegexError =>          write!(f, "Wrong email regex."),
            PasswordConditionsError => write!(f, "Password conditions are not met."),
        }
    }
}
