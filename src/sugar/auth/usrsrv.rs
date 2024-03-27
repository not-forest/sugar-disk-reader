//! Module that provides comunication with firebase's realtime database via it's API.
//!
//! This module handles all events related to user authentifications, which includes logins,
//! registration and modifications requested by users.

/// Module which contains all service funcions related to user authentification.
pub mod service {
    use firebase_rs::Firebase;

    use crate::sugar::{
        auth::{
            structures::{UserServiceStatus, UserP},
            errors::{UserServiceError, DataParseError},
        }, 
        parse::SugarParser, 
        FIREBASE_URI
    };


    /// Performs full application login procedure.
    ///
    /// Performs communication with firebase server and provides full login routine. Starts user's
    /// session if data match and 
    #[tokio::main]
    pub async fn login(mail: String, pass: String) -> UserServiceStatus { 
        log::debug!("Encountered login request with data: {:#?}", (&mail, &pass));
        let m = mail.clone();

        // Parsing.
        let parse_handle = tokio::spawn(async move {
            log::debug!("Parsing email...");
            SugarParser::parse_mail(m)?; // If mail fails, instant return of error.
           
            log::debug!("Parsing: OK");
            Ok(())
        });

        // Server side.
        
        let login_handle = tokio::spawn(async move {
            log::debug!("Connecting to firebase...");
            // Connecting to Firebase.
            if let Ok(fire) = Firebase::new(FIREBASE_URI) {
                // Trying to obtain a user by mail.
                return UserP::find(&fire, mail).await
            }

            log::error!("Connection error!");
            Err(UserServiceError::DatabaseConnectionError)
        });

        // Conclution.

        // Obtaining data from parser.
        if let Ok(out) = parse_handle.await {
            // Checking for any parse errors.
            if let Err(error) = out {
                log::error!("Parsing error: {}", error);
                
                return UserServiceStatus::UserSideError(
                    error
                )
            }
        } else {
            log::error!("Internal server error: Tokio task did not exit normally.");

            // Unable to get future from parser task.
            return UserServiceStatus::ServerSideError(
                UserServiceError::InputParseError
            )
        }

        // Obtaining data from login service.
        if let Ok(out) = login_handle.await {
            // Checking for any login errors and comparing provided info.
            match out {
                // If user exist, comparing email with the provided one.
                Ok(user) => {
                    if !user.check_pass(pass) {
                        // Password is wrong, returning an error.
                        return UserServiceStatus::UserSideError(
                            DataParseError::WrongPassword
                        )
                    }
                },
                // Error.
                Err(error) => {
                    log::error!("Serverside error: {}", error);
                    return UserServiceStatus::ServerSideError(
                        error
                    )
                },
            }
        } else {
            // Unable to get future from login task.
            return UserServiceStatus::ServerSideError(
                UserServiceError::UserGetError
            )
        }

        log::info!("Login: OK");
        UserServiceStatus::NoError
    }

    /// Performs full application signup procedure.
    ///
    /// With data provided, creates new 'Sugar' user, while checking if such user is not already
    /// exist.
    #[tokio::main]
    pub async fn signup(mail: String, pass: String, conf: String) -> UserServiceStatus {
        log::debug!("Encountered signup request with data: {:#?}", (&mail, &pass, &conf));
        let m = mail.clone();
        let p = pass.clone();

        // Parsing.

        let parse_handle = tokio::spawn(async move {
            log::debug!("Parsing email...");
            SugarParser::parse_mail(m)?; // If mail fails, instant return of error.
            log::debug!("Parsing password..."); 
            SugarParser::parse_pass(p)?; // If password fails, instant return of error.

            log::debug!("Parsing: OK");
            Ok(())
        });

        // Server side.
        
        let signup_handle = tokio::spawn(async move {
            log::debug!("Connecting to firebase...");
            // Establishing new connection with firebase.
            if let Ok(fire) = Firebase::new(FIREBASE_URI) {
                return UserP::new(mail, pass, &fire).await
            }

            log::error!("Connection error!");
            Err(UserServiceError::DatabaseConnectionError)
        });

        // Conclution.

        // Obtaining data from parser.
        if let Ok(out) = parse_handle.await {
            // Checking for any parse errors.
            if let Err(error) = out {
                log::error!("Parsing error: {}", error);
                
                return UserServiceStatus::UserSideError(
                    error
                )
            }
        } else {
            log::error!("Internal server error: Tokio task did not exit normally.");

            // Unable to get future from parser task.
            return UserServiceStatus::ServerSideError(
                UserServiceError::InputParseError
            )
        }

        // Obtaining data from the server. 
        if let Ok(out) = signup_handle.await {
            // Checking for any login errors and comparing provided info.
            if let Err(error) = out {
                log::error!("Serverside error: {}", error);
                return UserServiceStatus::ServerSideError(
                    error
                )
            }
        } else {
            // Unable to get future from login task.
            return UserServiceStatus::ServerSideError(
                UserServiceError::UserGetError
            )
        }

        log::info!("Signup: OK");
        UserServiceStatus::NoError
    }

    // Updates user's data by writing changed fields into the Firebase.
    /* #[tokio::main]
    pub async fn update_user<F>(f: F) -> UserServiceStatus where 
        F: FnOnce(&mut UserP) 
    {
        log::debug!("Encountered user update request.");

        if let Ok(fire) = Firebase::new(FIREBASE_URI) {
            // Getting local user's info from the local storage.
            let id = LocalStorage::get_user();

            if let Err(error) = UserP::set(id, &fire, f).await {
                return UserServiceStatus::ServerSideError(
                    UserServiceError::UserSetError
                )
            }
        } else {
            log::error!("Connection error!");
            return UserServiceStatus::ServerSideError(
                UserServiceError::DatabaseConnectionError
            )
        }

        log::info!("Signup: OK");
        UserServiceStatus::NoError 
    } */
}


