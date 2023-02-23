use std::sync::Arc;

use futures::FutureExt;
use jni::objects::{JObject, JClass};
use jni::JNIEnv;
use jni_fn::jni_fn;

use interop_android::deresultify;
use interop_android::{JavaDesc, JavaWrappableDesc, JavaWrappable};
use interop_android::future::into_java::FutureJava;

use tesseract_ipc_android::client::TransportIPCAndroid;

use crate::error::Error;
use crate::Core;

impl JavaDesc for Core {
    fn java_class<'a>(&'a self) -> &'a str {
        "one/tesseract/polkachat/rust/Core"
    }
}

impl JavaWrappableDesc for Core {
}

#[jni_fn("one.tesseract.polkachat.rust.Core")]
pub fn create<'a>(env: JNIEnv<'a>, _core_class: JClass<'a>, application: JObject<'a>/*, ui: JObject<'a>*/) -> JObject<'a> {
    deresultify(&env, || {
        android_log::init("PolkaChat")?;

        //let ui = UI::from_java(&env, ui)?;
        let ipc = TransportIPCAndroid::new(&env, application);

        let core = Arc::new(Core::new(|tesseract| {
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
        let account = this.account().map(move |account| {
            let env = vm.get_env()?;
            account.map(|account| {
                //account.public_key
                //let pk: sr25519::Public = account.public_key.as_slice().try_into().unwrap();
                //let sss:String = pk.to_ss58check();
                "thisiskeyplaceholderconvertlater".to_owned()
            }).and_then(|pubkey| {
                env.new_string(&pubkey).and_then(|jstr| {
                    env.new_global_ref(jstr)
                }).map_err(Error::from)
            })
        }).into_java(&env);

        Ok(account)
    })
}