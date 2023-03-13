macro_rules! impl_destroy_on_type {
    ($wrapper_type:ty) => {
        ::paste::paste! {
            #[no_mangle]
            #[doc = "ptr can be null (no-op in that case)"]
            pub unsafe extern "C" fn [<$wrapper_type:snake _destroy>](
                ptr: *mut $wrapper_type,
            ) -> c_int {
                crate::c_api::utils::catch_panic(|| {
                    if (!ptr.is_null()) {
                        crate::c_api::utils::check_ptr_is_non_null_and_aligned(ptr).unwrap();
                        drop(Box::from_raw(ptr));
                    }
                })
            }
        }
    };
}
