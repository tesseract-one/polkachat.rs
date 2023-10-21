use std::mem::ManuallyDrop;

use tesseract_swift_utils::{array::CArray, string::CString};

#[no_mangle]
pub unsafe extern "C" fn polkachat_carray_cstring_drop(this: &mut ManuallyDrop<CArray<ManuallyDrop<CString>>>) {
    let _ = ManuallyDrop::take(this);
}