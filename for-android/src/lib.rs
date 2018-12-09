use jni::JNIEnv;
use jni::objects::{JClass};
use jni::sys::jstring;

#[no_mangle]
pub extern "system" fn Java_com_example_wushu_for_1rust_MainActivity_stringFromJNI(env_ :JNIEnv,obj_:JClass) -> jstring  {
    let hello = env_.new_string("Hello from Rust")
        .expect("Couldn't create java string!");

    hello.into_inner()
}
