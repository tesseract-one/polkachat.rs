//need this for the sake of proper headers generation
pub use tesseract_utils::*;
pub use tesseract_client::*;

use std::sync::atomic::{AtomicBool, Ordering};

static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub (super) fn init() {
    use stderrlog::LogLevelNum;

    let level = if cfg!(debug_assertions) {
        LogLevelNum::Debug
        //LogLevelNum::Trace
    } else {
        LogLevelNum::Error
    };

    if !INITIALIZED.load(Ordering::Relaxed) {
        stderrlog::new().verbosity(level).show_module_names(true).init().unwrap();
        INITIALIZED.store(true, Ordering::Relaxed)
    }
}

#[no_mangle]
pub unsafe extern "C" fn polkachat_app_test() {
    init();

    debug!("It's alive!!!");
}