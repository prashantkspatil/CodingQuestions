
use jni::JNIEnv;
use jni::objects::{JClass, JIntArray, JString};
use jni::sys::{jboolean, jint, jstring};

use crate::functions;

#[unsafe(no_mangle)]
pub extern "system" fn Java_pp_example_codingqna_JniCall_reverse_1a_1string<'a>(
    mut env:JNIEnv<'a>,
    _jclass: JClass<'a>,
    input: JString
) -> jstring {

    let rust_string:String = env.get_string(&input)
        .expect("Couldn't get Java string")
        .into();

    let rev_string = functions::reverse_a_string(&rust_string);
    
    return env.new_string(rev_string).expect("Couldn't create Java string").into_raw();
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_pp_example_codingqna_JniCall_check_1for_1palindrome<'a>(
    mut env:JNIEnv<'a>,
    _jclass: JClass<'a>,
    input:JString
) -> jboolean {
    let rust_str: String = env.get_string(&input)
        .expect("Failed to convert to rust string")
        .into();
    let result = functions::check_for_palindrome(&rust_str);
    
    return if result { 1 } else { 0 };
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_pp_example_codingqna_JniCall_find_1the_1maximum_1element_1in_1an_1array<'a>(
    mut env:JNIEnv<'a>,
    _jclass: JClass<'a>,
    input: JIntArray
) -> jint {
    let rust_int_array = env.get_array_elements(&input, jni::objects::ReleaseMode::CopyBack)
        .expect("Failed to convert to rust string")
        .to_vec();
    let result = functions::find_the_maximum_element_in_an_array(&rust_int_array);
    
    return result;
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_pp_example_codingqna_JniCall_fizz_1buzz_1problem<'a>(
    mut env: JNIEnv<'a>,
    _jclass: JClass<'a>,
    input: jint
) -> jstring {
    let result: String = functions::fizz_buzz(input as u32);
    return env.new_string(result).unwrap_or_default().into_raw();
}