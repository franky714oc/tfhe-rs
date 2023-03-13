pub struct ConfigBuilder(pub(in crate::c_api) crate::typed_api::ConfigBuilder);
pub struct Config(pub(in crate::c_api) crate::typed_api::Config);

use crate::c_api::utils::*;
use std::os::raw::c_int;

impl_destroy_on_type!(ConfigBuilder);
impl_destroy_on_type!(Config);

#[no_mangle]
pub unsafe extern "C" fn config_builder_all_disabled(result: *mut *mut ConfigBuilder) -> c_int {
    catch_panic(|| {
        check_ptr_is_non_null_and_aligned(result).unwrap();

        let inner_builder = crate::typed_api::ConfigBuilder::all_disabled();

        *result = Box::into_raw(Box::new(ConfigBuilder(inner_builder)));
    })
}

#[no_mangle]
pub unsafe extern "C" fn config_builder_clone(
    input: *const ConfigBuilder,
    result: *mut *mut ConfigBuilder,
) -> c_int {
    catch_panic(|| {
        check_ptr_is_non_null_and_aligned(result).unwrap();

        let cloned = get_ref_checked(input).unwrap().0.clone();

        *result = Box::into_raw(Box::new(ConfigBuilder(cloned)));
    })
}

#[no_mangle]
pub unsafe extern "C" fn config_builder_enable_default_bool(
    builder: *mut *mut ConfigBuilder,
) -> c_int {
    catch_panic(|| {
        check_ptr_is_non_null_and_aligned(builder).unwrap();

        *builder = Box::into_raw(Box::new(ConfigBuilder(
            Box::from_raw(*builder).0.enable_default_bool(),
        )));
    })
}

#[no_mangle]
pub unsafe extern "C" fn config_builder_enable_default_uint8(
    builder: *mut *mut ConfigBuilder,
) -> c_int {
    catch_panic(|| {
        check_ptr_is_non_null_and_aligned(builder).unwrap();

        *builder = Box::into_raw(Box::new(ConfigBuilder(
            Box::from_raw(*builder).0.enable_default_uint8(),
        )));
    })
}

#[no_mangle]
pub unsafe extern "C" fn config_builder_enable_default_uint256(
    builder: *mut *mut ConfigBuilder,
) -> c_int {
    catch_panic(|| {
        check_ptr_is_non_null_and_aligned(builder).unwrap();

        *builder = Box::into_raw(Box::new(ConfigBuilder(
            Box::from_raw(*builder).0.enable_default_uint256(),
        )));
    })
}

#[no_mangle]
pub unsafe extern "C" fn config_builder_build(
    builder: *mut ConfigBuilder,
    result: *mut *mut Config,
) -> c_int {
    catch_panic(|| {
        check_ptr_is_non_null_and_aligned(result).unwrap();

        let config = Box::from_raw(builder).0.build();

        *result = Box::into_raw(Box::new(Config(config)));
    })
}
