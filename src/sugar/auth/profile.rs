//! Special module for manipulation with user's profile.
//!
//! This module provides several important functions for getting user's data
//! based on the current session as well as changing user's data via firebase.

use firebase_auth_sdk::{FireAuth, Error};
use firebase_auth_sdk::api::SignInResponse;
use crate::sugar::{api::FIREBASE_API_KEY, errors::LoginError, storage::LocalStorage};
use super::usrsrv::UserServiceStatus;


/// Tries to get the current user's email by current session.
///
/// # Returns
///
/// Will return an email as a string, if data was obtained successfully from 
/// the last login. Here, user service status is used for error codes only. 
pub async fn get_user() -> Result<String, UserServiceStatus> {
    let auth = FireAuth::new(FIREBASE_API_KEY.to_string());

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
