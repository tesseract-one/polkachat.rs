use std::mem::{ManuallyDrop, MaybeUninit};

use tesseract_swift::utils::{ptr::CAnyDropPtr, string::{CStringRef, CString}, traits::AsCRef, error::CError};

use crate::Result as PCResult;

#[repr(C)]
pub struct SUI {
    ptr: CAnyDropPtr,
    present_error: unsafe extern "C" fn(&SUI, CStringRef, *mut ManuallyDrop<CError>) -> bool,
}

impl SUI {
    pub (crate) fn present_error(&self, message: &str) -> PCResult<()> {
        let message: CString = message.into();
        let mut error = MaybeUninit::<ManuallyDrop<CError>>::uninit();
        let is_ok = unsafe {
            (self.present_error)(self, message.as_cref(), error.as_mut_ptr())
        };
        if is_ok { Ok(()) } else { 
            unsafe { Err(ManuallyDrop::into_inner(error.assume_init()).into()) }
        }
    }
}