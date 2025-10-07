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
    struct1: JObject,
    struct2: JObject,
    struct3: JObject,
    struct4: JObject,
) -> f64 {
    let field_ids = STRUCT_FIELD_IDS.get().unwrap();
    calculate_result_from_structs(&[
        TheStruct {
            first: env.get_field_unchecked(
                &struct1, 
                field_ids.0,
                ReturnType::Primitive(Primitive::Int),
            ).unwrap().i().unwrap(),
            second: env.get_field_unchecked(
                &struct1, 
                field_ids.1,
                ReturnType::Primitive(Primitive::Double),
            ).unwrap().d().unwrap(),
        },
        TheStruct {
            first: env.get_field_unchecked(
                &struct2, 
                field_ids.0,
                ReturnType::Primitive(Primitive::Int),
            ).unwrap().i().unwrap(),
            second: env.get_field_unchecked(
                &struct2, 
                field_ids.1,
                ReturnType::Primitive(Primitive::Double),
            ).unwrap().d().unwrap(),
        },
        TheStruct {
            first: env.get_field_unchecked(
                &struct3, 
                field_ids.0,
                ReturnType::Primitive(Primitive::Int),
            ).unwrap().i().unwrap(),
            second: env.get_field_unchecked(
                &struct3, 
                field_ids.1,
                ReturnType::Primitive(Primitive::Double),
            ).unwrap().d().unwrap(),
        },
        TheStruct {
            first: env.get_field_unchecked(
                &struct4, 
                field_ids.0,
                ReturnType::Primitive(Primitive::Int),
            ).unwrap().i().unwrap(),
            second: env.get_field_unchecked(
                &struct4, 
                field_ids.1,
                ReturnType::Primitive(Primitive::Double),
            ).unwrap().d().unwrap(),
        },
    ])
}

#[allow(clippy::missing_safety_doc)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_dev_gobley_test_jninioperfcomparison_RustLibrary_testUsingNio(
    env: JNIEnv,
    _class: JClass,
    structs: JByteBuffer,
) -> f64 {
    let buffer: &[u8] = unsafe {
        let buffer_address = env.get_direct_buffer_address(&structs).unwrap();
        let buffer_capacity = env.get_direct_buffer_capacity(&structs).unwrap();
        std::slice::from_raw_parts(buffer_address, buffer_capacity)
    };
    unsafe {
        calculate_result_from_structs(&[
            std::mem::transmute::<&[u8; 16], &TheStruct>(&buffer[0..16].try_into().unwrap()).clone(),
            std::mem::transmute::<&[u8; 16], &TheStruct>(&buffer[16..32].try_into().unwrap()).clone(),
            std::mem::transmute::<&[u8; 16], &TheStruct>(&buffer[32..48].try_into().unwrap()).clone(),
            std::mem::transmute::<&[u8; 16], &TheStruct>(&buffer[48..64].try_into().unwrap()).clone(),
        ])
    }
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
pub unsafe extern "C" fn Java_dev_gobley_test_jninioperfcomparison_RustLibrary_testUsingNio2(
    _env: JNIEnv,
    _class: JClass,
) -> f64 {
    let buffer: &[u8] = STACK_BUFFER.get().unwrap();
    let pos = i64::from_ne_bytes(buffer[0..8].try_into().unwrap()) as usize;
    let buffer = &buffer[pos..];
    calculate_result_from_structs(&[
        TheStruct {
            first: i32::from_ne_bytes(buffer[0..4].try_into().unwrap()),
            second: f64::from_ne_bytes(buffer[8..16].try_into().unwrap()),
        },
        TheStruct {
            first: i32::from_ne_bytes(buffer[16..20].try_into().unwrap()),
            second: f64::from_ne_bytes(buffer[24..32].try_into().unwrap()),
        },
        TheStruct {
            first: i32::from_ne_bytes(buffer[32..36].try_into().unwrap()),
            second: f64::from_ne_bytes(buffer[40..48].try_into().unwrap()),
        },
        TheStruct {
            first: i32::from_ne_bytes(buffer[48..52].try_into().unwrap()),
            second: f64::from_ne_bytes(buffer[56..64].try_into().unwrap()),
        },
    ])
}

fn calculate_result_from_structs(structs: &[TheStruct]) -> f64 {
    structs.iter().map(|s| s.second.powi(s.first)).sum()
}
