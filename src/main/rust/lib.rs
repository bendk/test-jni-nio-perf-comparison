use std::{io::{Cursor, Read}, sync::OnceLock};

use jni::{JNIEnv, objects::{JByteBuffer, JFieldID, JClass, JObject}, signature::{ReturnType, Primitive}};

pub struct TheStruct {
    first: i32,
    second: f64,
}


#[allow(clippy::missing_safety_doc)]
#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_dev_gobley_test_jninioperfcomparison_RustLibrary_testUsingJni(
    _env: JNIEnv,
    _class: JClass,
    a_first: i32,
    a_second: f64,
    b_first: i32,
    b_second: f64,
) -> f64 {
    calculate_result_from_structs(&[
        TheStruct { first: a_first, second: a_second },
        TheStruct { first: b_first, second: b_second },
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
    calculate_result_from_structs(&[
        TheStruct {
            first: i32::from_le_bytes(buffer[0..4].try_into().unwrap()),
            second: f64::from_le_bytes(buffer[8..16].try_into().unwrap()),
        },
        TheStruct {
            first: i32::from_le_bytes(buffer[16..20].try_into().unwrap()),
            second: f64::from_le_bytes(buffer[24..32].try_into().unwrap()),
        },
    ])
}

fn calculate_result_from_structs(structs: &[TheStruct]) -> f64 {
    structs.iter().map(|s| s.second.powi(s.first)).sum()
}
