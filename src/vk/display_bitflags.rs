// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated Display impls for bitflag types
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::fmt::{Debug, Display, Formatter, Result};

impl Display for FramebufferCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(FramebufferCreateFlags, &str)] =
            &[(FramebufferCreateFlags::Imageless, "Imageless")];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for FramebufferCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for QueryPoolCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(QueryPoolCreateFlags, &str)] =
            &[(QueryPoolCreateFlags::ResetKHR, "ResetKHR")];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for QueryPoolCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for RenderPassCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(RenderPassCreateFlags, &str)] = &[
            (RenderPassCreateFlags::TransformQCOM, "TransformQCOM"),
            (
                RenderPassCreateFlags::PerLayerFragmentDensityVALVE,
                "PerLayerFragmentDensityVALVE",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for RenderPassCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for SamplerCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(SamplerCreateFlags, &str)] = &[
            (SamplerCreateFlags::SubsampledEXT, "SubsampledEXT"),
            (
                SamplerCreateFlags::SubsampledCoarseReconstructionEXT,
                "SubsampledCoarseReconstructionEXT",
            ),
            (
                SamplerCreateFlags::DescriptorBufferCaptureReplayEXT,
                "DescriptorBufferCaptureReplayEXT",
            ),
            (
                SamplerCreateFlags::NonSeamlessCubeMapEXT,
                "NonSeamlessCubeMapEXT",
            ),
            (
                SamplerCreateFlags::ImageProcessingQCOM,
                "ImageProcessingQCOM",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for SamplerCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for PipelineLayoutCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(PipelineLayoutCreateFlags, &str)] = &[(
            PipelineLayoutCreateFlags::IndependentSetsEXT,
            "IndependentSetsEXT",
        )];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for PipelineLayoutCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for PipelineCacheCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(PipelineCacheCreateFlags, &str)] = &[
            (
                PipelineCacheCreateFlags::ExternallySynchronized,
                "ExternallySynchronized",
            ),
            (PipelineCacheCreateFlags::ReadOnly, "ReadOnly"),
            (
                PipelineCacheCreateFlags::UseApplicationStorage,
                "UseApplicationStorage",
            ),
            (
                PipelineCacheCreateFlags::InternallySynchronizedMergeKHR,
                "InternallySynchronizedMergeKHR",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for PipelineCacheCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for PipelineShaderStageCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(PipelineShaderStageCreateFlags, &str)] = &[
            (
                PipelineShaderStageCreateFlags::AllowVaryingSubgroupSize,
                "AllowVaryingSubgroupSize",
            ),
            (
                PipelineShaderStageCreateFlags::RequireFullSubgroups,
                "RequireFullSubgroups",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for PipelineShaderStageCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for DescriptorSetLayoutCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(DescriptorSetLayoutCreateFlags, &str)] = &[
            (
                DescriptorSetLayoutCreateFlags::UpdateAfterBindPool,
                "UpdateAfterBindPool",
            ),
            (
                DescriptorSetLayoutCreateFlags::PushDescriptor,
                "PushDescriptor",
            ),
            (
                DescriptorSetLayoutCreateFlags::DescriptorBufferEXT,
                "DescriptorBufferEXT",
            ),
            (
                DescriptorSetLayoutCreateFlags::EmbeddedImmutableSamplersEXT,
                "EmbeddedImmutableSamplersEXT",
            ),
            (
                DescriptorSetLayoutCreateFlags::IndirectBindableNV,
                "IndirectBindableNV",
            ),
            (
                DescriptorSetLayoutCreateFlags::HostOnlyPoolEXT,
                "HostOnlyPoolEXT",
            ),
            (DescriptorSetLayoutCreateFlags::PerStageNV, "PerStageNV"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for DescriptorSetLayoutCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for InstanceCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(InstanceCreateFlags, &str)] = &[(
            InstanceCreateFlags::EnumeratePortabilityKHR,
            "EnumeratePortabilityKHR",
        )];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for InstanceCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for DeviceQueueCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(DeviceQueueCreateFlags, &str)] = &[
            (DeviceQueueCreateFlags::Protected, "Protected"),
            (
                DeviceQueueCreateFlags::InternallySynchronizedKHR,
                "InternallySynchronizedKHR",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for DeviceQueueCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for QueueFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(QueueFlags, &str)] = &[
            (QueueFlags::Graphics, "Graphics"),
            (QueueFlags::Compute, "Compute"),
            (QueueFlags::Transfer, "Transfer"),
            (QueueFlags::SparseBinding, "SparseBinding"),
            (QueueFlags::Protected, "Protected"),
            (QueueFlags::VideoDecodeKHR, "VideoDecodeKHR"),
            (QueueFlags::VideoEncodeKHR, "VideoEncodeKHR"),
            (QueueFlags::OpticalFlowNV, "OpticalFlowNV"),
            (QueueFlags::DataGraphARM, "DataGraphARM"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for QueueFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for MemoryPropertyFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(MemoryPropertyFlags, &str)] = &[
            (MemoryPropertyFlags::DeviceLocal, "DeviceLocal"),
            (MemoryPropertyFlags::HostVisible, "HostVisible"),
            (MemoryPropertyFlags::HostCoherent, "HostCoherent"),
            (MemoryPropertyFlags::HostCached, "HostCached"),
            (MemoryPropertyFlags::LazilyAllocated, "LazilyAllocated"),
            (MemoryPropertyFlags::Protected, "Protected"),
            (MemoryPropertyFlags::DeviceCoherentAMD, "DeviceCoherentAMD"),
            (MemoryPropertyFlags::DeviceUncachedAMD, "DeviceUncachedAMD"),
            (MemoryPropertyFlags::RdmaCapableNV, "RdmaCapableNV"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for MemoryPropertyFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for MemoryHeapFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(MemoryHeapFlags, &str)] = &[
            (MemoryHeapFlags::DeviceLocal, "DeviceLocal"),
            (MemoryHeapFlags::MultiInstance, "MultiInstance"),
            (MemoryHeapFlags::SeuSafe, "SeuSafe"),
            (MemoryHeapFlags::TileMemoryQCOM, "TileMemoryQCOM"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for MemoryHeapFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for AccessFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(AccessFlags, &str)] = &[
            (AccessFlags::IndirectCommandRead, "IndirectCommandRead"),
            (AccessFlags::IndexRead, "IndexRead"),
            (AccessFlags::VertexAttributeRead, "VertexAttributeRead"),
            (AccessFlags::UniformRead, "UniformRead"),
            (AccessFlags::InputAttachmentRead, "InputAttachmentRead"),
            (AccessFlags::ShaderRead, "ShaderRead"),
            (AccessFlags::ShaderWrite, "ShaderWrite"),
            (AccessFlags::ColorAttachmentRead, "ColorAttachmentRead"),
            (AccessFlags::ColorAttachmentWrite, "ColorAttachmentWrite"),
            (
                AccessFlags::DepthStencilAttachmentRead,
                "DepthStencilAttachmentRead",
            ),
            (
                AccessFlags::DepthStencilAttachmentWrite,
                "DepthStencilAttachmentWrite",
            ),
            (AccessFlags::TransferRead, "TransferRead"),
            (AccessFlags::TransferWrite, "TransferWrite"),
            (AccessFlags::HostRead, "HostRead"),
            (AccessFlags::HostWrite, "HostWrite"),
            (AccessFlags::MemoryRead, "MemoryRead"),
            (AccessFlags::MemoryWrite, "MemoryWrite"),
            (AccessFlags::None, "None"),
            (
                AccessFlags::TransformFeedbackWriteEXT,
                "TransformFeedbackWriteEXT",
            ),
            (
                AccessFlags::TransformFeedbackCounterReadEXT,
                "TransformFeedbackCounterReadEXT",
            ),
            (
                AccessFlags::TransformFeedbackCounterWriteEXT,
                "TransformFeedbackCounterWriteEXT",
            ),
            (
                AccessFlags::ConditionalRenderingReadEXT,
                "ConditionalRenderingReadEXT",
            ),
            (
                AccessFlags::ColorAttachmentReadNoncoherentEXT,
                "ColorAttachmentReadNoncoherentEXT",
            ),
            (
                AccessFlags::AccelerationStructureReadKHR,
                "AccelerationStructureReadKHR",
            ),
            (
                AccessFlags::AccelerationStructureWriteKHR,
                "AccelerationStructureWriteKHR",
            ),
            (
                AccessFlags::FragmentDensityMapReadEXT,
                "FragmentDensityMapReadEXT",
            ),
            (
                AccessFlags::FragmentShadingRateAttachmentReadKHR,
                "FragmentShadingRateAttachmentReadKHR",
            ),
            (
                AccessFlags::CommandPreprocessReadEXT,
                "CommandPreprocessReadEXT",
            ),
            (
                AccessFlags::CommandPreprocessWriteEXT,
                "CommandPreprocessWriteEXT",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for AccessFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for BufferUsageFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(BufferUsageFlags, &str)] = &[
            (BufferUsageFlags::TransferSrc, "TransferSrc"),
            (BufferUsageFlags::TransferDst, "TransferDst"),
            (BufferUsageFlags::UniformTexelBuffer, "UniformTexelBuffer"),
            (BufferUsageFlags::StorageTexelBuffer, "StorageTexelBuffer"),
            (BufferUsageFlags::UniformBuffer, "UniformBuffer"),
            (BufferUsageFlags::StorageBuffer, "StorageBuffer"),
            (BufferUsageFlags::IndexBuffer, "IndexBuffer"),
            (BufferUsageFlags::VertexBuffer, "VertexBuffer"),
            (BufferUsageFlags::IndirectBuffer, "IndirectBuffer"),
            (BufferUsageFlags::ShaderDeviceAddress, "ShaderDeviceAddress"),
            (BufferUsageFlags::VideoDecodeSrcKHR, "VideoDecodeSrcKHR"),
            (BufferUsageFlags::VideoDecodeDstKHR, "VideoDecodeDstKHR"),
            (
                BufferUsageFlags::TransformFeedbackBufferEXT,
                "TransformFeedbackBufferEXT",
            ),
            (
                BufferUsageFlags::TransformFeedbackCounterBufferEXT,
                "TransformFeedbackCounterBufferEXT",
            ),
            (
                BufferUsageFlags::ConditionalRenderingEXT,
                "ConditionalRenderingEXT",
            ),
            (
                BufferUsageFlags::ExecutionGraphScratchAMDX,
                "ExecutionGraphScratchAMDX",
            ),
            (BufferUsageFlags::DescriptorHeapEXT, "DescriptorHeapEXT"),
            (
                BufferUsageFlags::AccelerationStructureBuildInputReadOnlyKHR,
                "AccelerationStructureBuildInputReadOnlyKHR",
            ),
            (
                BufferUsageFlags::AccelerationStructureStorageKHR,
                "AccelerationStructureStorageKHR",
            ),
            (
                BufferUsageFlags::ShaderBindingTableKHR,
                "ShaderBindingTableKHR",
            ),
            (BufferUsageFlags::VideoEncodeDstKHR, "VideoEncodeDstKHR"),
            (BufferUsageFlags::VideoEncodeSrcKHR, "VideoEncodeSrcKHR"),
            (
                BufferUsageFlags::SamplerDescriptorBufferEXT,
                "SamplerDescriptorBufferEXT",
            ),
            (
                BufferUsageFlags::ResourceDescriptorBufferEXT,
                "ResourceDescriptorBufferEXT",
            ),
            (
                BufferUsageFlags::PushDescriptorsDescriptorBufferEXT,
                "PushDescriptorsDescriptorBufferEXT",
            ),
            (
                BufferUsageFlags::MicromapBuildInputReadOnlyEXT,
                "MicromapBuildInputReadOnlyEXT",
            ),
            (BufferUsageFlags::MicromapStorageEXT, "MicromapStorageEXT"),
            (BufferUsageFlags::TileMemoryQCOM, "TileMemoryQCOM"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for BufferUsageFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for BufferCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(BufferCreateFlags, &str)] = &[
            (BufferCreateFlags::SparseBinding, "SparseBinding"),
            (BufferCreateFlags::SparseResidency, "SparseResidency"),
            (BufferCreateFlags::SparseAliased, "SparseAliased"),
            (BufferCreateFlags::Protected, "Protected"),
            (
                BufferCreateFlags::DeviceAddressCaptureReplay,
                "DeviceAddressCaptureReplay",
            ),
            (
                BufferCreateFlags::DescriptorBufferCaptureReplayEXT,
                "DescriptorBufferCaptureReplayEXT",
            ),
            (
                BufferCreateFlags::VideoProfileIndependentKHR,
                "VideoProfileIndependentKHR",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for BufferCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for ShaderStageFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(ShaderStageFlags, &str)] = &[
            (ShaderStageFlags::Vertex, "Vertex"),
            (ShaderStageFlags::TessellationControl, "TessellationControl"),
            (
                ShaderStageFlags::TessellationEvaluation,
                "TessellationEvaluation",
            ),
            (ShaderStageFlags::Geometry, "Geometry"),
            (ShaderStageFlags::Fragment, "Fragment"),
            (ShaderStageFlags::Compute, "Compute"),
            (ShaderStageFlags::AllGraphics, "AllGraphics"),
            (ShaderStageFlags::All, "All"),
            (ShaderStageFlags::RaygenKHR, "RaygenKHR"),
            (ShaderStageFlags::AnyHitKHR, "AnyHitKHR"),
            (ShaderStageFlags::ClosestHitKHR, "ClosestHitKHR"),
            (ShaderStageFlags::MissKHR, "MissKHR"),
            (ShaderStageFlags::IntersectionKHR, "IntersectionKHR"),
            (ShaderStageFlags::CallableKHR, "CallableKHR"),
            (ShaderStageFlags::TaskEXT, "TaskEXT"),
            (ShaderStageFlags::MeshEXT, "MeshEXT"),
            (
                ShaderStageFlags::SubpassShadingHUAWEI,
                "SubpassShadingHUAWEI",
            ),
            (
                ShaderStageFlags::ClusterCullingHUAWEI,
                "ClusterCullingHUAWEI",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for ShaderStageFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for ImageUsageFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(ImageUsageFlags, &str)] = &[
            (ImageUsageFlags::TransferSrc, "TransferSrc"),
            (ImageUsageFlags::TransferDst, "TransferDst"),
            (ImageUsageFlags::Sampled, "Sampled"),
            (ImageUsageFlags::Storage, "Storage"),
            (ImageUsageFlags::ColorAttachment, "ColorAttachment"),
            (
                ImageUsageFlags::DepthStencilAttachment,
                "DepthStencilAttachment",
            ),
            (ImageUsageFlags::TransientAttachment, "TransientAttachment"),
            (ImageUsageFlags::InputAttachment, "InputAttachment"),
            (ImageUsageFlags::HostTransfer, "HostTransfer"),
            (ImageUsageFlags::VideoDecodeDstKHR, "VideoDecodeDstKHR"),
            (ImageUsageFlags::VideoDecodeSrcKHR, "VideoDecodeSrcKHR"),
            (ImageUsageFlags::VideoDecodeDpbKHR, "VideoDecodeDpbKHR"),
            (
                ImageUsageFlags::FragmentDensityMapEXT,
                "FragmentDensityMapEXT",
            ),
            (
                ImageUsageFlags::FragmentShadingRateAttachmentKHR,
                "FragmentShadingRateAttachmentKHR",
            ),
            (ImageUsageFlags::VideoEncodeDstKHR, "VideoEncodeDstKHR"),
            (ImageUsageFlags::VideoEncodeSrcKHR, "VideoEncodeSrcKHR"),
            (ImageUsageFlags::VideoEncodeDpbKHR, "VideoEncodeDpbKHR"),
            (
                ImageUsageFlags::AttachmentFeedbackLoopEXT,
                "AttachmentFeedbackLoopEXT",
            ),
            (
                ImageUsageFlags::InvocationMaskHUAWEI,
                "InvocationMaskHUAWEI",
            ),
            (ImageUsageFlags::SampleWeightQCOM, "SampleWeightQCOM"),
            (
                ImageUsageFlags::SampleBlockMatchQCOM,
                "SampleBlockMatchQCOM",
            ),
            (ImageUsageFlags::TensorAliasingARM, "TensorAliasingARM"),
            (ImageUsageFlags::TileMemoryQCOM, "TileMemoryQCOM"),
            (
                ImageUsageFlags::VideoEncodeQuantizationDeltaMapKHR,
                "VideoEncodeQuantizationDeltaMapKHR",
            ),
            (
                ImageUsageFlags::VideoEncodeEmphasisMapKHR,
                "VideoEncodeEmphasisMapKHR",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for ImageUsageFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for ImageCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(ImageCreateFlags, &str)] = &[
            (ImageCreateFlags::SparseBinding, "SparseBinding"),
            (ImageCreateFlags::SparseResidency, "SparseResidency"),
            (ImageCreateFlags::SparseAliased, "SparseAliased"),
            (ImageCreateFlags::MutableFormat, "MutableFormat"),
            (ImageCreateFlags::CubeCompatible, "CubeCompatible"),
            (ImageCreateFlags::Alias, "Alias"),
            (
                ImageCreateFlags::SplitInstanceBindRegions,
                "SplitInstanceBindRegions",
            ),
            (
                ImageCreateFlags::Type2dArrayCompatible,
                "Type2dArrayCompatible",
            ),
            (
                ImageCreateFlags::BlockTexelViewCompatible,
                "BlockTexelViewCompatible",
            ),
            (ImageCreateFlags::ExtendedUsage, "ExtendedUsage"),
            (ImageCreateFlags::Protected, "Protected"),
            (ImageCreateFlags::Disjoint, "Disjoint"),
            (ImageCreateFlags::CornerSampledNV, "CornerSampledNV"),
            (
                ImageCreateFlags::DescriptorHeapCaptureReplayEXT,
                "DescriptorHeapCaptureReplayEXT",
            ),
            (
                ImageCreateFlags::SampleLocationsCompatibleDepthEXT,
                "SampleLocationsCompatibleDepthEXT",
            ),
            (ImageCreateFlags::SubsampledEXT, "SubsampledEXT"),
            (
                ImageCreateFlags::MultisampledRenderToSingleSampledEXT,
                "MultisampledRenderToSingleSampledEXT",
            ),
            (
                ImageCreateFlags::Type2dViewCompatibleEXT,
                "Type2dViewCompatibleEXT",
            ),
            (
                ImageCreateFlags::VideoProfileIndependentKHR,
                "VideoProfileIndependentKHR",
            ),
            (
                ImageCreateFlags::FragmentDensityMapOffsetEXT,
                "FragmentDensityMapOffsetEXT",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for ImageCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for ImageViewCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(ImageViewCreateFlags, &str)] = &[
            (
                ImageViewCreateFlags::FragmentDensityMapDynamicEXT,
                "FragmentDensityMapDynamicEXT",
            ),
            (
                ImageViewCreateFlags::DescriptorBufferCaptureReplayEXT,
                "DescriptorBufferCaptureReplayEXT",
            ),
            (
                ImageViewCreateFlags::FragmentDensityMapDeferredEXT,
                "FragmentDensityMapDeferredEXT",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for ImageViewCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for PipelineCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(PipelineCreateFlags, &str)] = &[
            (
                PipelineCreateFlags::DisableOptimization,
                "DisableOptimization",
            ),
            (PipelineCreateFlags::AllowDerivatives, "AllowDerivatives"),
            (PipelineCreateFlags::Derivative, "Derivative"),
            (PipelineCreateFlags::DispatchBase, "DispatchBase"),
            (
                PipelineCreateFlags::ViewIndexFromDeviceIndex,
                "ViewIndexFromDeviceIndex",
            ),
            (
                PipelineCreateFlags::FailOnPipelineCompileRequired,
                "FailOnPipelineCompileRequired",
            ),
            (
                PipelineCreateFlags::EarlyReturnOnFailure,
                "EarlyReturnOnFailure",
            ),
            (PipelineCreateFlags::NoProtectedAccess, "NoProtectedAccess"),
            (
                PipelineCreateFlags::ProtectedAccessOnly,
                "ProtectedAccessOnly",
            ),
            (
                PipelineCreateFlags::RayTracingNoNullAnyHitShadersKHR,
                "RayTracingNoNullAnyHitShadersKHR",
            ),
            (
                PipelineCreateFlags::RayTracingNoNullClosestHitShadersKHR,
                "RayTracingNoNullClosestHitShadersKHR",
            ),
            (
                PipelineCreateFlags::RayTracingNoNullMissShadersKHR,
                "RayTracingNoNullMissShadersKHR",
            ),
            (
                PipelineCreateFlags::RayTracingNoNullIntersectionShadersKHR,
                "RayTracingNoNullIntersectionShadersKHR",
            ),
            (
                PipelineCreateFlags::RayTracingSkipTrianglesKHR,
                "RayTracingSkipTrianglesKHR",
            ),
            (
                PipelineCreateFlags::RayTracingSkipAabbsKHR,
                "RayTracingSkipAabbsKHR",
            ),
            (
                PipelineCreateFlags::RayTracingShaderGroupHandleCaptureReplayKHR,
                "RayTracingShaderGroupHandleCaptureReplayKHR",
            ),
            (PipelineCreateFlags::DeferCompileNV, "DeferCompileNV"),
            (
                PipelineCreateFlags::RenderingFragmentDensityMapAttachmentEXT,
                "RenderingFragmentDensityMapAttachmentEXT",
            ),
            (
                PipelineCreateFlags::RenderingFragmentShadingRateAttachmentKHR,
                "RenderingFragmentShadingRateAttachmentKHR",
            ),
            (
                PipelineCreateFlags::CaptureStatisticsKHR,
                "CaptureStatisticsKHR",
            ),
            (
                PipelineCreateFlags::CaptureInternalRepresentationsKHR,
                "CaptureInternalRepresentationsKHR",
            ),
            (
                PipelineCreateFlags::IndirectBindableNV,
                "IndirectBindableNV",
            ),
            (PipelineCreateFlags::LibraryKHR, "LibraryKHR"),
            (
                PipelineCreateFlags::DescriptorBufferEXT,
                "DescriptorBufferEXT",
            ),
            (
                PipelineCreateFlags::RetainLinkTimeOptimizationInfoEXT,
                "RetainLinkTimeOptimizationInfoEXT",
            ),
            (
                PipelineCreateFlags::LinkTimeOptimizationEXT,
                "LinkTimeOptimizationEXT",
            ),
            (
                PipelineCreateFlags::RayTracingAllowMotionNV,
                "RayTracingAllowMotionNV",
            ),
            (
                PipelineCreateFlags::ColorAttachmentFeedbackLoopEXT,
                "ColorAttachmentFeedbackLoopEXT",
            ),
            (
                PipelineCreateFlags::DepthStencilAttachmentFeedbackLoopEXT,
                "DepthStencilAttachmentFeedbackLoopEXT",
            ),
            (
                PipelineCreateFlags::RayTracingOpacityMicromapEXT,
                "RayTracingOpacityMicromapEXT",
            ),
            (
                PipelineCreateFlags::RayTracingDisplacementMicromapNV,
                "RayTracingDisplacementMicromapNV",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for PipelineCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for ColorComponentFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(ColorComponentFlags, &str)] = &[
            (ColorComponentFlags::R, "R"),
            (ColorComponentFlags::G, "G"),
            (ColorComponentFlags::B, "B"),
            (ColorComponentFlags::A, "A"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for ColorComponentFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for FenceCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(FenceCreateFlags, &str)] = &[(FenceCreateFlags::Signaled, "Signaled")];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for FenceCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for FormatFeatureFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(FormatFeatureFlags, &str)] = &[
            (FormatFeatureFlags::SampledImage, "SampledImage"),
            (FormatFeatureFlags::StorageImage, "StorageImage"),
            (FormatFeatureFlags::StorageImageAtomic, "StorageImageAtomic"),
            (FormatFeatureFlags::UniformTexelBuffer, "UniformTexelBuffer"),
            (FormatFeatureFlags::StorageTexelBuffer, "StorageTexelBuffer"),
            (FormatFeatureFlags::StorageTexelBufferAtomic, "StorageTexelBufferAtomic"),
            (FormatFeatureFlags::VertexBuffer, "VertexBuffer"),
            (FormatFeatureFlags::ColorAttachment, "ColorAttachment"),
            (FormatFeatureFlags::ColorAttachmentBlend, "ColorAttachmentBlend"),
            (FormatFeatureFlags::DepthStencilAttachment, "DepthStencilAttachment"),
            (FormatFeatureFlags::BlitSrc, "BlitSrc"),
            (FormatFeatureFlags::BlitDst, "BlitDst"),
            (FormatFeatureFlags::SampledImageFilterLinear, "SampledImageFilterLinear"),
            (FormatFeatureFlags::TransferSrc, "TransferSrc"),
            (FormatFeatureFlags::TransferDst, "TransferDst"),
            (FormatFeatureFlags::MidpointChromaSamples, "MidpointChromaSamples"),
            (FormatFeatureFlags::SampledImageYcbcrConversionLinearFilter, "SampledImageYcbcrConversionLinearFilter"),
            (FormatFeatureFlags::SampledImageYcbcrConversionSeparateReconstructionFilter, "SampledImageYcbcrConversionSeparateReconstructionFilter"),
            (FormatFeatureFlags::SampledImageYcbcrConversionChromaReconstructionExplicit, "SampledImageYcbcrConversionChromaReconstructionExplicit"),
            (FormatFeatureFlags::SampledImageYcbcrConversionChromaReconstructionExplicitForceable, "SampledImageYcbcrConversionChromaReconstructionExplicitForceable"),
            (FormatFeatureFlags::Disjoint, "Disjoint"),
            (FormatFeatureFlags::CositedChromaSamples, "CositedChromaSamples"),
            (FormatFeatureFlags::SampledImageFilterMinmax, "SampledImageFilterMinmax"),
            (FormatFeatureFlags::VideoDecodeOutputKHR, "VideoDecodeOutputKHR"),
            (FormatFeatureFlags::VideoDecodeDpbKHR, "VideoDecodeDpbKHR"),
            (FormatFeatureFlags::AccelerationStructureVertexBufferKHR, "AccelerationStructureVertexBufferKHR"),
            (FormatFeatureFlags::SampledImageFilterCubicEXT, "SampledImageFilterCubicEXT"),
            (FormatFeatureFlags::FragmentDensityMapEXT, "FragmentDensityMapEXT"),
            (FormatFeatureFlags::FragmentShadingRateAttachmentKHR, "FragmentShadingRateAttachmentKHR"),
            (FormatFeatureFlags::VideoEncodeInputKHR, "VideoEncodeInputKHR"),
            (FormatFeatureFlags::VideoEncodeDpbKHR, "VideoEncodeDpbKHR"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for FormatFeatureFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for QueryControlFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(QueryControlFlags, &str)] = &[(QueryControlFlags::Precise, "Precise")];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for QueryControlFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for QueryResultFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(QueryResultFlags, &str)] = &[
            (QueryResultFlags::Type64, "Type64"),
            (QueryResultFlags::Wait, "Wait"),
            (QueryResultFlags::WithAvailability, "WithAvailability"),
            (QueryResultFlags::Partial, "Partial"),
            (QueryResultFlags::WithStatusKHR, "WithStatusKHR"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for QueryResultFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for EventCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(EventCreateFlags, &str)] = &[(EventCreateFlags::DeviceOnly, "DeviceOnly")];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for EventCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for CommandPoolCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(CommandPoolCreateFlags, &str)] = &[
            (CommandPoolCreateFlags::Transient, "Transient"),
            (
                CommandPoolCreateFlags::ResetCommandBuffer,
                "ResetCommandBuffer",
            ),
            (CommandPoolCreateFlags::Protected, "Protected"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for CommandPoolCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for CommandPoolResetFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(CommandPoolResetFlags, &str)] =
            &[(CommandPoolResetFlags::ReleaseResources, "ReleaseResources")];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for CommandPoolResetFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for CommandBufferResetFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(CommandBufferResetFlags, &str)] = &[(
            CommandBufferResetFlags::ReleaseResources,
            "ReleaseResources",
        )];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for CommandBufferResetFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for CommandBufferUsageFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(CommandBufferUsageFlags, &str)] = &[
            (CommandBufferUsageFlags::OneTimeSubmit, "OneTimeSubmit"),
            (
                CommandBufferUsageFlags::RenderPassContinue,
                "RenderPassContinue",
            ),
            (CommandBufferUsageFlags::SimultaneousUse, "SimultaneousUse"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for CommandBufferUsageFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for QueryPipelineStatisticFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(QueryPipelineStatisticFlags, &str)] = &[
            (
                QueryPipelineStatisticFlags::InputAssemblyVertices,
                "InputAssemblyVertices",
            ),
            (
                QueryPipelineStatisticFlags::InputAssemblyPrimitives,
                "InputAssemblyPrimitives",
            ),
            (
                QueryPipelineStatisticFlags::VertexShaderInvocations,
                "VertexShaderInvocations",
            ),
            (
                QueryPipelineStatisticFlags::GeometryShaderInvocations,
                "GeometryShaderInvocations",
            ),
            (
                QueryPipelineStatisticFlags::GeometryShaderPrimitives,
                "GeometryShaderPrimitives",
            ),
            (
                QueryPipelineStatisticFlags::ClippingInvocations,
                "ClippingInvocations",
            ),
            (
                QueryPipelineStatisticFlags::ClippingPrimitives,
                "ClippingPrimitives",
            ),
            (
                QueryPipelineStatisticFlags::FragmentShaderInvocations,
                "FragmentShaderInvocations",
            ),
            (
                QueryPipelineStatisticFlags::TessellationControlShaderPatches,
                "TessellationControlShaderPatches",
            ),
            (
                QueryPipelineStatisticFlags::TessellationEvaluationShaderInvocations,
                "TessellationEvaluationShaderInvocations",
            ),
            (
                QueryPipelineStatisticFlags::ComputeShaderInvocations,
                "ComputeShaderInvocations",
            ),
            (
                QueryPipelineStatisticFlags::TaskShaderInvocationsEXT,
                "TaskShaderInvocationsEXT",
            ),
            (
                QueryPipelineStatisticFlags::MeshShaderInvocationsEXT,
                "MeshShaderInvocationsEXT",
            ),
            (
                QueryPipelineStatisticFlags::ClusterCullingShaderInvocationsHUAWEI,
                "ClusterCullingShaderInvocationsHUAWEI",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for QueryPipelineStatisticFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for MemoryMapFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(MemoryMapFlags, &str)] = &[(MemoryMapFlags::PlacedEXT, "PlacedEXT")];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for MemoryMapFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for MemoryUnmapFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(MemoryUnmapFlags, &str)] = &[(MemoryUnmapFlags::ReserveEXT, "ReserveEXT")];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for MemoryUnmapFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for ImageAspectFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(ImageAspectFlags, &str)] = &[
            (ImageAspectFlags::Color, "Color"),
            (ImageAspectFlags::Depth, "Depth"),
            (ImageAspectFlags::Stencil, "Stencil"),
            (ImageAspectFlags::Metadata, "Metadata"),
            (ImageAspectFlags::Plane0, "Plane0"),
            (ImageAspectFlags::Plane1, "Plane1"),
            (ImageAspectFlags::Plane2, "Plane2"),
            (ImageAspectFlags::None, "None"),
            (ImageAspectFlags::MemoryPlane0EXT, "MemoryPlane0EXT"),
            (ImageAspectFlags::MemoryPlane1EXT, "MemoryPlane1EXT"),
            (ImageAspectFlags::MemoryPlane2EXT, "MemoryPlane2EXT"),
            (ImageAspectFlags::MemoryPlane3EXT, "MemoryPlane3EXT"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for ImageAspectFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for SparseMemoryBindFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(SparseMemoryBindFlags, &str)] =
            &[(SparseMemoryBindFlags::Metadata, "Metadata")];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for SparseMemoryBindFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for SparseImageFormatFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(SparseImageFormatFlags, &str)] = &[
            (SparseImageFormatFlags::SingleMiptail, "SingleMiptail"),
            (SparseImageFormatFlags::AlignedMipSize, "AlignedMipSize"),
            (
                SparseImageFormatFlags::NonstandardBlockSize,
                "NonstandardBlockSize",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for SparseImageFormatFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for SubpassDescriptionFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(SubpassDescriptionFlags, &str)] = &[
            (
                SubpassDescriptionFlags::PerViewAttributesNVX,
                "PerViewAttributesNVX",
            ),
            (
                SubpassDescriptionFlags::PerViewPositionXOnlyNVX,
                "PerViewPositionXOnlyNVX",
            ),
            (
                SubpassDescriptionFlags::TileShadingApronQCOM,
                "TileShadingApronQCOM",
            ),
            (
                SubpassDescriptionFlags::RasterizationOrderAttachmentColorAccessEXT,
                "RasterizationOrderAttachmentColorAccessEXT",
            ),
            (
                SubpassDescriptionFlags::RasterizationOrderAttachmentDepthAccessEXT,
                "RasterizationOrderAttachmentDepthAccessEXT",
            ),
            (
                SubpassDescriptionFlags::RasterizationOrderAttachmentStencilAccessEXT,
                "RasterizationOrderAttachmentStencilAccessEXT",
            ),
            (
                SubpassDescriptionFlags::EnableLegacyDitheringEXT,
                "EnableLegacyDitheringEXT",
            ),
            (
                SubpassDescriptionFlags::FragmentRegionEXT,
                "FragmentRegionEXT",
            ),
            (
                SubpassDescriptionFlags::CustomResolveEXT,
                "CustomResolveEXT",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for SubpassDescriptionFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for PipelineStageFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(PipelineStageFlags, &str)] = &[
            (PipelineStageFlags::TopOfPipe, "TopOfPipe"),
            (PipelineStageFlags::DrawIndirect, "DrawIndirect"),
            (PipelineStageFlags::VertexInput, "VertexInput"),
            (PipelineStageFlags::VertexShader, "VertexShader"),
            (
                PipelineStageFlags::TessellationControlShader,
                "TessellationControlShader",
            ),
            (
                PipelineStageFlags::TessellationEvaluationShader,
                "TessellationEvaluationShader",
            ),
            (PipelineStageFlags::GeometryShader, "GeometryShader"),
            (PipelineStageFlags::FragmentShader, "FragmentShader"),
            (PipelineStageFlags::EarlyFragmentTests, "EarlyFragmentTests"),
            (PipelineStageFlags::LateFragmentTests, "LateFragmentTests"),
            (
                PipelineStageFlags::ColorAttachmentOutput,
                "ColorAttachmentOutput",
            ),
            (PipelineStageFlags::ComputeShader, "ComputeShader"),
            (PipelineStageFlags::Transfer, "Transfer"),
            (PipelineStageFlags::BottomOfPipe, "BottomOfPipe"),
            (PipelineStageFlags::Host, "Host"),
            (PipelineStageFlags::AllGraphics, "AllGraphics"),
            (PipelineStageFlags::AllCommands, "AllCommands"),
            (PipelineStageFlags::None, "None"),
            (
                PipelineStageFlags::TransformFeedbackEXT,
                "TransformFeedbackEXT",
            ),
            (
                PipelineStageFlags::ConditionalRenderingEXT,
                "ConditionalRenderingEXT",
            ),
            (
                PipelineStageFlags::AccelerationStructureBuildKHR,
                "AccelerationStructureBuildKHR",
            ),
            (
                PipelineStageFlags::RayTracingShaderKHR,
                "RayTracingShaderKHR",
            ),
            (
                PipelineStageFlags::FragmentDensityProcessEXT,
                "FragmentDensityProcessEXT",
            ),
            (
                PipelineStageFlags::FragmentShadingRateAttachmentKHR,
                "FragmentShadingRateAttachmentKHR",
            ),
            (PipelineStageFlags::TaskShaderEXT, "TaskShaderEXT"),
            (PipelineStageFlags::MeshShaderEXT, "MeshShaderEXT"),
            (
                PipelineStageFlags::CommandPreprocessEXT,
                "CommandPreprocessEXT",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for PipelineStageFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for SampleCountFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(SampleCountFlags, &str)] = &[
            (SampleCountFlags::Type1, "Type1"),
            (SampleCountFlags::Type2, "Type2"),
            (SampleCountFlags::Type4, "Type4"),
            (SampleCountFlags::Type8, "Type8"),
            (SampleCountFlags::Type16, "Type16"),
            (SampleCountFlags::Type32, "Type32"),
            (SampleCountFlags::Type64, "Type64"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for SampleCountFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for AttachmentDescriptionFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(AttachmentDescriptionFlags, &str)] = &[
            (AttachmentDescriptionFlags::MayAlias, "MayAlias"),
            (
                AttachmentDescriptionFlags::ResolveSkipTransferFunctionKHR,
                "ResolveSkipTransferFunctionKHR",
            ),
            (
                AttachmentDescriptionFlags::ResolveEnableTransferFunctionKHR,
                "ResolveEnableTransferFunctionKHR",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for AttachmentDescriptionFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for StencilFaceFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(StencilFaceFlags, &str)] = &[
            (StencilFaceFlags::Front, "Front"),
            (StencilFaceFlags::Back, "Back"),
            (StencilFaceFlags::FrontAndBack, "FrontAndBack"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for StencilFaceFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for CullModeFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(CullModeFlags, &str)] = &[
            (CullModeFlags::None, "None"),
            (CullModeFlags::Front, "Front"),
            (CullModeFlags::Back, "Back"),
            (CullModeFlags::FrontAndBack, "FrontAndBack"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for CullModeFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for DescriptorPoolCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(DescriptorPoolCreateFlags, &str)] = &[
            (
                DescriptorPoolCreateFlags::FreeDescriptorSet,
                "FreeDescriptorSet",
            ),
            (
                DescriptorPoolCreateFlags::UpdateAfterBind,
                "UpdateAfterBind",
            ),
            (DescriptorPoolCreateFlags::HostOnlyEXT, "HostOnlyEXT"),
            (
                DescriptorPoolCreateFlags::AllowOverallocationSetsNV,
                "AllowOverallocationSetsNV",
            ),
            (
                DescriptorPoolCreateFlags::AllowOverallocationPoolsNV,
                "AllowOverallocationPoolsNV",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for DescriptorPoolCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for DependencyFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(DependencyFlags, &str)] = &[
            (DependencyFlags::ByRegion, "ByRegion"),
            (DependencyFlags::DeviceGroup, "DeviceGroup"),
            (DependencyFlags::ViewLocal, "ViewLocal"),
            (DependencyFlags::FeedbackLoopEXT, "FeedbackLoopEXT"),
            (
                DependencyFlags::QueueFamilyOwnershipTransferUseAllStagesKHR,
                "QueueFamilyOwnershipTransferUseAllStagesKHR",
            ),
            (DependencyFlags::AsymmetricEventKHR, "AsymmetricEventKHR"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for DependencyFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for SubgroupFeatureFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(SubgroupFeatureFlags, &str)] = &[
            (SubgroupFeatureFlags::Basic, "Basic"),
            (SubgroupFeatureFlags::Vote, "Vote"),
            (SubgroupFeatureFlags::Arithmetic, "Arithmetic"),
            (SubgroupFeatureFlags::Ballot, "Ballot"),
            (SubgroupFeatureFlags::Shuffle, "Shuffle"),
            (SubgroupFeatureFlags::ShuffleRelative, "ShuffleRelative"),
            (SubgroupFeatureFlags::Clustered, "Clustered"),
            (SubgroupFeatureFlags::Quad, "Quad"),
            (SubgroupFeatureFlags::Rotate, "Rotate"),
            (SubgroupFeatureFlags::RotateClustered, "RotateClustered"),
            (SubgroupFeatureFlags::PartitionedEXT, "PartitionedEXT"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for SubgroupFeatureFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for IndirectCommandsLayoutUsageFlagsNV {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(IndirectCommandsLayoutUsageFlagsNV, &str)] = &[
            (
                IndirectCommandsLayoutUsageFlagsNV::ExplicitPreprocess,
                "ExplicitPreprocess",
            ),
            (
                IndirectCommandsLayoutUsageFlagsNV::IndexedSequences,
                "IndexedSequences",
            ),
            (
                IndirectCommandsLayoutUsageFlagsNV::UnorderedSequences,
                "UnorderedSequences",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for IndirectCommandsLayoutUsageFlagsNV {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for IndirectStateFlagsNV {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(IndirectStateFlagsNV, &str)] =
            &[(IndirectStateFlagsNV::FlagFrontface, "FlagFrontface")];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for IndirectStateFlagsNV {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for GeometryFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(GeometryFlagsKHR, &str)] = &[
            (GeometryFlagsKHR::Opaque, "Opaque"),
            (
                GeometryFlagsKHR::NoDuplicateAnyHitInvocation,
                "NoDuplicateAnyHitInvocation",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for GeometryFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for GeometryInstanceFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(GeometryInstanceFlagsKHR, &str)] = &[
            (
                GeometryInstanceFlagsKHR::TriangleFacingCullDisable,
                "TriangleFacingCullDisable",
            ),
            (
                GeometryInstanceFlagsKHR::TriangleFlipFacing,
                "TriangleFlipFacing",
            ),
            (GeometryInstanceFlagsKHR::ForceOpaque, "ForceOpaque"),
            (GeometryInstanceFlagsKHR::ForceNoOpaque, "ForceNoOpaque"),
            (
                GeometryInstanceFlagsKHR::ForceOpacityMicromap2State,
                "ForceOpacityMicromap2State",
            ),
            (
                GeometryInstanceFlagsKHR::DisableOpacityMicromaps,
                "DisableOpacityMicromaps",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for GeometryInstanceFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for ClusterAccelerationStructureGeometryFlagsNV {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(ClusterAccelerationStructureGeometryFlagsNV, &str)] = &[
            (
                ClusterAccelerationStructureGeometryFlagsNV::CullDisable,
                "CullDisable",
            ),
            (
                ClusterAccelerationStructureGeometryFlagsNV::NoDuplicateAnyhitInvocation,
                "NoDuplicateAnyhitInvocation",
            ),
            (
                ClusterAccelerationStructureGeometryFlagsNV::Opaque,
                "Opaque",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for ClusterAccelerationStructureGeometryFlagsNV {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for ClusterAccelerationStructureClusterFlagsNV {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(ClusterAccelerationStructureClusterFlagsNV, &str)] = &[(
            ClusterAccelerationStructureClusterFlagsNV::AllowDisableOpacityMicromaps,
            "AllowDisableOpacityMicromaps",
        )];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for ClusterAccelerationStructureClusterFlagsNV {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for ClusterAccelerationStructureAddressResolutionFlagsNV {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(ClusterAccelerationStructureAddressResolutionFlagsNV, &str)] = &[
            (
                ClusterAccelerationStructureAddressResolutionFlagsNV::None,
                "None",
            ),
            (
                ClusterAccelerationStructureAddressResolutionFlagsNV::IndirectedDstImplicitData,
                "IndirectedDstImplicitData",
            ),
            (
                ClusterAccelerationStructureAddressResolutionFlagsNV::IndirectedScratchData,
                "IndirectedScratchData",
            ),
            (
                ClusterAccelerationStructureAddressResolutionFlagsNV::IndirectedDstAddressArray,
                "IndirectedDstAddressArray",
            ),
            (
                ClusterAccelerationStructureAddressResolutionFlagsNV::IndirectedDstSizesArray,
                "IndirectedDstSizesArray",
            ),
            (
                ClusterAccelerationStructureAddressResolutionFlagsNV::IndirectedSrcInfosArray,
                "IndirectedSrcInfosArray",
            ),
            (
                ClusterAccelerationStructureAddressResolutionFlagsNV::IndirectedSrcInfosCount,
                "IndirectedSrcInfosCount",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for ClusterAccelerationStructureAddressResolutionFlagsNV {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for BuildAccelerationStructureFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(BuildAccelerationStructureFlagsKHR, &str)] = &[
            (
                BuildAccelerationStructureFlagsKHR::AllowUpdate,
                "AllowUpdate",
            ),
            (
                BuildAccelerationStructureFlagsKHR::AllowCompaction,
                "AllowCompaction",
            ),
            (
                BuildAccelerationStructureFlagsKHR::PreferFastTrace,
                "PreferFastTrace",
            ),
            (
                BuildAccelerationStructureFlagsKHR::PreferFastBuild,
                "PreferFastBuild",
            ),
            (BuildAccelerationStructureFlagsKHR::LowMemory, "LowMemory"),
            (BuildAccelerationStructureFlagsKHR::Motion, "Motion"),
            (
                BuildAccelerationStructureFlagsKHR::AllowOpacityMicromapUpdate,
                "AllowOpacityMicromapUpdate",
            ),
            (
                BuildAccelerationStructureFlagsKHR::AllowDisableOpacityMicromaps,
                "AllowDisableOpacityMicromaps",
            ),
            (
                BuildAccelerationStructureFlagsKHR::AllowOpacityMicromapDataUpdate,
                "AllowOpacityMicromapDataUpdate",
            ),
            (
                BuildAccelerationStructureFlagsKHR::AllowDisplacementMicromapUpdate,
                "AllowDisplacementMicromapUpdate",
            ),
            (
                BuildAccelerationStructureFlagsKHR::AllowDataAccess,
                "AllowDataAccess",
            ),
            (
                BuildAccelerationStructureFlagsKHR::AllowClusterOpacityMicromaps,
                "AllowClusterOpacityMicromaps",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for BuildAccelerationStructureFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for AccelerationStructureCreateFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(AccelerationStructureCreateFlagsKHR, &str)] = &[
            (
                AccelerationStructureCreateFlagsKHR::DeviceAddressCaptureReplay,
                "DeviceAddressCaptureReplay",
            ),
            (
                AccelerationStructureCreateFlagsKHR::DescriptorBufferCaptureReplay,
                "DescriptorBufferCaptureReplay",
            ),
            (AccelerationStructureCreateFlagsKHR::Motion, "Motion"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for AccelerationStructureCreateFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for PipelineCreationFeedbackFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(PipelineCreationFeedbackFlags, &str)] = &[
            (PipelineCreationFeedbackFlags::Valid, "Valid"),
            (
                PipelineCreationFeedbackFlags::ApplicationPipelineCacheHit,
                "ApplicationPipelineCacheHit",
            ),
            (
                PipelineCreationFeedbackFlags::BasePipelineAcceleration,
                "BasePipelineAcceleration",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for PipelineCreationFeedbackFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for PerformanceCounterDescriptionFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(PerformanceCounterDescriptionFlagsKHR, &str)] = &[
            (
                PerformanceCounterDescriptionFlagsKHR::PerformanceImpacting,
                "PerformanceImpacting",
            ),
            (
                PerformanceCounterDescriptionFlagsKHR::ConcurrentlyImpacted,
                "ConcurrentlyImpacted",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for PerformanceCounterDescriptionFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for AcquireProfilingLockFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("0")
    }
}

impl Debug for AcquireProfilingLockFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for SemaphoreWaitFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(SemaphoreWaitFlags, &str)] = &[(SemaphoreWaitFlags::Any, "Any")];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for SemaphoreWaitFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for PipelineCompilerControlFlagsAMD {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("0")
    }
}

impl Debug for PipelineCompilerControlFlagsAMD {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for ShaderCorePropertiesFlagsAMD {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("0")
    }
}

impl Debug for ShaderCorePropertiesFlagsAMD {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for DeviceDiagnosticsConfigFlagsNV {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(DeviceDiagnosticsConfigFlagsNV, &str)] = &[
            (
                DeviceDiagnosticsConfigFlagsNV::EnableShaderDebugInfo,
                "EnableShaderDebugInfo",
            ),
            (
                DeviceDiagnosticsConfigFlagsNV::EnableResourceTracking,
                "EnableResourceTracking",
            ),
            (
                DeviceDiagnosticsConfigFlagsNV::EnableAutomaticCheckpoints,
                "EnableAutomaticCheckpoints",
            ),
            (
                DeviceDiagnosticsConfigFlagsNV::EnableShaderErrorReporting,
                "EnableShaderErrorReporting",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for DeviceDiagnosticsConfigFlagsNV {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for RefreshObjectFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("0")
    }
}

impl Debug for RefreshObjectFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for AccessFlags2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(AccessFlags2, &str)] = &[
            (AccessFlags2::None, "None"),
            (AccessFlags2::IndirectCommandRead, "IndirectCommandRead"),
            (AccessFlags2::IndexRead, "IndexRead"),
            (AccessFlags2::VertexAttributeRead, "VertexAttributeRead"),
            (AccessFlags2::UniformRead, "UniformRead"),
            (AccessFlags2::InputAttachmentRead, "InputAttachmentRead"),
            (AccessFlags2::ShaderRead, "ShaderRead"),
            (AccessFlags2::ShaderWrite, "ShaderWrite"),
            (AccessFlags2::ColorAttachmentRead, "ColorAttachmentRead"),
            (AccessFlags2::ColorAttachmentWrite, "ColorAttachmentWrite"),
            (
                AccessFlags2::DepthStencilAttachmentRead,
                "DepthStencilAttachmentRead",
            ),
            (
                AccessFlags2::DepthStencilAttachmentWrite,
                "DepthStencilAttachmentWrite",
            ),
            (AccessFlags2::TransferRead, "TransferRead"),
            (AccessFlags2::TransferWrite, "TransferWrite"),
            (AccessFlags2::HostRead, "HostRead"),
            (AccessFlags2::HostWrite, "HostWrite"),
            (AccessFlags2::MemoryRead, "MemoryRead"),
            (AccessFlags2::MemoryWrite, "MemoryWrite"),
            (AccessFlags2::ShaderSampledRead, "ShaderSampledRead"),
            (AccessFlags2::ShaderStorageRead, "ShaderStorageRead"),
            (AccessFlags2::ShaderStorageWrite, "ShaderStorageWrite"),
            (AccessFlags2::VideoDecodeReadKHR, "VideoDecodeReadKHR"),
            (AccessFlags2::VideoDecodeWriteKHR, "VideoDecodeWriteKHR"),
            (AccessFlags2::SamplerHeapReadEXT, "SamplerHeapReadEXT"),
            (AccessFlags2::ResourceHeapReadEXT, "ResourceHeapReadEXT"),
            (AccessFlags2::VideoEncodeReadKHR, "VideoEncodeReadKHR"),
            (AccessFlags2::VideoEncodeWriteKHR, "VideoEncodeWriteKHR"),
            (
                AccessFlags2::ShaderTileAttachmentReadQCOM,
                "ShaderTileAttachmentReadQCOM",
            ),
            (
                AccessFlags2::ShaderTileAttachmentWriteQCOM,
                "ShaderTileAttachmentWriteQCOM",
            ),
            (
                AccessFlags2::TransformFeedbackWriteEXT,
                "TransformFeedbackWriteEXT",
            ),
            (
                AccessFlags2::TransformFeedbackCounterReadEXT,
                "TransformFeedbackCounterReadEXT",
            ),
            (
                AccessFlags2::TransformFeedbackCounterWriteEXT,
                "TransformFeedbackCounterWriteEXT",
            ),
            (
                AccessFlags2::ConditionalRenderingReadEXT,
                "ConditionalRenderingReadEXT",
            ),
            (
                AccessFlags2::CommandPreprocessReadEXT,
                "CommandPreprocessReadEXT",
            ),
            (
                AccessFlags2::CommandPreprocessWriteEXT,
                "CommandPreprocessWriteEXT",
            ),
            (
                AccessFlags2::FragmentShadingRateAttachmentReadKHR,
                "FragmentShadingRateAttachmentReadKHR",
            ),
            (
                AccessFlags2::AccelerationStructureReadKHR,
                "AccelerationStructureReadKHR",
            ),
            (
                AccessFlags2::AccelerationStructureWriteKHR,
                "AccelerationStructureWriteKHR",
            ),
            (
                AccessFlags2::FragmentDensityMapReadEXT,
                "FragmentDensityMapReadEXT",
            ),
            (
                AccessFlags2::ColorAttachmentReadNoncoherentEXT,
                "ColorAttachmentReadNoncoherentEXT",
            ),
            (
                AccessFlags2::DescriptorBufferReadEXT,
                "DescriptorBufferReadEXT",
            ),
            (
                AccessFlags2::InvocationMaskReadHUAWEI,
                "InvocationMaskReadHUAWEI",
            ),
            (
                AccessFlags2::ShaderBindingTableReadKHR,
                "ShaderBindingTableReadKHR",
            ),
            (AccessFlags2::MicromapReadEXT, "MicromapReadEXT"),
            (AccessFlags2::MicromapWriteEXT, "MicromapWriteEXT"),
            (AccessFlags2::OpticalFlowReadNV, "OpticalFlowReadNV"),
            (AccessFlags2::OpticalFlowWriteNV, "OpticalFlowWriteNV"),
            (AccessFlags2::DataGraphReadARM, "DataGraphReadARM"),
            (AccessFlags2::DataGraphWriteARM, "DataGraphWriteARM"),
            (
                AccessFlags2::MemoryDecompressionReadEXT,
                "MemoryDecompressionReadEXT",
            ),
            (
                AccessFlags2::MemoryDecompressionWriteEXT,
                "MemoryDecompressionWriteEXT",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for AccessFlags2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for PipelineStageFlags2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(PipelineStageFlags2, &str)] = &[
            (PipelineStageFlags2::None, "None"),
            (PipelineStageFlags2::TopOfPipe, "TopOfPipe"),
            (PipelineStageFlags2::DrawIndirect, "DrawIndirect"),
            (PipelineStageFlags2::VertexInput, "VertexInput"),
            (PipelineStageFlags2::VertexShader, "VertexShader"),
            (
                PipelineStageFlags2::TessellationControlShader,
                "TessellationControlShader",
            ),
            (
                PipelineStageFlags2::TessellationEvaluationShader,
                "TessellationEvaluationShader",
            ),
            (PipelineStageFlags2::GeometryShader, "GeometryShader"),
            (PipelineStageFlags2::FragmentShader, "FragmentShader"),
            (
                PipelineStageFlags2::EarlyFragmentTests,
                "EarlyFragmentTests",
            ),
            (PipelineStageFlags2::LateFragmentTests, "LateFragmentTests"),
            (
                PipelineStageFlags2::ColorAttachmentOutput,
                "ColorAttachmentOutput",
            ),
            (PipelineStageFlags2::ComputeShader, "ComputeShader"),
            (PipelineStageFlags2::AllTransfer, "AllTransfer"),
            (PipelineStageFlags2::BottomOfPipe, "BottomOfPipe"),
            (PipelineStageFlags2::Host, "Host"),
            (PipelineStageFlags2::AllGraphics, "AllGraphics"),
            (PipelineStageFlags2::AllCommands, "AllCommands"),
            (PipelineStageFlags2::Copy, "Copy"),
            (PipelineStageFlags2::Resolve, "Resolve"),
            (PipelineStageFlags2::Blit, "Blit"),
            (PipelineStageFlags2::Clear, "Clear"),
            (PipelineStageFlags2::IndexInput, "IndexInput"),
            (
                PipelineStageFlags2::VertexAttributeInput,
                "VertexAttributeInput",
            ),
            (
                PipelineStageFlags2::PreRasterizationShaders,
                "PreRasterizationShaders",
            ),
            (PipelineStageFlags2::VideoDecodeKHR, "VideoDecodeKHR"),
            (PipelineStageFlags2::VideoEncodeKHR, "VideoEncodeKHR"),
            (
                PipelineStageFlags2::TransformFeedbackEXT,
                "TransformFeedbackEXT",
            ),
            (
                PipelineStageFlags2::ConditionalRenderingEXT,
                "ConditionalRenderingEXT",
            ),
            (
                PipelineStageFlags2::CommandPreprocessEXT,
                "CommandPreprocessEXT",
            ),
            (
                PipelineStageFlags2::FragmentShadingRateAttachmentKHR,
                "FragmentShadingRateAttachmentKHR",
            ),
            (
                PipelineStageFlags2::AccelerationStructureBuildKHR,
                "AccelerationStructureBuildKHR",
            ),
            (
                PipelineStageFlags2::RayTracingShaderKHR,
                "RayTracingShaderKHR",
            ),
            (
                PipelineStageFlags2::FragmentDensityProcessEXT,
                "FragmentDensityProcessEXT",
            ),
            (PipelineStageFlags2::TaskShaderEXT, "TaskShaderEXT"),
            (PipelineStageFlags2::MeshShaderEXT, "MeshShaderEXT"),
            (
                PipelineStageFlags2::SubpassShaderHUAWEI,
                "SubpassShaderHUAWEI",
            ),
            (
                PipelineStageFlags2::InvocationMaskHUAWEI,
                "InvocationMaskHUAWEI",
            ),
            (
                PipelineStageFlags2::AccelerationStructureCopyKHR,
                "AccelerationStructureCopyKHR",
            ),
            (PipelineStageFlags2::MicromapBuildEXT, "MicromapBuildEXT"),
            (
                PipelineStageFlags2::ClusterCullingShaderHUAWEI,
                "ClusterCullingShaderHUAWEI",
            ),
            (PipelineStageFlags2::OpticalFlowNV, "OpticalFlowNV"),
            (
                PipelineStageFlags2::ConvertCooperativeVectorMatrixNV,
                "ConvertCooperativeVectorMatrixNV",
            ),
            (PipelineStageFlags2::DataGraphARM, "DataGraphARM"),
            (PipelineStageFlags2::CopyIndirectKHR, "CopyIndirectKHR"),
            (
                PipelineStageFlags2::MemoryDecompressionEXT,
                "MemoryDecompressionEXT",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for PipelineStageFlags2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for FormatFeatureFlags2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(FormatFeatureFlags2, &str)] = &[
            (FormatFeatureFlags2::SampledImage, "SampledImage"),
            (FormatFeatureFlags2::StorageImage, "StorageImage"),
            (FormatFeatureFlags2::StorageImageAtomic, "StorageImageAtomic"),
            (FormatFeatureFlags2::UniformTexelBuffer, "UniformTexelBuffer"),
            (FormatFeatureFlags2::StorageTexelBuffer, "StorageTexelBuffer"),
            (FormatFeatureFlags2::StorageTexelBufferAtomic, "StorageTexelBufferAtomic"),
            (FormatFeatureFlags2::VertexBuffer, "VertexBuffer"),
            (FormatFeatureFlags2::ColorAttachment, "ColorAttachment"),
            (FormatFeatureFlags2::ColorAttachmentBlend, "ColorAttachmentBlend"),
            (FormatFeatureFlags2::DepthStencilAttachment, "DepthStencilAttachment"),
            (FormatFeatureFlags2::BlitSrc, "BlitSrc"),
            (FormatFeatureFlags2::BlitDst, "BlitDst"),
            (FormatFeatureFlags2::SampledImageFilterLinear, "SampledImageFilterLinear"),
            (FormatFeatureFlags2::TransferSrc, "TransferSrc"),
            (FormatFeatureFlags2::TransferDst, "TransferDst"),
            (FormatFeatureFlags2::SampledImageFilterMinmax, "SampledImageFilterMinmax"),
            (FormatFeatureFlags2::MidpointChromaSamples, "MidpointChromaSamples"),
            (FormatFeatureFlags2::SampledImageYcbcrConversionLinearFilter, "SampledImageYcbcrConversionLinearFilter"),
            (FormatFeatureFlags2::SampledImageYcbcrConversionSeparateReconstructionFilter, "SampledImageYcbcrConversionSeparateReconstructionFilter"),
            (FormatFeatureFlags2::SampledImageYcbcrConversionChromaReconstructionExplicit, "SampledImageYcbcrConversionChromaReconstructionExplicit"),
            (FormatFeatureFlags2::SampledImageYcbcrConversionChromaReconstructionExplicitForceable, "SampledImageYcbcrConversionChromaReconstructionExplicitForceable"),
            (FormatFeatureFlags2::Disjoint, "Disjoint"),
            (FormatFeatureFlags2::CositedChromaSamples, "CositedChromaSamples"),
            (FormatFeatureFlags2::StorageReadWithoutFormat, "StorageReadWithoutFormat"),
            (FormatFeatureFlags2::StorageWriteWithoutFormat, "StorageWriteWithoutFormat"),
            (FormatFeatureFlags2::SampledImageDepthComparison, "SampledImageDepthComparison"),
            (FormatFeatureFlags2::SampledImageFilterCubic, "SampledImageFilterCubic"),
            (FormatFeatureFlags2::HostImageTransfer, "HostImageTransfer"),
            (FormatFeatureFlags2::VideoDecodeOutputKHR, "VideoDecodeOutputKHR"),
            (FormatFeatureFlags2::VideoDecodeDpbKHR, "VideoDecodeDpbKHR"),
            (FormatFeatureFlags2::AccelerationStructureVertexBufferKHR, "AccelerationStructureVertexBufferKHR"),
            (FormatFeatureFlags2::FragmentDensityMapEXT, "FragmentDensityMapEXT"),
            (FormatFeatureFlags2::FragmentShadingRateAttachmentKHR, "FragmentShadingRateAttachmentKHR"),
            (FormatFeatureFlags2::VideoEncodeInputKHR, "VideoEncodeInputKHR"),
            (FormatFeatureFlags2::VideoEncodeDpbKHR, "VideoEncodeDpbKHR"),
            (FormatFeatureFlags2::AccelerationStructureRadiusBufferNV, "AccelerationStructureRadiusBufferNV"),
            (FormatFeatureFlags2::LinearColorAttachmentNV, "LinearColorAttachmentNV"),
            (FormatFeatureFlags2::WeightImageQCOM, "WeightImageQCOM"),
            (FormatFeatureFlags2::WeightSampledImageQCOM, "WeightSampledImageQCOM"),
            (FormatFeatureFlags2::BlockMatchingQCOM, "BlockMatchingQCOM"),
            (FormatFeatureFlags2::BoxFilterSampledQCOM, "BoxFilterSampledQCOM"),
            (FormatFeatureFlags2::TensorShaderARM, "TensorShaderARM"),
            (FormatFeatureFlags2::TensorImageAliasingARM, "TensorImageAliasingARM"),
            (FormatFeatureFlags2::OpticalFlowImageNV, "OpticalFlowImageNV"),
            (FormatFeatureFlags2::OpticalFlowVectorNV, "OpticalFlowVectorNV"),
            (FormatFeatureFlags2::OpticalFlowCostNV, "OpticalFlowCostNV"),
            (FormatFeatureFlags2::TensorDataGraphARM, "TensorDataGraphARM"),
            (FormatFeatureFlags2::CopyImageIndirectDstKHR, "CopyImageIndirectDstKHR"),
            (FormatFeatureFlags2::VideoEncodeQuantizationDeltaMapKHR, "VideoEncodeQuantizationDeltaMapKHR"),
            (FormatFeatureFlags2::VideoEncodeEmphasisMapKHR, "VideoEncodeEmphasisMapKHR"),
            (FormatFeatureFlags2::DepthCopyOnComputeQueueKHR, "DepthCopyOnComputeQueueKHR"),
            (FormatFeatureFlags2::DepthCopyOnTransferQueueKHR, "DepthCopyOnTransferQueueKHR"),
            (FormatFeatureFlags2::StencilCopyOnComputeQueueKHR, "StencilCopyOnComputeQueueKHR"),
            (FormatFeatureFlags2::StencilCopyOnTransferQueueKHR, "StencilCopyOnTransferQueueKHR"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for FormatFeatureFlags2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for RenderingFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(RenderingFlags, &str)] = &[
            (
                RenderingFlags::ContentsSecondaryCommandBuffers,
                "ContentsSecondaryCommandBuffers",
            ),
            (RenderingFlags::Suspending, "Suspending"),
            (RenderingFlags::Resuming, "Resuming"),
            (
                RenderingFlags::EnableLegacyDitheringEXT,
                "EnableLegacyDitheringEXT",
            ),
            (RenderingFlags::ContentsInlineKHR, "ContentsInlineKHR"),
            (
                RenderingFlags::PerLayerFragmentDensityVALVE,
                "PerLayerFragmentDensityVALVE",
            ),
            (RenderingFlags::FragmentRegionEXT, "FragmentRegionEXT"),
            (RenderingFlags::CustomResolveEXT, "CustomResolveEXT"),
            (
                RenderingFlags::LocalReadConcurrentAccessControlKHR,
                "LocalReadConcurrentAccessControlKHR",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for RenderingFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for MemoryDecompressionMethodFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(MemoryDecompressionMethodFlagsEXT, &str)] =
            &[(MemoryDecompressionMethodFlagsEXT::Gdeflate10, "Gdeflate10")];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for MemoryDecompressionMethodFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for BuildMicromapFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(BuildMicromapFlagsEXT, &str)] = &[
            (BuildMicromapFlagsEXT::PreferFastTrace, "PreferFastTrace"),
            (BuildMicromapFlagsEXT::PreferFastBuild, "PreferFastBuild"),
            (BuildMicromapFlagsEXT::AllowCompaction, "AllowCompaction"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for BuildMicromapFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for MicromapCreateFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(MicromapCreateFlagsEXT, &str)] = &[(
            MicromapCreateFlagsEXT::DeviceAddressCaptureReplay,
            "DeviceAddressCaptureReplay",
        )];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for MicromapCreateFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for IndirectCommandsLayoutUsageFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(IndirectCommandsLayoutUsageFlagsEXT, &str)] = &[
            (
                IndirectCommandsLayoutUsageFlagsEXT::ExplicitPreprocess,
                "ExplicitPreprocess",
            ),
            (
                IndirectCommandsLayoutUsageFlagsEXT::UnorderedSequences,
                "UnorderedSequences",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for IndirectCommandsLayoutUsageFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for IndirectCommandsInputModeFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(IndirectCommandsInputModeFlagsEXT, &str)] = &[
            (
                IndirectCommandsInputModeFlagsEXT::VulkanIndexBuffer,
                "VulkanIndexBuffer",
            ),
            (
                IndirectCommandsInputModeFlagsEXT::DxgiIndexBuffer,
                "DxgiIndexBuffer",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for IndirectCommandsInputModeFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for PipelineCreateFlags2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(PipelineCreateFlags2, &str)] = &[
            (
                PipelineCreateFlags2::DisableOptimization,
                "DisableOptimization",
            ),
            (PipelineCreateFlags2::AllowDerivatives, "AllowDerivatives"),
            (PipelineCreateFlags2::Derivative, "Derivative"),
            (
                PipelineCreateFlags2::ViewIndexFromDeviceIndex,
                "ViewIndexFromDeviceIndex",
            ),
            (PipelineCreateFlags2::DispatchBase, "DispatchBase"),
            (
                PipelineCreateFlags2::FailOnPipelineCompileRequired,
                "FailOnPipelineCompileRequired",
            ),
            (
                PipelineCreateFlags2::EarlyReturnOnFailure,
                "EarlyReturnOnFailure",
            ),
            (PipelineCreateFlags2::NoProtectedAccess, "NoProtectedAccess"),
            (
                PipelineCreateFlags2::ProtectedAccessOnly,
                "ProtectedAccessOnly",
            ),
            (
                PipelineCreateFlags2::ExecutionGraphAMDX,
                "ExecutionGraphAMDX",
            ),
            (PipelineCreateFlags2::DescriptorHeapEXT, "DescriptorHeapEXT"),
            (
                PipelineCreateFlags2::RayTracingAllowSpheresAndLinearSweptSpheresNV,
                "RayTracingAllowSpheresAndLinearSweptSpheresNV",
            ),
            (
                PipelineCreateFlags2::EnableLegacyDitheringEXT,
                "EnableLegacyDitheringEXT",
            ),
            (PipelineCreateFlags2::DeferCompileNV, "DeferCompileNV"),
            (
                PipelineCreateFlags2::CaptureStatisticsKHR,
                "CaptureStatisticsKHR",
            ),
            (
                PipelineCreateFlags2::CaptureInternalRepresentationsKHR,
                "CaptureInternalRepresentationsKHR",
            ),
            (
                PipelineCreateFlags2::LinkTimeOptimizationEXT,
                "LinkTimeOptimizationEXT",
            ),
            (
                PipelineCreateFlags2::RetainLinkTimeOptimizationInfoEXT,
                "RetainLinkTimeOptimizationInfoEXT",
            ),
            (PipelineCreateFlags2::LibraryKHR, "LibraryKHR"),
            (
                PipelineCreateFlags2::RayTracingSkipTrianglesKHR,
                "RayTracingSkipTrianglesKHR",
            ),
            (
                PipelineCreateFlags2::RayTracingSkipAabbsKHR,
                "RayTracingSkipAabbsKHR",
            ),
            (
                PipelineCreateFlags2::RayTracingNoNullAnyHitShadersKHR,
                "RayTracingNoNullAnyHitShadersKHR",
            ),
            (
                PipelineCreateFlags2::RayTracingNoNullClosestHitShadersKHR,
                "RayTracingNoNullClosestHitShadersKHR",
            ),
            (
                PipelineCreateFlags2::RayTracingNoNullMissShadersKHR,
                "RayTracingNoNullMissShadersKHR",
            ),
            (
                PipelineCreateFlags2::RayTracingNoNullIntersectionShadersKHR,
                "RayTracingNoNullIntersectionShadersKHR",
            ),
            (
                PipelineCreateFlags2::RayTracingShaderGroupHandleCaptureReplayKHR,
                "RayTracingShaderGroupHandleCaptureReplayKHR",
            ),
            (
                PipelineCreateFlags2::IndirectBindableNV,
                "IndirectBindableNV",
            ),
            (
                PipelineCreateFlags2::RayTracingAllowMotionNV,
                "RayTracingAllowMotionNV",
            ),
            (
                PipelineCreateFlags2::RenderingFragmentShadingRateAttachmentKHR,
                "RenderingFragmentShadingRateAttachmentKHR",
            ),
            (
                PipelineCreateFlags2::RenderingFragmentDensityMapAttachmentEXT,
                "RenderingFragmentDensityMapAttachmentEXT",
            ),
            (
                PipelineCreateFlags2::RayTracingOpacityMicromapEXT,
                "RayTracingOpacityMicromapEXT",
            ),
            (
                PipelineCreateFlags2::ColorAttachmentFeedbackLoopEXT,
                "ColorAttachmentFeedbackLoopEXT",
            ),
            (
                PipelineCreateFlags2::DepthStencilAttachmentFeedbackLoopEXT,
                "DepthStencilAttachmentFeedbackLoopEXT",
            ),
            (
                PipelineCreateFlags2::RayTracingDisplacementMicromapNV,
                "RayTracingDisplacementMicromapNV",
            ),
            (
                PipelineCreateFlags2::DescriptorBufferEXT,
                "DescriptorBufferEXT",
            ),
            (
                PipelineCreateFlags2::DisallowOpacityMicromapARM,
                "DisallowOpacityMicromapARM",
            ),
            (PipelineCreateFlags2::CaptureDataKHR, "CaptureDataKHR"),
            (
                PipelineCreateFlags2::IndirectBindableEXT,
                "IndirectBindableEXT",
            ),
            (
                PipelineCreateFlags2::PerLayerFragmentDensityVALVE,
                "PerLayerFragmentDensityVALVE",
            ),
            (
                PipelineCreateFlags2::Type64BitIndexingEXT,
                "Type64BitIndexingEXT",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for PipelineCreateFlags2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for BufferUsageFlags2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(BufferUsageFlags2, &str)] = &[
            (BufferUsageFlags2::TransferSrc, "TransferSrc"),
            (BufferUsageFlags2::TransferDst, "TransferDst"),
            (BufferUsageFlags2::UniformTexelBuffer, "UniformTexelBuffer"),
            (BufferUsageFlags2::StorageTexelBuffer, "StorageTexelBuffer"),
            (BufferUsageFlags2::UniformBuffer, "UniformBuffer"),
            (BufferUsageFlags2::StorageBuffer, "StorageBuffer"),
            (BufferUsageFlags2::IndexBuffer, "IndexBuffer"),
            (BufferUsageFlags2::VertexBuffer, "VertexBuffer"),
            (BufferUsageFlags2::IndirectBuffer, "IndirectBuffer"),
            (
                BufferUsageFlags2::ShaderDeviceAddress,
                "ShaderDeviceAddress",
            ),
            (
                BufferUsageFlags2::ExecutionGraphScratchAMDX,
                "ExecutionGraphScratchAMDX",
            ),
            (BufferUsageFlags2::DescriptorHeapEXT, "DescriptorHeapEXT"),
            (
                BufferUsageFlags2::ConditionalRenderingEXT,
                "ConditionalRenderingEXT",
            ),
            (
                BufferUsageFlags2::ShaderBindingTableKHR,
                "ShaderBindingTableKHR",
            ),
            (
                BufferUsageFlags2::TransformFeedbackBufferEXT,
                "TransformFeedbackBufferEXT",
            ),
            (
                BufferUsageFlags2::TransformFeedbackCounterBufferEXT,
                "TransformFeedbackCounterBufferEXT",
            ),
            (BufferUsageFlags2::VideoDecodeSrcKHR, "VideoDecodeSrcKHR"),
            (BufferUsageFlags2::VideoDecodeDstKHR, "VideoDecodeDstKHR"),
            (BufferUsageFlags2::VideoEncodeDstKHR, "VideoEncodeDstKHR"),
            (BufferUsageFlags2::VideoEncodeSrcKHR, "VideoEncodeSrcKHR"),
            (
                BufferUsageFlags2::AccelerationStructureBuildInputReadOnlyKHR,
                "AccelerationStructureBuildInputReadOnlyKHR",
            ),
            (
                BufferUsageFlags2::AccelerationStructureStorageKHR,
                "AccelerationStructureStorageKHR",
            ),
            (
                BufferUsageFlags2::SamplerDescriptorBufferEXT,
                "SamplerDescriptorBufferEXT",
            ),
            (
                BufferUsageFlags2::ResourceDescriptorBufferEXT,
                "ResourceDescriptorBufferEXT",
            ),
            (
                BufferUsageFlags2::PushDescriptorsDescriptorBufferEXT,
                "PushDescriptorsDescriptorBufferEXT",
            ),
            (
                BufferUsageFlags2::MicromapBuildInputReadOnlyEXT,
                "MicromapBuildInputReadOnlyEXT",
            ),
            (BufferUsageFlags2::MicromapStorageEXT, "MicromapStorageEXT"),
            (
                BufferUsageFlags2::CompressedDataDgf1AMDX,
                "CompressedDataDgf1AMDX",
            ),
            (
                BufferUsageFlags2::DataGraphForeignDescriptorARM,
                "DataGraphForeignDescriptorARM",
            ),
            (BufferUsageFlags2::TileMemoryQCOM, "TileMemoryQCOM"),
            (
                BufferUsageFlags2::MemoryDecompressionEXT,
                "MemoryDecompressionEXT",
            ),
            (
                BufferUsageFlags2::PreprocessBufferEXT,
                "PreprocessBufferEXT",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for BufferUsageFlags2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for AddressCopyFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(AddressCopyFlagsKHR, &str)] = &[
            (AddressCopyFlagsKHR::DeviceLocal, "DeviceLocal"),
            (AddressCopyFlagsKHR::Sparse, "Sparse"),
            (AddressCopyFlagsKHR::Protected, "Protected"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for AddressCopyFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for TensorCreateFlagsARM {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(TensorCreateFlagsARM, &str)] = &[
            (TensorCreateFlagsARM::MutableFormat, "MutableFormat"),
            (TensorCreateFlagsARM::Protected, "Protected"),
            (
                TensorCreateFlagsARM::DescriptorHeapCaptureReplay,
                "DescriptorHeapCaptureReplay",
            ),
            (
                TensorCreateFlagsARM::DescriptorBufferCaptureReplay,
                "DescriptorBufferCaptureReplay",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for TensorCreateFlagsARM {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for TensorUsageFlagsARM {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(TensorUsageFlagsARM, &str)] = &[
            (TensorUsageFlagsARM::Shader, "Shader"),
            (TensorUsageFlagsARM::TransferSrc, "TransferSrc"),
            (TensorUsageFlagsARM::TransferDst, "TransferDst"),
            (TensorUsageFlagsARM::ImageAliasing, "ImageAliasing"),
            (TensorUsageFlagsARM::DataGraph, "DataGraph"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for TensorUsageFlagsARM {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for TensorViewCreateFlagsARM {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(TensorViewCreateFlagsARM, &str)] = &[(
            TensorViewCreateFlagsARM::DescriptorBufferCaptureReplay,
            "DescriptorBufferCaptureReplay",
        )];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for TensorViewCreateFlagsARM {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for DataGraphPipelineSessionCreateFlagsARM {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(DataGraphPipelineSessionCreateFlagsARM, &str)] = &[(
            DataGraphPipelineSessionCreateFlagsARM::Protected,
            "Protected",
        )];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for DataGraphPipelineSessionCreateFlagsARM {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for DataGraphPipelineDispatchFlagsARM {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("0")
    }
}

impl Debug for DataGraphPipelineDispatchFlagsARM {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for VideoEncodeRgbModelConversionFlagsVALVE {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(VideoEncodeRgbModelConversionFlagsVALVE, &str)] = &[
            (
                VideoEncodeRgbModelConversionFlagsVALVE::RgbIdentity,
                "RgbIdentity",
            ),
            (
                VideoEncodeRgbModelConversionFlagsVALVE::YcbcrIdentity,
                "YcbcrIdentity",
            ),
            (
                VideoEncodeRgbModelConversionFlagsVALVE::Ycbcr709,
                "Ycbcr709",
            ),
            (
                VideoEncodeRgbModelConversionFlagsVALVE::Ycbcr601,
                "Ycbcr601",
            ),
            (
                VideoEncodeRgbModelConversionFlagsVALVE::Ycbcr2020,
                "Ycbcr2020",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for VideoEncodeRgbModelConversionFlagsVALVE {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for VideoEncodeRgbRangeCompressionFlagsVALVE {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(VideoEncodeRgbRangeCompressionFlagsVALVE, &str)] = &[
            (
                VideoEncodeRgbRangeCompressionFlagsVALVE::FullRange,
                "FullRange",
            ),
            (
                VideoEncodeRgbRangeCompressionFlagsVALVE::NarrowRange,
                "NarrowRange",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for VideoEncodeRgbRangeCompressionFlagsVALVE {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for VideoEncodeRgbChromaOffsetFlagsVALVE {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(VideoEncodeRgbChromaOffsetFlagsVALVE, &str)] = &[
            (
                VideoEncodeRgbChromaOffsetFlagsVALVE::CositedEven,
                "CositedEven",
            ),
            (VideoEncodeRgbChromaOffsetFlagsVALVE::Midpoint, "Midpoint"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for VideoEncodeRgbChromaOffsetFlagsVALVE {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for SpirvResourceTypeFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(SpirvResourceTypeFlagsEXT, &str)] = &[
            (SpirvResourceTypeFlagsEXT::All, "All"),
            (SpirvResourceTypeFlagsEXT::Sampler, "Sampler"),
            (SpirvResourceTypeFlagsEXT::SampledImage, "SampledImage"),
            (SpirvResourceTypeFlagsEXT::ReadOnlyImage, "ReadOnlyImage"),
            (SpirvResourceTypeFlagsEXT::ReadWriteImage, "ReadWriteImage"),
            (
                SpirvResourceTypeFlagsEXT::CombinedSampledImage,
                "CombinedSampledImage",
            ),
            (SpirvResourceTypeFlagsEXT::UniformBuffer, "UniformBuffer"),
            (
                SpirvResourceTypeFlagsEXT::ReadOnlyStorageBuffer,
                "ReadOnlyStorageBuffer",
            ),
            (
                SpirvResourceTypeFlagsEXT::ReadWriteStorageBuffer,
                "ReadWriteStorageBuffer",
            ),
            (
                SpirvResourceTypeFlagsEXT::AccelerationStructure,
                "AccelerationStructure",
            ),
            (SpirvResourceTypeFlagsEXT::Tensor, "Tensor"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for SpirvResourceTypeFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for CompositeAlphaFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(CompositeAlphaFlagsKHR, &str)] = &[
            (CompositeAlphaFlagsKHR::Opaque, "Opaque"),
            (CompositeAlphaFlagsKHR::PreMultiplied, "PreMultiplied"),
            (CompositeAlphaFlagsKHR::PostMultiplied, "PostMultiplied"),
            (CompositeAlphaFlagsKHR::Inherit, "Inherit"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for CompositeAlphaFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for DisplayPlaneAlphaFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(DisplayPlaneAlphaFlagsKHR, &str)] = &[
            (DisplayPlaneAlphaFlagsKHR::Opaque, "Opaque"),
            (DisplayPlaneAlphaFlagsKHR::Global, "Global"),
            (DisplayPlaneAlphaFlagsKHR::PerPixel, "PerPixel"),
            (
                DisplayPlaneAlphaFlagsKHR::PerPixelPremultiplied,
                "PerPixelPremultiplied",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for DisplayPlaneAlphaFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for SurfaceTransformFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(SurfaceTransformFlagsKHR, &str)] = &[
            (SurfaceTransformFlagsKHR::Identity, "Identity"),
            (SurfaceTransformFlagsKHR::Rotate90, "Rotate90"),
            (SurfaceTransformFlagsKHR::Rotate180, "Rotate180"),
            (SurfaceTransformFlagsKHR::Rotate270, "Rotate270"),
            (
                SurfaceTransformFlagsKHR::HorizontalMirror,
                "HorizontalMirror",
            ),
            (
                SurfaceTransformFlagsKHR::HorizontalMirrorRotate90,
                "HorizontalMirrorRotate90",
            ),
            (
                SurfaceTransformFlagsKHR::HorizontalMirrorRotate180,
                "HorizontalMirrorRotate180",
            ),
            (
                SurfaceTransformFlagsKHR::HorizontalMirrorRotate270,
                "HorizontalMirrorRotate270",
            ),
            (SurfaceTransformFlagsKHR::Inherit, "Inherit"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for SurfaceTransformFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for SwapchainCreateFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(SwapchainCreateFlagsKHR, &str)] = &[
            (
                SwapchainCreateFlagsKHR::SplitInstanceBindRegions,
                "SplitInstanceBindRegions",
            ),
            (SwapchainCreateFlagsKHR::Protected, "Protected"),
            (SwapchainCreateFlagsKHR::MutableFormat, "MutableFormat"),
            (SwapchainCreateFlagsKHR::PresentTiming, "PresentTiming"),
            (SwapchainCreateFlagsKHR::PresentId2, "PresentId2"),
            (SwapchainCreateFlagsKHR::PresentWait2, "PresentWait2"),
            (
                SwapchainCreateFlagsKHR::DeferredMemoryAllocation,
                "DeferredMemoryAllocation",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for SwapchainCreateFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for PeerMemoryFeatureFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(PeerMemoryFeatureFlags, &str)] = &[
            (PeerMemoryFeatureFlags::CopySrc, "CopySrc"),
            (PeerMemoryFeatureFlags::CopyDst, "CopyDst"),
            (PeerMemoryFeatureFlags::GenericSrc, "GenericSrc"),
            (PeerMemoryFeatureFlags::GenericDst, "GenericDst"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for PeerMemoryFeatureFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for MemoryAllocateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(MemoryAllocateFlags, &str)] = &[
            (MemoryAllocateFlags::DeviceMask, "DeviceMask"),
            (MemoryAllocateFlags::DeviceAddress, "DeviceAddress"),
            (
                MemoryAllocateFlags::DeviceAddressCaptureReplay,
                "DeviceAddressCaptureReplay",
            ),
            (MemoryAllocateFlags::ZeroInitializeEXT, "ZeroInitializeEXT"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for MemoryAllocateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for DeviceGroupPresentModeFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(DeviceGroupPresentModeFlagsKHR, &str)] = &[
            (DeviceGroupPresentModeFlagsKHR::Local, "Local"),
            (DeviceGroupPresentModeFlagsKHR::Remote, "Remote"),
            (DeviceGroupPresentModeFlagsKHR::Sum, "Sum"),
            (
                DeviceGroupPresentModeFlagsKHR::LocalMultiDevice,
                "LocalMultiDevice",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for DeviceGroupPresentModeFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for DebugReportFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(DebugReportFlagsEXT, &str)] = &[
            (DebugReportFlagsEXT::Information, "Information"),
            (DebugReportFlagsEXT::Warning, "Warning"),
            (
                DebugReportFlagsEXT::PerformanceWarning,
                "PerformanceWarning",
            ),
            (DebugReportFlagsEXT::Error, "Error"),
            (DebugReportFlagsEXT::Debug, "Debug"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for DebugReportFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for ExternalMemoryHandleTypeFlagsNV {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(ExternalMemoryHandleTypeFlagsNV, &str)] = &[
            (ExternalMemoryHandleTypeFlagsNV::OpaqueWin32, "OpaqueWin32"),
            (
                ExternalMemoryHandleTypeFlagsNV::OpaqueWin32Kmt,
                "OpaqueWin32Kmt",
            ),
            (ExternalMemoryHandleTypeFlagsNV::D3d11Image, "D3d11Image"),
            (
                ExternalMemoryHandleTypeFlagsNV::D3d11ImageKmt,
                "D3d11ImageKmt",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for ExternalMemoryHandleTypeFlagsNV {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for ClusterAccelerationStructureIndexFormatFlagsNV {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(ClusterAccelerationStructureIndexFormatFlagsNV, &str)] = &[
            (
                ClusterAccelerationStructureIndexFormatFlagsNV::Type8bit,
                "Type8bit",
            ),
            (
                ClusterAccelerationStructureIndexFormatFlagsNV::Type16bit,
                "Type16bit",
            ),
            (
                ClusterAccelerationStructureIndexFormatFlagsNV::Type32bit,
                "Type32bit",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for ClusterAccelerationStructureIndexFormatFlagsNV {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for ExternalMemoryFeatureFlagsNV {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(ExternalMemoryFeatureFlagsNV, &str)] = &[
            (ExternalMemoryFeatureFlagsNV::DedicatedOnly, "DedicatedOnly"),
            (ExternalMemoryFeatureFlagsNV::Exportable, "Exportable"),
            (ExternalMemoryFeatureFlagsNV::Importable, "Importable"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for ExternalMemoryFeatureFlagsNV {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for ExternalMemoryHandleTypeFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(ExternalMemoryHandleTypeFlags, &str)] = &[
            (ExternalMemoryHandleTypeFlags::OpaqueFd, "OpaqueFd"),
            (ExternalMemoryHandleTypeFlags::OpaqueWin32, "OpaqueWin32"),
            (
                ExternalMemoryHandleTypeFlags::OpaqueWin32Kmt,
                "OpaqueWin32Kmt",
            ),
            (ExternalMemoryHandleTypeFlags::D3d11Texture, "D3d11Texture"),
            (
                ExternalMemoryHandleTypeFlags::D3d11TextureKmt,
                "D3d11TextureKmt",
            ),
            (ExternalMemoryHandleTypeFlags::D3d12Heap, "D3d12Heap"),
            (
                ExternalMemoryHandleTypeFlags::D3d12Resource,
                "D3d12Resource",
            ),
            (ExternalMemoryHandleTypeFlags::DmaBufEXT, "DmaBufEXT"),
            (
                ExternalMemoryHandleTypeFlags::AndroidHardwareBufferANDROID,
                "AndroidHardwareBufferANDROID",
            ),
            (
                ExternalMemoryHandleTypeFlags::HostAllocationEXT,
                "HostAllocationEXT",
            ),
            (
                ExternalMemoryHandleTypeFlags::HostMappedForeignMemoryEXT,
                "HostMappedForeignMemoryEXT",
            ),
            (
                ExternalMemoryHandleTypeFlags::ZirconVmoFUCHSIA,
                "ZirconVmoFUCHSIA",
            ),
            (
                ExternalMemoryHandleTypeFlags::RdmaAddressNV,
                "RdmaAddressNV",
            ),
            (ExternalMemoryHandleTypeFlags::SciBufNV, "SciBufNV"),
            (
                ExternalMemoryHandleTypeFlags::OhNativeBufferOHOS,
                "OhNativeBufferOHOS",
            ),
            (
                ExternalMemoryHandleTypeFlags::ScreenBufferQNX,
                "ScreenBufferQNX",
            ),
            (ExternalMemoryHandleTypeFlags::MtlbufferEXT, "MtlbufferEXT"),
            (
                ExternalMemoryHandleTypeFlags::MtltextureEXT,
                "MtltextureEXT",
            ),
            (ExternalMemoryHandleTypeFlags::MtlheapEXT, "MtlheapEXT"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for ExternalMemoryHandleTypeFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for ExternalMemoryFeatureFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(ExternalMemoryFeatureFlags, &str)] = &[
            (ExternalMemoryFeatureFlags::DedicatedOnly, "DedicatedOnly"),
            (ExternalMemoryFeatureFlags::Exportable, "Exportable"),
            (ExternalMemoryFeatureFlags::Importable, "Importable"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for ExternalMemoryFeatureFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for ExternalSemaphoreHandleTypeFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(ExternalSemaphoreHandleTypeFlags, &str)] = &[
            (ExternalSemaphoreHandleTypeFlags::OpaqueFd, "OpaqueFd"),
            (ExternalSemaphoreHandleTypeFlags::OpaqueWin32, "OpaqueWin32"),
            (
                ExternalSemaphoreHandleTypeFlags::OpaqueWin32Kmt,
                "OpaqueWin32Kmt",
            ),
            (ExternalSemaphoreHandleTypeFlags::D3d12Fence, "D3d12Fence"),
            (ExternalSemaphoreHandleTypeFlags::SyncFd, "SyncFd"),
            (
                ExternalSemaphoreHandleTypeFlags::ZirconEventFUCHSIA,
                "ZirconEventFUCHSIA",
            ),
            (
                ExternalSemaphoreHandleTypeFlags::SciSyncObjNV,
                "SciSyncObjNV",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for ExternalSemaphoreHandleTypeFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for ExternalSemaphoreFeatureFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(ExternalSemaphoreFeatureFlags, &str)] = &[
            (ExternalSemaphoreFeatureFlags::Exportable, "Exportable"),
            (ExternalSemaphoreFeatureFlags::Importable, "Importable"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for ExternalSemaphoreFeatureFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for SemaphoreImportFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(SemaphoreImportFlags, &str)] =
            &[(SemaphoreImportFlags::Temporary, "Temporary")];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for SemaphoreImportFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for ExternalFenceHandleTypeFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(ExternalFenceHandleTypeFlags, &str)] = &[
            (ExternalFenceHandleTypeFlags::OpaqueFd, "OpaqueFd"),
            (ExternalFenceHandleTypeFlags::OpaqueWin32, "OpaqueWin32"),
            (
                ExternalFenceHandleTypeFlags::OpaqueWin32Kmt,
                "OpaqueWin32Kmt",
            ),
            (ExternalFenceHandleTypeFlags::SyncFd, "SyncFd"),
            (ExternalFenceHandleTypeFlags::SciSyncObjNV, "SciSyncObjNV"),
            (
                ExternalFenceHandleTypeFlags::SciSyncFenceNV,
                "SciSyncFenceNV",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for ExternalFenceHandleTypeFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for ExternalFenceFeatureFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(ExternalFenceFeatureFlags, &str)] = &[
            (ExternalFenceFeatureFlags::Exportable, "Exportable"),
            (ExternalFenceFeatureFlags::Importable, "Importable"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for ExternalFenceFeatureFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for FenceImportFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(FenceImportFlags, &str)] = &[(FenceImportFlags::Temporary, "Temporary")];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for FenceImportFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for SurfaceCounterFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(SurfaceCounterFlagsEXT, &str)] =
            &[(SurfaceCounterFlagsEXT::Vblank, "Vblank")];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for SurfaceCounterFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for DebugUtilsMessageSeverityFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(DebugUtilsMessageSeverityFlagsEXT, &str)] = &[
            (DebugUtilsMessageSeverityFlagsEXT::Verbose, "Verbose"),
            (DebugUtilsMessageSeverityFlagsEXT::Info, "Info"),
            (DebugUtilsMessageSeverityFlagsEXT::Warning, "Warning"),
            (DebugUtilsMessageSeverityFlagsEXT::Error, "Error"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for DebugUtilsMessageSeverityFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for DebugUtilsMessageTypeFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(DebugUtilsMessageTypeFlagsEXT, &str)] = &[
            (DebugUtilsMessageTypeFlagsEXT::General, "General"),
            (DebugUtilsMessageTypeFlagsEXT::Validation, "Validation"),
            (DebugUtilsMessageTypeFlagsEXT::Performance, "Performance"),
            (
                DebugUtilsMessageTypeFlagsEXT::DeviceAddressBinding,
                "DeviceAddressBinding",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for DebugUtilsMessageTypeFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for DescriptorBindingFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(DescriptorBindingFlags, &str)] = &[
            (DescriptorBindingFlags::UpdateAfterBind, "UpdateAfterBind"),
            (
                DescriptorBindingFlags::UpdateUnusedWhilePending,
                "UpdateUnusedWhilePending",
            ),
            (DescriptorBindingFlags::PartiallyBound, "PartiallyBound"),
            (
                DescriptorBindingFlags::VariableDescriptorCount,
                "VariableDescriptorCount",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for DescriptorBindingFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for ConditionalRenderingFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(ConditionalRenderingFlagsEXT, &str)] =
            &[(ConditionalRenderingFlagsEXT::Inverted, "Inverted")];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for ConditionalRenderingFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for ResolveModeFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(ResolveModeFlags, &str)] = &[
            (ResolveModeFlags::None, "None"),
            (ResolveModeFlags::SampleZero, "SampleZero"),
            (ResolveModeFlags::Average, "Average"),
            (ResolveModeFlags::Min, "Min"),
            (ResolveModeFlags::Max, "Max"),
            (
                ResolveModeFlags::ExternalFormatDownsampleANDROID,
                "ExternalFormatDownsampleANDROID",
            ),
            (ResolveModeFlags::CustomEXT, "CustomEXT"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for ResolveModeFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for ToolPurposeFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(ToolPurposeFlags, &str)] = &[
            (ToolPurposeFlags::Validation, "Validation"),
            (ToolPurposeFlags::Profiling, "Profiling"),
            (ToolPurposeFlags::Tracing, "Tracing"),
            (ToolPurposeFlags::AdditionalFeatures, "AdditionalFeatures"),
            (ToolPurposeFlags::ModifyingFeatures, "ModifyingFeatures"),
            (ToolPurposeFlags::DebugReportingEXT, "DebugReportingEXT"),
            (ToolPurposeFlags::DebugMarkersEXT, "DebugMarkersEXT"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for ToolPurposeFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for SubmitFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(SubmitFlags, &str)] = &[(SubmitFlags::Protected, "Protected")];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for SubmitFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for HostImageCopyFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(HostImageCopyFlags, &str)] = &[(HostImageCopyFlags::Memcpy, "Memcpy")];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for HostImageCopyFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for PartitionedAccelerationStructureInstanceFlagsNV {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(PartitionedAccelerationStructureInstanceFlagsNV, &str)] = &[
            (
                PartitionedAccelerationStructureInstanceFlagsNV::FlagTriangleFacingCullDisable,
                "FlagTriangleFacingCullDisable",
            ),
            (
                PartitionedAccelerationStructureInstanceFlagsNV::FlagTriangleFlipFacing,
                "FlagTriangleFlipFacing",
            ),
            (
                PartitionedAccelerationStructureInstanceFlagsNV::FlagForceOpaque,
                "FlagForceOpaque",
            ),
            (
                PartitionedAccelerationStructureInstanceFlagsNV::FlagForceNoOpaque,
                "FlagForceNoOpaque",
            ),
            (
                PartitionedAccelerationStructureInstanceFlagsNV::FlagEnableExplicitBoundingBox,
                "FlagEnableExplicitBoundingBox",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for PartitionedAccelerationStructureInstanceFlagsNV {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for ImageConstraintsInfoFlagsFUCHSIA {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(ImageConstraintsInfoFlagsFUCHSIA, &str)] = &[
            (
                ImageConstraintsInfoFlagsFUCHSIA::CpuReadRarely,
                "CpuReadRarely",
            ),
            (
                ImageConstraintsInfoFlagsFUCHSIA::CpuReadOften,
                "CpuReadOften",
            ),
            (
                ImageConstraintsInfoFlagsFUCHSIA::CpuWriteRarely,
                "CpuWriteRarely",
            ),
            (
                ImageConstraintsInfoFlagsFUCHSIA::CpuWriteOften,
                "CpuWriteOften",
            ),
            (
                ImageConstraintsInfoFlagsFUCHSIA::ProtectedOptional,
                "ProtectedOptional",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for ImageConstraintsInfoFlagsFUCHSIA {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for GraphicsPipelineLibraryFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(GraphicsPipelineLibraryFlagsEXT, &str)] = &[
            (
                GraphicsPipelineLibraryFlagsEXT::VertexInputInterface,
                "VertexInputInterface",
            ),
            (
                GraphicsPipelineLibraryFlagsEXT::PreRasterizationShaders,
                "PreRasterizationShaders",
            ),
            (
                GraphicsPipelineLibraryFlagsEXT::FragmentShader,
                "FragmentShader",
            ),
            (
                GraphicsPipelineLibraryFlagsEXT::FragmentOutputInterface,
                "FragmentOutputInterface",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for GraphicsPipelineLibraryFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for ImageCompressionFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(ImageCompressionFlagsEXT, &str)] = &[
            (ImageCompressionFlagsEXT::Default, "Default"),
            (
                ImageCompressionFlagsEXT::FixedRateDefault,
                "FixedRateDefault",
            ),
            (
                ImageCompressionFlagsEXT::FixedRateExplicit,
                "FixedRateExplicit",
            ),
            (ImageCompressionFlagsEXT::Disabled, "Disabled"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for ImageCompressionFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for ImageCompressionFixedRateFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(ImageCompressionFixedRateFlagsEXT, &str)] = &[
            (ImageCompressionFixedRateFlagsEXT::None, "None"),
            (ImageCompressionFixedRateFlagsEXT::Type1bpc, "Type1bpc"),
            (ImageCompressionFixedRateFlagsEXT::Type2bpc, "Type2bpc"),
            (ImageCompressionFixedRateFlagsEXT::Type3bpc, "Type3bpc"),
            (ImageCompressionFixedRateFlagsEXT::Type4bpc, "Type4bpc"),
            (ImageCompressionFixedRateFlagsEXT::Type5bpc, "Type5bpc"),
            (ImageCompressionFixedRateFlagsEXT::Type6bpc, "Type6bpc"),
            (ImageCompressionFixedRateFlagsEXT::Type7bpc, "Type7bpc"),
            (ImageCompressionFixedRateFlagsEXT::Type8bpc, "Type8bpc"),
            (ImageCompressionFixedRateFlagsEXT::Type9bpc, "Type9bpc"),
            (ImageCompressionFixedRateFlagsEXT::Type10bpc, "Type10bpc"),
            (ImageCompressionFixedRateFlagsEXT::Type11bpc, "Type11bpc"),
            (ImageCompressionFixedRateFlagsEXT::Type12bpc, "Type12bpc"),
            (ImageCompressionFixedRateFlagsEXT::Type13bpc, "Type13bpc"),
            (ImageCompressionFixedRateFlagsEXT::Type14bpc, "Type14bpc"),
            (ImageCompressionFixedRateFlagsEXT::Type15bpc, "Type15bpc"),
            (ImageCompressionFixedRateFlagsEXT::Type16bpc, "Type16bpc"),
            (ImageCompressionFixedRateFlagsEXT::Type17bpc, "Type17bpc"),
            (ImageCompressionFixedRateFlagsEXT::Type18bpc, "Type18bpc"),
            (ImageCompressionFixedRateFlagsEXT::Type19bpc, "Type19bpc"),
            (ImageCompressionFixedRateFlagsEXT::Type20bpc, "Type20bpc"),
            (ImageCompressionFixedRateFlagsEXT::Type21bpc, "Type21bpc"),
            (ImageCompressionFixedRateFlagsEXT::Type22bpc, "Type22bpc"),
            (ImageCompressionFixedRateFlagsEXT::Type23bpc, "Type23bpc"),
            (ImageCompressionFixedRateFlagsEXT::Type24bpc, "Type24bpc"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for ImageCompressionFixedRateFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for ExportMetalObjectTypeFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(ExportMetalObjectTypeFlagsEXT, &str)] = &[
            (ExportMetalObjectTypeFlagsEXT::MetalDevice, "MetalDevice"),
            (
                ExportMetalObjectTypeFlagsEXT::MetalCommandQueue,
                "MetalCommandQueue",
            ),
            (ExportMetalObjectTypeFlagsEXT::MetalBuffer, "MetalBuffer"),
            (ExportMetalObjectTypeFlagsEXT::MetalTexture, "MetalTexture"),
            (
                ExportMetalObjectTypeFlagsEXT::MetalIosurface,
                "MetalIosurface",
            ),
            (
                ExportMetalObjectTypeFlagsEXT::MetalSharedEvent,
                "MetalSharedEvent",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for ExportMetalObjectTypeFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for RenderingAttachmentFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(RenderingAttachmentFlagsKHR, &str)] = &[
            (
                RenderingAttachmentFlagsKHR::InputAttachmentFeedback,
                "InputAttachmentFeedback",
            ),
            (
                RenderingAttachmentFlagsKHR::ResolveSkipTransferFunction,
                "ResolveSkipTransferFunction",
            ),
            (
                RenderingAttachmentFlagsKHR::ResolveEnableTransferFunction,
                "ResolveEnableTransferFunction",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for RenderingAttachmentFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for ResolveImageFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(ResolveImageFlagsKHR, &str)] = &[
            (
                ResolveImageFlagsKHR::SkipTransferFunction,
                "SkipTransferFunction",
            ),
            (
                ResolveImageFlagsKHR::EnableTransferFunction,
                "EnableTransferFunction",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for ResolveImageFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for DeviceAddressBindingFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(DeviceAddressBindingFlagsEXT, &str)] = &[(
            DeviceAddressBindingFlagsEXT::InternalObject,
            "InternalObject",
        )];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for DeviceAddressBindingFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for OpticalFlowGridSizeFlagsNV {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(OpticalFlowGridSizeFlagsNV, &str)] = &[
            (OpticalFlowGridSizeFlagsNV::Unknown, "Unknown"),
            (OpticalFlowGridSizeFlagsNV::Type1x1, "Type1x1"),
            (OpticalFlowGridSizeFlagsNV::Type2x2, "Type2x2"),
            (OpticalFlowGridSizeFlagsNV::Type4x4, "Type4x4"),
            (OpticalFlowGridSizeFlagsNV::Type8x8, "Type8x8"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for OpticalFlowGridSizeFlagsNV {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for OpticalFlowUsageFlagsNV {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(OpticalFlowUsageFlagsNV, &str)] = &[
            (OpticalFlowUsageFlagsNV::Unknown, "Unknown"),
            (OpticalFlowUsageFlagsNV::Input, "Input"),
            (OpticalFlowUsageFlagsNV::Output, "Output"),
            (OpticalFlowUsageFlagsNV::Hint, "Hint"),
            (OpticalFlowUsageFlagsNV::Cost, "Cost"),
            (OpticalFlowUsageFlagsNV::GlobalFlow, "GlobalFlow"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for OpticalFlowUsageFlagsNV {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for OpticalFlowSessionCreateFlagsNV {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(OpticalFlowSessionCreateFlagsNV, &str)] = &[
            (OpticalFlowSessionCreateFlagsNV::EnableHint, "EnableHint"),
            (OpticalFlowSessionCreateFlagsNV::EnableCost, "EnableCost"),
            (
                OpticalFlowSessionCreateFlagsNV::EnableGlobalFlow,
                "EnableGlobalFlow",
            ),
            (
                OpticalFlowSessionCreateFlagsNV::AllowRegions,
                "AllowRegions",
            ),
            (
                OpticalFlowSessionCreateFlagsNV::BothDirections,
                "BothDirections",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for OpticalFlowSessionCreateFlagsNV {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for OpticalFlowExecuteFlagsNV {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(OpticalFlowExecuteFlagsNV, &str)] = &[(
            OpticalFlowExecuteFlagsNV::DisableTemporalHints,
            "DisableTemporalHints",
        )];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for OpticalFlowExecuteFlagsNV {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for FrameBoundaryFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(FrameBoundaryFlagsEXT, &str)] =
            &[(FrameBoundaryFlagsEXT::FrameEnd, "FrameEnd")];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for FrameBoundaryFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for PresentScalingFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(PresentScalingFlagsKHR, &str)] = &[
            (PresentScalingFlagsKHR::OneToOne, "OneToOne"),
            (
                PresentScalingFlagsKHR::AspectRatioStretch,
                "AspectRatioStretch",
            ),
            (PresentScalingFlagsKHR::Stretch, "Stretch"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for PresentScalingFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for PresentGravityFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(PresentGravityFlagsKHR, &str)] = &[
            (PresentGravityFlagsKHR::Min, "Min"),
            (PresentGravityFlagsKHR::Max, "Max"),
            (PresentGravityFlagsKHR::Centered, "Centered"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for PresentGravityFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for ShaderCreateFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(ShaderCreateFlagsEXT, &str)] = &[
            (ShaderCreateFlagsEXT::LinkStage, "LinkStage"),
            (ShaderCreateFlagsEXT::DescriptorHeap, "DescriptorHeap"),
            (
                ShaderCreateFlagsEXT::AllowVaryingSubgroupSize,
                "AllowVaryingSubgroupSize",
            ),
            (
                ShaderCreateFlagsEXT::RequireFullSubgroups,
                "RequireFullSubgroups",
            ),
            (ShaderCreateFlagsEXT::NoTaskShader, "NoTaskShader"),
            (ShaderCreateFlagsEXT::DispatchBase, "DispatchBase"),
            (
                ShaderCreateFlagsEXT::FragmentShadingRateAttachment,
                "FragmentShadingRateAttachment",
            ),
            (
                ShaderCreateFlagsEXT::FragmentDensityMapAttachment,
                "FragmentDensityMapAttachment",
            ),
            (ShaderCreateFlagsEXT::IndirectBindable, "IndirectBindable"),
            (ShaderCreateFlagsEXT::Type64BitIndexing, "Type64BitIndexing"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for ShaderCreateFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for TileShadingRenderPassFlagsQCOM {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(TileShadingRenderPassFlagsQCOM, &str)] = &[
            (TileShadingRenderPassFlagsQCOM::Enable, "Enable"),
            (
                TileShadingRenderPassFlagsQCOM::PerTileExecution,
                "PerTileExecution",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for TileShadingRenderPassFlagsQCOM {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for PhysicalDeviceSchedulingControlsFlagsARM {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(PhysicalDeviceSchedulingControlsFlagsARM, &str)] = &[(
            PhysicalDeviceSchedulingControlsFlagsARM::ShaderCoreCount,
            "ShaderCoreCount",
        )];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for PhysicalDeviceSchedulingControlsFlagsARM {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for PresentStageFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(PresentStageFlagsEXT, &str)] = &[
            (
                PresentStageFlagsEXT::QueueOperationsEnd,
                "QueueOperationsEnd",
            ),
            (PresentStageFlagsEXT::RequestDequeued, "RequestDequeued"),
            (
                PresentStageFlagsEXT::ImageFirstPixelOut,
                "ImageFirstPixelOut",
            ),
            (
                PresentStageFlagsEXT::ImageFirstPixelVisible,
                "ImageFirstPixelVisible",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for PresentStageFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for PastPresentationTimingFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(PastPresentationTimingFlagsEXT, &str)] = &[
            (
                PastPresentationTimingFlagsEXT::AllowPartialResults,
                "AllowPartialResults",
            ),
            (
                PastPresentationTimingFlagsEXT::AllowOutOfOrderResults,
                "AllowOutOfOrderResults",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for PastPresentationTimingFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for PresentTimingInfoFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(PresentTimingInfoFlagsEXT, &str)] = &[
            (
                PresentTimingInfoFlagsEXT::PresentAtRelativeTime,
                "PresentAtRelativeTime",
            ),
            (
                PresentTimingInfoFlagsEXT::PresentAtNearestRefreshCycle,
                "PresentAtNearestRefreshCycle",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for PresentTimingInfoFlagsEXT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for VideoCodecOperationFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(VideoCodecOperationFlagsKHR, &str)] = &[
            (VideoCodecOperationFlagsKHR::None, "None"),
            (VideoCodecOperationFlagsKHR::EncodeH264, "EncodeH264"),
            (VideoCodecOperationFlagsKHR::EncodeH265, "EncodeH265"),
            (VideoCodecOperationFlagsKHR::DecodeH264, "DecodeH264"),
            (VideoCodecOperationFlagsKHR::DecodeH265, "DecodeH265"),
            (VideoCodecOperationFlagsKHR::DecodeAv1, "DecodeAv1"),
            (VideoCodecOperationFlagsKHR::EncodeAv1, "EncodeAv1"),
            (VideoCodecOperationFlagsKHR::DecodeVp9, "DecodeVp9"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for VideoCodecOperationFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for VideoCapabilityFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(VideoCapabilityFlagsKHR, &str)] = &[
            (
                VideoCapabilityFlagsKHR::ProtectedContent,
                "ProtectedContent",
            ),
            (
                VideoCapabilityFlagsKHR::SeparateReferenceImages,
                "SeparateReferenceImages",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for VideoCapabilityFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for VideoSessionCreateFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(VideoSessionCreateFlagsKHR, &str)] = &[
            (
                VideoSessionCreateFlagsKHR::ProtectedContent,
                "ProtectedContent",
            ),
            (
                VideoSessionCreateFlagsKHR::AllowEncodeParameterOptimizations,
                "AllowEncodeParameterOptimizations",
            ),
            (VideoSessionCreateFlagsKHR::InlineQueries, "InlineQueries"),
            (
                VideoSessionCreateFlagsKHR::AllowEncodeQuantizationDeltaMap,
                "AllowEncodeQuantizationDeltaMap",
            ),
            (
                VideoSessionCreateFlagsKHR::AllowEncodeEmphasisMap,
                "AllowEncodeEmphasisMap",
            ),
            (
                VideoSessionCreateFlagsKHR::InlineSessionParameters,
                "InlineSessionParameters",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for VideoSessionCreateFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for VideoSessionParametersCreateFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(VideoSessionParametersCreateFlagsKHR, &str)] = &[(
            VideoSessionParametersCreateFlagsKHR::QuantizationMapCompatible,
            "QuantizationMapCompatible",
        )];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for VideoSessionParametersCreateFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for VideoCodingControlFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(VideoCodingControlFlagsKHR, &str)] = &[
            (VideoCodingControlFlagsKHR::Reset, "Reset"),
            (
                VideoCodingControlFlagsKHR::EncodeRateControl,
                "EncodeRateControl",
            ),
            (
                VideoCodingControlFlagsKHR::EncodeQualityLevel,
                "EncodeQualityLevel",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for VideoCodingControlFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for VideoDecodeUsageFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(VideoDecodeUsageFlagsKHR, &str)] = &[
            (VideoDecodeUsageFlagsKHR::Default, "Default"),
            (VideoDecodeUsageFlagsKHR::Transcoding, "Transcoding"),
            (VideoDecodeUsageFlagsKHR::Offline, "Offline"),
            (VideoDecodeUsageFlagsKHR::Streaming, "Streaming"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for VideoDecodeUsageFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for VideoDecodeCapabilityFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(VideoDecodeCapabilityFlagsKHR, &str)] = &[
            (
                VideoDecodeCapabilityFlagsKHR::DpbAndOutputCoincide,
                "DpbAndOutputCoincide",
            ),
            (
                VideoDecodeCapabilityFlagsKHR::DpbAndOutputDistinct,
                "DpbAndOutputDistinct",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for VideoDecodeCapabilityFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for VideoDecodeH264PictureLayoutFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(VideoDecodeH264PictureLayoutFlagsKHR, &str)] = &[
            (
                VideoDecodeH264PictureLayoutFlagsKHR::Progressive,
                "Progressive",
            ),
            (
                VideoDecodeH264PictureLayoutFlagsKHR::InterlacedInterleavedLines,
                "InterlacedInterleavedLines",
            ),
            (
                VideoDecodeH264PictureLayoutFlagsKHR::InterlacedSeparatePlanes,
                "InterlacedSeparatePlanes",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for VideoDecodeH264PictureLayoutFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for VideoEncodeFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(VideoEncodeFlagsKHR, &str)] = &[
            (VideoEncodeFlagsKHR::IntraRefresh, "IntraRefresh"),
            (
                VideoEncodeFlagsKHR::WithQuantizationDeltaMap,
                "WithQuantizationDeltaMap",
            ),
            (VideoEncodeFlagsKHR::WithEmphasisMap, "WithEmphasisMap"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for VideoEncodeFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for VideoEncodeUsageFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(VideoEncodeUsageFlagsKHR, &str)] = &[
            (VideoEncodeUsageFlagsKHR::Default, "Default"),
            (VideoEncodeUsageFlagsKHR::Transcoding, "Transcoding"),
            (VideoEncodeUsageFlagsKHR::Streaming, "Streaming"),
            (VideoEncodeUsageFlagsKHR::Recording, "Recording"),
            (VideoEncodeUsageFlagsKHR::Conferencing, "Conferencing"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for VideoEncodeUsageFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for VideoEncodeContentFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(VideoEncodeContentFlagsKHR, &str)] = &[
            (VideoEncodeContentFlagsKHR::Default, "Default"),
            (VideoEncodeContentFlagsKHR::Camera, "Camera"),
            (VideoEncodeContentFlagsKHR::Desktop, "Desktop"),
            (VideoEncodeContentFlagsKHR::Rendered, "Rendered"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for VideoEncodeContentFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for VideoEncodeCapabilityFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(VideoEncodeCapabilityFlagsKHR, &str)] = &[
            (
                VideoEncodeCapabilityFlagsKHR::PrecedingExternallyEncodedBytes,
                "PrecedingExternallyEncodedBytes",
            ),
            (
                VideoEncodeCapabilityFlagsKHR::InsufficientBitstreamBufferRangeDetection,
                "InsufficientBitstreamBufferRangeDetection",
            ),
            (
                VideoEncodeCapabilityFlagsKHR::QuantizationDeltaMap,
                "QuantizationDeltaMap",
            ),
            (VideoEncodeCapabilityFlagsKHR::EmphasisMap, "EmphasisMap"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for VideoEncodeCapabilityFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for VideoEncodeFeedbackFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(VideoEncodeFeedbackFlagsKHR, &str)] = &[
            (
                VideoEncodeFeedbackFlagsKHR::BitstreamBufferOffset,
                "BitstreamBufferOffset",
            ),
            (
                VideoEncodeFeedbackFlagsKHR::BitstreamBytesWritten,
                "BitstreamBytesWritten",
            ),
            (
                VideoEncodeFeedbackFlagsKHR::BitstreamHasOverrides,
                "BitstreamHasOverrides",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for VideoEncodeFeedbackFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for VideoEncodeRateControlModeFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(VideoEncodeRateControlModeFlagsKHR, &str)] = &[
            (VideoEncodeRateControlModeFlagsKHR::Default, "Default"),
            (VideoEncodeRateControlModeFlagsKHR::Disabled, "Disabled"),
            (VideoEncodeRateControlModeFlagsKHR::Cbr, "Cbr"),
            (VideoEncodeRateControlModeFlagsKHR::Vbr, "Vbr"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for VideoEncodeRateControlModeFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for VideoEncodeIntraRefreshModeFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(VideoEncodeIntraRefreshModeFlagsKHR, &str)] = &[
            (VideoEncodeIntraRefreshModeFlagsKHR::None, "None"),
            (
                VideoEncodeIntraRefreshModeFlagsKHR::PerPicturePartition,
                "PerPicturePartition",
            ),
            (
                VideoEncodeIntraRefreshModeFlagsKHR::BlockBased,
                "BlockBased",
            ),
            (
                VideoEncodeIntraRefreshModeFlagsKHR::BlockRowBased,
                "BlockRowBased",
            ),
            (
                VideoEncodeIntraRefreshModeFlagsKHR::BlockColumnBased,
                "BlockColumnBased",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for VideoEncodeIntraRefreshModeFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for VideoChromaSubsamplingFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(VideoChromaSubsamplingFlagsKHR, &str)] = &[
            (VideoChromaSubsamplingFlagsKHR::Invalid, "Invalid"),
            (VideoChromaSubsamplingFlagsKHR::Monochrome, "Monochrome"),
            (VideoChromaSubsamplingFlagsKHR::Type420, "Type420"),
            (VideoChromaSubsamplingFlagsKHR::Type422, "Type422"),
            (VideoChromaSubsamplingFlagsKHR::Type444, "Type444"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for VideoChromaSubsamplingFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for VideoComponentBitDepthFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(VideoComponentBitDepthFlagsKHR, &str)] = &[
            (VideoComponentBitDepthFlagsKHR::Invalid, "Invalid"),
            (VideoComponentBitDepthFlagsKHR::Type8, "Type8"),
            (VideoComponentBitDepthFlagsKHR::Type10, "Type10"),
            (VideoComponentBitDepthFlagsKHR::Type12, "Type12"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for VideoComponentBitDepthFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for VideoEncodeH264CapabilityFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(VideoEncodeH264CapabilityFlagsKHR, &str)] = &[
            (
                VideoEncodeH264CapabilityFlagsKHR::HrdCompliance,
                "HrdCompliance",
            ),
            (
                VideoEncodeH264CapabilityFlagsKHR::PredictionWeightTableGenerated,
                "PredictionWeightTableGenerated",
            ),
            (
                VideoEncodeH264CapabilityFlagsKHR::RowUnalignedSlice,
                "RowUnalignedSlice",
            ),
            (
                VideoEncodeH264CapabilityFlagsKHR::DifferentSliceType,
                "DifferentSliceType",
            ),
            (
                VideoEncodeH264CapabilityFlagsKHR::BFrameInL0List,
                "BFrameInL0List",
            ),
            (
                VideoEncodeH264CapabilityFlagsKHR::BFrameInL1List,
                "BFrameInL1List",
            ),
            (
                VideoEncodeH264CapabilityFlagsKHR::PerPictureTypeMinMaxQp,
                "PerPictureTypeMinMaxQp",
            ),
            (
                VideoEncodeH264CapabilityFlagsKHR::PerSliceConstantQp,
                "PerSliceConstantQp",
            ),
            (
                VideoEncodeH264CapabilityFlagsKHR::GeneratePrefixNalu,
                "GeneratePrefixNalu",
            ),
            (
                VideoEncodeH264CapabilityFlagsKHR::BPictureIntraRefresh,
                "BPictureIntraRefresh",
            ),
            (
                VideoEncodeH264CapabilityFlagsKHR::MbQpDiffWraparound,
                "MbQpDiffWraparound",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for VideoEncodeH264CapabilityFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for VideoEncodeH264StdFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(VideoEncodeH264StdFlagsKHR, &str)] = &[
            (
                VideoEncodeH264StdFlagsKHR::SeparateColorPlaneFlagSet,
                "SeparateColorPlaneFlagSet",
            ),
            (
                VideoEncodeH264StdFlagsKHR::QpprimeYZeroTransformBypassFlagSet,
                "QpprimeYZeroTransformBypassFlagSet",
            ),
            (
                VideoEncodeH264StdFlagsKHR::ScalingMatrixPresentFlagSet,
                "ScalingMatrixPresentFlagSet",
            ),
            (
                VideoEncodeH264StdFlagsKHR::ChromaQpIndexOffset,
                "ChromaQpIndexOffset",
            ),
            (
                VideoEncodeH264StdFlagsKHR::SecondChromaQpIndexOffset,
                "SecondChromaQpIndexOffset",
            ),
            (
                VideoEncodeH264StdFlagsKHR::PicInitQpMinus26,
                "PicInitQpMinus26",
            ),
            (
                VideoEncodeH264StdFlagsKHR::WeightedPredFlagSet,
                "WeightedPredFlagSet",
            ),
            (
                VideoEncodeH264StdFlagsKHR::WeightedBipredIdcExplicit,
                "WeightedBipredIdcExplicit",
            ),
            (
                VideoEncodeH264StdFlagsKHR::WeightedBipredIdcImplicit,
                "WeightedBipredIdcImplicit",
            ),
            (
                VideoEncodeH264StdFlagsKHR::Transform8x8ModeFlagSet,
                "Transform8x8ModeFlagSet",
            ),
            (
                VideoEncodeH264StdFlagsKHR::DirectSpatialMvPredFlagUnset,
                "DirectSpatialMvPredFlagUnset",
            ),
            (
                VideoEncodeH264StdFlagsKHR::EntropyCodingModeFlagUnset,
                "EntropyCodingModeFlagUnset",
            ),
            (
                VideoEncodeH264StdFlagsKHR::EntropyCodingModeFlagSet,
                "EntropyCodingModeFlagSet",
            ),
            (
                VideoEncodeH264StdFlagsKHR::Direct8x8InferenceFlagUnset,
                "Direct8x8InferenceFlagUnset",
            ),
            (
                VideoEncodeH264StdFlagsKHR::ConstrainedIntraPredFlagSet,
                "ConstrainedIntraPredFlagSet",
            ),
            (
                VideoEncodeH264StdFlagsKHR::DeblockingFilterDisabled,
                "DeblockingFilterDisabled",
            ),
            (
                VideoEncodeH264StdFlagsKHR::DeblockingFilterEnabled,
                "DeblockingFilterEnabled",
            ),
            (
                VideoEncodeH264StdFlagsKHR::DeblockingFilterPartial,
                "DeblockingFilterPartial",
            ),
            (VideoEncodeH264StdFlagsKHR::SliceQpDelta, "SliceQpDelta"),
            (
                VideoEncodeH264StdFlagsKHR::DifferentSliceQpDelta,
                "DifferentSliceQpDelta",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for VideoEncodeH264StdFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for VideoEncodeH264RateControlFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(VideoEncodeH264RateControlFlagsKHR, &str)] = &[
            (
                VideoEncodeH264RateControlFlagsKHR::AttemptHrdCompliance,
                "AttemptHrdCompliance",
            ),
            (VideoEncodeH264RateControlFlagsKHR::RegularGop, "RegularGop"),
            (
                VideoEncodeH264RateControlFlagsKHR::ReferencePatternFlat,
                "ReferencePatternFlat",
            ),
            (
                VideoEncodeH264RateControlFlagsKHR::ReferencePatternDyadic,
                "ReferencePatternDyadic",
            ),
            (
                VideoEncodeH264RateControlFlagsKHR::TemporalLayerPatternDyadic,
                "TemporalLayerPatternDyadic",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for VideoEncodeH264RateControlFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for VideoEncodeH265CapabilityFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(VideoEncodeH265CapabilityFlagsKHR, &str)] = &[
            (
                VideoEncodeH265CapabilityFlagsKHR::HrdCompliance,
                "HrdCompliance",
            ),
            (
                VideoEncodeH265CapabilityFlagsKHR::PredictionWeightTableGenerated,
                "PredictionWeightTableGenerated",
            ),
            (
                VideoEncodeH265CapabilityFlagsKHR::RowUnalignedSliceSegment,
                "RowUnalignedSliceSegment",
            ),
            (
                VideoEncodeH265CapabilityFlagsKHR::DifferentSliceSegmentType,
                "DifferentSliceSegmentType",
            ),
            (
                VideoEncodeH265CapabilityFlagsKHR::BFrameInL0List,
                "BFrameInL0List",
            ),
            (
                VideoEncodeH265CapabilityFlagsKHR::BFrameInL1List,
                "BFrameInL1List",
            ),
            (
                VideoEncodeH265CapabilityFlagsKHR::PerPictureTypeMinMaxQp,
                "PerPictureTypeMinMaxQp",
            ),
            (
                VideoEncodeH265CapabilityFlagsKHR::PerSliceSegmentConstantQp,
                "PerSliceSegmentConstantQp",
            ),
            (
                VideoEncodeH265CapabilityFlagsKHR::MultipleTilesPerSliceSegment,
                "MultipleTilesPerSliceSegment",
            ),
            (
                VideoEncodeH265CapabilityFlagsKHR::MultipleSliceSegmentsPerTile,
                "MultipleSliceSegmentsPerTile",
            ),
            (
                VideoEncodeH265CapabilityFlagsKHR::BPictureIntraRefresh,
                "BPictureIntraRefresh",
            ),
            (
                VideoEncodeH265CapabilityFlagsKHR::CuQpDiffWraparound,
                "CuQpDiffWraparound",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for VideoEncodeH265CapabilityFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for VideoEncodeH265StdFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(VideoEncodeH265StdFlagsKHR, &str)] = &[
            (
                VideoEncodeH265StdFlagsKHR::SeparateColorPlaneFlagSet,
                "SeparateColorPlaneFlagSet",
            ),
            (
                VideoEncodeH265StdFlagsKHR::SampleAdaptiveOffsetEnabledFlagSet,
                "SampleAdaptiveOffsetEnabledFlagSet",
            ),
            (
                VideoEncodeH265StdFlagsKHR::ScalingListDataPresentFlagSet,
                "ScalingListDataPresentFlagSet",
            ),
            (
                VideoEncodeH265StdFlagsKHR::PcmEnabledFlagSet,
                "PcmEnabledFlagSet",
            ),
            (
                VideoEncodeH265StdFlagsKHR::SpsTemporalMvpEnabledFlagSet,
                "SpsTemporalMvpEnabledFlagSet",
            ),
            (VideoEncodeH265StdFlagsKHR::InitQpMinus26, "InitQpMinus26"),
            (
                VideoEncodeH265StdFlagsKHR::WeightedPredFlagSet,
                "WeightedPredFlagSet",
            ),
            (
                VideoEncodeH265StdFlagsKHR::WeightedBipredFlagSet,
                "WeightedBipredFlagSet",
            ),
            (
                VideoEncodeH265StdFlagsKHR::Log2ParallelMergeLevelMinus2,
                "Log2ParallelMergeLevelMinus2",
            ),
            (
                VideoEncodeH265StdFlagsKHR::SignDataHidingEnabledFlagSet,
                "SignDataHidingEnabledFlagSet",
            ),
            (
                VideoEncodeH265StdFlagsKHR::TransformSkipEnabledFlagSet,
                "TransformSkipEnabledFlagSet",
            ),
            (
                VideoEncodeH265StdFlagsKHR::TransformSkipEnabledFlagUnset,
                "TransformSkipEnabledFlagUnset",
            ),
            (
                VideoEncodeH265StdFlagsKHR::PpsSliceChromaQpOffsetsPresentFlagSet,
                "PpsSliceChromaQpOffsetsPresentFlagSet",
            ),
            (
                VideoEncodeH265StdFlagsKHR::TransquantBypassEnabledFlagSet,
                "TransquantBypassEnabledFlagSet",
            ),
            (
                VideoEncodeH265StdFlagsKHR::ConstrainedIntraPredFlagSet,
                "ConstrainedIntraPredFlagSet",
            ),
            (
                VideoEncodeH265StdFlagsKHR::EntropyCodingSyncEnabledFlagSet,
                "EntropyCodingSyncEnabledFlagSet",
            ),
            (
                VideoEncodeH265StdFlagsKHR::DeblockingFilterOverrideEnabledFlagSet,
                "DeblockingFilterOverrideEnabledFlagSet",
            ),
            (
                VideoEncodeH265StdFlagsKHR::DependentSliceSegmentsEnabledFlagSet,
                "DependentSliceSegmentsEnabledFlagSet",
            ),
            (
                VideoEncodeH265StdFlagsKHR::DependentSliceSegmentFlagSet,
                "DependentSliceSegmentFlagSet",
            ),
            (VideoEncodeH265StdFlagsKHR::SliceQpDelta, "SliceQpDelta"),
            (
                VideoEncodeH265StdFlagsKHR::DifferentSliceQpDelta,
                "DifferentSliceQpDelta",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for VideoEncodeH265StdFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for VideoEncodeH265RateControlFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(VideoEncodeH265RateControlFlagsKHR, &str)] = &[
            (
                VideoEncodeH265RateControlFlagsKHR::AttemptHrdCompliance,
                "AttemptHrdCompliance",
            ),
            (VideoEncodeH265RateControlFlagsKHR::RegularGop, "RegularGop"),
            (
                VideoEncodeH265RateControlFlagsKHR::ReferencePatternFlat,
                "ReferencePatternFlat",
            ),
            (
                VideoEncodeH265RateControlFlagsKHR::ReferencePatternDyadic,
                "ReferencePatternDyadic",
            ),
            (
                VideoEncodeH265RateControlFlagsKHR::TemporalSubLayerPatternDyadic,
                "TemporalSubLayerPatternDyadic",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for VideoEncodeH265RateControlFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for VideoEncodeH265CtbSizeFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(VideoEncodeH265CtbSizeFlagsKHR, &str)] = &[
            (VideoEncodeH265CtbSizeFlagsKHR::Type16, "Type16"),
            (VideoEncodeH265CtbSizeFlagsKHR::Type32, "Type32"),
            (VideoEncodeH265CtbSizeFlagsKHR::Type64, "Type64"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for VideoEncodeH265CtbSizeFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for VideoEncodeH265TransformBlockSizeFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(VideoEncodeH265TransformBlockSizeFlagsKHR, &str)] = &[
            (VideoEncodeH265TransformBlockSizeFlagsKHR::Type4, "Type4"),
            (VideoEncodeH265TransformBlockSizeFlagsKHR::Type8, "Type8"),
            (VideoEncodeH265TransformBlockSizeFlagsKHR::Type16, "Type16"),
            (VideoEncodeH265TransformBlockSizeFlagsKHR::Type32, "Type32"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for VideoEncodeH265TransformBlockSizeFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for VideoEncodeAV1CapabilityFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(VideoEncodeAV1CapabilityFlagsKHR, &str)] = &[
            (
                VideoEncodeAV1CapabilityFlagsKHR::PerRateControlGroupMinMaxQIndex,
                "PerRateControlGroupMinMaxQIndex",
            ),
            (
                VideoEncodeAV1CapabilityFlagsKHR::GenerateObuExtensionHeader,
                "GenerateObuExtensionHeader",
            ),
            (
                VideoEncodeAV1CapabilityFlagsKHR::PrimaryReferenceCdfOnly,
                "PrimaryReferenceCdfOnly",
            ),
            (
                VideoEncodeAV1CapabilityFlagsKHR::FrameSizeOverride,
                "FrameSizeOverride",
            ),
            (
                VideoEncodeAV1CapabilityFlagsKHR::MotionVectorScaling,
                "MotionVectorScaling",
            ),
            (
                VideoEncodeAV1CapabilityFlagsKHR::CompoundPredictionIntraRefresh,
                "CompoundPredictionIntraRefresh",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for VideoEncodeAV1CapabilityFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for VideoEncodeAV1StdFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(VideoEncodeAV1StdFlagsKHR, &str)] = &[
            (
                VideoEncodeAV1StdFlagsKHR::UniformTileSpacingFlagSet,
                "UniformTileSpacingFlagSet",
            ),
            (
                VideoEncodeAV1StdFlagsKHR::SkipModePresentUnset,
                "SkipModePresentUnset",
            ),
            (
                VideoEncodeAV1StdFlagsKHR::PrimaryRefFrame,
                "PrimaryRefFrame",
            ),
            (VideoEncodeAV1StdFlagsKHR::DeltaQ, "DeltaQ"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for VideoEncodeAV1StdFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for VideoEncodeAV1RateControlFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(VideoEncodeAV1RateControlFlagsKHR, &str)] = &[
            (VideoEncodeAV1RateControlFlagsKHR::RegularGop, "RegularGop"),
            (
                VideoEncodeAV1RateControlFlagsKHR::TemporalLayerPatternDyadic,
                "TemporalLayerPatternDyadic",
            ),
            (
                VideoEncodeAV1RateControlFlagsKHR::ReferencePatternFlat,
                "ReferencePatternFlat",
            ),
            (
                VideoEncodeAV1RateControlFlagsKHR::ReferencePatternDyadic,
                "ReferencePatternDyadic",
            ),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for VideoEncodeAV1RateControlFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for VideoEncodeAV1SuperblockSizeFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(VideoEncodeAV1SuperblockSizeFlagsKHR, &str)] = &[
            (VideoEncodeAV1SuperblockSizeFlagsKHR::Type64, "Type64"),
            (VideoEncodeAV1SuperblockSizeFlagsKHR::Type128, "Type128"),
        ];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for VideoEncodeAV1SuperblockSizeFlagsKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for AccessFlags3KHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const BITS: &[(AccessFlags3KHR, &str)] = &[(AccessFlags3KHR::None, "None")];

        let mut first = true;
        for &(flag, name) in BITS {
            if self.contains(flag) {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl Debug for AccessFlags3KHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}
