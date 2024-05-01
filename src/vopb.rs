#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    reg_cfg_done: RegCfgDone,
    version_info: VersionInfo,
    sys_ctrl: SysCtrl,
    sys_ctrl1: SysCtrl1,
    dsp_ctrl0: DspCtrl0,
    dsp_ctrl1: DspCtrl1,
    dsp_bg: DspBg,
    mcu_ctrl: McuCtrl,
    wb_ctrl0: WbCtrl0,
    wb_ctrl1: WbCtrl1,
    wb_yrgb_mst: WbYrgbMst,
    wb_cbr_mst: WbCbrMst,
    win0_ctrl0: Win0Ctrl0,
    win0_ctrl1: Win0Ctrl1,
    win0_color_key: Win0ColorKey,
    win0_vir: Win0Vir,
    win0_yrgb_mst: Win0YrgbMst,
    win0_cbr_mst: Win0CbrMst,
    win0_act_info: Win0ActInfo,
    win0_dsp_info: Win0DspInfo,
    win0_dsp_st: Win0DspSt,
    win0_scl_factor_yrgb: Win0SclFactorYrgb,
    win0_scl_factor_cbr: Win0SclFactorCbr,
    win0_scl_offset: Win0SclOffset,
    win0_src_alpha_ctrl: Win0SrcAlphaCtrl,
    win0_dst_alpha_ctrl: Win0DstAlphaCtrl,
    win0_fading_ctrl: Win0FadingCtrl,
    win0_ctrl2: Win0Ctrl2,
    win1_ctrl0: Win1Ctrl0,
    win1_ctrl1: Win1Ctrl1,
    win1_color_key: Win1ColorKey,
    win1_vir: Win1Vir,
    win1_yrgb_mst: Win1YrgbMst,
    win1_cbr_mst: Win1CbrMst,
    win1_act_info: Win1ActInfo,
    win1_dsp_info: Win1DspInfo,
    win1_dsp_st: Win1DspSt,
    win1_scl_factor_yrgb: Win1SclFactorYrgb,
    win1_scl_factor_cbr: Win1SclFactorCbr,
    win1_scl_offset: Win1SclOffset,
    win1_src_alpha_ctrl: Win1SrcAlphaCtrl,
    win1_dst_alpha_ctrl: Win1DstAlphaCtrl,
    win1_fading_ctrl: Win1FadingCtrl,
    win1_ctrl2: Win1Ctrl2,
    win2_ctrl0: Win2Ctrl0,
    win2_ctrl1: Win2Ctrl1,
    win2_vir0_1: Win2Vir0_1,
    win2_vir2_3: Win2Vir2_3,
    win2_mst0: Win2Mst0,
    win2_dsp_info0: Win2DspInfo0,
    win2_dsp_st0: Win2DspSt0,
    win2_color_key: Win2ColorKey,
    win2_mst1: Win2Mst1,
    win2_dsp_info1: Win2DspInfo1,
    win2_dsp_st1: Win2DspSt1,
    win2_src_alpha_ctrl: Win2SrcAlphaCtrl,
    win2_mst2: Win2Mst2,
    win2_dsp_info2: Win2DspInfo2,
    win2_dsp_st2: Win2DspSt2,
    win2_dst_alpha_ctrl: Win2DstAlphaCtrl,
    win2_mst3: Win2Mst3,
    win2_dsp_info3: Win2DspInfo3,
    win2_dsp_st3: Win2DspSt3,
    win2_fading_ctrl: Win2FadingCtrl,
    win3_ctrl0: Win3Ctrl0,
    win3_ctrl1: Win3Ctrl1,
    win3_vir0_1: Win3Vir0_1,
    win3_vir2_3: Win3Vir2_3,
    win3_mst0: Win3Mst0,
    win3_dsp_info0: Win3DspInfo0,
    win3_dsp_st0: Win3DspSt0,
    win3_color_key: Win3ColorKey,
    win3_mst1: Win3Mst1,
    win3_dsp_info1: Win3DspInfo1,
    win3_dsp_st1: Win3DspSt1,
    win3_src_alpha_ctrl: Win3SrcAlphaCtrl,
    win3_mst2: Win3Mst2,
    win3_dsp_info2: Win3DspInfo2,
    win3_dsp_st2: Win3DspSt2,
    win3_dst_alpha_ctrl: Win3DstAlphaCtrl,
    win3_mst3: Win3Mst3,
    win3_dsp_info3: Win3DspInfo3,
    win3_dsp_st3: Win3DspSt3,
    win3_fading_ctrl: Win3FadingCtrl,
    hwc_ctrl0: HwcCtrl0,
    hwc_ctrl1: HwcCtrl1,
    hwc_mst: HwcMst,
    hwc_dsp_st: HwcDspSt,
    hwc_src_alpha_ctrl: HwcSrcAlphaCtrl,
    hwc_dst_alpha_ctrl: HwcDstAlphaCtrl,
    hwc_fading_ctrl: HwcFadingCtrl,
    _reserved91: [u8; 0x04],
    post_dsp_hact_info: PostDspHactInfo,
    post_dsp_vact_info: PostDspVactInfo,
    post_scl_factor_yrgb: PostSclFactorYrgb,
    post_reserved: PostReserved,
    post_scl_ctrl: PostSclCtrl,
    post_dsp_vact_info_f1: PostDspVactInfoF1,
    dsp_htotal_hs_end: DspHtotalHsEnd,
    dsp_hact_st_end: DspHactStEnd,
    dsp_vtotal_vs_end: DspVtotalVsEnd,
    dsp_vact_st_end: DspVactStEnd,
    dsp_vs_st_end_f1: DspVsStEndF1,
    dsp_vact_st_end_f1: DspVactStEndF1,
    pwm_ctrl: PwmCtrl,
    pwm_period_hpr: PwmPeriodHpr,
    pwm_duty_lpr: PwmDutyLpr,
    pwm_cnt: PwmCnt,
    bcsh_color_bar: BcshColorBar,
    bcsh_bcs: BcshBcs,
    bcsh_h: BcshH,
    bcsh_ctrl: BcshCtrl,
    cabc_ctrl0: CabcCtrl0,
    cabc_ctrl1: CabcCtrl1,
    cabc_ctrl2: CabcCtrl2,
    cabc_ctrl3: CabcCtrl3,
    cabc_gauss_line0_0: CabcGaussLine0_0,
    cabc_gauss_line0_1: CabcGaussLine0_1,
    cabc_gauss_line1_0: CabcGaussLine1_0,
    cabc_gauss_line1_1: CabcGaussLine1_1,
    cabc_gauss_line2_0: CabcGaussLine2_0,
    cabc_gauss_line2_1: CabcGaussLine2_1,
    frc_lower01_0: FrcLower01_0,
    frc_lower01_1: FrcLower01_1,
    frc_lower10_0: FrcLower10_0,
    frc_lower10_1: FrcLower10_1,
    frc_lower11_0: FrcLower11_0,
    frc_lower11_1: FrcLower11_1,
    afbcd0_ctrl: Afbcd0Ctrl,
    afbcd0_hdr_ptr: Afbcd0HdrPtr,
    afbcd0_pic_size: Afbcd0PicSize,
    afbcd0_status: Afbcd0Status,
    _reserved131: [u8; 0x70],
    intr_en0: IntrEn0,
    intr_clear0: IntrClear0,
    intr_status0: IntrStatus0,
    intr_raw_status0: IntrRawStatus0,
    intr_en1: IntrEn1,
    intr_clear1: IntrClear1,
    intr_status1: IntrStatus1,
    intr_raw_status1: IntrRawStatus1,
    line_flag: LineFlag,
    vop_status: VopStatus,
    blanking_value: BlankingValue,
    mcu_bypass_port: McuBypassPort,
    win0_dsp_bg: Win0DspBg,
    win1_dsp_bg: Win1DspBg,
    win2_dsp_bg: Win2DspBg,
    win3_dsp_bg: Win3DspBg,
    yuv2yuv_win: Yuv2yuvWin,
    _reserved148: [u8; 0x08],
    auto_gating_en: AutoGatingEn,
    _reserved149: [u8; 0x0210],
    win0_yuv2yuv_y2r_coe0: Win0Yuv2yuvY2rCoe0,
    win0_yuv2yuv_y2r_coe1: Win0Yuv2yuvY2rCoe1,
    win0_yuv2yuv_y2r_coe2: Win0Yuv2yuvY2rCoe2,
    win0_yuv2yuv_y2r_coe3: Win0Yuv2yuvY2rCoe3,
    win0_yuv2yuv_y2r_coe4: Win0Yuv2yuvY2rCoe4,
    win0_yuv2yuv_y2r_coe5: Win0Yuv2yuvY2rCoe5,
    win0_yuv2yuv_y2r_coe6: Win0Yuv2yuvY2rCoe6,
    win0_yuv2yuv_y2r_coe7: Win0Yuv2yuvY2rCoe7,
    win0_yuv2yuv_r2r_coe0: Win0Yuv2yuvR2rCoe0,
    win0_yuv2yuv_r2r_coe1: Win0Yuv2yuvR2rCoe1,
    win0_yuv2yuv_r2r_coe2: Win0Yuv2yuvR2rCoe2,
    win0_yuv2yuv_r2r_coe3: Win0Yuv2yuvR2rCoe3,
    win0_yuv2yuv_r2r_coe4: Win0Yuv2yuvR2rCoe4,
    win0_yuv2yuv_r2r_coe5: Win0Yuv2yuvR2rCoe5,
    win0_yuv2yuv_r2r_coe6: Win0Yuv2yuvR2rCoe6,
    win0_yuv2yuv_r2r_coe7: Win0Yuv2yuvR2rCoe7,
    win0_yuv2yuv_r2y_coe0: Win0Yuv2yuvR2yCoe0,
    win0_yuv2yuv_r2y_coe1: Win0Yuv2yuvR2yCoe1,
    win0_yuv2yuv_r2y_coe2: Win0Yuv2yuvR2yCoe2,
    win0_yuv2yuv_r2y_coe3: Win0Yuv2yuvR2yCoe3,
    win0_yuv2yuv_r2y_coe4: Win0Yuv2yuvR2yCoe4,
    win0_yuv2yuv_r2y_coe5: Win0Yuv2yuvR2yCoe5,
    win0_yuv2yuv_r2y_coe6: Win0Yuv2yuvR2yCoe6,
    win0_yuv2yuv_r2y_coe7: Win0Yuv2yuvR2yCoe7,
    win1_yuv2yuv_y2r_coe0: Win1Yuv2yuvY2rCoe0,
    win1_yuv2yuv_y2r_coe1: Win1Yuv2yuvY2rCoe1,
    win1_yuv2yuv_y2r_coe2: Win1Yuv2yuvY2rCoe2,
    win1_yuv2yuv_y2r_coe3: Win1Yuv2yuvY2rCoe3,
    win1_yuv2yuv_y2r_coe4: Win1Yuv2yuvY2rCoe4,
    win1_yuv2yuv_y2r_coe5: Win1Yuv2yuvY2rCoe5,
    win1_yuv2yuv_y2r_coe6: Win1Yuv2yuvY2rCoe6,
    win1_yuv2yuv_y2r_coe7: Win1Yuv2yuvY2rCoe7,
    win1_yuv2yuv_r2r_coe0: Win1Yuv2yuvR2rCoe0,
    win1_yuv2yuv_r2r_coe1: Win1Yuv2yuvR2rCoe1,
    win1_yuv2yuv_r2r_coe2: Win1Yuv2yuvR2rCoe2,
    win1_yuv2yuv_r2r_coe3: Win1Yuv2yuvR2rCoe3,
    win1_yuv2yuv_r2r_coe4: Win1Yuv2yuvR2rCoe4,
    win1_yuv2yuv_r2r_coe5: Win1Yuv2yuvR2rCoe5,
    win1_yuv2yuv_r2r_coe6: Win1Yuv2yuvR2rCoe6,
    win1_yuv2yuv_r2r_coe7: Win1Yuv2yuvR2rCoe7,
    win1_yuv2yuv_r2y_coe0: Win1Yuv2yuvR2yCoe0,
    win1_yuv2yuv_r2y_coe1: Win1Yuv2yuvR2yCoe1,
    win1_yuv2yuv_r2y_coe2: Win1Yuv2yuvR2yCoe2,
    win1_yuv2yuv_r2y_coe3: Win1Yuv2yuvR2yCoe3,
    win1_yuv2yuv_r2y_coe4: Win1Yuv2yuvR2yCoe4,
    win1_yuv2yuv_r2y_coe5: Win1Yuv2yuvR2yCoe5,
    win1_yuv2yuv_r2y_coe6: Win1Yuv2yuvR2yCoe6,
    win1_yuv2yuv_r2y_coe7: Win1Yuv2yuvR2yCoe7,
    win2_yuv2yuv_y2r_coe0: Win2Yuv2yuvY2rCoe0,
    win2_yuv2yuv_y2r_coe1: Win2Yuv2yuvY2rCoe1,
    win2_yuv2yuv_y2r_coe2: Win2Yuv2yuvY2rCoe2,
    win2_yuv2yuv_y2r_coe3: Win2Yuv2yuvY2rCoe3,
    win2_yuv2yuv_y2r_coe4: Win2Yuv2yuvY2rCoe4,
    win2_yuv2yuv_y2r_coe5: Win2Yuv2yuvY2rCoe5,
    win2_yuv2yuv_y2r_coe6: Win2Yuv2yuvY2rCoe6,
    win2_yuv2yuv_y2r_coe7: Win2Yuv2yuvY2rCoe7,
    win2_yuv2yuv_r2r_coe0: Win2Yuv2yuvR2rCoe0,
    win2_yuv2yuv_r2r_coe1: Win2Yuv2yuvR2rCoe1,
    win2_yuv2yuv_r2r_coe2: Win2Yuv2yuvR2rCoe2,
    win2_yuv2yuv_r2r_coe3: Win2Yuv2yuvR2rCoe3,
    win2_yuv2yuv_r2r_coe4: Win2Yuv2yuvR2rCoe4,
    win2_yuv2yuv_r2r_coe5: Win2Yuv2yuvR2rCoe5,
    win2_yuv2yuv_r2r_coe6: Win2Yuv2yuvR2rCoe6,
    win2_yuv2yuv_r2r_coe7: Win2Yuv2yuvR2rCoe7,
    win2_yuv2yuv_r2y_coe0: Win2Yuv2yuvR2yCoe0,
    win2_yuv2yuv_r2y_coe1: Win2Yuv2yuvR2yCoe1,
    win2_yuv2yuv_r2y_coe2: Win2Yuv2yuvR2yCoe2,
    win2_yuv2yuv_r2y_coe3: Win2Yuv2yuvR2yCoe3,
    win2_yuv2yuv_r2y_coe4: Win2Yuv2yuvR2yCoe4,
    win2_yuv2yuv_r2y_coe5: Win2Yuv2yuvR2yCoe5,
    win2_yuv2yuv_r2y_coe6: Win2Yuv2yuvR2yCoe6,
    win2_yuv2yuv_r2y_coe7: Win2Yuv2yuvR2yCoe7,
    win3_yuv2yuv_y2r_coe0: Win3Yuv2yuvY2rCoe0,
    win3_yuv2yuv_y2r_coe1: Win3Yuv2yuvY2rCoe1,
    win3_yuv2yuv_y2r_coe2: Win3Yuv2yuvY2rCoe2,
    win3_yuv2yuv_y2r_coe3: Win3Yuv2yuvY2rCoe3,
    win3_yuv2yuv_y2r_coe4: Win3Yuv2yuvY2rCoe4,
    win3_yuv2yuv_y2r_coe5: Win3Yuv2yuvY2rCoe5,
    win3_yuv2yuv_y2r_coe6: Win3Yuv2yuvY2rCoe6,
    win3_yuv2yuv_y2r_coe7: Win3Yuv2yuvY2rCoe7,
    win3_yuv2yuv_r2r_coe0: Win3Yuv2yuvR2rCoe0,
    win3_yuv2yuv_r2r_coe1: Win3Yuv2yuvR2rCoe1,
    win3_yuv2yuv_r2r_coe2: Win3Yuv2yuvR2rCoe2,
    win3_yuv2yuv_r2r_coe3: Win3Yuv2yuvR2rCoe3,
    win3_yuv2yuv_r2r_coe4: Win3Yuv2yuvR2rCoe4,
    win3_yuv2yuv_r2r_coe5: Win3Yuv2yuvR2rCoe5,
    win3_yuv2yuv_r2r_coe6: Win3Yuv2yuvR2rCoe6,
    win3_yuv2yuv_r2r_coe7: Win3Yuv2yuvR2rCoe7,
    win3_yuv2yuv_r2y_coe0: Win3Yuv2yuvR2yCoe0,
    win3_yuv2yuv_r2y_coe1: Win3Yuv2yuvR2yCoe1,
    win3_yuv2yuv_r2y_coe2: Win3Yuv2yuvR2yCoe2,
    win3_yuv2yuv_r2y_coe3: Win3Yuv2yuvR2yCoe3,
    win3_yuv2yuv_r2y_coe4: Win3Yuv2yuvR2yCoe4,
    win3_yuv2yuv_r2y_coe5: Win3Yuv2yuvR2yCoe5,
    win3_yuv2yuv_r2y_coe6: Win3Yuv2yuvR2yCoe6,
    win3_yuv2yuv_r2y_coe7: Win3Yuv2yuvR2yCoe7,
    _reserved245: [u8; 0x09a0],
    win2_lut_addr: Win2LutAddr,
    _reserved246: [u8; 0x03fc],
    win3_lut_addr: Win3LutAddr,
    _reserved247: [u8; 0x03fc],
    hwc_lut_addr: HwcLutAddr,
    _reserved248: [u8; 0x03fc],
    cabc_gamma_lut_addr: CabcGammaLutAddr,
    _reserved249: [u8; 0x03fc],
    gamma_lut_addr: GammaLutAddr,
}
impl RegisterBlock {
    #[doc = "0x00 - Register config done flag"]
    #[inline(always)]
    pub const fn reg_cfg_done(&self) -> &RegCfgDone {
        &self.reg_cfg_done
    }
    #[doc = "0x04 - Version for vop"]
    #[inline(always)]
    pub const fn version_info(&self) -> &VersionInfo {
        &self.version_info
    }
    #[doc = "0x08 - System control register0"]
    #[inline(always)]
    pub const fn sys_ctrl(&self) -> &SysCtrl {
        &self.sys_ctrl
    }
    #[doc = "0x0c - System control register1"]
    #[inline(always)]
    pub const fn sys_ctrl1(&self) -> &SysCtrl1 {
        &self.sys_ctrl1
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn dsp_ctrl0(&self) -> &DspCtrl0 {
        &self.dsp_ctrl0
    }
    #[doc = "0x14 - Display control register1"]
    #[inline(always)]
    pub const fn dsp_ctrl1(&self) -> &DspCtrl1 {
        &self.dsp_ctrl1
    }
    #[doc = "0x18 - Background color"]
    #[inline(always)]
    pub const fn dsp_bg(&self) -> &DspBg {
        &self.dsp_bg
    }
    #[doc = "0x1c - MCU mode control register"]
    #[inline(always)]
    pub const fn mcu_ctrl(&self) -> &McuCtrl {
        &self.mcu_ctrl
    }
    #[doc = "0x20 - write back ctrl0"]
    #[inline(always)]
    pub const fn wb_ctrl0(&self) -> &WbCtrl0 {
        &self.wb_ctrl0
    }
    #[doc = "0x24 - write back ctrl1"]
    #[inline(always)]
    pub const fn wb_ctrl1(&self) -> &WbCtrl1 {
        &self.wb_ctrl1
    }
    #[doc = "0x28 - write back yrgb mst"]
    #[inline(always)]
    pub const fn wb_yrgb_mst(&self) -> &WbYrgbMst {
        &self.wb_yrgb_mst
    }
    #[doc = "0x2c - write back cbr mst"]
    #[inline(always)]
    pub const fn wb_cbr_mst(&self) -> &WbCbrMst {
        &self.wb_cbr_mst
    }
    #[doc = "0x30 - Win0 ctrl register0"]
    #[inline(always)]
    pub const fn win0_ctrl0(&self) -> &Win0Ctrl0 {
        &self.win0_ctrl0
    }
    #[doc = "0x34 - Win0 ctrl register1"]
    #[inline(always)]
    pub const fn win0_ctrl1(&self) -> &Win0Ctrl1 {
        &self.win0_ctrl1
    }
    #[doc = "0x38 - Win0 color key register"]
    #[inline(always)]
    pub const fn win0_color_key(&self) -> &Win0ColorKey {
        &self.win0_color_key
    }
    #[doc = "0x3c - Win0 virtual stride"]
    #[inline(always)]
    pub const fn win0_vir(&self) -> &Win0Vir {
        &self.win0_vir
    }
    #[doc = "0x40 - Win0 YRGB memory start address"]
    #[inline(always)]
    pub const fn win0_yrgb_mst(&self) -> &Win0YrgbMst {
        &self.win0_yrgb_mst
    }
    #[doc = "0x44 - Win0 Cbr memory start address"]
    #[inline(always)]
    pub const fn win0_cbr_mst(&self) -> &Win0CbrMst {
        &self.win0_cbr_mst
    }
    #[doc = "0x48 - Win0 active window width/height"]
    #[inline(always)]
    pub const fn win0_act_info(&self) -> &Win0ActInfo {
        &self.win0_act_info
    }
    #[doc = "0x4c - Win0 display width/height on panel"]
    #[inline(always)]
    pub const fn win0_dsp_info(&self) -> &Win0DspInfo {
        &self.win0_dsp_info
    }
    #[doc = "0x50 - Win0 display start point on panel"]
    #[inline(always)]
    pub const fn win0_dsp_st(&self) -> &Win0DspSt {
        &self.win0_dsp_st
    }
    #[doc = "0x54 - Win0 YRGB scaling factor"]
    #[inline(always)]
    pub const fn win0_scl_factor_yrgb(&self) -> &Win0SclFactorYrgb {
        &self.win0_scl_factor_yrgb
    }
    #[doc = "0x58 - Win0 Cbr scaling factor"]
    #[inline(always)]
    pub const fn win0_scl_factor_cbr(&self) -> &Win0SclFactorCbr {
        &self.win0_scl_factor_cbr
    }
    #[doc = "0x5c - Win0 scaling start point offset"]
    #[inline(always)]
    pub const fn win0_scl_offset(&self) -> &Win0SclOffset {
        &self.win0_scl_offset
    }
    #[doc = "0x60 - Win0 alpha source control register"]
    #[inline(always)]
    pub const fn win0_src_alpha_ctrl(&self) -> &Win0SrcAlphaCtrl {
        &self.win0_src_alpha_ctrl
    }
    #[doc = "0x64 - Win0 alpha destination control register"]
    #[inline(always)]
    pub const fn win0_dst_alpha_ctrl(&self) -> &Win0DstAlphaCtrl {
        &self.win0_dst_alpha_ctrl
    }
    #[doc = "0x68 - Win0 fading contrl register"]
    #[inline(always)]
    pub const fn win0_fading_ctrl(&self) -> &Win0FadingCtrl {
        &self.win0_fading_ctrl
    }
    #[doc = "0x6c - Win0 ctrl register2"]
    #[inline(always)]
    pub const fn win0_ctrl2(&self) -> &Win0Ctrl2 {
        &self.win0_ctrl2
    }
    #[doc = "0x70 - Win1 ctrl register0"]
    #[inline(always)]
    pub const fn win1_ctrl0(&self) -> &Win1Ctrl0 {
        &self.win1_ctrl0
    }
    #[doc = "0x74 - Win1 ctrl register1"]
    #[inline(always)]
    pub const fn win1_ctrl1(&self) -> &Win1Ctrl1 {
        &self.win1_ctrl1
    }
    #[doc = "0x78 - Win1 color key register"]
    #[inline(always)]
    pub const fn win1_color_key(&self) -> &Win1ColorKey {
        &self.win1_color_key
    }
    #[doc = "0x7c - win1 virtual stride"]
    #[inline(always)]
    pub const fn win1_vir(&self) -> &Win1Vir {
        &self.win1_vir
    }
    #[doc = "0x80 - Win1 YRGB memory start address"]
    #[inline(always)]
    pub const fn win1_yrgb_mst(&self) -> &Win1YrgbMst {
        &self.win1_yrgb_mst
    }
    #[doc = "0x84 - Win1 Cbr memory start address"]
    #[inline(always)]
    pub const fn win1_cbr_mst(&self) -> &Win1CbrMst {
        &self.win1_cbr_mst
    }
    #[doc = "0x88 - Win1 active window width/height"]
    #[inline(always)]
    pub const fn win1_act_info(&self) -> &Win1ActInfo {
        &self.win1_act_info
    }
    #[doc = "0x8c - Win1 display width/height on panel"]
    #[inline(always)]
    pub const fn win1_dsp_info(&self) -> &Win1DspInfo {
        &self.win1_dsp_info
    }
    #[doc = "0x90 - Win1 display start point on panel"]
    #[inline(always)]
    pub const fn win1_dsp_st(&self) -> &Win1DspSt {
        &self.win1_dsp_st
    }
    #[doc = "0x94 - Win1 YRGB scaling factor"]
    #[inline(always)]
    pub const fn win1_scl_factor_yrgb(&self) -> &Win1SclFactorYrgb {
        &self.win1_scl_factor_yrgb
    }
    #[doc = "0x98 - Win1 Cbr scaling factor"]
    #[inline(always)]
    pub const fn win1_scl_factor_cbr(&self) -> &Win1SclFactorCbr {
        &self.win1_scl_factor_cbr
    }
    #[doc = "0x9c - Win1 scaling start point offset"]
    #[inline(always)]
    pub const fn win1_scl_offset(&self) -> &Win1SclOffset {
        &self.win1_scl_offset
    }
    #[doc = "0xa0 - Win1 alpha source control register"]
    #[inline(always)]
    pub const fn win1_src_alpha_ctrl(&self) -> &Win1SrcAlphaCtrl {
        &self.win1_src_alpha_ctrl
    }
    #[doc = "0xa4 - Win1 alpha destination control register"]
    #[inline(always)]
    pub const fn win1_dst_alpha_ctrl(&self) -> &Win1DstAlphaCtrl {
        &self.win1_dst_alpha_ctrl
    }
    #[doc = "0xa8 - Win1 fading contrl register"]
    #[inline(always)]
    pub const fn win1_fading_ctrl(&self) -> &Win1FadingCtrl {
        &self.win1_fading_ctrl
    }
    #[doc = "0xac - Win1 ctrl register2"]
    #[inline(always)]
    pub const fn win1_ctrl2(&self) -> &Win1Ctrl2 {
        &self.win1_ctrl2
    }
    #[doc = "0xb0 - win2 ctrl register0"]
    #[inline(always)]
    pub const fn win2_ctrl0(&self) -> &Win2Ctrl0 {
        &self.win2_ctrl0
    }
    #[doc = "0xb4 - win2 ctrl register1"]
    #[inline(always)]
    pub const fn win2_ctrl1(&self) -> &Win2Ctrl1 {
        &self.win2_ctrl1
    }
    #[doc = "0xb8 - Win2 virtual stride0 and virtaul stride1"]
    #[inline(always)]
    pub const fn win2_vir0_1(&self) -> &Win2Vir0_1 {
        &self.win2_vir0_1
    }
    #[doc = "0xbc - Win2 virtual stride2 and virtaul stride3"]
    #[inline(always)]
    pub const fn win2_vir2_3(&self) -> &Win2Vir2_3 {
        &self.win2_vir2_3
    }
    #[doc = "0xc0 - Win2 memory start address0"]
    #[inline(always)]
    pub const fn win2_mst0(&self) -> &Win2Mst0 {
        &self.win2_mst0
    }
    #[doc = "0xc4 - Win2 display width0/height0 on panel"]
    #[inline(always)]
    pub const fn win2_dsp_info0(&self) -> &Win2DspInfo0 {
        &self.win2_dsp_info0
    }
    #[doc = "0xc8 - Win2 display start point0 on panel"]
    #[inline(always)]
    pub const fn win2_dsp_st0(&self) -> &Win2DspSt0 {
        &self.win2_dsp_st0
    }
    #[doc = "0xcc - Win2 color key register"]
    #[inline(always)]
    pub const fn win2_color_key(&self) -> &Win2ColorKey {
        &self.win2_color_key
    }
    #[doc = "0xd0 - Win2 memory start address1"]
    #[inline(always)]
    pub const fn win2_mst1(&self) -> &Win2Mst1 {
        &self.win2_mst1
    }
    #[doc = "0xd4 - Win2 display width1/height1 on panel"]
    #[inline(always)]
    pub const fn win2_dsp_info1(&self) -> &Win2DspInfo1 {
        &self.win2_dsp_info1
    }
    #[doc = "0xd8 - Win2 display start point1 on panel"]
    #[inline(always)]
    pub const fn win2_dsp_st1(&self) -> &Win2DspSt1 {
        &self.win2_dsp_st1
    }
    #[doc = "0xdc - Win2 alpha source control register"]
    #[inline(always)]
    pub const fn win2_src_alpha_ctrl(&self) -> &Win2SrcAlphaCtrl {
        &self.win2_src_alpha_ctrl
    }
    #[doc = "0xe0 - Win2 memory start address2"]
    #[inline(always)]
    pub const fn win2_mst2(&self) -> &Win2Mst2 {
        &self.win2_mst2
    }
    #[doc = "0xe4 - Win2 display width2/height2 on panel"]
    #[inline(always)]
    pub const fn win2_dsp_info2(&self) -> &Win2DspInfo2 {
        &self.win2_dsp_info2
    }
    #[doc = "0xe8 - Win2 display start point2 on panel"]
    #[inline(always)]
    pub const fn win2_dsp_st2(&self) -> &Win2DspSt2 {
        &self.win2_dsp_st2
    }
    #[doc = "0xec - Win2 alpha destination control register"]
    #[inline(always)]
    pub const fn win2_dst_alpha_ctrl(&self) -> &Win2DstAlphaCtrl {
        &self.win2_dst_alpha_ctrl
    }
    #[doc = "0xf0 - Win2 memory start address3"]
    #[inline(always)]
    pub const fn win2_mst3(&self) -> &Win2Mst3 {
        &self.win2_mst3
    }
    #[doc = "0xf4 - Win2 display width3/height3 on panel"]
    #[inline(always)]
    pub const fn win2_dsp_info3(&self) -> &Win2DspInfo3 {
        &self.win2_dsp_info3
    }
    #[doc = "0xf8 - Win2 display start point3 on panel"]
    #[inline(always)]
    pub const fn win2_dsp_st3(&self) -> &Win2DspSt3 {
        &self.win2_dsp_st3
    }
    #[doc = "0xfc - Win2 fading contrl register"]
    #[inline(always)]
    pub const fn win2_fading_ctrl(&self) -> &Win2FadingCtrl {
        &self.win2_fading_ctrl
    }
    #[doc = "0x100 - Win3 ctrl register0"]
    #[inline(always)]
    pub const fn win3_ctrl0(&self) -> &Win3Ctrl0 {
        &self.win3_ctrl0
    }
    #[doc = "0x104 - Win3 ctrl register1"]
    #[inline(always)]
    pub const fn win3_ctrl1(&self) -> &Win3Ctrl1 {
        &self.win3_ctrl1
    }
    #[doc = "0x108 - Win3 virtual stride0 and virtaul stride1"]
    #[inline(always)]
    pub const fn win3_vir0_1(&self) -> &Win3Vir0_1 {
        &self.win3_vir0_1
    }
    #[doc = "0x10c - Win3 virtual stride2 and virtaul stride3"]
    #[inline(always)]
    pub const fn win3_vir2_3(&self) -> &Win3Vir2_3 {
        &self.win3_vir2_3
    }
    #[doc = "0x110 - Win3 memory start address0"]
    #[inline(always)]
    pub const fn win3_mst0(&self) -> &Win3Mst0 {
        &self.win3_mst0
    }
    #[doc = "0x114 - Win3 display width0/height0 on panel"]
    #[inline(always)]
    pub const fn win3_dsp_info0(&self) -> &Win3DspInfo0 {
        &self.win3_dsp_info0
    }
    #[doc = "0x118 - Win3 display start point0 on panel"]
    #[inline(always)]
    pub const fn win3_dsp_st0(&self) -> &Win3DspSt0 {
        &self.win3_dsp_st0
    }
    #[doc = "0x11c - Win3 color key register"]
    #[inline(always)]
    pub const fn win3_color_key(&self) -> &Win3ColorKey {
        &self.win3_color_key
    }
    #[doc = "0x120 - Win3 memory start address1"]
    #[inline(always)]
    pub const fn win3_mst1(&self) -> &Win3Mst1 {
        &self.win3_mst1
    }
    #[doc = "0x124 - Win3 display width1/height1 on panel"]
    #[inline(always)]
    pub const fn win3_dsp_info1(&self) -> &Win3DspInfo1 {
        &self.win3_dsp_info1
    }
    #[doc = "0x128 - Win3 display start point1 on panel"]
    #[inline(always)]
    pub const fn win3_dsp_st1(&self) -> &Win3DspSt1 {
        &self.win3_dsp_st1
    }
    #[doc = "0x12c - Win3 alpha source control register"]
    #[inline(always)]
    pub const fn win3_src_alpha_ctrl(&self) -> &Win3SrcAlphaCtrl {
        &self.win3_src_alpha_ctrl
    }
    #[doc = "0x130 - Win3 memory start address2"]
    #[inline(always)]
    pub const fn win3_mst2(&self) -> &Win3Mst2 {
        &self.win3_mst2
    }
    #[doc = "0x134 - Win3 display width2/height2 on panel"]
    #[inline(always)]
    pub const fn win3_dsp_info2(&self) -> &Win3DspInfo2 {
        &self.win3_dsp_info2
    }
    #[doc = "0x138 - Win3 display start point2 on panel"]
    #[inline(always)]
    pub const fn win3_dsp_st2(&self) -> &Win3DspSt2 {
        &self.win3_dsp_st2
    }
    #[doc = "0x13c - Win3 alpha destination control register"]
    #[inline(always)]
    pub const fn win3_dst_alpha_ctrl(&self) -> &Win3DstAlphaCtrl {
        &self.win3_dst_alpha_ctrl
    }
    #[doc = "0x140 - Win3 memory start address3"]
    #[inline(always)]
    pub const fn win3_mst3(&self) -> &Win3Mst3 {
        &self.win3_mst3
    }
    #[doc = "0x144 - Win3 display width3/height3 on panel"]
    #[inline(always)]
    pub const fn win3_dsp_info3(&self) -> &Win3DspInfo3 {
        &self.win3_dsp_info3
    }
    #[doc = "0x148 - Win3 display start point3 on panel"]
    #[inline(always)]
    pub const fn win3_dsp_st3(&self) -> &Win3DspSt3 {
        &self.win3_dsp_st3
    }
    #[doc = "0x14c - Win3 fading contrl register"]
    #[inline(always)]
    pub const fn win3_fading_ctrl(&self) -> &Win3FadingCtrl {
        &self.win3_fading_ctrl
    }
    #[doc = "0x150 - Hwc ctrl register0"]
    #[inline(always)]
    pub const fn hwc_ctrl0(&self) -> &HwcCtrl0 {
        &self.hwc_ctrl0
    }
    #[doc = "0x154 - Hwc ctrl register1"]
    #[inline(always)]
    pub const fn hwc_ctrl1(&self) -> &HwcCtrl1 {
        &self.hwc_ctrl1
    }
    #[doc = "0x158 - Hwc memory start address"]
    #[inline(always)]
    pub const fn hwc_mst(&self) -> &HwcMst {
        &self.hwc_mst
    }
    #[doc = "0x15c - Hwc display start point on panel"]
    #[inline(always)]
    pub const fn hwc_dsp_st(&self) -> &HwcDspSt {
        &self.hwc_dsp_st
    }
    #[doc = "0x160 - Hwc alpha source control register"]
    #[inline(always)]
    pub const fn hwc_src_alpha_ctrl(&self) -> &HwcSrcAlphaCtrl {
        &self.hwc_src_alpha_ctrl
    }
    #[doc = "0x164 - Hwc alpha destination control register"]
    #[inline(always)]
    pub const fn hwc_dst_alpha_ctrl(&self) -> &HwcDstAlphaCtrl {
        &self.hwc_dst_alpha_ctrl
    }
    #[doc = "0x168 - Hwc fading contrl register"]
    #[inline(always)]
    pub const fn hwc_fading_ctrl(&self) -> &HwcFadingCtrl {
        &self.hwc_fading_ctrl
    }
    #[doc = "0x170 - Post scaler down horizontal start and end"]
    #[inline(always)]
    pub const fn post_dsp_hact_info(&self) -> &PostDspHactInfo {
        &self.post_dsp_hact_info
    }
    #[doc = "0x174 - Panel active horizontal scanning start point and end point"]
    #[inline(always)]
    pub const fn post_dsp_vact_info(&self) -> &PostDspVactInfo {
        &self.post_dsp_vact_info
    }
    #[doc = "0x178 - Post yrgb scaling factor"]
    #[inline(always)]
    pub const fn post_scl_factor_yrgb(&self) -> &PostSclFactorYrgb {
        &self.post_scl_factor_yrgb
    }
    #[doc = "0x17c - Post reserved"]
    #[inline(always)]
    pub const fn post_reserved(&self) -> &PostReserved {
        &self.post_reserved
    }
    #[doc = "0x180 - Post scaling start point offset"]
    #[inline(always)]
    pub const fn post_scl_ctrl(&self) -> &PostSclCtrl {
        &self.post_scl_ctrl
    }
    #[doc = "0x184 - Panel active horizontal scanning start point and end point F1"]
    #[inline(always)]
    pub const fn post_dsp_vact_info_f1(&self) -> &PostDspVactInfoF1 {
        &self.post_dsp_vact_info_f1
    }
    #[doc = "0x188 - Panel scanning horizontal width and hsync pulse end point"]
    #[inline(always)]
    pub const fn dsp_htotal_hs_end(&self) -> &DspHtotalHsEnd {
        &self.dsp_htotal_hs_end
    }
    #[doc = "0x18c - Panel active horizontal scanning start point and end point"]
    #[inline(always)]
    pub const fn dsp_hact_st_end(&self) -> &DspHactStEnd {
        &self.dsp_hact_st_end
    }
    #[doc = "0x190 - Panel scanning vertical height and vsync pulse end point"]
    #[inline(always)]
    pub const fn dsp_vtotal_vs_end(&self) -> &DspVtotalVsEnd {
        &self.dsp_vtotal_vs_end
    }
    #[doc = "0x194 - Panel active vertical scanning start point and end point"]
    #[inline(always)]
    pub const fn dsp_vact_st_end(&self) -> &DspVactStEnd {
        &self.dsp_vact_st_end
    }
    #[doc = "0x198 - Vertical scanning start point and vsync pulse end point of even filed in interlace mode"]
    #[inline(always)]
    pub const fn dsp_vs_st_end_f1(&self) -> &DspVsStEndF1 {
        &self.dsp_vs_st_end_f1
    }
    #[doc = "0x19c - Vertical scanning active start point and end point of even filed in interlace mode"]
    #[inline(always)]
    pub const fn dsp_vact_st_end_f1(&self) -> &DspVactStEndF1 {
        &self.dsp_vact_st_end_f1
    }
    #[doc = "0x1a0 - PWM Control Register"]
    #[inline(always)]
    pub const fn pwm_ctrl(&self) -> &PwmCtrl {
        &self.pwm_ctrl
    }
    #[doc = "0x1a4 - PWM Period Register/High Polarity Capture Register"]
    #[inline(always)]
    pub const fn pwm_period_hpr(&self) -> &PwmPeriodHpr {
        &self.pwm_period_hpr
    }
    #[doc = "0x1a8 - PWM Duty Register/Low Polarity Capture Register"]
    #[inline(always)]
    pub const fn pwm_duty_lpr(&self) -> &PwmDutyLpr {
        &self.pwm_duty_lpr
    }
    #[doc = "0x1ac - PWM Counter Register"]
    #[inline(always)]
    pub const fn pwm_cnt(&self) -> &PwmCnt {
        &self.pwm_cnt
    }
    #[doc = "0x1b0 - Color bar config register"]
    #[inline(always)]
    pub const fn bcsh_color_bar(&self) -> &BcshColorBar {
        &self.bcsh_color_bar
    }
    #[doc = "0x1b4 - Brightness contrast saturation*contrast config register"]
    #[inline(always)]
    pub const fn bcsh_bcs(&self) -> &BcshBcs {
        &self.bcsh_bcs
    }
    #[doc = "0x1b8 - Sin hue and cos hue config register"]
    #[inline(always)]
    pub const fn bcsh_h(&self) -> &BcshH {
        &self.bcsh_h
    }
    #[doc = "0x1bc - BCSH contrl register"]
    #[inline(always)]
    pub const fn bcsh_ctrl(&self) -> &BcshCtrl {
        &self.bcsh_ctrl
    }
    #[doc = "0x1c0 - Content Adaptive Backlight Control register0"]
    #[inline(always)]
    pub const fn cabc_ctrl0(&self) -> &CabcCtrl0 {
        &self.cabc_ctrl0
    }
    #[doc = "0x1c4 - Content Adaptive Backlight Control register1"]
    #[inline(always)]
    pub const fn cabc_ctrl1(&self) -> &CabcCtrl1 {
        &self.cabc_ctrl1
    }
    #[doc = "0x1c8 - Content Adaptive Backlight Control register2"]
    #[inline(always)]
    pub const fn cabc_ctrl2(&self) -> &CabcCtrl2 {
        &self.cabc_ctrl2
    }
    #[doc = "0x1cc - Content Adaptive Backlight Control register3"]
    #[inline(always)]
    pub const fn cabc_ctrl3(&self) -> &CabcCtrl3 {
        &self.cabc_ctrl3
    }
    #[doc = "0x1d0 - CABC gauss line config register00"]
    #[inline(always)]
    pub const fn cabc_gauss_line0_0(&self) -> &CabcGaussLine0_0 {
        &self.cabc_gauss_line0_0
    }
    #[doc = "0x1d4 - CABC gauss line config register01"]
    #[inline(always)]
    pub const fn cabc_gauss_line0_1(&self) -> &CabcGaussLine0_1 {
        &self.cabc_gauss_line0_1
    }
    #[doc = "0x1d8 - CABC gauss line config register10"]
    #[inline(always)]
    pub const fn cabc_gauss_line1_0(&self) -> &CabcGaussLine1_0 {
        &self.cabc_gauss_line1_0
    }
    #[doc = "0x1dc - CABC gauss line config register11"]
    #[inline(always)]
    pub const fn cabc_gauss_line1_1(&self) -> &CabcGaussLine1_1 {
        &self.cabc_gauss_line1_1
    }
    #[doc = "0x1e0 - CABC gauss line config register20"]
    #[inline(always)]
    pub const fn cabc_gauss_line2_0(&self) -> &CabcGaussLine2_0 {
        &self.cabc_gauss_line2_0
    }
    #[doc = "0x1e4 - CABC gauss line config register21"]
    #[inline(always)]
    pub const fn cabc_gauss_line2_1(&self) -> &CabcGaussLine2_1 {
        &self.cabc_gauss_line2_1
    }
    #[doc = "0x1e8 - FRC lookup table config register010"]
    #[inline(always)]
    pub const fn frc_lower01_0(&self) -> &FrcLower01_0 {
        &self.frc_lower01_0
    }
    #[doc = "0x1ec - FRC lookup table config register011"]
    #[inline(always)]
    pub const fn frc_lower01_1(&self) -> &FrcLower01_1 {
        &self.frc_lower01_1
    }
    #[doc = "0x1f0 - FRC lookup table config register100"]
    #[inline(always)]
    pub const fn frc_lower10_0(&self) -> &FrcLower10_0 {
        &self.frc_lower10_0
    }
    #[doc = "0x1f4 - FRC lookup table config register101"]
    #[inline(always)]
    pub const fn frc_lower10_1(&self) -> &FrcLower10_1 {
        &self.frc_lower10_1
    }
    #[doc = "0x1f8 - FRC lookup table config register110"]
    #[inline(always)]
    pub const fn frc_lower11_0(&self) -> &FrcLower11_0 {
        &self.frc_lower11_0
    }
    #[doc = "0x1fc - FRC lookup table config register111"]
    #[inline(always)]
    pub const fn frc_lower11_1(&self) -> &FrcLower11_1 {
        &self.frc_lower11_1
    }
    #[doc = "0x200 - AFBCD0 control register"]
    #[inline(always)]
    pub const fn afbcd0_ctrl(&self) -> &Afbcd0Ctrl {
        &self.afbcd0_ctrl
    }
    #[doc = "0x204 - AFBCD0 memory start address"]
    #[inline(always)]
    pub const fn afbcd0_hdr_ptr(&self) -> &Afbcd0HdrPtr {
        &self.afbcd0_hdr_ptr
    }
    #[doc = "0x208 - AFBCD0 pic size"]
    #[inline(always)]
    pub const fn afbcd0_pic_size(&self) -> &Afbcd0PicSize {
        &self.afbcd0_pic_size
    }
    #[doc = "0x20c - AFBCD0 status"]
    #[inline(always)]
    pub const fn afbcd0_status(&self) -> &Afbcd0Status {
        &self.afbcd0_status
    }
    #[doc = "0x280 - Interrupt enable register"]
    #[inline(always)]
    pub const fn intr_en0(&self) -> &IntrEn0 {
        &self.intr_en0
    }
    #[doc = "0x284 - Interrupt clear register"]
    #[inline(always)]
    pub const fn intr_clear0(&self) -> &IntrClear0 {
        &self.intr_clear0
    }
    #[doc = "0x288 - interrupt status"]
    #[inline(always)]
    pub const fn intr_status0(&self) -> &IntrStatus0 {
        &self.intr_status0
    }
    #[doc = "0x28c - raw interrupt status"]
    #[inline(always)]
    pub const fn intr_raw_status0(&self) -> &IntrRawStatus0 {
        &self.intr_raw_status0
    }
    #[doc = "0x290 - Interrupt enable register"]
    #[inline(always)]
    pub const fn intr_en1(&self) -> &IntrEn1 {
        &self.intr_en1
    }
    #[doc = "0x294 - Interrupt clear register"]
    #[inline(always)]
    pub const fn intr_clear1(&self) -> &IntrClear1 {
        &self.intr_clear1
    }
    #[doc = "0x298 - interrupt status"]
    #[inline(always)]
    pub const fn intr_status1(&self) -> &IntrStatus1 {
        &self.intr_status1
    }
    #[doc = "0x29c - raw interrupt status"]
    #[inline(always)]
    pub const fn intr_raw_status1(&self) -> &IntrRawStatus1 {
        &self.intr_raw_status1
    }
    #[doc = "0x2a0 - Line flag config register"]
    #[inline(always)]
    pub const fn line_flag(&self) -> &LineFlag {
        &self.line_flag
    }
    #[doc = "0x2a4 - vop status register"]
    #[inline(always)]
    pub const fn vop_status(&self) -> &VopStatus {
        &self.vop_status
    }
    #[doc = "0x2a8 - Register0000 Abstract"]
    #[inline(always)]
    pub const fn blanking_value(&self) -> &BlankingValue {
        &self.blanking_value
    }
    #[doc = "0x2ac - MCU bypass port"]
    #[inline(always)]
    pub const fn mcu_bypass_port(&self) -> &McuBypassPort {
        &self.mcu_bypass_port
    }
    #[doc = "0x2b0 - Win0 layer background color"]
    #[inline(always)]
    pub const fn win0_dsp_bg(&self) -> &Win0DspBg {
        &self.win0_dsp_bg
    }
    #[doc = "0x2b4 - Win1 layer background color"]
    #[inline(always)]
    pub const fn win1_dsp_bg(&self) -> &Win1DspBg {
        &self.win1_dsp_bg
    }
    #[doc = "0x2b8 - Win2 layer background color"]
    #[inline(always)]
    pub const fn win2_dsp_bg(&self) -> &Win2DspBg {
        &self.win2_dsp_bg
    }
    #[doc = "0x2bc - Win3 layer background color"]
    #[inline(always)]
    pub const fn win3_dsp_bg(&self) -> &Win3DspBg {
        &self.win3_dsp_bg
    }
    #[doc = "0x2c0 - win yuv2yuv control register"]
    #[inline(always)]
    pub const fn yuv2yuv_win(&self) -> &Yuv2yuvWin {
        &self.yuv2yuv_win
    }
    #[doc = "0x2cc - Auto gating enable"]
    #[inline(always)]
    pub const fn auto_gating_en(&self) -> &AutoGatingEn {
        &self.auto_gating_en
    }
    #[doc = "0x4e0 - WIN0 yuv2yuv y2r cofficient"]
    #[inline(always)]
    pub const fn win0_yuv2yuv_y2r_coe0(&self) -> &Win0Yuv2yuvY2rCoe0 {
        &self.win0_yuv2yuv_y2r_coe0
    }
    #[doc = "0x4e4 - WIN0 yuv2yuv y2r cofficient"]
    #[inline(always)]
    pub const fn win0_yuv2yuv_y2r_coe1(&self) -> &Win0Yuv2yuvY2rCoe1 {
        &self.win0_yuv2yuv_y2r_coe1
    }
    #[doc = "0x4e8 - WIN0 yuv2yuv y2r cofficient"]
    #[inline(always)]
    pub const fn win0_yuv2yuv_y2r_coe2(&self) -> &Win0Yuv2yuvY2rCoe2 {
        &self.win0_yuv2yuv_y2r_coe2
    }
    #[doc = "0x4ec - WIN0yuv2yuv y2r cofficient"]
    #[inline(always)]
    pub const fn win0_yuv2yuv_y2r_coe3(&self) -> &Win0Yuv2yuvY2rCoe3 {
        &self.win0_yuv2yuv_y2r_coe3
    }
    #[doc = "0x4f0 - WIN0 yuv2yuv y2r cofficient"]
    #[inline(always)]
    pub const fn win0_yuv2yuv_y2r_coe4(&self) -> &Win0Yuv2yuvY2rCoe4 {
        &self.win0_yuv2yuv_y2r_coe4
    }
    #[doc = "0x4f4 - WIN0 yuv2yuv y2r cofficient"]
    #[inline(always)]
    pub const fn win0_yuv2yuv_y2r_coe5(&self) -> &Win0Yuv2yuvY2rCoe5 {
        &self.win0_yuv2yuv_y2r_coe5
    }
    #[doc = "0x4f8 - WIN0 yuv2yuv y2r cofficient"]
    #[inline(always)]
    pub const fn win0_yuv2yuv_y2r_coe6(&self) -> &Win0Yuv2yuvY2rCoe6 {
        &self.win0_yuv2yuv_y2r_coe6
    }
    #[doc = "0x4fc - WIN0 yuv2yuv y2r cofficient"]
    #[inline(always)]
    pub const fn win0_yuv2yuv_y2r_coe7(&self) -> &Win0Yuv2yuvY2rCoe7 {
        &self.win0_yuv2yuv_y2r_coe7
    }
    #[doc = "0x500 - WIN0 yuv2yuv r2r cofficient"]
    #[inline(always)]
    pub const fn win0_yuv2yuv_r2r_coe0(&self) -> &Win0Yuv2yuvR2rCoe0 {
        &self.win0_yuv2yuv_r2r_coe0
    }
    #[doc = "0x504 - WIN0 yuv2yuv r2r cofficient"]
    #[inline(always)]
    pub const fn win0_yuv2yuv_r2r_coe1(&self) -> &Win0Yuv2yuvR2rCoe1 {
        &self.win0_yuv2yuv_r2r_coe1
    }
    #[doc = "0x508 - WIN0 yuv2yuv r2r cofficient"]
    #[inline(always)]
    pub const fn win0_yuv2yuv_r2r_coe2(&self) -> &Win0Yuv2yuvR2rCoe2 {
        &self.win0_yuv2yuv_r2r_coe2
    }
    #[doc = "0x50c - WIN0 yuv2yuv r2r cofficient"]
    #[inline(always)]
    pub const fn win0_yuv2yuv_r2r_coe3(&self) -> &Win0Yuv2yuvR2rCoe3 {
        &self.win0_yuv2yuv_r2r_coe3
    }
    #[doc = "0x510 - WIN0 yuv2yuv r2r cofficient"]
    #[inline(always)]
    pub const fn win0_yuv2yuv_r2r_coe4(&self) -> &Win0Yuv2yuvR2rCoe4 {
        &self.win0_yuv2yuv_r2r_coe4
    }
    #[doc = "0x514 - WIN0 yuv2yuv r2r cofficient"]
    #[inline(always)]
    pub const fn win0_yuv2yuv_r2r_coe5(&self) -> &Win0Yuv2yuvR2rCoe5 {
        &self.win0_yuv2yuv_r2r_coe5
    }
    #[doc = "0x518 - WIN0 yuv2yuv r2r cofficient"]
    #[inline(always)]
    pub const fn win0_yuv2yuv_r2r_coe6(&self) -> &Win0Yuv2yuvR2rCoe6 {
        &self.win0_yuv2yuv_r2r_coe6
    }
    #[doc = "0x51c - WIN0 yuv2yuv r2r cofficient"]
    #[inline(always)]
    pub const fn win0_yuv2yuv_r2r_coe7(&self) -> &Win0Yuv2yuvR2rCoe7 {
        &self.win0_yuv2yuv_r2r_coe7
    }
    #[doc = "0x520 - WIN0 yuv2yuv r2y cofficient"]
    #[inline(always)]
    pub const fn win0_yuv2yuv_r2y_coe0(&self) -> &Win0Yuv2yuvR2yCoe0 {
        &self.win0_yuv2yuv_r2y_coe0
    }
    #[doc = "0x524 - WIN0 yuv2yuv r2y cofficient"]
    #[inline(always)]
    pub const fn win0_yuv2yuv_r2y_coe1(&self) -> &Win0Yuv2yuvR2yCoe1 {
        &self.win0_yuv2yuv_r2y_coe1
    }
    #[doc = "0x528 - WIN0 yuv2yuv r2y cofficient"]
    #[inline(always)]
    pub const fn win0_yuv2yuv_r2y_coe2(&self) -> &Win0Yuv2yuvR2yCoe2 {
        &self.win0_yuv2yuv_r2y_coe2
    }
    #[doc = "0x52c - WIN0 yuv2yuv r2y cofficient"]
    #[inline(always)]
    pub const fn win0_yuv2yuv_r2y_coe3(&self) -> &Win0Yuv2yuvR2yCoe3 {
        &self.win0_yuv2yuv_r2y_coe3
    }
    #[doc = "0x530 - WIN0 yuv2yuv r2y cofficient"]
    #[inline(always)]
    pub const fn win0_yuv2yuv_r2y_coe4(&self) -> &Win0Yuv2yuvR2yCoe4 {
        &self.win0_yuv2yuv_r2y_coe4
    }
    #[doc = "0x534 - WIN0 yuv2yuv r2y cofficient"]
    #[inline(always)]
    pub const fn win0_yuv2yuv_r2y_coe5(&self) -> &Win0Yuv2yuvR2yCoe5 {
        &self.win0_yuv2yuv_r2y_coe5
    }
    #[doc = "0x538 - WIN0 yuv2yuv r2y cofficient"]
    #[inline(always)]
    pub const fn win0_yuv2yuv_r2y_coe6(&self) -> &Win0Yuv2yuvR2yCoe6 {
        &self.win0_yuv2yuv_r2y_coe6
    }
    #[doc = "0x53c - WIN0 yuv2yuv r2y cofficient"]
    #[inline(always)]
    pub const fn win0_yuv2yuv_r2y_coe7(&self) -> &Win0Yuv2yuvR2yCoe7 {
        &self.win0_yuv2yuv_r2y_coe7
    }
    #[doc = "0x540 - WIN1 yuv2yuv y2r cofficient"]
    #[inline(always)]
    pub const fn win1_yuv2yuv_y2r_coe0(&self) -> &Win1Yuv2yuvY2rCoe0 {
        &self.win1_yuv2yuv_y2r_coe0
    }
    #[doc = "0x544 - WIN1 yuv2yuv y2r cofficient"]
    #[inline(always)]
    pub const fn win1_yuv2yuv_y2r_coe1(&self) -> &Win1Yuv2yuvY2rCoe1 {
        &self.win1_yuv2yuv_y2r_coe1
    }
    #[doc = "0x548 - WIN1 yuv2yuv y2r cofficient"]
    #[inline(always)]
    pub const fn win1_yuv2yuv_y2r_coe2(&self) -> &Win1Yuv2yuvY2rCoe2 {
        &self.win1_yuv2yuv_y2r_coe2
    }
    #[doc = "0x54c - WIN1 yuv2yuv y2r cofficient"]
    #[inline(always)]
    pub const fn win1_yuv2yuv_y2r_coe3(&self) -> &Win1Yuv2yuvY2rCoe3 {
        &self.win1_yuv2yuv_y2r_coe3
    }
    #[doc = "0x550 - WIN1 yuv2yuv y2r cofficient"]
    #[inline(always)]
    pub const fn win1_yuv2yuv_y2r_coe4(&self) -> &Win1Yuv2yuvY2rCoe4 {
        &self.win1_yuv2yuv_y2r_coe4
    }
    #[doc = "0x554 - WIN1 yuv2yuv y2r cofficient"]
    #[inline(always)]
    pub const fn win1_yuv2yuv_y2r_coe5(&self) -> &Win1Yuv2yuvY2rCoe5 {
        &self.win1_yuv2yuv_y2r_coe5
    }
    #[doc = "0x558 - WIN1 yuv2yuv y2r cofficient"]
    #[inline(always)]
    pub const fn win1_yuv2yuv_y2r_coe6(&self) -> &Win1Yuv2yuvY2rCoe6 {
        &self.win1_yuv2yuv_y2r_coe6
    }
    #[doc = "0x55c - WIN1 yuv2yuv y2r cofficient"]
    #[inline(always)]
    pub const fn win1_yuv2yuv_y2r_coe7(&self) -> &Win1Yuv2yuvY2rCoe7 {
        &self.win1_yuv2yuv_y2r_coe7
    }
    #[doc = "0x560 - WIN0 yuv2yuv r2r cofficient"]
    #[inline(always)]
    pub const fn win1_yuv2yuv_r2r_coe0(&self) -> &Win1Yuv2yuvR2rCoe0 {
        &self.win1_yuv2yuv_r2r_coe0
    }
    #[doc = "0x564 - WIN1 yuv2yuv r2r cofficient"]
    #[inline(always)]
    pub const fn win1_yuv2yuv_r2r_coe1(&self) -> &Win1Yuv2yuvR2rCoe1 {
        &self.win1_yuv2yuv_r2r_coe1
    }
    #[doc = "0x568 - WIN1 yuv2yuv r2r cofficient"]
    #[inline(always)]
    pub const fn win1_yuv2yuv_r2r_coe2(&self) -> &Win1Yuv2yuvR2rCoe2 {
        &self.win1_yuv2yuv_r2r_coe2
    }
    #[doc = "0x56c - WIN1 yuv2yuv r2r cofficient"]
    #[inline(always)]
    pub const fn win1_yuv2yuv_r2r_coe3(&self) -> &Win1Yuv2yuvR2rCoe3 {
        &self.win1_yuv2yuv_r2r_coe3
    }
    #[doc = "0x570 - WIN1 yuv2yuv r2r cofficient"]
    #[inline(always)]
    pub const fn win1_yuv2yuv_r2r_coe4(&self) -> &Win1Yuv2yuvR2rCoe4 {
        &self.win1_yuv2yuv_r2r_coe4
    }
    #[doc = "0x574 - WIN1 yuv2yuv r2r cofficient"]
    #[inline(always)]
    pub const fn win1_yuv2yuv_r2r_coe5(&self) -> &Win1Yuv2yuvR2rCoe5 {
        &self.win1_yuv2yuv_r2r_coe5
    }
    #[doc = "0x578 - WIN1 yuv2yuv r2r cofficient"]
    #[inline(always)]
    pub const fn win1_yuv2yuv_r2r_coe6(&self) -> &Win1Yuv2yuvR2rCoe6 {
        &self.win1_yuv2yuv_r2r_coe6
    }
    #[doc = "0x57c - WIN1 yuv2yuv r2r cofficient"]
    #[inline(always)]
    pub const fn win1_yuv2yuv_r2r_coe7(&self) -> &Win1Yuv2yuvR2rCoe7 {
        &self.win1_yuv2yuv_r2r_coe7
    }
    #[doc = "0x580 - WIN1 yuv2yuv r2y cofficient"]
    #[inline(always)]
    pub const fn win1_yuv2yuv_r2y_coe0(&self) -> &Win1Yuv2yuvR2yCoe0 {
        &self.win1_yuv2yuv_r2y_coe0
    }
    #[doc = "0x584 - WIN1 yuv2yuv r2y cofficient"]
    #[inline(always)]
    pub const fn win1_yuv2yuv_r2y_coe1(&self) -> &Win1Yuv2yuvR2yCoe1 {
        &self.win1_yuv2yuv_r2y_coe1
    }
    #[doc = "0x588 - WIN1 yuv2yuv r2y cofficient"]
    #[inline(always)]
    pub const fn win1_yuv2yuv_r2y_coe2(&self) -> &Win1Yuv2yuvR2yCoe2 {
        &self.win1_yuv2yuv_r2y_coe2
    }
    #[doc = "0x58c - WIN1 yuv2yuv r2y cofficient"]
    #[inline(always)]
    pub const fn win1_yuv2yuv_r2y_coe3(&self) -> &Win1Yuv2yuvR2yCoe3 {
        &self.win1_yuv2yuv_r2y_coe3
    }
    #[doc = "0x590 - WIN1 yuv2yuv r2y cofficient"]
    #[inline(always)]
    pub const fn win1_yuv2yuv_r2y_coe4(&self) -> &Win1Yuv2yuvR2yCoe4 {
        &self.win1_yuv2yuv_r2y_coe4
    }
    #[doc = "0x594 - WIN1 yuv2yuv r2y cofficient"]
    #[inline(always)]
    pub const fn win1_yuv2yuv_r2y_coe5(&self) -> &Win1Yuv2yuvR2yCoe5 {
        &self.win1_yuv2yuv_r2y_coe5
    }
    #[doc = "0x598 - WIN1 yuv2yuv r2y cofficient"]
    #[inline(always)]
    pub const fn win1_yuv2yuv_r2y_coe6(&self) -> &Win1Yuv2yuvR2yCoe6 {
        &self.win1_yuv2yuv_r2y_coe6
    }
    #[doc = "0x59c - WIN1 yuv2yuv r2y cofficient"]
    #[inline(always)]
    pub const fn win1_yuv2yuv_r2y_coe7(&self) -> &Win1Yuv2yuvR2yCoe7 {
        &self.win1_yuv2yuv_r2y_coe7
    }
    #[doc = "0x5a0 - WIN2 yuv2yuv y2r cofficient"]
    #[inline(always)]
    pub const fn win2_yuv2yuv_y2r_coe0(&self) -> &Win2Yuv2yuvY2rCoe0 {
        &self.win2_yuv2yuv_y2r_coe0
    }
    #[doc = "0x5a4 - WIN2 yuv2yuv y2r cofficient"]
    #[inline(always)]
    pub const fn win2_yuv2yuv_y2r_coe1(&self) -> &Win2Yuv2yuvY2rCoe1 {
        &self.win2_yuv2yuv_y2r_coe1
    }
    #[doc = "0x5a8 - WIN2 yuv2yuv y2r cofficient"]
    #[inline(always)]
    pub const fn win2_yuv2yuv_y2r_coe2(&self) -> &Win2Yuv2yuvY2rCoe2 {
        &self.win2_yuv2yuv_y2r_coe2
    }
    #[doc = "0x5ac - WIN2 yuv2yuv y2r cofficient"]
    #[inline(always)]
    pub const fn win2_yuv2yuv_y2r_coe3(&self) -> &Win2Yuv2yuvY2rCoe3 {
        &self.win2_yuv2yuv_y2r_coe3
    }
    #[doc = "0x5b0 - WIN2 yuv2yuv y2r cofficient"]
    #[inline(always)]
    pub const fn win2_yuv2yuv_y2r_coe4(&self) -> &Win2Yuv2yuvY2rCoe4 {
        &self.win2_yuv2yuv_y2r_coe4
    }
    #[doc = "0x5b4 - WIN2 yuv2yuv y2r cofficient"]
    #[inline(always)]
    pub const fn win2_yuv2yuv_y2r_coe5(&self) -> &Win2Yuv2yuvY2rCoe5 {
        &self.win2_yuv2yuv_y2r_coe5
    }
    #[doc = "0x5b8 - WIN2 yuv2yuv y2r cofficient"]
    #[inline(always)]
    pub const fn win2_yuv2yuv_y2r_coe6(&self) -> &Win2Yuv2yuvY2rCoe6 {
        &self.win2_yuv2yuv_y2r_coe6
    }
    #[doc = "0x5bc - WIN2 yuv2yuv y2r cofficient"]
    #[inline(always)]
    pub const fn win2_yuv2yuv_y2r_coe7(&self) -> &Win2Yuv2yuvY2rCoe7 {
        &self.win2_yuv2yuv_y2r_coe7
    }
    #[doc = "0x5c0 - WIN2 yuv2yuv r2r cofficient"]
    #[inline(always)]
    pub const fn win2_yuv2yuv_r2r_coe0(&self) -> &Win2Yuv2yuvR2rCoe0 {
        &self.win2_yuv2yuv_r2r_coe0
    }
    #[doc = "0x5c4 - WIN2 yuv2yuv r2r cofficient"]
    #[inline(always)]
    pub const fn win2_yuv2yuv_r2r_coe1(&self) -> &Win2Yuv2yuvR2rCoe1 {
        &self.win2_yuv2yuv_r2r_coe1
    }
    #[doc = "0x5c8 - WIN2 yuv2yuv r2r cofficient"]
    #[inline(always)]
    pub const fn win2_yuv2yuv_r2r_coe2(&self) -> &Win2Yuv2yuvR2rCoe2 {
        &self.win2_yuv2yuv_r2r_coe2
    }
    #[doc = "0x5cc - WIN2 yuv2yuv r2r cofficient"]
    #[inline(always)]
    pub const fn win2_yuv2yuv_r2r_coe3(&self) -> &Win2Yuv2yuvR2rCoe3 {
        &self.win2_yuv2yuv_r2r_coe3
    }
    #[doc = "0x5d0 - WIN2 yuv2yuv r2r cofficient"]
    #[inline(always)]
    pub const fn win2_yuv2yuv_r2r_coe4(&self) -> &Win2Yuv2yuvR2rCoe4 {
        &self.win2_yuv2yuv_r2r_coe4
    }
    #[doc = "0x5d4 - WIN2 yuv2yuv r2r cofficient"]
    #[inline(always)]
    pub const fn win2_yuv2yuv_r2r_coe5(&self) -> &Win2Yuv2yuvR2rCoe5 {
        &self.win2_yuv2yuv_r2r_coe5
    }
    #[doc = "0x5d8 - WIN2 yuv2yuv r2r cofficient"]
    #[inline(always)]
    pub const fn win2_yuv2yuv_r2r_coe6(&self) -> &Win2Yuv2yuvR2rCoe6 {
        &self.win2_yuv2yuv_r2r_coe6
    }
    #[doc = "0x5dc - WIN2 yuv2yuv r2r cofficient"]
    #[inline(always)]
    pub const fn win2_yuv2yuv_r2r_coe7(&self) -> &Win2Yuv2yuvR2rCoe7 {
        &self.win2_yuv2yuv_r2r_coe7
    }
    #[doc = "0x5e0 - WIN2 yuv2yuv r2y cofficient"]
    #[inline(always)]
    pub const fn win2_yuv2yuv_r2y_coe0(&self) -> &Win2Yuv2yuvR2yCoe0 {
        &self.win2_yuv2yuv_r2y_coe0
    }
    #[doc = "0x5e4 - WIN2 yuv2yuv r2y cofficient"]
    #[inline(always)]
    pub const fn win2_yuv2yuv_r2y_coe1(&self) -> &Win2Yuv2yuvR2yCoe1 {
        &self.win2_yuv2yuv_r2y_coe1
    }
    #[doc = "0x5e8 - WIN2 yuv2yuv r2y cofficient"]
    #[inline(always)]
    pub const fn win2_yuv2yuv_r2y_coe2(&self) -> &Win2Yuv2yuvR2yCoe2 {
        &self.win2_yuv2yuv_r2y_coe2
    }
    #[doc = "0x5ec - WIN2 yuv2yuv r2y cofficient"]
    #[inline(always)]
    pub const fn win2_yuv2yuv_r2y_coe3(&self) -> &Win2Yuv2yuvR2yCoe3 {
        &self.win2_yuv2yuv_r2y_coe3
    }
    #[doc = "0x5f0 - WIN2 yuv2yuv r2y cofficient"]
    #[inline(always)]
    pub const fn win2_yuv2yuv_r2y_coe4(&self) -> &Win2Yuv2yuvR2yCoe4 {
        &self.win2_yuv2yuv_r2y_coe4
    }
    #[doc = "0x5f4 - WIN2 yuv2yuv r2y cofficient"]
    #[inline(always)]
    pub const fn win2_yuv2yuv_r2y_coe5(&self) -> &Win2Yuv2yuvR2yCoe5 {
        &self.win2_yuv2yuv_r2y_coe5
    }
    #[doc = "0x5f8 - WIN2 yuv2yuv r2y cofficient"]
    #[inline(always)]
    pub const fn win2_yuv2yuv_r2y_coe6(&self) -> &Win2Yuv2yuvR2yCoe6 {
        &self.win2_yuv2yuv_r2y_coe6
    }
    #[doc = "0x5fc - WIN2 yuv2yuv r2y cofficient"]
    #[inline(always)]
    pub const fn win2_yuv2yuv_r2y_coe7(&self) -> &Win2Yuv2yuvR2yCoe7 {
        &self.win2_yuv2yuv_r2y_coe7
    }
    #[doc = "0x600 - WIN3 yuv2yuv y2r cofficient"]
    #[inline(always)]
    pub const fn win3_yuv2yuv_y2r_coe0(&self) -> &Win3Yuv2yuvY2rCoe0 {
        &self.win3_yuv2yuv_y2r_coe0
    }
    #[doc = "0x604 - WIN3 yuv2yuv y2r cofficient"]
    #[inline(always)]
    pub const fn win3_yuv2yuv_y2r_coe1(&self) -> &Win3Yuv2yuvY2rCoe1 {
        &self.win3_yuv2yuv_y2r_coe1
    }
    #[doc = "0x608 - WIN3 yuv2yuv y2r cofficient"]
    #[inline(always)]
    pub const fn win3_yuv2yuv_y2r_coe2(&self) -> &Win3Yuv2yuvY2rCoe2 {
        &self.win3_yuv2yuv_y2r_coe2
    }
    #[doc = "0x60c - WIN3 yuv2yuv y2r cofficient"]
    #[inline(always)]
    pub const fn win3_yuv2yuv_y2r_coe3(&self) -> &Win3Yuv2yuvY2rCoe3 {
        &self.win3_yuv2yuv_y2r_coe3
    }
    #[doc = "0x610 - WIN3 yuv2yuv y2r cofficient"]
    #[inline(always)]
    pub const fn win3_yuv2yuv_y2r_coe4(&self) -> &Win3Yuv2yuvY2rCoe4 {
        &self.win3_yuv2yuv_y2r_coe4
    }
    #[doc = "0x614 - WIN3 yuv2yuv y2r cofficient"]
    #[inline(always)]
    pub const fn win3_yuv2yuv_y2r_coe5(&self) -> &Win3Yuv2yuvY2rCoe5 {
        &self.win3_yuv2yuv_y2r_coe5
    }
    #[doc = "0x618 - WIN3 yuv2yuv y2r cofficient"]
    #[inline(always)]
    pub const fn win3_yuv2yuv_y2r_coe6(&self) -> &Win3Yuv2yuvY2rCoe6 {
        &self.win3_yuv2yuv_y2r_coe6
    }
    #[doc = "0x61c - WIN3 yuv2yuv y2r cofficient"]
    #[inline(always)]
    pub const fn win3_yuv2yuv_y2r_coe7(&self) -> &Win3Yuv2yuvY2rCoe7 {
        &self.win3_yuv2yuv_y2r_coe7
    }
    #[doc = "0x620 - WIN3 yuv2yuv r2r cofficient"]
    #[inline(always)]
    pub const fn win3_yuv2yuv_r2r_coe0(&self) -> &Win3Yuv2yuvR2rCoe0 {
        &self.win3_yuv2yuv_r2r_coe0
    }
    #[doc = "0x624 - WIN3 yuv2yuv r2r cofficient"]
    #[inline(always)]
    pub const fn win3_yuv2yuv_r2r_coe1(&self) -> &Win3Yuv2yuvR2rCoe1 {
        &self.win3_yuv2yuv_r2r_coe1
    }
    #[doc = "0x628 - WIN3 yuv2yuv r2r cofficient"]
    #[inline(always)]
    pub const fn win3_yuv2yuv_r2r_coe2(&self) -> &Win3Yuv2yuvR2rCoe2 {
        &self.win3_yuv2yuv_r2r_coe2
    }
    #[doc = "0x62c - WIN3 yuv2yuv r2r cofficient"]
    #[inline(always)]
    pub const fn win3_yuv2yuv_r2r_coe3(&self) -> &Win3Yuv2yuvR2rCoe3 {
        &self.win3_yuv2yuv_r2r_coe3
    }
    #[doc = "0x630 - WIN3 yuv2yuv r2r cofficient"]
    #[inline(always)]
    pub const fn win3_yuv2yuv_r2r_coe4(&self) -> &Win3Yuv2yuvR2rCoe4 {
        &self.win3_yuv2yuv_r2r_coe4
    }
    #[doc = "0x634 - WIN3 yuv2yuv r2r cofficient"]
    #[inline(always)]
    pub const fn win3_yuv2yuv_r2r_coe5(&self) -> &Win3Yuv2yuvR2rCoe5 {
        &self.win3_yuv2yuv_r2r_coe5
    }
    #[doc = "0x638 - WIN3 yuv2yuv r2r cofficient"]
    #[inline(always)]
    pub const fn win3_yuv2yuv_r2r_coe6(&self) -> &Win3Yuv2yuvR2rCoe6 {
        &self.win3_yuv2yuv_r2r_coe6
    }
    #[doc = "0x63c - WIN3 yuv2yuv r2r cofficient"]
    #[inline(always)]
    pub const fn win3_yuv2yuv_r2r_coe7(&self) -> &Win3Yuv2yuvR2rCoe7 {
        &self.win3_yuv2yuv_r2r_coe7
    }
    #[doc = "0x640 - WIN3 yuv2yuv r2y cofficient"]
    #[inline(always)]
    pub const fn win3_yuv2yuv_r2y_coe0(&self) -> &Win3Yuv2yuvR2yCoe0 {
        &self.win3_yuv2yuv_r2y_coe0
    }
    #[doc = "0x644 - WIN3 yuv2yuv r2y cofficient"]
    #[inline(always)]
    pub const fn win3_yuv2yuv_r2y_coe1(&self) -> &Win3Yuv2yuvR2yCoe1 {
        &self.win3_yuv2yuv_r2y_coe1
    }
    #[doc = "0x648 - WIN3 yuv2yuv r2y cofficient"]
    #[inline(always)]
    pub const fn win3_yuv2yuv_r2y_coe2(&self) -> &Win3Yuv2yuvR2yCoe2 {
        &self.win3_yuv2yuv_r2y_coe2
    }
    #[doc = "0x64c - WIN3 yuv2yuv cofficient"]
    #[inline(always)]
    pub const fn win3_yuv2yuv_r2y_coe3(&self) -> &Win3Yuv2yuvR2yCoe3 {
        &self.win3_yuv2yuv_r2y_coe3
    }
    #[doc = "0x650 - WIN3 yuv2yuv r2y cofficient"]
    #[inline(always)]
    pub const fn win3_yuv2yuv_r2y_coe4(&self) -> &Win3Yuv2yuvR2yCoe4 {
        &self.win3_yuv2yuv_r2y_coe4
    }
    #[doc = "0x654 - WIN3 yuv2yuv r2y cofficient"]
    #[inline(always)]
    pub const fn win3_yuv2yuv_r2y_coe5(&self) -> &Win3Yuv2yuvR2yCoe5 {
        &self.win3_yuv2yuv_r2y_coe5
    }
    #[doc = "0x658 - WIN3 yuv2yuv r2y cofficient"]
    #[inline(always)]
    pub const fn win3_yuv2yuv_r2y_coe6(&self) -> &Win3Yuv2yuvR2yCoe6 {
        &self.win3_yuv2yuv_r2y_coe6
    }
    #[doc = "0x65c - WIN3 yuv2yuv r2y cofficient"]
    #[inline(always)]
    pub const fn win3_yuv2yuv_r2y_coe7(&self) -> &Win3Yuv2yuvR2yCoe7 {
        &self.win3_yuv2yuv_r2y_coe7
    }
    #[doc = "0x1000 - Win2 lut base address"]
    #[inline(always)]
    pub const fn win2_lut_addr(&self) -> &Win2LutAddr {
        &self.win2_lut_addr
    }
    #[doc = "0x1400 - Win3 lut base address"]
    #[inline(always)]
    pub const fn win3_lut_addr(&self) -> &Win3LutAddr {
        &self.win3_lut_addr
    }
    #[doc = "0x1800 - Hwc lut base address"]
    #[inline(always)]
    pub const fn hwc_lut_addr(&self) -> &HwcLutAddr {
        &self.hwc_lut_addr
    }
    #[doc = "0x1c00 - CABC GAMMA lut base address"]
    #[inline(always)]
    pub const fn cabc_gamma_lut_addr(&self) -> &CabcGammaLutAddr {
        &self.cabc_gamma_lut_addr
    }
    #[doc = "0x2000 - GAMMA lut base address"]
    #[inline(always)]
    pub const fn gamma_lut_addr(&self) -> &GammaLutAddr {
        &self.gamma_lut_addr
    }
}
#[doc = "REG_CFG_DONE (rw) register accessor: Register config done flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_cfg_done::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_cfg_done::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_cfg_done`]
module"]
#[doc(alias = "REG_CFG_DONE")]
pub type RegCfgDone = crate::Reg<reg_cfg_done::RegCfgDoneSpec>;
#[doc = "Register config done flag"]
pub mod reg_cfg_done;
#[doc = "VERSION_INFO (r) register accessor: Version for vop\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version_info::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@version_info`]
module"]
#[doc(alias = "VERSION_INFO")]
pub type VersionInfo = crate::Reg<version_info::VersionInfoSpec>;
#[doc = "Version for vop"]
pub mod version_info;
#[doc = "SYS_CTRL (rw) register accessor: System control register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_ctrl`]
module"]
#[doc(alias = "SYS_CTRL")]
pub type SysCtrl = crate::Reg<sys_ctrl::SysCtrlSpec>;
#[doc = "System control register0"]
pub mod sys_ctrl;
#[doc = "SYS_CTRL1 (rw) register accessor: System control register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_ctrl1`]
module"]
#[doc(alias = "SYS_CTRL1")]
pub type SysCtrl1 = crate::Reg<sys_ctrl1::SysCtrl1Spec>;
#[doc = "System control register1"]
pub mod sys_ctrl1;
#[doc = "DSP_CTRL0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_ctrl0`]
module"]
#[doc(alias = "DSP_CTRL0")]
pub type DspCtrl0 = crate::Reg<dsp_ctrl0::DspCtrl0Spec>;
#[doc = ""]
pub mod dsp_ctrl0;
#[doc = "DSP_CTRL1 (rw) register accessor: Display control register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_ctrl1`]
module"]
#[doc(alias = "DSP_CTRL1")]
pub type DspCtrl1 = crate::Reg<dsp_ctrl1::DspCtrl1Spec>;
#[doc = "Display control register1"]
pub mod dsp_ctrl1;
#[doc = "DSP_BG (rw) register accessor: Background color\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_bg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_bg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_bg`]
module"]
#[doc(alias = "DSP_BG")]
pub type DspBg = crate::Reg<dsp_bg::DspBgSpec>;
#[doc = "Background color"]
pub mod dsp_bg;
#[doc = "MCU_CTRL (rw) register accessor: MCU mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcu_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcu_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcu_ctrl`]
module"]
#[doc(alias = "MCU_CTRL")]
pub type McuCtrl = crate::Reg<mcu_ctrl::McuCtrlSpec>;
#[doc = "MCU mode control register"]
pub mod mcu_ctrl;
#[doc = "WB_CTRL0 (rw) register accessor: write back ctrl0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wb_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wb_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wb_ctrl0`]
module"]
#[doc(alias = "WB_CTRL0")]
pub type WbCtrl0 = crate::Reg<wb_ctrl0::WbCtrl0Spec>;
#[doc = "write back ctrl0"]
pub mod wb_ctrl0;
#[doc = "WB_CTRL1 (rw) register accessor: write back ctrl1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wb_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wb_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wb_ctrl1`]
module"]
#[doc(alias = "WB_CTRL1")]
pub type WbCtrl1 = crate::Reg<wb_ctrl1::WbCtrl1Spec>;
#[doc = "write back ctrl1"]
pub mod wb_ctrl1;
#[doc = "WB_YRGB_MST (rw) register accessor: write back yrgb mst\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wb_yrgb_mst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wb_yrgb_mst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wb_yrgb_mst`]
module"]
#[doc(alias = "WB_YRGB_MST")]
pub type WbYrgbMst = crate::Reg<wb_yrgb_mst::WbYrgbMstSpec>;
#[doc = "write back yrgb mst"]
pub mod wb_yrgb_mst;
#[doc = "WB_CBR_MST (rw) register accessor: write back cbr mst\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wb_cbr_mst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wb_cbr_mst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wb_cbr_mst`]
module"]
#[doc(alias = "WB_CBR_MST")]
pub type WbCbrMst = crate::Reg<wb_cbr_mst::WbCbrMstSpec>;
#[doc = "write back cbr mst"]
pub mod wb_cbr_mst;
#[doc = "WIN0_CTRL0 (rw) register accessor: Win0 ctrl register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_ctrl0`]
module"]
#[doc(alias = "WIN0_CTRL0")]
pub type Win0Ctrl0 = crate::Reg<win0_ctrl0::Win0Ctrl0Spec>;
#[doc = "Win0 ctrl register0"]
pub mod win0_ctrl0;
#[doc = "WIN0_CTRL1 (rw) register accessor: Win0 ctrl register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_ctrl1`]
module"]
#[doc(alias = "WIN0_CTRL1")]
pub type Win0Ctrl1 = crate::Reg<win0_ctrl1::Win0Ctrl1Spec>;
#[doc = "Win0 ctrl register1"]
pub mod win0_ctrl1;
#[doc = "WIN0_COLOR_KEY (rw) register accessor: Win0 color key register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_color_key::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_color_key::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_color_key`]
module"]
#[doc(alias = "WIN0_COLOR_KEY")]
pub type Win0ColorKey = crate::Reg<win0_color_key::Win0ColorKeySpec>;
#[doc = "Win0 color key register"]
pub mod win0_color_key;
#[doc = "WIN0_VIR (rw) register accessor: Win0 virtual stride\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_vir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_vir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_vir`]
module"]
#[doc(alias = "WIN0_VIR")]
pub type Win0Vir = crate::Reg<win0_vir::Win0VirSpec>;
#[doc = "Win0 virtual stride"]
pub mod win0_vir;
#[doc = "WIN0_YRGB_MST (rw) register accessor: Win0 YRGB memory start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_yrgb_mst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_yrgb_mst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_yrgb_mst`]
module"]
#[doc(alias = "WIN0_YRGB_MST")]
pub type Win0YrgbMst = crate::Reg<win0_yrgb_mst::Win0YrgbMstSpec>;
#[doc = "Win0 YRGB memory start address"]
pub mod win0_yrgb_mst;
#[doc = "WIN0_CBR_MST (rw) register accessor: Win0 Cbr memory start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_cbr_mst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_cbr_mst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_cbr_mst`]
module"]
#[doc(alias = "WIN0_CBR_MST")]
pub type Win0CbrMst = crate::Reg<win0_cbr_mst::Win0CbrMstSpec>;
#[doc = "Win0 Cbr memory start address"]
pub mod win0_cbr_mst;
#[doc = "WIN0_ACT_INFO (rw) register accessor: Win0 active window width/height\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_act_info::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_act_info::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_act_info`]
module"]
#[doc(alias = "WIN0_ACT_INFO")]
pub type Win0ActInfo = crate::Reg<win0_act_info::Win0ActInfoSpec>;
#[doc = "Win0 active window width/height"]
pub mod win0_act_info;
#[doc = "WIN0_DSP_INFO (rw) register accessor: Win0 display width/height on panel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_dsp_info::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_dsp_info::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_dsp_info`]
module"]
#[doc(alias = "WIN0_DSP_INFO")]
pub type Win0DspInfo = crate::Reg<win0_dsp_info::Win0DspInfoSpec>;
#[doc = "Win0 display width/height on panel"]
pub mod win0_dsp_info;
#[doc = "WIN0_DSP_ST (rw) register accessor: Win0 display start point on panel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_dsp_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_dsp_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_dsp_st`]
module"]
#[doc(alias = "WIN0_DSP_ST")]
pub type Win0DspSt = crate::Reg<win0_dsp_st::Win0DspStSpec>;
#[doc = "Win0 display start point on panel"]
pub mod win0_dsp_st;
#[doc = "WIN0_SCL_FACTOR_YRGB (rw) register accessor: Win0 YRGB scaling factor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_scl_factor_yrgb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_scl_factor_yrgb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_scl_factor_yrgb`]
module"]
#[doc(alias = "WIN0_SCL_FACTOR_YRGB")]
pub type Win0SclFactorYrgb = crate::Reg<win0_scl_factor_yrgb::Win0SclFactorYrgbSpec>;
#[doc = "Win0 YRGB scaling factor"]
pub mod win0_scl_factor_yrgb;
#[doc = "WIN0_SCL_FACTOR_CBR (rw) register accessor: Win0 Cbr scaling factor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_scl_factor_cbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_scl_factor_cbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_scl_factor_cbr`]
module"]
#[doc(alias = "WIN0_SCL_FACTOR_CBR")]
pub type Win0SclFactorCbr = crate::Reg<win0_scl_factor_cbr::Win0SclFactorCbrSpec>;
#[doc = "Win0 Cbr scaling factor"]
pub mod win0_scl_factor_cbr;
#[doc = "WIN0_SCL_OFFSET (rw) register accessor: Win0 scaling start point offset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_scl_offset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_scl_offset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_scl_offset`]
module"]
#[doc(alias = "WIN0_SCL_OFFSET")]
pub type Win0SclOffset = crate::Reg<win0_scl_offset::Win0SclOffsetSpec>;
#[doc = "Win0 scaling start point offset"]
pub mod win0_scl_offset;
#[doc = "WIN0_SRC_ALPHA_CTRL (rw) register accessor: Win0 alpha source control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_src_alpha_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_src_alpha_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_src_alpha_ctrl`]
module"]
#[doc(alias = "WIN0_SRC_ALPHA_CTRL")]
pub type Win0SrcAlphaCtrl = crate::Reg<win0_src_alpha_ctrl::Win0SrcAlphaCtrlSpec>;
#[doc = "Win0 alpha source control register"]
pub mod win0_src_alpha_ctrl;
#[doc = "WIN0_DST_ALPHA_CTRL (rw) register accessor: Win0 alpha destination control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_dst_alpha_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_dst_alpha_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_dst_alpha_ctrl`]
module"]
#[doc(alias = "WIN0_DST_ALPHA_CTRL")]
pub type Win0DstAlphaCtrl = crate::Reg<win0_dst_alpha_ctrl::Win0DstAlphaCtrlSpec>;
#[doc = "Win0 alpha destination control register"]
pub mod win0_dst_alpha_ctrl;
#[doc = "WIN0_FADING_CTRL (rw) register accessor: Win0 fading contrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_fading_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_fading_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_fading_ctrl`]
module"]
#[doc(alias = "WIN0_FADING_CTRL")]
pub type Win0FadingCtrl = crate::Reg<win0_fading_ctrl::Win0FadingCtrlSpec>;
#[doc = "Win0 fading contrl register"]
pub mod win0_fading_ctrl;
#[doc = "WIN0_CTRL2 (rw) register accessor: Win0 ctrl register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_ctrl2`]
module"]
#[doc(alias = "WIN0_CTRL2")]
pub type Win0Ctrl2 = crate::Reg<win0_ctrl2::Win0Ctrl2Spec>;
#[doc = "Win0 ctrl register2"]
pub mod win0_ctrl2;
#[doc = "WIN1_CTRL0 (rw) register accessor: Win1 ctrl register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_ctrl0`]
module"]
#[doc(alias = "WIN1_CTRL0")]
pub type Win1Ctrl0 = crate::Reg<win1_ctrl0::Win1Ctrl0Spec>;
#[doc = "Win1 ctrl register0"]
pub mod win1_ctrl0;
#[doc = "WIN1_CTRL1 (rw) register accessor: Win1 ctrl register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_ctrl1`]
module"]
#[doc(alias = "WIN1_CTRL1")]
pub type Win1Ctrl1 = crate::Reg<win1_ctrl1::Win1Ctrl1Spec>;
#[doc = "Win1 ctrl register1"]
pub mod win1_ctrl1;
#[doc = "WIN1_COLOR_KEY (rw) register accessor: Win1 color key register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_color_key::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_color_key::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_color_key`]
module"]
#[doc(alias = "WIN1_COLOR_KEY")]
pub type Win1ColorKey = crate::Reg<win1_color_key::Win1ColorKeySpec>;
#[doc = "Win1 color key register"]
pub mod win1_color_key;
#[doc = "WIN1_VIR (rw) register accessor: win1 virtual stride\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_vir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_vir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_vir`]
module"]
#[doc(alias = "WIN1_VIR")]
pub type Win1Vir = crate::Reg<win1_vir::Win1VirSpec>;
#[doc = "win1 virtual stride"]
pub mod win1_vir;
#[doc = "WIN1_YRGB_MST (rw) register accessor: Win1 YRGB memory start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_yrgb_mst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_yrgb_mst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_yrgb_mst`]
module"]
#[doc(alias = "WIN1_YRGB_MST")]
pub type Win1YrgbMst = crate::Reg<win1_yrgb_mst::Win1YrgbMstSpec>;
#[doc = "Win1 YRGB memory start address"]
pub mod win1_yrgb_mst;
#[doc = "WIN1_CBR_MST (rw) register accessor: Win1 Cbr memory start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_cbr_mst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_cbr_mst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_cbr_mst`]
module"]
#[doc(alias = "WIN1_CBR_MST")]
pub type Win1CbrMst = crate::Reg<win1_cbr_mst::Win1CbrMstSpec>;
#[doc = "Win1 Cbr memory start address"]
pub mod win1_cbr_mst;
#[doc = "WIN1_ACT_INFO (rw) register accessor: Win1 active window width/height\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_act_info::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_act_info::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_act_info`]
module"]
#[doc(alias = "WIN1_ACT_INFO")]
pub type Win1ActInfo = crate::Reg<win1_act_info::Win1ActInfoSpec>;
#[doc = "Win1 active window width/height"]
pub mod win1_act_info;
#[doc = "WIN1_DSP_INFO (rw) register accessor: Win1 display width/height on panel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_dsp_info::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_dsp_info::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_dsp_info`]
module"]
#[doc(alias = "WIN1_DSP_INFO")]
pub type Win1DspInfo = crate::Reg<win1_dsp_info::Win1DspInfoSpec>;
#[doc = "Win1 display width/height on panel"]
pub mod win1_dsp_info;
#[doc = "WIN1_DSP_ST (rw) register accessor: Win1 display start point on panel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_dsp_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_dsp_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_dsp_st`]
module"]
#[doc(alias = "WIN1_DSP_ST")]
pub type Win1DspSt = crate::Reg<win1_dsp_st::Win1DspStSpec>;
#[doc = "Win1 display start point on panel"]
pub mod win1_dsp_st;
#[doc = "WIN1_SCL_FACTOR_YRGB (rw) register accessor: Win1 YRGB scaling factor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_scl_factor_yrgb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_scl_factor_yrgb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_scl_factor_yrgb`]
module"]
#[doc(alias = "WIN1_SCL_FACTOR_YRGB")]
pub type Win1SclFactorYrgb = crate::Reg<win1_scl_factor_yrgb::Win1SclFactorYrgbSpec>;
#[doc = "Win1 YRGB scaling factor"]
pub mod win1_scl_factor_yrgb;
#[doc = "WIN1_SCL_FACTOR_CBR (rw) register accessor: Win1 Cbr scaling factor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_scl_factor_cbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_scl_factor_cbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_scl_factor_cbr`]
module"]
#[doc(alias = "WIN1_SCL_FACTOR_CBR")]
pub type Win1SclFactorCbr = crate::Reg<win1_scl_factor_cbr::Win1SclFactorCbrSpec>;
#[doc = "Win1 Cbr scaling factor"]
pub mod win1_scl_factor_cbr;
#[doc = "WIN1_SCL_OFFSET (rw) register accessor: Win1 scaling start point offset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_scl_offset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_scl_offset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_scl_offset`]
module"]
#[doc(alias = "WIN1_SCL_OFFSET")]
pub type Win1SclOffset = crate::Reg<win1_scl_offset::Win1SclOffsetSpec>;
#[doc = "Win1 scaling start point offset"]
pub mod win1_scl_offset;
#[doc = "WIN1_SRC_ALPHA_CTRL (rw) register accessor: Win1 alpha source control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_src_alpha_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_src_alpha_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_src_alpha_ctrl`]
module"]
#[doc(alias = "WIN1_SRC_ALPHA_CTRL")]
pub type Win1SrcAlphaCtrl = crate::Reg<win1_src_alpha_ctrl::Win1SrcAlphaCtrlSpec>;
#[doc = "Win1 alpha source control register"]
pub mod win1_src_alpha_ctrl;
#[doc = "WIN1_DST_ALPHA_CTRL (rw) register accessor: Win1 alpha destination control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_dst_alpha_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_dst_alpha_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_dst_alpha_ctrl`]
module"]
#[doc(alias = "WIN1_DST_ALPHA_CTRL")]
pub type Win1DstAlphaCtrl = crate::Reg<win1_dst_alpha_ctrl::Win1DstAlphaCtrlSpec>;
#[doc = "Win1 alpha destination control register"]
pub mod win1_dst_alpha_ctrl;
#[doc = "WIN1_FADING_CTRL (rw) register accessor: Win1 fading contrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_fading_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_fading_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_fading_ctrl`]
module"]
#[doc(alias = "WIN1_FADING_CTRL")]
pub type Win1FadingCtrl = crate::Reg<win1_fading_ctrl::Win1FadingCtrlSpec>;
#[doc = "Win1 fading contrl register"]
pub mod win1_fading_ctrl;
#[doc = "WIN1_CTRL2 (rw) register accessor: Win1 ctrl register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_ctrl2`]
module"]
#[doc(alias = "WIN1_CTRL2")]
pub type Win1Ctrl2 = crate::Reg<win1_ctrl2::Win1Ctrl2Spec>;
#[doc = "Win1 ctrl register2"]
pub mod win1_ctrl2;
#[doc = "WIN2_CTRL0 (rw) register accessor: win2 ctrl register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_ctrl0`]
module"]
#[doc(alias = "WIN2_CTRL0")]
pub type Win2Ctrl0 = crate::Reg<win2_ctrl0::Win2Ctrl0Spec>;
#[doc = "win2 ctrl register0"]
pub mod win2_ctrl0;
#[doc = "WIN2_CTRL1 (rw) register accessor: win2 ctrl register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_ctrl1`]
module"]
#[doc(alias = "WIN2_CTRL1")]
pub type Win2Ctrl1 = crate::Reg<win2_ctrl1::Win2Ctrl1Spec>;
#[doc = "win2 ctrl register1"]
pub mod win2_ctrl1;
#[doc = "WIN2_VIR0_1 (rw) register accessor: Win2 virtual stride0 and virtaul stride1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_vir0_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_vir0_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_vir0_1`]
module"]
#[doc(alias = "WIN2_VIR0_1")]
pub type Win2Vir0_1 = crate::Reg<win2_vir0_1::Win2Vir0_1Spec>;
#[doc = "Win2 virtual stride0 and virtaul stride1"]
pub mod win2_vir0_1;
#[doc = "WIN2_VIR2_3 (rw) register accessor: Win2 virtual stride2 and virtaul stride3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_vir2_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_vir2_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_vir2_3`]
module"]
#[doc(alias = "WIN2_VIR2_3")]
pub type Win2Vir2_3 = crate::Reg<win2_vir2_3::Win2Vir2_3Spec>;
#[doc = "Win2 virtual stride2 and virtaul stride3"]
pub mod win2_vir2_3;
#[doc = "WIN2_MST0 (rw) register accessor: Win2 memory start address0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_mst0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_mst0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_mst0`]
module"]
#[doc(alias = "WIN2_MST0")]
pub type Win2Mst0 = crate::Reg<win2_mst0::Win2Mst0Spec>;
#[doc = "Win2 memory start address0"]
pub mod win2_mst0;
#[doc = "WIN2_DSP_INFO0 (rw) register accessor: Win2 display width0/height0 on panel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_dsp_info0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_dsp_info0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_dsp_info0`]
module"]
#[doc(alias = "WIN2_DSP_INFO0")]
pub type Win2DspInfo0 = crate::Reg<win2_dsp_info0::Win2DspInfo0Spec>;
#[doc = "Win2 display width0/height0 on panel"]
pub mod win2_dsp_info0;
#[doc = "WIN2_DSP_ST0 (rw) register accessor: Win2 display start point0 on panel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_dsp_st0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_dsp_st0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_dsp_st0`]
module"]
#[doc(alias = "WIN2_DSP_ST0")]
pub type Win2DspSt0 = crate::Reg<win2_dsp_st0::Win2DspSt0Spec>;
#[doc = "Win2 display start point0 on panel"]
pub mod win2_dsp_st0;
#[doc = "WIN2_COLOR_KEY (rw) register accessor: Win2 color key register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_color_key::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_color_key::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_color_key`]
module"]
#[doc(alias = "WIN2_COLOR_KEY")]
pub type Win2ColorKey = crate::Reg<win2_color_key::Win2ColorKeySpec>;
#[doc = "Win2 color key register"]
pub mod win2_color_key;
#[doc = "WIN2_MST1 (rw) register accessor: Win2 memory start address1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_mst1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_mst1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_mst1`]
module"]
#[doc(alias = "WIN2_MST1")]
pub type Win2Mst1 = crate::Reg<win2_mst1::Win2Mst1Spec>;
#[doc = "Win2 memory start address1"]
pub mod win2_mst1;
#[doc = "WIN2_DSP_INFO1 (rw) register accessor: Win2 display width1/height1 on panel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_dsp_info1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_dsp_info1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_dsp_info1`]
module"]
#[doc(alias = "WIN2_DSP_INFO1")]
pub type Win2DspInfo1 = crate::Reg<win2_dsp_info1::Win2DspInfo1Spec>;
#[doc = "Win2 display width1/height1 on panel"]
pub mod win2_dsp_info1;
#[doc = "WIN2_DSP_ST1 (rw) register accessor: Win2 display start point1 on panel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_dsp_st1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_dsp_st1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_dsp_st1`]
module"]
#[doc(alias = "WIN2_DSP_ST1")]
pub type Win2DspSt1 = crate::Reg<win2_dsp_st1::Win2DspSt1Spec>;
#[doc = "Win2 display start point1 on panel"]
pub mod win2_dsp_st1;
#[doc = "WIN2_SRC_ALPHA_CTRL (rw) register accessor: Win2 alpha source control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_src_alpha_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_src_alpha_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_src_alpha_ctrl`]
module"]
#[doc(alias = "WIN2_SRC_ALPHA_CTRL")]
pub type Win2SrcAlphaCtrl = crate::Reg<win2_src_alpha_ctrl::Win2SrcAlphaCtrlSpec>;
#[doc = "Win2 alpha source control register"]
pub mod win2_src_alpha_ctrl;
#[doc = "WIN2_MST2 (rw) register accessor: Win2 memory start address2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_mst2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_mst2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_mst2`]
module"]
#[doc(alias = "WIN2_MST2")]
pub type Win2Mst2 = crate::Reg<win2_mst2::Win2Mst2Spec>;
#[doc = "Win2 memory start address2"]
pub mod win2_mst2;
#[doc = "WIN2_DSP_INFO2 (rw) register accessor: Win2 display width2/height2 on panel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_dsp_info2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_dsp_info2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_dsp_info2`]
module"]
#[doc(alias = "WIN2_DSP_INFO2")]
pub type Win2DspInfo2 = crate::Reg<win2_dsp_info2::Win2DspInfo2Spec>;
#[doc = "Win2 display width2/height2 on panel"]
pub mod win2_dsp_info2;
#[doc = "WIN2_DSP_ST2 (rw) register accessor: Win2 display start point2 on panel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_dsp_st2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_dsp_st2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_dsp_st2`]
module"]
#[doc(alias = "WIN2_DSP_ST2")]
pub type Win2DspSt2 = crate::Reg<win2_dsp_st2::Win2DspSt2Spec>;
#[doc = "Win2 display start point2 on panel"]
pub mod win2_dsp_st2;
#[doc = "WIN2_DST_ALPHA_CTRL (rw) register accessor: Win2 alpha destination control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_dst_alpha_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_dst_alpha_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_dst_alpha_ctrl`]
module"]
#[doc(alias = "WIN2_DST_ALPHA_CTRL")]
pub type Win2DstAlphaCtrl = crate::Reg<win2_dst_alpha_ctrl::Win2DstAlphaCtrlSpec>;
#[doc = "Win2 alpha destination control register"]
pub mod win2_dst_alpha_ctrl;
#[doc = "WIN2_MST3 (rw) register accessor: Win2 memory start address3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_mst3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_mst3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_mst3`]
module"]
#[doc(alias = "WIN2_MST3")]
pub type Win2Mst3 = crate::Reg<win2_mst3::Win2Mst3Spec>;
#[doc = "Win2 memory start address3"]
pub mod win2_mst3;
#[doc = "WIN2_DSP_INFO3 (rw) register accessor: Win2 display width3/height3 on panel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_dsp_info3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_dsp_info3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_dsp_info3`]
module"]
#[doc(alias = "WIN2_DSP_INFO3")]
pub type Win2DspInfo3 = crate::Reg<win2_dsp_info3::Win2DspInfo3Spec>;
#[doc = "Win2 display width3/height3 on panel"]
pub mod win2_dsp_info3;
#[doc = "WIN2_DSP_ST3 (rw) register accessor: Win2 display start point3 on panel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_dsp_st3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_dsp_st3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_dsp_st3`]
module"]
#[doc(alias = "WIN2_DSP_ST3")]
pub type Win2DspSt3 = crate::Reg<win2_dsp_st3::Win2DspSt3Spec>;
#[doc = "Win2 display start point3 on panel"]
pub mod win2_dsp_st3;
#[doc = "WIN2_FADING_CTRL (rw) register accessor: Win2 fading contrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_fading_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_fading_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_fading_ctrl`]
module"]
#[doc(alias = "WIN2_FADING_CTRL")]
pub type Win2FadingCtrl = crate::Reg<win2_fading_ctrl::Win2FadingCtrlSpec>;
#[doc = "Win2 fading contrl register"]
pub mod win2_fading_ctrl;
#[doc = "WIN3_CTRL0 (rw) register accessor: Win3 ctrl register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_ctrl0`]
module"]
#[doc(alias = "WIN3_CTRL0")]
pub type Win3Ctrl0 = crate::Reg<win3_ctrl0::Win3Ctrl0Spec>;
#[doc = "Win3 ctrl register0"]
pub mod win3_ctrl0;
#[doc = "WIN3_CTRL1 (rw) register accessor: Win3 ctrl register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_ctrl1`]
module"]
#[doc(alias = "WIN3_CTRL1")]
pub type Win3Ctrl1 = crate::Reg<win3_ctrl1::Win3Ctrl1Spec>;
#[doc = "Win3 ctrl register1"]
pub mod win3_ctrl1;
#[doc = "WIN3_VIR0_1 (rw) register accessor: Win3 virtual stride0 and virtaul stride1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_vir0_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_vir0_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_vir0_1`]
module"]
#[doc(alias = "WIN3_VIR0_1")]
pub type Win3Vir0_1 = crate::Reg<win3_vir0_1::Win3Vir0_1Spec>;
#[doc = "Win3 virtual stride0 and virtaul stride1"]
pub mod win3_vir0_1;
#[doc = "WIN3_VIR2_3 (rw) register accessor: Win3 virtual stride2 and virtaul stride3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_vir2_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_vir2_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_vir2_3`]
module"]
#[doc(alias = "WIN3_VIR2_3")]
pub type Win3Vir2_3 = crate::Reg<win3_vir2_3::Win3Vir2_3Spec>;
#[doc = "Win3 virtual stride2 and virtaul stride3"]
pub mod win3_vir2_3;
#[doc = "WIN3_MST0 (rw) register accessor: Win3 memory start address0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_mst0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_mst0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_mst0`]
module"]
#[doc(alias = "WIN3_MST0")]
pub type Win3Mst0 = crate::Reg<win3_mst0::Win3Mst0Spec>;
#[doc = "Win3 memory start address0"]
pub mod win3_mst0;
#[doc = "WIN3_DSP_INFO0 (rw) register accessor: Win3 display width0/height0 on panel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_dsp_info0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_dsp_info0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_dsp_info0`]
module"]
#[doc(alias = "WIN3_DSP_INFO0")]
pub type Win3DspInfo0 = crate::Reg<win3_dsp_info0::Win3DspInfo0Spec>;
#[doc = "Win3 display width0/height0 on panel"]
pub mod win3_dsp_info0;
#[doc = "WIN3_DSP_ST0 (rw) register accessor: Win3 display start point0 on panel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_dsp_st0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_dsp_st0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_dsp_st0`]
module"]
#[doc(alias = "WIN3_DSP_ST0")]
pub type Win3DspSt0 = crate::Reg<win3_dsp_st0::Win3DspSt0Spec>;
#[doc = "Win3 display start point0 on panel"]
pub mod win3_dsp_st0;
#[doc = "WIN3_COLOR_KEY (rw) register accessor: Win3 color key register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_color_key::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_color_key::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_color_key`]
module"]
#[doc(alias = "WIN3_COLOR_KEY")]
pub type Win3ColorKey = crate::Reg<win3_color_key::Win3ColorKeySpec>;
#[doc = "Win3 color key register"]
pub mod win3_color_key;
#[doc = "WIN3_MST1 (rw) register accessor: Win3 memory start address1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_mst1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_mst1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_mst1`]
module"]
#[doc(alias = "WIN3_MST1")]
pub type Win3Mst1 = crate::Reg<win3_mst1::Win3Mst1Spec>;
#[doc = "Win3 memory start address1"]
pub mod win3_mst1;
#[doc = "WIN3_DSP_INFO1 (rw) register accessor: Win3 display width1/height1 on panel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_dsp_info1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_dsp_info1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_dsp_info1`]
module"]
#[doc(alias = "WIN3_DSP_INFO1")]
pub type Win3DspInfo1 = crate::Reg<win3_dsp_info1::Win3DspInfo1Spec>;
#[doc = "Win3 display width1/height1 on panel"]
pub mod win3_dsp_info1;
#[doc = "WIN3_DSP_ST1 (rw) register accessor: Win3 display start point1 on panel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_dsp_st1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_dsp_st1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_dsp_st1`]
module"]
#[doc(alias = "WIN3_DSP_ST1")]
pub type Win3DspSt1 = crate::Reg<win3_dsp_st1::Win3DspSt1Spec>;
#[doc = "Win3 display start point1 on panel"]
pub mod win3_dsp_st1;
#[doc = "WIN3_SRC_ALPHA_CTRL (rw) register accessor: Win3 alpha source control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_src_alpha_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_src_alpha_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_src_alpha_ctrl`]
module"]
#[doc(alias = "WIN3_SRC_ALPHA_CTRL")]
pub type Win3SrcAlphaCtrl = crate::Reg<win3_src_alpha_ctrl::Win3SrcAlphaCtrlSpec>;
#[doc = "Win3 alpha source control register"]
pub mod win3_src_alpha_ctrl;
#[doc = "WIN3_MST2 (rw) register accessor: Win3 memory start address2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_mst2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_mst2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_mst2`]
module"]
#[doc(alias = "WIN3_MST2")]
pub type Win3Mst2 = crate::Reg<win3_mst2::Win3Mst2Spec>;
#[doc = "Win3 memory start address2"]
pub mod win3_mst2;
#[doc = "WIN3_DSP_INFO2 (rw) register accessor: Win3 display width2/height2 on panel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_dsp_info2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_dsp_info2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_dsp_info2`]
module"]
#[doc(alias = "WIN3_DSP_INFO2")]
pub type Win3DspInfo2 = crate::Reg<win3_dsp_info2::Win3DspInfo2Spec>;
#[doc = "Win3 display width2/height2 on panel"]
pub mod win3_dsp_info2;
#[doc = "WIN3_DSP_ST2 (rw) register accessor: Win3 display start point2 on panel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_dsp_st2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_dsp_st2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_dsp_st2`]
module"]
#[doc(alias = "WIN3_DSP_ST2")]
pub type Win3DspSt2 = crate::Reg<win3_dsp_st2::Win3DspSt2Spec>;
#[doc = "Win3 display start point2 on panel"]
pub mod win3_dsp_st2;
#[doc = "WIN3_DST_ALPHA_CTRL (rw) register accessor: Win3 alpha destination control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_dst_alpha_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_dst_alpha_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_dst_alpha_ctrl`]
module"]
#[doc(alias = "WIN3_DST_ALPHA_CTRL")]
pub type Win3DstAlphaCtrl = crate::Reg<win3_dst_alpha_ctrl::Win3DstAlphaCtrlSpec>;
#[doc = "Win3 alpha destination control register"]
pub mod win3_dst_alpha_ctrl;
#[doc = "WIN3_MST3 (rw) register accessor: Win3 memory start address3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_mst3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_mst3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_mst3`]
module"]
#[doc(alias = "WIN3_MST3")]
pub type Win3Mst3 = crate::Reg<win3_mst3::Win3Mst3Spec>;
#[doc = "Win3 memory start address3"]
pub mod win3_mst3;
#[doc = "WIN3_DSP_INFO3 (rw) register accessor: Win3 display width3/height3 on panel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_dsp_info3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_dsp_info3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_dsp_info3`]
module"]
#[doc(alias = "WIN3_DSP_INFO3")]
pub type Win3DspInfo3 = crate::Reg<win3_dsp_info3::Win3DspInfo3Spec>;
#[doc = "Win3 display width3/height3 on panel"]
pub mod win3_dsp_info3;
#[doc = "WIN3_DSP_ST3 (rw) register accessor: Win3 display start point3 on panel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_dsp_st3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_dsp_st3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_dsp_st3`]
module"]
#[doc(alias = "WIN3_DSP_ST3")]
pub type Win3DspSt3 = crate::Reg<win3_dsp_st3::Win3DspSt3Spec>;
#[doc = "Win3 display start point3 on panel"]
pub mod win3_dsp_st3;
#[doc = "WIN3_FADING_CTRL (rw) register accessor: Win3 fading contrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_fading_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_fading_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_fading_ctrl`]
module"]
#[doc(alias = "WIN3_FADING_CTRL")]
pub type Win3FadingCtrl = crate::Reg<win3_fading_ctrl::Win3FadingCtrlSpec>;
#[doc = "Win3 fading contrl register"]
pub mod win3_fading_ctrl;
#[doc = "HWC_CTRL0 (rw) register accessor: Hwc ctrl register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwc_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwc_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwc_ctrl0`]
module"]
#[doc(alias = "HWC_CTRL0")]
pub type HwcCtrl0 = crate::Reg<hwc_ctrl0::HwcCtrl0Spec>;
#[doc = "Hwc ctrl register0"]
pub mod hwc_ctrl0;
#[doc = "HWC_CTRL1 (rw) register accessor: Hwc ctrl register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwc_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwc_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwc_ctrl1`]
module"]
#[doc(alias = "HWC_CTRL1")]
pub type HwcCtrl1 = crate::Reg<hwc_ctrl1::HwcCtrl1Spec>;
#[doc = "Hwc ctrl register1"]
pub mod hwc_ctrl1;
#[doc = "HWC_MST (rw) register accessor: Hwc memory start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwc_mst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwc_mst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwc_mst`]
module"]
#[doc(alias = "HWC_MST")]
pub type HwcMst = crate::Reg<hwc_mst::HwcMstSpec>;
#[doc = "Hwc memory start address"]
pub mod hwc_mst;
#[doc = "HWC_DSP_ST (rw) register accessor: Hwc display start point on panel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwc_dsp_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwc_dsp_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwc_dsp_st`]
module"]
#[doc(alias = "HWC_DSP_ST")]
pub type HwcDspSt = crate::Reg<hwc_dsp_st::HwcDspStSpec>;
#[doc = "Hwc display start point on panel"]
pub mod hwc_dsp_st;
#[doc = "HWC_SRC_ALPHA_CTRL (rw) register accessor: Hwc alpha source control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwc_src_alpha_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwc_src_alpha_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwc_src_alpha_ctrl`]
module"]
#[doc(alias = "HWC_SRC_ALPHA_CTRL")]
pub type HwcSrcAlphaCtrl = crate::Reg<hwc_src_alpha_ctrl::HwcSrcAlphaCtrlSpec>;
#[doc = "Hwc alpha source control register"]
pub mod hwc_src_alpha_ctrl;
#[doc = "HWC_DST_ALPHA_CTRL (rw) register accessor: Hwc alpha destination control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwc_dst_alpha_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwc_dst_alpha_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwc_dst_alpha_ctrl`]
module"]
#[doc(alias = "HWC_DST_ALPHA_CTRL")]
pub type HwcDstAlphaCtrl = crate::Reg<hwc_dst_alpha_ctrl::HwcDstAlphaCtrlSpec>;
#[doc = "Hwc alpha destination control register"]
pub mod hwc_dst_alpha_ctrl;
#[doc = "HWC_FADING_CTRL (rw) register accessor: Hwc fading contrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwc_fading_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwc_fading_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwc_fading_ctrl`]
module"]
#[doc(alias = "HWC_FADING_CTRL")]
pub type HwcFadingCtrl = crate::Reg<hwc_fading_ctrl::HwcFadingCtrlSpec>;
#[doc = "Hwc fading contrl register"]
pub mod hwc_fading_ctrl;
#[doc = "POST_DSP_HACT_INFO (rw) register accessor: Post scaler down horizontal start and end\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`post_dsp_hact_info::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`post_dsp_hact_info::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@post_dsp_hact_info`]
module"]
#[doc(alias = "POST_DSP_HACT_INFO")]
pub type PostDspHactInfo = crate::Reg<post_dsp_hact_info::PostDspHactInfoSpec>;
#[doc = "Post scaler down horizontal start and end"]
pub mod post_dsp_hact_info;
#[doc = "POST_DSP_VACT_INFO (rw) register accessor: Panel active horizontal scanning start point and end point\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`post_dsp_vact_info::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`post_dsp_vact_info::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@post_dsp_vact_info`]
module"]
#[doc(alias = "POST_DSP_VACT_INFO")]
pub type PostDspVactInfo = crate::Reg<post_dsp_vact_info::PostDspVactInfoSpec>;
#[doc = "Panel active horizontal scanning start point and end point"]
pub mod post_dsp_vact_info;
#[doc = "POST_SCL_FACTOR_YRGB (rw) register accessor: Post yrgb scaling factor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`post_scl_factor_yrgb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`post_scl_factor_yrgb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@post_scl_factor_yrgb`]
module"]
#[doc(alias = "POST_SCL_FACTOR_YRGB")]
pub type PostSclFactorYrgb = crate::Reg<post_scl_factor_yrgb::PostSclFactorYrgbSpec>;
#[doc = "Post yrgb scaling factor"]
pub mod post_scl_factor_yrgb;
#[doc = "POST_RESERVED (rw) register accessor: Post reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`post_reserved::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`post_reserved::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@post_reserved`]
module"]
#[doc(alias = "POST_RESERVED")]
pub type PostReserved = crate::Reg<post_reserved::PostReservedSpec>;
#[doc = "Post reserved"]
pub mod post_reserved;
#[doc = "POST_SCL_CTRL (rw) register accessor: Post scaling start point offset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`post_scl_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`post_scl_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@post_scl_ctrl`]
module"]
#[doc(alias = "POST_SCL_CTRL")]
pub type PostSclCtrl = crate::Reg<post_scl_ctrl::PostSclCtrlSpec>;
#[doc = "Post scaling start point offset"]
pub mod post_scl_ctrl;
#[doc = "POST_DSP_VACT_INFO_F1 (rw) register accessor: Panel active horizontal scanning start point and end point F1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`post_dsp_vact_info_f1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`post_dsp_vact_info_f1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@post_dsp_vact_info_f1`]
module"]
#[doc(alias = "POST_DSP_VACT_INFO_F1")]
pub type PostDspVactInfoF1 = crate::Reg<post_dsp_vact_info_f1::PostDspVactInfoF1Spec>;
#[doc = "Panel active horizontal scanning start point and end point F1"]
pub mod post_dsp_vact_info_f1;
#[doc = "DSP_HTOTAL_HS_END (rw) register accessor: Panel scanning horizontal width and hsync pulse end point\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_htotal_hs_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_htotal_hs_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_htotal_hs_end`]
module"]
#[doc(alias = "DSP_HTOTAL_HS_END")]
pub type DspHtotalHsEnd = crate::Reg<dsp_htotal_hs_end::DspHtotalHsEndSpec>;
#[doc = "Panel scanning horizontal width and hsync pulse end point"]
pub mod dsp_htotal_hs_end;
#[doc = "DSP_HACT_ST_END (rw) register accessor: Panel active horizontal scanning start point and end point\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_hact_st_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_hact_st_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_hact_st_end`]
module"]
#[doc(alias = "DSP_HACT_ST_END")]
pub type DspHactStEnd = crate::Reg<dsp_hact_st_end::DspHactStEndSpec>;
#[doc = "Panel active horizontal scanning start point and end point"]
pub mod dsp_hact_st_end;
#[doc = "DSP_VTOTAL_VS_END (rw) register accessor: Panel scanning vertical height and vsync pulse end point\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_vtotal_vs_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_vtotal_vs_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_vtotal_vs_end`]
module"]
#[doc(alias = "DSP_VTOTAL_VS_END")]
pub type DspVtotalVsEnd = crate::Reg<dsp_vtotal_vs_end::DspVtotalVsEndSpec>;
#[doc = "Panel scanning vertical height and vsync pulse end point"]
pub mod dsp_vtotal_vs_end;
#[doc = "DSP_VACT_ST_END (rw) register accessor: Panel active vertical scanning start point and end point\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_vact_st_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_vact_st_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_vact_st_end`]
module"]
#[doc(alias = "DSP_VACT_ST_END")]
pub type DspVactStEnd = crate::Reg<dsp_vact_st_end::DspVactStEndSpec>;
#[doc = "Panel active vertical scanning start point and end point"]
pub mod dsp_vact_st_end;
#[doc = "DSP_VS_ST_END_F1 (rw) register accessor: Vertical scanning start point and vsync pulse end point of even filed in interlace mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_vs_st_end_f1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_vs_st_end_f1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_vs_st_end_f1`]
module"]
#[doc(alias = "DSP_VS_ST_END_F1")]
pub type DspVsStEndF1 = crate::Reg<dsp_vs_st_end_f1::DspVsStEndF1Spec>;
#[doc = "Vertical scanning start point and vsync pulse end point of even filed in interlace mode"]
pub mod dsp_vs_st_end_f1;
#[doc = "DSP_VACT_ST_END_F1 (rw) register accessor: Vertical scanning active start point and end point of even filed in interlace mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_vact_st_end_f1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_vact_st_end_f1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_vact_st_end_f1`]
module"]
#[doc(alias = "DSP_VACT_ST_END_F1")]
pub type DspVactStEndF1 = crate::Reg<dsp_vact_st_end_f1::DspVactStEndF1Spec>;
#[doc = "Vertical scanning active start point and end point of even filed in interlace mode"]
pub mod dsp_vact_st_end_f1;
#[doc = "PWM_CTRL (rw) register accessor: PWM Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_ctrl`]
module"]
#[doc(alias = "PWM_CTRL")]
pub type PwmCtrl = crate::Reg<pwm_ctrl::PwmCtrlSpec>;
#[doc = "PWM Control Register"]
pub mod pwm_ctrl;
#[doc = "PWM_PERIOD_HPR (rw) register accessor: PWM Period Register/High Polarity Capture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_period_hpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_period_hpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_period_hpr`]
module"]
#[doc(alias = "PWM_PERIOD_HPR")]
pub type PwmPeriodHpr = crate::Reg<pwm_period_hpr::PwmPeriodHprSpec>;
#[doc = "PWM Period Register/High Polarity Capture Register"]
pub mod pwm_period_hpr;
#[doc = "PWM_DUTY_LPR (rw) register accessor: PWM Duty Register/Low Polarity Capture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_duty_lpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_duty_lpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_duty_lpr`]
module"]
#[doc(alias = "PWM_DUTY_LPR")]
pub type PwmDutyLpr = crate::Reg<pwm_duty_lpr::PwmDutyLprSpec>;
#[doc = "PWM Duty Register/Low Polarity Capture Register"]
pub mod pwm_duty_lpr;
#[doc = "PWM_CNT (r) register accessor: PWM Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_cnt`]
module"]
#[doc(alias = "PWM_CNT")]
pub type PwmCnt = crate::Reg<pwm_cnt::PwmCntSpec>;
#[doc = "PWM Counter Register"]
pub mod pwm_cnt;
#[doc = "BCSH_COLOR_BAR (rw) register accessor: Color bar config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcsh_color_bar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcsh_color_bar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcsh_color_bar`]
module"]
#[doc(alias = "BCSH_COLOR_BAR")]
pub type BcshColorBar = crate::Reg<bcsh_color_bar::BcshColorBarSpec>;
#[doc = "Color bar config register"]
pub mod bcsh_color_bar;
#[doc = "BCSH_BCS (rw) register accessor: Brightness contrast saturation*contrast config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcsh_bcs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcsh_bcs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcsh_bcs`]
module"]
#[doc(alias = "BCSH_BCS")]
pub type BcshBcs = crate::Reg<bcsh_bcs::BcshBcsSpec>;
#[doc = "Brightness contrast saturation*contrast config register"]
pub mod bcsh_bcs;
#[doc = "BCSH_H (rw) register accessor: Sin hue and cos hue config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcsh_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcsh_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcsh_h`]
module"]
#[doc(alias = "BCSH_H")]
pub type BcshH = crate::Reg<bcsh_h::BcshHSpec>;
#[doc = "Sin hue and cos hue config register"]
pub mod bcsh_h;
#[doc = "BCSH_CTRL (rw) register accessor: BCSH contrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcsh_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcsh_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcsh_ctrl`]
module"]
#[doc(alias = "BCSH_CTRL")]
pub type BcshCtrl = crate::Reg<bcsh_ctrl::BcshCtrlSpec>;
#[doc = "BCSH contrl register"]
pub mod bcsh_ctrl;
#[doc = "CABC_CTRL0 (rw) register accessor: Content Adaptive Backlight Control register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cabc_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cabc_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cabc_ctrl0`]
module"]
#[doc(alias = "CABC_CTRL0")]
pub type CabcCtrl0 = crate::Reg<cabc_ctrl0::CabcCtrl0Spec>;
#[doc = "Content Adaptive Backlight Control register0"]
pub mod cabc_ctrl0;
#[doc = "CABC_CTRL1 (rw) register accessor: Content Adaptive Backlight Control register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cabc_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cabc_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cabc_ctrl1`]
module"]
#[doc(alias = "CABC_CTRL1")]
pub type CabcCtrl1 = crate::Reg<cabc_ctrl1::CabcCtrl1Spec>;
#[doc = "Content Adaptive Backlight Control register1"]
pub mod cabc_ctrl1;
#[doc = "CABC_CTRL2 (rw) register accessor: Content Adaptive Backlight Control register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cabc_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cabc_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cabc_ctrl2`]
module"]
#[doc(alias = "CABC_CTRL2")]
pub type CabcCtrl2 = crate::Reg<cabc_ctrl2::CabcCtrl2Spec>;
#[doc = "Content Adaptive Backlight Control register2"]
pub mod cabc_ctrl2;
#[doc = "CABC_CTRL3 (rw) register accessor: Content Adaptive Backlight Control register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cabc_ctrl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cabc_ctrl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cabc_ctrl3`]
module"]
#[doc(alias = "CABC_CTRL3")]
pub type CabcCtrl3 = crate::Reg<cabc_ctrl3::CabcCtrl3Spec>;
#[doc = "Content Adaptive Backlight Control register3"]
pub mod cabc_ctrl3;
#[doc = "CABC_GAUSS_LINE0_0 (rw) register accessor: CABC gauss line config register00\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cabc_gauss_line0_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cabc_gauss_line0_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cabc_gauss_line0_0`]
module"]
#[doc(alias = "CABC_GAUSS_LINE0_0")]
pub type CabcGaussLine0_0 = crate::Reg<cabc_gauss_line0_0::CabcGaussLine0_0Spec>;
#[doc = "CABC gauss line config register00"]
pub mod cabc_gauss_line0_0;
#[doc = "CABC_GAUSS_LINE0_1 (rw) register accessor: CABC gauss line config register01\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cabc_gauss_line0_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cabc_gauss_line0_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cabc_gauss_line0_1`]
module"]
#[doc(alias = "CABC_GAUSS_LINE0_1")]
pub type CabcGaussLine0_1 = crate::Reg<cabc_gauss_line0_1::CabcGaussLine0_1Spec>;
#[doc = "CABC gauss line config register01"]
pub mod cabc_gauss_line0_1;
#[doc = "CABC_GAUSS_LINE1_0 (rw) register accessor: CABC gauss line config register10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cabc_gauss_line1_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cabc_gauss_line1_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cabc_gauss_line1_0`]
module"]
#[doc(alias = "CABC_GAUSS_LINE1_0")]
pub type CabcGaussLine1_0 = crate::Reg<cabc_gauss_line1_0::CabcGaussLine1_0Spec>;
#[doc = "CABC gauss line config register10"]
pub mod cabc_gauss_line1_0;
#[doc = "CABC_GAUSS_LINE1_1 (rw) register accessor: CABC gauss line config register11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cabc_gauss_line1_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cabc_gauss_line1_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cabc_gauss_line1_1`]
module"]
#[doc(alias = "CABC_GAUSS_LINE1_1")]
pub type CabcGaussLine1_1 = crate::Reg<cabc_gauss_line1_1::CabcGaussLine1_1Spec>;
#[doc = "CABC gauss line config register11"]
pub mod cabc_gauss_line1_1;
#[doc = "CABC_GAUSS_LINE2_0 (rw) register accessor: CABC gauss line config register20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cabc_gauss_line2_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cabc_gauss_line2_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cabc_gauss_line2_0`]
module"]
#[doc(alias = "CABC_GAUSS_LINE2_0")]
pub type CabcGaussLine2_0 = crate::Reg<cabc_gauss_line2_0::CabcGaussLine2_0Spec>;
#[doc = "CABC gauss line config register20"]
pub mod cabc_gauss_line2_0;
#[doc = "CABC_GAUSS_LINE2_1 (rw) register accessor: CABC gauss line config register21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cabc_gauss_line2_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cabc_gauss_line2_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cabc_gauss_line2_1`]
module"]
#[doc(alias = "CABC_GAUSS_LINE2_1")]
pub type CabcGaussLine2_1 = crate::Reg<cabc_gauss_line2_1::CabcGaussLine2_1Spec>;
#[doc = "CABC gauss line config register21"]
pub mod cabc_gauss_line2_1;
#[doc = "FRC_LOWER01_0 (rw) register accessor: FRC lookup table config register010\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frc_lower01_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frc_lower01_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frc_lower01_0`]
module"]
#[doc(alias = "FRC_LOWER01_0")]
pub type FrcLower01_0 = crate::Reg<frc_lower01_0::FrcLower01_0Spec>;
#[doc = "FRC lookup table config register010"]
pub mod frc_lower01_0;
#[doc = "FRC_LOWER01_1 (rw) register accessor: FRC lookup table config register011\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frc_lower01_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frc_lower01_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frc_lower01_1`]
module"]
#[doc(alias = "FRC_LOWER01_1")]
pub type FrcLower01_1 = crate::Reg<frc_lower01_1::FrcLower01_1Spec>;
#[doc = "FRC lookup table config register011"]
pub mod frc_lower01_1;
#[doc = "FRC_LOWER10_0 (rw) register accessor: FRC lookup table config register100\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frc_lower10_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frc_lower10_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frc_lower10_0`]
module"]
#[doc(alias = "FRC_LOWER10_0")]
pub type FrcLower10_0 = crate::Reg<frc_lower10_0::FrcLower10_0Spec>;
#[doc = "FRC lookup table config register100"]
pub mod frc_lower10_0;
#[doc = "FRC_LOWER10_1 (rw) register accessor: FRC lookup table config register101\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frc_lower10_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frc_lower10_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frc_lower10_1`]
module"]
#[doc(alias = "FRC_LOWER10_1")]
pub type FrcLower10_1 = crate::Reg<frc_lower10_1::FrcLower10_1Spec>;
#[doc = "FRC lookup table config register101"]
pub mod frc_lower10_1;
#[doc = "FRC_LOWER11_0 (rw) register accessor: FRC lookup table config register110\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frc_lower11_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frc_lower11_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frc_lower11_0`]
module"]
#[doc(alias = "FRC_LOWER11_0")]
pub type FrcLower11_0 = crate::Reg<frc_lower11_0::FrcLower11_0Spec>;
#[doc = "FRC lookup table config register110"]
pub mod frc_lower11_0;
#[doc = "FRC_LOWER11_1 (rw) register accessor: FRC lookup table config register111\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frc_lower11_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frc_lower11_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frc_lower11_1`]
module"]
#[doc(alias = "FRC_LOWER11_1")]
pub type FrcLower11_1 = crate::Reg<frc_lower11_1::FrcLower11_1Spec>;
#[doc = "FRC lookup table config register111"]
pub mod frc_lower11_1;
#[doc = "AFBCD0_CTRL (rw) register accessor: AFBCD0 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afbcd0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afbcd0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afbcd0_ctrl`]
module"]
#[doc(alias = "AFBCD0_CTRL")]
pub type Afbcd0Ctrl = crate::Reg<afbcd0_ctrl::Afbcd0CtrlSpec>;
#[doc = "AFBCD0 control register"]
pub mod afbcd0_ctrl;
#[doc = "AFBCD0_HDR_PTR (rw) register accessor: AFBCD0 memory start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afbcd0_hdr_ptr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afbcd0_hdr_ptr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afbcd0_hdr_ptr`]
module"]
#[doc(alias = "AFBCD0_HDR_PTR")]
pub type Afbcd0HdrPtr = crate::Reg<afbcd0_hdr_ptr::Afbcd0HdrPtrSpec>;
#[doc = "AFBCD0 memory start address"]
pub mod afbcd0_hdr_ptr;
#[doc = "AFBCD0_PIC_SIZE (rw) register accessor: AFBCD0 pic size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afbcd0_pic_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afbcd0_pic_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afbcd0_pic_size`]
module"]
#[doc(alias = "AFBCD0_PIC_SIZE")]
pub type Afbcd0PicSize = crate::Reg<afbcd0_pic_size::Afbcd0PicSizeSpec>;
#[doc = "AFBCD0 pic size"]
pub mod afbcd0_pic_size;
#[doc = "AFBCD0_STATUS (rw) register accessor: AFBCD0 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afbcd0_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afbcd0_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afbcd0_status`]
module"]
#[doc(alias = "AFBCD0_STATUS")]
pub type Afbcd0Status = crate::Reg<afbcd0_status::Afbcd0StatusSpec>;
#[doc = "AFBCD0 status"]
pub mod afbcd0_status;
#[doc = "INTR_EN0 (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_en0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_en0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_en0`]
module"]
#[doc(alias = "INTR_EN0")]
pub type IntrEn0 = crate::Reg<intr_en0::IntrEn0Spec>;
#[doc = "Interrupt enable register"]
pub mod intr_en0;
#[doc = "INTR_CLEAR0 (rw) register accessor: Interrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_clear0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_clear0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_clear0`]
module"]
#[doc(alias = "INTR_CLEAR0")]
pub type IntrClear0 = crate::Reg<intr_clear0::IntrClear0Spec>;
#[doc = "Interrupt clear register"]
pub mod intr_clear0;
#[doc = "INTR_STATUS0 (rw) register accessor: interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_status0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_status0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_status0`]
module"]
#[doc(alias = "INTR_STATUS0")]
pub type IntrStatus0 = crate::Reg<intr_status0::IntrStatus0Spec>;
#[doc = "interrupt status"]
pub mod intr_status0;
#[doc = "INTR_RAW_STATUS0 (r) register accessor: raw interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_raw_status0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_raw_status0`]
module"]
#[doc(alias = "INTR_RAW_STATUS0")]
pub type IntrRawStatus0 = crate::Reg<intr_raw_status0::IntrRawStatus0Spec>;
#[doc = "raw interrupt status"]
pub mod intr_raw_status0;
#[doc = "INTR_EN1 (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_en1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_en1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_en1`]
module"]
#[doc(alias = "INTR_EN1")]
pub type IntrEn1 = crate::Reg<intr_en1::IntrEn1Spec>;
#[doc = "Interrupt enable register"]
pub mod intr_en1;
#[doc = "INTR_CLEAR1 (rw) register accessor: Interrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_clear1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_clear1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_clear1`]
module"]
#[doc(alias = "INTR_CLEAR1")]
pub type IntrClear1 = crate::Reg<intr_clear1::IntrClear1Spec>;
#[doc = "Interrupt clear register"]
pub mod intr_clear1;
#[doc = "INTR_STATUS1 (rw) register accessor: interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_status1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_status1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_status1`]
module"]
#[doc(alias = "INTR_STATUS1")]
pub type IntrStatus1 = crate::Reg<intr_status1::IntrStatus1Spec>;
#[doc = "interrupt status"]
pub mod intr_status1;
#[doc = "INTR_RAW_STATUS1 (rw) register accessor: raw interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_raw_status1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_raw_status1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_raw_status1`]
module"]
#[doc(alias = "INTR_RAW_STATUS1")]
pub type IntrRawStatus1 = crate::Reg<intr_raw_status1::IntrRawStatus1Spec>;
#[doc = "raw interrupt status"]
pub mod intr_raw_status1;
#[doc = "LINE_FLAG (rw) register accessor: Line flag config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`line_flag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`line_flag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@line_flag`]
module"]
#[doc(alias = "LINE_FLAG")]
pub type LineFlag = crate::Reg<line_flag::LineFlagSpec>;
#[doc = "Line flag config register"]
pub mod line_flag;
#[doc = "VOP_STATUS (rw) register accessor: vop status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vop_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vop_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vop_status`]
module"]
#[doc(alias = "VOP_STATUS")]
pub type VopStatus = crate::Reg<vop_status::VopStatusSpec>;
#[doc = "vop status register"]
pub mod vop_status;
#[doc = "BLANKING_VALUE (rw) register accessor: Register0000 Abstract\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blanking_value::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blanking_value::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blanking_value`]
module"]
#[doc(alias = "BLANKING_VALUE")]
pub type BlankingValue = crate::Reg<blanking_value::BlankingValueSpec>;
#[doc = "Register0000 Abstract"]
pub mod blanking_value;
#[doc = "MCU_BYPASS_PORT (rw) register accessor: MCU bypass port\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcu_bypass_port::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcu_bypass_port::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcu_bypass_port`]
module"]
#[doc(alias = "MCU_BYPASS_PORT")]
pub type McuBypassPort = crate::Reg<mcu_bypass_port::McuBypassPortSpec>;
#[doc = "MCU bypass port"]
pub mod mcu_bypass_port;
#[doc = "WIN0_DSP_BG (rw) register accessor: Win0 layer background color\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_dsp_bg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_dsp_bg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_dsp_bg`]
module"]
#[doc(alias = "WIN0_DSP_BG")]
pub type Win0DspBg = crate::Reg<win0_dsp_bg::Win0DspBgSpec>;
#[doc = "Win0 layer background color"]
pub mod win0_dsp_bg;
#[doc = "WIN1_DSP_BG (rw) register accessor: Win1 layer background color\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_dsp_bg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_dsp_bg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_dsp_bg`]
module"]
#[doc(alias = "WIN1_DSP_BG")]
pub type Win1DspBg = crate::Reg<win1_dsp_bg::Win1DspBgSpec>;
#[doc = "Win1 layer background color"]
pub mod win1_dsp_bg;
#[doc = "WIN2_DSP_BG (rw) register accessor: Win2 layer background color\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_dsp_bg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_dsp_bg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_dsp_bg`]
module"]
#[doc(alias = "WIN2_DSP_BG")]
pub type Win2DspBg = crate::Reg<win2_dsp_bg::Win2DspBgSpec>;
#[doc = "Win2 layer background color"]
pub mod win2_dsp_bg;
#[doc = "WIN3_DSP_BG (rw) register accessor: Win3 layer background color\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_dsp_bg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_dsp_bg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_dsp_bg`]
module"]
#[doc(alias = "WIN3_DSP_BG")]
pub type Win3DspBg = crate::Reg<win3_dsp_bg::Win3DspBgSpec>;
#[doc = "Win3 layer background color"]
pub mod win3_dsp_bg;
#[doc = "YUV2YUV_WIN (rw) register accessor: win yuv2yuv control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`yuv2yuv_win::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`yuv2yuv_win::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@yuv2yuv_win`]
module"]
#[doc(alias = "YUV2YUV_WIN")]
pub type Yuv2yuvWin = crate::Reg<yuv2yuv_win::Yuv2yuvWinSpec>;
#[doc = "win yuv2yuv control register"]
pub mod yuv2yuv_win;
#[doc = "AUTO_GATING_EN (rw) register accessor: Auto gating enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auto_gating_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auto_gating_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@auto_gating_en`]
module"]
#[doc(alias = "AUTO_GATING_EN")]
pub type AutoGatingEn = crate::Reg<auto_gating_en::AutoGatingEnSpec>;
#[doc = "Auto gating enable"]
pub mod auto_gating_en;
#[doc = "WIN0_YUV2YUV_Y2R_COE0 (rw) register accessor: WIN0 yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_yuv2yuv_y2r_coe0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_yuv2yuv_y2r_coe0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_yuv2yuv_y2r_coe0`]
module"]
#[doc(alias = "WIN0_YUV2YUV_Y2R_COE0")]
pub type Win0Yuv2yuvY2rCoe0 = crate::Reg<win0_yuv2yuv_y2r_coe0::Win0Yuv2yuvY2rCoe0Spec>;
#[doc = "WIN0 yuv2yuv y2r cofficient"]
pub mod win0_yuv2yuv_y2r_coe0;
#[doc = "WIN0_YUV2YUV_Y2R_COE1 (rw) register accessor: WIN0 yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_yuv2yuv_y2r_coe1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_yuv2yuv_y2r_coe1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_yuv2yuv_y2r_coe1`]
module"]
#[doc(alias = "WIN0_YUV2YUV_Y2R_COE1")]
pub type Win0Yuv2yuvY2rCoe1 = crate::Reg<win0_yuv2yuv_y2r_coe1::Win0Yuv2yuvY2rCoe1Spec>;
#[doc = "WIN0 yuv2yuv y2r cofficient"]
pub mod win0_yuv2yuv_y2r_coe1;
#[doc = "WIN0_YUV2YUV_Y2R_COE2 (rw) register accessor: WIN0 yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_yuv2yuv_y2r_coe2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_yuv2yuv_y2r_coe2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_yuv2yuv_y2r_coe2`]
module"]
#[doc(alias = "WIN0_YUV2YUV_Y2R_COE2")]
pub type Win0Yuv2yuvY2rCoe2 = crate::Reg<win0_yuv2yuv_y2r_coe2::Win0Yuv2yuvY2rCoe2Spec>;
#[doc = "WIN0 yuv2yuv y2r cofficient"]
pub mod win0_yuv2yuv_y2r_coe2;
#[doc = "WIN0_YUV2YUV_Y2R_COE3 (rw) register accessor: WIN0yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_yuv2yuv_y2r_coe3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_yuv2yuv_y2r_coe3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_yuv2yuv_y2r_coe3`]
module"]
#[doc(alias = "WIN0_YUV2YUV_Y2R_COE3")]
pub type Win0Yuv2yuvY2rCoe3 = crate::Reg<win0_yuv2yuv_y2r_coe3::Win0Yuv2yuvY2rCoe3Spec>;
#[doc = "WIN0yuv2yuv y2r cofficient"]
pub mod win0_yuv2yuv_y2r_coe3;
#[doc = "WIN0_YUV2YUV_Y2R_COE4 (rw) register accessor: WIN0 yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_yuv2yuv_y2r_coe4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_yuv2yuv_y2r_coe4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_yuv2yuv_y2r_coe4`]
module"]
#[doc(alias = "WIN0_YUV2YUV_Y2R_COE4")]
pub type Win0Yuv2yuvY2rCoe4 = crate::Reg<win0_yuv2yuv_y2r_coe4::Win0Yuv2yuvY2rCoe4Spec>;
#[doc = "WIN0 yuv2yuv y2r cofficient"]
pub mod win0_yuv2yuv_y2r_coe4;
#[doc = "WIN0_YUV2YUV_Y2R_COE5 (rw) register accessor: WIN0 yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_yuv2yuv_y2r_coe5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_yuv2yuv_y2r_coe5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_yuv2yuv_y2r_coe5`]
module"]
#[doc(alias = "WIN0_YUV2YUV_Y2R_COE5")]
pub type Win0Yuv2yuvY2rCoe5 = crate::Reg<win0_yuv2yuv_y2r_coe5::Win0Yuv2yuvY2rCoe5Spec>;
#[doc = "WIN0 yuv2yuv y2r cofficient"]
pub mod win0_yuv2yuv_y2r_coe5;
#[doc = "WIN0_YUV2YUV_Y2R_COE6 (rw) register accessor: WIN0 yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_yuv2yuv_y2r_coe6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_yuv2yuv_y2r_coe6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_yuv2yuv_y2r_coe6`]
module"]
#[doc(alias = "WIN0_YUV2YUV_Y2R_COE6")]
pub type Win0Yuv2yuvY2rCoe6 = crate::Reg<win0_yuv2yuv_y2r_coe6::Win0Yuv2yuvY2rCoe6Spec>;
#[doc = "WIN0 yuv2yuv y2r cofficient"]
pub mod win0_yuv2yuv_y2r_coe6;
#[doc = "WIN0_YUV2YUV_Y2R_COE7 (rw) register accessor: WIN0 yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_yuv2yuv_y2r_coe7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_yuv2yuv_y2r_coe7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_yuv2yuv_y2r_coe7`]
module"]
#[doc(alias = "WIN0_YUV2YUV_Y2R_COE7")]
pub type Win0Yuv2yuvY2rCoe7 = crate::Reg<win0_yuv2yuv_y2r_coe7::Win0Yuv2yuvY2rCoe7Spec>;
#[doc = "WIN0 yuv2yuv y2r cofficient"]
pub mod win0_yuv2yuv_y2r_coe7;
#[doc = "WIN0_YUV2YUV_R2R_COE0 (rw) register accessor: WIN0 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_yuv2yuv_r2r_coe0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_yuv2yuv_r2r_coe0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_yuv2yuv_r2r_coe0`]
module"]
#[doc(alias = "WIN0_YUV2YUV_R2R_COE0")]
pub type Win0Yuv2yuvR2rCoe0 = crate::Reg<win0_yuv2yuv_r2r_coe0::Win0Yuv2yuvR2rCoe0Spec>;
#[doc = "WIN0 yuv2yuv r2r cofficient"]
pub mod win0_yuv2yuv_r2r_coe0;
#[doc = "WIN0_YUV2YUV_R2R_COE1 (rw) register accessor: WIN0 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_yuv2yuv_r2r_coe1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_yuv2yuv_r2r_coe1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_yuv2yuv_r2r_coe1`]
module"]
#[doc(alias = "WIN0_YUV2YUV_R2R_COE1")]
pub type Win0Yuv2yuvR2rCoe1 = crate::Reg<win0_yuv2yuv_r2r_coe1::Win0Yuv2yuvR2rCoe1Spec>;
#[doc = "WIN0 yuv2yuv r2r cofficient"]
pub mod win0_yuv2yuv_r2r_coe1;
#[doc = "WIN0_YUV2YUV_R2R_COE2 (rw) register accessor: WIN0 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_yuv2yuv_r2r_coe2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_yuv2yuv_r2r_coe2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_yuv2yuv_r2r_coe2`]
module"]
#[doc(alias = "WIN0_YUV2YUV_R2R_COE2")]
pub type Win0Yuv2yuvR2rCoe2 = crate::Reg<win0_yuv2yuv_r2r_coe2::Win0Yuv2yuvR2rCoe2Spec>;
#[doc = "WIN0 yuv2yuv r2r cofficient"]
pub mod win0_yuv2yuv_r2r_coe2;
#[doc = "WIN0_YUV2YUV_R2R_COE3 (rw) register accessor: WIN0 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_yuv2yuv_r2r_coe3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_yuv2yuv_r2r_coe3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_yuv2yuv_r2r_coe3`]
module"]
#[doc(alias = "WIN0_YUV2YUV_R2R_COE3")]
pub type Win0Yuv2yuvR2rCoe3 = crate::Reg<win0_yuv2yuv_r2r_coe3::Win0Yuv2yuvR2rCoe3Spec>;
#[doc = "WIN0 yuv2yuv r2r cofficient"]
pub mod win0_yuv2yuv_r2r_coe3;
#[doc = "WIN0_YUV2YUV_R2R_COE4 (rw) register accessor: WIN0 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_yuv2yuv_r2r_coe4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_yuv2yuv_r2r_coe4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_yuv2yuv_r2r_coe4`]
module"]
#[doc(alias = "WIN0_YUV2YUV_R2R_COE4")]
pub type Win0Yuv2yuvR2rCoe4 = crate::Reg<win0_yuv2yuv_r2r_coe4::Win0Yuv2yuvR2rCoe4Spec>;
#[doc = "WIN0 yuv2yuv r2r cofficient"]
pub mod win0_yuv2yuv_r2r_coe4;
#[doc = "WIN0_YUV2YUV_R2R_COE5 (rw) register accessor: WIN0 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_yuv2yuv_r2r_coe5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_yuv2yuv_r2r_coe5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_yuv2yuv_r2r_coe5`]
module"]
#[doc(alias = "WIN0_YUV2YUV_R2R_COE5")]
pub type Win0Yuv2yuvR2rCoe5 = crate::Reg<win0_yuv2yuv_r2r_coe5::Win0Yuv2yuvR2rCoe5Spec>;
#[doc = "WIN0 yuv2yuv r2r cofficient"]
pub mod win0_yuv2yuv_r2r_coe5;
#[doc = "WIN0_YUV2YUV_R2R_COE6 (rw) register accessor: WIN0 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_yuv2yuv_r2r_coe6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_yuv2yuv_r2r_coe6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_yuv2yuv_r2r_coe6`]
module"]
#[doc(alias = "WIN0_YUV2YUV_R2R_COE6")]
pub type Win0Yuv2yuvR2rCoe6 = crate::Reg<win0_yuv2yuv_r2r_coe6::Win0Yuv2yuvR2rCoe6Spec>;
#[doc = "WIN0 yuv2yuv r2r cofficient"]
pub mod win0_yuv2yuv_r2r_coe6;
#[doc = "WIN0_YUV2YUV_R2R_COE7 (rw) register accessor: WIN0 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_yuv2yuv_r2r_coe7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_yuv2yuv_r2r_coe7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_yuv2yuv_r2r_coe7`]
module"]
#[doc(alias = "WIN0_YUV2YUV_R2R_COE7")]
pub type Win0Yuv2yuvR2rCoe7 = crate::Reg<win0_yuv2yuv_r2r_coe7::Win0Yuv2yuvR2rCoe7Spec>;
#[doc = "WIN0 yuv2yuv r2r cofficient"]
pub mod win0_yuv2yuv_r2r_coe7;
#[doc = "WIN0_YUV2YUV_R2Y_COE0 (rw) register accessor: WIN0 yuv2yuv r2y cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_yuv2yuv_r2y_coe0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_yuv2yuv_r2y_coe0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_yuv2yuv_r2y_coe0`]
module"]
#[doc(alias = "WIN0_YUV2YUV_R2Y_COE0")]
pub type Win0Yuv2yuvR2yCoe0 = crate::Reg<win0_yuv2yuv_r2y_coe0::Win0Yuv2yuvR2yCoe0Spec>;
#[doc = "WIN0 yuv2yuv r2y cofficient"]
pub mod win0_yuv2yuv_r2y_coe0;
#[doc = "WIN0_YUV2YUV_R2Y_COE1 (rw) register accessor: WIN0 yuv2yuv r2y cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_yuv2yuv_r2y_coe1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_yuv2yuv_r2y_coe1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_yuv2yuv_r2y_coe1`]
module"]
#[doc(alias = "WIN0_YUV2YUV_R2Y_COE1")]
pub type Win0Yuv2yuvR2yCoe1 = crate::Reg<win0_yuv2yuv_r2y_coe1::Win0Yuv2yuvR2yCoe1Spec>;
#[doc = "WIN0 yuv2yuv r2y cofficient"]
pub mod win0_yuv2yuv_r2y_coe1;
#[doc = "WIN0_YUV2YUV_R2Y_COE2 (rw) register accessor: WIN0 yuv2yuv r2y cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_yuv2yuv_r2y_coe2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_yuv2yuv_r2y_coe2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_yuv2yuv_r2y_coe2`]
module"]
#[doc(alias = "WIN0_YUV2YUV_R2Y_COE2")]
pub type Win0Yuv2yuvR2yCoe2 = crate::Reg<win0_yuv2yuv_r2y_coe2::Win0Yuv2yuvR2yCoe2Spec>;
#[doc = "WIN0 yuv2yuv r2y cofficient"]
pub mod win0_yuv2yuv_r2y_coe2;
#[doc = "WIN0_YUV2YUV_R2Y_COE3 (rw) register accessor: WIN0 yuv2yuv r2y cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_yuv2yuv_r2y_coe3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_yuv2yuv_r2y_coe3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_yuv2yuv_r2y_coe3`]
module"]
#[doc(alias = "WIN0_YUV2YUV_R2Y_COE3")]
pub type Win0Yuv2yuvR2yCoe3 = crate::Reg<win0_yuv2yuv_r2y_coe3::Win0Yuv2yuvR2yCoe3Spec>;
#[doc = "WIN0 yuv2yuv r2y cofficient"]
pub mod win0_yuv2yuv_r2y_coe3;
#[doc = "WIN0_YUV2YUV_R2Y_COE4 (rw) register accessor: WIN0 yuv2yuv r2y cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_yuv2yuv_r2y_coe4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_yuv2yuv_r2y_coe4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_yuv2yuv_r2y_coe4`]
module"]
#[doc(alias = "WIN0_YUV2YUV_R2Y_COE4")]
pub type Win0Yuv2yuvR2yCoe4 = crate::Reg<win0_yuv2yuv_r2y_coe4::Win0Yuv2yuvR2yCoe4Spec>;
#[doc = "WIN0 yuv2yuv r2y cofficient"]
pub mod win0_yuv2yuv_r2y_coe4;
#[doc = "WIN0_YUV2YUV_R2Y_COE5 (rw) register accessor: WIN0 yuv2yuv r2y cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_yuv2yuv_r2y_coe5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_yuv2yuv_r2y_coe5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_yuv2yuv_r2y_coe5`]
module"]
#[doc(alias = "WIN0_YUV2YUV_R2Y_COE5")]
pub type Win0Yuv2yuvR2yCoe5 = crate::Reg<win0_yuv2yuv_r2y_coe5::Win0Yuv2yuvR2yCoe5Spec>;
#[doc = "WIN0 yuv2yuv r2y cofficient"]
pub mod win0_yuv2yuv_r2y_coe5;
#[doc = "WIN0_YUV2YUV_R2Y_COE6 (rw) register accessor: WIN0 yuv2yuv r2y cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_yuv2yuv_r2y_coe6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_yuv2yuv_r2y_coe6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_yuv2yuv_r2y_coe6`]
module"]
#[doc(alias = "WIN0_YUV2YUV_R2Y_COE6")]
pub type Win0Yuv2yuvR2yCoe6 = crate::Reg<win0_yuv2yuv_r2y_coe6::Win0Yuv2yuvR2yCoe6Spec>;
#[doc = "WIN0 yuv2yuv r2y cofficient"]
pub mod win0_yuv2yuv_r2y_coe6;
#[doc = "WIN0_YUV2YUV_R2Y_COE7 (rw) register accessor: WIN0 yuv2yuv r2y cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_yuv2yuv_r2y_coe7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_yuv2yuv_r2y_coe7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win0_yuv2yuv_r2y_coe7`]
module"]
#[doc(alias = "WIN0_YUV2YUV_R2Y_COE7")]
pub type Win0Yuv2yuvR2yCoe7 = crate::Reg<win0_yuv2yuv_r2y_coe7::Win0Yuv2yuvR2yCoe7Spec>;
#[doc = "WIN0 yuv2yuv r2y cofficient"]
pub mod win0_yuv2yuv_r2y_coe7;
#[doc = "WIN1_YUV2YUV_Y2R_COE0 (rw) register accessor: WIN1 yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_yuv2yuv_y2r_coe0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_yuv2yuv_y2r_coe0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_yuv2yuv_y2r_coe0`]
module"]
#[doc(alias = "WIN1_YUV2YUV_Y2R_COE0")]
pub type Win1Yuv2yuvY2rCoe0 = crate::Reg<win1_yuv2yuv_y2r_coe0::Win1Yuv2yuvY2rCoe0Spec>;
#[doc = "WIN1 yuv2yuv y2r cofficient"]
pub mod win1_yuv2yuv_y2r_coe0;
#[doc = "WIN1_YUV2YUV_Y2R_COE1 (rw) register accessor: WIN1 yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_yuv2yuv_y2r_coe1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_yuv2yuv_y2r_coe1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_yuv2yuv_y2r_coe1`]
module"]
#[doc(alias = "WIN1_YUV2YUV_Y2R_COE1")]
pub type Win1Yuv2yuvY2rCoe1 = crate::Reg<win1_yuv2yuv_y2r_coe1::Win1Yuv2yuvY2rCoe1Spec>;
#[doc = "WIN1 yuv2yuv y2r cofficient"]
pub mod win1_yuv2yuv_y2r_coe1;
#[doc = "WIN1_YUV2YUV_Y2R_COE2 (rw) register accessor: WIN1 yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_yuv2yuv_y2r_coe2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_yuv2yuv_y2r_coe2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_yuv2yuv_y2r_coe2`]
module"]
#[doc(alias = "WIN1_YUV2YUV_Y2R_COE2")]
pub type Win1Yuv2yuvY2rCoe2 = crate::Reg<win1_yuv2yuv_y2r_coe2::Win1Yuv2yuvY2rCoe2Spec>;
#[doc = "WIN1 yuv2yuv y2r cofficient"]
pub mod win1_yuv2yuv_y2r_coe2;
#[doc = "WIN1_YUV2YUV_Y2R_COE3 (rw) register accessor: WIN1 yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_yuv2yuv_y2r_coe3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_yuv2yuv_y2r_coe3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_yuv2yuv_y2r_coe3`]
module"]
#[doc(alias = "WIN1_YUV2YUV_Y2R_COE3")]
pub type Win1Yuv2yuvY2rCoe3 = crate::Reg<win1_yuv2yuv_y2r_coe3::Win1Yuv2yuvY2rCoe3Spec>;
#[doc = "WIN1 yuv2yuv y2r cofficient"]
pub mod win1_yuv2yuv_y2r_coe3;
#[doc = "WIN1_YUV2YUV_Y2R_COE4 (rw) register accessor: WIN1 yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_yuv2yuv_y2r_coe4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_yuv2yuv_y2r_coe4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_yuv2yuv_y2r_coe4`]
module"]
#[doc(alias = "WIN1_YUV2YUV_Y2R_COE4")]
pub type Win1Yuv2yuvY2rCoe4 = crate::Reg<win1_yuv2yuv_y2r_coe4::Win1Yuv2yuvY2rCoe4Spec>;
#[doc = "WIN1 yuv2yuv y2r cofficient"]
pub mod win1_yuv2yuv_y2r_coe4;
#[doc = "WIN1_YUV2YUV_Y2R_COE5 (rw) register accessor: WIN1 yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_yuv2yuv_y2r_coe5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_yuv2yuv_y2r_coe5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_yuv2yuv_y2r_coe5`]
module"]
#[doc(alias = "WIN1_YUV2YUV_Y2R_COE5")]
pub type Win1Yuv2yuvY2rCoe5 = crate::Reg<win1_yuv2yuv_y2r_coe5::Win1Yuv2yuvY2rCoe5Spec>;
#[doc = "WIN1 yuv2yuv y2r cofficient"]
pub mod win1_yuv2yuv_y2r_coe5;
#[doc = "WIN1_YUV2YUV_Y2R_COE6 (rw) register accessor: WIN1 yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_yuv2yuv_y2r_coe6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_yuv2yuv_y2r_coe6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_yuv2yuv_y2r_coe6`]
module"]
#[doc(alias = "WIN1_YUV2YUV_Y2R_COE6")]
pub type Win1Yuv2yuvY2rCoe6 = crate::Reg<win1_yuv2yuv_y2r_coe6::Win1Yuv2yuvY2rCoe6Spec>;
#[doc = "WIN1 yuv2yuv y2r cofficient"]
pub mod win1_yuv2yuv_y2r_coe6;
#[doc = "WIN1_YUV2YUV_Y2R_COE7 (rw) register accessor: WIN1 yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_yuv2yuv_y2r_coe7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_yuv2yuv_y2r_coe7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_yuv2yuv_y2r_coe7`]
module"]
#[doc(alias = "WIN1_YUV2YUV_Y2R_COE7")]
pub type Win1Yuv2yuvY2rCoe7 = crate::Reg<win1_yuv2yuv_y2r_coe7::Win1Yuv2yuvY2rCoe7Spec>;
#[doc = "WIN1 yuv2yuv y2r cofficient"]
pub mod win1_yuv2yuv_y2r_coe7;
#[doc = "WIN1_YUV2YUV_R2R_COE0 (rw) register accessor: WIN0 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_yuv2yuv_r2r_coe0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_yuv2yuv_r2r_coe0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_yuv2yuv_r2r_coe0`]
module"]
#[doc(alias = "WIN1_YUV2YUV_R2R_COE0")]
pub type Win1Yuv2yuvR2rCoe0 = crate::Reg<win1_yuv2yuv_r2r_coe0::Win1Yuv2yuvR2rCoe0Spec>;
#[doc = "WIN0 yuv2yuv r2r cofficient"]
pub mod win1_yuv2yuv_r2r_coe0;
#[doc = "WIN1_YUV2YUV_R2R_COE1 (rw) register accessor: WIN1 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_yuv2yuv_r2r_coe1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_yuv2yuv_r2r_coe1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_yuv2yuv_r2r_coe1`]
module"]
#[doc(alias = "WIN1_YUV2YUV_R2R_COE1")]
pub type Win1Yuv2yuvR2rCoe1 = crate::Reg<win1_yuv2yuv_r2r_coe1::Win1Yuv2yuvR2rCoe1Spec>;
#[doc = "WIN1 yuv2yuv r2r cofficient"]
pub mod win1_yuv2yuv_r2r_coe1;
#[doc = "WIN1_YUV2YUV_R2R_COE2 (rw) register accessor: WIN1 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_yuv2yuv_r2r_coe2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_yuv2yuv_r2r_coe2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_yuv2yuv_r2r_coe2`]
module"]
#[doc(alias = "WIN1_YUV2YUV_R2R_COE2")]
pub type Win1Yuv2yuvR2rCoe2 = crate::Reg<win1_yuv2yuv_r2r_coe2::Win1Yuv2yuvR2rCoe2Spec>;
#[doc = "WIN1 yuv2yuv r2r cofficient"]
pub mod win1_yuv2yuv_r2r_coe2;
#[doc = "WIN1_YUV2YUV_R2R_COE3 (rw) register accessor: WIN1 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_yuv2yuv_r2r_coe3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_yuv2yuv_r2r_coe3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_yuv2yuv_r2r_coe3`]
module"]
#[doc(alias = "WIN1_YUV2YUV_R2R_COE3")]
pub type Win1Yuv2yuvR2rCoe3 = crate::Reg<win1_yuv2yuv_r2r_coe3::Win1Yuv2yuvR2rCoe3Spec>;
#[doc = "WIN1 yuv2yuv r2r cofficient"]
pub mod win1_yuv2yuv_r2r_coe3;
#[doc = "WIN1_YUV2YUV_R2R_COE4 (rw) register accessor: WIN1 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_yuv2yuv_r2r_coe4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_yuv2yuv_r2r_coe4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_yuv2yuv_r2r_coe4`]
module"]
#[doc(alias = "WIN1_YUV2YUV_R2R_COE4")]
pub type Win1Yuv2yuvR2rCoe4 = crate::Reg<win1_yuv2yuv_r2r_coe4::Win1Yuv2yuvR2rCoe4Spec>;
#[doc = "WIN1 yuv2yuv r2r cofficient"]
pub mod win1_yuv2yuv_r2r_coe4;
#[doc = "WIN1_YUV2YUV_R2R_COE5 (rw) register accessor: WIN1 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_yuv2yuv_r2r_coe5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_yuv2yuv_r2r_coe5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_yuv2yuv_r2r_coe5`]
module"]
#[doc(alias = "WIN1_YUV2YUV_R2R_COE5")]
pub type Win1Yuv2yuvR2rCoe5 = crate::Reg<win1_yuv2yuv_r2r_coe5::Win1Yuv2yuvR2rCoe5Spec>;
#[doc = "WIN1 yuv2yuv r2r cofficient"]
pub mod win1_yuv2yuv_r2r_coe5;
#[doc = "WIN1_YUV2YUV_R2R_COE6 (rw) register accessor: WIN1 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_yuv2yuv_r2r_coe6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_yuv2yuv_r2r_coe6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_yuv2yuv_r2r_coe6`]
module"]
#[doc(alias = "WIN1_YUV2YUV_R2R_COE6")]
pub type Win1Yuv2yuvR2rCoe6 = crate::Reg<win1_yuv2yuv_r2r_coe6::Win1Yuv2yuvR2rCoe6Spec>;
#[doc = "WIN1 yuv2yuv r2r cofficient"]
pub mod win1_yuv2yuv_r2r_coe6;
#[doc = "WIN1_YUV2YUV_R2R_COE7 (rw) register accessor: WIN1 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_yuv2yuv_r2r_coe7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_yuv2yuv_r2r_coe7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_yuv2yuv_r2r_coe7`]
module"]
#[doc(alias = "WIN1_YUV2YUV_R2R_COE7")]
pub type Win1Yuv2yuvR2rCoe7 = crate::Reg<win1_yuv2yuv_r2r_coe7::Win1Yuv2yuvR2rCoe7Spec>;
#[doc = "WIN1 yuv2yuv r2r cofficient"]
pub mod win1_yuv2yuv_r2r_coe7;
#[doc = "WIN1_YUV2YUV_R2Y_COE0 (rw) register accessor: WIN1 yuv2yuv r2y cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_yuv2yuv_r2y_coe0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_yuv2yuv_r2y_coe0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_yuv2yuv_r2y_coe0`]
module"]
#[doc(alias = "WIN1_YUV2YUV_R2Y_COE0")]
pub type Win1Yuv2yuvR2yCoe0 = crate::Reg<win1_yuv2yuv_r2y_coe0::Win1Yuv2yuvR2yCoe0Spec>;
#[doc = "WIN1 yuv2yuv r2y cofficient"]
pub mod win1_yuv2yuv_r2y_coe0;
#[doc = "WIN1_YUV2YUV_R2Y_COE1 (rw) register accessor: WIN1 yuv2yuv r2y cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_yuv2yuv_r2y_coe1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_yuv2yuv_r2y_coe1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_yuv2yuv_r2y_coe1`]
module"]
#[doc(alias = "WIN1_YUV2YUV_R2Y_COE1")]
pub type Win1Yuv2yuvR2yCoe1 = crate::Reg<win1_yuv2yuv_r2y_coe1::Win1Yuv2yuvR2yCoe1Spec>;
#[doc = "WIN1 yuv2yuv r2y cofficient"]
pub mod win1_yuv2yuv_r2y_coe1;
#[doc = "WIN1_YUV2YUV_R2Y_COE2 (rw) register accessor: WIN1 yuv2yuv r2y cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_yuv2yuv_r2y_coe2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_yuv2yuv_r2y_coe2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_yuv2yuv_r2y_coe2`]
module"]
#[doc(alias = "WIN1_YUV2YUV_R2Y_COE2")]
pub type Win1Yuv2yuvR2yCoe2 = crate::Reg<win1_yuv2yuv_r2y_coe2::Win1Yuv2yuvR2yCoe2Spec>;
#[doc = "WIN1 yuv2yuv r2y cofficient"]
pub mod win1_yuv2yuv_r2y_coe2;
#[doc = "WIN1_YUV2YUV_R2Y_COE3 (rw) register accessor: WIN1 yuv2yuv r2y cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_yuv2yuv_r2y_coe3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_yuv2yuv_r2y_coe3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_yuv2yuv_r2y_coe3`]
module"]
#[doc(alias = "WIN1_YUV2YUV_R2Y_COE3")]
pub type Win1Yuv2yuvR2yCoe3 = crate::Reg<win1_yuv2yuv_r2y_coe3::Win1Yuv2yuvR2yCoe3Spec>;
#[doc = "WIN1 yuv2yuv r2y cofficient"]
pub mod win1_yuv2yuv_r2y_coe3;
#[doc = "WIN1_YUV2YUV_R2Y_COE4 (rw) register accessor: WIN1 yuv2yuv r2y cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_yuv2yuv_r2y_coe4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_yuv2yuv_r2y_coe4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_yuv2yuv_r2y_coe4`]
module"]
#[doc(alias = "WIN1_YUV2YUV_R2Y_COE4")]
pub type Win1Yuv2yuvR2yCoe4 = crate::Reg<win1_yuv2yuv_r2y_coe4::Win1Yuv2yuvR2yCoe4Spec>;
#[doc = "WIN1 yuv2yuv r2y cofficient"]
pub mod win1_yuv2yuv_r2y_coe4;
#[doc = "WIN1_YUV2YUV_R2Y_COE5 (rw) register accessor: WIN1 yuv2yuv r2y cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_yuv2yuv_r2y_coe5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_yuv2yuv_r2y_coe5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_yuv2yuv_r2y_coe5`]
module"]
#[doc(alias = "WIN1_YUV2YUV_R2Y_COE5")]
pub type Win1Yuv2yuvR2yCoe5 = crate::Reg<win1_yuv2yuv_r2y_coe5::Win1Yuv2yuvR2yCoe5Spec>;
#[doc = "WIN1 yuv2yuv r2y cofficient"]
pub mod win1_yuv2yuv_r2y_coe5;
#[doc = "WIN1_YUV2YUV_R2Y_COE6 (rw) register accessor: WIN1 yuv2yuv r2y cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_yuv2yuv_r2y_coe6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_yuv2yuv_r2y_coe6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_yuv2yuv_r2y_coe6`]
module"]
#[doc(alias = "WIN1_YUV2YUV_R2Y_COE6")]
pub type Win1Yuv2yuvR2yCoe6 = crate::Reg<win1_yuv2yuv_r2y_coe6::Win1Yuv2yuvR2yCoe6Spec>;
#[doc = "WIN1 yuv2yuv r2y cofficient"]
pub mod win1_yuv2yuv_r2y_coe6;
#[doc = "WIN1_YUV2YUV_R2Y_COE7 (rw) register accessor: WIN1 yuv2yuv r2y cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_yuv2yuv_r2y_coe7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_yuv2yuv_r2y_coe7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_yuv2yuv_r2y_coe7`]
module"]
#[doc(alias = "WIN1_YUV2YUV_R2Y_COE7")]
pub type Win1Yuv2yuvR2yCoe7 = crate::Reg<win1_yuv2yuv_r2y_coe7::Win1Yuv2yuvR2yCoe7Spec>;
#[doc = "WIN1 yuv2yuv r2y cofficient"]
pub mod win1_yuv2yuv_r2y_coe7;
#[doc = "WIN2_YUV2YUV_Y2R_COE0 (rw) register accessor: WIN2 yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_yuv2yuv_y2r_coe0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_yuv2yuv_y2r_coe0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_yuv2yuv_y2r_coe0`]
module"]
#[doc(alias = "WIN2_YUV2YUV_Y2R_COE0")]
pub type Win2Yuv2yuvY2rCoe0 = crate::Reg<win2_yuv2yuv_y2r_coe0::Win2Yuv2yuvY2rCoe0Spec>;
#[doc = "WIN2 yuv2yuv y2r cofficient"]
pub mod win2_yuv2yuv_y2r_coe0;
#[doc = "WIN2_YUV2YUV_Y2R_COE1 (rw) register accessor: WIN2 yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_yuv2yuv_y2r_coe1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_yuv2yuv_y2r_coe1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_yuv2yuv_y2r_coe1`]
module"]
#[doc(alias = "WIN2_YUV2YUV_Y2R_COE1")]
pub type Win2Yuv2yuvY2rCoe1 = crate::Reg<win2_yuv2yuv_y2r_coe1::Win2Yuv2yuvY2rCoe1Spec>;
#[doc = "WIN2 yuv2yuv y2r cofficient"]
pub mod win2_yuv2yuv_y2r_coe1;
#[doc = "WIN2_YUV2YUV_Y2R_COE2 (rw) register accessor: WIN2 yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_yuv2yuv_y2r_coe2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_yuv2yuv_y2r_coe2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_yuv2yuv_y2r_coe2`]
module"]
#[doc(alias = "WIN2_YUV2YUV_Y2R_COE2")]
pub type Win2Yuv2yuvY2rCoe2 = crate::Reg<win2_yuv2yuv_y2r_coe2::Win2Yuv2yuvY2rCoe2Spec>;
#[doc = "WIN2 yuv2yuv y2r cofficient"]
pub mod win2_yuv2yuv_y2r_coe2;
#[doc = "WIN2_YUV2YUV_Y2R_COE3 (rw) register accessor: WIN2 yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_yuv2yuv_y2r_coe3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_yuv2yuv_y2r_coe3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_yuv2yuv_y2r_coe3`]
module"]
#[doc(alias = "WIN2_YUV2YUV_Y2R_COE3")]
pub type Win2Yuv2yuvY2rCoe3 = crate::Reg<win2_yuv2yuv_y2r_coe3::Win2Yuv2yuvY2rCoe3Spec>;
#[doc = "WIN2 yuv2yuv y2r cofficient"]
pub mod win2_yuv2yuv_y2r_coe3;
#[doc = "WIN2_YUV2YUV_Y2R_COE4 (rw) register accessor: WIN2 yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_yuv2yuv_y2r_coe4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_yuv2yuv_y2r_coe4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_yuv2yuv_y2r_coe4`]
module"]
#[doc(alias = "WIN2_YUV2YUV_Y2R_COE4")]
pub type Win2Yuv2yuvY2rCoe4 = crate::Reg<win2_yuv2yuv_y2r_coe4::Win2Yuv2yuvY2rCoe4Spec>;
#[doc = "WIN2 yuv2yuv y2r cofficient"]
pub mod win2_yuv2yuv_y2r_coe4;
#[doc = "WIN2_YUV2YUV_Y2R_COE5 (rw) register accessor: WIN2 yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_yuv2yuv_y2r_coe5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_yuv2yuv_y2r_coe5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_yuv2yuv_y2r_coe5`]
module"]
#[doc(alias = "WIN2_YUV2YUV_Y2R_COE5")]
pub type Win2Yuv2yuvY2rCoe5 = crate::Reg<win2_yuv2yuv_y2r_coe5::Win2Yuv2yuvY2rCoe5Spec>;
#[doc = "WIN2 yuv2yuv y2r cofficient"]
pub mod win2_yuv2yuv_y2r_coe5;
#[doc = "WIN2_YUV2YUV_Y2R_COE6 (rw) register accessor: WIN2 yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_yuv2yuv_y2r_coe6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_yuv2yuv_y2r_coe6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_yuv2yuv_y2r_coe6`]
module"]
#[doc(alias = "WIN2_YUV2YUV_Y2R_COE6")]
pub type Win2Yuv2yuvY2rCoe6 = crate::Reg<win2_yuv2yuv_y2r_coe6::Win2Yuv2yuvY2rCoe6Spec>;
#[doc = "WIN2 yuv2yuv y2r cofficient"]
pub mod win2_yuv2yuv_y2r_coe6;
#[doc = "WIN2_YUV2YUV_Y2R_COE7 (rw) register accessor: WIN2 yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_yuv2yuv_y2r_coe7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_yuv2yuv_y2r_coe7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_yuv2yuv_y2r_coe7`]
module"]
#[doc(alias = "WIN2_YUV2YUV_Y2R_COE7")]
pub type Win2Yuv2yuvY2rCoe7 = crate::Reg<win2_yuv2yuv_y2r_coe7::Win2Yuv2yuvY2rCoe7Spec>;
#[doc = "WIN2 yuv2yuv y2r cofficient"]
pub mod win2_yuv2yuv_y2r_coe7;
#[doc = "WIN2_YUV2YUV_R2R_COE0 (rw) register accessor: WIN2 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_yuv2yuv_r2r_coe0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_yuv2yuv_r2r_coe0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_yuv2yuv_r2r_coe0`]
module"]
#[doc(alias = "WIN2_YUV2YUV_R2R_COE0")]
pub type Win2Yuv2yuvR2rCoe0 = crate::Reg<win2_yuv2yuv_r2r_coe0::Win2Yuv2yuvR2rCoe0Spec>;
#[doc = "WIN2 yuv2yuv r2r cofficient"]
pub mod win2_yuv2yuv_r2r_coe0;
#[doc = "WIN2_YUV2YUV_R2R_COE1 (rw) register accessor: WIN2 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_yuv2yuv_r2r_coe1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_yuv2yuv_r2r_coe1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_yuv2yuv_r2r_coe1`]
module"]
#[doc(alias = "WIN2_YUV2YUV_R2R_COE1")]
pub type Win2Yuv2yuvR2rCoe1 = crate::Reg<win2_yuv2yuv_r2r_coe1::Win2Yuv2yuvR2rCoe1Spec>;
#[doc = "WIN2 yuv2yuv r2r cofficient"]
pub mod win2_yuv2yuv_r2r_coe1;
#[doc = "WIN2_YUV2YUV_R2R_COE2 (rw) register accessor: WIN2 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_yuv2yuv_r2r_coe2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_yuv2yuv_r2r_coe2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_yuv2yuv_r2r_coe2`]
module"]
#[doc(alias = "WIN2_YUV2YUV_R2R_COE2")]
pub type Win2Yuv2yuvR2rCoe2 = crate::Reg<win2_yuv2yuv_r2r_coe2::Win2Yuv2yuvR2rCoe2Spec>;
#[doc = "WIN2 yuv2yuv r2r cofficient"]
pub mod win2_yuv2yuv_r2r_coe2;
#[doc = "WIN2_YUV2YUV_R2R_COE3 (rw) register accessor: WIN2 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_yuv2yuv_r2r_coe3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_yuv2yuv_r2r_coe3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_yuv2yuv_r2r_coe3`]
module"]
#[doc(alias = "WIN2_YUV2YUV_R2R_COE3")]
pub type Win2Yuv2yuvR2rCoe3 = crate::Reg<win2_yuv2yuv_r2r_coe3::Win2Yuv2yuvR2rCoe3Spec>;
#[doc = "WIN2 yuv2yuv r2r cofficient"]
pub mod win2_yuv2yuv_r2r_coe3;
#[doc = "WIN2_YUV2YUV_R2R_COE4 (rw) register accessor: WIN2 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_yuv2yuv_r2r_coe4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_yuv2yuv_r2r_coe4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_yuv2yuv_r2r_coe4`]
module"]
#[doc(alias = "WIN2_YUV2YUV_R2R_COE4")]
pub type Win2Yuv2yuvR2rCoe4 = crate::Reg<win2_yuv2yuv_r2r_coe4::Win2Yuv2yuvR2rCoe4Spec>;
#[doc = "WIN2 yuv2yuv r2r cofficient"]
pub mod win2_yuv2yuv_r2r_coe4;
#[doc = "WIN2_YUV2YUV_R2R_COE5 (rw) register accessor: WIN2 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_yuv2yuv_r2r_coe5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_yuv2yuv_r2r_coe5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_yuv2yuv_r2r_coe5`]
module"]
#[doc(alias = "WIN2_YUV2YUV_R2R_COE5")]
pub type Win2Yuv2yuvR2rCoe5 = crate::Reg<win2_yuv2yuv_r2r_coe5::Win2Yuv2yuvR2rCoe5Spec>;
#[doc = "WIN2 yuv2yuv r2r cofficient"]
pub mod win2_yuv2yuv_r2r_coe5;
#[doc = "WIN2_YUV2YUV_R2R_COE6 (rw) register accessor: WIN2 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_yuv2yuv_r2r_coe6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_yuv2yuv_r2r_coe6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_yuv2yuv_r2r_coe6`]
module"]
#[doc(alias = "WIN2_YUV2YUV_R2R_COE6")]
pub type Win2Yuv2yuvR2rCoe6 = crate::Reg<win2_yuv2yuv_r2r_coe6::Win2Yuv2yuvR2rCoe6Spec>;
#[doc = "WIN2 yuv2yuv r2r cofficient"]
pub mod win2_yuv2yuv_r2r_coe6;
#[doc = "WIN2_YUV2YUV_R2R_COE7 (rw) register accessor: WIN2 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_yuv2yuv_r2r_coe7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_yuv2yuv_r2r_coe7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_yuv2yuv_r2r_coe7`]
module"]
#[doc(alias = "WIN2_YUV2YUV_R2R_COE7")]
pub type Win2Yuv2yuvR2rCoe7 = crate::Reg<win2_yuv2yuv_r2r_coe7::Win2Yuv2yuvR2rCoe7Spec>;
#[doc = "WIN2 yuv2yuv r2r cofficient"]
pub mod win2_yuv2yuv_r2r_coe7;
#[doc = "WIN2_YUV2YUV_R2Y_COE0 (rw) register accessor: WIN2 yuv2yuv r2y cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_yuv2yuv_r2y_coe0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_yuv2yuv_r2y_coe0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_yuv2yuv_r2y_coe0`]
module"]
#[doc(alias = "WIN2_YUV2YUV_R2Y_COE0")]
pub type Win2Yuv2yuvR2yCoe0 = crate::Reg<win2_yuv2yuv_r2y_coe0::Win2Yuv2yuvR2yCoe0Spec>;
#[doc = "WIN2 yuv2yuv r2y cofficient"]
pub mod win2_yuv2yuv_r2y_coe0;
#[doc = "WIN2_YUV2YUV_R2Y_COE1 (rw) register accessor: WIN2 yuv2yuv r2y cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_yuv2yuv_r2y_coe1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_yuv2yuv_r2y_coe1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_yuv2yuv_r2y_coe1`]
module"]
#[doc(alias = "WIN2_YUV2YUV_R2Y_COE1")]
pub type Win2Yuv2yuvR2yCoe1 = crate::Reg<win2_yuv2yuv_r2y_coe1::Win2Yuv2yuvR2yCoe1Spec>;
#[doc = "WIN2 yuv2yuv r2y cofficient"]
pub mod win2_yuv2yuv_r2y_coe1;
#[doc = "WIN2_YUV2YUV_R2Y_COE2 (rw) register accessor: WIN2 yuv2yuv r2y cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_yuv2yuv_r2y_coe2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_yuv2yuv_r2y_coe2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_yuv2yuv_r2y_coe2`]
module"]
#[doc(alias = "WIN2_YUV2YUV_R2Y_COE2")]
pub type Win2Yuv2yuvR2yCoe2 = crate::Reg<win2_yuv2yuv_r2y_coe2::Win2Yuv2yuvR2yCoe2Spec>;
#[doc = "WIN2 yuv2yuv r2y cofficient"]
pub mod win2_yuv2yuv_r2y_coe2;
#[doc = "WIN2_YUV2YUV_R2Y_COE3 (rw) register accessor: WIN2 yuv2yuv r2y cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_yuv2yuv_r2y_coe3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_yuv2yuv_r2y_coe3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_yuv2yuv_r2y_coe3`]
module"]
#[doc(alias = "WIN2_YUV2YUV_R2Y_COE3")]
pub type Win2Yuv2yuvR2yCoe3 = crate::Reg<win2_yuv2yuv_r2y_coe3::Win2Yuv2yuvR2yCoe3Spec>;
#[doc = "WIN2 yuv2yuv r2y cofficient"]
pub mod win2_yuv2yuv_r2y_coe3;
#[doc = "WIN2_YUV2YUV_R2Y_COE4 (rw) register accessor: WIN2 yuv2yuv r2y cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_yuv2yuv_r2y_coe4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_yuv2yuv_r2y_coe4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_yuv2yuv_r2y_coe4`]
module"]
#[doc(alias = "WIN2_YUV2YUV_R2Y_COE4")]
pub type Win2Yuv2yuvR2yCoe4 = crate::Reg<win2_yuv2yuv_r2y_coe4::Win2Yuv2yuvR2yCoe4Spec>;
#[doc = "WIN2 yuv2yuv r2y cofficient"]
pub mod win2_yuv2yuv_r2y_coe4;
#[doc = "WIN2_YUV2YUV_R2Y_COE5 (rw) register accessor: WIN2 yuv2yuv r2y cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_yuv2yuv_r2y_coe5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_yuv2yuv_r2y_coe5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_yuv2yuv_r2y_coe5`]
module"]
#[doc(alias = "WIN2_YUV2YUV_R2Y_COE5")]
pub type Win2Yuv2yuvR2yCoe5 = crate::Reg<win2_yuv2yuv_r2y_coe5::Win2Yuv2yuvR2yCoe5Spec>;
#[doc = "WIN2 yuv2yuv r2y cofficient"]
pub mod win2_yuv2yuv_r2y_coe5;
#[doc = "WIN2_YUV2YUV_R2Y_COE6 (rw) register accessor: WIN2 yuv2yuv r2y cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_yuv2yuv_r2y_coe6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_yuv2yuv_r2y_coe6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_yuv2yuv_r2y_coe6`]
module"]
#[doc(alias = "WIN2_YUV2YUV_R2Y_COE6")]
pub type Win2Yuv2yuvR2yCoe6 = crate::Reg<win2_yuv2yuv_r2y_coe6::Win2Yuv2yuvR2yCoe6Spec>;
#[doc = "WIN2 yuv2yuv r2y cofficient"]
pub mod win2_yuv2yuv_r2y_coe6;
#[doc = "WIN2_YUV2YUV_R2Y_COE7 (rw) register accessor: WIN2 yuv2yuv r2y cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_yuv2yuv_r2y_coe7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_yuv2yuv_r2y_coe7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_yuv2yuv_r2y_coe7`]
module"]
#[doc(alias = "WIN2_YUV2YUV_R2Y_COE7")]
pub type Win2Yuv2yuvR2yCoe7 = crate::Reg<win2_yuv2yuv_r2y_coe7::Win2Yuv2yuvR2yCoe7Spec>;
#[doc = "WIN2 yuv2yuv r2y cofficient"]
pub mod win2_yuv2yuv_r2y_coe7;
#[doc = "WIN3_YUV2YUV_Y2R_COE0 (rw) register accessor: WIN3 yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_yuv2yuv_y2r_coe0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_yuv2yuv_y2r_coe0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_yuv2yuv_y2r_coe0`]
module"]
#[doc(alias = "WIN3_YUV2YUV_Y2R_COE0")]
pub type Win3Yuv2yuvY2rCoe0 = crate::Reg<win3_yuv2yuv_y2r_coe0::Win3Yuv2yuvY2rCoe0Spec>;
#[doc = "WIN3 yuv2yuv y2r cofficient"]
pub mod win3_yuv2yuv_y2r_coe0;
#[doc = "WIN3_YUV2YUV_Y2R_COE1 (rw) register accessor: WIN3 yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_yuv2yuv_y2r_coe1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_yuv2yuv_y2r_coe1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_yuv2yuv_y2r_coe1`]
module"]
#[doc(alias = "WIN3_YUV2YUV_Y2R_COE1")]
pub type Win3Yuv2yuvY2rCoe1 = crate::Reg<win3_yuv2yuv_y2r_coe1::Win3Yuv2yuvY2rCoe1Spec>;
#[doc = "WIN3 yuv2yuv y2r cofficient"]
pub mod win3_yuv2yuv_y2r_coe1;
#[doc = "WIN3_YUV2YUV_Y2R_COE2 (rw) register accessor: WIN3 yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_yuv2yuv_y2r_coe2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_yuv2yuv_y2r_coe2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_yuv2yuv_y2r_coe2`]
module"]
#[doc(alias = "WIN3_YUV2YUV_Y2R_COE2")]
pub type Win3Yuv2yuvY2rCoe2 = crate::Reg<win3_yuv2yuv_y2r_coe2::Win3Yuv2yuvY2rCoe2Spec>;
#[doc = "WIN3 yuv2yuv y2r cofficient"]
pub mod win3_yuv2yuv_y2r_coe2;
#[doc = "WIN3_YUV2YUV_Y2R_COE3 (rw) register accessor: WIN3 yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_yuv2yuv_y2r_coe3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_yuv2yuv_y2r_coe3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_yuv2yuv_y2r_coe3`]
module"]
#[doc(alias = "WIN3_YUV2YUV_Y2R_COE3")]
pub type Win3Yuv2yuvY2rCoe3 = crate::Reg<win3_yuv2yuv_y2r_coe3::Win3Yuv2yuvY2rCoe3Spec>;
#[doc = "WIN3 yuv2yuv y2r cofficient"]
pub mod win3_yuv2yuv_y2r_coe3;
#[doc = "WIN3_YUV2YUV_Y2R_COE4 (rw) register accessor: WIN3 yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_yuv2yuv_y2r_coe4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_yuv2yuv_y2r_coe4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_yuv2yuv_y2r_coe4`]
module"]
#[doc(alias = "WIN3_YUV2YUV_Y2R_COE4")]
pub type Win3Yuv2yuvY2rCoe4 = crate::Reg<win3_yuv2yuv_y2r_coe4::Win3Yuv2yuvY2rCoe4Spec>;
#[doc = "WIN3 yuv2yuv y2r cofficient"]
pub mod win3_yuv2yuv_y2r_coe4;
#[doc = "WIN3_YUV2YUV_Y2R_COE5 (rw) register accessor: WIN3 yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_yuv2yuv_y2r_coe5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_yuv2yuv_y2r_coe5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_yuv2yuv_y2r_coe5`]
module"]
#[doc(alias = "WIN3_YUV2YUV_Y2R_COE5")]
pub type Win3Yuv2yuvY2rCoe5 = crate::Reg<win3_yuv2yuv_y2r_coe5::Win3Yuv2yuvY2rCoe5Spec>;
#[doc = "WIN3 yuv2yuv y2r cofficient"]
pub mod win3_yuv2yuv_y2r_coe5;
#[doc = "WIN3_YUV2YUV_Y2R_COE6 (rw) register accessor: WIN3 yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_yuv2yuv_y2r_coe6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_yuv2yuv_y2r_coe6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_yuv2yuv_y2r_coe6`]
module"]
#[doc(alias = "WIN3_YUV2YUV_Y2R_COE6")]
pub type Win3Yuv2yuvY2rCoe6 = crate::Reg<win3_yuv2yuv_y2r_coe6::Win3Yuv2yuvY2rCoe6Spec>;
#[doc = "WIN3 yuv2yuv y2r cofficient"]
pub mod win3_yuv2yuv_y2r_coe6;
#[doc = "WIN3_YUV2YUV_Y2R_COE7 (rw) register accessor: WIN3 yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_yuv2yuv_y2r_coe7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_yuv2yuv_y2r_coe7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_yuv2yuv_y2r_coe7`]
module"]
#[doc(alias = "WIN3_YUV2YUV_Y2R_COE7")]
pub type Win3Yuv2yuvY2rCoe7 = crate::Reg<win3_yuv2yuv_y2r_coe7::Win3Yuv2yuvY2rCoe7Spec>;
#[doc = "WIN3 yuv2yuv y2r cofficient"]
pub mod win3_yuv2yuv_y2r_coe7;
#[doc = "WIN3_YUV2YUV_R2R_COE0 (rw) register accessor: WIN3 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_yuv2yuv_r2r_coe0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_yuv2yuv_r2r_coe0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_yuv2yuv_r2r_coe0`]
module"]
#[doc(alias = "WIN3_YUV2YUV_R2R_COE0")]
pub type Win3Yuv2yuvR2rCoe0 = crate::Reg<win3_yuv2yuv_r2r_coe0::Win3Yuv2yuvR2rCoe0Spec>;
#[doc = "WIN3 yuv2yuv r2r cofficient"]
pub mod win3_yuv2yuv_r2r_coe0;
#[doc = "WIN3_YUV2YUV_R2R_COE1 (rw) register accessor: WIN3 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_yuv2yuv_r2r_coe1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_yuv2yuv_r2r_coe1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_yuv2yuv_r2r_coe1`]
module"]
#[doc(alias = "WIN3_YUV2YUV_R2R_COE1")]
pub type Win3Yuv2yuvR2rCoe1 = crate::Reg<win3_yuv2yuv_r2r_coe1::Win3Yuv2yuvR2rCoe1Spec>;
#[doc = "WIN3 yuv2yuv r2r cofficient"]
pub mod win3_yuv2yuv_r2r_coe1;
#[doc = "WIN3_YUV2YUV_R2R_COE2 (rw) register accessor: WIN3 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_yuv2yuv_r2r_coe2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_yuv2yuv_r2r_coe2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_yuv2yuv_r2r_coe2`]
module"]
#[doc(alias = "WIN3_YUV2YUV_R2R_COE2")]
pub type Win3Yuv2yuvR2rCoe2 = crate::Reg<win3_yuv2yuv_r2r_coe2::Win3Yuv2yuvR2rCoe2Spec>;
#[doc = "WIN3 yuv2yuv r2r cofficient"]
pub mod win3_yuv2yuv_r2r_coe2;
#[doc = "WIN3_YUV2YUV_R2R_COE3 (rw) register accessor: WIN3 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_yuv2yuv_r2r_coe3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_yuv2yuv_r2r_coe3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_yuv2yuv_r2r_coe3`]
module"]
#[doc(alias = "WIN3_YUV2YUV_R2R_COE3")]
pub type Win3Yuv2yuvR2rCoe3 = crate::Reg<win3_yuv2yuv_r2r_coe3::Win3Yuv2yuvR2rCoe3Spec>;
#[doc = "WIN3 yuv2yuv r2r cofficient"]
pub mod win3_yuv2yuv_r2r_coe3;
#[doc = "WIN3_YUV2YUV_R2R_COE4 (rw) register accessor: WIN3 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_yuv2yuv_r2r_coe4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_yuv2yuv_r2r_coe4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_yuv2yuv_r2r_coe4`]
module"]
#[doc(alias = "WIN3_YUV2YUV_R2R_COE4")]
pub type Win3Yuv2yuvR2rCoe4 = crate::Reg<win3_yuv2yuv_r2r_coe4::Win3Yuv2yuvR2rCoe4Spec>;
#[doc = "WIN3 yuv2yuv r2r cofficient"]
pub mod win3_yuv2yuv_r2r_coe4;
#[doc = "WIN3_YUV2YUV_R2R_COE5 (rw) register accessor: WIN3 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_yuv2yuv_r2r_coe5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_yuv2yuv_r2r_coe5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_yuv2yuv_r2r_coe5`]
module"]
#[doc(alias = "WIN3_YUV2YUV_R2R_COE5")]
pub type Win3Yuv2yuvR2rCoe5 = crate::Reg<win3_yuv2yuv_r2r_coe5::Win3Yuv2yuvR2rCoe5Spec>;
#[doc = "WIN3 yuv2yuv r2r cofficient"]
pub mod win3_yuv2yuv_r2r_coe5;
#[doc = "WIN3_YUV2YUV_R2R_COE6 (rw) register accessor: WIN3 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_yuv2yuv_r2r_coe6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_yuv2yuv_r2r_coe6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_yuv2yuv_r2r_coe6`]
module"]
#[doc(alias = "WIN3_YUV2YUV_R2R_COE6")]
pub type Win3Yuv2yuvR2rCoe6 = crate::Reg<win3_yuv2yuv_r2r_coe6::Win3Yuv2yuvR2rCoe6Spec>;
#[doc = "WIN3 yuv2yuv r2r cofficient"]
pub mod win3_yuv2yuv_r2r_coe6;
#[doc = "WIN3_YUV2YUV_R2R_COE7 (rw) register accessor: WIN3 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_yuv2yuv_r2r_coe7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_yuv2yuv_r2r_coe7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_yuv2yuv_r2r_coe7`]
module"]
#[doc(alias = "WIN3_YUV2YUV_R2R_COE7")]
pub type Win3Yuv2yuvR2rCoe7 = crate::Reg<win3_yuv2yuv_r2r_coe7::Win3Yuv2yuvR2rCoe7Spec>;
#[doc = "WIN3 yuv2yuv r2r cofficient"]
pub mod win3_yuv2yuv_r2r_coe7;
#[doc = "WIN3_YUV2YUV_R2Y_COE0 (rw) register accessor: WIN3 yuv2yuv r2y cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_yuv2yuv_r2y_coe0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_yuv2yuv_r2y_coe0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_yuv2yuv_r2y_coe0`]
module"]
#[doc(alias = "WIN3_YUV2YUV_R2Y_COE0")]
pub type Win3Yuv2yuvR2yCoe0 = crate::Reg<win3_yuv2yuv_r2y_coe0::Win3Yuv2yuvR2yCoe0Spec>;
#[doc = "WIN3 yuv2yuv r2y cofficient"]
pub mod win3_yuv2yuv_r2y_coe0;
#[doc = "WIN3_YUV2YUV_R2Y_COE1 (rw) register accessor: WIN3 yuv2yuv r2y cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_yuv2yuv_r2y_coe1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_yuv2yuv_r2y_coe1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_yuv2yuv_r2y_coe1`]
module"]
#[doc(alias = "WIN3_YUV2YUV_R2Y_COE1")]
pub type Win3Yuv2yuvR2yCoe1 = crate::Reg<win3_yuv2yuv_r2y_coe1::Win3Yuv2yuvR2yCoe1Spec>;
#[doc = "WIN3 yuv2yuv r2y cofficient"]
pub mod win3_yuv2yuv_r2y_coe1;
#[doc = "WIN3_YUV2YUV_R2Y_COE2 (rw) register accessor: WIN3 yuv2yuv r2y cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_yuv2yuv_r2y_coe2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_yuv2yuv_r2y_coe2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_yuv2yuv_r2y_coe2`]
module"]
#[doc(alias = "WIN3_YUV2YUV_R2Y_COE2")]
pub type Win3Yuv2yuvR2yCoe2 = crate::Reg<win3_yuv2yuv_r2y_coe2::Win3Yuv2yuvR2yCoe2Spec>;
#[doc = "WIN3 yuv2yuv r2y cofficient"]
pub mod win3_yuv2yuv_r2y_coe2;
#[doc = "WIN3_YUV2YUV_R2Y_COE3 (rw) register accessor: WIN3 yuv2yuv cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_yuv2yuv_r2y_coe3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_yuv2yuv_r2y_coe3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_yuv2yuv_r2y_coe3`]
module"]
#[doc(alias = "WIN3_YUV2YUV_R2Y_COE3")]
pub type Win3Yuv2yuvR2yCoe3 = crate::Reg<win3_yuv2yuv_r2y_coe3::Win3Yuv2yuvR2yCoe3Spec>;
#[doc = "WIN3 yuv2yuv cofficient"]
pub mod win3_yuv2yuv_r2y_coe3;
#[doc = "WIN3_YUV2YUV_R2Y_COE4 (rw) register accessor: WIN3 yuv2yuv r2y cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_yuv2yuv_r2y_coe4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_yuv2yuv_r2y_coe4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_yuv2yuv_r2y_coe4`]
module"]
#[doc(alias = "WIN3_YUV2YUV_R2Y_COE4")]
pub type Win3Yuv2yuvR2yCoe4 = crate::Reg<win3_yuv2yuv_r2y_coe4::Win3Yuv2yuvR2yCoe4Spec>;
#[doc = "WIN3 yuv2yuv r2y cofficient"]
pub mod win3_yuv2yuv_r2y_coe4;
#[doc = "WIN3_YUV2YUV_R2Y_COE5 (rw) register accessor: WIN3 yuv2yuv r2y cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_yuv2yuv_r2y_coe5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_yuv2yuv_r2y_coe5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_yuv2yuv_r2y_coe5`]
module"]
#[doc(alias = "WIN3_YUV2YUV_R2Y_COE5")]
pub type Win3Yuv2yuvR2yCoe5 = crate::Reg<win3_yuv2yuv_r2y_coe5::Win3Yuv2yuvR2yCoe5Spec>;
#[doc = "WIN3 yuv2yuv r2y cofficient"]
pub mod win3_yuv2yuv_r2y_coe5;
#[doc = "WIN3_YUV2YUV_R2Y_COE6 (rw) register accessor: WIN3 yuv2yuv r2y cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_yuv2yuv_r2y_coe6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_yuv2yuv_r2y_coe6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_yuv2yuv_r2y_coe6`]
module"]
#[doc(alias = "WIN3_YUV2YUV_R2Y_COE6")]
pub type Win3Yuv2yuvR2yCoe6 = crate::Reg<win3_yuv2yuv_r2y_coe6::Win3Yuv2yuvR2yCoe6Spec>;
#[doc = "WIN3 yuv2yuv r2y cofficient"]
pub mod win3_yuv2yuv_r2y_coe6;
#[doc = "WIN3_YUV2YUV_R2Y_COE7 (rw) register accessor: WIN3 yuv2yuv r2y cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_yuv2yuv_r2y_coe7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_yuv2yuv_r2y_coe7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_yuv2yuv_r2y_coe7`]
module"]
#[doc(alias = "WIN3_YUV2YUV_R2Y_COE7")]
pub type Win3Yuv2yuvR2yCoe7 = crate::Reg<win3_yuv2yuv_r2y_coe7::Win3Yuv2yuvR2yCoe7Spec>;
#[doc = "WIN3 yuv2yuv r2y cofficient"]
pub mod win3_yuv2yuv_r2y_coe7;
#[doc = "WIN2_LUT_ADDR (rw) register accessor: Win2 lut base address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_lut_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_lut_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_lut_addr`]
module"]
#[doc(alias = "WIN2_LUT_ADDR")]
pub type Win2LutAddr = crate::Reg<win2_lut_addr::Win2LutAddrSpec>;
#[doc = "Win2 lut base address"]
pub mod win2_lut_addr;
#[doc = "WIN3_LUT_ADDR (rw) register accessor: Win3 lut base address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_lut_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_lut_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_lut_addr`]
module"]
#[doc(alias = "WIN3_LUT_ADDR")]
pub type Win3LutAddr = crate::Reg<win3_lut_addr::Win3LutAddrSpec>;
#[doc = "Win3 lut base address"]
pub mod win3_lut_addr;
#[doc = "HWC_LUT_ADDR (rw) register accessor: Hwc lut base address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwc_lut_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwc_lut_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwc_lut_addr`]
module"]
#[doc(alias = "HWC_LUT_ADDR")]
pub type HwcLutAddr = crate::Reg<hwc_lut_addr::HwcLutAddrSpec>;
#[doc = "Hwc lut base address"]
pub mod hwc_lut_addr;
#[doc = "CABC_GAMMA_LUT_ADDR (rw) register accessor: CABC GAMMA lut base address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cabc_gamma_lut_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cabc_gamma_lut_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cabc_gamma_lut_addr`]
module"]
#[doc(alias = "CABC_GAMMA_LUT_ADDR")]
pub type CabcGammaLutAddr = crate::Reg<cabc_gamma_lut_addr::CabcGammaLutAddrSpec>;
#[doc = "CABC GAMMA lut base address"]
pub mod cabc_gamma_lut_addr;
#[doc = "GAMMA_LUT_ADDR (rw) register accessor: GAMMA lut base address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gamma_lut_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_lut_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gamma_lut_addr`]
module"]
#[doc(alias = "GAMMA_LUT_ADDR")]
pub type GammaLutAddr = crate::Reg<gamma_lut_addr::GammaLutAddrSpec>;
#[doc = "GAMMA lut base address"]
pub mod gamma_lut_addr;
