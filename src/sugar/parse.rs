//! Custom module for parsing user's input.
//!
//! This module parses user's input from login, singup, configurations and data from
//! target computer.

use super::auth::errors::DataParseError;
use regex::Regex;
use once_cell::sync::Lazy;

/// Struct which handles all parsing activity related to user input and data.
///
/// This struct is ZST, therefore it will be optimized by the compiler to only provide linking to
/// all it's related functions.
pub struct SugarParser;

impl SugarParser {
    /// Parses provided user's mail.
    pub(crate) fn parse_mail(mail: String) -> Result<(), DataParseError> {
        // This static regex is checking email for correctness.
        static REG: Lazy<Regex> = Lazy::new(|| 
            Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
        );

        if !REG.is_match(&mail) {
            return Err(DataParseError::MailRegexError)
        }

        log::debug!("OK");
        Ok(())
    }

    /// Parses provided user's password.
    pub(crate) fn parse_pass(pass: String) -> Result<(), DataParseError> {
        // This static regex is checking if password conditions are met.
        //
        // # Conditions:
        // - At least 4 characters long.
        // - At least one special character.
        static REG: Lazy<Regex> = Lazy::new(|| 
            Regex::new(r"^(?=.*[!@#$%^&*()_+{}|:<>`\-=[\];,./])(.{4,})$").unwrap()
        );

        if !REG.is_match(&pass) {
            return Err(DataParseError::PasswordConditionsError)
        }

        log::debug!("OK");
        Ok(())
    }
}
