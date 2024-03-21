//! Main Rust backend library area for 'Sugar' application.
//!
//! As a main module for this library, it acts as a channel between Java's frontend and
//! Rust's backend. It provides pre-defined methods to communication with backend functions,
//! which serve main role in parsing read files, transfering data and deploying connection.
//!
//! All low level communication mechanisms are implemented in this library with purpose of
//! complete abstraction from Java's mechanics and structures, except for this module.

/// Main communication area.
///
/// This module provides all functions which can be used from Java frontend.
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use super::*;
    use self::jni::JNIEnv;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::jstring;

    // EXTERNS

}
