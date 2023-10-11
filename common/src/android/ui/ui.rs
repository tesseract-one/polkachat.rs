use async_trait::async_trait;

use jni::{JNIEnv, objects::JObject, errors::Result as JResult};

use crabdroid::ContextedGlobal;

use crate::ui::UIProtocol;
use crate::error::Result as PCResult;

use super::jui::JUI;

pub (crate) struct UI {
    internal: ContextedGlobal
}

impl UI {
    pub (in crate::android) fn from_java<'a: 'b, 'b>(env: &'b JNIEnv<'a>, ui: JObject<'a>) -> JResult<Self> {
        ContextedGlobal::from_local(env, ui).map(|ui| {
            UI {internal: ui}
        })
    }
}

#[async_trait]
impl UIProtocol for UI {
    fn present_error(&self, message: &str) -> PCResult<()> {
        Ok(self.internal.with_safe_context_rret(64, |env, ui| {
            let message = env.new_string(message)?;
            let jui = JUI::from_env(&env, ui);

            Ok(jui.present_error(message)?)
        })?)
    }
}