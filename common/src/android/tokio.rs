use tokio::runtime::Builder;

use jni::objects::GlobalRef;
use jni::{JNIEnv, JavaVM};

use crabdroid::env::AndroidEnv;

pub trait AndroidBuilder {
    fn jvm(&mut self, vm: JavaVM, class_loader: Option<GlobalRef>) -> &mut Self;
}

impl AndroidBuilder for Builder {
    fn jvm(&mut self, vm: JavaVM, class_loader: Option<GlobalRef>) -> &mut Self {
        self.on_thread_start(move || {
            let _ = vm
                .attach_current_thread_permanently()
                .expect("Can't attach thread to VM in Tokio pool");

            debug!("TOKIO POOL EXECUTOR THREAD IS INITIALIZED WITH JAVA");

            if let Some(loader) = class_loader.clone() {
                JNIEnv::set_thread_class_loader(loader).unwrap();
                debug!("TOKIO POOL EXECUTOR THREAD IS SET WITH CLASS LOADER");
            }

            debug!("THREAD POOL EXECUTOR THREAD INITIALIZATION FINISHED WELL");
        })
    }
}