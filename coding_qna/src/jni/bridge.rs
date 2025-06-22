
use jni::JNIEnv;
use jni::objects::{JClass, JIntArray, JObjectArray, JString};
use jni::sys::{jboolean, jchar, jint, jintArray, jobject, jobjectArray, jsize, jstring};

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

#[unsafe(no_mangle)]
pub unsafe extern "system" fn
Java_pp_example_codingqna_JniCall_findTheFirstNonRepeatedCharInString<'a>(
    mut env: JNIEnv<'a>,
    _jclass: JClass<'a>,
    input: JString
) -> jchar {
    let rust_str:String = env.get_string(&input)
        .unwrap()
        .into();
    let result = solutions::find_first_non_repeated_char_in_string(&rust_str);
    
    return result as u16;
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn
Java_pp_example_codingqna_JniCall_factorialUsingRecursion<'a>(
    _env: JNIEnv<'a>,
    _jclass: JClass<'a>,
    input: jint
) -> jint {
    let result = solutions::factorial_using_recursion(input);
    
    return result as jsize;
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn
Java_pp_example_codingqna_JniCall_findTheSecondLargestNumber<'a>(
    mut env: JNIEnv<'a>,
    _jclass: JClass<'a>,
    input: JIntArray
) -> jint {
    unsafe {
        let rust_arr: Vec<i32> = env.get_array_elements(&input, jni::objects::ReleaseMode::CopyBack)
        .unwrap()
        .to_vec();
        let result = solutions::find_the_second_largest(&rust_arr);
        
        return result as jsize;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn
Java_pp_example_codingqna_JniCall_sumOfDigitsInNumber<'a>(
    mut env: JNIEnv<'a>,
    _jclass: JClass<'a>,
    input: jint
) -> jint {
    unsafe {
        let result = solutions::sum_of_digits_in_number(input);
        
        return result as jsize;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn
Java_pp_example_codingqna_JniCall_findTheMissingNumberInArray<'a>(
    mut env: JNIEnv<'a>,
    _jclass: JClass<'a>,
    input: JIntArray
) -> jint {
    unsafe {
        let rust_arr: Vec<i32> = env.get_array_elements(&input, jni::objects::ReleaseMode::CopyBack)
        .unwrap()
        .to_vec();
        let result = solutions::find_the_missing_number_in_array(&rust_arr);
        
        return result;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn
Java_pp_example_codingqna_JniCall_checkIfTwoStringsAreAnagram<'a>(
    mut env: JNIEnv<'a>,
    _jclass: JClass<'a>,
    input1: JString,
    input2: JString
) -> jboolean {
    let rust_str1: String = env.get_string(&input1)
        .unwrap()
        .into();
    let rust_str2: String = env.get_string(&input2)
        .unwrap()
        .into();
    let result = solutions::check_if_two_strings_are_anagram(&rust_str1, &rust_str2);
    
    return u8::from(result); // equivalent to result.into()
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn
Java_pp_example_codingqna_JniCall_flattenAListOfIntegers<'a>(
    mut env: JNIEnv<'a>,
    _jclass: JClass<'a>,
    input: JObjectArray
) -> jintArray {
    unsafe {
        let mut rust_2d_arr: Vec<Vec<i32>> = vec![];
        let length = env.get_array_length(&input).unwrap();
        for i in 0 .. length {
            let obj: JIntArray = env.get_object_array_element(&input, i)
                .unwrap()
                .into();
            let int_vec = env.get_array_elements(&obj, jni::objects::ReleaseMode::CopyBack)
                .unwrap()
                .to_vec();
            rust_2d_arr.push(int_vec);
            env.delete_local_ref(obj).unwrap();
        }
        let result = solutions::flatten_a_list_of_integers(&rust_2d_arr);
        
        let ret = env.new_int_array(result.len() as i32).unwrap();
        env.set_int_array_region(&ret, 0, &result).unwrap();
        return ret.into_raw();
    }
}
