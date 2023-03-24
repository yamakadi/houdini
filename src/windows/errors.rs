use std::{
    error::Error,
    fmt,
    fmt::{Display, Formatter},
};
use windows::Win32::Foundation::HANDLE;

/// Lets you know why the executable couldn't be deleted.
///
/// If you encounter `HoudiniError::CouldNotDisposeFile` or possibly `HoudiniError::CouldNotAcquireHandle`,
/// you might be left with a difficult to delete executable.
#[derive(Debug)]
pub enum HoudiniError {
    CouldNotGetModuleName,
    CouldNotAcquireHandle,
    CouldNotRenameToStream,
    CouldNotDisposeFile,
    CouldNotCloseHandle(HANDLE),
    UnsupportedPlatform,
}

impl Error for HoudiniError {}

impl Display for HoudiniError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            HoudiniError::UnsupportedPlatform => write!(f, "this platform is not supported"),
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
