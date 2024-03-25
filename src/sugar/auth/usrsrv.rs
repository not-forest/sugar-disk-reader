//! Module that provides comunication with firebase's realtime database via it's API.
//!
//! This module handles all events related to user authentifications, which includes logins,
//! registration and modifications requested by users.

use firebase_rs::Firebase;
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;

/// Static users database values.
const USERS_DOC: &'static str = "users";

/// Module which contains all service funcions related to user authentification.
pub mod service {
    use firebase_rs::Firebase;
    use crate::sugar::FIREBASE_URI;
    use super::UserP;


    /// Performs full application login procedure.
    ///
    /// Performs communication with firebase server and provides full login routine. Starts user's
    /// session if data match and 
    #[tokio::main]
    pub async fn login(mail: String, pass: String) { 
         log::info!("Encountered login request with data: {:#?}", (mail, pass));
    }

    /// Performs full application signup procedure.
    ///
    /// With data provided, creates new 'Sugar' user, while checking if such user is not already
    /// exist.
    #[tokio::main]
    pub async fn signup(mail: String, pass: String, conf: String) {
         log::info!("Encountered signup request with data: {:#?}", (mail, pass, conf));
    }
}

/// Generic model of user entry within firebase database.
///
/// This user model is required for custom configurations, accesses and application
/// tasks on backend/low level. It does provide it's data to front-end user area, therefore
/// it only constains public fields, with only password being hashed. 
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct UserP {
    /// User's email.
    pub mail: String,
    /// User's hashed password.
    pub pass: String,
}

impl UserP {
    /// Creates a new instance of 'UserP'
    ///
    /// This function is only creating a local instance and not reading or writing
    /// anythin to the database. When creating new user, it automatically writes
    /// them as an entry to the firebase's database.
    ///
    /// Returned password will be a hashed version of provided, because the program must
    /// get rid of it as fast as possible to leak it to global scope. The hash is written
    /// in the database and cannot be read.
    pub async fn new(mail: String, pass: String, fire: &Firebase) -> Result<Self, UserServiceError> {
        let users = fire.at(USERS_DOC);
        let pass_hash = bcrypt::hash(pass).unwrap();    // Bcrypt.

        let user = Self {
            mail: mail.clone(), 
            pass: pass_hash, 
        }; 

        // Writes to database. If failed, returns response.
        if users.set(&user).await.is_err() {
            if users.at(&mail).get::<UserP>().await.is_err() {
                // User exists.
                return Err(UserServiceError::UserCreationError) 
            }
            // Internal fail.
            return Err(UserServiceError::UserGetError)
        }

        Ok(user)
    }

    /// Gets a user by provided id.
    pub async fn get(id: usize, fire: &Firebase) -> Result<Self, UserServiceError> {
        if let Ok(user) = fire
            .at(USERS_DOC)
            .at(&id.to_string())
            .get::<UserP>()
            .await {
                return Ok(user)
        }
        Err(UserServiceError::UserGetError)
    }

    /// Provides an interface to modify user entries in the database.
    pub async fn set<F>(id: usize, fire: &Firebase, modify_f: F) -> Result<(), UserServiceError> where
        F: FnOnce(&mut UserP)
    {
        if let Ok(mut user) = UserP::get(id, fire).await {
            modify_f(&mut user);              // Modifying the user.
            
            if fire.at(USERS_DOC).at(&id.to_string()).update(&user).await.is_err() {
                return Err(UserServiceError::UserSetError)
            }
            return Ok(())
        }
        Err(UserServiceError::UserGetError)
    }

    /// Gets N amount of queries from the database.
    pub async fn get_n(fire: &Firebase, amount: u32) -> Result<HashMap<String, UserP>, UserServiceError> {
        if let Ok(users) = fire
            .at(USERS_DOC)
            .with_params()
            .limit_to_first(amount)
            .finish()
            .get::<HashMap<String, UserP>>()
            .await {
            return Ok(users)
        }
        
        Err(UserServiceError::UserGetError)
    }

    /// Finds entry by provided query
    pub async fn find(fire: &Firebase, query: String) -> Result<UserP, UserServiceError> {
        if let Ok(user) = fire
            .at(USERS_DOC)
            .at(&query)
            .get::<UserP>()
            .await {
            return Ok(user)
        }
        
        Err(UserServiceError::UserGetError)

    } 
}

/// Error type for user service.
///
/// This error could be handled on higher level of function calls. 
#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UserServiceError {
    /// Unabling to create new user. Usually because user already exists.
    UserCreationError,
    /// Unabling to get user from firebase.
    UserGetError,
    /// Unabling to set data fields for user.
    UserSetError,
    /// Unabling to delete user.
    UserDeleteError,
}

impl Error for UserServiceError {}
impl Display for UserServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use UserServiceError::*;
        match self {
            UserCreationError => write!(f, "Unable to create new user."),
            UserGetError =>      write!(f, "Unable to get user from the database."),
            UserSetError =>      write!(f, "Unable to set data for user."),
            UserDeleteError =>   write!(f, "Unable to delete user."),
        }
    }
}
