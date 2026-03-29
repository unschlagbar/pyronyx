// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — vk/enums.rs
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

#![allow(non_camel_case_types)]

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH264ChromaFormatIdc.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum H264ChromaFormatIdc {
    #[default]
    Monochrome = 0,
    Type420 = 1,
    Type422 = 2,
    Type444 = 3,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH264ProfileIdc.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum H264ProfileIdc {
    /// Only constrained baseline is supported
    #[default]
    Baseline = 66,
    Main = 77,
    High = 100,
    High444Predictive = 244,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH264LevelIdc.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum H264LevelIdc {
    #[default]
    Type10 = 0,
    Type11 = 1,
    Type12 = 2,
    Type13 = 3,
    Type20 = 4,
    Type21 = 5,
    Type22 = 6,
    Type30 = 7,
    Type31 = 8,
    Type32 = 9,
    Type40 = 10,
    Type41 = 11,
    Type42 = 12,
    Type50 = 13,
    Type51 = 14,
    Type52 = 15,
    Type60 = 16,
    Type61 = 17,
    Type62 = 18,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH264PocType.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum H264PocType {
    #[default]
    Type0 = 0,
    Type1 = 1,
    Type2 = 2,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH264AspectRatioIdc.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum H264AspectRatioIdc {
    #[default]
    Unspecified = 0,
    Square = 1,
    Type1211 = 2,
    Type1011 = 3,
    Type1611 = 4,
    Type4033 = 5,
    Type2411 = 6,
    Type2011 = 7,
    Type3211 = 8,
    Type8033 = 9,
    Type1811 = 10,
    Type1511 = 11,
    Type6433 = 12,
    Type16099 = 13,
    Type43 = 14,
    Type32 = 15,
    Type21 = 16,
    ExtendedSar = 255,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH264WeightedBipredIdc.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum H264WeightedBipredIdc {
    #[default]
    Default = 0,
    Explicit = 1,
    Implicit = 2,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH264ModificationOfPicNumsIdc.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum H264ModificationOfPicNumsIdc {
    #[default]
    ShortTermSubtract = 0,
    ShortTermAdd = 1,
    LongTerm = 2,
    End = 3,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH264MemMgmtControlOp.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum H264MemMgmtControlOp {
    #[default]
    End = 0,
    UnmarkShortTerm = 1,
    UnmarkLongTerm = 2,
    MarkLongTerm = 3,
    SetMaxLongTermIndex = 4,
    UnmarkAll = 5,
    MarkCurrentAsLongTerm = 6,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH264CabacInitIdc.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum H264CabacInitIdc {
    #[default]
    Type0 = 0,
    Type1 = 1,
    Type2 = 2,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH264DisableDeblockingFilterIdc.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum H264DisableDeblockingFilterIdc {
    #[default]
    Disabled = 0,
    Enabled = 1,
    Partial = 2,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH264SliceType.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum H264SliceType {
    #[default]
    P = 0,
    B = 1,
    I = 2,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH264PictureType.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum H264PictureType {
    #[default]
    P = 0,
    B = 1,
    I = 2,
    Idr = 5,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH264NonVclNaluType.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum H264NonVclNaluType {
    #[default]
    Sps = 0,
    Pps = 1,
    Aud = 2,
    Prefix = 3,
    EndOfSequence = 4,
    EndOfStream = 5,
    Precoded = 6,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoDecodeH264FieldOrderCount.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DecodeH264FieldOrderCount {
    #[default]
    Top = 0,
    Bottom = 1,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH265ChromaFormatIdc.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum H265ChromaFormatIdc {
    #[default]
    Monochrome = 0,
    Type420 = 1,
    Type422 = 2,
    Type444 = 3,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH265ProfileIdc.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum H265ProfileIdc {
    #[default]
    Main = 1,
    Main10 = 2,
    MainStillPicture = 3,
    FormatRangeExtensions = 4,
    SccExtensions = 9,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH265LevelIdc.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum H265LevelIdc {
    #[default]
    Type10 = 0,
    Type20 = 1,
    Type21 = 2,
    Type30 = 3,
    Type31 = 4,
    Type40 = 5,
    Type41 = 6,
    Type50 = 7,
    Type51 = 8,
    Type52 = 9,
    Type60 = 10,
    Type61 = 11,
    Type62 = 12,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH265SliceType.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum H265SliceType {
    #[default]
    B = 0,
    P = 1,
    I = 2,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH265PictureType.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum H265PictureType {
    #[default]
    P = 0,
    B = 1,
    I = 2,
    Idr = 3,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoH265AspectRatioIdc.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum H265AspectRatioIdc {
    #[default]
    Unspecified = 0,
    Square = 1,
    Type1211 = 2,
    Type1011 = 3,
    Type1611 = 4,
    Type4033 = 5,
    Type2411 = 6,
    Type2011 = 7,
    Type3211 = 8,
    Type8033 = 9,
    Type1811 = 10,
    Type1511 = 11,
    Type6433 = 12,
    Type16099 = 13,
    Type43 = 14,
    Type32 = 15,
    Type21 = 16,
    ExtendedSar = 255,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoVP9Profile.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum VP9Profile {
    #[default]
    Type0 = 0,
    Type1 = 1,
    Type2 = 2,
    Type3 = 3,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoVP9Level.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum VP9Level {
    #[default]
    Type10 = 0,
    Type11 = 1,
    Type20 = 2,
    Type21 = 3,
    Type30 = 4,
    Type31 = 5,
    Type40 = 6,
    Type41 = 7,
    Type50 = 8,
    Type51 = 9,
    Type52 = 10,
    Type60 = 11,
    Type61 = 12,
    Type62 = 13,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoVP9FrameType.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum VP9FrameType {
    #[default]
    Key = 0,
    NonKey = 1,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoVP9ReferenceName.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum VP9ReferenceName {
    #[default]
    IntraFrame = 0,
    LastFrame = 1,
    GoldenFrame = 2,
    AltrefFrame = 3,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoVP9InterpolationFilter.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum VP9InterpolationFilter {
    #[default]
    Eighttap = 0,
    EighttapSmooth = 1,
    EighttapSharp = 2,
    Bilinear = 3,
    Switchable = 4,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoVP9ColorSpace.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum VP9ColorSpace {
    #[default]
    Unknown = 0,
    Bt601 = 1,
    Bt709 = 2,
    Smpte170 = 3,
    Smpte240 = 4,
    Bt2020 = 5,
    Reserved = 6,
    Rgb = 7,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoAV1Profile.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AV1Profile {
    #[default]
    Main = 0,
    High = 1,
    Professional = 2,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoAV1Level.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AV1Level {
    #[default]
    Type20 = 0,
    Type21 = 1,
    Type22 = 2,
    Type23 = 3,
    Type30 = 4,
    Type31 = 5,
    Type32 = 6,
    Type33 = 7,
    Type40 = 8,
    Type41 = 9,
    Type42 = 10,
    Type43 = 11,
    Type50 = 12,
    Type51 = 13,
    Type52 = 14,
    Type53 = 15,
    Type60 = 16,
    Type61 = 17,
    Type62 = 18,
    Type63 = 19,
    Type70 = 20,
    Type71 = 21,
    Type72 = 22,
    Type73 = 23,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoAV1FrameType.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AV1FrameType {
    #[default]
    Key = 0,
    Inter = 1,
    IntraOnly = 2,
    Switch = 3,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoAV1ReferenceName.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AV1ReferenceName {
    #[default]
    IntraFrame = 0,
    LastFrame = 1,
    Last2Frame = 2,
    Last3Frame = 3,
    GoldenFrame = 4,
    BwdrefFrame = 5,
    Altref2Frame = 6,
    AltrefFrame = 7,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoAV1InterpolationFilter.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AV1InterpolationFilter {
    #[default]
    Eighttap = 0,
    EighttapSmooth = 1,
    EighttapSharp = 2,
    Bilinear = 3,
    Switchable = 4,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoAV1TxMode.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AV1TxMode {
    #[default]
    Only4x4 = 0,
    Largest = 1,
    Select = 2,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoAV1FrameRestorationType.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AV1FrameRestorationType {
    #[default]
    None = 0,
    Wiener = 1,
    Sgrproj = 2,
    Switchable = 3,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoAV1ColorPrimaries.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AV1ColorPrimaries {
    #[default]
    Bt709 = 1,
    Unspecified = 2,
    Bt470M = 4,
    Bt470BG = 5,
    Bt601 = 6,
    Smpte240 = 7,
    GenericFilm = 8,
    Bt2020 = 9,
    Xyz = 10,
    Smpte431 = 11,
    Smpte432 = 12,
    Ebu3213 = 22,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoAV1TransferCharacteristics.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AV1TransferCharacteristics {
    #[default]
    Reserved0 = 0,
    Bt709 = 1,
    Unspecified = 2,
    Reserved3 = 3,
    Bt470M = 4,
    Bt470BG = 5,
    Bt601 = 6,
    Smpte240 = 7,
    Linear = 8,
    Log100 = 9,
    Log100Sqrt10 = 10,
    Iec61966 = 11,
    Bt1361 = 12,
    Srgb = 13,
    Bt202010 = 14,
    Bt202012 = 15,
    Smpte2084 = 16,
    Smpte428 = 17,
    Hlg = 18,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoAV1MatrixCoefficients.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AV1MatrixCoefficients {
    #[default]
    Identity = 0,
    Bt709 = 1,
    Unspecified = 2,
    Reserved3 = 3,
    Fcc = 4,
    Bt470BG = 5,
    Bt601 = 6,
    Smpte240 = 7,
    SmpteYcgco = 8,
    Bt2020Ncl = 9,
    Bt2020Cl = 10,
    Smpte2085 = 11,
    ChromatNcl = 12,
    ChromatCl = 13,
    Ictcp = 14,
    Invalid = 2147483647,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/StdVideoAV1ChromaSamplePosition.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AV1ChromaSamplePosition {
    #[default]
    Unknown = 0,
    Vertical = 1,
    Colocated = 2,
    Reserved = 3,
    Invalid = 2147483647,
}
