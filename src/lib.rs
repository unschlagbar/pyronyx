use core::ffi::c_char;
use core::fmt::Debug;
use core::ptr;

mod extensions;
pub mod impl_command_buffer;
pub mod impl_device;
pub mod impl_instance;
pub mod impl_physical_device;
pub mod impl_queue;
pub mod macros;
pub mod video;
pub mod vk;
pub mod vtables;

pub use extensions::*;

use crate::vk::{PFN_vkVoidFunction, vkInstance};

unsafe extern "system" {
    pub unsafe fn vkGetInstanceProcAddr(
        instance: vkInstance,
        name: *const c_char,
    ) -> PFN_vkVoidFunction;
}

pub(crate) fn raw_option<T>(value: Option<&T>) -> *const T {
    match value {
        Some(inner) => inner,
        _ => ptr::null(),
    }
}

#[allow(unused)]
#[inline]
pub(crate) fn read_into_vec_result<N: Copy + Default + TryInto<usize>, T>(
    f: impl Fn(&mut N, *mut T) -> vk::vkResult,
) -> Result<Vec<T>, vk::vkResult>
where
    <N as TryInto<usize>>::Error: Debug,
{
    loop {
        let mut count = N::default();
        f(&mut count, ptr::null_mut()).result()?;
        let mut data = Vec::with_capacity(count.try_into().unwrap());

        let err_code = f(&mut count, data.as_mut_ptr());
        if !matches!(err_code, vk::vkResult::Incomplete) {
            return unsafe { err_code.set_len_on_success(data, count.try_into().unwrap()) };
        }
    }
}

#[allow(unused)]
#[inline]
pub(crate) fn read_into_vec<N: Copy + Default + TryInto<usize>, T>(
    f: impl Fn(&mut N, *mut T),
) -> Vec<T>
where
    <N as TryInto<usize>>::Error: Debug,
{
    let mut count = N::default();
    f(&mut count, ptr::null_mut());
    let mut data = Vec::with_capacity(count.try_into().unwrap());

    f(&mut count, data.as_mut_ptr());
    unsafe { data.set_len(count.try_into().unwrap()) };
    data
}
