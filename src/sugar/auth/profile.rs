//! Special module for manipulation with user's profile.
//!
//! This module provides several important functions for getting user's data
//! based on the current session as well as changing user's data via firebase.

use firebase_auth_sdk::{FireAuth, Error};
use firebase_auth_sdk::api::{SignInResponse, UpdateUser, User};
use crate::sugar::{api::FIREBASE_API_KEY, errors::LoginError, storage::LocalStorage};
use super::usrsrv::UserServiceStatus;


/// Tries to get the current user's email by current session.
///
/// # Returns
///
/// Will return an email as a string, if data was obtained successfully from 
/// the last login. Here, user service status is used for error codes only. 
pub async fn get_user() -> Result<String, UserServiceStatus> {
    match LocalStorage::read::<SignInResponse>("login_response") {
        Ok(mut res) => {
            let mut token = res.id_token;
            let auth = FireAuth::new(FIREBASE_API_KEY.to_string());

            'token: loop {
                // Verifying the current ID token.
                return match auth.verify_id_token(token.as_str()).await {
                    Ok(claim) => {
                        log::info!("Token login success: {:#?}", claim);

                        Ok(res.email)
                    },
                    Err(err) => match err.clone() {
                        Error::API(s) => {
                            log::error!("Firebase API error: {}", s);
                            Err(UserServiceStatus::LoginError(
                                    LoginError::KEY_PARSING_ERROR
                            ))
                        }, 
                        Error::Token(s) => {
                            log::error!("Token error: {}", s);

                            let token_err = err.into();

                            // Trying to refresh the expired token right away.
                            match token_err {
                                // If expired, refreshing.
                                LoginError::TOKEN_EXPIRED => {
                                    if let Some(refresh_token) = res.refresh_token {
                                        match auth.refresh_id_token(refresh_token.as_str()).await {
                                            Ok(claim) => {
                                                log::info!("Token refreshed, retrying to verify. Claim: {:#?}", claim);
                                                token = claim.id_token;
                                                res.id_token = token.clone();
                                                res.refresh_token = Some(claim.refresh_token);

                                                // Writing new token data to the
                                                LocalStorage::write(&res, "login_response").ok();
                                            },
                                            Err(err) => {
                                                log::error!("Unable to refresh the token. Login is required: {:#?}", err);
                                                return Err(UserServiceStatus::LoginError(token_err))
                                            },
                                        };
                                    }

                                    // Retrying with new token.
                                    continue 'token
                                },
                                _ => Err(UserServiceStatus::LoginError(token_err)),
                            }
                        },
                        _ => unreachable!(),
                    },
                }
            }
        },
        Err(err) => Err(UserServiceStatus::StorageError(err)),
    }
}

/// Gets user id based on the current session.
///
/// Will return an error if the current session is expired, or unabling to
/// read data from the local storage.
pub async fn get_user_id() -> Result<usize, UserServiceStatus> {
    match LocalStorage::read::<SignInResponse>("login_response") {
        Ok(res) => Ok(res.local_id.parse().expect("Unable to represent user's id as a number value.")),
        Err(err) => Err(UserServiceStatus::StorageError(err)),    
    }
}

/// Gets current logged in user as User structure.
pub async fn get_self() -> Result<User, UserServiceStatus> {
    let auth = FireAuth::new(FIREBASE_API_KEY.to_string());

    match LocalStorage::read::<SignInResponse>("login_response") {
        Ok(res) => {
            match auth.get_user_info(&res.id_token).await {
                Ok(user) => Ok(user),
                Err(err) => {
                    log::error!("Obtained error while trying to obtain user info: {}", err);

                    Err(UserServiceStatus::LoginError(err.into()))
                }
            }
        },
        Err(err) => Err(UserServiceStatus::StorageError(err)),    
    }
}

/// Gets current user password based on the session.
///
/// The returned password will be a hashed version.
pub async fn get_current_pass() -> Result<String, UserServiceStatus> {
    match get_self().await {
        Ok(user) => Ok(user.password_hash),
        Err(err) => Err(err),
    }
}

/// Makes a request to firebase for mail changing.
#[tokio::main]
pub async fn change_mail(mail: String) -> UserServiceStatus {
    use crate::sugar::errors::SignupError;
    let auth = FireAuth::new(FIREBASE_API_KEY.to_string());

    // If the mail is the same, there is no need to load the server.  
    if mail == get_user().await.ok().unwrap_or_default() {
        return UserServiceStatus::SignupError(SignupError::EMAIL_EXISTS);
    }
    
    match LocalStorage::read::<SignInResponse>("login_response") {
        Ok(mut res) => {
            // Trying to change email, since we have obtained the token.
            match auth.change_email(&res.id_token, &mail, true).await {
                Ok(secure_token) => {
                    // If secured token is obtained, trying to save it for the future use.
                    // Email is updated by that point.
                    log::info!("Changed mail successfully to: {}", &mail);

                    // Since the email has changed, we manually changing the login info.
                    res.email = mail;

                    // Writing changes.
                    if let Err(err) = LocalStorage::write(&res, "login_response") {
                        return UserServiceStatus::StorageError(err)
                    }

                    // Writing new info about the updated user to the local storage for not
                    // overloading the server.
                    match LocalStorage::write::<UpdateUser>(&secure_token, "secure_token") {
                        Ok(_) => UserServiceStatus::NoError,
                        Err(err) => UserServiceStatus::StorageError(err),
                    }
                },
                Err(err) => { 
                    match &err {
                        Error::User(s) => {
                            if s.contains("EMAIL_EXISTS") {
                                return UserServiceStatus::SignupError(SignupError::EMAIL_EXISTS)
                            }
                        }
                        _ => (),
                    }

                    log::error!("Obtained error while trying to change email: {}", err);
                    UserServiceStatus::LoginError(err.into())
                }
            }
        },
        Err(err) => UserServiceStatus::StorageError(err),
    }
}

/// Changes the current password to the new provided one.
///
/// Input data is required, because the application does not physically owns user's
/// password and only establishes communication between firebase and the mobile.
#[tokio::main]
pub async fn change_pass(old_pass: String, new_pass: String) -> UserServiceStatus {
    // If we will obtain a proper login response, it would mean that old_pass was correct.
    if let Ok(mail) = get_user().await {
        match super::usrsrv::service::login(mail, old_pass) {
            UserServiceStatus::NoError => (), // Continuing.
            status @ _ => return status, // Passing the status forward.
        } 
    }

    let auth = FireAuth::new(FIREBASE_API_KEY.to_string());

    // Trying to change the password.
    match LocalStorage::read::<SignInResponse>("login_response") {
        Ok(res) => {
            // Trying to change email, since we have obtained the token.
            match auth.change_password(&res.id_token, &new_pass, true).await {
                Ok(secure_token) => {
                    // If secured token is obtained, trying to save it for the future use.
                    // Email is updated by that point.
                    log::info!("Changed password successfully to: {}", &new_pass);

                    // Writing new info about the updated user to the local storage for not
                    // overloading the server.
                    match LocalStorage::write::<UpdateUser>(&secure_token, "secure_token") {
                        Ok(_) => UserServiceStatus::NoError,
                        Err(err) => UserServiceStatus::StorageError(err),
                    }
                },
                Err(err) => {
                    log::error!("Obtained error while trying to change the password: {}", err);
                    UserServiceStatus::LoginError(err.into())
                }
            }
        },
        Err(err) => UserServiceStatus::StorageError(err),
    }
}
