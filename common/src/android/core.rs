use std::sync::Arc;

use jni::{
    objects::{JObject, JClass, JString, JList},
    JNIEnv, sys::jint
};
use jni_fn::jni_fn;

use crabdroid::{
    future::JCompletionStage,
    JavaDesc, JavaWrappableDesc, JavaWrappable, JavaErrorContext
};

use tesseract_android::client::transport::IPCTransport;

use crate::{
    Error, Core, UI
};

impl JavaDesc for Core {
    fn java_class<'a>(&'a self) -> &'a str {
        "one/tesseract/polkachat/rust/Core"
    }
}

impl JavaWrappableDesc for Core {
}

#[jni_fn("one.tesseract.polkachat.rust.Core")]
pub fn create<'a>(env: JNIEnv<'a>, _core_class: JClass<'a>, application: JObject<'a>, ui: JObject<'a>, loader: JObject<'a>) -> JObject<'a> {
    use tokio::runtime::Builder;
    use super::tokio::AndroidBuilder;

    Error::java_context(&env, || {
        let log_level = if cfg!(debug_assertions) {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Error
        };
        android_logger::init_once(
            android_logger::Config::default()
                .with_max_level(log_level)
                .with_tag("PolkaChat"),
        );
        
        log_panics::Config::new()
            .backtrace_mode(log_panics::BacktraceMode::Resolved)
            .install_panic_hook();

        let vm = env.get_java_vm()?;
        let loader = env.new_global_ref(loader)?;
        let runtime = Builder::new_multi_thread()
            .enable_all()
            .worker_threads(16)
            .jvm(vm, Some(loader))
            .build().map_err(Error::from)?;

        let ui = UI::from_java(&env, ui)?;
        let ipc = IPCTransport::new(&env, application);

        let core = Arc::new(Core::new(ui, runtime, |tesseract| {
            tesseract.transport(ipc)
        }));

        Ok(core.java_ref::<Core>(&env, None)?)
    })
}

#[jni_fn("one.tesseract.polkachat.rust.Core")]
pub fn account<'a>(env: JNIEnv<'a>, this: JObject<'a>) -> JObject<'a> { //CompletableFuture<String>
    Error::java_context(&env, || {
        let this = Core::from_java_ref(this, &env)?;

        JCompletionStage::launch_async(&env, async move |vm| {
            let account = this.account_string().await?;

            let env = vm.get_env()?;

            let account = env.new_string(&account)?;

            Ok(env.new_global_ref(account)?)
        })
    })
}

#[jni_fn("one.tesseract.polkachat.rust.Core")]
pub fn messages<'a>(env: JNIEnv<'a>, this: JObject<'a>, from: jint) -> JObject<'a> { //CompletableFuture<List<String>>
    use crabdroid::iter::ExactSizeIteratorJava;

    Error::java_context(&env, || {
        let this = Core::from_java_ref(this, &env)?;

        JCompletionStage::launch_async(&env, async move |vm| {
            let messages = this.messages(from as u32).await?;

            let env = vm.get_env()?;

            let messages = messages.into_iter().map(|message| {
                env.new_string(&message)
            });

            let class = env.find_class("java/lang/String")?;

            let list: JList = messages.try_collect_java(&env, class)?;

            Ok(env.new_global_ref(list)?)
        })
    })
}

#[jni_fn("one.tesseract.polkachat.rust.Core")]
pub fn send<'a>(env: JNIEnv<'a>, this: JObject<'a>, message: JString<'a>) -> JObject<'a> { //CompletableFuture<Unit>
    crate::Error::java_context(&env, || {
        let this = Core::from_java_ref(this, &env)?;
        let message = env.get_string(message)?.into();

        JCompletionStage::launch_async(&env, async move |vm| {
            this.send(message).await?;

            let env = vm.get_env()?;
            Ok(env.new_global_ref(JObject::null())?)
        })
    })
}
