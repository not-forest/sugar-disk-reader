//! Implementation of local storage.
//!
//! This module contains various structures to read and write data from different kind of user's storage.
//! It is primeraly used to save temporary and global data within the local storage of the phone.
//! However it also provide an interface to Java's front-end for communicating with cloud storage.

use lazy_static::lazy_static;
use std::{fs::File, io::ErrorKind, path::{Path, PathBuf}, sync::RwLock};
use serde::{de::DeserializeOwned, Serialize};
use super::errors::StorageError; 

// Will be set during initialization phase. 
lazy_static! {
    /// Local sugar application directory.
    pub static ref FILES_DIR: RwLock<Box<PathBuf>> = RwLock::new(Box::new(PathBuf::new()));
    /// Local sugar cache directory.
    pub static ref CACHE_DIR: RwLock<Box<PathBuf>> = RwLock::new(Box::new(PathBuf::new()));
    /// External files directory.
    pub static ref EXT_FILES_DIR: RwLock<Box<PathBuf>> = RwLock::new(Box::new(PathBuf::new()));
    /// External cache directory.
    pub static ref EXT_CACHE_DIR: RwLock<Box<PathBuf>> = RwLock::new(Box::new(PathBuf::new()));
}

/// Custom structure that provides a local interface with data written on phone's disk.
pub struct LocalStorage;

impl LocalStorage {
    /// Writes any data that can be represented as JSON to the local storage.
    ///
    /// If write fails, will return one of pre defined errors that match it's result. If everything
    /// will go accordingly, will return the amount of bytes written to file.
    pub fn write(data: &impl Serialize, dest: &'static str) -> Result<usize, StorageError> {
        let dest = Path::new(
            FILES_DIR.read().unwrap().as_ref()
        ).join(dest.to_owned()).with_extension("json");

        let out = {
            if let Ok(data) = serde_json::to_string(data).as_ref() {
                let length = data.len();
                log::info!("Writing {} bytes to local storage: {}", length, dest.to_string_lossy());

                if length == 0 {
                    return Err(StorageError::NO_DATA)
                };

                match File::create(dest) {
                    Ok(buf_writer) => match serde_json::to_writer(buf_writer, data) {
                        Ok(_) => Ok(length),
                        Err(_) => Err(StorageError::SERIALIZATION_ERROR),
                    },
                    Err(err) => match err.kind() {
                        ErrorKind::TimedOut => Err(StorageError::TIME_OUT),
                        ErrorKind::InvalidData => Err(StorageError::BAD_DATA),
                        ErrorKind::Interrupted => Err(StorageError::INTERRUPTED),
                        ErrorKind::OutOfMemory => Err(StorageError::OUT_OF_MEMORY),
                        _ => unreachable!(),
                    },
                }
            } else {
                Err(StorageError::SERIALIZATION_ERROR)
            }
        };

        if let Err(ref err) = out {
            log::error!("Local storage WRITE error: {}", err);
        }

        out
    }

    /// Reads the data from internal storage.
    ///
    /// If write fails, will return one of pre defined errors that match it's result. If everything
    /// will go accordingly, will return a deserialized version of the data. 
    pub fn read<T: DeserializeOwned>(dest: &'static str) -> Result<T, StorageError> { 
        let dest = Path::new(
            FILES_DIR.read().unwrap().as_ref()
        ).join(dest.to_owned()).with_extension("json");
        
        let out = {
            log::info!("Reading data from local storage: {}", dest.to_string_lossy());
            match File::open(dest) {
                Ok(buf_reader) => match serde_json::from_reader(buf_reader) {
                    Ok(data) => Ok(data),
                    Err(_) => Err(StorageError::SERIALIZATION_ERROR),
                },
                Err(err) => match err.kind() {
                    ErrorKind::NotFound => Err(StorageError::FILE_NOT_EXIST),
                    ErrorKind::TimedOut => Err(StorageError::TIME_OUT),
                    ErrorKind::InvalidData => Err(StorageError::BAD_DATA),
                    ErrorKind::Interrupted => Err(StorageError::INTERRUPTED),
                    ErrorKind::OutOfMemory => Err(StorageError::OUT_OF_MEMORY),
                    _ => unreachable!(),
                },
            }
        };

        if let Err(ref err) = out {
            log::error!("Loal storage READ error: {}", err);
        }

        out
    }
}
