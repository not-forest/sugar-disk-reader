//! Defines all data structures related to user service.

use firebase_rs::Firebase;
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use super::errors::{DataParseError, UserServiceError};

/// Static users database values.
const USERS_DOC: &'static str = "users";

/// Status codes from user service.
///
/// It defines a set of numerical values, mapped to different output information, which allows for
/// understanding between Rust's side and Java.
#[repr(u8)]
pub enum UserServiceStatus {
    /// Marks that no server side errors and user side errors occured.
    NoError =                       0,
    /// Marks that something wrong happend on server side.
    ///
    /// This error is unsolvable from front-end side, therefore it is only used as a marker to
    /// signal user about the error.
    ServerSideError(UserServiceError),
    /// Marks that some wrong info was provided from user's side.
    ///
    /// This will tell front-end that user has provided some info, which is impossible to parse or
    /// it is considered a wrong information after parsing.
    UserSideError(DataParseError),
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
    pub(crate) async fn new(mail: String, pass: String, fire: &Firebase) -> Result<Self, UserServiceError> {
        log::info!("Creating new user: {:?}", (&mail, &pass));
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
                log::error!("User exist.");
                return Err(UserServiceError::UserCreationError) 
            }
            // Internal fail.
            log::error!("Internal server error.");
            return Err(UserServiceError::UserGetError)
        }

        log::debug!("OK");
        Ok(user)
    }

    /// Compares the provided password and hashed string.
    pub(crate) fn check_pass(&self, pass: String) -> bool {
        bcrypt::verify(&pass, &self.pass)
    }

    /// Gets a user by provided id.
    pub(crate) async fn get(id: usize, fire: &Firebase) -> Result<Self, UserServiceError> {
        if let Ok(user) = fire
            .at(USERS_DOC)
            .at(&id.to_string())
            .get::<UserP>()
            .await {
                log::debug!("OK");
                return Ok(user)
        }
        log::error!("Unable to locate user by ID.");
        Err(UserServiceError::UserGetError)
    }

    /// Provides an interface to modify user entries in the database.
    pub(crate) async fn set<F>(id: usize, fire: &Firebase, modify_f: F) -> Result<(), UserServiceError> where
        F: FnOnce(&mut UserP)
    {
        if let Ok(mut user) = UserP::get(id, fire).await {
            modify_f(&mut user);              // Modifying the user.
            
            if fire.at(USERS_DOC).at(&id.to_string()).update(&user).await.is_err() {
                log::error!("Serverside error. Unable to update user's data.");
                return Err(UserServiceError::UserSetError)
            }
            log::debug!("OK");
            return Ok(())
        }
        Err(UserServiceError::UserGetError)
    }

    /// Gets N amount of queries from the database.
    pub(crate) async fn get_n(fire: &Firebase, amount: u32) -> Result<HashMap<String, UserP>, UserServiceError> {
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
    pub(crate) async fn find(fire: &Firebase, query: String) -> Result<UserP, UserServiceError> {
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
