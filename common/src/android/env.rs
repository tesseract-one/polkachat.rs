use jni::{
    descriptors::Desc,
    errors::{
        Result as JResult,
        Error as JError
    },
    objects::{JClass, JObject, JList, AutoLocal},
    sys::{jsize, jobjectArray},
    JNIEnv
};

use std::result::Result;

pub (super) trait EnvExt<'a> {
    unsafe fn new_list_from_array<'b>(&'b self, array: jobjectArray) -> JResult<JList<'a, 'b>> where 'a: 'b;

    fn collect_iter_into_array<'c, C, T, I>(&self, element_class: C, iter: I) -> JResult<jobjectArray>
    where
        C: Desc<'a, JClass<'c>>,
        T: Into<JObject<'a>>,
        I: ExactSizeIterator<Item=T>;

    fn try_collect_iter_into_array<'c, C, T, E, I>(&self, element_class: C, iter: I) -> Result<jobjectArray, E>
    where
        C: Desc<'a, JClass<'c>>,
        T: Into<JObject<'a>>,
        E: From<JError>,
        I: ExactSizeIterator<Item=Result<T, E>>;

    fn collect_iter_into_list<'b, 'c, C, T, I>(&'b self, element_class: C, iter: I) -> JResult<JList<'a, 'b>>
    where
        'a: 'b,
        C: Desc<'a, JClass<'c>>,
        T: Into<JObject<'a>>,
        I: ExactSizeIterator<Item=T>;

    fn try_collect_iter_into_list<'b, 'c, C, T, E, I>(&'b self, element_class: C, iter: I) -> Result<JList<'a, 'b>, E>
        where
        'a: 'b,
        C: Desc<'a, JClass<'c>>,
        T: Into<JObject<'a>>,
        E: From<JError>,
        I: ExactSizeIterator<Item=Result<T, E>>;
}

impl<'a> EnvExt<'a> for JNIEnv<'a> {
    unsafe fn new_list_from_array<'b>(&'b self, array: jobjectArray) -> JResult<JList<'a, 'b>> where 'a: 'b {
        let object = self.call_static_method(
            self.find_class("java/util/Arrays")?,
            "asList",
            "([Ljava/lang/Object;)Ljava/util/List;",
            &[JObject::from_raw(array).into()],
        )?
        .l()?;

        JList::from_env(&self, object)
    }

    fn collect_iter_into_array<'c, C, T, I>(&self, element_class: C, iter: I) -> JResult<jobjectArray>
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
                let object = AutoLocal::new(self, object.into());
                self.set_object_array_element(array, i as jsize, object.as_obj())?;
            }
            Ok(array)
        }

    fn try_collect_iter_into_array<'c, C, T, E, I>(&self, element_class: C, iter: I) -> Result<jobjectArray, E>
    where
        C: Desc<'a, JClass<'c>>,
        T: Into<JObject<'a>>,
        E: From<JError>,
        I: ExactSizeIterator<Item=Result<T, E>> {
            let array = self.new_object_array(
                iter.len() as jsize,
                element_class,
                JObject::null(),
            )?;

            for (i, object) in iter.enumerate() {
                let object = AutoLocal::new(self, object?.into());
                self.set_object_array_element(array, i as jsize, object.as_obj())?;
            }
            Ok(array)
        }

    fn collect_iter_into_list<'b, 'c, C, T, I>(&'b self, element_class: C, iter: I) -> JResult<JList<'a, 'b>>
    where
        'a: 'b,
        C: Desc<'a, JClass<'c>>,
        T: Into<JObject<'a>>,
        I: ExactSizeIterator<Item=T> {
            let array = self.collect_iter_into_array(element_class, iter)?;
            unsafe {
                self.new_list_from_array(array)
            }
        }
    
    fn try_collect_iter_into_list<'b, 'c, C, T, E, I>(&'b self, element_class: C, iter: I) -> Result<JList<'a, 'b>, E>
    where
        'a: 'b,
        C: Desc<'a, JClass<'c>>,
        T: Into<JObject<'a>>,
        E: From<JError>,
        I: ExactSizeIterator<Item=Result<T, E>> {
            let array = self.try_collect_iter_into_array(element_class, iter)?;
            Ok(unsafe {
                self.new_list_from_array(array)
            }?)
        }
}