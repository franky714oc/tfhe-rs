use crate::c_api::utils::*;
use std::os::raw::c_int;

pub struct U256(pub(in crate::c_api) crate::integer::U256);

impl_destroy_on_type!(U256);


// TODO move functions into the real u256 type

#[no_mangle]
pub unsafe extern "C" fn u256_from_words(input: *const u64, result: *mut *mut U256) -> c_int {
    catch_panic(|| {
        let low = (*input as u128) + ((*input.offset(1) as u128) << 64u128);
        let high = *input.offset(2) as u128 + ((*input.offset(3) as u128) << 64u128);

        let inner = crate::integer::U256::from((low, high));

        *result = Box::into_raw(Box::new(U256(inner)));
    })
}

#[no_mangle]
pub unsafe extern "C" fn u256_from_little_endian_bytes(
    input: *const u8,
    result: *mut *mut U256,
) -> c_int {
    catch_panic(|| {
        let mut inner = crate::integer::U256::default();

        let slc = inner.0.as_mut_slice();

        let u8_slc = unsafe { std::slice::from_raw_parts_mut(slc.as_mut_ptr() as *mut u8, 32) };

        let input = unsafe { std::slice::from_raw_parts(input, 32) };

        u8_slc.copy_from_slice(input);

        *result = Box::into_raw(Box::new(U256(inner)));
    })
}

#[no_mangle]
pub unsafe extern "C" fn u256_from_big_endian_bytes(
    input: *const u8,
    result: *mut *mut U256,
) -> c_int {
    catch_panic(|| {
        let mut inner = crate::integer::U256::default();

        let slc = inner.0.as_mut_slice();

        let u8_slc = unsafe { std::slice::from_raw_parts_mut(slc.as_mut_ptr() as *mut u8, 32) };

        let input = unsafe { std::slice::from_raw_parts(input, 32) };

        u8_slc.copy_from_slice(input);

        u8_slc.reverse();

        *result = Box::into_raw(Box::new(U256(inner)));
    })
}

/// `result` __MUST__ have at least 32 elements
#[no_mangle]
pub unsafe extern "C" fn u256_little_endian_bytes(input: *const U256, result: *mut u8) -> c_int {
    catch_panic(|| {
        check_ptr_is_non_null_and_aligned(result).unwrap();
        let input = get_ref_checked(input).unwrap();

        let bytes = std::slice::from_raw_parts_mut(result, 32);

        let u8_slc = unsafe { std::slice::from_raw_parts(input.0 .0.as_ptr() as *const u8, 32) };

        bytes.copy_from_slice(u8_slc)
    })
}

/// `result` __MUST__ have at least 32 elements
#[no_mangle]
pub unsafe extern "C" fn u256_big_endian_bytes(input: *const U256, result: *mut u8) -> c_int {
    catch_panic(|| {
        check_ptr_is_non_null_and_aligned(result).unwrap();
        let input = get_ref_checked(input).unwrap();

        let bytes = std::slice::from_raw_parts_mut(result, 32);

        let u8_slc = unsafe { std::slice::from_raw_parts(input.0 .0.as_ptr() as *const u8, 32) };

        bytes.copy_from_slice(u8_slc);
        bytes.reverse();
    })
}
