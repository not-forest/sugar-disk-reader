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

        pub use usrsrv::{UserP, UserServiceError, service};
    }
    /// All external API related symbols.
    pub mod api;

    pub use api::FIREBASE_URI;
}

/// Main communication area.
///
/// This module provides all functions which can be used from Java frontend.
#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    use std::env;

    use super::*;

    use crate::sugar::auth::service::{login, signup};
    
    use jni::JNIEnv;
    use jni::objects::{JClass, JString};
    use jni::sys::jstring;

    use log::LevelFilter;
    use android_logger::{Config, FilterBuilder};

    // EXTERNS
    /// Initialization code from rust's side.
    #[no_mangle]
    pub extern fn Java_com_notforest_sugar_SugarInit_rustInit() {
        // Initializing logger.
        android_logger::init_once(
            Config::default()
                .with_max_level(LevelFilter::Trace)                     // limit log level
                .with_tag("RUST_BACKEND")                               // logs will show under mytag tag
                .with_filter(                                           // configure messages for specific crate
                    FilterBuilder::new()
                        .parse("debug,hello::crate=error")
                        .build()
                )
        );
    }

    /// Wrapper function to provide java's strings to rust interface.
    ///
    /// Will be called by Java's front-end, when user creates new 'Sugar' account.
    #[no_mangle]
    pub extern fn Java_com_notforest_sugar_SignupActivity_signUp(
        mut env: JNIEnv, 
        _: JClass, 
        java_mail: JString,
        java_pass: JString,
        java_conf: JString
    ) {
        let mail = env.get_string(&java_mail).expect("Could not parse Java string.").into();
        let pass = env.get_string(&java_pass).expect("Could not parse Java string.").into();
        let conf = env.get_string(&java_conf).expect("Could not parse Java string.").into();

        signup(mail, pass, conf); 
    }

    #[no_mangle]
    pub extern fn Java_com_notforest_sugar_LoginActivity_login(
        mut env: JNIEnv,
        _: JClass,
        java_mail: JString,
        java_pass: JString,
    ) {
        let mail = env.get_string(&java_mail).expect("Could not parse Java string.").into();
        let pass = env.get_string(&java_pass).expect("Could not parse Java string.").into();
        
        login(mail, pass);
    }
}
