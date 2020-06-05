use crate::error::{LaszipError, Result};

/// Loads the laszip library
pub fn load_laszip_library() {
    let result = unsafe { laszip_sys::laszip_load_dll() };
    if result != 0 {
        let err = unsafe { libc::dlerror() } as *const _;
        if err != std::ptr::null() {
            let err_msg = unsafe { std::ffi::CStr::from_ptr(err) };
            panic!("Error while loading laszip_sys: {:?}", err_msg);
        }
    }
}

///
pub fn create_laszip() -> laszip_sys::laszip_POINTER {
    let mut reader: laszip_sys::laszip_POINTER = std::ptr::null_mut();
    assert_eq!(0, unsafe { laszip_sys::laszip_create(&mut reader) });

    reader
}

///
pub trait ErrorHandler {
    fn handle_error(&self, res: laszip_sys::laszip_I32) -> Result<()>;
}

impl ErrorHandler for laszip_sys::laszip_POINTER {
    fn handle_error(&self, res: laszip_sys::laszip_I32) -> Result<()> {
        if res == 0 {
            return Ok(());
        }

        if self.is_null() {
            Err(LaszipError {
                error: "Attempt to get laszip error from null pointer".to_string(),
            })
        } else {
            let mut err: *mut std::os::raw::c_char = std::ptr::null_mut();
            let res = unsafe { laszip_sys::laszip_get_error(*self, &mut err as *mut *mut _) };

            if res == 0 {
                Err(LaszipError {
                    error: "Unknown error in laszip".to_string(),
                })
            } else {
                let c_string = unsafe { std::ffi::CStr::from_ptr(err) };
                Err(LaszipError {
                    error: c_string.to_string_lossy().into_owned(),
                })
            }
        }
    }
}
