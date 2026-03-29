mod bitflags;
mod commands;
mod constants;
mod display_bitflags;
mod enums;
mod handles;
mod platform_types;
mod types;

use core::ffi::CStr;
use core::mem::{MaybeUninit, transmute};
use core::{fmt, ptr};

pub use bitflags::*;
pub(crate) use commands::*;
pub use constants::*;
pub use enums::*;
pub use handles::*;
pub use platform_types::*;
pub use types::*;

use crate::utils::read_into_vec_result;
use crate::vtables::to_option;

pub use crate::vkGetInstanceProcAddr as get_instance_proc_addr;

pub fn enumerate_instance_layer_properties() -> Result<Vec<LayerProperties>, vkResult> {
    let pfn: vkEnumerateInstanceLayerProperties = to_option(unsafe {
        transmute(get_instance_proc_addr(
            vkInstance::null(),
            c"vkEnumerateInstanceLayerProperties".as_ptr(),
        ))
    })
    .unwrap();
    read_into_vec_result(|count, data| unsafe { (pfn)(count, data) })
}

pub fn enumerate_instance_extension_properties(
    layer_name: Option<&CStr>,
) -> Result<Vec<ExtensionProperties>, vkResult> {
    let pfn: vkEnumerateInstanceExtensionProperties = to_option(unsafe {
        transmute(get_instance_proc_addr(
            vkInstance::null(),
            c"vkEnumerateInstanceExtensionProperties".as_ptr(),
        ))
    })
    .unwrap();
    read_into_vec_result(|count, data| unsafe {
        (pfn)(
            layer_name.map_or(ptr::null(), |str| str.as_ptr()),
            count,
            data,
        )
    })
}

impl vkResult {
    #[inline]
    pub fn init_on_success<T>(&self, v: MaybeUninit<T>) -> Result<T, vkResult> {
        match self {
            Self::Success => Ok(unsafe { v.assume_init() }),
            _ => Err(*self),
        }
    }

    #[inline]
    pub fn result(&self) -> Result<(), vkResult> {
        match self {
            Self::Success => Ok(()),
            _ => Err(*self),
        }
    }

    #[inline]
    pub unsafe fn set_len_on_success<T>(
        self,
        mut v: Vec<T>,
        len: usize,
    ) -> Result<Vec<T>, vkResult> {
        match self {
            Self::Success => {
                unsafe { v.set_len(len) };
                Ok(v)
            }
            _ => Err(self),
        }
    }
}

impl fmt::Display for vkResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            vkResult::Success => "Success",
            vkResult::NotReady => "NotReady",
            vkResult::Timeout => "Timeout",
            vkResult::EventSet => "EventSet",
            vkResult::EventReset => "EventReset",
            vkResult::Incomplete => "Incomplete",
            vkResult::ErrorOutOfHostMemory => "ErrorOutOfHostMemory",
            vkResult::ErrorOutOfDeviceMemory => "ErrorOutOfDeviceMemory",
            vkResult::ErrorInitializationFailed => "ErrorInitializationFailed",
            vkResult::ErrorDeviceLost => "ErrorDeviceLost",
            vkResult::ErrorMemoryMapFailed => "ErrorMemoryMapFailed",
            vkResult::ErrorLayerNotPresent => "ErrorLayerNotPresent",
            vkResult::ErrorExtensionNotPresent => "ErrorExtensionNotPresent",
            vkResult::ErrorFeatureNotPresent => "ErrorFeatureNotPresent",
            vkResult::ErrorIncompatibleDriver => "ErrorIncompatibleDriver",
            vkResult::ErrorTooManyObjects => "ErrorTooManyObjects",
            vkResult::ErrorFormatNotSupported => "ErrorFormatNotSupported",
            vkResult::ErrorFragmentedPool => "ErrorFragmentedPool",
            vkResult::ErrorUnknown => "ErrorUnknown",
            vkResult::ErrorValidationFailed => "ErrorValidationFailed",
            vkResult::ErrorOutOfPoolMemory => "ErrorOutOfPoolMemory",
            vkResult::ErrorInvalidExternalHandle => "ErrorInvalidExternalHandle",
            vkResult::ErrorInvalidOpaqueCaptureAddress => "ErrorInvalidOpaqueCaptureAddress",
            vkResult::ErrorFragmentation => "ErrorFragmentation",
            vkResult::PipelineCompileRequired => "PipelineCompileRequired",
            vkResult::ErrorNotPermitted => "ErrorNotPermitted",
            vkResult::ErrorInvalidPipelineCacheData => "ErrorInvalidPipelineCacheData",
            vkResult::ErrorNoPipelineMatch => "ErrorNoPipelineMatch",
            vkResult::ErrorSurfaceLostKHR => "ErrorSurfaceLostKHR",
            vkResult::ErrorNativeWindowInUseKHR => "ErrorNativeWindowInUseKHR",
            vkResult::SuboptimalKHR => "SuboptimalKHR",
            vkResult::ErrorOutOfDateKHR => "ErrorOutOfDateKHR",
            vkResult::ErrorIncompatibleDisplayKHR => "ErrorIncompatibleDisplayKHR",
            vkResult::ErrorInvalidShaderNV => "ErrorInvalidShaderNV",
            vkResult::ErrorImageUsageNotSupportedKHR => "ErrorImageUsageNotSupportedKHR",
            vkResult::ErrorVideoPictureLayoutNotSupportedKHR => {
                "ErrorVideoPictureLayoutNotSupportedKHR"
            }
            vkResult::ErrorVideoProfileOperationNotSupportedKHR => {
                "ErrorVideoProfileOperationNotSupportedKHR"
            }
            vkResult::ErrorVideoProfileFormatNotSupportedKHR => {
                "ErrorVideoProfileFormatNotSupportedKHR"
            }
            vkResult::ErrorVideoProfileCodecNotSupportedKHR => {
                "ErrorVideoProfileCodecNotSupportedKHR"
            }
            vkResult::ErrorVideoStdVersionNotSupportedKHR => "ErrorVideoStdVersionNotSupportedKHR",
            vkResult::ErrorInvalidDrmFormatModifierPlaneLayoutEXT => {
                "ErrorInvalidDrmFormatModifierPlaneLayoutEXT"
            }
            vkResult::ErrorPresentTimingQueueFullEXT => "ErrorPresentTimingQueueFullEXT",
            vkResult::ErrorFullScreenExclusiveModeLostEXT => "ErrorFullScreenExclusiveModeLostEXT",
            vkResult::ThreadIdleKHR => "ThreadIdleKHR",
            vkResult::ThreadDoneKHR => "ThreadDoneKHR",
            vkResult::OperationDeferredKHR => "OperationDeferredKHR",
            vkResult::OperationNotDeferredKHR => "OperationNotDeferredKHR",
            vkResult::ErrorInvalidVideoStdParametersKHR => "ErrorInvalidVideoStdParametersKHR",
            vkResult::ErrorCompressionExhaustedEXT => "ErrorCompressionExhaustedEXT",
            vkResult::IncompatibleShaderBinaryEXT => "IncompatibleShaderBinaryEXT",
            vkResult::PipelineBinaryMissingKHR => "PipelineBinaryMissingKHR",
            vkResult::ErrorNotEnoughSpaceKHR => "ErrorNotEnoughSpaceKHR",
        };

        write!(f, "{s}")
    }
}
impl vkPhysicalDevice {
    pub unsafe fn to_physical_device(self, instance: &Instance) -> PhysicalDevice {
        let v_table = &instance.v_table().physical_device;
        let v_table = unsafe { transmute::<&_, &'static _>(v_table) };

        PhysicalDevice {
            handle: self,
            v_table,
        }
    }
}

impl PhysicalDeviceMemoryProperties {
    #[inline]
    pub fn memory_types_as_slice(&self) -> &[MemoryType] {
        &self.memory_types[..self.memory_type_count as _]
    }
    #[inline]
    pub fn memory_heaps_as_slice(&self) -> &[MemoryHeap] {
        &self.memory_heaps[..self.memory_heap_count as _]
    }
}

impl ColorComponentFlags {
    /// Contraction of [`R`][Self::R] | [`G`][Self::G] | [`B`][Self::B] | [`A`][Self::A]
    pub const RGBA: Self = Self(Self::R.0 | Self::G.0 | Self::B.0 | Self::A.0);
}
