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
    OutOfHostMemory = -1,
    /// A device memory allocation has failed
    OutOfDeviceMemory = -2,
    /// Initialization of an object has failed
    InitializationFailed = -3,
    /// The logical device has been lost. See <<devsandqueues-lost-device>>
    DeviceLost = -4,
    /// Mapping of a memory object has failed
    MemoryMapFailed = -5,
    /// Layer specified does not exist
    LayerNotPresent = -6,
    /// Extension specified does not exist
    ExtensionNotPresent = -7,
    /// Requested feature is not available on this device
    FeatureNotPresent = -8,
    /// Unable to find a Vulkan driver
    IncompatibleDriver = -9,
    /// Too many objects of the type have already been created
    TooManyObjects = -10,
    /// Requested format is not supported on this device
    FormatNotSupported = -11,
    /// A requested pool allocation has failed due to fragmentation of the pool's memory
    FragmentedPool = -12,
    /// An unknown error has occurred, due to an implementation or application bug
    Unknown = -13,
    ValidationFailed = -1000011001,
    OutOfPoolMemory = -1000069000,
    InvalidExternalHandle = -1000072003,
    InvalidOpaqueCaptureAddress = -1000257000,
    Fragmentation = -1000161000,
    PipelineCompileRequired = 1000297000,
    NotPermitted = -1000174001,
    InvalidPipelineCacheData = -1000298000,
    NoPipelineMatch = -1000298001,
    SurfaceLostKHR = -1000000000,
    NativeWindowInUseKHR = -1000000001,
    SuboptimalKHR = 1000001003,
    OutOfDateKHR = -1000001004,
    IncompatibleDisplayKHR = -1000003001,
    InvalidShaderNV = -1000012000,
    ImageUsageNotSupportedKHR = -1000023000,
    VideoPictureLayoutNotSupportedKHR = -1000023001,
    VideoProfileOperationNotSupportedKHR = -1000023002,
    VideoProfileFormatNotSupportedKHR = -1000023003,
    VideoProfileCodecNotSupportedKHR = -1000023004,
    VideoStdVersionNotSupportedKHR = -1000023005,
    InvalidDrmFormatModifierPlaneLayoutEXT = -1000158000,
    PresentTimingQueueFullEXT = -1000208000,
    FullScreenExclusiveModeLostEXT = -1000255000,
    ThreadIdleKHR = 1000268000,
    ThreadDoneKHR = 1000268001,
    OperationDeferredKHR = 1000268002,
    OperationNotDeferredKHR = 1000268003,
    InvalidVideoStdParametersKHR = -1000299000,
    CompressionExhaustedEXT = -1000338000,
    IncompatibleShaderBinaryEXT = 1000482000,
    PipelineBinaryMissingKHR = 1000483000,
    NotEnoughSpaceKHR = -1000483000,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::NotReady => "NotReady",
            Self::Timeout => "Timeout",
            Self::EventSet => "EventSet",
            Self::EventReset => "EventReset",
            Self::Incomplete => "Incomplete",
            Self::OutOfHostMemory => "ErrorOutOfHostMemory",
            Self::OutOfDeviceMemory => "ErrorOutOfDeviceMemory",
            Self::InitializationFailed => "ErrorInitializationFailed",
            Self::DeviceLost => "ErrorDeviceLost",
            Self::MemoryMapFailed => "ErrorMemoryMapFailed",
            Self::LayerNotPresent => "ErrorLayerNotPresent",
            Self::ExtensionNotPresent => "ErrorExtensionNotPresent",
            Self::FeatureNotPresent => "ErrorFeatureNotPresent",
            Self::IncompatibleDriver => "ErrorIncompatibleDriver",
            Self::TooManyObjects => "ErrorTooManyObjects",
            Self::FormatNotSupported => "ErrorFormatNotSupported",
            Self::FragmentedPool => "ErrorFragmentedPool",
            Self::Unknown => "ErrorUnknown",
            Self::ValidationFailed => "ErrorValidationFailed",
            Self::OutOfPoolMemory => "ErrorOutOfPoolMemory",
            Self::InvalidExternalHandle => "ErrorInvalidExternalHandle",
            Self::InvalidOpaqueCaptureAddress => "ErrorInvalidOpaqueCaptureAddress",
            Self::Fragmentation => "ErrorFragmentation",
            Self::PipelineCompileRequired => "PipelineCompileRequired",
            Self::NotPermitted => "ErrorNotPermitted",
            Self::InvalidPipelineCacheData => "ErrorInvalidPipelineCacheData",
            Self::NoPipelineMatch => "ErrorNoPipelineMatch",
            Self::SurfaceLostKHR => "ErrorSurfaceLostKHR",
            Self::NativeWindowInUseKHR => "ErrorNativeWindowInUseKHR",
            Self::SuboptimalKHR => "SuboptimalKHR",
            Self::OutOfDateKHR => "ErrorOutOfDateKHR",
            Self::IncompatibleDisplayKHR => "ErrorIncompatibleDisplayKHR",
            Self::InvalidShaderNV => "ErrorInvalidShaderNV",
            Self::ImageUsageNotSupportedKHR => "ErrorImageUsageNotSupportedKHR",
            Self::VideoPictureLayoutNotSupportedKHR => "ErrorVideoPictureLayoutNotSupportedKHR",
            Self::VideoProfileOperationNotSupportedKHR => {
                "ErrorVideoProfileOperationNotSupportedKHR"
            }
            Self::VideoProfileFormatNotSupportedKHR => "ErrorVideoProfileFormatNotSupportedKHR",
            Self::VideoProfileCodecNotSupportedKHR => "ErrorVideoProfileCodecNotSupportedKHR",
            Self::VideoStdVersionNotSupportedKHR => "ErrorVideoStdVersionNotSupportedKHR",
            Self::InvalidDrmFormatModifierPlaneLayoutEXT => {
                "ErrorInvalidDrmFormatModifierPlaneLayoutEXT"
            }
            Self::PresentTimingQueueFullEXT => "ErrorPresentTimingQueueFullEXT",
            Self::FullScreenExclusiveModeLostEXT => "ErrorFullScreenExclusiveModeLostEXT",
            Self::ThreadIdleKHR => "ThreadIdleKHR",
            Self::ThreadDoneKHR => "ThreadDoneKHR",
            Self::OperationDeferredKHR => "OperationDeferredKHR",
            Self::OperationNotDeferredKHR => "OperationNotDeferredKHR",
            Self::InvalidVideoStdParametersKHR => "ErrorInvalidVideoStdParametersKHR",
            Self::CompressionExhaustedEXT => "ErrorCompressionExhaustedEXT",
            Self::IncompatibleShaderBinaryEXT => "IncompatibleShaderBinaryEXT",
            Self::PipelineBinaryMissingKHR => "PipelineBinaryMissingKHR",
            Self::NotEnoughSpaceKHR => "ErrorNotEnoughSpaceKHR",
        };

        write!(f, "{s}")
    }
}

impl From<vkResult> for Error {
    #[inline]
    fn from(value: vkResult) -> Self {
        match value {
            vkResult::Success => unreachable!(),

            vkResult::NotReady => Self::NotReady,
            vkResult::Timeout => Self::Timeout,
            vkResult::EventSet => Self::EventSet,
            vkResult::EventReset => Self::EventReset,
            vkResult::Incomplete => Self::Incomplete,
            vkResult::ErrorOutOfHostMemory => Self::OutOfHostMemory,
            vkResult::ErrorOutOfDeviceMemory => Self::OutOfDeviceMemory,
            vkResult::ErrorInitializationFailed => Self::InitializationFailed,
            vkResult::ErrorDeviceLost => Self::DeviceLost,
            vkResult::ErrorMemoryMapFailed => Self::MemoryMapFailed,
            vkResult::ErrorLayerNotPresent => Self::LayerNotPresent,
            vkResult::ErrorExtensionNotPresent => Self::ExtensionNotPresent,
            vkResult::ErrorFeatureNotPresent => Self::FeatureNotPresent,
            vkResult::ErrorIncompatibleDriver => Self::IncompatibleDriver,
            vkResult::ErrorTooManyObjects => Self::TooManyObjects,
            vkResult::ErrorFormatNotSupported => Self::FormatNotSupported,
            vkResult::ErrorFragmentedPool => Self::FragmentedPool,
            vkResult::ErrorUnknown => Self::Unknown,
            vkResult::ErrorValidationFailed => Self::ValidationFailed,
            vkResult::ErrorOutOfPoolMemory => Self::OutOfPoolMemory,
            vkResult::ErrorInvalidExternalHandle => Self::InvalidExternalHandle,
            vkResult::ErrorInvalidOpaqueCaptureAddress => Self::InvalidOpaqueCaptureAddress,
            vkResult::ErrorFragmentation => Self::Fragmentation,
            vkResult::PipelineCompileRequired => Self::PipelineCompileRequired,
            vkResult::ErrorNotPermitted => Self::NotPermitted,
            vkResult::ErrorInvalidPipelineCacheData => Self::InvalidPipelineCacheData,
            vkResult::ErrorNoPipelineMatch => Self::NoPipelineMatch,
            vkResult::ErrorSurfaceLostKHR => Self::SurfaceLostKHR,
            vkResult::ErrorNativeWindowInUseKHR => Self::NativeWindowInUseKHR,
            vkResult::SuboptimalKHR => Self::SuboptimalKHR,
            vkResult::ErrorOutOfDateKHR => Self::OutOfDateKHR,
            vkResult::ErrorIncompatibleDisplayKHR => Self::IncompatibleDisplayKHR,
            vkResult::ErrorInvalidShaderNV => Self::InvalidShaderNV,
            vkResult::ErrorImageUsageNotSupportedKHR => Self::ImageUsageNotSupportedKHR,
            vkResult::ErrorVideoPictureLayoutNotSupportedKHR => {
                Self::VideoPictureLayoutNotSupportedKHR
            }
            vkResult::ErrorVideoProfileOperationNotSupportedKHR => {
                Self::VideoProfileOperationNotSupportedKHR
            }
            vkResult::ErrorVideoProfileFormatNotSupportedKHR => {
                Self::VideoProfileFormatNotSupportedKHR
            }
            vkResult::ErrorVideoProfileCodecNotSupportedKHR => {
                Self::VideoProfileCodecNotSupportedKHR
            }
            vkResult::ErrorVideoStdVersionNotSupportedKHR => Self::VideoStdVersionNotSupportedKHR,
            vkResult::ErrorInvalidDrmFormatModifierPlaneLayoutEXT => {
                Self::InvalidDrmFormatModifierPlaneLayoutEXT
            }
            vkResult::ErrorPresentTimingQueueFullEXT => Self::PresentTimingQueueFullEXT,
            vkResult::ErrorFullScreenExclusiveModeLostEXT => Self::FullScreenExclusiveModeLostEXT,
            vkResult::ThreadIdleKHR => Self::ThreadIdleKHR,
            vkResult::ThreadDoneKHR => Self::ThreadDoneKHR,
            vkResult::OperationDeferredKHR => Self::OperationDeferredKHR,
            vkResult::OperationNotDeferredKHR => Self::OperationNotDeferredKHR,
            vkResult::ErrorInvalidVideoStdParametersKHR => Self::InvalidVideoStdParametersKHR,
            vkResult::ErrorCompressionExhaustedEXT => Self::CompressionExhaustedEXT,
            vkResult::IncompatibleShaderBinaryEXT => Self::IncompatibleShaderBinaryEXT,
            vkResult::PipelineBinaryMissingKHR => Self::PipelineBinaryMissingKHR,
            vkResult::ErrorNotEnoughSpaceKHR => Self::NotEnoughSpaceKHR,
        }
    }
}
