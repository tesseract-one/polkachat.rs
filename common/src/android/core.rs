use std::sync::Arc;

use jni::{
    objects::{JObject, JClass, JString, JList},
    JNIEnv, sys::jint
};
use jni_fn::jni_fn;

use interop_android::{
    future::{FutureExtJava, IntoJava},
    JavaDesc, JavaWrappableDesc, JavaWrappable, deresultify
};

use tesseract_ipc_android::client::TransportIPCAndroid;

use crate::{
    error::Error,
    Core, UI
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

    deresultify(&env, || {
        android_logger::init_once(
            android_logger::Config::default()
                .with_max_level(log::LevelFilter::Trace)
                .with_tag("PolkaChat"),
        );
        log_panics::init();

        let vm = env.get_java_vm()?;
        let loader = env.new_global_ref(loader)?;
        let runtime = Builder::new_multi_thread()
            .enable_all()
            .worker_threads(16)
            .jvm(vm, Some(loader))
            .build()?;

        let ui = UI::from_java(&env, ui)?;
        let ipc = TransportIPCAndroid::new(&env, application);

        let core = Arc::new(Core::new(ui, runtime, |tesseract| {
            tesseract.transport(ipc)
        }));

        Ok(core.java_ref::<Core>(&env, None)?)
    })
}

#[jni_fn("one.tesseract.polkachat.rust.Core")]
pub fn account<'a>(env: JNIEnv<'a>, this: JObject<'a>) -> JObject<'a> {
    deresultify(&env, || {
        let this = Core::from_java_ref(this, &env)?;

        let account = this.account_string().map_ok_java(&env, |env, account| {
            Ok(env.new_string(&account)?.into())
        }).boxed_into_java(&env);

        Ok(account)
    })
}

#[jni_fn("one.tesseract.polkachat.rust.Core")]
pub fn messages<'a>(env: JNIEnv<'a>, this: JObject<'a>, from: jint) -> JObject<'a> {
    use interop_android::iter::ExactSizeIteratorJava;

    deresultify(&env, || {
        let this = Core::from_java_ref(this, &env)?;

        Ok(this.messages(from as u32).map_ok_java(&env, |env, messages| {
            let messages = messages.into_iter().map(|message| {
                env.new_string(&message).map_err(Error::from)
            });

            let class = env.find_class("java/lang/String")?;

            let list: JList = messages.try_collect_java(&env, class)?;

            Ok(list.into())
        }).boxed_into_java(&env))
    })
}

#[jni_fn("one.tesseract.polkachat.rust.Core")]
pub fn send<'a>(env: JNIEnv<'a>, this: JObject<'a>, message: JString<'a>) -> JObject<'a> {
    deresultify(&env, || {
        let this = Core::from_java_ref(this, &env)?;
        let message = env.get_string(message)?.into();

        Ok(this.send(message).map_ok_java(&env, |_, _| {
            Ok(JObject::null())
        }).boxed_into_java(&env))
    })
}
