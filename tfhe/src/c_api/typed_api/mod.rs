use crate::c_api::typed_api::u256::U256;
use crate::c_api::utils::*;
use crate::typed_api::prelude::*;
use std::os::raw::c_int;

#[macro_use]
mod utils;
pub mod config;
pub mod keys;
pub mod u256;

pub struct FheBool(crate::typed_api::FheBool);
pub struct FheUint8(crate::typed_api::FheUint8);
pub struct FheUint256(crate::typed_api::FheUint256);

use keys::{ConcreteClientKey, ConcretePublicKey};

impl_destroy_on_type!(FheBool);
impl_destroy_on_type!(FheUint8);
impl_destroy_on_type!(FheUint256);

macro_rules! impl_try_encrypt_with_client_key_on_type {
    ($wrapper_type:ty{$wrapped_type:ty}, $input_type:ty) => {
        ::paste::paste! {
            #[no_mangle]
            pub unsafe extern "C" fn  [<$wrapper_type:snake _try_encrypt_with_client_key_ $input_type:snake>](
                value: $input_type,
                client_key: *const ConcreteClientKey,
                result: *mut *mut $wrapper_type,
            ) -> c_int {
                catch_panic(|| {
                    let client_key = get_ref_checked(client_key).unwrap();

                    let inner = <$wrapped_type>::try_encrypt(value, &client_key.0).unwrap();

                    *result = Box::into_raw(Box::new($wrapper_type(inner)));
                })
            }
        }
    };
}

macro_rules! impl_try_encrypt_with_public_key_on_type {
    ($wrapper_type:ty{$wrapped_type:ty}, $input_type:ty) => {
        ::paste::paste! {
            #[no_mangle]
            pub unsafe extern "C" fn  [<$wrapper_type:snake _try_encrypt_with_public_key>](
                value: $input_type,
                public_key: *const ConcretePublicKey,
                result: *mut *mut $wrapper_type,
            ) -> c_int {
                catch_panic(|| {
                    let public_key = get_ref_checked(public_key).unwrap();

                    let inner = <$wrapped_type>::try_encrypt(value, &public_key.0).unwrap();

                    *result = Box::into_raw(Box::new($wrapper_type(inner)));
                })
            }
        }
    };
}

impl_try_encrypt_with_client_key_on_type!(FheBool{crate::typed_api::FheBool}, bool);
impl_try_encrypt_with_client_key_on_type!(FheUint8{crate::typed_api::FheUint8}, u8);

impl_try_encrypt_with_public_key_on_type!(FheBool{crate::typed_api::FheBool}, bool);
impl_try_encrypt_with_public_key_on_type!(FheUint8{crate::typed_api::FheUint8}, u8);

#[no_mangle]
pub unsafe extern "C" fn fhe_uint256_try_encrypt_with_client_key_u256(
    value: *const U256,
    client_key: *const ConcreteClientKey,
    result: *mut *mut FheUint256,
) -> c_int {
    catch_panic(|| {
        let client_key = get_ref_checked(client_key).unwrap();

        let inner = <crate::typed_api::FheUint256>::try_encrypt((*value).0, &client_key.0).unwrap();

        *result = Box::into_raw(Box::new(FheUint256(inner)));
    })
}

#[no_mangle]
pub unsafe extern "C" fn fhe_uint256_try_encrypt_with_public_key_u256(
    value: *const U256,
    public_key: *const ConcretePublicKey,
    result: *mut *mut FheUint256,
) -> c_int {
    catch_panic(|| {
        let public_key = get_ref_checked(public_key).unwrap();

        let inner = <crate::typed_api::FheUint256>::try_encrypt((*value).0, &public_key.0).unwrap();

        *result = Box::into_raw(Box::new(FheUint256(inner)));
    })
}

macro_rules! impl_decrypt_on_type {
    ($wrapper_type:ty{$wrapped_type:ty}, $output_type:ty) => {
        ::paste::paste! {
            #[no_mangle]
            pub unsafe extern "C" fn  [<$wrapper_type:snake _decrypt>](
                encrypted_value: *const $wrapper_type,
                client_key: *const ConcreteClientKey,
                result: *mut $output_type,
            ) -> c_int {
                catch_panic(|| {
                    let client_key = get_ref_checked(client_key).unwrap();
                    let encrypted_value = get_ref_checked(encrypted_value).unwrap();

                    *result = encrypted_value.0.decrypt(&client_key.0);
                })
            }
        }
    };
}

impl_decrypt_on_type!(FheBool{crate::typed_api::FheBool}, bool);
impl_decrypt_on_type!(FheUint8{crate::typed_api::FheUint8}, u8);

#[no_mangle]
pub unsafe extern "C" fn fhe_uint256_decrypt(
    encrypted_value: *const FheUint256,
    client_key: *const ConcreteClientKey,
    result: *mut *mut U256,
) -> c_int {
    catch_panic(|| {
        let client_key = get_ref_checked(client_key).unwrap();
        let encrypted_value = get_ref_checked(encrypted_value).unwrap();

        let inner: crate::integer::U256 = encrypted_value.0.decrypt(&client_key.0);
        *result = Box::into_raw(Box::new(U256(inner)));
    })
}

use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul, MulAssign,
    Sub, SubAssign,
};

macro_rules! impl_binary_fn_on_type {
    ($wrapper_type:ty => $($binary_fn_name:ident),* $(,)?) => {
        $(
            ::paste::paste! {
                #[no_mangle]
                pub unsafe extern "C" fn [<$wrapper_type:snake _ $binary_fn_name>](
                    lhs: *const $wrapper_type,
                    rhs: *const $wrapper_type,
                    result: *mut *mut $wrapper_type,
                ) -> c_int {
                    catch_panic(|| {
                        let lhs = get_ref_checked(lhs).unwrap();
                        let rhs = get_ref_checked(rhs).unwrap();

                        let inner = (&lhs.0).$binary_fn_name(&rhs.0);

                        *result = Box::into_raw(Box::new($wrapper_type(inner)));
                    })
                }
            }
        )*
    };
}

// Meant for types on which makes use of interior mutability
macro_rules! impl_binary_fn_on_type_mut {
    ($wrapper_type:ty => $($binary_fn_name:ident),* $(,)?) => {
        $(
           ::paste::paste! {
                #[no_mangle]
                pub unsafe extern "C" fn [<$wrapper_type:snake _ $binary_fn_name>](
                    lhs: *mut $wrapper_type,
                    rhs: *mut $wrapper_type,
                    result: *mut *mut $wrapper_type,
                ) -> c_int {
                    catch_panic(|| {
                        let lhs = get_mut_checked(lhs).unwrap();
                        let rhs = get_mut_checked(rhs).unwrap();

                        let inner = (&lhs.0).$binary_fn_name(&rhs.0);

                        *result = Box::into_raw(Box::new($wrapper_type(inner)));
                    })
                }
            }
        )*
    };
}

// Meant for types on which makes use of interior mutability
macro_rules! impl_binary_assign_fn_on_type_mut {
    ($wrapper_type:ty => $($binary_assign_fn_name:ident),* $(,)?) => {
        $(
           ::paste::paste! {
                #[no_mangle]
                pub unsafe extern "C" fn [<$wrapper_type:snake _ $binary_assign_fn_name>](
                    lhs: *mut $wrapper_type,
                    rhs: *mut $wrapper_type,
                ) -> c_int {
                    catch_panic(|| {
                        let lhs = get_mut_checked(lhs).unwrap();
                        let rhs = get_mut_checked(rhs).unwrap();

                        lhs.0.$binary_assign_fn_name(&rhs.0);
                    })
                }
            }
        )*
    };
}

impl_binary_fn_on_type!(FheBool => bitand, bitor, bitxor);

impl_binary_fn_on_type_mut!(FheUint8 => add, sub, mul, bitand, bitor, bitxor, eq, ge, gt, le, lt, min, max);
impl_binary_fn_on_type_mut!(FheUint256 => add, sub, mul, bitand, bitor, bitxor, eq, ge, gt, le, lt, min, max);
impl_binary_assign_fn_on_type_mut!(FheUint8 => add_assign, sub_assign, mul_assign, bitand_assign, bitor_assign, bitxor_assign);
impl_binary_assign_fn_on_type_mut!(FheUint256 => add_assign, sub_assign, mul_assign, bitand_assign, bitor_assign, bitxor_assign);
