use std::mem::ManuallyDrop;

use tesseract_utils::{ptr::CAnyDropPtr, string::{CStringRef, CString}, result::CResult, Nothing};

use crate::Result as PCResult;

#[repr(C)]
pub struct SUI {
    ptr: CAnyDropPtr,
    present_error: unsafe extern "C" fn(&SUI, CStringRef) -> ManuallyDrop<CResult<Nothing>>,
}

impl SUI {
    pub (crate) fn present_error(&self, message: &str) -> PCResult<()> {
        let message: CString = message.into();
        unsafe {
            let cresult = ManuallyDrop::into_inner(
                (self.present_error)(self, message.as_ptr()));
            let result: Result<(), _> = cresult.into();
            Ok(result?)
        }
    }
}