//! Module that provides communication with firebase's real time database via it's API.
//!
//! This module handles all events related to user authentications, which includes logins,
//! registration and modifications requested by users.

use crate::sugar::errors::{InternalError, SignupError, LoginError};

/// Special wrapper around service status number, that will be returned to the front-end.
#[repr(u8)]
pub enum UserServiceStatus {
    /// Status which indicates that everything went right.
    NoError = 0,
    /// Error, which comes from the application itself.
    InternalError(InternalError),
    /// Errors from the firebase server.
    SignupError(SignupError),
    LoginError(LoginError),
}

/// Module which contains all service functions related to user authentications.
pub mod service {
    use firebase_auth_sdk::FireAuth;
    use pwhash::bcrypt;

    use super::UserServiceStatus;
    use crate::sugar::api::FIREBASE_API_KEY;
    use crate::sugar::errors::InternalError;

    /// Performs full application login procedure.
    ///
    /// Performs communication with firebase server and provides full login routine. Starts user's
    /// session if data match and 
    #[tokio::main]
    pub async fn login(mail: String, pass: String) -> UserServiceStatus { 
        log::debug!("Encountered login request with data: {:#?}", (&mail, &pass));
        
        // Authentications service.
        let auth = FireAuth::new(FIREBASE_API_KEY.to_string());
        match bcrypt::hash(pass) {
            Ok(pass) => {
                // Parsing an output from firebase server.
                match auth.sign_in_email(&mail, &pass, true).await {
                    Ok(res) => {
                        log::debug!("Obtained response: {:#?}", res);

                        log::info!("Login: OK");
                        UserServiceStatus::NoError
                    },
                    Err(err) => {
                        log::error!("Login error: {}", err);

                        UserServiceStatus::LoginError(err.into())
                    },
                }
            },
            Err(err) => {
                log::error!("Hash error: {}", err);

                // Throw error for now. Maybe strip the symbols later.
                UserServiceStatus::InternalError(
                    InternalError::HASH_ERROR
                )
            }
        }
    }

    /// Performs full application signup procedure.
    ///
    /// With data provided, creates new 'Sugar' user, while checking if such user is not already
    /// exist.
    #[tokio::main]
    pub async fn signup(mail: String, pass: String, conf: String) -> UserServiceStatus {
        log::debug!("Encountered signup request with data: {:#?}", (&mail, &pass, &conf));
        
        // Authentications service.
        let auth = FireAuth::new(FIREBASE_API_KEY.to_string());

        match bcrypt::hash(pass) {
            Ok(pass) => {
                // Parsing an output from firebase server.
                match auth.sign_up_email(&mail, &pass, true).await {
                    Ok(res) => {
                        log::debug!("Obtained response: {:#?}", res);

                        log::info!("Signup: OK");
                        UserServiceStatus::NoError
                    },
                    Err(err) => {
                        log::error!("Sign up error: {}", err);

                        UserServiceStatus::SignupError(err.into())
                    },
                }
            },
            Err(err) => {
                log::error!("Hash error: {}", err);

                // Throw error for now. Maybe strip the symbols later.
                UserServiceStatus::InternalError(
                    InternalError::HASH_ERROR
                )
            }
        }
    }
}

// Formatting each error as a status code.
impl Into<u8> for UserServiceStatus {
    fn into(self) -> u8 {
        match self {
            Self::NoError => 0,
            Self::InternalError(err) => err as u8,
            Self::SignupError(err) => err as u8,
            Self::LoginError(err) => err as u8,
        }
    }
}
