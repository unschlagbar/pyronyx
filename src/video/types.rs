// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — vk/types.rs
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

#![allow(non_camel_case_types)]

use super::enums::*;
use core::marker::PhantomData;
use core::{mem, ptr};

// ── Constants ───────────────────────────────────────────────
pub const H264_CPB_CNT_LIST_SIZE: u32 = 32;
pub const H264_SCALING_LIST_4X4_NUM_LISTS: u32 = 6;
pub const H264_SCALING_LIST_4X4_NUM_ELEMENTS: u32 = 16;
pub const H264_SCALING_LIST_8X8_NUM_LISTS: u32 = 6;
pub const H264_SCALING_LIST_8X8_NUM_ELEMENTS: u32 = 64;
pub const H264_MAX_NUM_LIST_REF: u32 = 32;
pub const H264_MAX_CHROMA_PLANES: u32 = 2;
pub const H264_NO_REFERENCE_PICTURE: u8 = 0xFF;
pub const DECODE_H264_FIELD_ORDER_COUNT_LIST_SIZE: u32 = 2;
pub const H265_CPB_CNT_LIST_SIZE: u32 = 32;
pub const H265_SUBLAYERS_LIST_SIZE: u32 = 7;
pub const H265_SCALING_LIST_4X4_NUM_LISTS: u32 = 6;
pub const H265_SCALING_LIST_4X4_NUM_ELEMENTS: u32 = 16;
pub const H265_SCALING_LIST_8X8_NUM_LISTS: u32 = 6;
pub const H265_SCALING_LIST_8X8_NUM_ELEMENTS: u32 = 64;
pub const H265_SCALING_LIST_16X16_NUM_LISTS: u32 = 6;
pub const H265_SCALING_LIST_16X16_NUM_ELEMENTS: u32 = 64;
pub const H265_SCALING_LIST_32X32_NUM_LISTS: u32 = 2;
pub const H265_SCALING_LIST_32X32_NUM_ELEMENTS: u32 = 64;
pub const H265_CHROMA_QP_OFFSET_LIST_SIZE: u32 = 6;
pub const H265_CHROMA_QP_OFFSET_TILE_COLS_LIST_SIZE: u32 = 19;
pub const H265_CHROMA_QP_OFFSET_TILE_ROWS_LIST_SIZE: u32 = 21;
pub const H265_PREDICTOR_PALETTE_COMPONENTS_LIST_SIZE: u32 = 3;
pub const H265_PREDICTOR_PALETTE_COMP_ENTRIES_LIST_SIZE: u32 = 128;
pub const H265_MAX_NUM_LIST_REF: u32 = 15;
pub const H265_MAX_CHROMA_PLANES: u32 = 2;
pub const H265_MAX_SHORT_TERM_REF_PIC_SETS: u32 = 64;
pub const H265_MAX_DPB_SIZE: u32 = 16;
pub const H265_MAX_LONG_TERM_REF_PICS_SPS: u32 = 32;
pub const H265_MAX_LONG_TERM_PICS: u32 = 16;
pub const H265_MAX_DELTA_POC: u32 = 48;
pub const H265_NO_REFERENCE_PICTURE: u8 = 0xFF;
pub const DECODE_H265_REF_PIC_SET_LIST_SIZE: u32 = 8;
pub const VP9_NUM_REF_FRAMES: u32 = 8;
pub const VP9_REFS_PER_FRAME: u32 = 3;
pub const VP9_MAX_REF_FRAMES: u32 = 4;
pub const VP9_LOOP_FILTER_ADJUSTMENTS: u32 = 2;
pub const VP9_MAX_SEGMENTS: u32 = 8;
pub const VP9_SEG_LVL_MAX: u32 = 4;
pub const VP9_MAX_SEGMENTATION_TREE_PROBS: u32 = 7;
pub const VP9_MAX_SEGMENTATION_PRED_PROB: u32 = 3;
pub const AV1_NUM_REF_FRAMES: u32 = 8;
pub const AV1_REFS_PER_FRAME: u32 = 7;
pub const AV1_TOTAL_REFS_PER_FRAME: u32 = 8;
pub const AV1_MAX_TILE_COLS: u32 = 64;
pub const AV1_MAX_TILE_ROWS: u32 = 64;
pub const AV1_MAX_SEGMENTS: u32 = 8;
pub const AV1_SEG_LVL_MAX: u32 = 8;
pub const AV1_PRIMARY_REF_NONE: u8 = 7;
pub const AV1_SELECT_INTEGER_MV: u8 = 2;
pub const AV1_SELECT_SCREEN_CONTENT_TOOLS: u32 = 2;
pub const AV1_SKIP_MODE_FRAMES: u32 = 2;
pub const AV1_MAX_LOOP_FILTER_STRENGTHS: u32 = 4;
pub const AV1_LOOP_FILTER_ADJUSTMENTS: u32 = 2;
pub const AV1_MAX_CDEF_FILTER_STRENGTHS: u32 = 8;
pub const AV1_MAX_NUM_PLANES: u32 = 3;
pub const AV1_GLOBAL_MOTION_PARAMS: u32 = 6;
pub const AV1_MAX_NUM_Y_POINTS: u32 = 14;
pub const AV1_MAX_NUM_CB_POINTS: u32 = 10;
pub const AV1_MAX_NUM_CR_POINTS: u32 = 10;
pub const AV1_MAX_NUM_POS_LUMA: u32 = 24;
pub const AV1_MAX_NUM_POS_CHROMA: u32 = 25;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct H264SpsVuiFlags {
    pub aspect_ratio_info_present_flag: u32,
    pub overscan_info_present_flag: u32,
    pub overscan_appropriate_flag: u32,
    pub video_signal_type_present_flag: u32,
    pub video_full_range_flag: u32,
    pub color_description_present_flag: u32,
    pub chroma_loc_info_present_flag: u32,
    pub timing_info_present_flag: u32,
    pub fixed_frame_rate_flag: u32,
    pub bitstream_restriction_flag: u32,
    pub nal_hrd_parameters_present_flag: u32,
    pub vcl_hrd_parameters_present_flag: u32,
}
impl Default for H264SpsVuiFlags {
    fn default() -> Self {
        Self {
            aspect_ratio_info_present_flag: 0,
            overscan_info_present_flag: 0,
            overscan_appropriate_flag: 0,
            video_signal_type_present_flag: 0,
            video_full_range_flag: 0,
            color_description_present_flag: 0,
            chroma_loc_info_present_flag: 0,
            timing_info_present_flag: 0,
            fixed_frame_rate_flag: 0,
            bitstream_restriction_flag: 0,
            nal_hrd_parameters_present_flag: 0,
            vcl_hrd_parameters_present_flag: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct H264HrdParameters {
    pub cpb_cnt_minus1: u8,
    pub bit_rate_scale: u8,
    pub cpb_size_scale: u8,
    /// Reserved for future use and must be initialized with 0.
    pub reserved1: u8,
    /// cpb_cnt_minus1 number of valid elements
    pub bit_rate_value_minus1: [u32; H264_CPB_CNT_LIST_SIZE as usize],
    /// cpb_cnt_minus1 number of valid elements
    pub cpb_size_value_minus1: [u32; H264_CPB_CNT_LIST_SIZE as usize],
    /// cpb_cnt_minus1 number of valid elements
    pub cbr_flag: [u8; H264_CPB_CNT_LIST_SIZE as usize],
    pub initial_cpb_removal_delay_length_minus1: u32,
    pub cpb_removal_delay_length_minus1: u32,
    pub dpb_output_delay_length_minus1: u32,
    pub time_offset_length: u32,
}
impl Default for H264HrdParameters {
    fn default() -> Self {
        Self {
            cpb_cnt_minus1: 0,
            bit_rate_scale: 0,
            cpb_size_scale: 0,
            reserved1: 0,
            bit_rate_value_minus1: unsafe { mem::zeroed() },
            cpb_size_value_minus1: unsafe { mem::zeroed() },
            cbr_flag: unsafe { mem::zeroed() },
            initial_cpb_removal_delay_length_minus1: 0,
            cpb_removal_delay_length_minus1: 0,
            dpb_output_delay_length_minus1: 0,
            time_offset_length: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct H264SequenceParameterSetVui<'a> {
    pub flags: H264SpsVuiFlags,
    pub aspect_ratio_idc: H264AspectRatioIdc,
    pub sar_width: u16,
    pub sar_height: u16,
    pub video_format: u8,
    pub colour_primaries: u8,
    pub transfer_characteristics: u8,
    pub matrix_coefficients: u8,
    pub num_units_in_tick: u32,
    pub time_scale: u32,
    pub max_num_reorder_frames: u8,
    pub max_dec_frame_buffering: u8,
    pub chroma_sample_loc_type_top_field: u8,
    pub chroma_sample_loc_type_bottom_field: u8,
    /// Reserved for future use and must be initialized with 0.
    pub reserved1: u32,
    /// must be a valid ptr to hrd_parameters, if nal_hrd_parameters_present_flag or vcl_hrd_parameters_present_flag are set
    /// Nullable
    pub hrd_parameters: *const H264HrdParameters,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for H264SequenceParameterSetVui<'_> {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            aspect_ratio_idc: Default::default(),
            sar_width: 0,
            sar_height: 0,
            video_format: 0,
            colour_primaries: 0,
            transfer_characteristics: 0,
            matrix_coefficients: 0,
            num_units_in_tick: 0,
            time_scale: 0,
            max_num_reorder_frames: 0,
            max_dec_frame_buffering: 0,
            chroma_sample_loc_type_top_field: 0,
            chroma_sample_loc_type_bottom_field: 0,
            reserved1: 0,
            hrd_parameters: ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct H264SpsFlags {
    pub constraint_set0_flag: u32,
    pub constraint_set1_flag: u32,
    pub constraint_set2_flag: u32,
    pub constraint_set3_flag: u32,
    pub constraint_set4_flag: u32,
    pub constraint_set5_flag: u32,
    pub direct_8x8_inference_flag: u32,
    pub mb_adaptive_frame_field_flag: u32,
    pub frame_mbs_only_flag: u32,
    pub delta_pic_order_always_zero_flag: u32,
    pub separate_colour_plane_flag: u32,
    pub gaps_in_frame_num_value_allowed_flag: u32,
    pub qpprime_y_zero_transform_bypass_flag: u32,
    pub frame_cropping_flag: u32,
    pub seq_scaling_matrix_present_flag: u32,
    pub vui_parameters_present_flag: u32,
}
impl Default for H264SpsFlags {
    fn default() -> Self {
        Self {
            constraint_set0_flag: 0,
            constraint_set1_flag: 0,
            constraint_set2_flag: 0,
            constraint_set3_flag: 0,
            constraint_set4_flag: 0,
            constraint_set5_flag: 0,
            direct_8x8_inference_flag: 0,
            mb_adaptive_frame_field_flag: 0,
            frame_mbs_only_flag: 0,
            delta_pic_order_always_zero_flag: 0,
            separate_colour_plane_flag: 0,
            gaps_in_frame_num_value_allowed_flag: 0,
            qpprime_y_zero_transform_bypass_flag: 0,
            frame_cropping_flag: 0,
            seq_scaling_matrix_present_flag: 0,
            vui_parameters_present_flag: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct H264ScalingLists {
    pub scaling_list_present_mask: u16,
    pub use_default_scaling_matrix_mask: u16,
    pub scaling_list4x4: [[u8; H264_SCALING_LIST_4X4_NUM_ELEMENTS as usize];
        H264_SCALING_LIST_4X4_NUM_LISTS as usize],
    pub scaling_list8x8: [[u8; H264_SCALING_LIST_8X8_NUM_ELEMENTS as usize];
        H264_SCALING_LIST_8X8_NUM_LISTS as usize],
}
impl Default for H264ScalingLists {
    fn default() -> Self {
        Self {
            scaling_list_present_mask: 0,
            use_default_scaling_matrix_mask: 0,
            scaling_list4x4: unsafe { mem::zeroed() },
            scaling_list8x8: unsafe { mem::zeroed() },
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct H264SequenceParameterSet<'a> {
    pub flags: H264SpsFlags,
    pub profile_idc: H264ProfileIdc,
    pub level_idc: H264LevelIdc,
    pub chroma_format_idc: H264ChromaFormatIdc,
    pub seq_parameter_set_id: u8,
    pub bit_depth_luma_minus8: u8,
    pub bit_depth_chroma_minus8: u8,
    pub log2_max_frame_num_minus4: u8,
    pub pic_order_cnt_type: H264PocType,
    pub offset_for_non_ref_pic: i32,
    pub offset_for_top_to_bottom_field: i32,
    pub log2_max_pic_order_cnt_lsb_minus4: u8,
    pub num_ref_frames_in_pic_order_cnt_cycle: u8,
    pub max_num_ref_frames: u8,
    /// Reserved for future use and must be initialized with 0.
    pub reserved1: u8,
    pub pic_width_in_mbs_minus1: u32,
    pub pic_height_in_map_units_minus1: u32,
    pub frame_crop_left_offset: u32,
    pub frame_crop_right_offset: u32,
    pub frame_crop_top_offset: u32,
    pub frame_crop_bottom_offset: u32,
    /// Reserved for future use and must be initialized with 0.
    pub reserved2: u32,
    /// Len: `num_ref_frames_in_pic_order_cnt_cycle`
    pub offset_for_ref_frame: *const i32,
    /// Must be a valid pointer if seq_scaling_matrix_present_flag is set
    /// Nullable
    pub scaling_lists: *const H264ScalingLists,
    /// Must be a valid pointer if StdVideoH264SpsFlags:vui_parameters_present_flag is set
    /// Nullable
    pub sequence_parameter_set_vui: *const H264SequenceParameterSetVui<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for H264SequenceParameterSet<'_> {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            profile_idc: Default::default(),
            level_idc: Default::default(),
            chroma_format_idc: Default::default(),
            seq_parameter_set_id: 0,
            bit_depth_luma_minus8: 0,
            bit_depth_chroma_minus8: 0,
            log2_max_frame_num_minus4: 0,
            pic_order_cnt_type: Default::default(),
            offset_for_non_ref_pic: 0,
            offset_for_top_to_bottom_field: 0,
            log2_max_pic_order_cnt_lsb_minus4: 0,
            num_ref_frames_in_pic_order_cnt_cycle: 0,
            max_num_ref_frames: 0,
            reserved1: 0,
            pic_width_in_mbs_minus1: 0,
            pic_height_in_map_units_minus1: 0,
            frame_crop_left_offset: 0,
            frame_crop_right_offset: 0,
            frame_crop_top_offset: 0,
            frame_crop_bottom_offset: 0,
            reserved2: 0,
            offset_for_ref_frame: ptr::null_mut(),
            scaling_lists: ptr::null_mut(),
            sequence_parameter_set_vui: ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct H264PpsFlags {
    pub transform_8x8_mode_flag: u32,
    pub redundant_pic_cnt_present_flag: u32,
    pub constrained_intra_pred_flag: u32,
    pub deblocking_filter_control_present_flag: u32,
    pub weighted_pred_flag: u32,
    pub bottom_field_pic_order_in_frame_present_flag: u32,
    pub entropy_coding_mode_flag: u32,
    pub pic_scaling_matrix_present_flag: u32,
}
impl Default for H264PpsFlags {
    fn default() -> Self {
        Self {
            transform_8x8_mode_flag: 0,
            redundant_pic_cnt_present_flag: 0,
            constrained_intra_pred_flag: 0,
            deblocking_filter_control_present_flag: 0,
            weighted_pred_flag: 0,
            bottom_field_pic_order_in_frame_present_flag: 0,
            entropy_coding_mode_flag: 0,
            pic_scaling_matrix_present_flag: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct H264PictureParameterSet<'a> {
    pub flags: H264PpsFlags,
    pub seq_parameter_set_id: u8,
    pub pic_parameter_set_id: u8,
    pub num_ref_idx_l0_default_active_minus1: u8,
    pub num_ref_idx_l1_default_active_minus1: u8,
    pub weighted_bipred_idc: H264WeightedBipredIdc,
    pub pic_init_qp_minus26: i8,
    pub pic_init_qs_minus26: i8,
    pub chroma_qp_index_offset: i8,
    pub second_chroma_qp_index_offset: i8,
    /// Must be a valid pointer if StdVideoH264PpsFlags::pic_scaling_matrix_present_flag is set.
    /// Nullable
    pub scaling_lists: *const H264ScalingLists,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for H264PictureParameterSet<'_> {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            seq_parameter_set_id: 0,
            pic_parameter_set_id: 0,
            num_ref_idx_l0_default_active_minus1: 0,
            num_ref_idx_l1_default_active_minus1: 0,
            weighted_bipred_idc: Default::default(),
            pic_init_qp_minus26: 0,
            pic_init_qs_minus26: 0,
            chroma_qp_index_offset: 0,
            second_chroma_qp_index_offset: 0,
            scaling_lists: ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DecodeH264PictureInfoFlags {
    /// Is field picture
    pub field_pic_flag: u32,
    /// Is intra picture
    pub is_intra: u32,
    /// instantaneous decoding refresh (IDR) picture
    pub idr_pic_flag: u32,
    /// bottom (true) or top (false) field if field_pic_flag is set.
    pub bottom_field_flag: u32,
    /// This only applies to picture info, and not to the DPB lists.
    pub is_reference: u32,
    /// complementary field pair, complementary non-reference field pair, complementary reference field pair
    pub complementary_field_pair: u32,
}
impl Default for DecodeH264PictureInfoFlags {
    fn default() -> Self {
        Self {
            field_pic_flag: 0,
            is_intra: 0,
            idr_pic_flag: 0,
            bottom_field_flag: 0,
            is_reference: 0,
            complementary_field_pair: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DecodeH264PictureInfo {
    pub flags: DecodeH264PictureInfoFlags,
    /// Selecting SPS id from the Sequence Parameters Set
    pub seq_parameter_set_id: u8,
    /// Selecting PPS id from the Picture Parameters Set
    pub pic_parameter_set_id: u8,
    /// Reserved for future use and must be initialized with 0.
    pub reserved1: u8,
    /// Reserved for future use and must be initialized with 0.
    pub reserved2: u8,
    /// 7.4.3 Slice header semantics
    pub frame_num: u16,
    /// 7.4.3 Slice header semantics
    pub idr_pic_id: u16,
    /// TopFieldOrderCnt and BottomFieldOrderCnt fields.
    pub pic_order_cnt: [i32; DECODE_H264_FIELD_ORDER_COUNT_LIST_SIZE as usize],
}
impl Default for DecodeH264PictureInfo {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            seq_parameter_set_id: 0,
            pic_parameter_set_id: 0,
            reserved1: 0,
            reserved2: 0,
            frame_num: 0,
            idr_pic_id: 0,
            pic_order_cnt: unsafe { mem::zeroed() },
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DecodeH264ReferenceInfoFlags {
    /// Reference is used for top field reference.
    pub top_field_flag: u32,
    /// Reference is used for bottom field reference.
    pub bottom_field_flag: u32,
    /// A picture that is marked as "used for long-term reference", derived binary value from clause 8.2.5.1 Sequence of operations for decoded reference picture marking process
    pub used_for_long_term_reference: u32,
    /// Must be handled in accordance with 8.2.5.2: Decoding process for gaps in frame_num
    pub is_non_existing: u32,
}
impl Default for DecodeH264ReferenceInfoFlags {
    fn default() -> Self {
        Self {
            top_field_flag: 0,
            bottom_field_flag: 0,
            used_for_long_term_reference: 0,
            is_non_existing: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DecodeH264ReferenceInfo {
    pub flags: DecodeH264ReferenceInfoFlags,
    /// 7.4.3.3 Decoded reference picture marking semantics
    pub frame_num: u16,
    /// for structure members 32-bit packing/alignment
    pub reserved: u16,
    /// TopFieldOrderCnt and BottomFieldOrderCnt fields.
    pub pic_order_cnt: [i32; DECODE_H264_FIELD_ORDER_COUNT_LIST_SIZE as usize],
}
impl Default for DecodeH264ReferenceInfo {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            frame_num: 0,
            reserved: 0,
            pic_order_cnt: unsafe { mem::zeroed() },
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EncodeH264WeightTableFlags {
    /// each bit n represents the nth entry in reference list l0, n <= num_ref_idx_l0_active_minus1
    pub luma_weight_l0_flag: u32,
    /// each bit n represents the nth entry in reference list l0, n <= num_ref_idx_l0_active_minus1
    pub chroma_weight_l0_flag: u32,
    /// each bit n represents the nth entry in reference list l1, n <= num_ref_idx_l1_active_minus1
    pub luma_weight_l1_flag: u32,
    /// each bit n represents the nth entry in reference list l1, n <= num_ref_idx_l1_active_minus1
    pub chroma_weight_l1_flag: u32,
}
impl Default for EncodeH264WeightTableFlags {
    fn default() -> Self {
        Self {
            luma_weight_l0_flag: 0,
            chroma_weight_l0_flag: 0,
            luma_weight_l1_flag: 0,
            chroma_weight_l1_flag: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EncodeH264WeightTable {
    pub flags: EncodeH264WeightTableFlags,
    pub luma_log2_weight_denom: u8,
    pub chroma_log2_weight_denom: u8,
    /// valid entry range is [0, num_ref_idx_l0_active_minus1]
    pub luma_weight_l0: [i8; H264_MAX_NUM_LIST_REF as usize],
    /// valid entry range is [0, num_ref_idx_l0_active_minus1]
    pub luma_offset_l0: [i8; H264_MAX_NUM_LIST_REF as usize],
    /// [i][j]: valid entry range for i is [0, num_ref_idx_l0_active_minus1]; j = 0 for Cb, j = 1 for Cr
    pub chroma_weight_l0: [[i8; H264_MAX_CHROMA_PLANES as usize]; H264_MAX_NUM_LIST_REF as usize],
    /// [i][j]: valid entry range for i is [0, num_ref_idx_l0_active_minus1]; j = 0 for Cb, j = 1 for Cr
    pub chroma_offset_l0: [[i8; H264_MAX_CHROMA_PLANES as usize]; H264_MAX_NUM_LIST_REF as usize],
    /// valid entry range is [0, num_ref_idx_l1_active_minus1]
    pub luma_weight_l1: [i8; H264_MAX_NUM_LIST_REF as usize],
    /// valid entry range is [0, num_ref_idx_l1_active_minus1]
    pub luma_offset_l1: [i8; H264_MAX_NUM_LIST_REF as usize],
    /// [i][j]: valid entry range for i is [0, num_ref_idx_l1_active_minus1]; j = 0 for Cb, j = 1 for Cr
    pub chroma_weight_l1: [[i8; H264_MAX_CHROMA_PLANES as usize]; H264_MAX_NUM_LIST_REF as usize],
    /// [i][j]: valid entry range for i is [0, num_ref_idx_l1_active_minus1]; j = 0 for Cb, j = 1 for Cr
    pub chroma_offset_l1: [[i8; H264_MAX_CHROMA_PLANES as usize]; H264_MAX_NUM_LIST_REF as usize],
}
impl Default for EncodeH264WeightTable {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            luma_log2_weight_denom: 0,
            chroma_log2_weight_denom: 0,
            luma_weight_l0: unsafe { mem::zeroed() },
            luma_offset_l0: unsafe { mem::zeroed() },
            chroma_weight_l0: unsafe { mem::zeroed() },
            chroma_offset_l0: unsafe { mem::zeroed() },
            luma_weight_l1: unsafe { mem::zeroed() },
            luma_offset_l1: unsafe { mem::zeroed() },
            chroma_weight_l1: unsafe { mem::zeroed() },
            chroma_offset_l1: unsafe { mem::zeroed() },
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EncodeH264SliceHeaderFlags {
    pub direct_spatial_mv_pred_flag: u32,
    pub num_ref_idx_active_override_flag: u32,
    pub reserved: u32,
}
impl Default for EncodeH264SliceHeaderFlags {
    fn default() -> Self {
        Self {
            direct_spatial_mv_pred_flag: 0,
            num_ref_idx_active_override_flag: 0,
            reserved: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EncodeH264PictureInfoFlags {
    pub idr_pic_flag: u32,
    /// A reference picture, i.e. a picture with nal_ref_idc not equal to 0, as defined in clause 3.136
    pub is_reference: u32,
    pub no_output_of_prior_pics_flag: u32,
    pub long_term_reference_flag: u32,
    pub adaptive_ref_pic_marking_mode_flag: u32,
    pub reserved: u32,
}
impl Default for EncodeH264PictureInfoFlags {
    fn default() -> Self {
        Self {
            idr_pic_flag: 0,
            is_reference: 0,
            no_output_of_prior_pics_flag: 0,
            long_term_reference_flag: 0,
            adaptive_ref_pic_marking_mode_flag: 0,
            reserved: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EncodeH264ReferenceInfoFlags {
    /// A picture that is marked as "used for long-term reference", derived binary value from clause 8.2.5.1 Sequence of operations for decoded reference picture marking process
    pub used_for_long_term_reference: u32,
    pub reserved: u32,
}
impl Default for EncodeH264ReferenceInfoFlags {
    fn default() -> Self {
        Self {
            used_for_long_term_reference: 0,
            reserved: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EncodeH264ReferenceListsInfoFlags {
    pub ref_pic_list_modification_flag_l0: u32,
    pub ref_pic_list_modification_flag_l1: u32,
    pub reserved: u32,
}
impl Default for EncodeH264ReferenceListsInfoFlags {
    fn default() -> Self {
        Self {
            ref_pic_list_modification_flag_l0: 0,
            ref_pic_list_modification_flag_l1: 0,
            reserved: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EncodeH264RefListModEntry {
    pub modification_of_pic_nums_idc: H264ModificationOfPicNumsIdc,
    pub abs_diff_pic_num_minus1: u16,
    pub long_term_pic_num: u16,
}
impl Default for EncodeH264RefListModEntry {
    fn default() -> Self {
        Self {
            modification_of_pic_nums_idc: Default::default(),
            abs_diff_pic_num_minus1: 0,
            long_term_pic_num: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EncodeH264RefPicMarkingEntry {
    pub memory_management_control_operation: H264MemMgmtControlOp,
    pub difference_of_pic_nums_minus1: u16,
    pub long_term_pic_num: u16,
    pub long_term_frame_idx: u16,
    pub max_long_term_frame_idx_plus1: u16,
}
impl Default for EncodeH264RefPicMarkingEntry {
    fn default() -> Self {
        Self {
            memory_management_control_operation: Default::default(),
            difference_of_pic_nums_minus1: 0,
            long_term_pic_num: 0,
            long_term_frame_idx: 0,
            max_long_term_frame_idx_plus1: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EncodeH264ReferenceListsInfo<'a> {
    pub flags: EncodeH264ReferenceListsInfoFlags,
    pub num_ref_idx_l0_active_minus1: u8,
    pub num_ref_idx_l1_active_minus1: u8,
    /// slotIndex as used in VkVideoReferenceSlotInfoKHR structures or STD_VIDEO_H264_NO_REFERENCE_PICTURE
    pub ref_pic_list0: [u8; H264_MAX_NUM_LIST_REF as usize],
    /// slotIndex as used in VkVideoReferenceSlotInfoKHR structures or STD_VIDEO_H264_NO_REFERENCE_PICTURE
    pub ref_pic_list1: [u8; H264_MAX_NUM_LIST_REF as usize],
    pub ref_list0_mod_op_count: u8,
    pub ref_list1_mod_op_count: u8,
    pub ref_pic_marking_op_count: u8,
    /// Reserved for future use and must be initialized with 0.
    pub reserved1: [u8; 7],
    /// Must be a valid pointer to an array with size refList0ModOpCount if ref_pic_list_modification_flag_l0 is set and contains the RefList0 modification parameters as defined in section 7.4.3.1
    /// Len: `ref_list0_mod_op_count`
    pub ref_list0_mod_operations: *const EncodeH264RefListModEntry,
    /// Must be a valid pointer to an array with size refList1ModOpCount if ref_pic_list_modification_flag_l1 is set and contains the RefList1 modification parameters as defined in section 7.4.3.1
    /// Len: `ref_list1_mod_op_count`
    pub ref_list1_mod_operations: *const EncodeH264RefListModEntry,
    /// Must be a valid pointer to an array with size refPicMarkingOpCount and contains the reference picture markings as defined in section 7.4.3.3
    /// Len: `ref_pic_marking_op_count`
    pub ref_pic_marking_operations: *const EncodeH264RefPicMarkingEntry,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for EncodeH264ReferenceListsInfo<'_> {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            num_ref_idx_l0_active_minus1: 0,
            num_ref_idx_l1_active_minus1: 0,
            ref_pic_list0: unsafe { mem::zeroed() },
            ref_pic_list1: unsafe { mem::zeroed() },
            ref_list0_mod_op_count: 0,
            ref_list1_mod_op_count: 0,
            ref_pic_marking_op_count: 0,
            reserved1: unsafe { mem::zeroed() },
            ref_list0_mod_operations: ptr::null_mut(),
            ref_list1_mod_operations: ptr::null_mut(),
            ref_pic_marking_operations: ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EncodeH264PictureInfo<'a> {
    pub flags: EncodeH264PictureInfoFlags,
    /// Selecting SPS id from the Sequence Parameters Set
    pub seq_parameter_set_id: u8,
    /// Selecting PPS from the Picture Parameters for all StdVideoEncodeH264SliceHeader(s)
    pub pic_parameter_set_id: u8,
    pub idr_pic_id: u16,
    pub primary_pic_type: H264PictureType,
    pub frame_num: u32,
    /// Picture order count, as defined in 8.2
    pub pic_order_cnt: i32,
    /// Temporal identifier of the picture, as defined in G.7.3.1.1 / G.7.4.1.1
    pub temporal_id: u8,
    /// Reserved for future use and must be initialized with 0.
    pub reserved1: [u8; 3],
    /// Nullable
    pub ref_lists: *const EncodeH264ReferenceListsInfo<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for EncodeH264PictureInfo<'_> {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            seq_parameter_set_id: 0,
            pic_parameter_set_id: 0,
            idr_pic_id: 0,
            primary_pic_type: Default::default(),
            frame_num: 0,
            pic_order_cnt: 0,
            temporal_id: 0,
            reserved1: unsafe { mem::zeroed() },
            ref_lists: ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EncodeH264ReferenceInfo {
    pub flags: EncodeH264ReferenceInfoFlags,
    pub primary_pic_type: H264PictureType,
    /// Frame number, as defined in 8.2
    pub frame_num: u32,
    /// Picture order count, as defined in 8.2
    pub pic_order_cnt: i32,
    pub long_term_pic_num: u16,
    pub long_term_frame_idx: u16,
    /// Temporal identifier of the picture, as defined in G.7.3.1.1 / G.7.4.1.1
    pub temporal_id: u8,
}
impl Default for EncodeH264ReferenceInfo {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            primary_pic_type: Default::default(),
            frame_num: 0,
            pic_order_cnt: 0,
            long_term_pic_num: 0,
            long_term_frame_idx: 0,
            temporal_id: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EncodeH264SliceHeader<'a> {
    pub flags: EncodeH264SliceHeaderFlags,
    pub first_mb_in_slice: u32,
    pub slice_type: H264SliceType,
    pub slice_alpha_c0_offset_div2: i8,
    pub slice_beta_offset_div2: i8,
    pub slice_qp_delta: i8,
    /// Reserved for future use and must be initialized with 0.
    pub reserved1: u8,
    pub cabac_init_idc: H264CabacInitIdc,
    pub disable_deblocking_filter_idc: H264DisableDeblockingFilterIdc,
    /// Nullable
    pub weight_table: *const EncodeH264WeightTable,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for EncodeH264SliceHeader<'_> {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            first_mb_in_slice: 0,
            slice_type: Default::default(),
            slice_alpha_c0_offset_div2: 0,
            slice_beta_offset_div2: 0,
            slice_qp_delta: 0,
            reserved1: 0,
            cabac_init_idc: Default::default(),
            disable_deblocking_filter_idc: Default::default(),
            weight_table: ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct H265ProfileTierLevelFlags {
    pub general_tier_flag: u32,
    pub general_progressive_source_flag: u32,
    pub general_interlaced_source_flag: u32,
    pub general_non_packed_constraint_flag: u32,
    pub general_frame_only_constraint_flag: u32,
}
impl Default for H265ProfileTierLevelFlags {
    fn default() -> Self {
        Self {
            general_tier_flag: 0,
            general_progressive_source_flag: 0,
            general_interlaced_source_flag: 0,
            general_non_packed_constraint_flag: 0,
            general_frame_only_constraint_flag: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct H265ProfileTierLevel {
    pub flags: H265ProfileTierLevelFlags,
    pub general_profile_idc: H265ProfileIdc,
    pub general_level_idc: H265LevelIdc,
}
impl Default for H265ProfileTierLevel {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            general_profile_idc: Default::default(),
            general_level_idc: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct H265DecPicBufMgr {
    /// represents sps_max_latency_increase_plus1 or vps_max_latency_increase_plus1
    pub max_latency_increase_plus1: [u32; H265_SUBLAYERS_LIST_SIZE as usize],
    /// represents sps_max_dec_pic_buffering_minus1 or vps_max_dec_pic_buffering_minus1
    pub max_dec_pic_buffering_minus1: [u8; H265_SUBLAYERS_LIST_SIZE as usize],
    /// represents sps_max_num_reorder_pics or vps_max_num_reorder_pics
    pub max_num_reorder_pics: [u8; H265_SUBLAYERS_LIST_SIZE as usize],
}
impl Default for H265DecPicBufMgr {
    fn default() -> Self {
        Self {
            max_latency_increase_plus1: unsafe { mem::zeroed() },
            max_dec_pic_buffering_minus1: unsafe { mem::zeroed() },
            max_num_reorder_pics: unsafe { mem::zeroed() },
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct H265SubLayerHrdParameters {
    pub bit_rate_value_minus1: [u32; H265_CPB_CNT_LIST_SIZE as usize],
    pub cpb_size_value_minus1: [u32; H265_CPB_CNT_LIST_SIZE as usize],
    pub cpb_size_du_value_minus1: [u32; H265_CPB_CNT_LIST_SIZE as usize],
    pub bit_rate_du_value_minus1: [u32; H265_CPB_CNT_LIST_SIZE as usize],
    /// each bit represents a range of CpbCounts (bit 0 - cpb_cnt_minus1) per sub-layer
    pub cbr_flag: u32,
}
impl Default for H265SubLayerHrdParameters {
    fn default() -> Self {
        Self {
            bit_rate_value_minus1: unsafe { mem::zeroed() },
            cpb_size_value_minus1: unsafe { mem::zeroed() },
            cpb_size_du_value_minus1: unsafe { mem::zeroed() },
            bit_rate_du_value_minus1: unsafe { mem::zeroed() },
            cbr_flag: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct H265HrdFlags {
    pub nal_hrd_parameters_present_flag: u32,
    pub vcl_hrd_parameters_present_flag: u32,
    pub sub_pic_hrd_params_present_flag: u32,
    pub sub_pic_cpb_params_in_pic_timing_sei_flag: u32,
    /// each bit represents a sublayer, bit 0 - vps_max_sub_layers_minus1
    pub fixed_pic_rate_general_flag: u32,
    /// each bit represents a sublayer, bit 0 - vps_max_sub_layers_minus1
    pub fixed_pic_rate_within_cvs_flag: u32,
    /// each bit represents a sublayer, bit 0 - vps_max_sub_layers_minus1
    pub low_delay_hrd_flag: u32,
}
impl Default for H265HrdFlags {
    fn default() -> Self {
        Self {
            nal_hrd_parameters_present_flag: 0,
            vcl_hrd_parameters_present_flag: 0,
            sub_pic_hrd_params_present_flag: 0,
            sub_pic_cpb_params_in_pic_timing_sei_flag: 0,
            fixed_pic_rate_general_flag: 0,
            fixed_pic_rate_within_cvs_flag: 0,
            low_delay_hrd_flag: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct H265HrdParameters<'a> {
    pub flags: H265HrdFlags,
    pub tick_divisor_minus2: u8,
    pub du_cpb_removal_delay_increment_length_minus1: u8,
    pub dpb_output_delay_du_length_minus1: u8,
    pub bit_rate_scale: u8,
    pub cpb_size_scale: u8,
    pub cpb_size_du_scale: u8,
    pub initial_cpb_removal_delay_length_minus1: u8,
    pub au_cpb_removal_delay_length_minus1: u8,
    pub dpb_output_delay_length_minus1: u8,
    pub cpb_cnt_minus1: [u8; H265_SUBLAYERS_LIST_SIZE as usize],
    pub elemental_duration_in_tc_minus1: [u16; H265_SUBLAYERS_LIST_SIZE as usize],
    /// Reserved for future use and must be initialized with 0.
    pub reserved: [u16; 3],
    /// if flags.nal_hrd_parameters_present_flag is set, then this must be a ptr to an array of StdVideoH265SubLayerHrdParameters with a size specified by sps_max_sub_layers_minus1 + 1 or vps_max_sub_layers_minus1 + 1, depending on whether the HRD parameters are part of the SPS or VPS, respectively.
    /// Nullable
    /// Len: `max_sub_layers_minus1_1`
    pub sub_layer_hrd_parameters_nal: *const H265SubLayerHrdParameters,
    /// if flags.vcl_hrd_parameters_present_flag is set, then this must be a ptr to an array of StdVideoH265SubLayerHrdParameters with a size specified by sps_max_sub_layers_minus1 + 1 or vps_max_sub_layers_minus1 + 1, depending on whether the HRD parameters are part of the SPS or VPS, respectively.
    /// Nullable
    /// Len: `max_sub_layers_minus1_1`
    pub sub_layer_hrd_parameters_vcl: *const H265SubLayerHrdParameters,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for H265HrdParameters<'_> {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            tick_divisor_minus2: 0,
            du_cpb_removal_delay_increment_length_minus1: 0,
            dpb_output_delay_du_length_minus1: 0,
            bit_rate_scale: 0,
            cpb_size_scale: 0,
            cpb_size_du_scale: 0,
            initial_cpb_removal_delay_length_minus1: 0,
            au_cpb_removal_delay_length_minus1: 0,
            dpb_output_delay_length_minus1: 0,
            cpb_cnt_minus1: unsafe { mem::zeroed() },
            elemental_duration_in_tc_minus1: unsafe { mem::zeroed() },
            reserved: unsafe { mem::zeroed() },
            sub_layer_hrd_parameters_nal: ptr::null_mut(),
            sub_layer_hrd_parameters_vcl: ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct H265VpsFlags {
    pub vps_temporal_id_nesting_flag: u32,
    pub vps_sub_layer_ordering_info_present_flag: u32,
    pub vps_timing_info_present_flag: u32,
    pub vps_poc_proportional_to_timing_flag: u32,
}
impl Default for H265VpsFlags {
    fn default() -> Self {
        Self {
            vps_temporal_id_nesting_flag: 0,
            vps_sub_layer_ordering_info_present_flag: 0,
            vps_timing_info_present_flag: 0,
            vps_poc_proportional_to_timing_flag: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct H265VideoParameterSet<'a> {
    pub flags: H265VpsFlags,
    pub vps_video_parameter_set_id: u8,
    pub vps_max_sub_layers_minus1: u8,
    /// Reserved for future use and must be initialized with 0.
    pub reserved1: u8,
    /// Reserved for future use and must be initialized with 0.
    pub reserved2: u8,
    pub vps_num_units_in_tick: u32,
    pub vps_time_scale: u32,
    pub vps_num_ticks_poc_diff_one_minus1: u32,
    /// Reserved for future use and must be initialized with 0.
    pub reserved3: u32,
    /// Nullable
    pub dec_pic_buf_mgr: *const H265DecPicBufMgr,
    /// Nullable
    pub hrd_parameters: *const H265HrdParameters<'a>,
    /// Nullable
    pub profile_tier_level: *const H265ProfileTierLevel,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for H265VideoParameterSet<'_> {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            vps_video_parameter_set_id: 0,
            vps_max_sub_layers_minus1: 0,
            reserved1: 0,
            reserved2: 0,
            vps_num_units_in_tick: 0,
            vps_time_scale: 0,
            vps_num_ticks_poc_diff_one_minus1: 0,
            reserved3: 0,
            dec_pic_buf_mgr: ptr::null_mut(),
            hrd_parameters: ptr::null_mut(),
            profile_tier_level: ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct H265ScalingLists {
    /// ScalingList[ 0 ][ MatrixID ][ i ] (sizeID = 0)
    pub scaling_list4x4: [[u8; H265_SCALING_LIST_4X4_NUM_ELEMENTS as usize];
        H265_SCALING_LIST_4X4_NUM_LISTS as usize],
    /// ScalingList[ 1 ][ MatrixID ][ i ] (sizeID = 1)
    pub scaling_list8x8: [[u8; H265_SCALING_LIST_8X8_NUM_ELEMENTS as usize];
        H265_SCALING_LIST_8X8_NUM_LISTS as usize],
    /// ScalingList[ 2 ][ Matri]xID ][ i ] (sizeID = 2)
    pub scaling_list16x16: [[u8; H265_SCALING_LIST_16X16_NUM_ELEMENTS as usize];
        H265_SCALING_LIST_16X16_NUM_LISTS as usize],
    /// ScalingList[ 3 ][ MatrixID ][ i ] (sizeID = 3)
    pub scaling_list32x32: [[u8; H265_SCALING_LIST_32X32_NUM_ELEMENTS as usize];
        H265_SCALING_LIST_32X32_NUM_LISTS as usize],
    /// scaling_list_dc_coef_minus8[ sizeID - 2 ][ matrixID ] + 8, sizeID = 2
    pub scaling_list_dc_coef16x16: [u8; H265_SCALING_LIST_16X16_NUM_LISTS as usize],
    /// scaling_list_dc_coef_minus8[ sizeID - 2 ][ matrixID ] + 8. sizeID = 3
    pub scaling_list_dc_coef32x32: [u8; H265_SCALING_LIST_32X32_NUM_LISTS as usize],
}
impl Default for H265ScalingLists {
    fn default() -> Self {
        Self {
            scaling_list4x4: unsafe { mem::zeroed() },
            scaling_list8x8: unsafe { mem::zeroed() },
            scaling_list16x16: unsafe { mem::zeroed() },
            scaling_list32x32: unsafe { mem::zeroed() },
            scaling_list_dc_coef16x16: unsafe { mem::zeroed() },
            scaling_list_dc_coef32x32: unsafe { mem::zeroed() },
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct H265ShortTermRefPicSetFlags {
    pub inter_ref_pic_set_prediction_flag: u32,
    pub delta_rps_sign: u32,
}
impl Default for H265ShortTermRefPicSetFlags {
    fn default() -> Self {
        Self {
            inter_ref_pic_set_prediction_flag: 0,
            delta_rps_sign: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct H265ShortTermRefPicSet {
    pub flags: H265ShortTermRefPicSetFlags,
    pub delta_idx_minus1: u32,
    /// each bit represents a use_delta_flag[j] syntax
    pub use_delta_flag: u16,
    pub abs_delta_rps_minus1: u16,
    /// each bit represents a used_by_curr_pic_flag[j] syntax
    pub used_by_curr_pic_flag: u16,
    /// each bit represents a used_by_curr_pic_s0_flag[i] syntax
    pub used_by_curr_pic_s0_flag: u16,
    /// each bit represents a used_by_curr_pic_s1_flag[i] syntax
    pub used_by_curr_pic_s1_flag: u16,
    /// Reserved for future use and must be initialized with 0.
    pub reserved1: u16,
    /// Reserved for future use and must be initialized with 0.
    pub reserved2: u8,
    /// Reserved for future use and must be initialized with 0.
    pub reserved3: u8,
    pub num_negative_pics: u8,
    pub num_positive_pics: u8,
    pub delta_poc_s0_minus1: [u16; H265_MAX_DPB_SIZE as usize],
    pub delta_poc_s1_minus1: [u16; H265_MAX_DPB_SIZE as usize],
}
impl Default for H265ShortTermRefPicSet {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            delta_idx_minus1: 0,
            use_delta_flag: 0,
            abs_delta_rps_minus1: 0,
            used_by_curr_pic_flag: 0,
            used_by_curr_pic_s0_flag: 0,
            used_by_curr_pic_s1_flag: 0,
            reserved1: 0,
            reserved2: 0,
            reserved3: 0,
            num_negative_pics: 0,
            num_positive_pics: 0,
            delta_poc_s0_minus1: unsafe { mem::zeroed() },
            delta_poc_s1_minus1: unsafe { mem::zeroed() },
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct H265LongTermRefPicsSps {
    /// each bit represents a used_by_curr_pic_lt_sps_flag[i] syntax
    pub used_by_curr_pic_lt_sps_flag: u32,
    pub lt_ref_pic_poc_lsb_sps: [u32; H265_MAX_LONG_TERM_REF_PICS_SPS as usize],
}
impl Default for H265LongTermRefPicsSps {
    fn default() -> Self {
        Self {
            used_by_curr_pic_lt_sps_flag: 0,
            lt_ref_pic_poc_lsb_sps: unsafe { mem::zeroed() },
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct H265SpsVuiFlags {
    pub aspect_ratio_info_present_flag: u32,
    pub overscan_info_present_flag: u32,
    pub overscan_appropriate_flag: u32,
    pub video_signal_type_present_flag: u32,
    pub video_full_range_flag: u32,
    pub colour_description_present_flag: u32,
    pub chroma_loc_info_present_flag: u32,
    pub neutral_chroma_indication_flag: u32,
    pub field_seq_flag: u32,
    pub frame_field_info_present_flag: u32,
    pub default_display_window_flag: u32,
    pub vui_timing_info_present_flag: u32,
    pub vui_poc_proportional_to_timing_flag: u32,
    pub vui_hrd_parameters_present_flag: u32,
    pub bitstream_restriction_flag: u32,
    pub tiles_fixed_structure_flag: u32,
    pub motion_vectors_over_pic_boundaries_flag: u32,
    pub restricted_ref_pic_lists_flag: u32,
}
impl Default for H265SpsVuiFlags {
    fn default() -> Self {
        Self {
            aspect_ratio_info_present_flag: 0,
            overscan_info_present_flag: 0,
            overscan_appropriate_flag: 0,
            video_signal_type_present_flag: 0,
            video_full_range_flag: 0,
            colour_description_present_flag: 0,
            chroma_loc_info_present_flag: 0,
            neutral_chroma_indication_flag: 0,
            field_seq_flag: 0,
            frame_field_info_present_flag: 0,
            default_display_window_flag: 0,
            vui_timing_info_present_flag: 0,
            vui_poc_proportional_to_timing_flag: 0,
            vui_hrd_parameters_present_flag: 0,
            bitstream_restriction_flag: 0,
            tiles_fixed_structure_flag: 0,
            motion_vectors_over_pic_boundaries_flag: 0,
            restricted_ref_pic_lists_flag: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct H265SequenceParameterSetVui<'a> {
    pub flags: H265SpsVuiFlags,
    pub aspect_ratio_idc: H265AspectRatioIdc,
    pub sar_width: u16,
    pub sar_height: u16,
    pub video_format: u8,
    pub colour_primaries: u8,
    pub transfer_characteristics: u8,
    pub matrix_coeffs: u8,
    pub chroma_sample_loc_type_top_field: u8,
    pub chroma_sample_loc_type_bottom_field: u8,
    /// Reserved for future use and must be initialized with 0.
    pub reserved1: u8,
    /// Reserved for future use and must be initialized with 0.
    pub reserved2: u8,
    pub def_disp_win_left_offset: u16,
    pub def_disp_win_right_offset: u16,
    pub def_disp_win_top_offset: u16,
    pub def_disp_win_bottom_offset: u16,
    pub vui_num_units_in_tick: u32,
    pub vui_time_scale: u32,
    pub vui_num_ticks_poc_diff_one_minus1: u32,
    pub min_spatial_segmentation_idc: u16,
    /// Reserved for future use and must be initialized with 0.
    pub reserved3: u16,
    pub max_bytes_per_pic_denom: u8,
    pub max_bits_per_min_cu_denom: u8,
    pub log2_max_mv_length_horizontal: u8,
    pub log2_max_mv_length_vertical: u8,
    /// Nullable
    pub hrd_parameters: *const H265HrdParameters<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for H265SequenceParameterSetVui<'_> {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            aspect_ratio_idc: Default::default(),
            sar_width: 0,
            sar_height: 0,
            video_format: 0,
            colour_primaries: 0,
            transfer_characteristics: 0,
            matrix_coeffs: 0,
            chroma_sample_loc_type_top_field: 0,
            chroma_sample_loc_type_bottom_field: 0,
            reserved1: 0,
            reserved2: 0,
            def_disp_win_left_offset: 0,
            def_disp_win_right_offset: 0,
            def_disp_win_top_offset: 0,
            def_disp_win_bottom_offset: 0,
            vui_num_units_in_tick: 0,
            vui_time_scale: 0,
            vui_num_ticks_poc_diff_one_minus1: 0,
            min_spatial_segmentation_idc: 0,
            reserved3: 0,
            max_bytes_per_pic_denom: 0,
            max_bits_per_min_cu_denom: 0,
            log2_max_mv_length_horizontal: 0,
            log2_max_mv_length_vertical: 0,
            hrd_parameters: ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct H265PredictorPaletteEntries {
    pub predictor_palette_entries: [[u16; H265_PREDICTOR_PALETTE_COMP_ENTRIES_LIST_SIZE as usize];
        H265_PREDICTOR_PALETTE_COMPONENTS_LIST_SIZE as usize],
}
impl Default for H265PredictorPaletteEntries {
    fn default() -> Self {
        Self {
            predictor_palette_entries: unsafe { mem::zeroed() },
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct H265SpsFlags {
    pub sps_temporal_id_nesting_flag: u32,
    pub separate_colour_plane_flag: u32,
    pub conformance_window_flag: u32,
    pub sps_sub_layer_ordering_info_present_flag: u32,
    pub scaling_list_enabled_flag: u32,
    pub sps_scaling_list_data_present_flag: u32,
    pub amp_enabled_flag: u32,
    pub sample_adaptive_offset_enabled_flag: u32,
    pub pcm_enabled_flag: u32,
    pub pcm_loop_filter_disabled_flag: u32,
    pub long_term_ref_pics_present_flag: u32,
    pub sps_temporal_mvp_enabled_flag: u32,
    pub strong_intra_smoothing_enabled_flag: u32,
    pub vui_parameters_present_flag: u32,
    pub sps_extension_present_flag: u32,
    pub sps_range_extension_flag: u32,
    pub transform_skip_rotation_enabled_flag: u32,
    pub transform_skip_context_enabled_flag: u32,
    pub implicit_rdpcm_enabled_flag: u32,
    pub explicit_rdpcm_enabled_flag: u32,
    pub extended_precision_processing_flag: u32,
    pub intra_smoothing_disabled_flag: u32,
    pub high_precision_offsets_enabled_flag: u32,
    pub persistent_rice_adaptation_enabled_flag: u32,
    pub cabac_bypass_alignment_enabled_flag: u32,
    pub sps_scc_extension_flag: u32,
    pub sps_curr_pic_ref_enabled_flag: u32,
    pub palette_mode_enabled_flag: u32,
    pub sps_palette_predictor_initializers_present_flag: u32,
    pub intra_boundary_filtering_disabled_flag: u32,
}
impl Default for H265SpsFlags {
    fn default() -> Self {
        Self {
            sps_temporal_id_nesting_flag: 0,
            separate_colour_plane_flag: 0,
            conformance_window_flag: 0,
            sps_sub_layer_ordering_info_present_flag: 0,
            scaling_list_enabled_flag: 0,
            sps_scaling_list_data_present_flag: 0,
            amp_enabled_flag: 0,
            sample_adaptive_offset_enabled_flag: 0,
            pcm_enabled_flag: 0,
            pcm_loop_filter_disabled_flag: 0,
            long_term_ref_pics_present_flag: 0,
            sps_temporal_mvp_enabled_flag: 0,
            strong_intra_smoothing_enabled_flag: 0,
            vui_parameters_present_flag: 0,
            sps_extension_present_flag: 0,
            sps_range_extension_flag: 0,
            transform_skip_rotation_enabled_flag: 0,
            transform_skip_context_enabled_flag: 0,
            implicit_rdpcm_enabled_flag: 0,
            explicit_rdpcm_enabled_flag: 0,
            extended_precision_processing_flag: 0,
            intra_smoothing_disabled_flag: 0,
            high_precision_offsets_enabled_flag: 0,
            persistent_rice_adaptation_enabled_flag: 0,
            cabac_bypass_alignment_enabled_flag: 0,
            sps_scc_extension_flag: 0,
            sps_curr_pic_ref_enabled_flag: 0,
            palette_mode_enabled_flag: 0,
            sps_palette_predictor_initializers_present_flag: 0,
            intra_boundary_filtering_disabled_flag: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct H265SequenceParameterSet<'a> {
    pub flags: H265SpsFlags,
    pub chroma_format_idc: H265ChromaFormatIdc,
    pub pic_width_in_luma_samples: u32,
    pub pic_height_in_luma_samples: u32,
    pub sps_video_parameter_set_id: u8,
    pub sps_max_sub_layers_minus1: u8,
    pub sps_seq_parameter_set_id: u8,
    pub bit_depth_luma_minus8: u8,
    pub bit_depth_chroma_minus8: u8,
    pub log2_max_pic_order_cnt_lsb_minus4: u8,
    pub log2_min_luma_coding_block_size_minus3: u8,
    pub log2_diff_max_min_luma_coding_block_size: u8,
    pub log2_min_luma_transform_block_size_minus2: u8,
    pub log2_diff_max_min_luma_transform_block_size: u8,
    pub max_transform_hierarchy_depth_inter: u8,
    pub max_transform_hierarchy_depth_intra: u8,
    pub num_short_term_ref_pic_sets: u8,
    pub num_long_term_ref_pics_sps: u8,
    pub pcm_sample_bit_depth_luma_minus1: u8,
    pub pcm_sample_bit_depth_chroma_minus1: u8,
    pub log2_min_pcm_luma_coding_block_size_minus3: u8,
    pub log2_diff_max_min_pcm_luma_coding_block_size: u8,
    /// Reserved for future use and must be initialized with 0.
    pub reserved1: u8,
    /// Reserved for future use and must be initialized with 0.
    pub reserved2: u8,
    pub palette_max_size: u8,
    pub delta_palette_max_predictor_size: u8,
    pub motion_vector_resolution_control_idc: u8,
    pub sps_num_palette_predictor_initializers_minus1: u8,
    pub conf_win_left_offset: u32,
    pub conf_win_right_offset: u32,
    pub conf_win_top_offset: u32,
    pub conf_win_bottom_offset: u32,
    /// Nullable
    pub profile_tier_level: *const H265ProfileTierLevel,
    /// Nullable
    pub dec_pic_buf_mgr: *const H265DecPicBufMgr,
    /// Must be a valid pointer if sps_scaling_list_data_present_flag is set
    /// Nullable
    pub scaling_lists: *const H265ScalingLists,
    /// Must be a valid pointer to an array with size num_short_term_ref_pic_sets if num_short_term_ref_pic_sets is not 0.
    /// Len: `num_short_term_ref_pic_sets`
    pub short_term_ref_pic_set: *const H265ShortTermRefPicSet,
    /// Must be a valid pointer if long_term_ref_pics_present_flag is set
    /// Nullable
    pub long_term_ref_pics_sps: *const H265LongTermRefPicsSps,
    /// Must be a valid pointer if StdVideoH265SpsFlags:vui_parameters_present_flag is set palette_max_size
    /// Nullable
    pub sequence_parameter_set_vui: *const H265SequenceParameterSetVui<'a>,
    /// Must be a valid pointer if sps_palette_predictor_initializer_present_flag is set
    /// Nullable
    pub predictor_palette_entries: *const H265PredictorPaletteEntries,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for H265SequenceParameterSet<'_> {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            chroma_format_idc: Default::default(),
            pic_width_in_luma_samples: 0,
            pic_height_in_luma_samples: 0,
            sps_video_parameter_set_id: 0,
            sps_max_sub_layers_minus1: 0,
            sps_seq_parameter_set_id: 0,
            bit_depth_luma_minus8: 0,
            bit_depth_chroma_minus8: 0,
            log2_max_pic_order_cnt_lsb_minus4: 0,
            log2_min_luma_coding_block_size_minus3: 0,
            log2_diff_max_min_luma_coding_block_size: 0,
            log2_min_luma_transform_block_size_minus2: 0,
            log2_diff_max_min_luma_transform_block_size: 0,
            max_transform_hierarchy_depth_inter: 0,
            max_transform_hierarchy_depth_intra: 0,
            num_short_term_ref_pic_sets: 0,
            num_long_term_ref_pics_sps: 0,
            pcm_sample_bit_depth_luma_minus1: 0,
            pcm_sample_bit_depth_chroma_minus1: 0,
            log2_min_pcm_luma_coding_block_size_minus3: 0,
            log2_diff_max_min_pcm_luma_coding_block_size: 0,
            reserved1: 0,
            reserved2: 0,
            palette_max_size: 0,
            delta_palette_max_predictor_size: 0,
            motion_vector_resolution_control_idc: 0,
            sps_num_palette_predictor_initializers_minus1: 0,
            conf_win_left_offset: 0,
            conf_win_right_offset: 0,
            conf_win_top_offset: 0,
            conf_win_bottom_offset: 0,
            profile_tier_level: ptr::null_mut(),
            dec_pic_buf_mgr: ptr::null_mut(),
            scaling_lists: ptr::null_mut(),
            short_term_ref_pic_set: ptr::null_mut(),
            long_term_ref_pics_sps: ptr::null_mut(),
            sequence_parameter_set_vui: ptr::null_mut(),
            predictor_palette_entries: ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct H265PpsFlags {
    pub dependent_slice_segments_enabled_flag: u32,
    pub output_flag_present_flag: u32,
    pub sign_data_hiding_enabled_flag: u32,
    pub cabac_init_present_flag: u32,
    pub constrained_intra_pred_flag: u32,
    pub transform_skip_enabled_flag: u32,
    pub cu_qp_delta_enabled_flag: u32,
    pub pps_slice_chroma_qp_offsets_present_flag: u32,
    pub weighted_pred_flag: u32,
    pub weighted_bipred_flag: u32,
    pub transquant_bypass_enabled_flag: u32,
    pub tiles_enabled_flag: u32,
    pub entropy_coding_sync_enabled_flag: u32,
    pub uniform_spacing_flag: u32,
    pub loop_filter_across_tiles_enabled_flag: u32,
    pub pps_loop_filter_across_slices_enabled_flag: u32,
    pub deblocking_filter_control_present_flag: u32,
    pub deblocking_filter_override_enabled_flag: u32,
    pub pps_deblocking_filter_disabled_flag: u32,
    pub pps_scaling_list_data_present_flag: u32,
    pub lists_modification_present_flag: u32,
    pub slice_segment_header_extension_present_flag: u32,
    pub pps_extension_present_flag: u32,
    pub cross_component_prediction_enabled_flag: u32,
    pub chroma_qp_offset_list_enabled_flag: u32,
    pub pps_curr_pic_ref_enabled_flag: u32,
    pub residual_adaptive_colour_transform_enabled_flag: u32,
    pub pps_slice_act_qp_offsets_present_flag: u32,
    pub pps_palette_predictor_initializers_present_flag: u32,
    pub monochrome_palette_flag: u32,
    pub pps_range_extension_flag: u32,
}
impl Default for H265PpsFlags {
    fn default() -> Self {
        Self {
            dependent_slice_segments_enabled_flag: 0,
            output_flag_present_flag: 0,
            sign_data_hiding_enabled_flag: 0,
            cabac_init_present_flag: 0,
            constrained_intra_pred_flag: 0,
            transform_skip_enabled_flag: 0,
            cu_qp_delta_enabled_flag: 0,
            pps_slice_chroma_qp_offsets_present_flag: 0,
            weighted_pred_flag: 0,
            weighted_bipred_flag: 0,
            transquant_bypass_enabled_flag: 0,
            tiles_enabled_flag: 0,
            entropy_coding_sync_enabled_flag: 0,
            uniform_spacing_flag: 0,
            loop_filter_across_tiles_enabled_flag: 0,
            pps_loop_filter_across_slices_enabled_flag: 0,
            deblocking_filter_control_present_flag: 0,
            deblocking_filter_override_enabled_flag: 0,
            pps_deblocking_filter_disabled_flag: 0,
            pps_scaling_list_data_present_flag: 0,
            lists_modification_present_flag: 0,
            slice_segment_header_extension_present_flag: 0,
            pps_extension_present_flag: 0,
            cross_component_prediction_enabled_flag: 0,
            chroma_qp_offset_list_enabled_flag: 0,
            pps_curr_pic_ref_enabled_flag: 0,
            residual_adaptive_colour_transform_enabled_flag: 0,
            pps_slice_act_qp_offsets_present_flag: 0,
            pps_palette_predictor_initializers_present_flag: 0,
            monochrome_palette_flag: 0,
            pps_range_extension_flag: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct H265PictureParameterSet<'a> {
    pub flags: H265PpsFlags,
    pub pps_pic_parameter_set_id: u8,
    pub pps_seq_parameter_set_id: u8,
    pub sps_video_parameter_set_id: u8,
    pub num_extra_slice_header_bits: u8,
    pub num_ref_idx_l0_default_active_minus1: u8,
    pub num_ref_idx_l1_default_active_minus1: u8,
    pub init_qp_minus26: i8,
    pub diff_cu_qp_delta_depth: u8,
    pub pps_cb_qp_offset: i8,
    pub pps_cr_qp_offset: i8,
    pub pps_beta_offset_div2: i8,
    pub pps_tc_offset_div2: i8,
    pub log2_parallel_merge_level_minus2: u8,
    pub log2_max_transform_skip_block_size_minus2: u8,
    pub diff_cu_chroma_qp_offset_depth: u8,
    pub chroma_qp_offset_list_len_minus1: u8,
    pub cb_qp_offset_list: [i8; H265_CHROMA_QP_OFFSET_LIST_SIZE as usize],
    pub cr_qp_offset_list: [i8; H265_CHROMA_QP_OFFSET_LIST_SIZE as usize],
    pub log2_sao_offset_scale_luma: u8,
    pub log2_sao_offset_scale_chroma: u8,
    pub pps_act_y_qp_offset_plus5: i8,
    pub pps_act_cb_qp_offset_plus5: i8,
    pub pps_act_cr_qp_offset_plus3: i8,
    pub pps_num_palette_predictor_initializers: u8,
    pub luma_bit_depth_entry_minus8: u8,
    pub chroma_bit_depth_entry_minus8: u8,
    pub num_tile_columns_minus1: u8,
    pub num_tile_rows_minus1: u8,
    /// Reserved for future use and must be initialized with 0.
    pub reserved1: u8,
    /// Reserved for future use and must be initialized with 0.
    pub reserved2: u8,
    pub column_width_minus1: [u16; H265_CHROMA_QP_OFFSET_TILE_COLS_LIST_SIZE as usize],
    pub row_height_minus1: [u16; H265_CHROMA_QP_OFFSET_TILE_ROWS_LIST_SIZE as usize],
    /// Reserved for future use and must be initialized with 0.
    pub reserved3: u32,
    /// Must be a valid pointer if pps_scaling_list_data_present_flag is set
    /// Nullable
    pub scaling_lists: *const H265ScalingLists,
    /// Must be a valid pointer if pps_palette_predictor_initializer_present_flag is set
    /// Nullable
    pub predictor_palette_entries: *const H265PredictorPaletteEntries,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for H265PictureParameterSet<'_> {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            pps_pic_parameter_set_id: 0,
            pps_seq_parameter_set_id: 0,
            sps_video_parameter_set_id: 0,
            num_extra_slice_header_bits: 0,
            num_ref_idx_l0_default_active_minus1: 0,
            num_ref_idx_l1_default_active_minus1: 0,
            init_qp_minus26: 0,
            diff_cu_qp_delta_depth: 0,
            pps_cb_qp_offset: 0,
            pps_cr_qp_offset: 0,
            pps_beta_offset_div2: 0,
            pps_tc_offset_div2: 0,
            log2_parallel_merge_level_minus2: 0,
            log2_max_transform_skip_block_size_minus2: 0,
            diff_cu_chroma_qp_offset_depth: 0,
            chroma_qp_offset_list_len_minus1: 0,
            cb_qp_offset_list: unsafe { mem::zeroed() },
            cr_qp_offset_list: unsafe { mem::zeroed() },
            log2_sao_offset_scale_luma: 0,
            log2_sao_offset_scale_chroma: 0,
            pps_act_y_qp_offset_plus5: 0,
            pps_act_cb_qp_offset_plus5: 0,
            pps_act_cr_qp_offset_plus3: 0,
            pps_num_palette_predictor_initializers: 0,
            luma_bit_depth_entry_minus8: 0,
            chroma_bit_depth_entry_minus8: 0,
            num_tile_columns_minus1: 0,
            num_tile_rows_minus1: 0,
            reserved1: 0,
            reserved2: 0,
            column_width_minus1: unsafe { mem::zeroed() },
            row_height_minus1: unsafe { mem::zeroed() },
            reserved3: 0,
            scaling_lists: ptr::null_mut(),
            predictor_palette_entries: ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DecodeH265PictureInfoFlags {
    pub irap_pic_flag: u32,
    pub idr_pic_flag: u32,
    pub is_reference: u32,
    pub short_term_ref_pic_set_sps_flag: u32,
}
impl Default for DecodeH265PictureInfoFlags {
    fn default() -> Self {
        Self {
            irap_pic_flag: 0,
            idr_pic_flag: 0,
            is_reference: 0,
            short_term_ref_pic_set_sps_flag: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DecodeH265PictureInfo {
    pub flags: DecodeH265PictureInfoFlags,
    /// Selecting VPS id from the Video Parameters Set
    pub sps_video_parameter_set_id: u8,
    /// Selecting SPS id from the Sequence Parameters Set
    pub pps_seq_parameter_set_id: u8,
    /// Selecting PPS id from the Picture Parameters Set
    pub pps_pic_parameter_set_id: u8,
    /// NumDeltaPocs[ RefRpsIdx ] when short_term_ref_pic_set_sps_flag = 1, otherwise 0
    pub num_delta_pocs_of_ref_rps_idx: u8,
    pub pic_order_cnt_val: i32,
    /// number of bits used in st_ref_pic_set() when short_term_ref_pic_set_sps_flag is 0otherwise set to 0.
    pub num_bits_for_st_ref_pic_set_in_slice: u16,
    pub reserved: u16,
    /// slotIndex as used in VkVideoReferenceSlotInfoKHR structures representing pReferenceSlots in VkVideoDecodeInfoKHR or STD_VIDEO_H265_NO_REFERENCE_PICTURE
    pub ref_pic_set_st_curr_before: [u8; DECODE_H265_REF_PIC_SET_LIST_SIZE as usize],
    /// slotIndex as used in VkVideoReferenceSlotInfoKHR structures representing pReferenceSlots in VkVideoDecodeInfoKHR or STD_VIDEO_H265_NO_REFERENCE_PICTURE
    pub ref_pic_set_st_curr_after: [u8; DECODE_H265_REF_PIC_SET_LIST_SIZE as usize],
    /// slotIndex as used in VkVideoReferenceSlotInfoKHR structures representing pReferenceSlots in VkVideoDecodeInfoKHR or STD_VIDEO_H265_NO_REFERENCE_PICTURE
    pub ref_pic_set_lt_curr: [u8; DECODE_H265_REF_PIC_SET_LIST_SIZE as usize],
}
impl Default for DecodeH265PictureInfo {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            sps_video_parameter_set_id: 0,
            pps_seq_parameter_set_id: 0,
            pps_pic_parameter_set_id: 0,
            num_delta_pocs_of_ref_rps_idx: 0,
            pic_order_cnt_val: 0,
            num_bits_for_st_ref_pic_set_in_slice: 0,
            reserved: 0,
            ref_pic_set_st_curr_before: unsafe { mem::zeroed() },
            ref_pic_set_st_curr_after: unsafe { mem::zeroed() },
            ref_pic_set_lt_curr: unsafe { mem::zeroed() },
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DecodeH265ReferenceInfoFlags {
    /// A picture that is marked as "used for long-term reference", derived binary value from clause 8.3.2 Decoding process for reference picture set
    pub used_for_long_term_reference: u32,
    /// A picture that is marked as "unused for reference", derived binary value from clause 8.3.2 Decoding process for reference picture set
    pub unused_for_reference: u32,
}
impl Default for DecodeH265ReferenceInfoFlags {
    fn default() -> Self {
        Self {
            used_for_long_term_reference: 0,
            unused_for_reference: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DecodeH265ReferenceInfo {
    pub flags: DecodeH265ReferenceInfoFlags,
    pub pic_order_cnt_val: i32,
}
impl Default for DecodeH265ReferenceInfo {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            pic_order_cnt_val: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EncodeH265WeightTableFlags {
    /// each bit n represents the nth entry in reference list l0, n <= num_ref_idx_l0_active_minus1
    pub luma_weight_l0_flag: u16,
    /// each bit n represents the nth entry in reference list l0, n <= num_ref_idx_l0_active_minus1
    pub chroma_weight_l0_flag: u16,
    /// each bit n represents the nth entry in reference list l1, n <= num_ref_idx_l1_active_minus1
    pub luma_weight_l1_flag: u16,
    /// each bit n represents the nth entry in reference list l1, n <= num_ref_idx_l1_active_minus1
    pub chroma_weight_l1_flag: u16,
}
impl Default for EncodeH265WeightTableFlags {
    fn default() -> Self {
        Self {
            luma_weight_l0_flag: 0,
            chroma_weight_l0_flag: 0,
            luma_weight_l1_flag: 0,
            chroma_weight_l1_flag: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EncodeH265WeightTable {
    pub flags: EncodeH265WeightTableFlags,
    /// [0, 7]
    pub luma_log2_weight_denom: u8,
    pub delta_chroma_log2_weight_denom: i8,
    /// comment
    pub delta_luma_weight_l0: [i8; H265_MAX_NUM_LIST_REF as usize],
    /// comment
    pub luma_offset_l0: [i8; H265_MAX_NUM_LIST_REF as usize],
    /// [i][j]: valid entry range for i is [0, num_ref_idx_l0_active_minus1]; j = 0 for Cb, j = 1 for Cr
    pub delta_chroma_weight_l0:
        [[i8; H265_MAX_CHROMA_PLANES as usize]; H265_MAX_NUM_LIST_REF as usize],
    /// [i][j]: valid entry range for i is [0, num_ref_idx_l0_active_minus1]; j = 0 for Cb, j = 1 for Cr
    pub delta_chroma_offset_l0:
        [[i8; H265_MAX_CHROMA_PLANES as usize]; H265_MAX_NUM_LIST_REF as usize],
    pub delta_luma_weight_l1: [i8; H265_MAX_NUM_LIST_REF as usize],
    pub luma_offset_l1: [i8; H265_MAX_NUM_LIST_REF as usize],
    /// [i][j]: valid entry range for i is [0, num_ref_idx_l1_active_minus1]; j = 0 for Cb, j = 1 for Cr
    pub delta_chroma_weight_l1:
        [[i8; H265_MAX_CHROMA_PLANES as usize]; H265_MAX_NUM_LIST_REF as usize],
    /// [i][j]: valid entry range for i is [0, num_ref_idx_l1_active_minus1]; j = 0 for Cb, j = 1 for Cr
    pub delta_chroma_offset_l1:
        [[i8; H265_MAX_CHROMA_PLANES as usize]; H265_MAX_NUM_LIST_REF as usize],
}
impl Default for EncodeH265WeightTable {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            luma_log2_weight_denom: 0,
            delta_chroma_log2_weight_denom: 0,
            delta_luma_weight_l0: unsafe { mem::zeroed() },
            luma_offset_l0: unsafe { mem::zeroed() },
            delta_chroma_weight_l0: unsafe { mem::zeroed() },
            delta_chroma_offset_l0: unsafe { mem::zeroed() },
            delta_luma_weight_l1: unsafe { mem::zeroed() },
            luma_offset_l1: unsafe { mem::zeroed() },
            delta_chroma_weight_l1: unsafe { mem::zeroed() },
            delta_chroma_offset_l1: unsafe { mem::zeroed() },
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EncodeH265LongTermRefPics {
    pub num_long_term_sps: u8,
    pub num_long_term_pics: u8,
    pub lt_idx_sps: [u8; H265_MAX_LONG_TERM_REF_PICS_SPS as usize],
    pub poc_lsb_lt: [u8; H265_MAX_LONG_TERM_PICS as usize],
    /// each bit represents a used_by_curr_pic_lt_flag[i] syntax
    pub used_by_curr_pic_lt_flag: u16,
    pub delta_poc_msb_present_flag: [u8; H265_MAX_DELTA_POC as usize],
    pub delta_poc_msb_cycle_lt: [u8; H265_MAX_DELTA_POC as usize],
}
impl Default for EncodeH265LongTermRefPics {
    fn default() -> Self {
        Self {
            num_long_term_sps: 0,
            num_long_term_pics: 0,
            lt_idx_sps: unsafe { mem::zeroed() },
            poc_lsb_lt: unsafe { mem::zeroed() },
            used_by_curr_pic_lt_flag: 0,
            delta_poc_msb_present_flag: unsafe { mem::zeroed() },
            delta_poc_msb_cycle_lt: unsafe { mem::zeroed() },
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EncodeH265SliceSegmentHeaderFlags {
    pub first_slice_segment_in_pic_flag: u32,
    pub dependent_slice_segment_flag: u32,
    pub slice_sao_luma_flag: u32,
    pub slice_sao_chroma_flag: u32,
    pub num_ref_idx_active_override_flag: u32,
    pub mvd_l1_zero_flag: u32,
    pub cabac_init_flag: u32,
    pub cu_chroma_qp_offset_enabled_flag: u32,
    pub deblocking_filter_override_flag: u32,
    pub slice_deblocking_filter_disabled_flag: u32,
    pub collocated_from_l0_flag: u32,
    pub slice_loop_filter_across_slices_enabled_flag: u32,
    pub reserved: u32,
}
impl Default for EncodeH265SliceSegmentHeaderFlags {
    fn default() -> Self {
        Self {
            first_slice_segment_in_pic_flag: 0,
            dependent_slice_segment_flag: 0,
            slice_sao_luma_flag: 0,
            slice_sao_chroma_flag: 0,
            num_ref_idx_active_override_flag: 0,
            mvd_l1_zero_flag: 0,
            cabac_init_flag: 0,
            cu_chroma_qp_offset_enabled_flag: 0,
            deblocking_filter_override_flag: 0,
            slice_deblocking_filter_disabled_flag: 0,
            collocated_from_l0_flag: 0,
            slice_loop_filter_across_slices_enabled_flag: 0,
            reserved: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EncodeH265SliceSegmentHeader<'a> {
    pub flags: EncodeH265SliceSegmentHeaderFlags,
    pub slice_type: H265SliceType,
    pub slice_segment_address: u32,
    pub collocated_ref_idx: u8,
    pub max_num_merge_cand: u8,
    /// [-12, 12]
    pub slice_cb_qp_offset: i8,
    /// [-12, 12]
    pub slice_cr_qp_offset: i8,
    /// [-6, 6]
    pub slice_beta_offset_div2: i8,
    /// [-6, 6]
    pub slice_tc_offset_div2: i8,
    pub slice_act_y_qp_offset: i8,
    pub slice_act_cb_qp_offset: i8,
    pub slice_act_cr_qp_offset: i8,
    pub slice_qp_delta: i8,
    /// Reserved for future use and must be initialized with 0.
    pub reserved1: u16,
    /// Nullable
    pub weight_table: *const EncodeH265WeightTable,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for EncodeH265SliceSegmentHeader<'_> {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            slice_type: Default::default(),
            slice_segment_address: 0,
            collocated_ref_idx: 0,
            max_num_merge_cand: 0,
            slice_cb_qp_offset: 0,
            slice_cr_qp_offset: 0,
            slice_beta_offset_div2: 0,
            slice_tc_offset_div2: 0,
            slice_act_y_qp_offset: 0,
            slice_act_cb_qp_offset: 0,
            slice_act_cr_qp_offset: 0,
            slice_qp_delta: 0,
            reserved1: 0,
            weight_table: ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EncodeH265ReferenceListsInfoFlags {
    pub ref_pic_list_modification_flag_l0: u32,
    pub ref_pic_list_modification_flag_l1: u32,
    pub reserved: u32,
}
impl Default for EncodeH265ReferenceListsInfoFlags {
    fn default() -> Self {
        Self {
            ref_pic_list_modification_flag_l0: 0,
            ref_pic_list_modification_flag_l1: 0,
            reserved: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EncodeH265ReferenceListsInfo {
    pub flags: EncodeH265ReferenceListsInfoFlags,
    pub num_ref_idx_l0_active_minus1: u8,
    pub num_ref_idx_l1_active_minus1: u8,
    /// slotIndex as used in VkVideoReferenceSlotInfoKHR structures or STD_VIDEO_H265_NO_REFERENCE_PICTURE
    pub ref_pic_list0: [u8; H265_MAX_NUM_LIST_REF as usize],
    /// slotIndex as used in VkVideoReferenceSlotInfoKHR structures or STD_VIDEO_H265_NO_REFERENCE_PICTURE
    pub ref_pic_list1: [u8; H265_MAX_NUM_LIST_REF as usize],
    pub list_entry_l0: [u8; H265_MAX_NUM_LIST_REF as usize],
    pub list_entry_l1: [u8; H265_MAX_NUM_LIST_REF as usize],
}
impl Default for EncodeH265ReferenceListsInfo {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            num_ref_idx_l0_active_minus1: 0,
            num_ref_idx_l1_active_minus1: 0,
            ref_pic_list0: unsafe { mem::zeroed() },
            ref_pic_list1: unsafe { mem::zeroed() },
            list_entry_l0: unsafe { mem::zeroed() },
            list_entry_l1: unsafe { mem::zeroed() },
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EncodeH265PictureInfoFlags {
    /// A reference picture, as defined in clause 3.132
    pub is_reference: u32,
    /// A reference picture, as defined in clause 3.73
    pub irap_pic_flag: u32,
    /// A picture that is marked as "used for long-term reference", derived binary value from clause 8.3.2 Decoding process for reference picture set
    pub used_for_long_term_reference: u32,
    pub discardable_flag: u32,
    pub cross_layer_bla_flag: u32,
    pub pic_output_flag: u32,
    pub no_output_of_prior_pics_flag: u32,
    pub short_term_ref_pic_set_sps_flag: u32,
    pub slice_temporal_mvp_enabled_flag: u32,
    pub reserved: u32,
}
impl Default for EncodeH265PictureInfoFlags {
    fn default() -> Self {
        Self {
            is_reference: 0,
            irap_pic_flag: 0,
            used_for_long_term_reference: 0,
            discardable_flag: 0,
            cross_layer_bla_flag: 0,
            pic_output_flag: 0,
            no_output_of_prior_pics_flag: 0,
            short_term_ref_pic_set_sps_flag: 0,
            slice_temporal_mvp_enabled_flag: 0,
            reserved: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EncodeH265PictureInfo<'a> {
    pub flags: EncodeH265PictureInfoFlags,
    pub pic_type: H265PictureType,
    /// Selecting VPS id from the Video Parameters Set
    pub sps_video_parameter_set_id: u8,
    /// Selecting SPS id from the Sequence Parameters Set
    pub pps_seq_parameter_set_id: u8,
    /// Selecting PPS id from the Picture Parameters Set
    pub pps_pic_parameter_set_id: u8,
    pub short_term_ref_pic_set_idx: u8,
    /// Picture order count derived as specified in 8.3.1
    pub pic_order_cnt_val: i32,
    /// Temporal ID, as defined in 7.4.2.2
    pub temporal_id: u8,
    /// Reserved for future use and must be initialized with 0.
    pub reserved1: [u8; 7],
    /// Nullable
    pub ref_lists: *const EncodeH265ReferenceListsInfo,
    /// Must be a valid pointer if short_term_ref_pic_set_sps_flag is not set
    /// Nullable
    pub short_term_ref_pic_set: *const H265ShortTermRefPicSet,
    /// Must be a valid pointer if long_term_ref_pics_present_flag is set
    /// Nullable
    pub long_term_ref_pics: *const EncodeH265LongTermRefPics,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for EncodeH265PictureInfo<'_> {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            pic_type: Default::default(),
            sps_video_parameter_set_id: 0,
            pps_seq_parameter_set_id: 0,
            pps_pic_parameter_set_id: 0,
            short_term_ref_pic_set_idx: 0,
            pic_order_cnt_val: 0,
            temporal_id: 0,
            reserved1: unsafe { mem::zeroed() },
            ref_lists: ptr::null_mut(),
            short_term_ref_pic_set: ptr::null_mut(),
            long_term_ref_pics: ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EncodeH265ReferenceInfoFlags {
    /// A picture that is marked as "used for long-term reference", derived binary value from clause 8.3.2 Decoding process for reference picture set
    pub used_for_long_term_reference: u32,
    /// A picture that is marked as "unused for reference", derived binary value from clause 8.3.2 Decoding process for reference picture set
    pub unused_for_reference: u32,
    pub reserved: u32,
}
impl Default for EncodeH265ReferenceInfoFlags {
    fn default() -> Self {
        Self {
            used_for_long_term_reference: 0,
            unused_for_reference: 0,
            reserved: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EncodeH265ReferenceInfo {
    pub flags: EncodeH265ReferenceInfoFlags,
    pub pic_type: H265PictureType,
    /// Picture order count derived as specified in 8.3.1
    pub pic_order_cnt_val: i32,
    /// Temporal ID, as defined in 7.4.2.2
    pub temporal_id: u8,
}
impl Default for EncodeH265ReferenceInfo {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            pic_type: Default::default(),
            pic_order_cnt_val: 0,
            temporal_id: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VP9ColorConfigFlags {
    pub color_range: u32,
    pub reserved: u32,
}
impl Default for VP9ColorConfigFlags {
    fn default() -> Self {
        Self {
            color_range: 0,
            reserved: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VP9ColorConfig {
    pub flags: VP9ColorConfigFlags,
    pub bit_depth: u8,
    pub subsampling_x: u8,
    pub subsampling_y: u8,
    /// Reserved for future use and must be initialized with 0.
    pub reserved1: u8,
    pub color_space: VP9ColorSpace,
}
impl Default for VP9ColorConfig {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            bit_depth: 0,
            subsampling_x: 0,
            subsampling_y: 0,
            reserved1: 0,
            color_space: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VP9LoopFilterFlags {
    pub loop_filter_delta_enabled: u32,
    pub loop_filter_delta_update: u32,
    pub reserved: u32,
}
impl Default for VP9LoopFilterFlags {
    fn default() -> Self {
        Self {
            loop_filter_delta_enabled: 0,
            loop_filter_delta_update: 0,
            reserved: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VP9LoopFilter {
    pub flags: VP9LoopFilterFlags,
    pub loop_filter_level: u8,
    pub loop_filter_sharpness: u8,
    pub update_ref_delta: u8,
    pub loop_filter_ref_deltas: [i8; VP9_MAX_REF_FRAMES as usize],
    pub update_mode_delta: u8,
    pub loop_filter_mode_deltas: [i8; VP9_LOOP_FILTER_ADJUSTMENTS as usize],
}
impl Default for VP9LoopFilter {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            loop_filter_level: 0,
            loop_filter_sharpness: 0,
            update_ref_delta: 0,
            loop_filter_ref_deltas: unsafe { mem::zeroed() },
            update_mode_delta: 0,
            loop_filter_mode_deltas: unsafe { mem::zeroed() },
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VP9SegmentationFlags {
    pub segmentation_update_map: u32,
    pub segmentation_temporal_update: u32,
    pub segmentation_update_data: u32,
    pub segmentation_abs_or_delta_update: u32,
    pub reserved: u32,
}
impl Default for VP9SegmentationFlags {
    fn default() -> Self {
        Self {
            segmentation_update_map: 0,
            segmentation_temporal_update: 0,
            segmentation_update_data: 0,
            segmentation_abs_or_delta_update: 0,
            reserved: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VP9Segmentation {
    pub flags: VP9SegmentationFlags,
    pub segmentation_tree_probs: [u8; VP9_MAX_SEGMENTATION_TREE_PROBS as usize],
    pub segmentation_pred_prob: [u8; VP9_MAX_SEGMENTATION_PRED_PROB as usize],
    pub feature_enabled: [u8; VP9_MAX_SEGMENTS as usize],
    pub feature_data: [[i16; VP9_SEG_LVL_MAX as usize]; VP9_MAX_SEGMENTS as usize],
}
impl Default for VP9Segmentation {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            segmentation_tree_probs: unsafe { mem::zeroed() },
            segmentation_pred_prob: unsafe { mem::zeroed() },
            feature_enabled: unsafe { mem::zeroed() },
            feature_data: unsafe { mem::zeroed() },
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DecodeVP9PictureInfoFlags {
    pub error_resilient_mode: u32,
    pub intra_only: u32,
    pub allow_high_precision_mv: u32,
    pub refresh_frame_context: u32,
    pub frame_parallel_decoding_mode: u32,
    pub segmentation_enabled: u32,
    pub show_frame: u32,
    pub use_prev_frame_mvs: u32,
    pub reserved: u32,
}
impl Default for DecodeVP9PictureInfoFlags {
    fn default() -> Self {
        Self {
            error_resilient_mode: 0,
            intra_only: 0,
            allow_high_precision_mv: 0,
            refresh_frame_context: 0,
            frame_parallel_decoding_mode: 0,
            segmentation_enabled: 0,
            show_frame: 0,
            use_prev_frame_mvs: 0,
            reserved: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DecodeVP9PictureInfo<'a> {
    pub flags: DecodeVP9PictureInfoFlags,
    pub profile: VP9Profile,
    pub frame_type: VP9FrameType,
    pub frame_context_idx: u8,
    pub reset_frame_context: u8,
    pub refresh_frame_flags: u8,
    pub ref_frame_sign_bias_mask: u8,
    pub interpolation_filter: VP9InterpolationFilter,
    pub base_q_idx: u8,
    pub delta_q_y_dc: i8,
    pub delta_q_uv_dc: i8,
    pub delta_q_uv_ac: i8,
    pub tile_cols_log2: u8,
    pub tile_rows_log2: u8,
    /// Reserved for future use and must be initialized with 0.
    pub reserved1: [u16; 3],
    pub color_config: *const VP9ColorConfig,
    pub loop_filter: *const VP9LoopFilter,
    /// Nullable
    pub segmentation: *const VP9Segmentation,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DecodeVP9PictureInfo<'_> {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            profile: Default::default(),
            frame_type: Default::default(),
            frame_context_idx: 0,
            reset_frame_context: 0,
            refresh_frame_flags: 0,
            ref_frame_sign_bias_mask: 0,
            interpolation_filter: Default::default(),
            base_q_idx: 0,
            delta_q_y_dc: 0,
            delta_q_uv_dc: 0,
            delta_q_uv_ac: 0,
            tile_cols_log2: 0,
            tile_rows_log2: 0,
            reserved1: unsafe { mem::zeroed() },
            color_config: ptr::null_mut(),
            loop_filter: ptr::null_mut(),
            segmentation: ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AV1ColorConfigFlags {
    pub mono_chrome: u32,
    pub color_range: u32,
    pub separate_uv_delta_q: u32,
    pub color_description_present_flag: u32,
    pub reserved: u32,
}
impl Default for AV1ColorConfigFlags {
    fn default() -> Self {
        Self {
            mono_chrome: 0,
            color_range: 0,
            separate_uv_delta_q: 0,
            color_description_present_flag: 0,
            reserved: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AV1ColorConfig {
    pub flags: AV1ColorConfigFlags,
    pub bit_depth: u8,
    pub subsampling_x: u8,
    pub subsampling_y: u8,
    /// Reserved for future use and must be initialized with 0.
    pub reserved1: u8,
    pub color_primaries: AV1ColorPrimaries,
    pub transfer_characteristics: AV1TransferCharacteristics,
    pub matrix_coefficients: AV1MatrixCoefficients,
    pub chroma_sample_position: AV1ChromaSamplePosition,
}
impl Default for AV1ColorConfig {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            bit_depth: 0,
            subsampling_x: 0,
            subsampling_y: 0,
            reserved1: 0,
            color_primaries: Default::default(),
            transfer_characteristics: Default::default(),
            matrix_coefficients: Default::default(),
            chroma_sample_position: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AV1TimingInfoFlags {
    pub equal_picture_interval: u32,
    pub reserved: u32,
}
impl Default for AV1TimingInfoFlags {
    fn default() -> Self {
        Self {
            equal_picture_interval: 0,
            reserved: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AV1TimingInfo {
    pub flags: AV1TimingInfoFlags,
    pub num_units_in_display_tick: u32,
    pub time_scale: u32,
    pub num_ticks_per_picture_minus_1: u32,
}
impl Default for AV1TimingInfo {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            num_units_in_display_tick: 0,
            time_scale: 0,
            num_ticks_per_picture_minus_1: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AV1SequenceHeaderFlags {
    pub still_picture: u32,
    pub reduced_still_picture_header: u32,
    pub use_128x128_superblock: u32,
    pub enable_filter_intra: u32,
    pub enable_intra_edge_filter: u32,
    pub enable_interintra_compound: u32,
    pub enable_masked_compound: u32,
    pub enable_warped_motion: u32,
    pub enable_dual_filter: u32,
    pub enable_order_hint: u32,
    pub enable_jnt_comp: u32,
    pub enable_ref_frame_mvs: u32,
    pub frame_id_numbers_present_flag: u32,
    pub enable_superres: u32,
    pub enable_cdef: u32,
    pub enable_restoration: u32,
    pub film_grain_params_present: u32,
    pub timing_info_present_flag: u32,
    pub initial_display_delay_present_flag: u32,
    pub reserved: u32,
}
impl Default for AV1SequenceHeaderFlags {
    fn default() -> Self {
        Self {
            still_picture: 0,
            reduced_still_picture_header: 0,
            use_128x128_superblock: 0,
            enable_filter_intra: 0,
            enable_intra_edge_filter: 0,
            enable_interintra_compound: 0,
            enable_masked_compound: 0,
            enable_warped_motion: 0,
            enable_dual_filter: 0,
            enable_order_hint: 0,
            enable_jnt_comp: 0,
            enable_ref_frame_mvs: 0,
            frame_id_numbers_present_flag: 0,
            enable_superres: 0,
            enable_cdef: 0,
            enable_restoration: 0,
            film_grain_params_present: 0,
            timing_info_present_flag: 0,
            initial_display_delay_present_flag: 0,
            reserved: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AV1SequenceHeader<'a> {
    pub flags: AV1SequenceHeaderFlags,
    pub seq_profile: AV1Profile,
    pub frame_width_bits_minus_1: u8,
    pub frame_height_bits_minus_1: u8,
    pub max_frame_width_minus_1: u16,
    pub max_frame_height_minus_1: u16,
    pub delta_frame_id_length_minus_2: u8,
    pub additional_frame_id_length_minus_1: u8,
    pub order_hint_bits_minus_1: u8,
    /// The final value of of seq_force_integer_mv per the value of seq_choose_integer_mv.
    pub seq_force_integer_mv: u8,
    /// The final value of of seq_force_screen_content_tools per the value of seq_choose_screen_content_tools.
    pub seq_force_screen_content_tools: u8,
    /// Reserved for future use and must be initialized with 0.
    pub reserved1: [u8; 5],
    pub color_config: *const AV1ColorConfig,
    /// Nullable
    pub timing_info: *const AV1TimingInfo,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for AV1SequenceHeader<'_> {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            seq_profile: Default::default(),
            frame_width_bits_minus_1: 0,
            frame_height_bits_minus_1: 0,
            max_frame_width_minus_1: 0,
            max_frame_height_minus_1: 0,
            delta_frame_id_length_minus_2: 0,
            additional_frame_id_length_minus_1: 0,
            order_hint_bits_minus_1: 0,
            seq_force_integer_mv: 0,
            seq_force_screen_content_tools: 0,
            reserved1: unsafe { mem::zeroed() },
            color_config: ptr::null_mut(),
            timing_info: ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AV1LoopFilterFlags {
    pub loop_filter_delta_enabled: u32,
    pub loop_filter_delta_update: u32,
    pub reserved: u32,
}
impl Default for AV1LoopFilterFlags {
    fn default() -> Self {
        Self {
            loop_filter_delta_enabled: 0,
            loop_filter_delta_update: 0,
            reserved: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AV1LoopFilter {
    pub flags: AV1LoopFilterFlags,
    pub loop_filter_level: [u8; AV1_MAX_LOOP_FILTER_STRENGTHS as usize],
    pub loop_filter_sharpness: u8,
    pub update_ref_delta: u8,
    pub loop_filter_ref_deltas: [i8; AV1_TOTAL_REFS_PER_FRAME as usize],
    pub update_mode_delta: u8,
    pub loop_filter_mode_deltas: [i8; AV1_LOOP_FILTER_ADJUSTMENTS as usize],
}
impl Default for AV1LoopFilter {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            loop_filter_level: unsafe { mem::zeroed() },
            loop_filter_sharpness: 0,
            update_ref_delta: 0,
            loop_filter_ref_deltas: unsafe { mem::zeroed() },
            update_mode_delta: 0,
            loop_filter_mode_deltas: unsafe { mem::zeroed() },
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AV1QuantizationFlags {
    pub using_qmatrix: u32,
    pub diff_uv_delta: u32,
    pub reserved: u32,
}
impl Default for AV1QuantizationFlags {
    fn default() -> Self {
        Self {
            using_qmatrix: 0,
            diff_uv_delta: 0,
            reserved: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AV1Quantization {
    pub flags: AV1QuantizationFlags,
    pub base_q_idx: u8,
    pub delta_qy_dc: i8,
    pub delta_qu_dc: i8,
    pub delta_qu_ac: i8,
    pub delta_qv_dc: i8,
    pub delta_qv_ac: i8,
    pub qm_y: u8,
    pub qm_u: u8,
    pub qm_v: u8,
}
impl Default for AV1Quantization {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            base_q_idx: 0,
            delta_qy_dc: 0,
            delta_qu_dc: 0,
            delta_qu_ac: 0,
            delta_qv_dc: 0,
            delta_qv_ac: 0,
            qm_y: 0,
            qm_u: 0,
            qm_v: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AV1Segmentation {
    /// Each element contains 8 (SEG_LVL_MAX) bits, one bit for each feature within the segment
    pub feature_enabled: [u8; AV1_MAX_SEGMENTS as usize],
    pub feature_data: [[i16; AV1_SEG_LVL_MAX as usize]; AV1_MAX_SEGMENTS as usize],
}
impl Default for AV1Segmentation {
    fn default() -> Self {
        Self {
            feature_enabled: unsafe { mem::zeroed() },
            feature_data: unsafe { mem::zeroed() },
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AV1TileInfoFlags {
    pub uniform_tile_spacing_flag: u32,
    pub reserved: u32,
}
impl Default for AV1TileInfoFlags {
    fn default() -> Self {
        Self {
            uniform_tile_spacing_flag: 0,
            reserved: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AV1TileInfo<'a> {
    pub flags: AV1TileInfoFlags,
    pub tile_cols: u8,
    pub tile_rows: u8,
    pub context_update_tile_id: u16,
    pub tile_size_bytes_minus_1: u8,
    /// Reserved for future use and must be initialized with 0.
    pub reserved1: [u8; 7],
    /// TileCols number of elements
    /// Len: `tile_cols`
    pub mi_col_starts: *const u16,
    /// TileRows number of elements
    /// Len: `tile_rows`
    pub mi_row_starts: *const u16,
    /// TileCols number of elements
    /// Len: `tile_cols`
    pub width_in_sbs_minus1: *const u16,
    /// TileRows number of elements
    /// Len: `tile_rows`
    pub height_in_sbs_minus1: *const u16,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for AV1TileInfo<'_> {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            tile_cols: 0,
            tile_rows: 0,
            context_update_tile_id: 0,
            tile_size_bytes_minus_1: 0,
            reserved1: unsafe { mem::zeroed() },
            mi_col_starts: ptr::null_mut(),
            mi_row_starts: ptr::null_mut(),
            width_in_sbs_minus1: ptr::null_mut(),
            height_in_sbs_minus1: ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AV1CDEF {
    pub cdef_damping_minus_3: u8,
    pub cdef_bits: u8,
    pub cdef_y_pri_strength: [u8; AV1_MAX_CDEF_FILTER_STRENGTHS as usize],
    pub cdef_y_sec_strength: [u8; AV1_MAX_CDEF_FILTER_STRENGTHS as usize],
    pub cdef_uv_pri_strength: [u8; AV1_MAX_CDEF_FILTER_STRENGTHS as usize],
    pub cdef_uv_sec_strength: [u8; AV1_MAX_CDEF_FILTER_STRENGTHS as usize],
}
impl Default for AV1CDEF {
    fn default() -> Self {
        Self {
            cdef_damping_minus_3: 0,
            cdef_bits: 0,
            cdef_y_pri_strength: unsafe { mem::zeroed() },
            cdef_y_sec_strength: unsafe { mem::zeroed() },
            cdef_uv_pri_strength: unsafe { mem::zeroed() },
            cdef_uv_sec_strength: unsafe { mem::zeroed() },
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AV1LoopRestoration {
    pub frame_restoration_type: [AV1FrameRestorationType; AV1_MAX_NUM_PLANES as usize],
    pub loop_restoration_size: [u16; AV1_MAX_NUM_PLANES as usize],
}
impl Default for AV1LoopRestoration {
    fn default() -> Self {
        Self {
            frame_restoration_type: unsafe { mem::zeroed() },
            loop_restoration_size: unsafe { mem::zeroed() },
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AV1GlobalMotion {
    pub gm_type: [u8; AV1_NUM_REF_FRAMES as usize],
    pub gm_params: [[i32; AV1_GLOBAL_MOTION_PARAMS as usize]; AV1_NUM_REF_FRAMES as usize],
}
impl Default for AV1GlobalMotion {
    fn default() -> Self {
        Self {
            gm_type: unsafe { mem::zeroed() },
            gm_params: unsafe { mem::zeroed() },
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AV1FilmGrainFlags {
    pub chroma_scaling_from_luma: u32,
    pub overlap_flag: u32,
    pub clip_to_restricted_range: u32,
    pub update_grain: u32,
    pub reserved: u32,
}
impl Default for AV1FilmGrainFlags {
    fn default() -> Self {
        Self {
            chroma_scaling_from_luma: 0,
            overlap_flag: 0,
            clip_to_restricted_range: 0,
            update_grain: 0,
            reserved: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AV1FilmGrain {
    pub flags: AV1FilmGrainFlags,
    pub grain_scaling_minus_8: u8,
    pub ar_coeff_lag: u8,
    pub ar_coeff_shift_minus_6: u8,
    pub grain_scale_shift: u8,
    pub grain_seed: u16,
    pub film_grain_params_ref_idx: u8,
    pub num_y_points: u8,
    pub point_y_value: [u8; AV1_MAX_NUM_Y_POINTS as usize],
    pub point_y_scaling: [u8; AV1_MAX_NUM_Y_POINTS as usize],
    pub num_cb_points: u8,
    pub point_cb_value: [u8; AV1_MAX_NUM_CB_POINTS as usize],
    pub point_cb_scaling: [u8; AV1_MAX_NUM_CB_POINTS as usize],
    pub num_cr_points: u8,
    pub point_cr_value: [u8; AV1_MAX_NUM_CR_POINTS as usize],
    pub point_cr_scaling: [u8; AV1_MAX_NUM_CR_POINTS as usize],
    pub ar_coeffs_y_plus_128: [i8; AV1_MAX_NUM_POS_LUMA as usize],
    pub ar_coeffs_cb_plus_128: [i8; AV1_MAX_NUM_POS_CHROMA as usize],
    pub ar_coeffs_cr_plus_128: [i8; AV1_MAX_NUM_POS_CHROMA as usize],
    pub cb_mult: u8,
    pub cb_luma_mult: u8,
    pub cb_offset: u16,
    pub cr_mult: u8,
    pub cr_luma_mult: u8,
    pub cr_offset: u16,
}
impl Default for AV1FilmGrain {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            grain_scaling_minus_8: 0,
            ar_coeff_lag: 0,
            ar_coeff_shift_minus_6: 0,
            grain_scale_shift: 0,
            grain_seed: 0,
            film_grain_params_ref_idx: 0,
            num_y_points: 0,
            point_y_value: unsafe { mem::zeroed() },
            point_y_scaling: unsafe { mem::zeroed() },
            num_cb_points: 0,
            point_cb_value: unsafe { mem::zeroed() },
            point_cb_scaling: unsafe { mem::zeroed() },
            num_cr_points: 0,
            point_cr_value: unsafe { mem::zeroed() },
            point_cr_scaling: unsafe { mem::zeroed() },
            ar_coeffs_y_plus_128: unsafe { mem::zeroed() },
            ar_coeffs_cb_plus_128: unsafe { mem::zeroed() },
            ar_coeffs_cr_plus_128: unsafe { mem::zeroed() },
            cb_mult: 0,
            cb_luma_mult: 0,
            cb_offset: 0,
            cr_mult: 0,
            cr_luma_mult: 0,
            cr_offset: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DecodeAV1PictureInfoFlags {
    pub error_resilient_mode: u32,
    pub disable_cdf_update: u32,
    pub use_superres: u32,
    pub render_and_frame_size_different: u32,
    pub allow_screen_content_tools: u32,
    pub is_filter_switchable: u32,
    pub force_integer_mv: u32,
    pub frame_size_override_flag: u32,
    pub buffer_removal_time_present_flag: u32,
    pub allow_intrabc: u32,
    pub frame_refs_short_signaling: u32,
    pub allow_high_precision_mv: u32,
    pub is_motion_mode_switchable: u32,
    pub use_ref_frame_mvs: u32,
    pub disable_frame_end_update_cdf: u32,
    pub allow_warped_motion: u32,
    pub reduced_tx_set: u32,
    pub reference_select: u32,
    pub skip_mode_present: u32,
    pub delta_q_present: u32,
    pub delta_lf_present: u32,
    pub delta_lf_multi: u32,
    pub segmentation_enabled: u32,
    pub segmentation_update_map: u32,
    pub segmentation_temporal_update: u32,
    pub segmentation_update_data: u32,
    pub uses_lr: u32,
    pub uses_chroma_lr: u32,
    pub apply_grain: u32,
    pub reserved: u32,
}
impl Default for DecodeAV1PictureInfoFlags {
    fn default() -> Self {
        Self {
            error_resilient_mode: 0,
            disable_cdf_update: 0,
            use_superres: 0,
            render_and_frame_size_different: 0,
            allow_screen_content_tools: 0,
            is_filter_switchable: 0,
            force_integer_mv: 0,
            frame_size_override_flag: 0,
            buffer_removal_time_present_flag: 0,
            allow_intrabc: 0,
            frame_refs_short_signaling: 0,
            allow_high_precision_mv: 0,
            is_motion_mode_switchable: 0,
            use_ref_frame_mvs: 0,
            disable_frame_end_update_cdf: 0,
            allow_warped_motion: 0,
            reduced_tx_set: 0,
            reference_select: 0,
            skip_mode_present: 0,
            delta_q_present: 0,
            delta_lf_present: 0,
            delta_lf_multi: 0,
            segmentation_enabled: 0,
            segmentation_update_map: 0,
            segmentation_temporal_update: 0,
            segmentation_update_data: 0,
            uses_lr: 0,
            uses_chroma_lr: 0,
            apply_grain: 0,
            reserved: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DecodeAV1PictureInfo<'a> {
    pub flags: DecodeAV1PictureInfoFlags,
    pub frame_type: AV1FrameType,
    pub current_frame_id: u32,
    pub order_hint: u8,
    pub primary_ref_frame: u8,
    pub refresh_frame_flags: u8,
    /// Reserved for future use and must be initialized with 0.
    pub reserved1: u8,
    pub interpolation_filter: AV1InterpolationFilter,
    pub tx_mode: AV1TxMode,
    pub delta_q_res: u8,
    pub delta_lf_res: u8,
    pub skip_mode_frame: [u8; AV1_SKIP_MODE_FRAMES as usize],
    pub coded_denom: u8,
    /// Reserved for future use and must be initialized with 0.
    pub reserved2: [u8; 3],
    pub order_hints: [u8; AV1_NUM_REF_FRAMES as usize],
    pub expected_frame_id: [u32; AV1_NUM_REF_FRAMES as usize],
    pub tile_info: *const AV1TileInfo<'a>,
    pub quantization: *const AV1Quantization,
    /// Nullable
    pub segmentation: *const AV1Segmentation,
    pub loop_filter: *const AV1LoopFilter,
    /// Nullable
    pub cdef: *const AV1CDEF,
    /// Nullable
    pub loop_restoration: *const AV1LoopRestoration,
    pub global_motion: *const AV1GlobalMotion,
    /// Nullable
    pub film_grain: *const AV1FilmGrain,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DecodeAV1PictureInfo<'_> {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            frame_type: Default::default(),
            current_frame_id: 0,
            order_hint: 0,
            primary_ref_frame: 0,
            refresh_frame_flags: 0,
            reserved1: 0,
            interpolation_filter: Default::default(),
            tx_mode: Default::default(),
            delta_q_res: 0,
            delta_lf_res: 0,
            skip_mode_frame: unsafe { mem::zeroed() },
            coded_denom: 0,
            reserved2: unsafe { mem::zeroed() },
            order_hints: unsafe { mem::zeroed() },
            expected_frame_id: unsafe { mem::zeroed() },
            tile_info: ptr::null_mut(),
            quantization: ptr::null_mut(),
            segmentation: ptr::null_mut(),
            loop_filter: ptr::null_mut(),
            cdef: ptr::null_mut(),
            loop_restoration: ptr::null_mut(),
            global_motion: ptr::null_mut(),
            film_grain: ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DecodeAV1ReferenceInfoFlags {
    pub disable_frame_end_update_cdf: u32,
    pub segmentation_enabled: u32,
    pub reserved: u32,
}
impl Default for DecodeAV1ReferenceInfoFlags {
    fn default() -> Self {
        Self {
            disable_frame_end_update_cdf: 0,
            segmentation_enabled: 0,
            reserved: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DecodeAV1ReferenceInfo {
    pub flags: DecodeAV1ReferenceInfoFlags,
    pub frame_type: u8,
    pub ref_frame_sign_bias: u8,
    pub order_hint: u8,
    pub saved_order_hints: [u8; AV1_NUM_REF_FRAMES as usize],
}
impl Default for DecodeAV1ReferenceInfo {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            frame_type: 0,
            ref_frame_sign_bias: 0,
            order_hint: 0,
            saved_order_hints: unsafe { mem::zeroed() },
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EncodeAV1ExtensionHeader {
    pub temporal_id: u8,
    pub spatial_id: u8,
}
impl Default for EncodeAV1ExtensionHeader {
    fn default() -> Self {
        Self {
            temporal_id: 0,
            spatial_id: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EncodeAV1DecoderModelInfo {
    pub buffer_delay_length_minus_1: u8,
    pub buffer_removal_time_length_minus_1: u8,
    pub frame_presentation_time_length_minus_1: u8,
    /// Reserved for future use and must be initialized with 0.
    pub reserved1: u8,
    pub num_units_in_decoding_tick: u32,
}
impl Default for EncodeAV1DecoderModelInfo {
    fn default() -> Self {
        Self {
            buffer_delay_length_minus_1: 0,
            buffer_removal_time_length_minus_1: 0,
            frame_presentation_time_length_minus_1: 0,
            reserved1: 0,
            num_units_in_decoding_tick: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EncodeAV1OperatingPointInfoFlags {
    pub decoder_model_present_for_this_op: u32,
    pub low_delay_mode_flag: u32,
    pub initial_display_delay_present_for_this_op: u32,
    pub reserved: u32,
}
impl Default for EncodeAV1OperatingPointInfoFlags {
    fn default() -> Self {
        Self {
            decoder_model_present_for_this_op: 0,
            low_delay_mode_flag: 0,
            initial_display_delay_present_for_this_op: 0,
            reserved: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EncodeAV1OperatingPointInfo {
    pub flags: EncodeAV1OperatingPointInfoFlags,
    pub operating_point_idc: u16,
    pub seq_level_idx: u8,
    pub seq_tier: u8,
    pub decoder_buffer_delay: u32,
    pub encoder_buffer_delay: u32,
    pub initial_display_delay_minus_1: u8,
}
impl Default for EncodeAV1OperatingPointInfo {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            operating_point_idc: 0,
            seq_level_idx: 0,
            seq_tier: 0,
            decoder_buffer_delay: 0,
            encoder_buffer_delay: 0,
            initial_display_delay_minus_1: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EncodeAV1PictureInfoFlags {
    pub error_resilient_mode: u32,
    pub disable_cdf_update: u32,
    pub use_superres: u32,
    pub render_and_frame_size_different: u32,
    pub allow_screen_content_tools: u32,
    pub is_filter_switchable: u32,
    pub force_integer_mv: u32,
    pub frame_size_override_flag: u32,
    pub buffer_removal_time_present_flag: u32,
    pub allow_intrabc: u32,
    pub frame_refs_short_signaling: u32,
    pub allow_high_precision_mv: u32,
    pub is_motion_mode_switchable: u32,
    pub use_ref_frame_mvs: u32,
    pub disable_frame_end_update_cdf: u32,
    pub allow_warped_motion: u32,
    pub reduced_tx_set: u32,
    pub skip_mode_present: u32,
    pub delta_q_present: u32,
    pub delta_lf_present: u32,
    pub delta_lf_multi: u32,
    pub segmentation_enabled: u32,
    pub segmentation_update_map: u32,
    pub segmentation_temporal_update: u32,
    pub segmentation_update_data: u32,
    pub uses_lr: u32,
    pub uses_chroma_lr: u32,
    pub show_frame: u32,
    pub showable_frame: u32,
    pub reserved: u32,
}
impl Default for EncodeAV1PictureInfoFlags {
    fn default() -> Self {
        Self {
            error_resilient_mode: 0,
            disable_cdf_update: 0,
            use_superres: 0,
            render_and_frame_size_different: 0,
            allow_screen_content_tools: 0,
            is_filter_switchable: 0,
            force_integer_mv: 0,
            frame_size_override_flag: 0,
            buffer_removal_time_present_flag: 0,
            allow_intrabc: 0,
            frame_refs_short_signaling: 0,
            allow_high_precision_mv: 0,
            is_motion_mode_switchable: 0,
            use_ref_frame_mvs: 0,
            disable_frame_end_update_cdf: 0,
            allow_warped_motion: 0,
            reduced_tx_set: 0,
            skip_mode_present: 0,
            delta_q_present: 0,
            delta_lf_present: 0,
            delta_lf_multi: 0,
            segmentation_enabled: 0,
            segmentation_update_map: 0,
            segmentation_temporal_update: 0,
            segmentation_update_data: 0,
            uses_lr: 0,
            uses_chroma_lr: 0,
            show_frame: 0,
            showable_frame: 0,
            reserved: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EncodeAV1PictureInfo<'a> {
    pub flags: EncodeAV1PictureInfoFlags,
    pub frame_type: AV1FrameType,
    pub frame_presentation_time: u32,
    pub current_frame_id: u32,
    pub order_hint: u8,
    pub primary_ref_frame: u8,
    pub refresh_frame_flags: u8,
    pub coded_denom: u8,
    pub render_width_minus_1: u16,
    pub render_height_minus_1: u16,
    pub interpolation_filter: AV1InterpolationFilter,
    pub tx_mode: AV1TxMode,
    pub delta_q_res: u8,
    pub delta_lf_res: u8,
    pub ref_order_hint: [u8; AV1_NUM_REF_FRAMES as usize],
    pub ref_frame_idx: [i8; AV1_REFS_PER_FRAME as usize],
    /// Reserved for future use and must be initialized with 0.
    pub reserved1: [u8; 3],
    pub delta_frame_id_minus_1: [u32; AV1_REFS_PER_FRAME as usize],
    /// Nullable
    pub tile_info: *const AV1TileInfo<'a>,
    pub quantization: *const AV1Quantization,
    /// Nullable
    pub segmentation: *const AV1Segmentation,
    pub loop_filter: *const AV1LoopFilter,
    /// Nullable
    pub cdef: *const AV1CDEF,
    /// Nullable
    pub loop_restoration: *const AV1LoopRestoration,
    pub global_motion: *const AV1GlobalMotion,
    /// Nullable
    pub extension_header: *const EncodeAV1ExtensionHeader,
    /// Nullable
    pub buffer_removal_times: *const u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for EncodeAV1PictureInfo<'_> {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            frame_type: Default::default(),
            frame_presentation_time: 0,
            current_frame_id: 0,
            order_hint: 0,
            primary_ref_frame: 0,
            refresh_frame_flags: 0,
            coded_denom: 0,
            render_width_minus_1: 0,
            render_height_minus_1: 0,
            interpolation_filter: Default::default(),
            tx_mode: Default::default(),
            delta_q_res: 0,
            delta_lf_res: 0,
            ref_order_hint: unsafe { mem::zeroed() },
            ref_frame_idx: unsafe { mem::zeroed() },
            reserved1: unsafe { mem::zeroed() },
            delta_frame_id_minus_1: unsafe { mem::zeroed() },
            tile_info: ptr::null_mut(),
            quantization: ptr::null_mut(),
            segmentation: ptr::null_mut(),
            loop_filter: ptr::null_mut(),
            cdef: ptr::null_mut(),
            loop_restoration: ptr::null_mut(),
            global_motion: ptr::null_mut(),
            extension_header: ptr::null_mut(),
            buffer_removal_times: ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EncodeAV1ReferenceInfoFlags {
    pub disable_frame_end_update_cdf: u32,
    pub segmentation_enabled: u32,
    pub reserved: u32,
}
impl Default for EncodeAV1ReferenceInfoFlags {
    fn default() -> Self {
        Self {
            disable_frame_end_update_cdf: 0,
            segmentation_enabled: 0,
            reserved: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EncodeAV1ReferenceInfo<'a> {
    pub flags: EncodeAV1ReferenceInfoFlags,
    pub ref_frame_id: u32,
    pub frame_type: AV1FrameType,
    pub order_hint: u8,
    /// Reserved for future use and must be initialized with 0.
    pub reserved1: [u8; 3],
    /// Nullable
    pub extension_header: *const EncodeAV1ExtensionHeader,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for EncodeAV1ReferenceInfo<'_> {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            ref_frame_id: 0,
            frame_type: Default::default(),
            order_hint: 0,
            reserved1: unsafe { mem::zeroed() },
            extension_header: ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
