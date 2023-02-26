use std::sync::Arc;

use futures::FutureExt;

use jni::{
    objects::{JObject, JClass, JString},
    JNIEnv
};
use jni_fn::jni_fn;

use interop_android::{
    future::into_java::FutureJava,
    JavaDesc, JavaWrappableDesc, JavaWrappable, deresultify
};

use tesseract_ipc_android::client::TransportIPCAndroid;

use crate::{
    error::Error,
    Core
};

impl JavaDesc for Core {
    fn java_class<'a>(&'a self) -> &'a str {
        "one/tesseract/polkachat/rust/Core"
    }
}

impl JavaWrappableDesc for Core {
}

#[jni_fn("one.tesseract.polkachat.rust.Core")]
pub fn create<'a>(env: JNIEnv<'a>, _core_class: JClass<'a>, application: JObject<'a>, loader: JObject<'a>/*, ui: JObject<'a>*/) -> JObject<'a> {
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
            .jvm(vm, Some(loader))
            .build()?;

        //let ui = UI::from_java(&env, ui)?;
        let ipc = TransportIPCAndroid::new(&env, application);

        let core = Arc::new(Core::new(runtime, |tesseract| {
            tesseract.transport(ipc)
        }));

        Ok(core.java_ref::<Core>(&env, None)?)
    })
}

#[jni_fn("one.tesseract.polkachat.rust.Core")]
pub fn account<'a>(env: JNIEnv<'a>, this: JObject<'a>) -> JObject<'a> {
    deresultify(&env, || {
        let vm = env.get_java_vm()?;
        let this = Core::from_java_ref(this, &env)?;
        let account = this.account_string().map(move |account| {
            let env = vm.get_env()?;
            account.and_then(|pubkey| {
                env.new_string(&pubkey).and_then(|jstr| {
                    env.new_global_ref(jstr)
                }).map_err(Error::from)
            })
        }).into_java(&env);

        Ok(account)
    })
}

#[jni_fn("one.tesseract.polkachat.rust.Core")]
pub fn messages<'a>(env: JNIEnv<'a>, this: JObject<'a>) -> JObject<'a> {
    use super::env::EnvExt;

    deresultify(&env, || {
        let this = Core::from_java_ref(this, &env)?;
        let vm = env.get_java_vm()?;

        Ok(this.messages().map(move |messages| {
            let env = vm.get_env()?;
            messages.and_then(|messages| {
                let messages = messages.into_iter().map(|message| {
                    env.new_string(&message).map_err(Error::from)
                });

                let class = env.find_class("java/lang/String")?;

                let list = env.try_collect_iter_into_list(class, messages)?;
                let list = env.new_global_ref(list)?;

                Ok(list)
            })
        }).into_java(&env))
    })
}

#[jni_fn("one.tesseract.polkachat.rust.Core")]
pub fn send<'a>(env: JNIEnv<'a>, this: JObject<'a>, message: JString<'a>) -> JObject<'a> {
    deresultify(&env, || {
        let this = Core::from_java_ref(this, &env)?;
        let message = env.get_string(message)?.into();
        let vm = env.get_java_vm()?;

        Ok(this.send(message).map(move |result| {
            let env = vm.get_env()?;
            result.and_then (|_| {
                Ok(env.new_global_ref(JObject::null())?)
            })
        }).into_java(&env))
    })
}
