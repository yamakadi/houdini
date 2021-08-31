//! I document the current module!

pub(crate) mod errors;

use std::{
    ffi::OsStr,
    mem::size_of,
    os::windows::ffi::OsStrExt,
    ptr::{copy, null_mut},
};

use winapi::{
    ctypes::{c_int, c_void},
    shared::{
        minwindef::{BOOL, DWORD, FALSE, HMODULE, LPVOID, MAX_PATH},
        ntdef::{HANDLE, NULL, TRUE, WCHAR},
    },
    um::{
        fileapi::{
            CreateFileW, SetFileInformationByHandle, FILE_DISPOSITION_INFO, FILE_RENAME_INFO,
            OPEN_EXISTING,
        },
        handleapi::{CloseHandle, INVALID_HANDLE_VALUE},
        libloaderapi::GetModuleFileNameW,
        minwinbase::{FileDispositionInfo, FileRenameInfo, LPSECURITY_ATTRIBUTES},
        winnt::{DELETE, FILE_ATTRIBUTE_NORMAL},
    },
};

use errors::HoudiniError;

pub(crate) const DEFAULT_PLACEHOLDER: &[u8; 9] = b":svcmsrpc";

pub(crate) fn disappear(placeholder: &[u8; 9]) -> Result<(), HoudiniError> {
    let filename = get_filename()?;

    let handle = open(filename.clone().as_str())?;

    #[cfg(feature = "debug")]
    println!("[*] Attempting to rename file to stream");
    rename(placeholder, handle)?;

    #[cfg(feature = "debug")]
    println!("[*] Successfully renamed file primary :$DATA ADS to specified stream, closing initial handle");
    close(handle)?;

    let handle = open(filename.clone().as_str())?;

    dispose(handle)?;

    #[cfg(feature = "debug")]
    println!("[*] Closing handle to trigger deletion deposition");
    close(handle)?;

    Ok(())
}

fn open(path: &str) -> Result<HANDLE, HoudiniError> {
    let os_path: Vec<u16> = OsStr::new(path)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect::<Vec<_>>();

    let handle: HANDLE = unsafe {
        CreateFileW(
            os_path.as_ptr(),
            DELETE,
            0,
            null_mut() as LPSECURITY_ATTRIBUTES,
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            0 as HANDLE,
        )
    };

    #[cfg(feature = "debug")]
    println!("[*] Acquired handle: {:?}", handle);

    if handle == INVALID_HANDLE_VALUE {
        return Err(HoudiniError::CouldNotAcquireHandle);
    }

    return Ok(handle);
}

fn rename(placeholder: &[u8; 9], handle: HANDLE) -> Result<(), HoudiniError> {
    let filename = placeholder.map(|b| b as WCHAR);
    let length = size_of::<[WCHAR; 9]>();

    let mut file_rename_info: FILE_RENAME_INFO = FILE_RENAME_INFO {
        ReplaceIfExists: FALSE,
        RootDirectory: NULL,
        FileNameLength: length as DWORD,
        FileName: [0],
    };

    unsafe {
        copy(
            filename.as_ptr(),
            file_rename_info.FileName.as_mut_ptr(),
            length,
        )
    };

    let buffer_size = size_of::<[WCHAR; 9]>() + size_of::<FILE_RENAME_INFO>();

    let file_rename_info_ptr: *mut c_void =
        unsafe { std::mem::transmute::<&mut FILE_RENAME_INFO, LPVOID>(&mut file_rename_info) };

    let status: BOOL = unsafe {
        SetFileInformationByHandle(
            handle,
            FileRenameInfo,
            file_rename_info_ptr,
            buffer_size as DWORD,
        )
    };

    if status as c_int == 1 {
        Ok(())
    } else {
        Err(HoudiniError::CouldNotRenameToStream)
    }
}

fn dispose(handle: HANDLE) -> Result<(), HoudiniError> {
    let mut file_delete: FILE_DISPOSITION_INFO = FILE_DISPOSITION_INFO { DeleteFile: TRUE };

    let file_delete_ptr: *mut c_void =
        unsafe { std::mem::transmute::<&mut FILE_DISPOSITION_INFO, LPVOID>(&mut file_delete) };

    let status: BOOL = unsafe {
        SetFileInformationByHandle(
            handle,
            FileDispositionInfo,
            file_delete_ptr,
            size_of::<FILE_DISPOSITION_INFO>() as DWORD,
        )
    };

    if status as c_int == 1 {
        Ok(())
    } else {
        Err(HoudiniError::CouldNotDisposeFile)
    }
}

fn close(handle: HANDLE) -> Result<(), HoudiniError> {
    let status = unsafe { CloseHandle(handle) };

    if status as c_int == 1 {
        Ok(())
    } else {
        Err(HoudiniError::CouldNotCloseHandle(handle))
    }
}

fn get_filename() -> Result<String, HoudiniError> {
    let mut filename_buffer = [0u16; MAX_PATH];

    let filename_length = unsafe {
        GetModuleFileNameW(
            0 as HMODULE,
            filename_buffer.as_mut_ptr(),
            MAX_PATH as DWORD,
        )
    };

    let len = filename_buffer.iter().take_while(|&&c| c != 0).count();
    let filename = String::from_utf16_lossy(&filename_buffer[..len]);

    #[cfg(feature = "debug")]
    {
        println!("[*] Filename: {:?}", filename.clone());
        println!("[*] Filename length: {:?}", filename_length);
    }

    if filename.is_empty() {
        Err(HoudiniError::CouldNotGetModuleName)
    } else {
        Ok(filename)
    }
}
