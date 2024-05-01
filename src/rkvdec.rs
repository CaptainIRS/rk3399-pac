#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    swreg0_id: Swreg0Id,
    swreg1_int: Swreg1Int,
    swreg2_sysctrl: Swreg2Sysctrl,
    swreg3_picpar: Swreg3Picpar,
    swreg4_strm_rlc_base: Swreg4StrmRlcBase,
    swreg5_stream_rlc_len: Swreg5StreamRlcLen,
    swreg6_cabactbl_prob_base: Swreg6CabactblProbBase,
    swreg7_decout_base: Swreg7DecoutBase,
    swreg8_y_virstride: Swreg8YVirstride,
    swreg9_yuv_virstride: Swreg9YuvVirstride,
    _reserved_10_swreg10: [u8; 0x04],
    _reserved_11_swreg11: [u8; 0x04],
    _reserved_12_swreg12: [u8; 0x04],
    _reserved_13_swreg13: [u8; 0x04],
    _reserved_14_swreg14: [u8; 0x04],
    _reserved_15_swreg15: [u8; 0x04],
    _reserved_16_swreg16: [u8; 0x04],
    _reserved_17_swreg17: [u8; 0x04],
    _reserved_18_swreg18: [u8; 0x04],
    _reserved_19_swreg19: [u8; 0x04],
    _reserved_20_swreg20: [u8; 0x04],
    _reserved_21_swreg21: [u8; 0x04],
    _reserved_22_swreg22: [u8; 0x04],
    _reserved_23_swreg23: [u8; 0x04],
    _reserved_24_swreg24: [u8; 0x04],
    _reserved_25_swreg25: [u8; 0x04],
    _reserved_26_swreg26: [u8; 0x04],
    _reserved_27_swreg27: [u8; 0x04],
    _reserved_28_swreg28: [u8; 0x04],
    _reserved_29_swreg29: [u8; 0x04],
    _reserved_30_swreg30: [u8; 0x04],
    _reserved_31_swreg31: [u8; 0x04],
    _reserved_32_swreg32: [u8; 0x04],
    _reserved_33_swreg33: [u8; 0x04],
    _reserved_34_swreg34: [u8; 0x04],
    _reserved_35_swreg35: [u8; 0x04],
    _reserved_36_swreg36: [u8; 0x04],
    _reserved_37_swreg37: [u8; 0x04],
    _reserved_38_swreg38: [u8; 0x04],
    _reserved_39_swreg39: [u8; 0x04],
    swreg40_cur_poc: Swreg40CurPoc,
    swreg41_rlcwrite_base: Swreg41RlcwriteBase,
    swreg42_pps_base: Swreg42PpsBase,
    swreg43_rps_base: Swreg43RpsBase,
    swreg44_strmd_error_en: Swreg44StrmdErrorEn,
    _reserved_45_swreg45: [u8; 0x04],
    swreg46_strmd_error_ctu: Swreg46StrmdErrorCtu,
    swreg47_sao_ctu_position: Swreg47SaoCtuPosition,
    _reserved_48_swreg48: [u8; 0x04],
    _reserved_49_swreg49: [u8; 0x04],
    _reserved_50_swreg50: [u8; 0x04],
    _reserved_51_swreg51: [u8; 0x04],
    _reserved_52_swreg52: [u8; 0x04],
    swreg53_h264_refer19_poc: Swreg53H264Refer19Poc,
    swreg54_h264_refer20_poc: Swreg54H264Refer20Poc,
    swreg55_h264_refer21_poc: Swreg55H264Refer21Poc,
    swreg56_h264_refer22_poc: Swreg56H264Refer22Poc,
    swreg57_h264_refer23_poc: Swreg57H264Refer23Poc,
    swreg58_h264_refer24_poc: Swreg58H264Refer24Poc,
    swreg59_h264_refer25_poc: Swreg59H264Refer25Poc,
    swreg60_h264_refer26_poc: Swreg60H264Refer26Poc,
    swreg61_h264_refer27_poc: Swreg61H264Refer27Poc,
    swreg62_h264_refer28_poc: Swreg62H264Refer28Poc,
    swreg63_h264_refer29_poc: Swreg63H264Refer29Poc,
    swreg64_performance_cycle: Swreg64PerformanceCycle,
    swreg65_axi_ddr_rdata: Swreg65AxiDdrRdata,
    swreg66_axi_ddr_wdata: Swreg66AxiDdrWdata,
    _reserved67: [u8; 0x04],
    swreg68_performance_sel: Swreg68PerformanceSel,
    swreg69_performance_cnt0: Swreg69PerformanceCnt0,
    swreg70_performance_cnt1: Swreg70PerformanceCnt1,
    swreg71_performance_cnt2: Swreg71PerformanceCnt2,
    swreg72_h264_refer30_poc: Swreg72H264Refer30Poc,
    swreg73_h264_refer31_poc: Swreg73H264Refer31Poc,
    swreg74_h264_cur_poc1: Swreg74H264CurPoc1,
    swreg75_h264_errorinfo_base: Swreg75H264ErrorinfoBase,
    swreg76_h264_errorinfo_num: Swreg76H264ErrorinfoNum,
    swreg77_h264_error_e: Swreg77H264ErrorE,
}
impl RegisterBlock {
    #[doc = "0x00 - ID register (read only)"]
    #[inline(always)]
    pub const fn swreg0_id(&self) -> &Swreg0Id {
        &self.swreg0_id
    }
    #[doc = "0x04 - interrupt and decoder enable register"]
    #[inline(always)]
    pub const fn swreg1_int(&self) -> &Swreg1Int {
        &self.swreg1_int
    }
    #[doc = "0x08 - Data input and output endian setting and sys ctrl"]
    #[inline(always)]
    pub const fn swreg2_sysctrl(&self) -> &Swreg2Sysctrl {
        &self.swreg2_sysctrl
    }
    #[doc = "0x0c - picture parameters"]
    #[inline(always)]
    pub const fn swreg3_picpar(&self) -> &Swreg3Picpar {
        &self.swreg3_picpar
    }
    #[doc = "0x10 - the stream or rlc data base address"]
    #[inline(always)]
    pub const fn swreg4_strm_rlc_base(&self) -> &Swreg4StrmRlcBase {
        &self.swreg4_strm_rlc_base
    }
    #[doc = "0x14 - amount of stream bytes or rlc data byte in the input buffer or the"]
    #[inline(always)]
    pub const fn swreg5_stream_rlc_len(&self) -> &Swreg5StreamRlcLen {
        &self.swreg5_stream_rlc_len
    }
    #[doc = "0x18 - the base address of cabac table"]
    #[inline(always)]
    pub const fn swreg6_cabactbl_prob_base(&self) -> &Swreg6CabactblProbBase {
        &self.swreg6_cabactbl_prob_base
    }
    #[doc = "0x1c - base address of decoder output picture base address"]
    #[inline(always)]
    pub const fn swreg7_decout_base(&self) -> &Swreg7DecoutBase {
        &self.swreg7_decout_base
    }
    #[doc = "0x20 - the ouput picture y virtual stride"]
    #[inline(always)]
    pub const fn swreg8_y_virstride(&self) -> &Swreg8YVirstride {
        &self.swreg8_y_virstride
    }
    #[doc = "0x24 - the ouput picture yuv virtual stride"]
    #[inline(always)]
    pub const fn swreg9_yuv_virstride(&self) -> &Swreg9YuvVirstride {
        &self.swreg9_yuv_virstride
    }
    #[doc = "0x28 - vp9 compressed header offset"]
    #[inline(always)]
    pub const fn swreg10_vp9_cprheader_offset(&self) -> &Swreg10Vp9CprheaderOffset {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x28 - base address for reference picture index 0"]
    #[inline(always)]
    pub const fn swreg10_h264_refer0_base(&self) -> &Swreg10H264Refer0Base {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x28 - base address for reference picture index 0"]
    #[inline(always)]
    pub const fn swreg10_hevc_refer0_base(&self) -> &Swreg10HevcRefer0Base {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x2c - base address for last frame"]
    #[inline(always)]
    pub const fn swreg11_vp9_referlast_base(&self) -> &Swreg11Vp9ReferlastBase {
        unsafe { &*(self as *const Self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x2c - base address for reference picture index 1"]
    #[inline(always)]
    pub const fn swreg11_h264_refer1_base(&self) -> &Swreg11H264Refer1Base {
        unsafe { &*(self as *const Self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x2c - base address for reference picture index 1"]
    #[inline(always)]
    pub const fn swreg11_hevc_refer1_base(&self) -> &Swreg11HevcRefer1Base {
        unsafe { &*(self as *const Self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x30 - base address for golden frame"]
    #[inline(always)]
    pub const fn swreg12_vp9_refergolden_base(&self) -> &Swreg12Vp9RefergoldenBase {
        unsafe { &*(self as *const Self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x30 - base address for reference picture index 2"]
    #[inline(always)]
    pub const fn swreg12_h264_refer2_base(&self) -> &Swreg12H264Refer2Base {
        unsafe { &*(self as *const Self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x30 - base address for reference picture index 2"]
    #[inline(always)]
    pub const fn swreg12_hevc_refer2_base(&self) -> &Swreg12HevcRefer2Base {
        unsafe { &*(self as *const Self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x34 - base address for referalfter frame"]
    #[inline(always)]
    pub const fn swreg13_vp9_referalfter_base(&self) -> &Swreg13Vp9ReferalfterBase {
        unsafe { &*(self as *const Self).cast::<u8>().add(52).cast() }
    }
    #[doc = "0x34 - base address for reference picture index 3"]
    #[inline(always)]
    pub const fn swreg13_h264_refer3_base(&self) -> &Swreg13H264Refer3Base {
        unsafe { &*(self as *const Self).cast::<u8>().add(52).cast() }
    }
    #[doc = "0x34 - base address for reference picture index 3"]
    #[inline(always)]
    pub const fn swreg13_hevc_refer3_base(&self) -> &Swreg13HevcRefer3Base {
        unsafe { &*(self as *const Self).cast::<u8>().add(52).cast() }
    }
    #[doc = "0x38 - vp9 count base addr"]
    #[inline(always)]
    pub const fn swreg14_vp9count_base(&self) -> &Swreg14Vp9countBase {
        unsafe { &*(self as *const Self).cast::<u8>().add(56).cast() }
    }
    #[doc = "0x38 - base address for reference picture index 4"]
    #[inline(always)]
    pub const fn swreg14_h264_refer4_base(&self) -> &Swreg14H264Refer4Base {
        unsafe { &*(self as *const Self).cast::<u8>().add(56).cast() }
    }
    #[doc = "0x38 - base address for reference picture index 4"]
    #[inline(always)]
    pub const fn swreg14_hevc_refer4_base(&self) -> &Swreg14HevcRefer4Base {
        unsafe { &*(self as *const Self).cast::<u8>().add(56).cast() }
    }
    #[doc = "0x3c - base address for reference picture index 5"]
    #[inline(always)]
    pub const fn swreg15_hevc_refer5_base(&self) -> &Swreg15HevcRefer5Base {
        unsafe { &*(self as *const Self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x3c - base address for last frame segment id"]
    #[inline(always)]
    pub const fn swreg15_vp9_segidlast_base(&self) -> &Swreg15Vp9SegidlastBase {
        unsafe { &*(self as *const Self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x3c - base address for reference picture index 5"]
    #[inline(always)]
    pub const fn swreg15_h264_refer5_base(&self) -> &Swreg15H264Refer5Base {
        unsafe { &*(self as *const Self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x40 - base address for reference picture index 6"]
    #[inline(always)]
    pub const fn swreg16_hevc_refer6_base(&self) -> &Swreg16HevcRefer6Base {
        unsafe { &*(self as *const Self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x40 - base address for cur frame segment id"]
    #[inline(always)]
    pub const fn swreg16_vp9_segidcur_base(&self) -> &Swreg16Vp9SegidcurBase {
        unsafe { &*(self as *const Self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x40 - base address for reference picture index 6"]
    #[inline(always)]
    pub const fn swreg16_h264_refer6_base(&self) -> &Swreg16H264Refer6Base {
        unsafe { &*(self as *const Self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x44 - base address for reference picture index 7"]
    #[inline(always)]
    pub const fn swreg17_hevc_refer7_base(&self) -> &Swreg17HevcRefer7Base {
        unsafe { &*(self as *const Self).cast::<u8>().add(68).cast() }
    }
    #[doc = "0x44 - vp9 last frame size"]
    #[inline(always)]
    pub const fn swreg17_vp9_frame_size_last(&self) -> &Swreg17Vp9FrameSizeLast {
        unsafe { &*(self as *const Self).cast::<u8>().add(68).cast() }
    }
    #[doc = "0x44 - base address for reference picture index 7"]
    #[inline(always)]
    pub const fn swreg17_h264_refer7_base(&self) -> &Swreg17H264Refer7Base {
        unsafe { &*(self as *const Self).cast::<u8>().add(68).cast() }
    }
    #[doc = "0x48 - base address for reference picture index 8"]
    #[inline(always)]
    pub const fn swreg18_hevc_refer8_base(&self) -> &Swreg18HevcRefer8Base {
        unsafe { &*(self as *const Self).cast::<u8>().add(72).cast() }
    }
    #[doc = "0x48 - vp9 golden frame size"]
    #[inline(always)]
    pub const fn swreg18_vp9_frame_size_golden(&self) -> &Swreg18Vp9FrameSizeGolden {
        unsafe { &*(self as *const Self).cast::<u8>().add(72).cast() }
    }
    #[doc = "0x48 - base address for reference picture index 8"]
    #[inline(always)]
    pub const fn swreg18_h264_refer8_base(&self) -> &Swreg18H264Refer8Base {
        unsafe { &*(self as *const Self).cast::<u8>().add(72).cast() }
    }
    #[doc = "0x4c - base address for reference picture index 9"]
    #[inline(always)]
    pub const fn swreg19_hevc_refer9_base(&self) -> &Swreg19HevcRefer9Base {
        unsafe { &*(self as *const Self).cast::<u8>().add(76).cast() }
    }
    #[doc = "0x4c - vp9 alfter frame size"]
    #[inline(always)]
    pub const fn swreg19_vp9_frame_size_altref(&self) -> &Swreg19Vp9FrameSizeAltref {
        unsafe { &*(self as *const Self).cast::<u8>().add(76).cast() }
    }
    #[doc = "0x4c - base address for reference picture index 9"]
    #[inline(always)]
    pub const fn swreg19_h264_refer9_base(&self) -> &Swreg19H264Refer9Base {
        unsafe { &*(self as *const Self).cast::<u8>().add(76).cast() }
    }
    #[doc = "0x50 - base address for reference picture index 10"]
    #[inline(always)]
    pub const fn swreg20_hevc_refer10_base(&self) -> &Swreg20HevcRefer10Base {
        unsafe { &*(self as *const Self).cast::<u8>().add(80).cast() }
    }
    #[doc = "0x50 - vp9 segid syntax grp0"]
    #[inline(always)]
    pub const fn swreg20_vp9_segid_grp0(&self) -> &Swreg20Vp9SegidGrp0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(80).cast() }
    }
    #[doc = "0x50 - base address for reference picture index 10"]
    #[inline(always)]
    pub const fn swreg20_h264_refer10_base(&self) -> &Swreg20H264Refer10Base {
        unsafe { &*(self as *const Self).cast::<u8>().add(80).cast() }
    }
    #[doc = "0x54 - base address for reference picture index 11"]
    #[inline(always)]
    pub const fn swreg21_hevc_refer11_base(&self) -> &Swreg21HevcRefer11Base {
        unsafe { &*(self as *const Self).cast::<u8>().add(84).cast() }
    }
    #[doc = "0x54 - vp9 segid syntax grp1"]
    #[inline(always)]
    pub const fn swreg21_vp9_segid_grp1(&self) -> &Swreg21Vp9SegidGrp1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(84).cast() }
    }
    #[doc = "0x54 - base address for reference picture index 11"]
    #[inline(always)]
    pub const fn swreg21_h264_refer11_base(&self) -> &Swreg21H264Refer11Base {
        unsafe { &*(self as *const Self).cast::<u8>().add(84).cast() }
    }
    #[doc = "0x58 - base address for reference picture index 12"]
    #[inline(always)]
    pub const fn swreg22_hevc_refer12_base(&self) -> &Swreg22HevcRefer12Base {
        unsafe { &*(self as *const Self).cast::<u8>().add(88).cast() }
    }
    #[doc = "0x58 - vp9 segid syntax grp2"]
    #[inline(always)]
    pub const fn swreg22_vp9_segid_grp2(&self) -> &Swreg22Vp9SegidGrp2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(88).cast() }
    }
    #[doc = "0x58 - base address for reference picture index 12"]
    #[inline(always)]
    pub const fn swreg22_h264_refer12_base(&self) -> &Swreg22H264Refer12Base {
        unsafe { &*(self as *const Self).cast::<u8>().add(88).cast() }
    }
    #[doc = "0x5c - base address for reference picture index 13"]
    #[inline(always)]
    pub const fn swreg23_hevc_refer13_base(&self) -> &Swreg23HevcRefer13Base {
        unsafe { &*(self as *const Self).cast::<u8>().add(92).cast() }
    }
    #[doc = "0x5c - vp9 segid syntax grp3"]
    #[inline(always)]
    pub const fn swreg23_vp9_segid_grp3(&self) -> &Swreg23Vp9SegidGrp3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(92).cast() }
    }
    #[doc = "0x5c - base address for reference picture index 13"]
    #[inline(always)]
    pub const fn swreg23_h264_refer13_base(&self) -> &Swreg23H264Refer13Base {
        unsafe { &*(self as *const Self).cast::<u8>().add(92).cast() }
    }
    #[doc = "0x60 - base address for reference picture index 14"]
    #[inline(always)]
    pub const fn swreg24_hevc_refer14_base(&self) -> &Swreg24HevcRefer14Base {
        unsafe { &*(self as *const Self).cast::<u8>().add(96).cast() }
    }
    #[doc = "0x60 - vp9 segid syntax grp4"]
    #[inline(always)]
    pub const fn swreg24_vp9_segid_grp4(&self) -> &Swreg24Vp9SegidGrp4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(96).cast() }
    }
    #[doc = "0x60 - base address for reference picture index 14"]
    #[inline(always)]
    pub const fn swreg24_h264_refer14_base(&self) -> &Swreg24H264Refer14Base {
        unsafe { &*(self as *const Self).cast::<u8>().add(96).cast() }
    }
    #[doc = "0x64 - the poc of reference picture index 0"]
    #[inline(always)]
    pub const fn swreg25_refer0_poc(&self) -> &Swreg25Refer0Poc {
        unsafe { &*(self as *const Self).cast::<u8>().add(100).cast() }
    }
    #[doc = "0x64 - vp9 segid syntax grp5"]
    #[inline(always)]
    pub const fn swreg25_vp9_segid_grp5(&self) -> &Swreg25Vp9SegidGrp5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(100).cast() }
    }
    #[doc = "0x68 - the poc of reference picture index 1"]
    #[inline(always)]
    pub const fn swreg26_refer1_poc(&self) -> &Swreg26Refer1Poc {
        unsafe { &*(self as *const Self).cast::<u8>().add(104).cast() }
    }
    #[doc = "0x68 - vp9 segid syntax grp6"]
    #[inline(always)]
    pub const fn swreg26_vp9_segid_grp6(&self) -> &Swreg26Vp9SegidGrp6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(104).cast() }
    }
    #[doc = "0x6c - the poc of reference picture index 2"]
    #[inline(always)]
    pub const fn swreg27_refer2_poc(&self) -> &Swreg27Refer2Poc {
        unsafe { &*(self as *const Self).cast::<u8>().add(108).cast() }
    }
    #[doc = "0x6c - vp9 segid syntax grp7"]
    #[inline(always)]
    pub const fn swreg27_vp9_segid_grp7(&self) -> &Swreg27Vp9SegidGrp7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(108).cast() }
    }
    #[doc = "0x70 - the poc of reference picture index 3"]
    #[inline(always)]
    pub const fn swreg28_refer3_poc(&self) -> &Swreg28Refer3Poc {
        unsafe { &*(self as *const Self).cast::<u8>().add(112).cast() }
    }
    #[doc = "0x70 - vp9 compressed header config info"]
    #[inline(always)]
    pub const fn swreg28_vp9_cprheader_config(&self) -> &Swreg28Vp9CprheaderConfig {
        unsafe { &*(self as *const Self).cast::<u8>().add(112).cast() }
    }
    #[doc = "0x74 - the poc of reference picture index 4"]
    #[inline(always)]
    pub const fn swreg29_refer4_poc(&self) -> &Swreg29Refer4Poc {
        unsafe { &*(self as *const Self).cast::<u8>().add(116).cast() }
    }
    #[doc = "0x74 - scaling factor for last reference picture"]
    #[inline(always)]
    pub const fn swreg29_vp9_lref_scale(&self) -> &Swreg29Vp9LrefScale {
        unsafe { &*(self as *const Self).cast::<u8>().add(116).cast() }
    }
    #[doc = "0x78 - the poc of reference picture index 5"]
    #[inline(always)]
    pub const fn swreg30_refer5_poc(&self) -> &Swreg30Refer5Poc {
        unsafe { &*(self as *const Self).cast::<u8>().add(120).cast() }
    }
    #[doc = "0x78 - scaling factor for golden reference picture"]
    #[inline(always)]
    pub const fn swreg30_vp9_gref_scale(&self) -> &Swreg30Vp9GrefScale {
        unsafe { &*(self as *const Self).cast::<u8>().add(120).cast() }
    }
    #[doc = "0x7c - the poc of reference picture index 6"]
    #[inline(always)]
    pub const fn swreg31_refer6_poc(&self) -> &Swreg31Refer6Poc {
        unsafe { &*(self as *const Self).cast::<u8>().add(124).cast() }
    }
    #[doc = "0x7c - scaling factor for alfter reference picture"]
    #[inline(always)]
    pub const fn swreg31_vp9_aref_scale(&self) -> &Swreg31Vp9ArefScale {
        unsafe { &*(self as *const Self).cast::<u8>().add(124).cast() }
    }
    #[doc = "0x80 - the poc of reference picture index 7"]
    #[inline(always)]
    pub const fn swreg32_refer7_poc(&self) -> &Swreg32Refer7Poc {
        unsafe { &*(self as *const Self).cast::<u8>().add(128).cast() }
    }
    #[doc = "0x80 - vp9 ref deltas"]
    #[inline(always)]
    pub const fn swreg32_vp9_ref_deltas_lastframe(&self) -> &Swreg32Vp9RefDeltasLastframe {
        unsafe { &*(self as *const Self).cast::<u8>().add(128).cast() }
    }
    #[doc = "0x84 - the poc of reference picture index 8"]
    #[inline(always)]
    pub const fn swreg33_refer8_poc(&self) -> &Swreg33Refer8Poc {
        unsafe { &*(self as *const Self).cast::<u8>().add(132).cast() }
    }
    #[doc = "0x84 - vp9 info for lastframe"]
    #[inline(always)]
    pub const fn swreg33_vp9_info_lastframe(&self) -> &Swreg33Vp9InfoLastframe {
        unsafe { &*(self as *const Self).cast::<u8>().add(132).cast() }
    }
    #[doc = "0x88 - the poc of reference picture index 9"]
    #[inline(always)]
    pub const fn swreg34_refer9_poc(&self) -> &Swreg34Refer9Poc {
        unsafe { &*(self as *const Self).cast::<u8>().add(136).cast() }
    }
    #[doc = "0x88 - inter cmd base addr"]
    #[inline(always)]
    pub const fn swreg34_vp9_intercmd_base(&self) -> &Swreg34Vp9IntercmdBase {
        unsafe { &*(self as *const Self).cast::<u8>().add(136).cast() }
    }
    #[doc = "0x8c - the poc of reference picture index 10"]
    #[inline(always)]
    pub const fn swreg35_refer10_poc(&self) -> &Swreg35Refer10Poc {
        unsafe { &*(self as *const Self).cast::<u8>().add(140).cast() }
    }
    #[doc = "0x8c - vp9 intercmd num"]
    #[inline(always)]
    pub const fn swreg35_vp9_intercmd_num(&self) -> &Swreg35Vp9IntercmdNum {
        unsafe { &*(self as *const Self).cast::<u8>().add(140).cast() }
    }
    #[doc = "0x90 - the poc of reference picture index 11"]
    #[inline(always)]
    pub const fn swreg36_refer11_poc(&self) -> &Swreg36Refer11Poc {
        unsafe { &*(self as *const Self).cast::<u8>().add(144).cast() }
    }
    #[doc = "0x90 - vp9 lasttile size"]
    #[inline(always)]
    pub const fn swreg36_vp9_lasttile_size(&self) -> &Swreg36Vp9LasttileSize {
        unsafe { &*(self as *const Self).cast::<u8>().add(144).cast() }
    }
    #[doc = "0x94 - the poc of reference picture index 12"]
    #[inline(always)]
    pub const fn swreg37_refer12_poc(&self) -> &Swreg37Refer12Poc {
        unsafe { &*(self as *const Self).cast::<u8>().add(148).cast() }
    }
    #[doc = "0x94 - Register0000 Abstract"]
    #[inline(always)]
    pub const fn swreg37_vp9_lastf_hor_virstride(&self) -> &Swreg37Vp9LastfHorVirstride {
        unsafe { &*(self as *const Self).cast::<u8>().add(148).cast() }
    }
    #[doc = "0x98 - the poc of reference picture index 13"]
    #[inline(always)]
    pub const fn swreg38_refer13_poc(&self) -> &Swreg38Refer13Poc {
        unsafe { &*(self as *const Self).cast::<u8>().add(152).cast() }
    }
    #[doc = "0x98 - vp9 golden frame horizontal virstride"]
    #[inline(always)]
    pub const fn swreg38_vp9_goldenf_hor_virstride(&self) -> &Swreg38Vp9GoldenfHorVirstride {
        unsafe { &*(self as *const Self).cast::<u8>().add(152).cast() }
    }
    #[doc = "0x9c - the poc of reference picture index 14"]
    #[inline(always)]
    pub const fn swreg39_refer14_poc(&self) -> &Swreg39Refer14Poc {
        unsafe { &*(self as *const Self).cast::<u8>().add(156).cast() }
    }
    #[doc = "0x9c - vp9 altref frame horizontal virstride"]
    #[inline(always)]
    pub const fn swreg39_vp9_altreff_hor_virstride(&self) -> &Swreg39Vp9AltreffHorVirstride {
        unsafe { &*(self as *const Self).cast::<u8>().add(156).cast() }
    }
    #[doc = "0xa0 - the poc of cur picture"]
    #[inline(always)]
    pub const fn swreg40_cur_poc(&self) -> &Swreg40CurPoc {
        &self.swreg40_cur_poc
    }
    #[doc = "0xa4 - the base address or rlcwrite base addr"]
    #[inline(always)]
    pub const fn swreg41_rlcwrite_base(&self) -> &Swreg41RlcwriteBase {
        &self.swreg41_rlcwrite_base
    }
    #[doc = "0xa8 - the base address of pps"]
    #[inline(always)]
    pub const fn swreg42_pps_base(&self) -> &Swreg42PpsBase {
        &self.swreg42_pps_base
    }
    #[doc = "0xac - the base address of rps"]
    #[inline(always)]
    pub const fn swreg43_rps_base(&self) -> &Swreg43RpsBase {
        &self.swreg43_rps_base
    }
    #[doc = "0xb0 - cabac error enable config"]
    #[inline(always)]
    pub const fn swreg44_strmd_error_en(&self) -> &Swreg44StrmdErrorEn {
        &self.swreg44_strmd_error_en
    }
    #[doc = "0xb4 - vp9 error info"]
    #[inline(always)]
    pub const fn swreg45_vp9_error_info0(&self) -> &Swreg45Vp9ErrorInfo0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(180).cast() }
    }
    #[doc = "0xb4 - cabac error status"]
    #[inline(always)]
    pub const fn swreg45_strmd_error_status(&self) -> &Swreg45StrmdErrorStatus {
        unsafe { &*(self as *const Self).cast::<u8>().add(180).cast() }
    }
    #[doc = "0xb8 - strmd error ctu"]
    #[inline(always)]
    pub const fn swreg46_strmd_error_ctu(&self) -> &Swreg46StrmdErrorCtu {
        &self.swreg46_strmd_error_ctu
    }
    #[doc = "0xbc - sao ctu position"]
    #[inline(always)]
    pub const fn swreg47_sao_ctu_position(&self) -> &Swreg47SaoCtuPosition {
        &self.swreg47_sao_ctu_position
    }
    #[doc = "0xc0 - last ref ystride"]
    #[inline(always)]
    pub const fn swreg48_vp9_last_ystride(&self) -> &Swreg48Vp9LastYstride {
        unsafe { &*(self as *const Self).cast::<u8>().add(192).cast() }
    }
    #[doc = "0xc0 - base address for reference picture index 15"]
    #[inline(always)]
    pub const fn swreg48_h264_refer15_base(&self) -> &Swreg48H264Refer15Base {
        unsafe { &*(self as *const Self).cast::<u8>().add(192).cast() }
    }
    #[doc = "0xc4 - golden ref ystride"]
    #[inline(always)]
    pub const fn swreg49_vp9_golden_ystride(&self) -> &Swreg49Vp9GoldenYstride {
        unsafe { &*(self as *const Self).cast::<u8>().add(196).cast() }
    }
    #[doc = "0xc4 - the poc of reference picture index 15"]
    #[inline(always)]
    pub const fn swreg49_h264_refer15_poc(&self) -> &Swreg49H264Refer15Poc {
        unsafe { &*(self as *const Self).cast::<u8>().add(196).cast() }
    }
    #[doc = "0xc8 - altref ref ystride"]
    #[inline(always)]
    pub const fn swreg50_vp9_altrefy_ystride(&self) -> &Swreg50Vp9AltrefyYstride {
        unsafe { &*(self as *const Self).cast::<u8>().add(200).cast() }
    }
    #[doc = "0xc8 - the poc of reference picture index 16"]
    #[inline(always)]
    pub const fn swreg50_h264_refer16_poc(&self) -> &Swreg50H264Refer16Poc {
        unsafe { &*(self as *const Self).cast::<u8>().add(200).cast() }
    }
    #[doc = "0xcc - vp9 lastref yuv stride"]
    #[inline(always)]
    pub const fn swreg51_vp9_lastref_yuvstride(&self) -> &Swreg51Vp9LastrefYuvstride {
        unsafe { &*(self as *const Self).cast::<u8>().add(204).cast() }
    }
    #[doc = "0xcc - the poc of reference picture index 17"]
    #[inline(always)]
    pub const fn swreg51_h264_refer17_poc(&self) -> &Swreg51H264Refer17Poc {
        unsafe { &*(self as *const Self).cast::<u8>().add(204).cast() }
    }
    #[doc = "0xd0 - the poc of reference picture index 18"]
    #[inline(always)]
    pub const fn swreg52_h264_refer18_poc(&self) -> &Swreg52H264Refer18Poc {
        unsafe { &*(self as *const Self).cast::<u8>().add(208).cast() }
    }
    #[doc = "0xd0 - vp9 colmv ref base"]
    #[inline(always)]
    pub const fn swreg52_vp9_refcolmv_base(&self) -> &Swreg52Vp9RefcolmvBase {
        unsafe { &*(self as *const Self).cast::<u8>().add(208).cast() }
    }
    #[doc = "0xd4 - the poc of reference picture index 19"]
    #[inline(always)]
    pub const fn swreg53_h264_refer19_poc(&self) -> &Swreg53H264Refer19Poc {
        &self.swreg53_h264_refer19_poc
    }
    #[doc = "0xd8 - the poc of reference picture index 20"]
    #[inline(always)]
    pub const fn swreg54_h264_refer20_poc(&self) -> &Swreg54H264Refer20Poc {
        &self.swreg54_h264_refer20_poc
    }
    #[doc = "0xdc - the poc of reference picture index 21"]
    #[inline(always)]
    pub const fn swreg55_h264_refer21_poc(&self) -> &Swreg55H264Refer21Poc {
        &self.swreg55_h264_refer21_poc
    }
    #[doc = "0xe0 - the poc of reference picture index 22"]
    #[inline(always)]
    pub const fn swreg56_h264_refer22_poc(&self) -> &Swreg56H264Refer22Poc {
        &self.swreg56_h264_refer22_poc
    }
    #[doc = "0xe4 - the poc of reference picture index 23"]
    #[inline(always)]
    pub const fn swreg57_h264_refer23_poc(&self) -> &Swreg57H264Refer23Poc {
        &self.swreg57_h264_refer23_poc
    }
    #[doc = "0xe8 - the poc of reference picture index 24"]
    #[inline(always)]
    pub const fn swreg58_h264_refer24_poc(&self) -> &Swreg58H264Refer24Poc {
        &self.swreg58_h264_refer24_poc
    }
    #[doc = "0xec - the poc of reference picture index 25"]
    #[inline(always)]
    pub const fn swreg59_h264_refer25_poc(&self) -> &Swreg59H264Refer25Poc {
        &self.swreg59_h264_refer25_poc
    }
    #[doc = "0xf0 - the poc of reference picture index 26"]
    #[inline(always)]
    pub const fn swreg60_h264_refer26_poc(&self) -> &Swreg60H264Refer26Poc {
        &self.swreg60_h264_refer26_poc
    }
    #[doc = "0xf4 - the poc of reference picture index 27"]
    #[inline(always)]
    pub const fn swreg61_h264_refer27_poc(&self) -> &Swreg61H264Refer27Poc {
        &self.swreg61_h264_refer27_poc
    }
    #[doc = "0xf8 - the poc of reference picture index 28"]
    #[inline(always)]
    pub const fn swreg62_h264_refer28_poc(&self) -> &Swreg62H264Refer28Poc {
        &self.swreg62_h264_refer28_poc
    }
    #[doc = "0xfc - the poc of reference picture index 29"]
    #[inline(always)]
    pub const fn swreg63_h264_refer29_poc(&self) -> &Swreg63H264Refer29Poc {
        &self.swreg63_h264_refer29_poc
    }
    #[doc = "0x100 - hevc performance cycle"]
    #[inline(always)]
    pub const fn swreg64_performance_cycle(&self) -> &Swreg64PerformanceCycle {
        &self.swreg64_performance_cycle
    }
    #[doc = "0x104 - axi ddr read data num"]
    #[inline(always)]
    pub const fn swreg65_axi_ddr_rdata(&self) -> &Swreg65AxiDdrRdata {
        &self.swreg65_axi_ddr_rdata
    }
    #[doc = "0x108 - axi ddr write data number"]
    #[inline(always)]
    pub const fn swreg66_axi_ddr_wdata(&self) -> &Swreg66AxiDdrWdata {
        &self.swreg66_axi_ddr_wdata
    }
    #[doc = "0x110 - "]
    #[inline(always)]
    pub const fn swreg68_performance_sel(&self) -> &Swreg68PerformanceSel {
        &self.swreg68_performance_sel
    }
    #[doc = "0x114 - "]
    #[inline(always)]
    pub const fn swreg69_performance_cnt0(&self) -> &Swreg69PerformanceCnt0 {
        &self.swreg69_performance_cnt0
    }
    #[doc = "0x118 - "]
    #[inline(always)]
    pub const fn swreg70_performance_cnt1(&self) -> &Swreg70PerformanceCnt1 {
        &self.swreg70_performance_cnt1
    }
    #[doc = "0x11c - "]
    #[inline(always)]
    pub const fn swreg71_performance_cnt2(&self) -> &Swreg71PerformanceCnt2 {
        &self.swreg71_performance_cnt2
    }
    #[doc = "0x120 - the poc of reference picture index 30"]
    #[inline(always)]
    pub const fn swreg72_h264_refer30_poc(&self) -> &Swreg72H264Refer30Poc {
        &self.swreg72_h264_refer30_poc
    }
    #[doc = "0x124 - the poc of reference picture index 31"]
    #[inline(always)]
    pub const fn swreg73_h264_refer31_poc(&self) -> &Swreg73H264Refer31Poc {
        &self.swreg73_h264_refer31_poc
    }
    #[doc = "0x128 - h264 cur poc for bottom filed"]
    #[inline(always)]
    pub const fn swreg74_h264_cur_poc1(&self) -> &Swreg74H264CurPoc1 {
        &self.swreg74_h264_cur_poc1
    }
    #[doc = "0x12c - h264 error info base addr"]
    #[inline(always)]
    pub const fn swreg75_h264_errorinfo_base(&self) -> &Swreg75H264ErrorinfoBase {
        &self.swreg75_h264_errorinfo_base
    }
    #[doc = "0x130 - h264 error info num"]
    #[inline(always)]
    pub const fn swreg76_h264_errorinfo_num(&self) -> &Swreg76H264ErrorinfoNum {
        &self.swreg76_h264_errorinfo_num
    }
    #[doc = "0x134 - h264 error enable high bits"]
    #[inline(always)]
    pub const fn swreg77_h264_error_e(&self) -> &Swreg77H264ErrorE {
        &self.swreg77_h264_error_e
    }
}
#[doc = "SWREG0_ID (rw) register accessor: ID register (read only)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg0_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg0_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg0_id`]
module"]
#[doc(alias = "SWREG0_ID")]
pub type Swreg0Id = crate::Reg<swreg0_id::Swreg0IdSpec>;
#[doc = "ID register (read only)"]
pub mod swreg0_id;
#[doc = "SWREG1_INT (rw) register accessor: interrupt and decoder enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg1_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg1_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg1_int`]
module"]
#[doc(alias = "SWREG1_INT")]
pub type Swreg1Int = crate::Reg<swreg1_int::Swreg1IntSpec>;
#[doc = "interrupt and decoder enable register"]
pub mod swreg1_int;
#[doc = "SWREG2_SYSCTRL (rw) register accessor: Data input and output endian setting and sys ctrl\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg2_sysctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg2_sysctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg2_sysctrl`]
module"]
#[doc(alias = "SWREG2_SYSCTRL")]
pub type Swreg2Sysctrl = crate::Reg<swreg2_sysctrl::Swreg2SysctrlSpec>;
#[doc = "Data input and output endian setting and sys ctrl"]
pub mod swreg2_sysctrl;
#[doc = "SWREG3_PICPAR (rw) register accessor: picture parameters\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg3_picpar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg3_picpar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg3_picpar`]
module"]
#[doc(alias = "SWREG3_PICPAR")]
pub type Swreg3Picpar = crate::Reg<swreg3_picpar::Swreg3PicparSpec>;
#[doc = "picture parameters"]
pub mod swreg3_picpar;
#[doc = "SWREG4_STRM_RLC_BASE (rw) register accessor: the stream or rlc data base address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg4_strm_rlc_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg4_strm_rlc_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg4_strm_rlc_base`]
module"]
#[doc(alias = "SWREG4_STRM_RLC_BASE")]
pub type Swreg4StrmRlcBase = crate::Reg<swreg4_strm_rlc_base::Swreg4StrmRlcBaseSpec>;
#[doc = "the stream or rlc data base address"]
pub mod swreg4_strm_rlc_base;
#[doc = "SWREG5_STREAM_RLC_LEN (rw) register accessor: amount of stream bytes or rlc data byte in the input buffer or the\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg5_stream_rlc_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg5_stream_rlc_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg5_stream_rlc_len`]
module"]
#[doc(alias = "SWREG5_STREAM_RLC_LEN")]
pub type Swreg5StreamRlcLen = crate::Reg<swreg5_stream_rlc_len::Swreg5StreamRlcLenSpec>;
#[doc = "amount of stream bytes or rlc data byte in the input buffer or the"]
pub mod swreg5_stream_rlc_len;
#[doc = "SWREG6_CABACTBL_PROB_BASE (rw) register accessor: the base address of cabac table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg6_cabactbl_prob_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg6_cabactbl_prob_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg6_cabactbl_prob_base`]
module"]
#[doc(alias = "SWREG6_CABACTBL_PROB_BASE")]
pub type Swreg6CabactblProbBase = crate::Reg<swreg6_cabactbl_prob_base::Swreg6CabactblProbBaseSpec>;
#[doc = "the base address of cabac table"]
pub mod swreg6_cabactbl_prob_base;
#[doc = "SWREG7_DECOUT_BASE (rw) register accessor: base address of decoder output picture base address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg7_decout_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg7_decout_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg7_decout_base`]
module"]
#[doc(alias = "SWREG7_DECOUT_BASE")]
pub type Swreg7DecoutBase = crate::Reg<swreg7_decout_base::Swreg7DecoutBaseSpec>;
#[doc = "base address of decoder output picture base address"]
pub mod swreg7_decout_base;
#[doc = "SWREG8_Y_VIRSTRIDE (rw) register accessor: the ouput picture y virtual stride\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg8_y_virstride::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg8_y_virstride::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg8_y_virstride`]
module"]
#[doc(alias = "SWREG8_Y_VIRSTRIDE")]
pub type Swreg8YVirstride = crate::Reg<swreg8_y_virstride::Swreg8YVirstrideSpec>;
#[doc = "the ouput picture y virtual stride"]
pub mod swreg8_y_virstride;
#[doc = "SWREG9_YUV_VIRSTRIDE (rw) register accessor: the ouput picture yuv virtual stride\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg9_yuv_virstride::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg9_yuv_virstride::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg9_yuv_virstride`]
module"]
#[doc(alias = "SWREG9_YUV_VIRSTRIDE")]
pub type Swreg9YuvVirstride = crate::Reg<swreg9_yuv_virstride::Swreg9YuvVirstrideSpec>;
#[doc = "the ouput picture yuv virtual stride"]
pub mod swreg9_yuv_virstride;
#[doc = "SWREG10_HEVC_REFER0_BASE (rw) register accessor: base address for reference picture index 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg10_hevc_refer0_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg10_hevc_refer0_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg10_hevc_refer0_base`]
module"]
#[doc(alias = "SWREG10_HEVC_REFER0_BASE")]
pub type Swreg10HevcRefer0Base = crate::Reg<swreg10_hevc_refer0_base::Swreg10HevcRefer0BaseSpec>;
#[doc = "base address for reference picture index 0"]
pub mod swreg10_hevc_refer0_base;
#[doc = "SWREG10_H264_REFER0_BASE (rw) register accessor: base address for reference picture index 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg10_h264_refer0_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg10_h264_refer0_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg10_h264_refer0_base`]
module"]
#[doc(alias = "SWREG10_H264_REFER0_BASE")]
pub type Swreg10H264Refer0Base = crate::Reg<swreg10_h264_refer0_base::Swreg10H264Refer0BaseSpec>;
#[doc = "base address for reference picture index 0"]
pub mod swreg10_h264_refer0_base;
#[doc = "SWREG10_VP9_CPRHEADER_OFFSET (rw) register accessor: vp9 compressed header offset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg10_vp9_cprheader_offset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg10_vp9_cprheader_offset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg10_vp9_cprheader_offset`]
module"]
#[doc(alias = "SWREG10_VP9_CPRHEADER_OFFSET")]
pub type Swreg10Vp9CprheaderOffset =
    crate::Reg<swreg10_vp9_cprheader_offset::Swreg10Vp9CprheaderOffsetSpec>;
#[doc = "vp9 compressed header offset"]
pub mod swreg10_vp9_cprheader_offset;
#[doc = "SWREG11_HEVC_REFER1_BASE (rw) register accessor: base address for reference picture index 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg11_hevc_refer1_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg11_hevc_refer1_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg11_hevc_refer1_base`]
module"]
#[doc(alias = "SWREG11_HEVC_REFER1_BASE")]
pub type Swreg11HevcRefer1Base = crate::Reg<swreg11_hevc_refer1_base::Swreg11HevcRefer1BaseSpec>;
#[doc = "base address for reference picture index 1"]
pub mod swreg11_hevc_refer1_base;
#[doc = "SWREG11_H264_REFER1_BASE (rw) register accessor: base address for reference picture index 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg11_h264_refer1_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg11_h264_refer1_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg11_h264_refer1_base`]
module"]
#[doc(alias = "SWREG11_H264_REFER1_BASE")]
pub type Swreg11H264Refer1Base = crate::Reg<swreg11_h264_refer1_base::Swreg11H264Refer1BaseSpec>;
#[doc = "base address for reference picture index 1"]
pub mod swreg11_h264_refer1_base;
#[doc = "SWREG11_VP9_REFERLAST_BASE (rw) register accessor: base address for last frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg11_vp9_referlast_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg11_vp9_referlast_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg11_vp9_referlast_base`]
module"]
#[doc(alias = "SWREG11_VP9_REFERLAST_BASE")]
pub type Swreg11Vp9ReferlastBase =
    crate::Reg<swreg11_vp9_referlast_base::Swreg11Vp9ReferlastBaseSpec>;
#[doc = "base address for last frame"]
pub mod swreg11_vp9_referlast_base;
#[doc = "SWREG12_HEVC_REFER2_BASE (rw) register accessor: base address for reference picture index 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg12_hevc_refer2_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg12_hevc_refer2_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg12_hevc_refer2_base`]
module"]
#[doc(alias = "SWREG12_HEVC_REFER2_BASE")]
pub type Swreg12HevcRefer2Base = crate::Reg<swreg12_hevc_refer2_base::Swreg12HevcRefer2BaseSpec>;
#[doc = "base address for reference picture index 2"]
pub mod swreg12_hevc_refer2_base;
#[doc = "SWREG12_H264_REFER2_BASE (rw) register accessor: base address for reference picture index 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg12_h264_refer2_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg12_h264_refer2_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg12_h264_refer2_base`]
module"]
#[doc(alias = "SWREG12_H264_REFER2_BASE")]
pub type Swreg12H264Refer2Base = crate::Reg<swreg12_h264_refer2_base::Swreg12H264Refer2BaseSpec>;
#[doc = "base address for reference picture index 2"]
pub mod swreg12_h264_refer2_base;
#[doc = "SWREG12_VP9_REFERGOLDEN_BASE (rw) register accessor: base address for golden frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg12_vp9_refergolden_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg12_vp9_refergolden_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg12_vp9_refergolden_base`]
module"]
#[doc(alias = "SWREG12_VP9_REFERGOLDEN_BASE")]
pub type Swreg12Vp9RefergoldenBase =
    crate::Reg<swreg12_vp9_refergolden_base::Swreg12Vp9RefergoldenBaseSpec>;
#[doc = "base address for golden frame"]
pub mod swreg12_vp9_refergolden_base;
#[doc = "SWREG13_HEVC_REFER3_BASE (rw) register accessor: base address for reference picture index 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg13_hevc_refer3_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg13_hevc_refer3_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg13_hevc_refer3_base`]
module"]
#[doc(alias = "SWREG13_HEVC_REFER3_BASE")]
pub type Swreg13HevcRefer3Base = crate::Reg<swreg13_hevc_refer3_base::Swreg13HevcRefer3BaseSpec>;
#[doc = "base address for reference picture index 3"]
pub mod swreg13_hevc_refer3_base;
#[doc = "SWREG13_H264_REFER3_BASE (rw) register accessor: base address for reference picture index 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg13_h264_refer3_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg13_h264_refer3_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg13_h264_refer3_base`]
module"]
#[doc(alias = "SWREG13_H264_REFER3_BASE")]
pub type Swreg13H264Refer3Base = crate::Reg<swreg13_h264_refer3_base::Swreg13H264Refer3BaseSpec>;
#[doc = "base address for reference picture index 3"]
pub mod swreg13_h264_refer3_base;
#[doc = "SWREG13_VP9_REFERALFTER_BASE (rw) register accessor: base address for referalfter frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg13_vp9_referalfter_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg13_vp9_referalfter_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg13_vp9_referalfter_base`]
module"]
#[doc(alias = "SWREG13_VP9_REFERALFTER_BASE")]
pub type Swreg13Vp9ReferalfterBase =
    crate::Reg<swreg13_vp9_referalfter_base::Swreg13Vp9ReferalfterBaseSpec>;
#[doc = "base address for referalfter frame"]
pub mod swreg13_vp9_referalfter_base;
#[doc = "SWREG14_HEVC_REFER4_BASE (rw) register accessor: base address for reference picture index 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg14_hevc_refer4_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg14_hevc_refer4_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg14_hevc_refer4_base`]
module"]
#[doc(alias = "SWREG14_HEVC_REFER4_BASE")]
pub type Swreg14HevcRefer4Base = crate::Reg<swreg14_hevc_refer4_base::Swreg14HevcRefer4BaseSpec>;
#[doc = "base address for reference picture index 4"]
pub mod swreg14_hevc_refer4_base;
#[doc = "SWREG14_H264_REFER4_BASE (rw) register accessor: base address for reference picture index 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg14_h264_refer4_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg14_h264_refer4_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg14_h264_refer4_base`]
module"]
#[doc(alias = "SWREG14_H264_REFER4_BASE")]
pub type Swreg14H264Refer4Base = crate::Reg<swreg14_h264_refer4_base::Swreg14H264Refer4BaseSpec>;
#[doc = "base address for reference picture index 4"]
pub mod swreg14_h264_refer4_base;
#[doc = "SWREG14_VP9COUNT_BASE (rw) register accessor: vp9 count base addr\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg14_vp9count_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg14_vp9count_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg14_vp9count_base`]
module"]
#[doc(alias = "SWREG14_VP9COUNT_BASE")]
pub type Swreg14Vp9countBase = crate::Reg<swreg14_vp9count_base::Swreg14Vp9countBaseSpec>;
#[doc = "vp9 count base addr"]
pub mod swreg14_vp9count_base;
#[doc = "SWREG15_H264_REFER5_BASE (rw) register accessor: base address for reference picture index 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg15_h264_refer5_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg15_h264_refer5_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg15_h264_refer5_base`]
module"]
#[doc(alias = "SWREG15_H264_REFER5_BASE")]
pub type Swreg15H264Refer5Base = crate::Reg<swreg15_h264_refer5_base::Swreg15H264Refer5BaseSpec>;
#[doc = "base address for reference picture index 5"]
pub mod swreg15_h264_refer5_base;
#[doc = "SWREG15_VP9_SEGIDLAST_BASE (rw) register accessor: base address for last frame segment id\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg15_vp9_segidlast_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg15_vp9_segidlast_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg15_vp9_segidlast_base`]
module"]
#[doc(alias = "SWREG15_VP9_SEGIDLAST_BASE")]
pub type Swreg15Vp9SegidlastBase =
    crate::Reg<swreg15_vp9_segidlast_base::Swreg15Vp9SegidlastBaseSpec>;
#[doc = "base address for last frame segment id"]
pub mod swreg15_vp9_segidlast_base;
#[doc = "SWREG15_HEVC_REFER5_BASE (rw) register accessor: base address for reference picture index 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg15_hevc_refer5_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg15_hevc_refer5_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg15_hevc_refer5_base`]
module"]
#[doc(alias = "SWREG15_HEVC_REFER5_BASE")]
pub type Swreg15HevcRefer5Base = crate::Reg<swreg15_hevc_refer5_base::Swreg15HevcRefer5BaseSpec>;
#[doc = "base address for reference picture index 5"]
pub mod swreg15_hevc_refer5_base;
#[doc = "SWREG16_H264_REFER6_BASE (rw) register accessor: base address for reference picture index 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg16_h264_refer6_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg16_h264_refer6_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg16_h264_refer6_base`]
module"]
#[doc(alias = "SWREG16_H264_REFER6_BASE")]
pub type Swreg16H264Refer6Base = crate::Reg<swreg16_h264_refer6_base::Swreg16H264Refer6BaseSpec>;
#[doc = "base address for reference picture index 6"]
pub mod swreg16_h264_refer6_base;
#[doc = "SWREG16_VP9_SEGIDCUR_BASE (rw) register accessor: base address for cur frame segment id\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg16_vp9_segidcur_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg16_vp9_segidcur_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg16_vp9_segidcur_base`]
module"]
#[doc(alias = "SWREG16_VP9_SEGIDCUR_BASE")]
pub type Swreg16Vp9SegidcurBase = crate::Reg<swreg16_vp9_segidcur_base::Swreg16Vp9SegidcurBaseSpec>;
#[doc = "base address for cur frame segment id"]
pub mod swreg16_vp9_segidcur_base;
#[doc = "SWREG16_HEVC_REFER6_BASE (rw) register accessor: base address for reference picture index 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg16_hevc_refer6_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg16_hevc_refer6_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg16_hevc_refer6_base`]
module"]
#[doc(alias = "SWREG16_HEVC_REFER6_BASE")]
pub type Swreg16HevcRefer6Base = crate::Reg<swreg16_hevc_refer6_base::Swreg16HevcRefer6BaseSpec>;
#[doc = "base address for reference picture index 6"]
pub mod swreg16_hevc_refer6_base;
#[doc = "SWREG17_H264_REFER7_BASE (rw) register accessor: base address for reference picture index 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg17_h264_refer7_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg17_h264_refer7_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg17_h264_refer7_base`]
module"]
#[doc(alias = "SWREG17_H264_REFER7_BASE")]
pub type Swreg17H264Refer7Base = crate::Reg<swreg17_h264_refer7_base::Swreg17H264Refer7BaseSpec>;
#[doc = "base address for reference picture index 7"]
pub mod swreg17_h264_refer7_base;
#[doc = "SWREG17_VP9_FRAME_SIZE_LAST (rw) register accessor: vp9 last frame size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg17_vp9_frame_size_last::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg17_vp9_frame_size_last::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg17_vp9_frame_size_last`]
module"]
#[doc(alias = "SWREG17_VP9_FRAME_SIZE_LAST")]
pub type Swreg17Vp9FrameSizeLast =
    crate::Reg<swreg17_vp9_frame_size_last::Swreg17Vp9FrameSizeLastSpec>;
#[doc = "vp9 last frame size"]
pub mod swreg17_vp9_frame_size_last;
#[doc = "SWREG17_HEVC_REFER7_BASE (rw) register accessor: base address for reference picture index 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg17_hevc_refer7_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg17_hevc_refer7_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg17_hevc_refer7_base`]
module"]
#[doc(alias = "SWREG17_HEVC_REFER7_BASE")]
pub type Swreg17HevcRefer7Base = crate::Reg<swreg17_hevc_refer7_base::Swreg17HevcRefer7BaseSpec>;
#[doc = "base address for reference picture index 7"]
pub mod swreg17_hevc_refer7_base;
#[doc = "SWREG18_H264_REFER8_BASE (rw) register accessor: base address for reference picture index 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg18_h264_refer8_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg18_h264_refer8_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg18_h264_refer8_base`]
module"]
#[doc(alias = "SWREG18_H264_REFER8_BASE")]
pub type Swreg18H264Refer8Base = crate::Reg<swreg18_h264_refer8_base::Swreg18H264Refer8BaseSpec>;
#[doc = "base address for reference picture index 8"]
pub mod swreg18_h264_refer8_base;
#[doc = "SWREG18_VP9_FRAME_SIZE_GOLDEN (rw) register accessor: vp9 golden frame size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg18_vp9_frame_size_golden::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg18_vp9_frame_size_golden::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg18_vp9_frame_size_golden`]
module"]
#[doc(alias = "SWREG18_VP9_FRAME_SIZE_GOLDEN")]
pub type Swreg18Vp9FrameSizeGolden =
    crate::Reg<swreg18_vp9_frame_size_golden::Swreg18Vp9FrameSizeGoldenSpec>;
#[doc = "vp9 golden frame size"]
pub mod swreg18_vp9_frame_size_golden;
#[doc = "SWREG18_HEVC_REFER8_BASE (rw) register accessor: base address for reference picture index 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg18_hevc_refer8_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg18_hevc_refer8_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg18_hevc_refer8_base`]
module"]
#[doc(alias = "SWREG18_HEVC_REFER8_BASE")]
pub type Swreg18HevcRefer8Base = crate::Reg<swreg18_hevc_refer8_base::Swreg18HevcRefer8BaseSpec>;
#[doc = "base address for reference picture index 8"]
pub mod swreg18_hevc_refer8_base;
#[doc = "SWREG19_H264_REFER9_BASE (rw) register accessor: base address for reference picture index 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg19_h264_refer9_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg19_h264_refer9_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg19_h264_refer9_base`]
module"]
#[doc(alias = "SWREG19_H264_REFER9_BASE")]
pub type Swreg19H264Refer9Base = crate::Reg<swreg19_h264_refer9_base::Swreg19H264Refer9BaseSpec>;
#[doc = "base address for reference picture index 9"]
pub mod swreg19_h264_refer9_base;
#[doc = "SWREG19_VP9_FRAME_SIZE_ALTREF (rw) register accessor: vp9 alfter frame size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg19_vp9_frame_size_altref::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg19_vp9_frame_size_altref::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg19_vp9_frame_size_altref`]
module"]
#[doc(alias = "SWREG19_VP9_FRAME_SIZE_ALTREF")]
pub type Swreg19Vp9FrameSizeAltref =
    crate::Reg<swreg19_vp9_frame_size_altref::Swreg19Vp9FrameSizeAltrefSpec>;
#[doc = "vp9 alfter frame size"]
pub mod swreg19_vp9_frame_size_altref;
#[doc = "SWREG19_HEVC_REFER9_BASE (rw) register accessor: base address for reference picture index 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg19_hevc_refer9_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg19_hevc_refer9_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg19_hevc_refer9_base`]
module"]
#[doc(alias = "SWREG19_HEVC_REFER9_BASE")]
pub type Swreg19HevcRefer9Base = crate::Reg<swreg19_hevc_refer9_base::Swreg19HevcRefer9BaseSpec>;
#[doc = "base address for reference picture index 9"]
pub mod swreg19_hevc_refer9_base;
#[doc = "SWREG20_H264_REFER10_BASE (rw) register accessor: base address for reference picture index 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg20_h264_refer10_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg20_h264_refer10_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg20_h264_refer10_base`]
module"]
#[doc(alias = "SWREG20_H264_REFER10_BASE")]
pub type Swreg20H264Refer10Base = crate::Reg<swreg20_h264_refer10_base::Swreg20H264Refer10BaseSpec>;
#[doc = "base address for reference picture index 10"]
pub mod swreg20_h264_refer10_base;
#[doc = "SWREG20_VP9_SEGID_GRP0 (rw) register accessor: vp9 segid syntax grp0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg20_vp9_segid_grp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg20_vp9_segid_grp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg20_vp9_segid_grp0`]
module"]
#[doc(alias = "SWREG20_VP9_SEGID_GRP0")]
pub type Swreg20Vp9SegidGrp0 = crate::Reg<swreg20_vp9_segid_grp0::Swreg20Vp9SegidGrp0Spec>;
#[doc = "vp9 segid syntax grp0"]
pub mod swreg20_vp9_segid_grp0;
#[doc = "SWREG20_HEVC_REFER10_BASE (rw) register accessor: base address for reference picture index 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg20_hevc_refer10_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg20_hevc_refer10_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg20_hevc_refer10_base`]
module"]
#[doc(alias = "SWREG20_HEVC_REFER10_BASE")]
pub type Swreg20HevcRefer10Base = crate::Reg<swreg20_hevc_refer10_base::Swreg20HevcRefer10BaseSpec>;
#[doc = "base address for reference picture index 10"]
pub mod swreg20_hevc_refer10_base;
#[doc = "SWREG21_H264_REFER11_BASE (rw) register accessor: base address for reference picture index 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg21_h264_refer11_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg21_h264_refer11_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg21_h264_refer11_base`]
module"]
#[doc(alias = "SWREG21_H264_REFER11_BASE")]
pub type Swreg21H264Refer11Base = crate::Reg<swreg21_h264_refer11_base::Swreg21H264Refer11BaseSpec>;
#[doc = "base address for reference picture index 11"]
pub mod swreg21_h264_refer11_base;
#[doc = "SWREG21_VP9_SEGID_GRP1 (rw) register accessor: vp9 segid syntax grp1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg21_vp9_segid_grp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg21_vp9_segid_grp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg21_vp9_segid_grp1`]
module"]
#[doc(alias = "SWREG21_VP9_SEGID_GRP1")]
pub type Swreg21Vp9SegidGrp1 = crate::Reg<swreg21_vp9_segid_grp1::Swreg21Vp9SegidGrp1Spec>;
#[doc = "vp9 segid syntax grp1"]
pub mod swreg21_vp9_segid_grp1;
#[doc = "SWREG21_HEVC_REFER11_BASE (rw) register accessor: base address for reference picture index 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg21_hevc_refer11_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg21_hevc_refer11_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg21_hevc_refer11_base`]
module"]
#[doc(alias = "SWREG21_HEVC_REFER11_BASE")]
pub type Swreg21HevcRefer11Base = crate::Reg<swreg21_hevc_refer11_base::Swreg21HevcRefer11BaseSpec>;
#[doc = "base address for reference picture index 11"]
pub mod swreg21_hevc_refer11_base;
#[doc = "SWREG22_H264_REFER12_BASE (rw) register accessor: base address for reference picture index 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg22_h264_refer12_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg22_h264_refer12_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg22_h264_refer12_base`]
module"]
#[doc(alias = "SWREG22_H264_REFER12_BASE")]
pub type Swreg22H264Refer12Base = crate::Reg<swreg22_h264_refer12_base::Swreg22H264Refer12BaseSpec>;
#[doc = "base address for reference picture index 12"]
pub mod swreg22_h264_refer12_base;
#[doc = "SWREG22_VP9_SEGID_GRP2 (rw) register accessor: vp9 segid syntax grp2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg22_vp9_segid_grp2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg22_vp9_segid_grp2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg22_vp9_segid_grp2`]
module"]
#[doc(alias = "SWREG22_VP9_SEGID_GRP2")]
pub type Swreg22Vp9SegidGrp2 = crate::Reg<swreg22_vp9_segid_grp2::Swreg22Vp9SegidGrp2Spec>;
#[doc = "vp9 segid syntax grp2"]
pub mod swreg22_vp9_segid_grp2;
#[doc = "SWREG22_HEVC_REFER12_BASE (rw) register accessor: base address for reference picture index 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg22_hevc_refer12_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg22_hevc_refer12_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg22_hevc_refer12_base`]
module"]
#[doc(alias = "SWREG22_HEVC_REFER12_BASE")]
pub type Swreg22HevcRefer12Base = crate::Reg<swreg22_hevc_refer12_base::Swreg22HevcRefer12BaseSpec>;
#[doc = "base address for reference picture index 12"]
pub mod swreg22_hevc_refer12_base;
#[doc = "SWREG23_H264_REFER13_BASE (rw) register accessor: base address for reference picture index 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg23_h264_refer13_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg23_h264_refer13_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg23_h264_refer13_base`]
module"]
#[doc(alias = "SWREG23_H264_REFER13_BASE")]
pub type Swreg23H264Refer13Base = crate::Reg<swreg23_h264_refer13_base::Swreg23H264Refer13BaseSpec>;
#[doc = "base address for reference picture index 13"]
pub mod swreg23_h264_refer13_base;
#[doc = "SWREG23_VP9_SEGID_GRP3 (rw) register accessor: vp9 segid syntax grp3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg23_vp9_segid_grp3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg23_vp9_segid_grp3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg23_vp9_segid_grp3`]
module"]
#[doc(alias = "SWREG23_VP9_SEGID_GRP3")]
pub type Swreg23Vp9SegidGrp3 = crate::Reg<swreg23_vp9_segid_grp3::Swreg23Vp9SegidGrp3Spec>;
#[doc = "vp9 segid syntax grp3"]
pub mod swreg23_vp9_segid_grp3;
#[doc = "SWREG23_HEVC_REFER13_BASE (rw) register accessor: base address for reference picture index 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg23_hevc_refer13_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg23_hevc_refer13_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg23_hevc_refer13_base`]
module"]
#[doc(alias = "SWREG23_HEVC_REFER13_BASE")]
pub type Swreg23HevcRefer13Base = crate::Reg<swreg23_hevc_refer13_base::Swreg23HevcRefer13BaseSpec>;
#[doc = "base address for reference picture index 13"]
pub mod swreg23_hevc_refer13_base;
#[doc = "SWREG24_H264_REFER14_BASE (rw) register accessor: base address for reference picture index 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg24_h264_refer14_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg24_h264_refer14_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg24_h264_refer14_base`]
module"]
#[doc(alias = "SWREG24_H264_REFER14_BASE")]
pub type Swreg24H264Refer14Base = crate::Reg<swreg24_h264_refer14_base::Swreg24H264Refer14BaseSpec>;
#[doc = "base address for reference picture index 14"]
pub mod swreg24_h264_refer14_base;
#[doc = "SWREG24_VP9_SEGID_GRP4 (rw) register accessor: vp9 segid syntax grp4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg24_vp9_segid_grp4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg24_vp9_segid_grp4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg24_vp9_segid_grp4`]
module"]
#[doc(alias = "SWREG24_VP9_SEGID_GRP4")]
pub type Swreg24Vp9SegidGrp4 = crate::Reg<swreg24_vp9_segid_grp4::Swreg24Vp9SegidGrp4Spec>;
#[doc = "vp9 segid syntax grp4"]
pub mod swreg24_vp9_segid_grp4;
#[doc = "SWREG24_HEVC_REFER14_BASE (rw) register accessor: base address for reference picture index 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg24_hevc_refer14_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg24_hevc_refer14_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg24_hevc_refer14_base`]
module"]
#[doc(alias = "SWREG24_HEVC_REFER14_BASE")]
pub type Swreg24HevcRefer14Base = crate::Reg<swreg24_hevc_refer14_base::Swreg24HevcRefer14BaseSpec>;
#[doc = "base address for reference picture index 14"]
pub mod swreg24_hevc_refer14_base;
#[doc = "SWREG25_VP9_SEGID_GRP5 (rw) register accessor: vp9 segid syntax grp5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg25_vp9_segid_grp5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg25_vp9_segid_grp5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg25_vp9_segid_grp5`]
module"]
#[doc(alias = "SWREG25_VP9_SEGID_GRP5")]
pub type Swreg25Vp9SegidGrp5 = crate::Reg<swreg25_vp9_segid_grp5::Swreg25Vp9SegidGrp5Spec>;
#[doc = "vp9 segid syntax grp5"]
pub mod swreg25_vp9_segid_grp5;
#[doc = "SWREG25_REFER0_POC (rw) register accessor: the poc of reference picture index 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg25_refer0_poc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg25_refer0_poc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg25_refer0_poc`]
module"]
#[doc(alias = "SWREG25_REFER0_POC")]
pub type Swreg25Refer0Poc = crate::Reg<swreg25_refer0_poc::Swreg25Refer0PocSpec>;
#[doc = "the poc of reference picture index 0"]
pub mod swreg25_refer0_poc;
#[doc = "SWREG26_VP9_SEGID_GRP6 (rw) register accessor: vp9 segid syntax grp6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg26_vp9_segid_grp6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg26_vp9_segid_grp6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg26_vp9_segid_grp6`]
module"]
#[doc(alias = "SWREG26_VP9_SEGID_GRP6")]
pub type Swreg26Vp9SegidGrp6 = crate::Reg<swreg26_vp9_segid_grp6::Swreg26Vp9SegidGrp6Spec>;
#[doc = "vp9 segid syntax grp6"]
pub mod swreg26_vp9_segid_grp6;
#[doc = "SWREG26_REFER1_POC (rw) register accessor: the poc of reference picture index 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg26_refer1_poc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg26_refer1_poc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg26_refer1_poc`]
module"]
#[doc(alias = "SWREG26_REFER1_POC")]
pub type Swreg26Refer1Poc = crate::Reg<swreg26_refer1_poc::Swreg26Refer1PocSpec>;
#[doc = "the poc of reference picture index 1"]
pub mod swreg26_refer1_poc;
#[doc = "SWREG27_VP9_SEGID_GRP7 (rw) register accessor: vp9 segid syntax grp7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg27_vp9_segid_grp7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg27_vp9_segid_grp7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg27_vp9_segid_grp7`]
module"]
#[doc(alias = "SWREG27_VP9_SEGID_GRP7")]
pub type Swreg27Vp9SegidGrp7 = crate::Reg<swreg27_vp9_segid_grp7::Swreg27Vp9SegidGrp7Spec>;
#[doc = "vp9 segid syntax grp7"]
pub mod swreg27_vp9_segid_grp7;
#[doc = "SWREG27_REFER2_POC (rw) register accessor: the poc of reference picture index 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg27_refer2_poc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg27_refer2_poc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg27_refer2_poc`]
module"]
#[doc(alias = "SWREG27_REFER2_POC")]
pub type Swreg27Refer2Poc = crate::Reg<swreg27_refer2_poc::Swreg27Refer2PocSpec>;
#[doc = "the poc of reference picture index 2"]
pub mod swreg27_refer2_poc;
#[doc = "SWREG28_VP9_CPRHEADER_CONFIG (rw) register accessor: vp9 compressed header config info\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg28_vp9_cprheader_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg28_vp9_cprheader_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg28_vp9_cprheader_config`]
module"]
#[doc(alias = "SWREG28_VP9_CPRHEADER_CONFIG")]
pub type Swreg28Vp9CprheaderConfig =
    crate::Reg<swreg28_vp9_cprheader_config::Swreg28Vp9CprheaderConfigSpec>;
#[doc = "vp9 compressed header config info"]
pub mod swreg28_vp9_cprheader_config;
#[doc = "SWREG28_REFER3_POC (rw) register accessor: the poc of reference picture index 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg28_refer3_poc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg28_refer3_poc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg28_refer3_poc`]
module"]
#[doc(alias = "SWREG28_REFER3_POC")]
pub type Swreg28Refer3Poc = crate::Reg<swreg28_refer3_poc::Swreg28Refer3PocSpec>;
#[doc = "the poc of reference picture index 3"]
pub mod swreg28_refer3_poc;
#[doc = "SWREG29_VP9_LREF_SCALE (rw) register accessor: scaling factor for last reference picture\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg29_vp9_lref_scale::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg29_vp9_lref_scale::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg29_vp9_lref_scale`]
module"]
#[doc(alias = "SWREG29_VP9_LREF_SCALE")]
pub type Swreg29Vp9LrefScale = crate::Reg<swreg29_vp9_lref_scale::Swreg29Vp9LrefScaleSpec>;
#[doc = "scaling factor for last reference picture"]
pub mod swreg29_vp9_lref_scale;
#[doc = "SWREG29_REFER4_POC (rw) register accessor: the poc of reference picture index 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg29_refer4_poc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg29_refer4_poc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg29_refer4_poc`]
module"]
#[doc(alias = "SWREG29_REFER4_POC")]
pub type Swreg29Refer4Poc = crate::Reg<swreg29_refer4_poc::Swreg29Refer4PocSpec>;
#[doc = "the poc of reference picture index 4"]
pub mod swreg29_refer4_poc;
#[doc = "SWREG30_VP9_GREF_SCALE (rw) register accessor: scaling factor for golden reference picture\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg30_vp9_gref_scale::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg30_vp9_gref_scale::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg30_vp9_gref_scale`]
module"]
#[doc(alias = "SWREG30_VP9_GREF_SCALE")]
pub type Swreg30Vp9GrefScale = crate::Reg<swreg30_vp9_gref_scale::Swreg30Vp9GrefScaleSpec>;
#[doc = "scaling factor for golden reference picture"]
pub mod swreg30_vp9_gref_scale;
#[doc = "SWREG30_REFER5_POC (rw) register accessor: the poc of reference picture index 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg30_refer5_poc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg30_refer5_poc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg30_refer5_poc`]
module"]
#[doc(alias = "SWREG30_REFER5_POC")]
pub type Swreg30Refer5Poc = crate::Reg<swreg30_refer5_poc::Swreg30Refer5PocSpec>;
#[doc = "the poc of reference picture index 5"]
pub mod swreg30_refer5_poc;
#[doc = "SWREG31_VP9_AREF_SCALE (rw) register accessor: scaling factor for alfter reference picture\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg31_vp9_aref_scale::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg31_vp9_aref_scale::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg31_vp9_aref_scale`]
module"]
#[doc(alias = "SWREG31_VP9_AREF_SCALE")]
pub type Swreg31Vp9ArefScale = crate::Reg<swreg31_vp9_aref_scale::Swreg31Vp9ArefScaleSpec>;
#[doc = "scaling factor for alfter reference picture"]
pub mod swreg31_vp9_aref_scale;
#[doc = "SWREG31_REFER6_POC (rw) register accessor: the poc of reference picture index 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg31_refer6_poc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg31_refer6_poc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg31_refer6_poc`]
module"]
#[doc(alias = "SWREG31_REFER6_POC")]
pub type Swreg31Refer6Poc = crate::Reg<swreg31_refer6_poc::Swreg31Refer6PocSpec>;
#[doc = "the poc of reference picture index 6"]
pub mod swreg31_refer6_poc;
#[doc = "SWREG32_VP9_REF_DELTAS_LASTFRAME (rw) register accessor: vp9 ref deltas\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg32_vp9_ref_deltas_lastframe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg32_vp9_ref_deltas_lastframe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg32_vp9_ref_deltas_lastframe`]
module"]
#[doc(alias = "SWREG32_VP9_REF_DELTAS_LASTFRAME")]
pub type Swreg32Vp9RefDeltasLastframe =
    crate::Reg<swreg32_vp9_ref_deltas_lastframe::Swreg32Vp9RefDeltasLastframeSpec>;
#[doc = "vp9 ref deltas"]
pub mod swreg32_vp9_ref_deltas_lastframe;
#[doc = "SWREG32_REFER7_POC (rw) register accessor: the poc of reference picture index 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg32_refer7_poc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg32_refer7_poc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg32_refer7_poc`]
module"]
#[doc(alias = "SWREG32_REFER7_POC")]
pub type Swreg32Refer7Poc = crate::Reg<swreg32_refer7_poc::Swreg32Refer7PocSpec>;
#[doc = "the poc of reference picture index 7"]
pub mod swreg32_refer7_poc;
#[doc = "SWREG33_VP9_INFO_LASTFRAME (rw) register accessor: vp9 info for lastframe\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg33_vp9_info_lastframe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg33_vp9_info_lastframe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg33_vp9_info_lastframe`]
module"]
#[doc(alias = "SWREG33_VP9_INFO_LASTFRAME")]
pub type Swreg33Vp9InfoLastframe =
    crate::Reg<swreg33_vp9_info_lastframe::Swreg33Vp9InfoLastframeSpec>;
#[doc = "vp9 info for lastframe"]
pub mod swreg33_vp9_info_lastframe;
#[doc = "SWREG33_REFER8_POC (rw) register accessor: the poc of reference picture index 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg33_refer8_poc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg33_refer8_poc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg33_refer8_poc`]
module"]
#[doc(alias = "SWREG33_REFER8_POC")]
pub type Swreg33Refer8Poc = crate::Reg<swreg33_refer8_poc::Swreg33Refer8PocSpec>;
#[doc = "the poc of reference picture index 8"]
pub mod swreg33_refer8_poc;
#[doc = "SWREG34_VP9_INTERCMD_BASE (rw) register accessor: inter cmd base addr\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg34_vp9_intercmd_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg34_vp9_intercmd_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg34_vp9_intercmd_base`]
module"]
#[doc(alias = "SWREG34_VP9_INTERCMD_BASE")]
pub type Swreg34Vp9IntercmdBase = crate::Reg<swreg34_vp9_intercmd_base::Swreg34Vp9IntercmdBaseSpec>;
#[doc = "inter cmd base addr"]
pub mod swreg34_vp9_intercmd_base;
#[doc = "SWREG34_REFER9_POC (rw) register accessor: the poc of reference picture index 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg34_refer9_poc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg34_refer9_poc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg34_refer9_poc`]
module"]
#[doc(alias = "SWREG34_REFER9_POC")]
pub type Swreg34Refer9Poc = crate::Reg<swreg34_refer9_poc::Swreg34Refer9PocSpec>;
#[doc = "the poc of reference picture index 9"]
pub mod swreg34_refer9_poc;
#[doc = "SWREG35_VP9_INTERCMD_NUM (rw) register accessor: vp9 intercmd num\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg35_vp9_intercmd_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg35_vp9_intercmd_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg35_vp9_intercmd_num`]
module"]
#[doc(alias = "SWREG35_VP9_INTERCMD_NUM")]
pub type Swreg35Vp9IntercmdNum = crate::Reg<swreg35_vp9_intercmd_num::Swreg35Vp9IntercmdNumSpec>;
#[doc = "vp9 intercmd num"]
pub mod swreg35_vp9_intercmd_num;
#[doc = "SWREG35_REFER10_POC (rw) register accessor: the poc of reference picture index 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg35_refer10_poc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg35_refer10_poc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg35_refer10_poc`]
module"]
#[doc(alias = "SWREG35_REFER10_POC")]
pub type Swreg35Refer10Poc = crate::Reg<swreg35_refer10_poc::Swreg35Refer10PocSpec>;
#[doc = "the poc of reference picture index 10"]
pub mod swreg35_refer10_poc;
#[doc = "SWREG36_VP9_LASTTILE_SIZE (rw) register accessor: vp9 lasttile size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg36_vp9_lasttile_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg36_vp9_lasttile_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg36_vp9_lasttile_size`]
module"]
#[doc(alias = "SWREG36_VP9_LASTTILE_SIZE")]
pub type Swreg36Vp9LasttileSize = crate::Reg<swreg36_vp9_lasttile_size::Swreg36Vp9LasttileSizeSpec>;
#[doc = "vp9 lasttile size"]
pub mod swreg36_vp9_lasttile_size;
#[doc = "SWREG36_REFER11_POC (rw) register accessor: the poc of reference picture index 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg36_refer11_poc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg36_refer11_poc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg36_refer11_poc`]
module"]
#[doc(alias = "SWREG36_REFER11_POC")]
pub type Swreg36Refer11Poc = crate::Reg<swreg36_refer11_poc::Swreg36Refer11PocSpec>;
#[doc = "the poc of reference picture index 11"]
pub mod swreg36_refer11_poc;
#[doc = "SWREG37_VP9_LASTF_HOR_VIRSTRIDE (rw) register accessor: Register0000 Abstract\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg37_vp9_lastf_hor_virstride::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg37_vp9_lastf_hor_virstride::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg37_vp9_lastf_hor_virstride`]
module"]
#[doc(alias = "SWREG37_VP9_LASTF_HOR_VIRSTRIDE")]
pub type Swreg37Vp9LastfHorVirstride =
    crate::Reg<swreg37_vp9_lastf_hor_virstride::Swreg37Vp9LastfHorVirstrideSpec>;
#[doc = "Register0000 Abstract"]
pub mod swreg37_vp9_lastf_hor_virstride;
#[doc = "SWREG37_REFER12_POC (rw) register accessor: the poc of reference picture index 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg37_refer12_poc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg37_refer12_poc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg37_refer12_poc`]
module"]
#[doc(alias = "SWREG37_REFER12_POC")]
pub type Swreg37Refer12Poc = crate::Reg<swreg37_refer12_poc::Swreg37Refer12PocSpec>;
#[doc = "the poc of reference picture index 12"]
pub mod swreg37_refer12_poc;
#[doc = "SWREG38_VP9_GOLDENF_HOR_VIRSTRIDE (rw) register accessor: vp9 golden frame horizontal virstride\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg38_vp9_goldenf_hor_virstride::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg38_vp9_goldenf_hor_virstride::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg38_vp9_goldenf_hor_virstride`]
module"]
#[doc(alias = "SWREG38_VP9_GOLDENF_HOR_VIRSTRIDE")]
pub type Swreg38Vp9GoldenfHorVirstride =
    crate::Reg<swreg38_vp9_goldenf_hor_virstride::Swreg38Vp9GoldenfHorVirstrideSpec>;
#[doc = "vp9 golden frame horizontal virstride"]
pub mod swreg38_vp9_goldenf_hor_virstride;
#[doc = "SWREG38_REFER13_POC (rw) register accessor: the poc of reference picture index 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg38_refer13_poc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg38_refer13_poc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg38_refer13_poc`]
module"]
#[doc(alias = "SWREG38_REFER13_POC")]
pub type Swreg38Refer13Poc = crate::Reg<swreg38_refer13_poc::Swreg38Refer13PocSpec>;
#[doc = "the poc of reference picture index 13"]
pub mod swreg38_refer13_poc;
#[doc = "SWREG39_VP9_ALTREFF_HOR_VIRSTRIDE (rw) register accessor: vp9 altref frame horizontal virstride\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg39_vp9_altreff_hor_virstride::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg39_vp9_altreff_hor_virstride::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg39_vp9_altreff_hor_virstride`]
module"]
#[doc(alias = "SWREG39_VP9_ALTREFF_HOR_VIRSTRIDE")]
pub type Swreg39Vp9AltreffHorVirstride =
    crate::Reg<swreg39_vp9_altreff_hor_virstride::Swreg39Vp9AltreffHorVirstrideSpec>;
#[doc = "vp9 altref frame horizontal virstride"]
pub mod swreg39_vp9_altreff_hor_virstride;
#[doc = "SWREG39_REFER14_POC (rw) register accessor: the poc of reference picture index 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg39_refer14_poc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg39_refer14_poc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg39_refer14_poc`]
module"]
#[doc(alias = "SWREG39_REFER14_POC")]
pub type Swreg39Refer14Poc = crate::Reg<swreg39_refer14_poc::Swreg39Refer14PocSpec>;
#[doc = "the poc of reference picture index 14"]
pub mod swreg39_refer14_poc;
#[doc = "SWREG40_CUR_POC (rw) register accessor: the poc of cur picture\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg40_cur_poc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg40_cur_poc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg40_cur_poc`]
module"]
#[doc(alias = "SWREG40_CUR_POC")]
pub type Swreg40CurPoc = crate::Reg<swreg40_cur_poc::Swreg40CurPocSpec>;
#[doc = "the poc of cur picture"]
pub mod swreg40_cur_poc;
#[doc = "SWREG41_RLCWRITE_BASE (rw) register accessor: the base address or rlcwrite base addr\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg41_rlcwrite_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg41_rlcwrite_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg41_rlcwrite_base`]
module"]
#[doc(alias = "SWREG41_RLCWRITE_BASE")]
pub type Swreg41RlcwriteBase = crate::Reg<swreg41_rlcwrite_base::Swreg41RlcwriteBaseSpec>;
#[doc = "the base address or rlcwrite base addr"]
pub mod swreg41_rlcwrite_base;
#[doc = "SWREG42_PPS_BASE (rw) register accessor: the base address of pps\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg42_pps_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg42_pps_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg42_pps_base`]
module"]
#[doc(alias = "SWREG42_PPS_BASE")]
pub type Swreg42PpsBase = crate::Reg<swreg42_pps_base::Swreg42PpsBaseSpec>;
#[doc = "the base address of pps"]
pub mod swreg42_pps_base;
#[doc = "SWREG43_RPS_BASE (rw) register accessor: the base address of rps\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg43_rps_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg43_rps_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg43_rps_base`]
module"]
#[doc(alias = "SWREG43_RPS_BASE")]
pub type Swreg43RpsBase = crate::Reg<swreg43_rps_base::Swreg43RpsBaseSpec>;
#[doc = "the base address of rps"]
pub mod swreg43_rps_base;
#[doc = "SWREG44_STRMD_ERROR_EN (rw) register accessor: cabac error enable config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg44_strmd_error_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg44_strmd_error_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg44_strmd_error_en`]
module"]
#[doc(alias = "SWREG44_STRMD_ERROR_EN")]
pub type Swreg44StrmdErrorEn = crate::Reg<swreg44_strmd_error_en::Swreg44StrmdErrorEnSpec>;
#[doc = "cabac error enable config"]
pub mod swreg44_strmd_error_en;
#[doc = "SWREG45_STRMD_ERROR_STATUS (rw) register accessor: cabac error status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg45_strmd_error_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg45_strmd_error_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg45_strmd_error_status`]
module"]
#[doc(alias = "SWREG45_STRMD_ERROR_STATUS")]
pub type Swreg45StrmdErrorStatus =
    crate::Reg<swreg45_strmd_error_status::Swreg45StrmdErrorStatusSpec>;
#[doc = "cabac error status"]
pub mod swreg45_strmd_error_status;
#[doc = "SWREG45_VP9_ERROR_INFO0 (rw) register accessor: vp9 error info\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg45_vp9_error_info0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg45_vp9_error_info0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg45_vp9_error_info0`]
module"]
#[doc(alias = "SWREG45_VP9_ERROR_INFO0")]
pub type Swreg45Vp9ErrorInfo0 = crate::Reg<swreg45_vp9_error_info0::Swreg45Vp9ErrorInfo0Spec>;
#[doc = "vp9 error info"]
pub mod swreg45_vp9_error_info0;
#[doc = "SWREG46_STRMD_ERROR_CTU (rw) register accessor: strmd error ctu\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg46_strmd_error_ctu::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg46_strmd_error_ctu::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg46_strmd_error_ctu`]
module"]
#[doc(alias = "SWREG46_STRMD_ERROR_CTU")]
pub type Swreg46StrmdErrorCtu = crate::Reg<swreg46_strmd_error_ctu::Swreg46StrmdErrorCtuSpec>;
#[doc = "strmd error ctu"]
pub mod swreg46_strmd_error_ctu;
#[doc = "SWREG47_SAO_CTU_POSITION (rw) register accessor: sao ctu position\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg47_sao_ctu_position::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg47_sao_ctu_position::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg47_sao_ctu_position`]
module"]
#[doc(alias = "SWREG47_SAO_CTU_POSITION")]
pub type Swreg47SaoCtuPosition = crate::Reg<swreg47_sao_ctu_position::Swreg47SaoCtuPositionSpec>;
#[doc = "sao ctu position"]
pub mod swreg47_sao_ctu_position;
#[doc = "SWREG48_H264_REFER15_BASE (rw) register accessor: base address for reference picture index 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg48_h264_refer15_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg48_h264_refer15_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg48_h264_refer15_base`]
module"]
#[doc(alias = "SWREG48_H264_REFER15_BASE")]
pub type Swreg48H264Refer15Base = crate::Reg<swreg48_h264_refer15_base::Swreg48H264Refer15BaseSpec>;
#[doc = "base address for reference picture index 15"]
pub mod swreg48_h264_refer15_base;
#[doc = "SWREG48_VP9_LAST_YSTRIDE (rw) register accessor: last ref ystride\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg48_vp9_last_ystride::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg48_vp9_last_ystride::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg48_vp9_last_ystride`]
module"]
#[doc(alias = "SWREG48_VP9_LAST_YSTRIDE")]
pub type Swreg48Vp9LastYstride = crate::Reg<swreg48_vp9_last_ystride::Swreg48Vp9LastYstrideSpec>;
#[doc = "last ref ystride"]
pub mod swreg48_vp9_last_ystride;
#[doc = "SWREG49_H264_REFER15_POC (rw) register accessor: the poc of reference picture index 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg49_h264_refer15_poc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg49_h264_refer15_poc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg49_h264_refer15_poc`]
module"]
#[doc(alias = "SWREG49_H264_REFER15_POC")]
pub type Swreg49H264Refer15Poc = crate::Reg<swreg49_h264_refer15_poc::Swreg49H264Refer15PocSpec>;
#[doc = "the poc of reference picture index 15"]
pub mod swreg49_h264_refer15_poc;
#[doc = "SWREG49_VP9_GOLDEN_YSTRIDE (rw) register accessor: golden ref ystride\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg49_vp9_golden_ystride::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg49_vp9_golden_ystride::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg49_vp9_golden_ystride`]
module"]
#[doc(alias = "SWREG49_VP9_GOLDEN_YSTRIDE")]
pub type Swreg49Vp9GoldenYstride =
    crate::Reg<swreg49_vp9_golden_ystride::Swreg49Vp9GoldenYstrideSpec>;
#[doc = "golden ref ystride"]
pub mod swreg49_vp9_golden_ystride;
#[doc = "SWREG50_H264_REFER16_POC (rw) register accessor: the poc of reference picture index 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg50_h264_refer16_poc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg50_h264_refer16_poc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg50_h264_refer16_poc`]
module"]
#[doc(alias = "SWREG50_H264_REFER16_POC")]
pub type Swreg50H264Refer16Poc = crate::Reg<swreg50_h264_refer16_poc::Swreg50H264Refer16PocSpec>;
#[doc = "the poc of reference picture index 16"]
pub mod swreg50_h264_refer16_poc;
#[doc = "SWREG50_VP9_ALTREFY_YSTRIDE (rw) register accessor: altref ref ystride\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg50_vp9_altrefy_ystride::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg50_vp9_altrefy_ystride::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg50_vp9_altrefy_ystride`]
module"]
#[doc(alias = "SWREG50_VP9_ALTREFY_YSTRIDE")]
pub type Swreg50Vp9AltrefyYstride =
    crate::Reg<swreg50_vp9_altrefy_ystride::Swreg50Vp9AltrefyYstrideSpec>;
#[doc = "altref ref ystride"]
pub mod swreg50_vp9_altrefy_ystride;
#[doc = "SWREG51_H264_REFER17_POC (rw) register accessor: the poc of reference picture index 17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg51_h264_refer17_poc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg51_h264_refer17_poc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg51_h264_refer17_poc`]
module"]
#[doc(alias = "SWREG51_H264_REFER17_POC")]
pub type Swreg51H264Refer17Poc = crate::Reg<swreg51_h264_refer17_poc::Swreg51H264Refer17PocSpec>;
#[doc = "the poc of reference picture index 17"]
pub mod swreg51_h264_refer17_poc;
#[doc = "SWREG51_VP9_LASTREF_YUVSTRIDE (rw) register accessor: vp9 lastref yuv stride\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg51_vp9_lastref_yuvstride::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg51_vp9_lastref_yuvstride::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg51_vp9_lastref_yuvstride`]
module"]
#[doc(alias = "SWREG51_VP9_LASTREF_YUVSTRIDE")]
pub type Swreg51Vp9LastrefYuvstride =
    crate::Reg<swreg51_vp9_lastref_yuvstride::Swreg51Vp9LastrefYuvstrideSpec>;
#[doc = "vp9 lastref yuv stride"]
pub mod swreg51_vp9_lastref_yuvstride;
#[doc = "SWREG52_VP9_REFCOLMV_BASE (rw) register accessor: vp9 colmv ref base\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg52_vp9_refcolmv_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg52_vp9_refcolmv_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg52_vp9_refcolmv_base`]
module"]
#[doc(alias = "SWREG52_VP9_REFCOLMV_BASE")]
pub type Swreg52Vp9RefcolmvBase = crate::Reg<swreg52_vp9_refcolmv_base::Swreg52Vp9RefcolmvBaseSpec>;
#[doc = "vp9 colmv ref base"]
pub mod swreg52_vp9_refcolmv_base;
#[doc = "SWREG52_H264_REFER18_POC (rw) register accessor: the poc of reference picture index 18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg52_h264_refer18_poc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg52_h264_refer18_poc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg52_h264_refer18_poc`]
module"]
#[doc(alias = "SWREG52_H264_REFER18_POC")]
pub type Swreg52H264Refer18Poc = crate::Reg<swreg52_h264_refer18_poc::Swreg52H264Refer18PocSpec>;
#[doc = "the poc of reference picture index 18"]
pub mod swreg52_h264_refer18_poc;
#[doc = "SWREG53_H264_REFER19_POC (rw) register accessor: the poc of reference picture index 19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg53_h264_refer19_poc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg53_h264_refer19_poc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg53_h264_refer19_poc`]
module"]
#[doc(alias = "SWREG53_H264_REFER19_POC")]
pub type Swreg53H264Refer19Poc = crate::Reg<swreg53_h264_refer19_poc::Swreg53H264Refer19PocSpec>;
#[doc = "the poc of reference picture index 19"]
pub mod swreg53_h264_refer19_poc;
#[doc = "SWREG54_H264_REFER20_POC (rw) register accessor: the poc of reference picture index 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg54_h264_refer20_poc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg54_h264_refer20_poc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg54_h264_refer20_poc`]
module"]
#[doc(alias = "SWREG54_H264_REFER20_POC")]
pub type Swreg54H264Refer20Poc = crate::Reg<swreg54_h264_refer20_poc::Swreg54H264Refer20PocSpec>;
#[doc = "the poc of reference picture index 20"]
pub mod swreg54_h264_refer20_poc;
#[doc = "SWREG55_H264_REFER21_POC (rw) register accessor: the poc of reference picture index 21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg55_h264_refer21_poc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg55_h264_refer21_poc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg55_h264_refer21_poc`]
module"]
#[doc(alias = "SWREG55_H264_REFER21_POC")]
pub type Swreg55H264Refer21Poc = crate::Reg<swreg55_h264_refer21_poc::Swreg55H264Refer21PocSpec>;
#[doc = "the poc of reference picture index 21"]
pub mod swreg55_h264_refer21_poc;
#[doc = "SWREG56_H264_REFER22_POC (rw) register accessor: the poc of reference picture index 22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg56_h264_refer22_poc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg56_h264_refer22_poc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg56_h264_refer22_poc`]
module"]
#[doc(alias = "SWREG56_H264_REFER22_POC")]
pub type Swreg56H264Refer22Poc = crate::Reg<swreg56_h264_refer22_poc::Swreg56H264Refer22PocSpec>;
#[doc = "the poc of reference picture index 22"]
pub mod swreg56_h264_refer22_poc;
#[doc = "SWREG57_H264_REFER23_POC (rw) register accessor: the poc of reference picture index 23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg57_h264_refer23_poc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg57_h264_refer23_poc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg57_h264_refer23_poc`]
module"]
#[doc(alias = "SWREG57_H264_REFER23_POC")]
pub type Swreg57H264Refer23Poc = crate::Reg<swreg57_h264_refer23_poc::Swreg57H264Refer23PocSpec>;
#[doc = "the poc of reference picture index 23"]
pub mod swreg57_h264_refer23_poc;
#[doc = "SWREG58_H264_REFER24_POC (rw) register accessor: the poc of reference picture index 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg58_h264_refer24_poc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg58_h264_refer24_poc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg58_h264_refer24_poc`]
module"]
#[doc(alias = "SWREG58_H264_REFER24_POC")]
pub type Swreg58H264Refer24Poc = crate::Reg<swreg58_h264_refer24_poc::Swreg58H264Refer24PocSpec>;
#[doc = "the poc of reference picture index 24"]
pub mod swreg58_h264_refer24_poc;
#[doc = "SWREG59_H264_REFER25_POC (rw) register accessor: the poc of reference picture index 25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg59_h264_refer25_poc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg59_h264_refer25_poc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg59_h264_refer25_poc`]
module"]
#[doc(alias = "SWREG59_H264_REFER25_POC")]
pub type Swreg59H264Refer25Poc = crate::Reg<swreg59_h264_refer25_poc::Swreg59H264Refer25PocSpec>;
#[doc = "the poc of reference picture index 25"]
pub mod swreg59_h264_refer25_poc;
#[doc = "SWREG60_H264_REFER26_POC (rw) register accessor: the poc of reference picture index 26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg60_h264_refer26_poc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg60_h264_refer26_poc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg60_h264_refer26_poc`]
module"]
#[doc(alias = "SWREG60_H264_REFER26_POC")]
pub type Swreg60H264Refer26Poc = crate::Reg<swreg60_h264_refer26_poc::Swreg60H264Refer26PocSpec>;
#[doc = "the poc of reference picture index 26"]
pub mod swreg60_h264_refer26_poc;
#[doc = "SWREG61_H264_REFER27_POC (rw) register accessor: the poc of reference picture index 27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg61_h264_refer27_poc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg61_h264_refer27_poc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg61_h264_refer27_poc`]
module"]
#[doc(alias = "SWREG61_H264_REFER27_POC")]
pub type Swreg61H264Refer27Poc = crate::Reg<swreg61_h264_refer27_poc::Swreg61H264Refer27PocSpec>;
#[doc = "the poc of reference picture index 27"]
pub mod swreg61_h264_refer27_poc;
#[doc = "SWREG62_H264_REFER28_POC (rw) register accessor: the poc of reference picture index 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg62_h264_refer28_poc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg62_h264_refer28_poc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg62_h264_refer28_poc`]
module"]
#[doc(alias = "SWREG62_H264_REFER28_POC")]
pub type Swreg62H264Refer28Poc = crate::Reg<swreg62_h264_refer28_poc::Swreg62H264Refer28PocSpec>;
#[doc = "the poc of reference picture index 28"]
pub mod swreg62_h264_refer28_poc;
#[doc = "SWREG63_H264_REFER29_POC (rw) register accessor: the poc of reference picture index 29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg63_h264_refer29_poc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg63_h264_refer29_poc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg63_h264_refer29_poc`]
module"]
#[doc(alias = "SWREG63_H264_REFER29_POC")]
pub type Swreg63H264Refer29Poc = crate::Reg<swreg63_h264_refer29_poc::Swreg63H264Refer29PocSpec>;
#[doc = "the poc of reference picture index 29"]
pub mod swreg63_h264_refer29_poc;
#[doc = "SWREG64_PERFORMANCE_CYCLE (rw) register accessor: hevc performance cycle\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg64_performance_cycle::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg64_performance_cycle::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg64_performance_cycle`]
module"]
#[doc(alias = "SWREG64_PERFORMANCE_CYCLE")]
pub type Swreg64PerformanceCycle =
    crate::Reg<swreg64_performance_cycle::Swreg64PerformanceCycleSpec>;
#[doc = "hevc performance cycle"]
pub mod swreg64_performance_cycle;
#[doc = "SWREG65_AXI_DDR_RDATA (rw) register accessor: axi ddr read data num\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg65_axi_ddr_rdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg65_axi_ddr_rdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg65_axi_ddr_rdata`]
module"]
#[doc(alias = "SWREG65_AXI_DDR_RDATA")]
pub type Swreg65AxiDdrRdata = crate::Reg<swreg65_axi_ddr_rdata::Swreg65AxiDdrRdataSpec>;
#[doc = "axi ddr read data num"]
pub mod swreg65_axi_ddr_rdata;
#[doc = "SWREG66_AXI_DDR_WDATA (rw) register accessor: axi ddr write data number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg66_axi_ddr_wdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg66_axi_ddr_wdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg66_axi_ddr_wdata`]
module"]
#[doc(alias = "SWREG66_AXI_DDR_WDATA")]
pub type Swreg66AxiDdrWdata = crate::Reg<swreg66_axi_ddr_wdata::Swreg66AxiDdrWdataSpec>;
#[doc = "axi ddr write data number"]
pub mod swreg66_axi_ddr_wdata;
#[doc = "SWREG68_PERFORMANCE_SEL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg68_performance_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg68_performance_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg68_performance_sel`]
module"]
#[doc(alias = "SWREG68_PERFORMANCE_SEL")]
pub type Swreg68PerformanceSel = crate::Reg<swreg68_performance_sel::Swreg68PerformanceSelSpec>;
#[doc = ""]
pub mod swreg68_performance_sel;
#[doc = "SWREG69_PERFORMANCE_CNT0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg69_performance_cnt0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg69_performance_cnt0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg69_performance_cnt0`]
module"]
#[doc(alias = "SWREG69_PERFORMANCE_CNT0")]
pub type Swreg69PerformanceCnt0 = crate::Reg<swreg69_performance_cnt0::Swreg69PerformanceCnt0Spec>;
#[doc = ""]
pub mod swreg69_performance_cnt0;
#[doc = "SWREG70_PERFORMANCE_CNT1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg70_performance_cnt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg70_performance_cnt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg70_performance_cnt1`]
module"]
#[doc(alias = "SWREG70_PERFORMANCE_CNT1")]
pub type Swreg70PerformanceCnt1 = crate::Reg<swreg70_performance_cnt1::Swreg70PerformanceCnt1Spec>;
#[doc = ""]
pub mod swreg70_performance_cnt1;
#[doc = "SWREG71_PERFORMANCE_CNT2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg71_performance_cnt2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg71_performance_cnt2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg71_performance_cnt2`]
module"]
#[doc(alias = "SWREG71_PERFORMANCE_CNT2")]
pub type Swreg71PerformanceCnt2 = crate::Reg<swreg71_performance_cnt2::Swreg71PerformanceCnt2Spec>;
#[doc = ""]
pub mod swreg71_performance_cnt2;
#[doc = "SWREG72_H264_REFER30_POC (rw) register accessor: the poc of reference picture index 30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg72_h264_refer30_poc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg72_h264_refer30_poc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg72_h264_refer30_poc`]
module"]
#[doc(alias = "SWREG72_H264_REFER30_POC")]
pub type Swreg72H264Refer30Poc = crate::Reg<swreg72_h264_refer30_poc::Swreg72H264Refer30PocSpec>;
#[doc = "the poc of reference picture index 30"]
pub mod swreg72_h264_refer30_poc;
#[doc = "SWREG73_H264_REFER31_POC (rw) register accessor: the poc of reference picture index 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg73_h264_refer31_poc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg73_h264_refer31_poc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg73_h264_refer31_poc`]
module"]
#[doc(alias = "SWREG73_H264_REFER31_POC")]
pub type Swreg73H264Refer31Poc = crate::Reg<swreg73_h264_refer31_poc::Swreg73H264Refer31PocSpec>;
#[doc = "the poc of reference picture index 31"]
pub mod swreg73_h264_refer31_poc;
#[doc = "SWREG74_H264_CUR_POC1 (rw) register accessor: h264 cur poc for bottom filed\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg74_h264_cur_poc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg74_h264_cur_poc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg74_h264_cur_poc1`]
module"]
#[doc(alias = "SWREG74_H264_CUR_POC1")]
pub type Swreg74H264CurPoc1 = crate::Reg<swreg74_h264_cur_poc1::Swreg74H264CurPoc1Spec>;
#[doc = "h264 cur poc for bottom filed"]
pub mod swreg74_h264_cur_poc1;
#[doc = "SWREG75_H264_ERRORINFO_BASE (rw) register accessor: h264 error info base addr\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg75_h264_errorinfo_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg75_h264_errorinfo_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg75_h264_errorinfo_base`]
module"]
#[doc(alias = "SWREG75_H264_ERRORINFO_BASE")]
pub type Swreg75H264ErrorinfoBase =
    crate::Reg<swreg75_h264_errorinfo_base::Swreg75H264ErrorinfoBaseSpec>;
#[doc = "h264 error info base addr"]
pub mod swreg75_h264_errorinfo_base;
#[doc = "SWREG76_H264_ERRORINFO_NUM (rw) register accessor: h264 error info num\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg76_h264_errorinfo_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg76_h264_errorinfo_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg76_h264_errorinfo_num`]
module"]
#[doc(alias = "SWREG76_H264_ERRORINFO_NUM")]
pub type Swreg76H264ErrorinfoNum =
    crate::Reg<swreg76_h264_errorinfo_num::Swreg76H264ErrorinfoNumSpec>;
#[doc = "h264 error info num"]
pub mod swreg76_h264_errorinfo_num;
#[doc = "SWREG77_H264_ERROR_E (rw) register accessor: h264 error enable high bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg77_h264_error_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg77_h264_error_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg77_h264_error_e`]
module"]
#[doc(alias = "SWREG77_H264_ERROR_E")]
pub type Swreg77H264ErrorE = crate::Reg<swreg77_h264_error_e::Swreg77H264ErrorESpec>;
#[doc = "h264 error enable high bits"]
pub mod swreg77_h264_error_e;
