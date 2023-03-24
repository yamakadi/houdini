pub(crate) mod errors;

use errors::HoudiniError;
use std::{env::current_exe, fs::remove_file};

pub fn disappear() -> Result<(), HoudiniError> {
    #[cfg(feature = "debug")]
    println!("[*] Getting current executable");
    let filename = match current_exe() {
        Ok(p) => p,
        Err(e) => return Err(HoudiniError::CouldNotGetExecutable(e.to_string())),
    };

    #[cfg(feature = "debug")]
    println!("    > {:?}", filename.clone().into_os_string());

    #[cfg(feature = "debug")]
    println!("[*] Attempting to unlink file");
    return match remove_file(filename) {
        Ok(_) => Ok(()),
        Err(e) => Err(HoudiniError::CouldNotUnlinkExecutable(e.to_string())),
    };
}
