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
        pub mod profile;

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
    use std::path::Path;

    use super::*;

    
    use jni::JNIEnv;
    use jni::objects::{JClass, JString};
    use jni::sys::jstring;

    use log::LevelFilter;
    use android_logger::{Config, FilterBuilder};
    use sugar::auth::profile::{change_mail, change_pass};
    use sugar::auth::service::{fast_login, login, logout, signup};
    use sugar::auth::usrsrv::UserServiceStatus;
    use sugar::storage::{FILES_DIR, CACHE_DIR, EXT_FILES_DIR, EXT_CACHE_DIR};

    // EXTERNS
    /// Initialization code from rust's side.
    #[no_mangle]
    pub extern fn Java_com_notforest_sugar_SugarInit_rustInit(
        mut env: JNIEnv, 
        _: JClass, 
        files_dir: JString,
        cache_dir: JString,
        ext_files_dir: JString,
        ext_cache_dir: JString,
    ) {
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
        log::info!("Logger initialized");

        // Converting
        let files_dir: String = env.get_string(&files_dir).expect("Could not parse Java string.").into();
        let cache_dir: String = env.get_string(&cache_dir).expect("Could not parse Java string.").into();
        let ext_files_dir: String = env.get_string(&ext_files_dir).expect("Could not parse Java string.").into();
        let ext_cache_dir: String = env.get_string(&ext_cache_dir).expect("Could not parse Java string.").into();
 
        log::info!("Files directory at: {}", files_dir);
        log::info!("Cache directory at: {}", cache_dir);
        log::info!("External files directory at: {}", ext_files_dir);
        log::info!("External cache directory at: {}", ext_cache_dir);

        // Getting info about directories from the environment.
        FILES_DIR.write().unwrap().push(Path::new(files_dir.as_str()));
        CACHE_DIR.write().unwrap().push(Path::new(cache_dir.as_str()));
        EXT_FILES_DIR.write().unwrap().push(Path::new(ext_files_dir.as_str()));
        EXT_CACHE_DIR.write().unwrap().push(Path::new(ext_cache_dir.as_str()));

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

    /// Fast login method by current token credentials.
    ///
    /// Will be called by Java's front-end, when user creates new 'Sugar' account.
    #[no_mangle]
    pub extern fn Java_com_notforest_sugar_LoginActivity_loginFast(
        mut env: JNIEnv,
        _: JClass
    ) -> jstring {
        log::info!("Begin: login");

        let st = if let Some(s) = fast_login() { s } else { "".to_string() };
        env.new_string(st).expect("Unable to create new Java string from environment.")
            .into_raw()
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

    /// Wrapper function for logging out.
    ///
    /// Will be called by Java's front-end, when user creates new 'Sugar' account.
    #[no_mangle]
    pub extern fn Java_com_notforest_sugar_ui_profile_ProfileFragment_logout() -> u8 {
        log::info!("Begin: logout");
        logout().into()
    }

    #[no_mangle]
    pub extern fn Java_com_notforest_sugar_ui_profile_ProfileFragment_changeEmail(
        mut env: JNIEnv,
        _: JClass,
        java_mail: JString,
    ) -> u8 { 
        log::info!("Begin: change email address");
        // Converting
        let mail = env.get_string(&java_mail).expect("Could not parse Java string.").into();

        change_mail(mail).into()
    }

    #[no_mangle]
    pub extern fn Java_com_notforest_sugar_ui_profile_ProfileFragment_changePassword(
        mut env: JNIEnv,
        _: JClass,
        pass_old: JString,
        pass_new: JString,
    ) -> u8 {
        log::info!("Begin: change password");
        // Converting
        //let old = env.get_string(&pass_old).expect("Could not parse Java string.").into();
        let new = env.get_string(&pass_new).expect("Could not parse Java string.").into();

        change_pass(new).into()
    }
}
