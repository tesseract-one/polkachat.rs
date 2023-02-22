mod core;

use jni::objects::{JObject, JClass, JString};
use jni::JNIEnv;
use jni_fn::jni_fn;

#[jni_fn("one.tesseract.polkachat.MainActivity")]
pub fn test<'a>(env: JNIEnv<'a>, this: JObject<'a>) {
    //deresultify(&env, || {
        android_log::init("PolkaChat").unwrap();
        debug!("It's alive!!!")
    //})
}