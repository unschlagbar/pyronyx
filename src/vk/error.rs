use crate::vk::vkResult;
use core::fmt;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkResult.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// A fence or query has not yet completed
    NotReady = 1,
    /// A wait operation has not completed in the specified time
    Timeout = 2,
    /// An event is signaled
    EventSet = 3,
    /// An event is unsignaled
    EventReset = 4,
    /// A return array was too small for the result
    Incomplete = 5,
    /// A host memory allocation has failed
    ErrorOutOfHostMemory = -1,
    /// A device memory allocation has failed
    ErrorOutOfDeviceMemory = -2,
    /// Initialization of an object has failed
    ErrorInitializationFailed = -3,
    /// The logical device has been lost. See <<devsandqueues-lost-device>>
    ErrorDeviceLost = -4,
    /// Mapping of a memory object has failed
    ErrorMemoryMapFailed = -5,
    /// Layer specified does not exist
    ErrorLayerNotPresent = -6,
    /// Extension specified does not exist
    ErrorExtensionNotPresent = -7,
    /// Requested feature is not available on this device
    ErrorFeatureNotPresent = -8,
    /// Unable to find a Vulkan driver
    ErrorIncompatibleDriver = -9,
    /// Too many objects of the type have already been created
    ErrorTooManyObjects = -10,
    /// Requested format is not supported on this device
    ErrorFormatNotSupported = -11,
    /// A requested pool allocation has failed due to fragmentation of the pool's memory
    ErrorFragmentedPool = -12,
    /// An unknown error has occurred, due to an implementation or application bug
    ErrorUnknown = -13,
    ErrorValidationFailed = -1000011001,
    ErrorOutOfPoolMemory = -1000069000,
    ErrorInvalidExternalHandle = -1000072003,
    ErrorInvalidOpaqueCaptureAddress = -1000257000,
    ErrorFragmentation = -1000161000,
    PipelineCompileRequired = 1000297000,
    ErrorNotPermitted = -1000174001,
    ErrorInvalidPipelineCacheData = -1000298000,
    ErrorNoPipelineMatch = -1000298001,
    ErrorSurfaceLostKHR = -1000000000,
    ErrorNativeWindowInUseKHR = -1000000001,
    SuboptimalKHR = 1000001003,
    ErrorOutOfDateKHR = -1000001004,
    ErrorIncompatibleDisplayKHR = -1000003001,
    ErrorInvalidShaderNV = -1000012000,
    ErrorImageUsageNotSupportedKHR = -1000023000,
    ErrorVideoPictureLayoutNotSupportedKHR = -1000023001,
    ErrorVideoProfileOperationNotSupportedKHR = -1000023002,
    ErrorVideoProfileFormatNotSupportedKHR = -1000023003,
    ErrorVideoProfileCodecNotSupportedKHR = -1000023004,
    ErrorVideoStdVersionNotSupportedKHR = -1000023005,
    ErrorInvalidDrmFormatModifierPlaneLayoutEXT = -1000158000,
    ErrorPresentTimingQueueFullEXT = -1000208000,
    ErrorFullScreenExclusiveModeLostEXT = -1000255000,
    ThreadIdleKHR = 1000268000,
    ThreadDoneKHR = 1000268001,
    OperationDeferredKHR = 1000268002,
    OperationNotDeferredKHR = 1000268003,
    ErrorInvalidVideoStdParametersKHR = -1000299000,
    ErrorCompressionExhaustedEXT = -1000338000,
    IncompatibleShaderBinaryEXT = 1000482000,
    PipelineBinaryMissingKHR = 1000483000,
    ErrorNotEnoughSpaceKHR = -1000483000,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::NotReady => "NotReady",
            Self::Timeout => "Timeout",
            Self::EventSet => "EventSet",
            Self::EventReset => "EventReset",
            Self::Incomplete => "Incomplete",
            Self::ErrorOutOfHostMemory => "ErrorOutOfHostMemory",
            Self::ErrorOutOfDeviceMemory => "ErrorOutOfDeviceMemory",
            Self::ErrorInitializationFailed => "ErrorInitializationFailed",
            Self::ErrorDeviceLost => "ErrorDeviceLost",
            Self::ErrorMemoryMapFailed => "ErrorMemoryMapFailed",
            Self::ErrorLayerNotPresent => "ErrorLayerNotPresent",
            Self::ErrorExtensionNotPresent => "ErrorExtensionNotPresent",
            Self::ErrorFeatureNotPresent => "ErrorFeatureNotPresent",
            Self::ErrorIncompatibleDriver => "ErrorIncompatibleDriver",
            Self::ErrorTooManyObjects => "ErrorTooManyObjects",
            Self::ErrorFormatNotSupported => "ErrorFormatNotSupported",
            Self::ErrorFragmentedPool => "ErrorFragmentedPool",
            Self::ErrorUnknown => "ErrorUnknown",
            Self::ErrorValidationFailed => "ErrorValidationFailed",
            Self::ErrorOutOfPoolMemory => "ErrorOutOfPoolMemory",
            Self::ErrorInvalidExternalHandle => "ErrorInvalidExternalHandle",
            Self::ErrorInvalidOpaqueCaptureAddress => "ErrorInvalidOpaqueCaptureAddress",
            Self::ErrorFragmentation => "ErrorFragmentation",
            Self::PipelineCompileRequired => "PipelineCompileRequired",
            Self::ErrorNotPermitted => "ErrorNotPermitted",
            Self::ErrorInvalidPipelineCacheData => "ErrorInvalidPipelineCacheData",
            Self::ErrorNoPipelineMatch => "ErrorNoPipelineMatch",
            Self::ErrorSurfaceLostKHR => "ErrorSurfaceLostKHR",
            Self::ErrorNativeWindowInUseKHR => "ErrorNativeWindowInUseKHR",
            Self::SuboptimalKHR => "SuboptimalKHR",
            Self::ErrorOutOfDateKHR => "ErrorOutOfDateKHR",
            Self::ErrorIncompatibleDisplayKHR => "ErrorIncompatibleDisplayKHR",
            Self::ErrorInvalidShaderNV => "ErrorInvalidShaderNV",
            Self::ErrorImageUsageNotSupportedKHR => "ErrorImageUsageNotSupportedKHR",
            Self::ErrorVideoPictureLayoutNotSupportedKHR => {
                "ErrorVideoPictureLayoutNotSupportedKHR"
            }
            Self::ErrorVideoProfileOperationNotSupportedKHR => {
                "ErrorVideoProfileOperationNotSupportedKHR"
            }
            Self::ErrorVideoProfileFormatNotSupportedKHR => {
                "ErrorVideoProfileFormatNotSupportedKHR"
            }
            Self::ErrorVideoProfileCodecNotSupportedKHR => "ErrorVideoProfileCodecNotSupportedKHR",
            Self::ErrorVideoStdVersionNotSupportedKHR => "ErrorVideoStdVersionNotSupportedKHR",
            Self::ErrorInvalidDrmFormatModifierPlaneLayoutEXT => {
                "ErrorInvalidDrmFormatModifierPlaneLayoutEXT"
            }
            Self::ErrorPresentTimingQueueFullEXT => "ErrorPresentTimingQueueFullEXT",
            Self::ErrorFullScreenExclusiveModeLostEXT => "ErrorFullScreenExclusiveModeLostEXT",
            Self::ThreadIdleKHR => "ThreadIdleKHR",
            Self::ThreadDoneKHR => "ThreadDoneKHR",
            Self::OperationDeferredKHR => "OperationDeferredKHR",
            Self::OperationNotDeferredKHR => "OperationNotDeferredKHR",
            Self::ErrorInvalidVideoStdParametersKHR => "ErrorInvalidVideoStdParametersKHR",
            Self::ErrorCompressionExhaustedEXT => "ErrorCompressionExhaustedEXT",
            Self::IncompatibleShaderBinaryEXT => "IncompatibleShaderBinaryEXT",
            Self::PipelineBinaryMissingKHR => "PipelineBinaryMissingKHR",
            Self::ErrorNotEnoughSpaceKHR => "ErrorNotEnoughSpaceKHR",
        };

        write!(f, "{s}")
    }
}

impl From<vkResult> for Error {
    #[inline]
    fn from(value: vkResult) -> Self {
        match value {
            vkResult::Success => unreachable!(),

            vkResult::NotReady => Error::NotReady,
            vkResult::Timeout => Error::Timeout,
            vkResult::EventSet => Error::EventSet,
            vkResult::EventReset => Error::EventReset,
            vkResult::Incomplete => Error::Incomplete,
            vkResult::ErrorOutOfHostMemory => Error::ErrorOutOfHostMemory,
            vkResult::ErrorOutOfDeviceMemory => Error::ErrorOutOfDeviceMemory,
            vkResult::ErrorInitializationFailed => Error::ErrorInitializationFailed,
            vkResult::ErrorDeviceLost => Error::ErrorDeviceLost,
            vkResult::ErrorMemoryMapFailed => Error::ErrorMemoryMapFailed,
            vkResult::ErrorLayerNotPresent => Error::ErrorLayerNotPresent,
            vkResult::ErrorExtensionNotPresent => Error::ErrorExtensionNotPresent,
            vkResult::ErrorFeatureNotPresent => Error::ErrorFeatureNotPresent,
            vkResult::ErrorIncompatibleDriver => Error::ErrorIncompatibleDriver,
            vkResult::ErrorTooManyObjects => Error::ErrorTooManyObjects,
            vkResult::ErrorFormatNotSupported => Error::ErrorFormatNotSupported,
            vkResult::ErrorFragmentedPool => Error::ErrorFragmentedPool,
            vkResult::ErrorUnknown => Error::ErrorUnknown,
            vkResult::ErrorValidationFailed => Error::ErrorValidationFailed,
            vkResult::ErrorOutOfPoolMemory => Error::ErrorOutOfPoolMemory,
            vkResult::ErrorInvalidExternalHandle => Error::ErrorInvalidExternalHandle,
            vkResult::ErrorInvalidOpaqueCaptureAddress => Error::ErrorInvalidOpaqueCaptureAddress,
            vkResult::ErrorFragmentation => Error::ErrorFragmentation,
            vkResult::PipelineCompileRequired => Error::PipelineCompileRequired,
            vkResult::ErrorNotPermitted => Error::ErrorNotPermitted,
            vkResult::ErrorInvalidPipelineCacheData => Error::ErrorInvalidPipelineCacheData,
            vkResult::ErrorNoPipelineMatch => Error::ErrorNoPipelineMatch,
            vkResult::ErrorSurfaceLostKHR => Error::ErrorSurfaceLostKHR,
            vkResult::ErrorNativeWindowInUseKHR => Error::ErrorNativeWindowInUseKHR,
            vkResult::SuboptimalKHR => Error::SuboptimalKHR,
            vkResult::ErrorOutOfDateKHR => Error::ErrorOutOfDateKHR,
            vkResult::ErrorIncompatibleDisplayKHR => Error::ErrorIncompatibleDisplayKHR,
            vkResult::ErrorInvalidShaderNV => Error::ErrorInvalidShaderNV,
            vkResult::ErrorImageUsageNotSupportedKHR => Error::ErrorImageUsageNotSupportedKHR,
            vkResult::ErrorVideoPictureLayoutNotSupportedKHR => {
                Error::ErrorVideoPictureLayoutNotSupportedKHR
            }
            vkResult::ErrorVideoProfileOperationNotSupportedKHR => {
                Error::ErrorVideoProfileOperationNotSupportedKHR
            }
            vkResult::ErrorVideoProfileFormatNotSupportedKHR => {
                Error::ErrorVideoProfileFormatNotSupportedKHR
            }
            vkResult::ErrorVideoProfileCodecNotSupportedKHR => {
                Error::ErrorVideoProfileCodecNotSupportedKHR
            }
            vkResult::ErrorVideoStdVersionNotSupportedKHR => {
                Error::ErrorVideoStdVersionNotSupportedKHR
            }
            vkResult::ErrorInvalidDrmFormatModifierPlaneLayoutEXT => {
                Error::ErrorInvalidDrmFormatModifierPlaneLayoutEXT
            }
            vkResult::ErrorPresentTimingQueueFullEXT => Error::ErrorPresentTimingQueueFullEXT,
            vkResult::ErrorFullScreenExclusiveModeLostEXT => {
                Error::ErrorFullScreenExclusiveModeLostEXT
            }
            vkResult::ThreadIdleKHR => Error::ThreadIdleKHR,
            vkResult::ThreadDoneKHR => Error::ThreadDoneKHR,
            vkResult::OperationDeferredKHR => Error::OperationDeferredKHR,
            vkResult::OperationNotDeferredKHR => Error::OperationNotDeferredKHR,
            vkResult::ErrorInvalidVideoStdParametersKHR => Error::ErrorInvalidVideoStdParametersKHR,
            vkResult::ErrorCompressionExhaustedEXT => Error::ErrorCompressionExhaustedEXT,
            vkResult::IncompatibleShaderBinaryEXT => Error::IncompatibleShaderBinaryEXT,
            vkResult::PipelineBinaryMissingKHR => Error::PipelineBinaryMissingKHR,
            vkResult::ErrorNotEnoughSpaceKHR => Error::ErrorNotEnoughSpaceKHR,
        }
    }
}
