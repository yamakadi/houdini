use std::{error::Error, fmt};

/// Lets you know why the executable couldn't be deleted.
///
/// There aren't any known side-effects if the delete operation fails on a unix system.
#[derive(Debug)]
pub enum HoudiniError {
    CouldNotGetExecutable(String),
    CouldNotUnlinkExecutable(String),
    UnsupportedPlatform,
}

impl Error for HoudiniError {}

impl fmt::Display for HoudiniError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            HoudiniError::UnsupportedPlatform => write!(f, "this platform is not supported"),
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
