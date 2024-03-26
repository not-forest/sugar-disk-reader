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
            errors::UserServiceError,
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
                    if user.check_pass(pass) {
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
    pub(crate) async fn signup(mail: String, pass: String, conf: String) -> UserServiceStatus {
        log::debug!("Encountered signup request with data: {:#?}", (&mail, &pass, &conf));
    
        log::info!("Signup: OK");
        UserServiceStatus::NoError
    }
}


