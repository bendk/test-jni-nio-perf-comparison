use std::{io::{Cursor, Read}, sync::OnceLock};

use jni::{JNIEnv, objects::{JByteBuffer, JFieldID, JClass, JObject}, signature::{ReturnType, Primitive}};

#[derive(Clone)]
pub struct TheStruct {
    first: i32,
    second: f64,
}

// A "stack", that we manage ourselves and use to pass structs across the FFI
//
// Note: I could only get a leaked vec to work, a static array causes a segfault (maybe because the
// memory is flagged read-only?).
static STACK_BUFFER: OnceLock<&'static [u8]> = OnceLock::new();

static STRUCT_FIELD_IDS: OnceLock<(JFieldID, JFieldID)> = OnceLock::new();

#[allow(clippy::missing_safety_doc)]
#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_dev_gobley_test_jninioperfcomparison_RustLibrary_initJni(
    mut env: JNIEnv,
    _class: JClass,
) {
    println!("{}", std::mem::offset_of!(TheStruct, first));
    println!("{}", std::mem::offset_of!(TheStruct, second));
    let class = env.find_class("dev/gobley/test/jninioperfcomparison/TheStruct").unwrap();
    let _ = STRUCT_FIELD_IDS.set((
        env.get_field_id(&class, "first", "I").unwrap(),
        env.get_field_id(&class, "second", "D").unwrap(),
    ));
}

#[allow(clippy::missing_safety_doc)]
#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_dev_gobley_test_jninioperfcomparison_RustLibrary_testUsingJni(
    mut env: JNIEnv,
    _class: JClass,
    a: f64,
    b: i32,
    c: f64,
    d: i32,
) -> f64 {
    a.powi(b) + c.powi(d)
}

#[allow(clippy::missing_safety_doc)]
#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_dev_gobley_test_jninioperfcomparison_RustLibrary_getStackBuffer(
    mut env: JNIEnv,
    _class: JClass,
) -> jni::sys::jobject {
    let buf: Vec<u8> = vec![0; 1024];
    let leaked_slice = buf.leak();
    STACK_BUFFER.set(leaked_slice).unwrap();
    let buf = unsafe {
        env.new_direct_byte_buffer(leaked_slice.as_ptr().cast_mut(), 1024).unwrap()
    };
    buf.into_raw()
}

#[allow(clippy::missing_safety_doc)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_dev_gobley_test_jninioperfcomparison_RustLibrary_testUsingNio(
    _env: JNIEnv,
    _class: JClass,
) {
    let buffer: &[u8] = STACK_BUFFER.get().unwrap();
    let pos = i64::from_ne_bytes(buffer[0..8].try_into().unwrap()) as usize;
    let buffer = &buffer[pos..];
    let a = f64::from_ne_bytes(buffer[0..8].try_into().unwrap());
    let b = i32::from_ne_bytes(buffer[8..12].try_into().unwrap());
    let c = f64::from_ne_bytes(buffer[16..24].try_into().unwrap());
    let d = i32::from_ne_bytes(buffer[24..28].try_into().unwrap());
    let result = a.powi(b) + c.powi(d);
    unsafe {
        *(std::mem::transmute::<_, &mut[u8; 8]>(buffer.as_ptr().cast_mut())) = result.to_ne_bytes()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn testUsingJnaNio() {
    let buffer: &[u8] = STACK_BUFFER.get().unwrap();
    let pos = i64::from_ne_bytes(buffer[0..8].try_into().unwrap()) as usize;
    let buffer = &buffer[pos..];
    let a = f64::from_ne_bytes(buffer[0..8].try_into().unwrap());
    let b = i32::from_ne_bytes(buffer[8..12].try_into().unwrap());
    let c = f64::from_ne_bytes(buffer[16..24].try_into().unwrap());
    let d = i32::from_ne_bytes(buffer[24..28].try_into().unwrap());
    let result = a.powi(b) + c.powi(d);
    unsafe {
        *(std::mem::transmute::<_, &mut[u8; 8]>(buffer.as_ptr().cast_mut())) = result.to_ne_bytes()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn jnaNioGetStackPointer() -> *mut () {
    let buf: Vec<u8> = vec![0; 1024];
    let leaked_slice = buf.leak();
    STACK_BUFFER.set(leaked_slice).unwrap();
    leaked_slice.as_ptr().cast_mut().cast()
}

