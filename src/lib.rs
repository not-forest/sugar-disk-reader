//! Main Rust backend library area for 'Sugar' application.
//!
//! As a main module for this library, it acts as a bridge between Java's frontend and
//! Rust's backend. It provides pre-defined methods to communication with backend functions,
//! which serve main role in parsing read files, transfering data and deploying connection.
//!
//! All low level communication mechanisms are implemented in this library with purpose of
//! complete abstraction from Java's mechanics and structures, except for this module. Rust
//! also handles interractions with firebase API and manages authentications.

/// Main library area.
///
/// This module contains all necessary sub-modules, which are the main building blocks of the
/// backend side.
pub mod sugar {
    /// Authentications handling.
    ///
    /// Provides a backend to authentication service mainly via firebase API.
    pub mod auth {
        /// User service.
        pub mod usrsrv;

        pub use usrsrv::service;
    }
    /// Application defined errors with status codes.
    pub mod errors;
    /// All external API related symbols.
    pub mod api;
    /// Custom parsing for user's input.
    pub mod parse;
    /// All storage related functions.
    pub mod storage;

    pub use api::FIREBASE_URI;
}

/// Main communication area.
///
/// This module provides all functions which can be used from Java frontend.
#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    use std::{env, mem};

    use super::*;

    use crate::sugar::auth::service::{login, signup};
    
    use jni::JNIEnv;
    use jni::objects::{JClass, JString};

    use log::LevelFilter;
    use android_logger::{Config, FilterBuilder};
    use sugar::auth::usrsrv::UserServiceStatus;

    // EXTERNS
    /// Initialization code from rust's side.
    #[no_mangle]
    pub extern fn Java_com_notforest_sugar_SugarInit_rustInit() {
        // Initializing logger.
        android_logger::init_once(
            Config::default()
                .with_max_level(LevelFilter::Trace)                     // limit log level
                .with_tag("RUST_BACKEND")                               
                .with_filter(                                           // configure messages for specific crate
                    FilterBuilder::new()
                        .parse("debug,hello::crate=error")
                        .build()
                )
        );
        log::debug!("Debug mode enabled");
        log::info!("OK");
    }

    /// Wrapper function to provide java's strings to rust signup interface.
    ///
    /// Will be called by Java's front-end, when user creates new 'Sugar' account.
    #[no_mangle]
    pub extern fn Java_com_notforest_sugar_SignupActivity_signUp(
        mut env: JNIEnv, 
        _: JClass, 
        java_mail: JString,
        java_pass: JString,
        java_conf: JString
    ) -> u8 {
        log::info!("Begin: signup");
        // Converting
        let mail = env.get_string(&java_mail).expect("Could not parse Java string.").into();
        let pass = env.get_string(&java_pass).expect("Could not parse Java string.").into();
        let conf = env.get_string(&java_conf).expect("Could not parse Java string.").into();

        signup(mail, pass, conf).into() 
    }

    /// Wrapper function to provide java's strings to rust login interface.
    ///
    /// Will be called by Java's front-end, when user creates new 'Sugar' account.
    #[no_mangle]
    pub extern fn Java_com_notforest_sugar_LoginActivity_login(
        mut env: JNIEnv,
        _: JClass,
        java_mail: JString,
        java_pass: JString,
    ) -> u8 {
        log::info!("Begin: login");
        // Converting
        let mail = env.get_string(&java_mail).expect("Could not parse Java string.").into();
        let pass = env.get_string(&java_pass).expect("Could not parse Java string.").into();
        
        login(mail, pass).into()
    }
}
