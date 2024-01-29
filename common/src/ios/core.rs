use std::{
    mem::ManuallyDrop,
    sync::Arc,
    u32
};

use errorcon::convertible::ErrorContext;

use tesseract_swift::client::transport::ClientTransport;

use tesseract_swift::utils::{
    Nothing,
    array::CArray,
    error::CError,
    future::CFuture,
    future_impls::{CFutureString, CFutureNothing},
    string::CString,
    response::CMoveResponse,
    ptr::{CAnyRustPtr, IntoAnyPtr}, 
};

use crate::{Core, Error};

use super::ui::{UI, SUI};

pub type CCore = CAnyRustPtr;

impl IntoAnyPtr for Core {}

#[no_mangle]
pub unsafe extern "C" fn polkachat_ccore_create(
    ui: SUI, ipc: ClientTransport,
    ret: &mut ManuallyDrop<CCore>, err: &mut ManuallyDrop<CError>
) -> bool {
    use tokio::runtime::Builder;

    Error::context(|| {
        super::logger::init()?;

        let runtime = Builder::new_multi_thread()
            .enable_all()
            .worker_threads(16)
            .build()?;

        let core = Core::new(UI::new(ui) , runtime, |tesseract| {
            tesseract.transport(ipc)
        });
    
        Ok(CAnyRustPtr::new(Arc::new(core)))
    }).response(ret, err)
}

#[no_mangle]
pub unsafe extern "C" fn polkachat_ccore_account(ccore: ManuallyDrop<CCore>) -> ManuallyDrop<CFutureString> {
    let core = ccore.as_ref::<Arc<Core>>().map(|arc| Arc::clone(arc));
    let tx = Error::context_async(async || {
        Ok(core?.account_string().await?.into())
    });

    ManuallyDrop::new(tx.into())
}

#[no_mangle]
pub unsafe extern "C" fn polkachat_ccore_messages(ccore: ManuallyDrop<CCore>, from: u32) -> ManuallyDrop<CFuture<CArray<CString>>> {
    let core = ccore.as_ref::<Arc<Core>>().map(|arc| Arc::clone(arc));

    let tx = Error::context_async(async move || {
        let messages = core?.messages(from).await?;

        let messages: Vec<CString> = messages.into_iter().map(|message| {
            message.into()
        }).collect();

        Ok(messages.into())
    });
    ManuallyDrop::new(tx.into())
}

#[no_mangle]
pub unsafe extern "C" fn polkachat_ccore_send(ccore: ManuallyDrop<CCore>, message: CString) -> ManuallyDrop<CFutureNothing> {
    let message = message.try_into();
    let core = ccore.as_ref::<Arc<Core>>().map(|arc| Arc::clone(arc));

    let tx = Error::context_async(async || {
        let _ = core?.send(message?).await?;

        Ok(Nothing::default())
    });

    ManuallyDrop::new(tx.into())
}

#[no_mangle]
pub unsafe extern "C" fn polkachat_ccore_drop(ccore: &mut ManuallyDrop<CCore>) {
    let core = ManuallyDrop::take(ccore);
    drop(core)
}