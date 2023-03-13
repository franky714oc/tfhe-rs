use crate::c_api::utils::*;
use std::os::raw::c_int;

pub struct ConcreteClientKey(pub(crate) crate::typed_api::ClientKey);
pub struct ConcretePublicKey(pub(crate) crate::typed_api::PublicKey);
pub struct ConcreteServerKey(pub(crate) crate::typed_api::ServerKey);

impl_destroy_on_type!(ConcreteClientKey);
impl_destroy_on_type!(ConcretePublicKey);
impl_destroy_on_type!(ConcreteServerKey);

#[no_mangle]
pub unsafe extern "C" fn concrete_generate_keys(
    config: *mut super::config::Config,
    result_client_key: *mut *mut ConcreteClientKey,
    result_server_key: *mut *mut ConcreteServerKey,
) -> c_int {
    catch_panic(|| {
        check_ptr_is_non_null_and_aligned(result_client_key).unwrap();
        check_ptr_is_non_null_and_aligned(result_server_key).unwrap();

        *result_client_key = std::ptr::null_mut();
        *result_server_key = std::ptr::null_mut();

        let config = Box::from_raw(config);

        let (cks, sks) = crate::typed_api::generate_keys(config.0);

        *result_client_key = Box::into_raw(Box::new(ConcreteClientKey(cks)));
        *result_server_key = Box::into_raw(Box::new(ConcreteServerKey(sks)));
    })
}

#[no_mangle]
pub unsafe extern "C" fn concrete_set_server_key(server_key: *const ConcreteServerKey) -> c_int {
    catch_panic(|| {
        let server_key = get_ref_checked(server_key).unwrap();

        let cloned = server_key.0.clone();
        crate::typed_api::set_server_key(cloned);
    })
}

/// result can be null
#[no_mangle]
pub unsafe extern "C" fn concrete_unset_server_key(result: *mut *mut ConcreteServerKey) -> c_int {
    catch_panic(|| {
        let previous_key = crate::typed_api::unset_server_key();

        if !result.is_null() {
            *result = Box::into_raw(Box::new(ConcreteServerKey(previous_key)))
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn concrete_public_key_new(
    client_key: *const ConcreteClientKey,
    result_public_key: *mut *mut ConcretePublicKey,
) -> c_int {
    catch_panic(|| {
        let client_key = get_ref_checked(client_key).unwrap();
        let inner = crate::typed_api::PublicKey::new(&client_key.0);

        *result_public_key = Box::into_raw(Box::new(ConcretePublicKey(inner)));
    })
}
