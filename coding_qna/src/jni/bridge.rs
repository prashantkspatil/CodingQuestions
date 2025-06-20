
use jni::JNIEnv;
use jni::objects::{JClass, JIntArray, JString};
use jni::sys::{jboolean, jint, jintArray, jsize, jstring};

use crate::solutions;

#[unsafe(no_mangle)]
pub extern "system" fn
Java_pp_example_codingqna_JniCall_reverse_1a_1string<'a>(
    mut env:JNIEnv<'a>,
    _jclass: JClass<'a>,
    input: JString
) -> jstring {

    let rust_string:String = env.get_string(&input)
        .expect("Couldn't get Java string")
        .into();

    let rev_string = solutions::reverse_a_string(&rust_string);
    
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
    let result = solutions::check_for_palindrome(&rust_str);
    
    return if result { 1 } else { 0 };
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_pp_example_codingqna_JniCall_find_1the_1maximum_1element_1in_1an_1array<'a>(
    mut env:JNIEnv<'a>,
    _jclass: JClass<'a>,
    input: JIntArray
) -> jint {
    unsafe {
        let rust_int_array = env.get_array_elements(&input, jni::objects::ReleaseMode::CopyBack)
            .expect("Failed to convert to rust string")
            .to_vec();
        let result = solutions::find_the_maximum_element_in_an_array(&rust_int_array);
    
        return result;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_pp_example_codingqna_JniCall_fizz_1buzz_1problem<'a>(
    env: JNIEnv<'a>,
    _jclass: JClass<'a>,
    input: jint
) -> jstring {
    let result: String = solutions::fizz_buzz(input as u32);
    return env.new_string(result).unwrap_or_default().into_raw();
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_pp_example_codingqna_JniCall_count_1vowels_1in_1a_1string<'a>(
    mut env: JNIEnv<'a>,
    _jclass: JClass<'a>,
    input: JString
) -> jint {
    let rut_str: String = env.get_string(&input).unwrap().into();
    let result:i32 = solutions::count_vowels_in_a_string(&rut_str);
    return result;
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn
Java_pp_example_codingqna_JniCall_remove_1duplicates_1from_1the_1list<'a>(
    mut env: JNIEnv<'a>,
    _jclass: JClass<'a>,
    input: JIntArray
) -> jintArray {
    unsafe {
        let rust_vec:Vec<i32> = env.get_array_elements(&input, jni::objects::ReleaseMode::CopyBack)
            .unwrap().to_vec();
        let mut result:Vec<i32> = solutions::remove_duplicates_from_the_list(&rust_vec);
        result.sort();
        let j_int_array = env.new_int_array(result.len() as jsize)
            .expect("Filed to create java int array");
        
        env.set_int_array_region(&j_int_array, 0, &result)
            .expect("failed to copy");

        return j_int_array.into_raw();
    }
}