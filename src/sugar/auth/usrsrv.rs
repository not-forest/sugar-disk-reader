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

    use super::UserServiceStatus;
    use crate::sugar::{
        api::FIREBASE_API_KEY, errors::StorageError, storage::LocalStorage
    };

    /// Performs full application login procedure.
    ///
    /// Performs communication with firebase server and provides full login routine. Starts user's
    /// session if data match and 
    #[tokio::main]
    pub async fn login(mail: String, pass: String) -> UserServiceStatus { 
        log::debug!("Encountered login request with data: {:#?}", (&mail, &pass));
        
        // Authentications service.
        let auth = FireAuth::new(FIREBASE_API_KEY.to_string());

        'main: loop {
            // Parsing an output from firebase server.
            return match auth.sign_in_email(&mail, &pass, true).await {
                Ok(res) => {
                    log::debug!("Obtained response: {:#?}", res);

                    // Writing newest response to the local storage for use later.
                    'inner: loop {
                        if let Err(err) = LocalStorage::write(&res, "login_response") {
                            match err {
                                // It is better to retry if our write was interrupted at that point.
                                StorageError::INTERRUPTED => continue 'inner,
                                // Obtained bad data for some reason. Retrying the whole procedure.
                                StorageError::NO_DATA | StorageError::BAD_DATA => continue 'main,
                                _ => (),
                            }
                        }

                        log::info!("Latest response info saved.");
                        break
                    }

                    log::info!("Login: OK");
                    UserServiceStatus::NoError
                },
                Err(err) => {
                    let converted = err.clone().into();
                    log::error!("Login error: {}", &converted);
                    log::error!("Details: {}", &err);

                    UserServiceStatus::SignupError(converted)
                },
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

        'main: loop {
            // Parsing an output from firebase server.
            return match auth.sign_up_email(&mail, &pass, true).await {
                Ok(res) => {
                    log::debug!("Obtained response: {:#?}", res);

                    // Writing newest response to the local storage for use later.
                    'inner: loop {
                        if let Err(err) = LocalStorage::write(&res, "signup_response") {
                            match err {
                                // It is better to retry if our write was interrupted at that point.
                                StorageError::INTERRUPTED => continue 'inner,
                                // Obtained bad data for some reason. Retrying the whole procedure.
                                StorageError::NO_DATA | StorageError::BAD_DATA => continue 'main,
                                _ => (),
                            }
                        }

                        log::info!("Latest response info saved.");
                        break
                    }

                    log::info!("Signup: OK");
                    UserServiceStatus::NoError
                },
                Err(err) => {
                    let converted = err.clone().into();
                    log::error!("Signup error: {}", &converted);
                    log::error!("Details: {}", &err);

                    UserServiceStatus::SignupError(converted)
                },
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
