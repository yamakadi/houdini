use std::{error::Error, fmt};

#[cfg(target_os = "windows")]
use windows::Win32::Foundation::HANDLE;

impl Error for HoudiniError {}

#[cfg(not(target_os = "windows"))]
#[derive(Debug)]
pub enum HoudiniError {
    CouldNotGetExecutable(String),
    CouldNotUnlinkExecutable(String),
}

impl fmt::Display for HoudiniError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            HoudiniError::CouldNotGetExecutable(error) => {
                write!(f, "failed to get the executable path with error: {}", error)
            },
            HoudiniError::CouldNotUnlinkExecutable(error) => {
                write!(
                    f,
                    "failed to unlink the executable for the current process with error: {}",
                    error
                )
            },
        }
    }
}

#[cfg(target_os = "windows")]
#[derive(Debug, Clone)]
pub enum HoudiniError {
    CouldNotGetModuleName,
    CouldNotAcquireHandle,
    CouldNotRenameToStream,
    CouldNotDisposeFile,
    CouldNotCloseHandle(HANDLE),
}

#[cfg(target_os = "windows")]
impl fmt::Display for HoudiniError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HoudiniError::CouldNotGetModuleName => write!(f, "failed to get the module name"),
            HoudiniError::CouldNotAcquireHandle => {
                write!(f, "failed to acquire handle for the current process")
            },
            HoudiniError::CouldNotRenameToStream => write!(f, "failed to rename to stream"),
            HoudiniError::CouldNotDisposeFile => write!(f, "failed to dispose file"),
            HoudiniError::CouldNotCloseHandle(handle) => {
                write!(f, "could not close handle: {:?}", handle)
            },
        }
    }
}
