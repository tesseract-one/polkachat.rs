use jni::{
    descriptors::Desc,
    errors::Result,
    objects::{JClass, JObject, JList},
    sys::{jsize, jobjectArray},
    JNIEnv
};

pub (super) trait EnvExt<'a> {
    fn new_object_array_from_iter<'c, C, T, I>(&self, element_class: C, iter: I) -> Result<jobjectArray>
    where
        C: Desc<'a, JClass<'c>>,
        T: Into<JObject<'a>>,
        I: ExactSizeIterator<Item=T>;

    unsafe fn new_list_from_array<'b>(&'b self, array: jobjectArray) -> Result<JList<'a, 'b>> where 'a: 'b;

    fn new_list_from_iter<'b, 'c, C, T, I>(&'b self, element_class: C, iter: I) -> Result<JList<'a, 'b>>
    where
        'a: 'b,
        C: Desc<'a, JClass<'c>>,
        T: Into<JObject<'a>>,
        I: ExactSizeIterator<Item=T>;
}

impl<'a> EnvExt<'a> for JNIEnv<'a> {
    fn new_object_array_from_iter<'c, C, T, I>(&self, element_class: C, iter: I) -> Result<jobjectArray>
    where
        C: Desc<'a, JClass<'c>>,
        T: Into<JObject<'a>>,
        I: ExactSizeIterator<Item=T> {
            let array = self.new_object_array(
                iter.len() as jsize,
                element_class,
                JObject::null(),
            )?;
            for (i, object) in iter.enumerate() {
                self.set_object_array_element(array, i as jsize, object)?;
            }
            Ok(array)
        }

    unsafe fn new_list_from_array<'b>(&'b self, array: jobjectArray) -> Result<JList<'a, 'b>> where 'a: 'b {
        let object = self.call_static_method(
            self.find_class("java/util/Arrays")?,
            "asList",
            "([Ljava/lang/Object;)Ljava/util/List;",
            &[JObject::from_raw(array).into()],
        )?
        .l()?;

        JList::from_env(&self, object)
    }

    fn new_list_from_iter<'b, 'c, C, T, I>(&'b self, element_class: C, iter: I) -> Result<JList<'a, 'b>>
    where
        'a: 'b,
        C: Desc<'a, JClass<'c>>,
        T: Into<JObject<'a>>,
        I: ExactSizeIterator<Item=T> {
            let array = self.new_object_array_from_iter(element_class, iter)?;
            unsafe {
                self.new_list_from_array(array)
            }
        }
}