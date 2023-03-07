use std::mem::ManuallyDrop;
use std::sync::Arc;
use std::u32;

use futures::FutureExt;
use tesseract_client::transport;
use tesseract_utils::array::CArray;
use tesseract_utils::error::CError;
use tesseract_utils::future::CFuture;
use tesseract_utils::future_impls::CFutureString;
use tesseract_utils::panic::handle_exception_result;
use tesseract_utils::string::CString;
use tesseract_utils::{ptr::{CAnyRustPtr, IntoAnyPtr}, string::CStringRef, traits::TryAsRef, response::CResponse};

use crate::Core;
use crate::Error;

use super::ui::UI;
use super::ui::SUI;

pub type CCore = CAnyRustPtr;

impl IntoAnyPtr for Core {
}

#[no_mangle]
pub unsafe extern "C" fn polkachat_ccore_create(ui: SUI, ipc: transport::NativeTransport, ret: &mut ManuallyDrop<CCore>, err: &mut ManuallyDrop<CError>) -> bool {
    use tokio::runtime::Builder;

    tesseract_utils::tesseract_utils_init();

    handle_exception_result(|| {
        super::logger::init().map_err(|err| Into::<CError>::into(err))?;
        
        let runtime = Builder::new_multi_thread()
            .enable_all()
            .worker_threads(16)
            .build()
            .map_err(crate::Error::from)
            .map_err(|err| Into::<CError>::into(err))?;

        let core = Core::new(UI::new(ui) , runtime, |tesseract| {
            tesseract.transport(ipc)
        });

        let ptr = CAnyRustPtr::new(Arc::new(core));

        Ok(ptr)
    }).response(ret, err)
}

#[no_mangle]
pub unsafe extern "C" fn polkachat_ccore_account(ccore: ManuallyDrop<CCore>) -> ManuallyDrop<CFutureString> {
    ManuallyDrop::new(async move {
        let core = Arc::clone(ccore.as_ref::<Arc<Core>>()?);
        Ok(core.account_string().await?.into())
    }.map(|result| {
        result.map_err(|e: Error| e.into())
    }).into())
}

#[no_mangle]
pub unsafe extern "C" fn polkachat_ccore_messages(ccore: ManuallyDrop<CCore>, from: u32) -> ManuallyDrop<CFuture<CArray<CString>>> {
    ManuallyDrop::new(async move {
        let core = Arc::clone(ccore.as_ref::<Arc<Core>>()?);
        let messages = core.messages(from).await?;

        let messages: Vec<CString> = messages.into_iter().map(|message| {
            message.into()
        }).collect();

        Ok(messages.into())
    }.map(|result| {
        result.map_err(|e: Error| e.into())
    }).into())
}

#[no_mangle]
pub unsafe extern "C" fn polkachat_ccore_drop(ccore: &mut ManuallyDrop<CCore>) {
    let core = ManuallyDrop::take(ccore);
    drop(core)
}