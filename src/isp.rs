#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    vi_ccl: ViCcl,
    _reserved1: [u8; 0x0c],
    vi_iccl: ViIccl,
    vi_ircl: ViIrcl,
    vi_dpcl: ViDpcl,
    _reserved4: [u8; 0x01e4],
    img_eff_ctrl: ImgEffCtrl,
    img_eff_color_sel: ImgEffColorSel,
    img_eff_mat_1: ImgEffMat1,
    img_eff_mat_2: ImgEffMat2,
    img_eff_mat_3: ImgEffMat3,
    img_eff_mat_4: ImgEffMat4,
    img_eff_mat_5: ImgEffMat5,
    img_eff_tint: ImgEffTint,
    img_eff_ctrl_shd: ImgEffCtrlShd,
    img_eff_sharpen: ImgEffSharpen,
    _reserved14: [u8; 0xd8],
    super_imp_ctrl: SuperImpCtrl,
    super_imp_offset_x: SuperImpOffsetX,
    super_imp_offset_y: SuperImpOffsetY,
    _reserved17: [u8; 0xf4],
    ctrl: Ctrl,
    acq_prop: AcqProp,
    acq_h_offs: AcqHOffs,
    acq_v_offs: AcqVOffs,
    acq_h_size: AcqHSize,
    acq_v_size: AcqVSize,
    acq_nr_frames: AcqNrFrames,
    gamma_dx_lo: GammaDxLo,
    gamma_dx_hi: GammaDxHi,
    gamma_r_y: [GammaRY; 17],
    gamma_g_y: [GammaGY; 17],
    gamma_b_y: [GammaBY; 17],
    _reserved29: [u8; 0x20],
    awb_prop: AwbProp,
    awb_h_offs: AwbHOffs,
    awb_v_offs: AwbVOffs,
    awb_h_size: AwbHSize,
    awb_v_size: AwbVSize,
    awb_frames: AwbFrames,
    awb_ref: AwbRef,
    awb_thresh: AwbThresh,
    _reserved37: [u8; 0x08],
    awb_gain_g: AwbGainG,
    awb_gain_rb: AwbGainRb,
    awb_white_cnt: AwbWhiteCnt,
    awb_mean: AwbMean,
    _reserved41: [u8; 0x28],
    cc_coeff_0: CcCoeff0,
    cc_coeff_1: CcCoeff1,
    cc_coeff_2: CcCoeff2,
    cc_coeff_3: CcCoeff3,
    cc_coeff_4: CcCoeff4,
    cc_coeff_5: CcCoeff5,
    cc_coeff_6: CcCoeff6,
    cc_coeff_7: CcCoeff7,
    cc_coeff_8: CcCoeff8,
    out_h_offs: OutHOffs,
    out_v_offs: OutVOffs,
    out_h_size: OutHSize,
    out_v_size: OutVSize,
    demosaic: Demosaic,
    flags_shd: FlagsShd,
    _reserved56: [u8; 0x10],
    imsc: Imsc,
    ris: Ris,
    mis: Mis,
    icr: Icr,
    isr: Isr,
    ct_coeff: [CtCoeff; 9],
    gamma_out_mode: GammaOutMode,
    gamma_out_y: [GammaOutY; 17],
    err: Err,
    err_clr: ErrClr,
    frame_count: FrameCount,
    ct_offset_r: CtOffsetR,
    ct_offset_g: CtOffsetG,
    ct_offset_b: CtOffsetB,
    _reserved70: [u8; 0x0c],
    flash_cmd: FlashCmd,
    flash_config: FlashConfig,
    flash_prediv: FlashPrediv,
    flash_delay: FlashDelay,
    flash_time: FlashTime,
    flash_maxp: FlashMaxp,
    _reserved76: [u8; 0x08],
    sh_ctrl: ShCtrl,
    sh_prediv: ShPrediv,
    sh_delay: ShDelay,
    sh_time: ShTime,
    _reserved80: [u8; 0x0170],
    cproc_ctrl: CprocCtrl,
    cproc_contrast: CprocContrast,
    cproc_brightness: CprocBrightness,
    cproc_saturation: CprocSaturation,
    cproc_hue: CprocHue,
    _reserved85: [u8; 0x03ec],
    mrsz_ctrl: MrszCtrl,
    mrsz_scale_hy: MrszScaleHy,
    mrsz_scale_hcb: MrszScaleHcb,
    mrsz_scale_hcr: MrszScaleHcr,
    mrsz_scale_vy: MrszScaleVy,
    mrsz_scale_vc: MrszScaleVc,
    mrsz_phase_hy: MrszPhaseHy,
    mrsz_phase_hc: MrszPhaseHc,
    mrsz_phase_vy: MrszPhaseVy,
    mrsz_phase_vc: MrszPhaseVc,
    mrsz_scale_lut_addr: MrszScaleLutAddr,
    mrsz_scale_lut: MrszScaleLut,
    mrsz_ctrl_shd: MrszCtrlShd,
    mrsz_scale_hy_shd: MrszScaleHyShd,
    mrsz_scale_hcb_shd: MrszScaleHcbShd,
    mrsz_scale_hcr_shd: MrszScaleHcrShd,
    mrsz_scale_vy_shd: MrszScaleVyShd,
    mrsz_scale_vc_shd: MrszScaleVcShd,
    mrsz_phase_hy_shd: MrszPhaseHyShd,
    mrsz_phase_hc_shd: MrszPhaseHcShd,
    mrsz_phase_vy_shd: MrszPhaseVyShd,
    mrsz_phase_vc_shd: MrszPhaseVcShd,
    _reserved107: [u8; 0x03a8],
    srsz_ctrl: SrszCtrl,
    srsz_scale_hy: SrszScaleHy,
    srsz_scale_hcb: SrszScaleHcb,
    srsz_scale_hcr: SrszScaleHcr,
    srsz_scale_vy: SrszScaleVy,
    srsz_scale_vc: SrszScaleVc,
    srsz_phase_hy: SrszPhaseHy,
    srsz_phase_hc: SrszPhaseHc,
    srsz_phase_vy: SrszPhaseVy,
    srsz_phase_vc: SrszPhaseVc,
    srsz_scale_lut_addr: SrszScaleLutAddr,
    srsz_scale_lut: SrszScaleLut,
    srsz_ctrl_shd: SrszCtrlShd,
    srsz_scale_hy_shd: SrszScaleHyShd,
    srsz_scale_hcb_shd: SrszScaleHcbShd,
    srsz_scale_hcr_shd: SrszScaleHcrShd,
    srsz_scale_vy_shd: SrszScaleVyShd,
    srsz_scale_vc_shd: SrszScaleVcShd,
    srsz_phase_hy_shd: SrszPhaseHyShd,
    srsz_phase_hc_shd: SrszPhaseHcShd,
    srsz_phase_vy_shd: SrszPhaseVyShd,
    srsz_phase_vc_shd: SrszPhaseVcShd,
    _reserved129: [u8; 0x03a8],
    mi_ctrl: MiCtrl,
    mi_init: MiInit,
    mi_mp_y_base_ad_init: MiMpYBaseAdInit,
    mi_mp_y_size_init: MiMpYSizeInit,
    mi_mp_y_offs_cnt_init: MiMpYOffsCntInit,
    mi_mp_y_offs_cnt_start: MiMpYOffsCntStart,
    mi_mp_y_irq_offs_init: MiMpYIrqOffsInit,
    mi_mp_cb_base_ad_init: MiMpCbBaseAdInit,
    mi_mp_cb_size_init: MiMpCbSizeInit,
    mi_mp_cb_offs_cnt_init: MiMpCbOffsCntInit,
    mi_mp_cb_offs_cnt_start: MiMpCbOffsCntStart,
    mi_mp_cr_base_ad_init: MiMpCrBaseAdInit,
    mi_mp_cr_size_init: MiMpCrSizeInit,
    mi_mp_cr_offs_cnt_init: MiMpCrOffsCntInit,
    mi_mp_cr_offs_cnt_start: MiMpCrOffsCntStart,
    mi_sp_y_base_ad_init: MiSpYBaseAdInit,
    mi_sp_y_size_init: MiSpYSizeInit,
    mi_sp_y_offs_cnt_init: MiSpYOffsCntInit,
    mi_sp_y_offs_cnt_start: MiSpYOffsCntStart,
    mi_sp_y_llength: MiSpYLlength,
    mi_sp_cb_base_ad_init: MiSpCbBaseAdInit,
    mi_sp_cb_size_init: MiSpCbSizeInit,
    mi_sp_cb_offs_cnt_init: MiSpCbOffsCntInit,
    mi_sp_cb_offs_cnt_start: MiSpCbOffsCntStart,
    mi_sp_cr_base_ad_init: MiSpCrBaseAdInit,
    mi_sp_cr_size_init: MiSpCrSizeInit,
    mi_sp_cr_offs_cnt_init: MiSpCrOffsCntInit,
    mi_sp_cr_offs_cnt_start: MiSpCrOffsCntStart,
    mi_byte_cnt: MiByteCnt,
    mi_ctrl_shd: MiCtrlShd,
    mi_mp_y_base_ad_shd: MiMpYBaseAdShd,
    mi_mp_y_size_shd: MiMpYSizeShd,
    mi_mp_y_offs_cnt_shd: MiMpYOffsCntShd,
    mi_mp_y_irq_offs_shd: MiMpYIrqOffsShd,
    mi_mp_cb_base_ad_shd: MiMpCbBaseAdShd,
    mi_mp_cb_size_shd: MiMpCbSizeShd,
    mi_mp_cb_offs_cnt_shd: MiMpCbOffsCntShd,
    mi_mp_cr_base_ad_shd: MiMpCrBaseAdShd,
    mi_mp_cr_size_shd: MiMpCrSizeShd,
    mi_mp_cr_offs_cnt_shd: MiMpCrOffsCntShd,
    mi_sp_y_base_ad_shd: MiSpYBaseAdShd,
    mi_sp_y_size_shd: MiSpYSizeShd,
    mi_sp_y_offs_cnt_shd: MiSpYOffsCntShd,
    _reserved172: [u8; 0x04],
    mi_sp_cb_base_ad_shd: MiSpCbBaseAdShd,
    mi_sp_cb_size_shd: MiSpCbSizeShd,
    mi_sp_cb_offs_cnt_shd: MiSpCbOffsCntShd,
    mi_sp_cr_base_ad_shd: MiSpCrBaseAdShd,
    mi_sp_cr_size_shd: MiSpCrSizeShd,
    mi_sp_cr_offs_cnt_shd: MiSpCrOffsCntShd,
    mi_dma_y_pic_start_ad: MiDmaYPicStartAd,
    mi_dma_y_pic_width: MiDmaYPicWidth,
    mi_dma_y_llength: MiDmaYLlength,
    mi_dma_y_pic_size: MiDmaYPicSize,
    mi_dma_cb_pic_start_ad: MiDmaCbPicStartAd,
    _reserved183: [u8; 0x0c],
    mi_dma_cr_pic_start_ad: MiDmaCrPicStartAd,
    _reserved184: [u8; 0x0c],
    mi_imsc: MiImsc,
    mi_ris: MiRis,
    mi_mis: MiMis,
    mi_icr: MiIcr,
    mi_isr: MiIsr,
    mi_status: MiStatus,
    mi_status_clr: MiStatusClr,
    mi_sp_y_pic_width: MiSpYPicWidth,
    mi_sp_y_pic_height: MiSpYPicHeight,
    mi_sp_y_pic_size: MiSpYPicSize,
    mi_dma_ctrl: MiDmaCtrl,
    mi_dma_start: MiDmaStart,
    mi_dma_status: MiDmaStatus,
    mi_pixel_cnt: MiPixelCnt,
    mi_mp_y_base_ad_init2: MiMpYBaseAdInit2,
    mi_mp_cb_base_ad_init2: MiMpCbBaseAdInit2,
    mi_mp_cr_base_ad_init2: MiMpCrBaseAdInit2,
    mi_sp_y_base_ad_init2: MiSpYBaseAdInit2,
    mi_sp_cb_base_ad_init2: MiSpCbBaseAdInit2,
    mi_sp_cr_base_ad_init2: MiSpCrBaseAdInit2,
    mi_xtd_format_ctrl: MiXtdFormatCtrl,
    _reserved205: [u8; 0x06b4],
    mipi_ctrl: MipiCtrl,
    mipi_status: MipiStatus,
    mipi_imsc: MipiImsc,
    mipi_ris: MipiRis,
    mipi_mis: MipiMis,
    mipi_icr: MipiIcr,
    mipi_isr: MipiIsr,
    mipi_cur_data_id: MipiCurDataId,
    mipi_img_data_sel: MipiImgDataSel,
    mipi_add_data_sel_1: MipiAddDataSel1,
    mipi_add_data_sel_2: MipiAddDataSel2,
    mipi_add_data_sel_3: MipiAddDataSel3,
    mipi_add_data_sel_4: MipiAddDataSel4,
    mipi_add_data_fifo: MipiAddDataFifo,
    _reserved219: [u8; 0x04],
    mipi_compressed_mode: MipiCompressedMode,
    mipi_frame: MipiFrame,
    mipi_gen_short_dt: MipiGenShortDt,
    mipi_gen_short_8_9: MipiGenShort8_9,
    mipi_gen_short_a_b: MipiGenShortAB,
    mipi_gen_short_c_d: MipiGenShortCD,
    mipi_gen_short_e_f: MipiGenShortEF,
    _reserved226: [u8; 0x03ac],
    afm_lt_a: AfmLtA,
    afm_rb_a: AfmRbA,
    afm_lt_b: AfmLtB,
    afm_rb_b: AfmRbB,
    afm_lt_c: AfmLtC,
    afm_rb_c: AfmRbC,
    afm_thres: AfmThres,
    afm_var_shift: AfmVarShift,
    afm_sum_a: AfmSumA,
    afm_sum_b: AfmSumB,
    afm_sum_c: AfmSumC,
    afm_lum_a: AfmLumA,
    afm_lum_b: AfmLumB,
    afm_lum_c: AfmLumC,
    _reserved240: [u8; 0x01c4],
    lsc_ctrl: LscCtrl,
    lsc_r_table_addr: LscRTableAddr,
    lsc_gr_table_addr: LscGrTableAddr,
    lsc_b_table_addr: LscBTableAddr,
    lsc_gb_table_addr: LscGbTableAddr,
    lsc_r_table_data: LscRTableData,
    lsc_gr_table_data: LscGrTableData,
    lsc_b_table_data: LscBTableData,
    lsc_gb_table_data: LscGbTableData,
    lsc_xgrad_01: LscXgrad01,
    lsc_xgrad_23: LscXgrad23,
    lsc_xgrad_45: LscXgrad45,
    lsc_xgrad_67: LscXgrad67,
    lsc_ygrad_01: LscYgrad01,
    lsc_ygrad_23: LscYgrad23,
    lsc_ygrad_45: LscYgrad45,
    lsc_ygrad_67: LscYgrad67,
    lsc_xsize_01: LscXsize01,
    lsc_xsize_23: LscXsize23,
    lsc_xsize_45: LscXsize45,
    lsc_xsize_67: LscXsize67,
    lsc_ysize_01: LscYsize01,
    lsc_ysize_23: LscYsize23,
    lsc_ysize_45: LscYsize45,
    lsc_ysize_67: LscYsize67,
    lsc_table_sel: LscTableSel,
    lsc_status: LscStatus,
    _reserved267: [u8; 0x94],
    is_ctrl: IsCtrl,
    is_recenter: IsRecenter,
    is_h_offs: IsHOffs,
    is_v_offs: IsVOffs,
    is_h_size: IsHSize,
    is_v_size: IsVSize,
    is_max_dx: IsMaxDx,
    is_max_dy: IsMaxDy,
    is_displace: IsDisplace,
    is_h_offs_shd: IsHOffsShd,
    is_v_offs_shd: IsVOffsShd,
    is_h_size_shd: IsHSizeShd,
    is_v_size_shd: IsVSizeShd,
    _reserved280: [u8; 0xcc],
    hist_prop: HistProp,
    hist_h_offs: HistHOffs,
    hist_v_offs: HistVOffs,
    hist_h_size: HistHSize,
    hist_v_size: HistVSize,
    hist_bin: [HistBin; 16],
    hist_weight_00to30: HistWeight00to30,
    hist_weight_40to21: HistWeight40to21,
    hist_weight_31to12: HistWeight31to12,
    hist_weight_22to03: HistWeight22to03,
    hist_weight_13to43: HistWeight13to43,
    hist_weight_04to34: HistWeight04to34,
    hist_weight_44: HistWeight44,
    _reserved293: [u8; 0x90],
    filt_mode: FiltMode,
    _reserved294: [u8; 0x24],
    filt_thresh_bl0: FiltThreshBl0,
    filt_thresh_bl1: FiltThreshBl1,
    filt_thresh_sh0: FiltThreshSh0,
    filt_thresh_sh1: FiltThreshSh1,
    filt_lum_weight: FiltLumWeight,
    filt_fac_sh1: FiltFacSh1,
    filt_fac_sh0: FiltFacSh0,
    filt_fac_mid: FiltFacMid,
    filt_fac_bl0: FiltFacBl0,
    filt_fac_bl1: FiltFacBl1,
    _reserved304: [u8; 0x30],
    cac_ctrl: CacCtrl,
    cac_count_start: CacCountStart,
    cac_a: CacA,
    cac_b: CacB,
    cac_c: CacC,
    cac_x_norm: CacXNorm,
    cac_y_norm: CacYNorm,
    _reserved311: [u8; 0x64],
    exp_ctrl: ExpCtrl,
    exp_h_offset: ExpHOffset,
    exp_v_offset: ExpVOffset,
    exp_h_size: ExpHSize,
    exp_v_size: ExpVSize,
    exp_mean_00: ExpMean00,
    exp_mean_10: ExpMean10,
    exp_mean_20: ExpMean20,
    exp_mean_30: ExpMean30,
    exp_mean_40: ExpMean40,
    exp_mean_01: ExpMean01,
    exp_mean_11: ExpMean11,
    exp_mean_21: ExpMean21,
    exp_mean_31: ExpMean31,
    exp_mean_41: ExpMean41,
    exp_mean_02: ExpMean02,
    exp_mean_12: ExpMean12,
    exp_mean_22: ExpMean22,
    exp_mean_32: ExpMean32,
    exp_mean_42: ExpMean42,
    exp_mean_03: ExpMean03,
    exp_mean_13: ExpMean13,
    exp_mean_23: ExpMean23,
    exp_mean_33: ExpMean33,
    exp_mean_43: ExpMean43,
    exp_mean_04: ExpMean04,
    exp_mean_14: ExpMean14,
    exp_mean_24: ExpMean24,
    exp_mean_34: ExpMean34,
    exp_mean_44: ExpMean44,
    _reserved341: [u8; 0x88],
    bls_ctrl: BlsCtrl,
    bls_samples: BlsSamples,
    bls_h1_start: BlsH1Start,
    bls_h1_stop: BlsH1Stop,
    bls_v1_start: BlsV1Start,
    bls_v1_stop: BlsV1Stop,
    bls_h2_start: BlsH2Start,
    bls_h2_stop: BlsH2Stop,
    bls_v2_start: BlsV2Start,
    bls_v2_stop: BlsV2Stop,
    bls_a_fixed: BlsAFixed,
    bls_b_fixed: BlsBFixed,
    bls_c_fixed: BlsCFixed,
    bls_d_fixed: BlsDFixed,
    bls_a_measured: BlsAMeasured,
    bls_b_measured: BlsBMeasured,
    bls_c_measured: BlsCMeasured,
    bls_d_measured: BlsDMeasured,
    _reserved359: [u8; 0xb8],
    dpf_mode: DpfMode,
    dpf_strength_r: DpfStrengthR,
    dpf_strength_g: DpfStrengthG,
    dpf_strength_b: DpfStrengthB,
    dpf_s_weight_g_1_4: DpfSWeightG1_4,
    dpf_s_weight_g_5_6: DpfSWeightG5_6,
    dpf_s_weight_rb_1_4: DpfSWeightRb1_4,
    dpf_s_weight_rb_5_6: DpfSWeightRb5_6,
    dpf_nll_coeff: [DpfNllCoeff; 17],
    dpf_nf_gain_r: DpfNfGainR,
    dpf_nf_gain_gr: DpfNfGainGr,
    dpf_nf_gain_gb: DpfNfGainGb,
    dpf_nf_gain_b: DpfNfGainB,
    _reserved372: [u8; 0x8c],
    dpcc_mode: DpccMode,
    dpcc_output_mode: DpccOutputMode,
    dpcc_set_use: DpccSetUse,
    dpcc_methods_set_1: DpccMethodsSet1,
    dpcc_methods_set_2: DpccMethodsSet2,
    dpcc_methods_set_3: DpccMethodsSet3,
    dpcc_line_thresh_1: DpccLineThresh1,
    dpcc_line_mad_fac_1: DpccLineMadFac1,
    dpcc_pg_fac_1: DpccPgFac1,
    dpcc_rnd_thresh_1: DpccRndThresh1,
    dpcc_rg_fac_1: DpccRgFac1,
    dpcc_line_thresh_2: DpccLineThresh2,
    dpcc_line_mad_fac_2: DpccLineMadFac2,
    dpcc_pg_fac_2: DpccPgFac2,
    dpcc_rnd_thresh_2: DpccRndThresh2,
    dpcc_rg_fac_2: DpccRgFac2,
    dpcc_line_thresh_3: DpccLineThresh3,
    dpcc_line_mad_fac_3: DpccLineMadFac3,
    dpcc_pg_fac_3: DpccPgFac3,
    dpcc_rnd_thresh_3: DpccRndThresh3,
    dpcc_rg_fac_3: DpccRgFac3,
    dpcc_ro_limits: DpccRoLimits,
    dpcc_rnd_offs: DpccRndOffs,
    dpcc_bpt_ctrl: DpccBptCtrl,
    dpcc_bpt_number: DpccBptNumber,
    dpcc_bpt_addr: DpccBptAddr,
    dpcc_bpt_data: DpccBptData,
    _reserved399: [u8; 0x94],
    wdr_ctrl: WdrCtrl,
    wdr_tonecurve_1: WdrTonecurve1,
    wdr_tonecurve_2: WdrTonecurve2,
    wdr_tonecurve_3: WdrTonecurve3,
    wdr_tonecurve_4: WdrTonecurve4,
    wdr_tonecurve_ym: [WdrTonecurveYm; 33],
    wdr_offset: WdrOffset,
    wdr_deltamin: WdrDeltamin,
    wdr_tonecurve_1_shd: WdrTonecurve1Shd,
    wdr_tonecurve_2_shd: WdrTonecurve2Shd,
    wdr_tonecurve_3_shd: WdrTonecurve3Shd,
    wdr_tonecurve_4_shd: WdrTonecurve4Shd,
    wdr_tonecurve_ym_shd: [WdrTonecurveYmShd; 33],
    _reserved412: [u8; 0x03cc],
    vsm_mode: VsmMode,
    vsm_h_offs: VsmHOffs,
    vsm_v_offs: VsmVOffs,
    vsm_h_size: VsmHSize,
    vsm_v_size: VsmVSize,
    vsm_h_segments: VsmHSegments,
    vsm_v_segments: VsmVSegments,
    vsm_delta_h: VsmDeltaH,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    #[inline(always)]
    pub const fn vi_ccl(&self) -> &ViCcl {
        &self.vi_ccl
    }
    #[doc = "0x10 - Internal clock control register"]
    #[inline(always)]
    pub const fn vi_iccl(&self) -> &ViIccl {
        &self.vi_iccl
    }
    #[doc = "0x14 - Internal reset control register"]
    #[inline(always)]
    pub const fn vi_ircl(&self) -> &ViIrcl {
        &self.vi_ircl
    }
    #[doc = "0x18 - Data path control register"]
    #[inline(always)]
    pub const fn vi_dpcl(&self) -> &ViDpcl {
        &self.vi_dpcl
    }
    #[doc = "0x200 - Global control register\n\nNote: full_range for image effects is supported in ISP M5_v6, M5_v7 only \n\n\n\n"]
    #[inline(always)]
    pub const fn img_eff_ctrl(&self) -> &ImgEffCtrl {
        &self.img_eff_ctrl
    }
    #[doc = "0x204 - Color selection register (for color selection effect)"]
    #[inline(always)]
    pub const fn img_eff_color_sel(&self) -> &ImgEffColorSel {
        &self.img_eff_color_sel
    }
    #[doc = "0x208 - 3x3 matrix coefficients for emboss effect (1)\n\nNote: possible values for coefficients: 000 (1), 001 (2), 010 (4), 011 (8),100 (-1), 101 \n\n(-2), 110 (-4), 111 (-8) \n\n\n\n"]
    #[inline(always)]
    pub const fn img_eff_mat_1(&self) -> &ImgEffMat1 {
        &self.img_eff_mat_1
    }
    #[doc = "0x20c - 3x3 matrix coefficients for emboss effect (2)"]
    #[inline(always)]
    pub const fn img_eff_mat_2(&self) -> &ImgEffMat2 {
        &self.img_eff_mat_2
    }
    #[doc = "0x210 - 3x3 matrix coefficients for emboss(3) effect / \n\n\n\nsketch/sharpen(1) effect\n\nNote: possible values for \n\n\n\ncoefficients: 000 (1), 001 (2), 010 \n\n\n\n(4), 011 (8), \n\n100 (-1), 101 (-2), 110 (-4), 111 (-8) \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn img_eff_mat_3(&self) -> &ImgEffMat3 {
        &self.img_eff_mat_3
    }
    #[doc = "0x214 - 3x3 matrix coefficients for sketch/sharpen effect (2)\n\nNote: possible values for coefficients: 000 (1), 001 (2), 010 (4), 011 (8), \n\n100 (-1), 101 (-2), 110 (-4), 111 (-8) \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn img_eff_mat_4(&self) -> &ImgEffMat4 {
        &self.img_eff_mat_4
    }
    #[doc = "0x218 - 3x3 matrix coefficients for sketch/sharpen effect (3)\n\nNote: possible values for coefficients: 000 (1), 001 (2), 010 (4), 011 (8), \n\n100 (-1), 101 (-2), 110 (-4), 111 (-8) \n\n\n\n"]
    #[inline(always)]
    pub const fn img_eff_mat_5(&self) -> &ImgEffMat5 {
        &self.img_eff_mat_5
    }
    #[doc = "0x21c - Chrominance increment values of a tint (used for sepia effect)\n\nNote: Calculation process of incr_cr and incr_cb: \n\ntint values given in RGB format: R G B \n\nconverted to Cb and Cr: Cb = -0.148*R - 0.291*G + 0.439*B + 128 Cr = 0.439*R - \n\n\n\n0.368*G - 0.071*B + 128 \n\ncalculating of the increments inc_Cb = (128 – Cb)/110 inc_Cr = (128 – Cr)/110 \n\nregister entry of the increments with an accuracy of 1/64 incr_cb = inc_Cb * 64 \n\nincr_cr = inc_Cr * 64 \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn img_eff_tint(&self) -> &ImgEffTint {
        &self.img_eff_tint
    }
    #[doc = "0x220 - Shadow register for control register"]
    #[inline(always)]
    pub const fn img_eff_ctrl_shd(&self) -> &ImgEffCtrlShd {
        &self.img_eff_ctrl_shd
    }
    #[doc = "0x224 - Factor and threshold for sharpen effect\n\nNote: For the sharpening effect the convolution mask must be set to the values \\[-1 -1 -1; \n\n-1 8 -1; -1 -1 -1\\]. \n\n\n\nThe convolution mask for sharpening is defined by the values sket_coef_xx in registers \n\nIMG_EFF_MAT_3 through IMG_EFF_MAT_5. Sketch effect and sharpening effect share the \n\nsame mask registers. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn img_eff_sharpen(&self) -> &ImgEffSharpen {
        &self.img_eff_sharpen
    }
    #[doc = "0x300 - Global control register\n\nNote: in the bypass mode the data stream from Image \n\n\n\nEffect is transmitted to MUX module without overlaying \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn super_imp_ctrl(&self) -> &SuperImpCtrl {
        &self.super_imp_ctrl
    }
    #[doc = "0x304 - Offset x register"]
    #[inline(always)]
    pub const fn super_imp_offset_x(&self) -> &SuperImpOffsetX {
        &self.super_imp_offset_x
    }
    #[doc = "0x308 - Offset y register\n\nNote: the offset_y is positive and refers to the \n\n\n\nreference image \n\n\n\n"]
    #[inline(always)]
    pub const fn super_imp_offset_y(&self) -> &SuperImpOffsetY {
        &self.super_imp_offset_y
    }
    #[doc = "0x400 - global control register\n\nNote: partly write-only \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x404 - ISP acquisition properties"]
    #[inline(always)]
    pub const fn acq_prop(&self) -> &AcqProp {
        &self.acq_prop
    }
    #[doc = "0x408 - horizontal input offset"]
    #[inline(always)]
    pub const fn acq_h_offs(&self) -> &AcqHOffs {
        &self.acq_h_offs
    }
    #[doc = "0x40c - vertical input offset"]
    #[inline(always)]
    pub const fn acq_v_offs(&self) -> &AcqVOffs {
        &self.acq_v_offs
    }
    #[doc = "0x410 - horizontal input size"]
    #[inline(always)]
    pub const fn acq_h_size(&self) -> &AcqHSize {
        &self.acq_h_size
    }
    #[doc = "0x414 - vertical input size"]
    #[inline(always)]
    pub const fn acq_v_size(&self) -> &AcqVSize {
        &self.acq_v_size
    }
    #[doc = "0x418 - Number of frames to be captured"]
    #[inline(always)]
    pub const fn acq_nr_frames(&self) -> &AcqNrFrames {
        &self.acq_nr_frames
    }
    #[doc = "0x41c - De-Gamma Curve definition lower x increments (sampling points)"]
    #[inline(always)]
    pub const fn gamma_dx_lo(&self) -> &GammaDxLo {
        &self.gamma_dx_lo
    }
    #[doc = "0x420 - De-Gamma Curve definition higher x increments (sampling points)"]
    #[inline(always)]
    pub const fn gamma_dx_hi(&self) -> &GammaDxHi {
        &self.gamma_dx_hi
    }
    #[doc = "0x424..0x468 - De-Gamma Curve definition y red n (n=0..16)\n\nNote: The reset values define a linear curve which has the same effect as bypass. \n\n\n\nReset values are: Y_00 = 0x0000, Y_01 = 0x0100, Y_02 = 0x0200, Y_03 = 0x0300, Y_04 \n\n\n\n= 0x0400,Y_05 = 0x0500, Y_06 = 0x0600, Y_07 = 0x0700, Y_08 = 0x0800, Y_09 = \n\n\n\n0x0900, Y_10 = 0x0A00, Y_11 = 0x0B00, Y_12 = 0x0C00, Y_13 = 0x0D00, Y_14 = \n\n\n\n0x0E00, Y_15 = 0x0F00, Y_16 = 0x0FFF \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn gamma_r_y(&self, n: usize) -> &GammaRY {
        &self.gamma_r_y[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x424..0x468 - De-Gamma Curve definition y red n (n=0..16)\n\nNote: The reset values define a linear curve which has the same effect as bypass. \n\n\n\nReset values are: Y_00 = 0x0000, Y_01 = 0x0100, Y_02 = 0x0200, Y_03 = 0x0300, Y_04 \n\n\n\n= 0x0400,Y_05 = 0x0500, Y_06 = 0x0600, Y_07 = 0x0700, Y_08 = 0x0800, Y_09 = \n\n\n\n0x0900, Y_10 = 0x0A00, Y_11 = 0x0B00, Y_12 = 0x0C00, Y_13 = 0x0D00, Y_14 = \n\n\n\n0x0E00, Y_15 = 0x0F00, Y_16 = 0x0FFF \n\n\n\n \n\n"]
    #[inline(always)]
    pub fn gamma_r_y_iter(&self) -> impl Iterator<Item = &GammaRY> {
        self.gamma_r_y.iter()
    }
    #[doc = "0x468..0x4ac - De-Gamma Curve definition y green n (n=0..16)\n\nNote: The reset values define a linear curve which has the same effect as bypass. \n\n\n\nReset values are: Y_00 = 0x0000, Y_01 = 0x0100, Y_02 = 0x0200, Y_03 = 0x0300, Y_04 \n\n\n\n= 0x0400,Y_05 = 0x0500, Y_06 = 0x0600, Y_07 = 0x0700, Y_08 = 0x0800, Y_09 = \n\n\n\n0x0900, Y_10 = 0x0A00, Y_11 = 0x0B00, Y_12 = 0x0C00, Y_13 = 0x0D00, Y_14 = \n\n\n\n0x0E00, Y_15 = 0x0F00, Y_16 = 0x0FFF \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn gamma_g_y(&self, n: usize) -> &GammaGY {
        &self.gamma_g_y[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x468..0x4ac - De-Gamma Curve definition y green n (n=0..16)\n\nNote: The reset values define a linear curve which has the same effect as bypass. \n\n\n\nReset values are: Y_00 = 0x0000, Y_01 = 0x0100, Y_02 = 0x0200, Y_03 = 0x0300, Y_04 \n\n\n\n= 0x0400,Y_05 = 0x0500, Y_06 = 0x0600, Y_07 = 0x0700, Y_08 = 0x0800, Y_09 = \n\n\n\n0x0900, Y_10 = 0x0A00, Y_11 = 0x0B00, Y_12 = 0x0C00, Y_13 = 0x0D00, Y_14 = \n\n\n\n0x0E00, Y_15 = 0x0F00, Y_16 = 0x0FFF \n\n\n\n \n\n"]
    #[inline(always)]
    pub fn gamma_g_y_iter(&self) -> impl Iterator<Item = &GammaGY> {
        self.gamma_g_y.iter()
    }
    #[doc = "0x4ac..0x4f0 - De-Gamma Curve definition y blue n (n=0..16)\n\nNote: The reset values define a linear curve which has the same effect as bypass. \n\n\n\nReset values are: Y_00 = 0x0000, Y_01 = 0x0100, Y_02 = 0x0200, Y_03 = 0x0300, Y_04 \n\n\n\n= 0x0400, \n\n\n\nY_05 = 0x0500, Y_06 = 0x0600, Y_07 = 0x0700, Y_08 = 0x0800, \n\n\n\nY_09 = 0x0900, Y_10 = 0x0A00, Y_11 = 0x0B00, Y_12 = 0x0C00, Y_13 = \n\n\n\n0x0D00, Y_14 = 0x0E00, Y_15 = 0x0F00, Y_16 = 0x0FFF \n\n"]
    #[inline(always)]
    pub const fn gamma_b_y(&self, n: usize) -> &GammaBY {
        &self.gamma_b_y[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x4ac..0x4f0 - De-Gamma Curve definition y blue n (n=0..16)\n\nNote: The reset values define a linear curve which has the same effect as bypass. \n\n\n\nReset values are: Y_00 = 0x0000, Y_01 = 0x0100, Y_02 = 0x0200, Y_03 = 0x0300, Y_04 \n\n\n\n= 0x0400, \n\n\n\nY_05 = 0x0500, Y_06 = 0x0600, Y_07 = 0x0700, Y_08 = 0x0800, \n\n\n\nY_09 = 0x0900, Y_10 = 0x0A00, Y_11 = 0x0B00, Y_12 = 0x0C00, Y_13 = \n\n\n\n0x0D00, Y_14 = 0x0E00, Y_15 = 0x0F00, Y_16 = 0x0FFF \n\n"]
    #[inline(always)]
    pub fn gamma_b_y_iter(&self) -> impl Iterator<Item = &GammaBY> {
        self.gamma_b_y.iter()
    }
    #[doc = "0x510 - Auto white balance properties\n\nNote: The following conversion matrix is used for calculating the YCbCr values: \n\n\n\nY = 16 + 0.2500 R + 0.5000 G + 0.1094 B \n\nCb = 128 - 0.1406 R - 0.2969 G + 0.4375 B \n\nCr = 128 + 0.4375 R - 0.3750 G - 0.0625 B \n\n\n\n"]
    #[inline(always)]
    pub const fn awb_prop(&self) -> &AwbProp {
        &self.awb_prop
    }
    #[doc = "0x514 - Auto white balance horizontal offset of measure window"]
    #[inline(always)]
    pub const fn awb_h_offs(&self) -> &AwbHOffs {
        &self.awb_h_offs
    }
    #[doc = "0x518 - Auto white balance vertical offset of measure window"]
    #[inline(always)]
    pub const fn awb_v_offs(&self) -> &AwbVOffs {
        &self.awb_v_offs
    }
    #[doc = "0x51c - Auto white balance horizontal window size"]
    #[inline(always)]
    pub const fn awb_h_size(&self) -> &AwbHSize {
        &self.awb_h_size
    }
    #[doc = "0x520 - Auto white balance vertical window size"]
    #[inline(always)]
    pub const fn awb_v_size(&self) -> &AwbVSize {
        &self.awb_v_size
    }
    #[doc = "0x524 - Auto white balance mean value over multiple frames"]
    #[inline(always)]
    pub const fn awb_frames(&self) -> &AwbFrames {
        &self.awb_frames
    }
    #[doc = "0x528 - Auto white balance reference Cb/Cr values"]
    #[inline(always)]
    pub const fn awb_ref(&self) -> &AwbRef {
        &self.awb_ref
    }
    #[doc = "0x52c - Auto white balance threshold values"]
    #[inline(always)]
    pub const fn awb_thresh(&self) -> &AwbThresh {
        &self.awb_thresh
    }
    #[doc = "0x538 - Auto white balance gain green"]
    #[inline(always)]
    pub const fn awb_gain_g(&self) -> &AwbGainG {
        &self.awb_gain_g
    }
    #[doc = "0x53c - Auto white balance gain red and blue"]
    #[inline(always)]
    pub const fn awb_gain_rb(&self) -> &AwbGainRb {
        &self.awb_gain_rb
    }
    #[doc = "0x540 - Auto white balance white pixel count"]
    #[inline(always)]
    pub const fn awb_white_cnt(&self) -> &AwbWhiteCnt {
        &self.awb_white_cnt
    }
    #[doc = "0x544 - Auto white balance measured mean value"]
    #[inline(always)]
    pub const fn awb_mean(&self) -> &AwbMean {
        &self.awb_mean
    }
    #[doc = "0x570 - Color conversion coefficient 0\n\nNote: all color conversion coefficients are signed integer values with 7 bit fractional \n\n\n\npart, range -2 to 1.992 \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn cc_coeff_0(&self) -> &CcCoeff0 {
        &self.cc_coeff_0
    }
    #[doc = "0x574 - Color conversion coefficient 1"]
    #[inline(always)]
    pub const fn cc_coeff_1(&self) -> &CcCoeff1 {
        &self.cc_coeff_1
    }
    #[doc = "0x578 - Color conversion coefficient 2"]
    #[inline(always)]
    pub const fn cc_coeff_2(&self) -> &CcCoeff2 {
        &self.cc_coeff_2
    }
    #[doc = "0x57c - Color conversion coefficient 3"]
    #[inline(always)]
    pub const fn cc_coeff_3(&self) -> &CcCoeff3 {
        &self.cc_coeff_3
    }
    #[doc = "0x580 - Color conversion coefficient 4"]
    #[inline(always)]
    pub const fn cc_coeff_4(&self) -> &CcCoeff4 {
        &self.cc_coeff_4
    }
    #[doc = "0x584 - Color conversion coefficient 5"]
    #[inline(always)]
    pub const fn cc_coeff_5(&self) -> &CcCoeff5 {
        &self.cc_coeff_5
    }
    #[doc = "0x588 - Color conversion coefficient 6"]
    #[inline(always)]
    pub const fn cc_coeff_6(&self) -> &CcCoeff6 {
        &self.cc_coeff_6
    }
    #[doc = "0x58c - Color conversion coefficient 7"]
    #[inline(always)]
    pub const fn cc_coeff_7(&self) -> &CcCoeff7 {
        &self.cc_coeff_7
    }
    #[doc = "0x590 - Color conversion coefficient 8"]
    #[inline(always)]
    pub const fn cc_coeff_8(&self) -> &CcCoeff8 {
        &self.cc_coeff_8
    }
    #[doc = "0x594 - Horizontal offset of output window"]
    #[inline(always)]
    pub const fn out_h_offs(&self) -> &OutHOffs {
        &self.out_h_offs
    }
    #[doc = "0x598 - Vertical offset of output window"]
    #[inline(always)]
    pub const fn out_v_offs(&self) -> &OutVOffs {
        &self.out_v_offs
    }
    #[doc = "0x59c - Output horizontal picture size"]
    #[inline(always)]
    pub const fn out_h_size(&self) -> &OutHSize {
        &self.out_h_size
    }
    #[doc = "0x5a0 - Output vertical picture size"]
    #[inline(always)]
    pub const fn out_v_size(&self) -> &OutVSize {
        &self.out_v_size
    }
    #[doc = "0x5a4 - Demosaic parameters"]
    #[inline(always)]
    pub const fn demosaic(&self) -> &Demosaic {
        &self.demosaic
    }
    #[doc = "0x5a8 - Flags (current status) of certain signals and Shadow regs \n\n\n\nfor enable signals"]
    #[inline(always)]
    pub const fn flags_shd(&self) -> &FlagsShd {
        &self.flags_shd
    }
    #[doc = "0x5bc - Interrupt mask"]
    #[inline(always)]
    pub const fn imsc(&self) -> &Imsc {
        &self.imsc
    }
    #[doc = "0x5c0 - Raw interrupt status"]
    #[inline(always)]
    pub const fn ris(&self) -> &Ris {
        &self.ris
    }
    #[doc = "0x5c4 - Masked interrupt status"]
    #[inline(always)]
    pub const fn mis(&self) -> &Mis {
        &self.mis
    }
    #[doc = "0x5c8 - Interrupt clear register"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x5cc - Interrupt set register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x5d0..0x5f4 - cross-talk configuration register (color correction matrix) n (n=0..8)\n\nNote: Reset values generate a matrix which does not modify the pixel values. Reset \n\nvalues are: coeff_0 = 0x80, coeff_1 = 0x00, coeff_2 = 0x00, coeff_3 = 0x00, coeff_4 = 0x80, \n\n\n\ncoeff_5 = 0x00, coeff_6 = 0x00, coeff_7 = 0x00, coeff_8 = 0x80 \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn ct_coeff(&self, n: usize) -> &CtCoeff {
        &self.ct_coeff[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x5d0..0x5f4 - cross-talk configuration register (color correction matrix) n (n=0..8)\n\nNote: Reset values generate a matrix which does not modify the pixel values. Reset \n\nvalues are: coeff_0 = 0x80, coeff_1 = 0x00, coeff_2 = 0x00, coeff_3 = 0x00, coeff_4 = 0x80, \n\n\n\ncoeff_5 = 0x00, coeff_6 = 0x00, coeff_7 = 0x00, coeff_8 = 0x80 \n\n\n\n \n\n"]
    #[inline(always)]
    pub fn ct_coeff_iter(&self) -> impl Iterator<Item = &CtCoeff> {
        self.ct_coeff.iter()
    }
    #[doc = "0x5f4 - gamma segmentation mode register for output gamma"]
    #[inline(always)]
    pub const fn gamma_out_mode(&self) -> &GammaOutMode {
        &self.gamma_out_mode
    }
    #[doc = "0x5f8..0x63c - Gamma Out Curve definition y_ n (n=0..16)\n\nNote: Reset values generate a standard gamma of 2.2. Reset values are: \n\ny_00 = 0x000, y_01 = 0x049, y_02 = 0x089, y_03 = 0x0B7, y_04 = 0x0DF, y_05 = \n\n\n\n0x11F, y_06 = 0x154, y_07 = 0x183, y_08 = 0x1AD, y_09 = 0x1F6, y_10 = 0x235, y_11 = \n\n0x26F, y_12 = 0x2D3, y_13 = 0x32A, y_14 = 0x378, y_15 = 0x3BF, y_16 = 0x3FF \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn gamma_out_y(&self, n: usize) -> &GammaOutY {
        &self.gamma_out_y[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x5f8..0x63c - Gamma Out Curve definition y_ n (n=0..16)\n\nNote: Reset values generate a standard gamma of 2.2. Reset values are: \n\ny_00 = 0x000, y_01 = 0x049, y_02 = 0x089, y_03 = 0x0B7, y_04 = 0x0DF, y_05 = \n\n\n\n0x11F, y_06 = 0x154, y_07 = 0x183, y_08 = 0x1AD, y_09 = 0x1F6, y_10 = 0x235, y_11 = \n\n0x26F, y_12 = 0x2D3, y_13 = 0x32A, y_14 = 0x378, y_15 = 0x3BF, y_16 = 0x3FF \n\n\n\n \n\n"]
    #[inline(always)]
    pub fn gamma_out_y_iter(&self) -> impl Iterator<Item = &GammaOutY> {
        self.gamma_out_y.iter()
    }
    #[doc = "0x63c - ISP error register\n\nNote: For debug purposes the ISP_ERR und ISP_ERR_CLR are implemented. For the case \n\nwhen a PIC_SIZE_ERR interrupt is signaled the SW is able to see in which submodule this error \n\nis generated. Writing to the ISP_ERR_CLR register clears this bit. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn err(&self) -> &Err {
        &self.err
    }
    #[doc = "0x640 - ISP error clear register"]
    #[inline(always)]
    pub const fn err_clr(&self) -> &ErrClr {
        &self.err_clr
    }
    #[doc = "0x644 - Frame counter\n\nNote: In the ISP_FRAME_COUNT register the number of processed frames are displayed. \n\nFor example: If a 8 is programmed into the ISP_ACQ_NR_FRAMES register, a read access to \n\nthe ISP_FRAME_COUNT register during processing of the first picture shows a 7. \n\n\n\nAfter the entire frames are processed the ISP_OFF interrupt is generated and the \n\nISP_FRAME_COUNT has the count zero. In case a '0' is programmed into the \n\nISP_ACQ_NR_FRAMES register (continues mode) the ISP_FRAME_COUNT register keeps the \n\nvalue '0'. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn frame_count(&self) -> &FrameCount {
        &self.frame_count
    }
    #[doc = "0x648 - cross-talk offset red"]
    #[inline(always)]
    pub const fn ct_offset_r(&self) -> &CtOffsetR {
        &self.ct_offset_r
    }
    #[doc = "0x64c - cross-talk offset green"]
    #[inline(always)]
    pub const fn ct_offset_g(&self) -> &CtOffsetG {
        &self.ct_offset_g
    }
    #[doc = "0x650 - cross-talk offset blue"]
    #[inline(always)]
    pub const fn ct_offset_b(&self) -> &CtOffsetB {
        &self.ct_offset_b
    }
    #[doc = "0x660 - Flash command\n\nNote: This is the command register for flash light and prelight activation. If the 'rw' bits \n\n(e.g. 'fl_cap_del') are re-programmed during operation, the following scheme shall be \n\napplied: \n\n\n\nprelight is active (prelight_on = 1 has been set before): Every write access to this register \n\nshall use prelight_on = 1 (to prevent undesired switch off of the prelight). \n\n\n\nprelight is off: Every write access to this register shall use prelight_on = 0 (to prevent \n\nundesired switch on of the prelight). \n\n\n\n"]
    #[inline(always)]
    pub const fn flash_cmd(&self) -> &FlashCmd {
        &self.flash_cmd
    }
    #[doc = "0x664 - Flash config"]
    #[inline(always)]
    pub const fn flash_config(&self) -> &FlashConfig {
        &self.flash_config
    }
    #[doc = "0x668 - Flash Counter Pre-Divider"]
    #[inline(always)]
    pub const fn flash_prediv(&self) -> &FlashPrediv {
        &self.flash_prediv
    }
    #[doc = "0x66c - Flash Delay\n\nNote: Example: \n\nfl_delay = (10s * 100MHz) / (1023 + 1) – 1 = 976561 \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn flash_delay(&self) -> &FlashDelay {
        &self.flash_delay
    }
    #[doc = "0x670 - Flash time\n\nNote: Example: \n\nfl_time = (500ms * 100MHz) / (700 + 1) – 1 = 71530 \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn flash_time(&self) -> &FlashTime {
        &self.flash_time
    }
    #[doc = "0x674 - Maximum value for flash or preflash\n\nNote: Example: \n\n\n\nfl_maxp = (10s * 100MHz) / (16384) – 1 = 61034 \n\n\n\n"]
    #[inline(always)]
    pub const fn flash_maxp(&self) -> &FlashMaxp {
        &self.flash_maxp
    }
    #[doc = "0x680 - mechanical shutter control"]
    #[inline(always)]
    pub const fn sh_ctrl(&self) -> &ShCtrl {
        &self.sh_ctrl
    }
    #[doc = "0x684 - Mech. Shutter Counter Pre-Divider"]
    #[inline(always)]
    pub const fn sh_prediv(&self) -> &ShPrediv {
        &self.sh_prediv
    }
    #[doc = "0x688 - Delay register\n\nNote: Example: \n\nsh_delay = (250us * 100MHz) / (50 + 1) – 1 = 489 \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn sh_delay(&self) -> &ShDelay {
        &self.sh_delay
    }
    #[doc = "0x68c - Time register\n\nNote: Example: \n\n\n\nsh_time = (10s * 100MHz) / (1023 + 1) – 1 = 976561 \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn sh_time(&self) -> &ShTime {
        &self.sh_time
    }
    #[doc = "0x800 - Global control register"]
    #[inline(always)]
    pub const fn cproc_ctrl(&self) -> &CprocCtrl {
        &self.cproc_ctrl
    }
    #[doc = "0x804 - Color Processing contrast register"]
    #[inline(always)]
    pub const fn cproc_contrast(&self) -> &CprocContrast {
        &self.cproc_contrast
    }
    #[doc = "0x808 - Color Processing brightness register"]
    #[inline(always)]
    pub const fn cproc_brightness(&self) -> &CprocBrightness {
        &self.cproc_brightness
    }
    #[doc = "0x80c - Color Processing saturation register"]
    #[inline(always)]
    pub const fn cproc_saturation(&self) -> &CprocSaturation {
        &self.cproc_saturation
    }
    #[doc = "0x810 - Color Processing hue register"]
    #[inline(always)]
    pub const fn cproc_hue(&self) -> &CprocHue {
        &self.cproc_hue
    }
    #[doc = "0xc00 - global control register"]
    #[inline(always)]
    pub const fn mrsz_ctrl(&self) -> &MrszCtrl {
        &self.mrsz_ctrl
    }
    #[doc = "0xc04 - horizontal luminance scale factor register\n\nNote: The size of the output picture is calculated as follows: \n\n\n\n \n\nupscaling: (size_in - 1) / (size_out - 1)) = scale downscaling: (size_out - 1) / (size_in - 1)) \n\n\n\n= scale, \n\n \n\nwhere size_in/out is the width or height of the in/output picture. The value of the \n\n\n\nrespective MRSZ_SCALE register then has to be \n\nint(scale x 2^14) for upscaling and \n\nint(scale x 2^14)+1 for downscaling. \n\nFor downscaling this formula has no restriction. In upscaling processes the limit is factor 5. \n\n\n\nThe output is at max. 5 MegaPixel. \n\nIf a format conversion is performed, the scale factors have to be different for the \n\n\n\nluminance and the chrominance component, respectively. For example, for a format \n\nconversion from 4:2:2 to 4:2:0 the scale register value for the vertical \n\n\n\nchrominance component should be half of the vertical luminance scale register value. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mrsz_scale_hy(&self) -> &MrszScaleHy {
        &self.mrsz_scale_hy
    }
    #[doc = "0xc08 - horizontal Cb scale factor register"]
    #[inline(always)]
    pub const fn mrsz_scale_hcb(&self) -> &MrszScaleHcb {
        &self.mrsz_scale_hcb
    }
    #[doc = "0xc0c - horizontal Cr scale factor register"]
    #[inline(always)]
    pub const fn mrsz_scale_hcr(&self) -> &MrszScaleHcr {
        &self.mrsz_scale_hcr
    }
    #[doc = "0xc10 - vertical luminance scale factor register"]
    #[inline(always)]
    pub const fn mrsz_scale_vy(&self) -> &MrszScaleVy {
        &self.mrsz_scale_vy
    }
    #[doc = "0xc14 - vertical chrominance scale factor register\n\nNote: The size of the output picture is calculated as follows: (size_out - 1) / (size_in - 1)) \n\n\n\n= scale, \n\nwhere size_in/out is the width or heigth of the in/output picture. The values of the \n\n\n\nMRSZ_SCALE registers then have to be int(scale x 2^14)+1 \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mrsz_scale_vc(&self) -> &MrszScaleVc {
        &self.mrsz_scale_vc
    }
    #[doc = "0xc18 - horizontal luminance phase register"]
    #[inline(always)]
    pub const fn mrsz_phase_hy(&self) -> &MrszPhaseHy {
        &self.mrsz_phase_hy
    }
    #[doc = "0xc1c - horizontal chrominance phase register"]
    #[inline(always)]
    pub const fn mrsz_phase_hc(&self) -> &MrszPhaseHc {
        &self.mrsz_phase_hc
    }
    #[doc = "0xc20 - vertical luminance phase register"]
    #[inline(always)]
    pub const fn mrsz_phase_vy(&self) -> &MrszPhaseVy {
        &self.mrsz_phase_vy
    }
    #[doc = "0xc24 - vertical chrominance phase register"]
    #[inline(always)]
    pub const fn mrsz_phase_vc(&self) -> &MrszPhaseVc {
        &self.mrsz_phase_vc
    }
    #[doc = "0xc28 - Address pointer of up-scaling look up table"]
    #[inline(always)]
    pub const fn mrsz_scale_lut_addr(&self) -> &MrszScaleLutAddr {
        &self.mrsz_scale_lut_addr
    }
    #[doc = "0xc2c - Entry of up-scaling look up table"]
    #[inline(always)]
    pub const fn mrsz_scale_lut(&self) -> &MrszScaleLut {
        &self.mrsz_scale_lut
    }
    #[doc = "0xc30 - global control shadow register"]
    #[inline(always)]
    pub const fn mrsz_ctrl_shd(&self) -> &MrszCtrlShd {
        &self.mrsz_ctrl_shd
    }
    #[doc = "0xc34 - horizontal luminance scale factor shadow register"]
    #[inline(always)]
    pub const fn mrsz_scale_hy_shd(&self) -> &MrszScaleHyShd {
        &self.mrsz_scale_hy_shd
    }
    #[doc = "0xc38 - horizontal Cb scale factor shadow register"]
    #[inline(always)]
    pub const fn mrsz_scale_hcb_shd(&self) -> &MrszScaleHcbShd {
        &self.mrsz_scale_hcb_shd
    }
    #[doc = "0xc3c - horizontal Cr scale factor shadow register"]
    #[inline(always)]
    pub const fn mrsz_scale_hcr_shd(&self) -> &MrszScaleHcrShd {
        &self.mrsz_scale_hcr_shd
    }
    #[doc = "0xc40 - vertical luminance scale factor shadow register"]
    #[inline(always)]
    pub const fn mrsz_scale_vy_shd(&self) -> &MrszScaleVyShd {
        &self.mrsz_scale_vy_shd
    }
    #[doc = "0xc44 - vertical chrominance scale factor shadow register"]
    #[inline(always)]
    pub const fn mrsz_scale_vc_shd(&self) -> &MrszScaleVcShd {
        &self.mrsz_scale_vc_shd
    }
    #[doc = "0xc48 - horizontal luminance phase shadow register"]
    #[inline(always)]
    pub const fn mrsz_phase_hy_shd(&self) -> &MrszPhaseHyShd {
        &self.mrsz_phase_hy_shd
    }
    #[doc = "0xc4c - horizontal chrominance phase shadow register"]
    #[inline(always)]
    pub const fn mrsz_phase_hc_shd(&self) -> &MrszPhaseHcShd {
        &self.mrsz_phase_hc_shd
    }
    #[doc = "0xc50 - vertical luminance phase shadow register"]
    #[inline(always)]
    pub const fn mrsz_phase_vy_shd(&self) -> &MrszPhaseVyShd {
        &self.mrsz_phase_vy_shd
    }
    #[doc = "0xc54 - vertical chrominance phase shadow register"]
    #[inline(always)]
    pub const fn mrsz_phase_vc_shd(&self) -> &MrszPhaseVcShd {
        &self.mrsz_phase_vc_shd
    }
    #[doc = "0x1000 - global control register"]
    #[inline(always)]
    pub const fn srsz_ctrl(&self) -> &SrszCtrl {
        &self.srsz_ctrl
    }
    #[doc = "0x1004 - horizontal luminance scale factor register\n\nNote: The size of the output picture is calculated as follows: \n\nupscaling: (size_in - 1) / (size_out - 1)) = scale downscaling: (size_out - 1) / (size_in - 1)) \n\n\n\n= scale, \n\nwhere size_in/out is the width or height of the in/output picture. The value of the \n\n\n\nrespective SRSZ_SCALE register then has to be \n\nint(scale x 2^14) for upscaling and \n\nint(scale x 2^14)+1 for downscaling. \n\nFor downscaling this formula has no restriction. In upscaling processes the limit is factor 5. \n\n\n\nIf a format conversion is performed, the scale factors have to be different for the luminance \n\nand the chrominance component, respectively. For example, for a \n\n\n\nformat conversion from 4:2:2 to 4:2:0 the scale register value for the vertical \n\nchrominance component should be half of the vertical luminance scale register value. \n\n\n\n"]
    #[inline(always)]
    pub const fn srsz_scale_hy(&self) -> &SrszScaleHy {
        &self.srsz_scale_hy
    }
    #[doc = "0x1008 - horizontal chrominance scale factor register"]
    #[inline(always)]
    pub const fn srsz_scale_hcb(&self) -> &SrszScaleHcb {
        &self.srsz_scale_hcb
    }
    #[doc = "0x100c - horizontal chrominance scale factor register"]
    #[inline(always)]
    pub const fn srsz_scale_hcr(&self) -> &SrszScaleHcr {
        &self.srsz_scale_hcr
    }
    #[doc = "0x1010 - vertical luminance scale factor register"]
    #[inline(always)]
    pub const fn srsz_scale_vy(&self) -> &SrszScaleVy {
        &self.srsz_scale_vy
    }
    #[doc = "0x1014 - vertical chrominance scale factor register"]
    #[inline(always)]
    pub const fn srsz_scale_vc(&self) -> &SrszScaleVc {
        &self.srsz_scale_vc
    }
    #[doc = "0x1018 - horizontal luminance phase register"]
    #[inline(always)]
    pub const fn srsz_phase_hy(&self) -> &SrszPhaseHy {
        &self.srsz_phase_hy
    }
    #[doc = "0x101c - horizontal chrominance phase register"]
    #[inline(always)]
    pub const fn srsz_phase_hc(&self) -> &SrszPhaseHc {
        &self.srsz_phase_hc
    }
    #[doc = "0x1020 - vertical luminance phase register"]
    #[inline(always)]
    pub const fn srsz_phase_vy(&self) -> &SrszPhaseVy {
        &self.srsz_phase_vy
    }
    #[doc = "0x1024 - vertical chrominance phase register"]
    #[inline(always)]
    pub const fn srsz_phase_vc(&self) -> &SrszPhaseVc {
        &self.srsz_phase_vc
    }
    #[doc = "0x1028 - Address pointer of up-scaling look up table"]
    #[inline(always)]
    pub const fn srsz_scale_lut_addr(&self) -> &SrszScaleLutAddr {
        &self.srsz_scale_lut_addr
    }
    #[doc = "0x102c - Entry of up-scaling look up table"]
    #[inline(always)]
    pub const fn srsz_scale_lut(&self) -> &SrszScaleLut {
        &self.srsz_scale_lut
    }
    #[doc = "0x1030 - global control shadow register"]
    #[inline(always)]
    pub const fn srsz_ctrl_shd(&self) -> &SrszCtrlShd {
        &self.srsz_ctrl_shd
    }
    #[doc = "0x1034 - horizontal luminance scale factor shadow register"]
    #[inline(always)]
    pub const fn srsz_scale_hy_shd(&self) -> &SrszScaleHyShd {
        &self.srsz_scale_hy_shd
    }
    #[doc = "0x1038 - horizontal Cb scale factor shadow register"]
    #[inline(always)]
    pub const fn srsz_scale_hcb_shd(&self) -> &SrszScaleHcbShd {
        &self.srsz_scale_hcb_shd
    }
    #[doc = "0x103c - horizontal Cr scale factor shadow register"]
    #[inline(always)]
    pub const fn srsz_scale_hcr_shd(&self) -> &SrszScaleHcrShd {
        &self.srsz_scale_hcr_shd
    }
    #[doc = "0x1040 - vertical luminance scale factor shadow register"]
    #[inline(always)]
    pub const fn srsz_scale_vy_shd(&self) -> &SrszScaleVyShd {
        &self.srsz_scale_vy_shd
    }
    #[doc = "0x1044 - vertical chrominance scale factor shadow register"]
    #[inline(always)]
    pub const fn srsz_scale_vc_shd(&self) -> &SrszScaleVcShd {
        &self.srsz_scale_vc_shd
    }
    #[doc = "0x1048 - horizontal luminance phase shadow register"]
    #[inline(always)]
    pub const fn srsz_phase_hy_shd(&self) -> &SrszPhaseHyShd {
        &self.srsz_phase_hy_shd
    }
    #[doc = "0x104c - horizontal chrominance phase shadow register"]
    #[inline(always)]
    pub const fn srsz_phase_hc_shd(&self) -> &SrszPhaseHcShd {
        &self.srsz_phase_hc_shd
    }
    #[doc = "0x1050 - vertical luminance phase shadow register"]
    #[inline(always)]
    pub const fn srsz_phase_vy_shd(&self) -> &SrszPhaseVyShd {
        &self.srsz_phase_vy_shd
    }
    #[doc = "0x1054 - vertical chrominance phase shadow register"]
    #[inline(always)]
    pub const fn srsz_phase_vc_shd(&self) -> &SrszPhaseVcShd {
        &self.srsz_phase_vc_shd
    }
    #[doc = "0x1400 - Global control register"]
    #[inline(always)]
    pub const fn mi_ctrl(&self) -> &MiCtrl {
        &self.mi_ctrl
    }
    #[doc = "0x1404 - Control register for address init and skip function"]
    #[inline(always)]
    pub const fn mi_init(&self) -> &MiInit {
        &self.mi_init
    }
    #[doc = "0x1408 - Base address for main picture Y component, JPEG or raw data\n\nNote: This register protects from non-aligned access. The bits 0 to 2 are hard wired to \n\n'000'. As a consequence any byte address that is written to the register will automatically be \n\nre-mapped to the next lower 64 bit aligned address: write(MI_MP_Y_BASE_AD_INIT, \n\naddress_value) is equivalent to write(MI_Y_BASE_AD_INIT, address_value &amp; 0xFFFFFFF8). \n\nAnyhow, in order to avoid confusion it is NOT recommended to use non-aligned address values \n\nfor access. It is also NOT recommended to actively consider the register slice for register \n\naccess in order to avoid unneccessary mask and shift operations. \n\n\n\nIn addition, if ISP provides AXI interfaces the programmed base address shall be \n\n\n\nburst aligned with respect to the burst length configured in MI_CTRL . \n\n\n\nSet control bit init_base_en before updating so that a forced or automatic update can take \n\n\n\neffect. \n\n"]
    #[inline(always)]
    pub const fn mi_mp_y_base_ad_init(&self) -> &MiMpYBaseAdInit {
        &self.mi_mp_y_base_ad_init
    }
    #[doc = "0x140c - Size of main picture Y component, JPEG or raw data\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nSet control bit init_base_en before updating so that a forced or automatic update can take \n\neffect. \n\n\n\n \n\n\n\n"]
    #[inline(always)]
    pub const fn mi_mp_y_size_init(&self) -> &MiMpYSizeInit {
        &self.mi_mp_y_size_init
    }
    #[doc = "0x1410 - Offset counter init value for main picture Y, JPEG or raw data\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nSet control bit init_base_en before updating so that a forced or automatic update can \n\n\n\ntake effect. Check exceptional handling in skip modes. \n\n\n\n"]
    #[inline(always)]
    pub const fn mi_mp_y_offs_cnt_init(&self) -> &MiMpYOffsCntInit {
        &self.mi_mp_y_offs_cnt_init
    }
    #[doc = "0x1414 - Offset counter start value for main picture Y, JPEG or raw data\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_mp_y_offs_cnt_start(&self) -> &MiMpYOffsCntStart {
        &self.mi_mp_y_offs_cnt_start
    }
    #[doc = "0x1418 - Fill level interrupt offset value for main picture Y, JPEG or raw data\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_mp_y_irq_offs_init(&self) -> &MiMpYIrqOffsInit {
        &self.mi_mp_y_irq_offs_init
    }
    #[doc = "0x141c - Base address for main picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_mp_cb_base_ad_init(&self) -> &MiMpCbBaseAdInit {
        &self.mi_mp_cb_base_ad_init
    }
    #[doc = "0x1420 - Size of main picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_mp_cb_size_init(&self) -> &MiMpCbSizeInit {
        &self.mi_mp_cb_size_init
    }
    #[doc = "0x1424 - Offset counter init value for main picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_mp_cb_offs_cnt_init(&self) -> &MiMpCbOffsCntInit {
        &self.mi_mp_cb_offs_cnt_init
    }
    #[doc = "0x1428 - Offset counter start value for main picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_mp_cb_offs_cnt_start(&self) -> &MiMpCbOffsCntStart {
        &self.mi_mp_cb_offs_cnt_start
    }
    #[doc = "0x142c - Base address for main picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_mp_cr_base_ad_init(&self) -> &MiMpCrBaseAdInit {
        &self.mi_mp_cr_base_ad_init
    }
    #[doc = "0x1430 - Size of main picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_mp_cr_size_init(&self) -> &MiMpCrSizeInit {
        &self.mi_mp_cr_size_init
    }
    #[doc = "0x1434 - Offset counter init value for main picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_mp_cr_offs_cnt_init(&self) -> &MiMpCrOffsCntInit {
        &self.mi_mp_cr_offs_cnt_init
    }
    #[doc = "0x1438 - Offset counter start value for main picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_mp_cr_offs_cnt_start(&self) -> &MiMpCrOffsCntStart {
        &self.mi_mp_cr_offs_cnt_start
    }
    #[doc = "0x143c - Base address for self picture Y component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_sp_y_base_ad_init(&self) -> &MiSpYBaseAdInit {
        &self.mi_sp_y_base_ad_init
    }
    #[doc = "0x1440 - Size of self picture Y component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_sp_y_size_init(&self) -> &MiSpYSizeInit {
        &self.mi_sp_y_size_init
    }
    #[doc = "0x1444 - Offset counter init value for self picture Y component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_sp_y_offs_cnt_init(&self) -> &MiSpYOffsCntInit {
        &self.mi_sp_y_offs_cnt_init
    }
    #[doc = "0x1448 - Offset counter start value for self picture Y component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_sp_y_offs_cnt_start(&self) -> &MiSpYOffsCntStart {
        &self.mi_sp_y_offs_cnt_start
    }
    #[doc = "0x144c - Line length of self picture Y component\n\nNote: Programmed value becomes effective \n\nimmediately. So write to the register only if no picture data \n\nis sent to the self path. \n\n\n\n \n\n\n\n"]
    #[inline(always)]
    pub const fn mi_sp_y_llength(&self) -> &MiSpYLlength {
        &self.mi_sp_y_llength
    }
    #[doc = "0x1450 - Base address for self picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_sp_cb_base_ad_init(&self) -> &MiSpCbBaseAdInit {
        &self.mi_sp_cb_base_ad_init
    }
    #[doc = "0x1454 - Size of self picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_sp_cb_size_init(&self) -> &MiSpCbSizeInit {
        &self.mi_sp_cb_size_init
    }
    #[doc = "0x1458 - Offset counter init value for self picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n \n\n"]
    #[inline(always)]
    pub const fn mi_sp_cb_offs_cnt_init(&self) -> &MiSpCbOffsCntInit {
        &self.mi_sp_cb_offs_cnt_init
    }
    #[doc = "0x145c - Offset counter start value for self picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n\n\n"]
    #[inline(always)]
    pub const fn mi_sp_cb_offs_cnt_start(&self) -> &MiSpCbOffsCntStart {
        &self.mi_sp_cb_offs_cnt_start
    }
    #[doc = "0x1460 - Base address for self picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_sp_cr_base_ad_init(&self) -> &MiSpCrBaseAdInit {
        &self.mi_sp_cr_base_ad_init
    }
    #[doc = "0x1464 - Size of self picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_sp_cr_size_init(&self) -> &MiSpCrSizeInit {
        &self.mi_sp_cr_size_init
    }
    #[doc = "0x1468 - Offset counter init value for self picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_sp_cr_offs_cnt_init(&self) -> &MiSpCrOffsCntInit {
        &self.mi_sp_cr_offs_cnt_init
    }
    #[doc = "0x146c - Offset counter start value for self picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_sp_cr_offs_cnt_start(&self) -> &MiSpCrOffsCntStart {
        &self.mi_sp_cr_offs_cnt_start
    }
    #[doc = "0x1470 - Counter value of JPEG or RAW data bytes"]
    #[inline(always)]
    pub const fn mi_byte_cnt(&self) -> &MiByteCnt {
        &self.mi_byte_cnt
    }
    #[doc = "0x1474 - global control internal shadow register"]
    #[inline(always)]
    pub const fn mi_ctrl_shd(&self) -> &MiCtrlShd {
        &self.mi_ctrl_shd
    }
    #[doc = "0x1478 - Base address shadow register for main picture Y \n\n\n\ncomponent, JPEG or raw data ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n"]
    #[inline(always)]
    pub const fn mi_mp_y_base_ad_shd(&self) -> &MiMpYBaseAdShd {
        &self.mi_mp_y_base_ad_shd
    }
    #[doc = "0x147c - Size shadow register of main picture Y component, JPEG \n\n\n\nor raw data\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_mp_y_size_shd(&self) -> &MiMpYSizeShd {
        &self.mi_mp_y_size_shd
    }
    #[doc = "0x1480 - Current offset counter of main picture Y component, JPEG \n\n\n\nor raw data ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_mp_y_offs_cnt_shd(&self) -> &MiMpYOffsCntShd {
        &self.mi_mp_y_offs_cnt_shd
    }
    #[doc = "0x1484 - Shadow register of fill level interrupt offset value for main \n\n\n\npicture Y component, JPEG or raw data\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n"]
    #[inline(always)]
    pub const fn mi_mp_y_irq_offs_shd(&self) -> &MiMpYIrqOffsShd {
        &self.mi_mp_y_irq_offs_shd
    }
    #[doc = "0x1488 - Base address shadow register for main picture Cb \n\n\n\ncomponent ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_mp_cb_base_ad_shd(&self) -> &MiMpCbBaseAdShd {
        &self.mi_mp_cb_base_ad_shd
    }
    #[doc = "0x148c - Size shadow register of main picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_mp_cb_size_shd(&self) -> &MiMpCbSizeShd {
        &self.mi_mp_cb_size_shd
    }
    #[doc = "0x1490 - Current offset counter of main picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n"]
    #[inline(always)]
    pub const fn mi_mp_cb_offs_cnt_shd(&self) -> &MiMpCbOffsCntShd {
        &self.mi_mp_cb_offs_cnt_shd
    }
    #[doc = "0x1494 - Base address shadow register for main picture Cr \n\n\n\ncomponent ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_mp_cr_base_ad_shd(&self) -> &MiMpCrBaseAdShd {
        &self.mi_mp_cr_base_ad_shd
    }
    #[doc = "0x1498 - Size shadow register of main picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_mp_cr_size_shd(&self) -> &MiMpCrSizeShd {
        &self.mi_mp_cr_size_shd
    }
    #[doc = "0x149c - Current offset counter of main picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n"]
    #[inline(always)]
    pub const fn mi_mp_cr_offs_cnt_shd(&self) -> &MiMpCrOffsCntShd {
        &self.mi_mp_cr_offs_cnt_shd
    }
    #[doc = "0x14a0 - Base address shadow register for self picture Y \n\n\n\ncomponent ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_sp_y_base_ad_shd(&self) -> &MiSpYBaseAdShd {
        &self.mi_sp_y_base_ad_shd
    }
    #[doc = "0x14a4 - Size shadow register of self picture Y component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_sp_y_size_shd(&self) -> &MiSpYSizeShd {
        &self.mi_sp_y_size_shd
    }
    #[doc = "0x14a8 - Current offset counter of self picture Y component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n"]
    #[inline(always)]
    pub const fn mi_sp_y_offs_cnt_shd(&self) -> &MiSpYOffsCntShd {
        &self.mi_sp_y_offs_cnt_shd
    }
    #[doc = "0x14b0 - Base address shadow register for self picture Cb \n\n\n\ncomponent ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_sp_cb_base_ad_shd(&self) -> &MiSpCbBaseAdShd {
        &self.mi_sp_cb_base_ad_shd
    }
    #[doc = "0x14b4 - Size shadow register of self picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_sp_cb_size_shd(&self) -> &MiSpCbSizeShd {
        &self.mi_sp_cb_size_shd
    }
    #[doc = "0x14b8 - Current offset counter of self picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n"]
    #[inline(always)]
    pub const fn mi_sp_cb_offs_cnt_shd(&self) -> &MiSpCbOffsCntShd {
        &self.mi_sp_cb_offs_cnt_shd
    }
    #[doc = "0x14bc - Base address shadow register for self picture Cr \n\n\n\ncomponent ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_sp_cr_base_ad_shd(&self) -> &MiSpCrBaseAdShd {
        &self.mi_sp_cr_base_ad_shd
    }
    #[doc = "0x14c0 - Size shadow register of self picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_sp_cr_size_shd(&self) -> &MiSpCrSizeShd {
        &self.mi_sp_cr_size_shd
    }
    #[doc = "0x14c4 - Current offset counter of self picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n"]
    #[inline(always)]
    pub const fn mi_sp_cr_offs_cnt_shd(&self) -> &MiSpCrOffsCntShd {
        &self.mi_sp_cr_offs_cnt_shd
    }
    #[doc = "0x14c8 - Y component image start address\n\nNote: Must be multiple of 4 in interleaved mode. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_dma_y_pic_start_ad(&self) -> &MiDmaYPicStartAd {
        &self.mi_dma_y_pic_start_ad
    }
    #[doc = "0x14cc - Y component image width"]
    #[inline(always)]
    pub const fn mi_dma_y_pic_width(&self) -> &MiDmaYPicWidth {
        &self.mi_dma_y_pic_width
    }
    #[doc = "0x14d0 - Y component original line length"]
    #[inline(always)]
    pub const fn mi_dma_y_llength(&self) -> &MiDmaYLlength {
        &self.mi_dma_y_llength
    }
    #[doc = "0x14d4 - Y component image size"]
    #[inline(always)]
    pub const fn mi_dma_y_pic_size(&self) -> &MiDmaYPicSize {
        &self.mi_dma_y_pic_size
    }
    #[doc = "0x14d8 - Cb component image start address\n\nNote: Must be multiple of 2 in semi-planar mode. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_dma_cb_pic_start_ad(&self) -> &MiDmaCbPicStartAd {
        &self.mi_dma_cb_pic_start_ad
    }
    #[doc = "0x14e8 - Cr component image start address"]
    #[inline(always)]
    pub const fn mi_dma_cr_pic_start_ad(&self) -> &MiDmaCrPicStartAd {
        &self.mi_dma_cr_pic_start_ad
    }
    #[doc = "0x14f8 - Interrupt Mask („1‟: interrupt active, „0‟: interrupt masked)"]
    #[inline(always)]
    pub const fn mi_imsc(&self) -> &MiImsc {
        &self.mi_imsc
    }
    #[doc = "0x14fc - Raw Interrupt Status"]
    #[inline(always)]
    pub const fn mi_ris(&self) -> &MiRis {
        &self.mi_ris
    }
    #[doc = "0x1500 - Masked Interrupt Status"]
    #[inline(always)]
    pub const fn mi_mis(&self) -> &MiMis {
        &self.mi_mis
    }
    #[doc = "0x1504 - Interrupt Clear Register"]
    #[inline(always)]
    pub const fn mi_icr(&self) -> &MiIcr {
        &self.mi_icr
    }
    #[doc = "0x1508 - Interrupt Set Register"]
    #[inline(always)]
    pub const fn mi_isr(&self) -> &MiIsr {
        &self.mi_isr
    }
    #[doc = "0x150c - MI Status Register"]
    #[inline(always)]
    pub const fn mi_status(&self) -> &MiStatus {
        &self.mi_status
    }
    #[doc = "0x1510 - MI Status Clear Register"]
    #[inline(always)]
    pub const fn mi_status_clr(&self) -> &MiStatusClr {
        &self.mi_status_clr
    }
    #[doc = "0x1514 - Y component image width\n\nNote: Programmed value becomes effective \n\nimmediately. So write to the register only if no picture \n\ndata is sent to the self path. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_sp_y_pic_width(&self) -> &MiSpYPicWidth {
        &self.mi_sp_y_pic_width
    }
    #[doc = "0x1518 - Y component image height\n\nNote: Programmed value becomes effective \n\n"]
    #[inline(always)]
    pub const fn mi_sp_y_pic_height(&self) -> &MiSpYPicHeight {
        &self.mi_sp_y_pic_height
    }
    #[doc = "0x151c - Y component image size\n\nNote: Programmed value becomes effective \n\n\n\nimmediately. So write to the register only if no picture \n\n\n\ndata is sent to the self path. \n\n"]
    #[inline(always)]
    pub const fn mi_sp_y_pic_size(&self) -> &MiSpYPicSize {
        &self.mi_sp_y_pic_size
    }
    #[doc = "0x1520 - DMA control register\n\nNote: The dma_ready \n\ninterrupt is raised as usual, but the dma_frame_end \n\ninterrupt will not be generated until v_end has been \n\nenabled again. \n\n\n\n9 dma_continuous_en Enables continuous mode. If set the same frame is \n\nread back over and over. A start pulse on dma_start is \n\nneeded only for the first time. To stop continuous mode \n\nreset this bit (takes effect after the next frame end) or \n\nexecute a soft reset. This bit is intended to be used in \n\nconjunction with the Superimpose feature. \n\n\n\n8 dma_byte_swap Enables change of DMA byte order of the 32 bit \n\n\n\ninput word at read port \n\n1: byte order is mirrored but the bit order within one \n\n\n\nbyte doesn‟t change \n\n\n\n0: no byte mirroring \n\n\n\n7:6 dma_inout_format Selects input/output format of \n\n\n\nDMA picture. 11: YCbCr 4:4:4 \n\n\n\n10: YCbCr 4:2:2 \n\n\n\n01: YCbCr 4:2:0 \n\n\n\n00: YCbCr 4:0:0 \n\n\n\n5:4 dma_read_format Defines how YCbCr picture data is read from \n\n\n\nmemory. 00: planar \n\n01: semi planar, for YCbCr 4:2:x \n\n10: interleaved (combined), for YCbCr 4:2:2 and RGB \n\n\n\nonly 11: reserved \n\n\n\n3:2 dma_burst_len_chrom Burst length for Cb or Cr data affecting DMA \n\n\n\nread port. 00: 4-beat bursts \n\n\n\n01: 8-beat bursts \n\n\n\n10: 16-beat bursts \n\n11: reserved \n\nIgnored if 8- or 16-beat bursts are not supported. \n\n\n\nDMA control register Reset value: 0000'0000H \n\n \n\n\n\nAddress: ISP_MI_BASE + 0120H Mode : rw \n\n\n\nBit\n\n\n\ns \n\n\n\nName Description \n\n\n\n1:0 dma_burst_len_lum Burst length for Y data affecting DMA read port. \n\n\n\n00: 4-beat bursts \n\n\n\n01: 8-beat bursts \n\n10: 16-beat bursts \n\n\n\n11: reserved \n\n\n\nIgnored if 8- or 16-beat bursts are not supported. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_dma_ctrl(&self) -> &MiDmaCtrl {
        &self.mi_dma_ctrl
    }
    #[doc = "0x1524 - DMA start register"]
    #[inline(always)]
    pub const fn mi_dma_start(&self) -> &MiDmaStart {
        &self.mi_dma_start
    }
    #[doc = "0x1528 - DMA status register"]
    #[inline(always)]
    pub const fn mi_dma_status(&self) -> &MiDmaStatus {
        &self.mi_dma_status
    }
    #[doc = "0x152c - Counter value for defect pixel list"]
    #[inline(always)]
    pub const fn mi_pixel_cnt(&self) -> &MiPixelCnt {
        &self.mi_pixel_cnt
    }
    #[doc = "0x1530 - Base address 2 (ping pong) for main picture Y component, \n\n\n\nJPEG or raw data\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_mp_y_base_ad_init2(&self) -> &MiMpYBaseAdInit2 {
        &self.mi_mp_y_base_ad_init2
    }
    #[doc = "0x1534 - Base address 2 (pingpong) for main picture Cb component\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_mp_cb_base_ad_init2(&self) -> &MiMpCbBaseAdInit2 {
        &self.mi_mp_cb_base_ad_init2
    }
    #[doc = "0x1538 - Base address 2 (pingpong) for main picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_mp_cr_base_ad_init2(&self) -> &MiMpCrBaseAdInit2 {
        &self.mi_mp_cr_base_ad_init2
    }
    #[doc = "0x153c - Base address 2 (ping pong) for self picture Y component\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_sp_y_base_ad_init2(&self) -> &MiSpYBaseAdInit2 {
        &self.mi_sp_y_base_ad_init2
    }
    #[doc = "0x1540 - Base address 2 (pingpong) for self picture Cb component\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mi_sp_cb_base_ad_init2(&self) -> &MiSpCbBaseAdInit2 {
        &self.mi_sp_cb_base_ad_init2
    }
    #[doc = "0x1544 - Base address 2 (pingpong) for self picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n"]
    #[inline(always)]
    pub const fn mi_sp_cr_base_ad_init2(&self) -> &MiSpCrBaseAdInit2 {
        &self.mi_sp_cr_base_ad_init2
    }
    #[doc = "0x1548 - Extended Storage Format Control for main, self and dma read path"]
    #[inline(always)]
    pub const fn mi_xtd_format_ctrl(&self) -> &MiXtdFormatCtrl {
        &self.mi_xtd_format_ctrl
    }
    #[doc = "0x1c00 - global control register"]
    #[inline(always)]
    pub const fn mipi_ctrl(&self) -> &MipiCtrl {
        &self.mipi_ctrl
    }
    #[doc = "0x1c04 - global status register"]
    #[inline(always)]
    pub const fn mipi_status(&self) -> &MipiStatus {
        &self.mipi_status
    }
    #[doc = "0x1c08 - Interrupt mask"]
    #[inline(always)]
    pub const fn mipi_imsc(&self) -> &MipiImsc {
        &self.mipi_imsc
    }
    #[doc = "0x1c0c - Raw interrupt status"]
    #[inline(always)]
    pub const fn mipi_ris(&self) -> &MipiRis {
        &self.mipi_ris
    }
    #[doc = "0x1c10 - Masked interrupt status"]
    #[inline(always)]
    pub const fn mipi_mis(&self) -> &MipiMis {
        &self.mipi_mis
    }
    #[doc = "0x1c14 - Interrupt clear register\n\nNote: clears corresponding bits in MIPI_RIS register \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mipi_icr(&self) -> &MipiIcr {
        &self.mipi_icr
    }
    #[doc = "0x1c18 - Interrupt set register\n\nNote: sets corresponding bits in MIPI_RIS register \n\n\n\n \n\n\n\n"]
    #[inline(always)]
    pub const fn mipi_isr(&self) -> &MipiIsr {
        &self.mipi_isr
    }
    #[doc = "0x1c1c - Current Data Identifier"]
    #[inline(always)]
    pub const fn mipi_cur_data_id(&self) -> &MipiCurDataId {
        &self.mipi_cur_data_id
    }
    #[doc = "0x1c20 - Image Data Selector"]
    #[inline(always)]
    pub const fn mipi_img_data_sel(&self) -> &MipiImgDataSel {
        &self.mipi_img_data_sel
    }
    #[doc = "0x1c24 - Additional Data Selector 1"]
    #[inline(always)]
    pub const fn mipi_add_data_sel_1(&self) -> &MipiAddDataSel1 {
        &self.mipi_add_data_sel_1
    }
    #[doc = "0x1c28 - Additional Data Selector 2"]
    #[inline(always)]
    pub const fn mipi_add_data_sel_2(&self) -> &MipiAddDataSel2 {
        &self.mipi_add_data_sel_2
    }
    #[doc = "0x1c2c - Additional Data Selector 3"]
    #[inline(always)]
    pub const fn mipi_add_data_sel_3(&self) -> &MipiAddDataSel3 {
        &self.mipi_add_data_sel_3
    }
    #[doc = "0x1c30 - Additional Data Selector 4"]
    #[inline(always)]
    pub const fn mipi_add_data_sel_4(&self) -> &MipiAddDataSel4 {
        &self.mipi_add_data_sel_4
    }
    #[doc = "0x1c34 - Additional Data Fifo"]
    #[inline(always)]
    pub const fn mipi_add_data_fifo(&self) -> &MipiAddDataFifo {
        &self.mipi_add_data_fifo
    }
    #[doc = "0x1c3c - controls processing of compressed raw data types\n\nNote: This register is only available in MIPI interface version 2 of ISP \n\n\n\n \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mipi_compressed_mode(&self) -> &MipiCompressedMode {
        &self.mipi_compressed_mode
    }
    #[doc = "0x1c40 - frame number from frame start and frame end short packets\n\nNote: This register is only available in MIPI interface version 2 of ISP \n\n\n\n"]
    #[inline(always)]
    pub const fn mipi_frame(&self) -> &MipiFrame {
        &self.mipi_frame
    }
    #[doc = "0x1c44 - data type flags for received generic short packets\n\nNote: This register is only available in MIPI interface version 2 of ISP \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mipi_gen_short_dt(&self) -> &MipiGenShortDt {
        &self.mipi_gen_short_dt
    }
    #[doc = "0x1c48 - data field for generic short packets of data type 0x8 and 0x9\n\nNote: This register is only available in MIPI interface version 2 of ISP \n\n\n\n"]
    #[inline(always)]
    pub const fn mipi_gen_short_8_9(&self) -> &MipiGenShort8_9 {
        &self.mipi_gen_short_8_9
    }
    #[doc = "0x1c4c - data field for generic short packets of data type 0xA and 0xB\n\nNote: This register is only available in MIPI interface version 2 of ISP \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn mipi_gen_short_a_b(&self) -> &MipiGenShortAB {
        &self.mipi_gen_short_a_b
    }
    #[doc = "0x1c50 - data field for generic short packets of data type 0xC and 0xD\n\nNote: This register is only available in MIPI interface version 2 of ISP \n\n\n\n \n\n\n\n"]
    #[inline(always)]
    pub const fn mipi_gen_short_c_d(&self) -> &MipiGenShortCD {
        &self.mipi_gen_short_c_d
    }
    #[doc = "0x1c54 - data field for generic short packets of data type 0xE and 0xF\n\nNote: This register is only available in MIPI interface version 2 of ISP \n\n\n\n \n\n\n\n \n\n\n\n \n\n\n\n \n\n\n\n \n\n\n\n \n\n \n\n\n\nThis is the control register for AF measurement unit Reset value: 0000'0000H \n\n \n\n\n\nAddress: ISP_AFM_BASE + 0000H Mode : rw \n\n\n\nBit\n\n\n\ns \n\n\n\nName Description \n\n\n\n31:\n\n\n\n1 \n\n\n\n--- unused \n\n\n\n0 afm_en AF measurement enable \n\n\n\n0: AF measurement is \n\n\n\ndisabled 1: AF \n\n\n\nmeasurement is enabled \n\n\n\nWriting a 1 to this register starts a new measurement \n\n\n\nand resets the afm_fin (measurement finished) interrupt \n\n\n\nto 0. \n\n\n\nAs long as the afm_en is 1, the AFM unit \n\n"]
    #[inline(always)]
    pub const fn mipi_gen_short_e_f(&self) -> &MipiGenShortEF {
        &self.mipi_gen_short_e_f
    }
    #[doc = "0x2004 - Top Left corner of measure window A"]
    #[inline(always)]
    pub const fn afm_lt_a(&self) -> &AfmLtA {
        &self.afm_lt_a
    }
    #[doc = "0x2008 - Bottom right corner of measure window A"]
    #[inline(always)]
    pub const fn afm_rb_a(&self) -> &AfmRbA {
        &self.afm_rb_a
    }
    #[doc = "0x200c - Top left corner of measure window B"]
    #[inline(always)]
    pub const fn afm_lt_b(&self) -> &AfmLtB {
        &self.afm_lt_b
    }
    #[doc = "0x2010 - Bottom right corner of measure window B"]
    #[inline(always)]
    pub const fn afm_rb_b(&self) -> &AfmRbB {
        &self.afm_rb_b
    }
    #[doc = "0x2014 - Top left corner of measure window C"]
    #[inline(always)]
    pub const fn afm_lt_c(&self) -> &AfmLtC {
        &self.afm_lt_c
    }
    #[doc = "0x2018 - Bottom right corner of measure window C"]
    #[inline(always)]
    pub const fn afm_rb_c(&self) -> &AfmRbC {
        &self.afm_rb_c
    }
    #[doc = "0x201c - Threshold register"]
    #[inline(always)]
    pub const fn afm_thres(&self) -> &AfmThres {
        &self.afm_thres
    }
    #[doc = "0x2020 - Variable shift register"]
    #[inline(always)]
    pub const fn afm_var_shift(&self) -> &AfmVarShift {
        &self.afm_var_shift
    }
    #[doc = "0x2024 - Sharpness Value Status Register of Window A"]
    #[inline(always)]
    pub const fn afm_sum_a(&self) -> &AfmSumA {
        &self.afm_sum_a
    }
    #[doc = "0x2028 - Sharpness Value Status Register of Window B"]
    #[inline(always)]
    pub const fn afm_sum_b(&self) -> &AfmSumB {
        &self.afm_sum_b
    }
    #[doc = "0x202c - Sharpness Value Status Register of Window C"]
    #[inline(always)]
    pub const fn afm_sum_c(&self) -> &AfmSumC {
        &self.afm_sum_c
    }
    #[doc = "0x2030 - Luminance Value Status Register of Window A"]
    #[inline(always)]
    pub const fn afm_lum_a(&self) -> &AfmLumA {
        &self.afm_lum_a
    }
    #[doc = "0x2034 - Luminance Value Status Register of Window B"]
    #[inline(always)]
    pub const fn afm_lum_b(&self) -> &AfmLumB {
        &self.afm_lum_b
    }
    #[doc = "0x2038 - Luminance Value Status Register of Window C"]
    #[inline(always)]
    pub const fn afm_lum_c(&self) -> &AfmLumC {
        &self.afm_lum_c
    }
    #[doc = "0x2200 - Lens shade control"]
    #[inline(always)]
    pub const fn lsc_ctrl(&self) -> &LscCtrl {
        &self.lsc_ctrl
    }
    #[doc = "0x2204 - Table RAM Address for red component\n\nNote: The table values are written into an internal RAM. The RAM Address is generated per \n\nauto- increment. The tables values will be read back by a continuous read access to the \n\ncorresponding register. The read address is auto-incremented for each read access to that \n\nregister and is reset to a specific value by a write access to the ISP_LSC_TABLE_ADDR \n\nregister. \n\n\n\nTable set 0 access by SW at table address 0...152. Table set 1 access at table address \n\n153...305. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn lsc_r_table_addr(&self) -> &LscRTableAddr {
        &self.lsc_r_table_addr
    }
    #[doc = "0x2208 - Table RAM Address for green (red) component\n\nNote: MKOE tbc: Orignial register mode was rwh which is no longer supported with new \n\nversion of SIG-> rwhh Table set 0 access by SW at table address 0...153. Table set 1 access at \n\ntable address 154...307. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn lsc_gr_table_addr(&self) -> &LscGrTableAddr {
        &self.lsc_gr_table_addr
    }
    #[doc = "0x220c - Table RAM Address for blue component\n\nNote: MKOE tbc: Orignial register mode was rwh which is no longer supported with new \n\nversion of SIG-> rwhh Table set 0 access by SW at table address 0...153. Table set 1 access at \n\ntable address 154...307. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn lsc_b_table_addr(&self) -> &LscBTableAddr {
        &self.lsc_b_table_addr
    }
    #[doc = "0x2210 - Table RAM Address for green (blue) component\n\nNote: MKOE tbc: Orignial register mode was rwh which is no longer supported with new \n\nversion of SIG-> rwhh Table set 0 access by SW at table address 0...153. Table set 1 access at \n\ntable address 154...307. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn lsc_gb_table_addr(&self) -> &LscGbTableAddr {
        &self.lsc_gb_table_addr
    }
    #[doc = "0x2214 - Sample table red\n\nNote: The programmed sample value is immediately written into the RAM. The RAM \n\naddress is generated per auto-increment. The parameter RAMs for Lens Shade Correction and \n\nBad Pixel Correction can only be programmed, if the RGB Bayer path is switched on via \n\nISP_CTRL register (ISP_MODE bits). \n\n\n\nTable set 0 access by SW at table address 0...153. Table set 1 access at table address \n\n\n\n154...307."]
    #[inline(always)]
    pub const fn lsc_r_table_data(&self) -> &LscRTableData {
        &self.lsc_r_table_data
    }
    #[doc = "0x2218 - Sample table green (red)\n\nNote: The programmed sample value is immediately written into the RAM. The RAM \n\naddress is generated per auto-increment. The parameter RAMs for Lens Shade Correction and \n\nBad Pixel Correction can only be programmed, if the RGB Bayer path is switched on via \n\nISP_CTRL register (ISP_MODE bits). \n\n\n\nTable set 0 access by SW at table address 0...153. Table set 1 access at table address \n\n\n\n154...307. \n\n\n\n"]
    #[inline(always)]
    pub const fn lsc_gr_table_data(&self) -> &LscGrTableData {
        &self.lsc_gr_table_data
    }
    #[doc = "0x221c - Sample table blue\n\nNote: The programmed sample value is immediately written into the RAM. The \n\n\n\nRAM address is generated per auto-increment. The parameter RAMs for Lens Shade \n\n\n\nCorrection and Bad Pixel Correction can only be programmed, if the RGB Bayer path is \n\n\n\nswitched on via ISP_CTRL register (ISP_MODE bits). \n\n\n\nTable set 0 access by SW at table address 0...153. Table set 1 access at table address \n\n\n\n154...307. \n\n\n\n"]
    #[inline(always)]
    pub const fn lsc_b_table_data(&self) -> &LscBTableData {
        &self.lsc_b_table_data
    }
    #[doc = "0x2220 - Sample table green (blue)\n\nNote: The programmed sample value is immediately written into the RAM. The RAM \n\naddress is generated per auto-increment. The parameter RAMs for Lens Shade Correction and \n\nBad Pixel Correction can only be programmed, if the RGB Bayer path is switched on via \n\nISP_CTRL register (ISP_MODE bits).Table set 0 access by SW at table address 0...153. Table \n\nset 1 access at table address 154...307. \n\n\n\n \n\n\n\n \n\n\n\n"]
    #[inline(always)]
    pub const fn lsc_gb_table_data(&self) -> &LscGbTableData {
        &self.lsc_gb_table_data
    }
    #[doc = "0x2224 - Gradient table x"]
    #[inline(always)]
    pub const fn lsc_xgrad_01(&self) -> &LscXgrad01 {
        &self.lsc_xgrad_01
    }
    #[doc = "0x2228 - Gradient table x"]
    #[inline(always)]
    pub const fn lsc_xgrad_23(&self) -> &LscXgrad23 {
        &self.lsc_xgrad_23
    }
    #[doc = "0x222c - Gradient table x"]
    #[inline(always)]
    pub const fn lsc_xgrad_45(&self) -> &LscXgrad45 {
        &self.lsc_xgrad_45
    }
    #[doc = "0x2230 - Gradient table x"]
    #[inline(always)]
    pub const fn lsc_xgrad_67(&self) -> &LscXgrad67 {
        &self.lsc_xgrad_67
    }
    #[doc = "0x2234 - Gradient table y"]
    #[inline(always)]
    pub const fn lsc_ygrad_01(&self) -> &LscYgrad01 {
        &self.lsc_ygrad_01
    }
    #[doc = "0x2238 - Gradient table y"]
    #[inline(always)]
    pub const fn lsc_ygrad_23(&self) -> &LscYgrad23 {
        &self.lsc_ygrad_23
    }
    #[doc = "0x223c - Gradient table y"]
    #[inline(always)]
    pub const fn lsc_ygrad_45(&self) -> &LscYgrad45 {
        &self.lsc_ygrad_45
    }
    #[doc = "0x2240 - Gradient table y"]
    #[inline(always)]
    pub const fn lsc_ygrad_67(&self) -> &LscYgrad67 {
        &self.lsc_ygrad_67
    }
    #[doc = "0x2244 - Size table\n\nNote: The sector size in x-direction must be greater than 12 pixels. The sum of the sector \n\nsizes in x- direction must be 'picture width / 2'. The sum of the sector sizes in y-direction must \n\nbe 'picture height / 2'. No interrupt is generated if above requirements are not fulfilled and the \n\nbehaviour of the hardware cannot be predicted. \n\n\n\nThe sector size in x-direction was defined to be 9 bits for preliminary ISP versions. \n\n\n\n"]
    #[inline(always)]
    pub const fn lsc_xsize_01(&self) -> &LscXsize01 {
        &self.lsc_xsize_01
    }
    #[doc = "0x2248 - Size table\n\nNote: minimum sector size is 10 in x direction \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn lsc_xsize_23(&self) -> &LscXsize23 {
        &self.lsc_xsize_23
    }
    #[doc = "0x224c - Size table\n\nNote: minimum sector size is 10 in x direction \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn lsc_xsize_45(&self) -> &LscXsize45 {
        &self.lsc_xsize_45
    }
    #[doc = "0x2250 - Size table\n\nNote: minimum sector size is 10 in x direction \n\n\n\n"]
    #[inline(always)]
    pub const fn lsc_xsize_67(&self) -> &LscXsize67 {
        &self.lsc_xsize_67
    }
    #[doc = "0x2254 - Size table\n\nNote: minimum sector size is 8 in y direction. \n\n\n\nThe sector size in y-direction was defined to be 9 bits for preliminary ISP versions. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn lsc_ysize_01(&self) -> &LscYsize01 {
        &self.lsc_ysize_01
    }
    #[doc = "0x2258 - Size table\n\nNote: minimum sector size is 8 in y direction \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn lsc_ysize_23(&self) -> &LscYsize23 {
        &self.lsc_ysize_23
    }
    #[doc = "0x225c - Size table\n\nNote: minimum sector size is 8 in y direction \n\n\n\n"]
    #[inline(always)]
    pub const fn lsc_ysize_45(&self) -> &LscYsize45 {
        &self.lsc_ysize_45
    }
    #[doc = "0x2260 - Size table\n\nNote: minimum sector size is 8 in y direction \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn lsc_ysize_67(&self) -> &LscYsize67 {
        &self.lsc_ysize_67
    }
    #[doc = "0x2264 - Lens shade table set selection\n\nNote: Table set 0 access by SW at table address 0...153. Table set 1 access at table \n\naddress 154...307. For LSC4_MSZ the table set 1 is physically not available: \n\nISP_LSC_TABLE_SEL shall always be 0 for this HW configuration. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn lsc_table_sel(&self) -> &LscTableSel {
        &self.lsc_table_sel
    }
    #[doc = "0x2268 - Lens shade status"]
    #[inline(always)]
    pub const fn lsc_status(&self) -> &LscStatus {
        &self.lsc_status
    }
    #[doc = "0x2300 - Image Stabilization Control Register"]
    #[inline(always)]
    pub const fn is_ctrl(&self) -> &IsCtrl {
        &self.is_ctrl
    }
    #[doc = "0x2304 - Recenter register"]
    #[inline(always)]
    pub const fn is_recenter(&self) -> &IsRecenter {
        &self.is_recenter
    }
    #[doc = "0x2308 - Horizontal offset of output window"]
    #[inline(always)]
    pub const fn is_h_offs(&self) -> &IsHOffs {
        &self.is_h_offs
    }
    #[doc = "0x230c - Vertical offset of output window"]
    #[inline(always)]
    pub const fn is_v_offs(&self) -> &IsVOffs {
        &self.is_v_offs
    }
    #[doc = "0x2310 - Output horizontal picture size"]
    #[inline(always)]
    pub const fn is_h_size(&self) -> &IsHSize {
        &self.is_h_size
    }
    #[doc = "0x2314 - Output vertical picture size"]
    #[inline(always)]
    pub const fn is_v_size(&self) -> &IsVSize {
        &self.is_v_size
    }
    #[doc = "0x2318 - Maximum Horizontal Displacement"]
    #[inline(always)]
    pub const fn is_max_dx(&self) -> &IsMaxDx {
        &self.is_max_dx
    }
    #[doc = "0x231c - Maximum Vertical Displacement"]
    #[inline(always)]
    pub const fn is_max_dy(&self) -> &IsMaxDy {
        &self.is_max_dy
    }
    #[doc = "0x2320 - Camera displacement"]
    #[inline(always)]
    pub const fn is_displace(&self) -> &IsDisplace {
        &self.is_displace
    }
    #[doc = "0x2324 - current horizontal offset of output window (shadow register)"]
    #[inline(always)]
    pub const fn is_h_offs_shd(&self) -> &IsHOffsShd {
        &self.is_h_offs_shd
    }
    #[doc = "0x2328 - current vertical offset of output window (shadow register)"]
    #[inline(always)]
    pub const fn is_v_offs_shd(&self) -> &IsVOffsShd {
        &self.is_v_offs_shd
    }
    #[doc = "0x232c - current output horizontal picture size (shadow register)"]
    #[inline(always)]
    pub const fn is_h_size_shd(&self) -> &IsHSizeShd {
        &self.is_h_size_shd
    }
    #[doc = "0x2330 - current output vertical picture size (shadow register)"]
    #[inline(always)]
    pub const fn is_v_size_shd(&self) -> &IsVSizeShd {
        &self.is_v_size_shd
    }
    #[doc = "0x2400 - Histogram properties\n\nNote: If RGB combined mode is used, then the 3 color components are sampled one \n\n\n\nafter the other. The software has to assure that all 3 color components are inside the \n\n\n\nselected window. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn hist_prop(&self) -> &HistProp {
        &self.hist_prop
    }
    #[doc = "0x2404 - Histogram window horizontal offset for first window of \n\n\n\n25 sub- windows\n\nNote: histogram measurement is done in 25 sub-windows like the exposure \n\n\n\nmeasurement, if histogram version 3 is implemented. All earlier versions use just one \n\n\n\nwindow. \n\n \n\n"]
    #[inline(always)]
    pub const fn hist_h_offs(&self) -> &HistHOffs {
        &self.hist_h_offs
    }
    #[doc = "0x2408 - Histogram window vertical offset for first window of 25 sub-windows\n\nNote: histogram measurement is done in 25 sub-windows like the exposure \n\nmeasurement, if histogram version 3 is implemented. All earlier versions use just one window. \n\n\n\n \n\n\n\n"]
    #[inline(always)]
    pub const fn hist_v_offs(&self) -> &HistVOffs {
        &self.hist_v_offs
    }
    #[doc = "0x240c - Horizontal (sub-)window size\n\nNote: hist_h_offset + hist_h_size x 5 should be less than or equal to the horizontal size \n\n\n\nof the picture, if histogram version 3 is implemented. Otherwise hist_h_size is the horizontal \n\n\n\nsize of the measurement window in pixels. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn hist_h_size(&self) -> &HistHSize {
        &self.hist_h_size
    }
    #[doc = "0x2410 - Vertical (sub-)window size\n\nNote: hist_v_offset + hist_v_size x 5 should be less than or equal to the vertical size \n\n\n\nof the picture, if histogram version 3 is implemented. Otherwise hist_v_size is the vertical \n\n\n\nsize of the measurement window in lines. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn hist_v_size(&self) -> &HistVSize {
        &self.hist_v_size
    }
    #[doc = "0x2414..0x2454 - histogram measurement result bin n (n=0..15)\n\nNote: MKOE tbc: Orignial register mode was rh which is no longer supported with new \n\nversion of SIG -> r \n\n\n\n \n\n\n\n"]
    #[inline(always)]
    pub const fn hist_bin(&self, n: usize) -> &HistBin {
        &self.hist_bin[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2414..0x2454 - histogram measurement result bin n (n=0..15)\n\nNote: MKOE tbc: Orignial register mode was rh which is no longer supported with new \n\nversion of SIG -> r \n\n\n\n \n\n\n\n"]
    #[inline(always)]
    pub fn hist_bin_iter(&self) -> impl Iterator<Item = &HistBin> {
        self.hist_bin.iter()
    }
    #[doc = "0x2454 - Weighting factor for sub-windows\n\nNote: Allowed value range for weight factor is 0 to 16. The resulting weight is \n\nregister_value / 16. The host software has to limit the register value for each factor to 16. \n\nWeight registers are available, if histogram version 3 is implemented. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn hist_weight_00to30(&self) -> &HistWeight00to30 {
        &self.hist_weight_00to30
    }
    #[doc = "0x2458 - Weighting factor for sub-windows\n\nNote: Allowed value range for weight factor is 0 to 16. The resulting weight is \n\n\n\nregister_value / 16. The host software has to limit the register value for each factor to 16. \n\n\n\nWeight registers are available, if histogram version 3 is implemented. \n\n\n\n \n\n\n\n"]
    #[inline(always)]
    pub const fn hist_weight_40to21(&self) -> &HistWeight40to21 {
        &self.hist_weight_40to21
    }
    #[doc = "0x245c - Weighting factor for sub-windows\n\nNote: Allowed value range for weight factor is 0 to 16. The resulting weight is \n\nregister_value / 16. The host software has to limit the register value for each factor to 16. \n\nWeight registers are available, if histogram version 3 is implemented. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn hist_weight_31to12(&self) -> &HistWeight31to12 {
        &self.hist_weight_31to12
    }
    #[doc = "0x2460 - Weighting factor for sub-windows\n\nNote: Allowed value range for weight factor is 0 to 16. The resulting weight is \n\nregister_value / 16. The host software has to limit the register value for each factor to 16. \n\nWeight registers are available, if histogram version 3 is implemented. \n\n\n\n \n\n\n\n"]
    #[inline(always)]
    pub const fn hist_weight_22to03(&self) -> &HistWeight22to03 {
        &self.hist_weight_22to03
    }
    #[doc = "0x2464 - Weighting factor for sub-windows\n\nNote: Allowed value range for weight factor is 0 to 16. The resulting weight is \n\nregister_value / 16. The host software has to limit the register value for each factor to 16. \n\nWeight registers are available, if histogram version 3 is implemented. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn hist_weight_13to43(&self) -> &HistWeight13to43 {
        &self.hist_weight_13to43
    }
    #[doc = "0x2468 - Weighting factor for sub-windows\n\nNote: Allowed value range for weight factor is 0 to 16. The resulting weight is \n\nregister_value / 16. The host software has to limit the register value for each factor to 16. \n\nWeight registers are available, if histogram version 3 is implemented. \n\n\n\n"]
    #[inline(always)]
    pub const fn hist_weight_04to34(&self) -> &HistWeight04to34 {
        &self.hist_weight_04to34
    }
    #[doc = "0x246c - Weighting factor for sub-windows\n\nNote: Allowed value range for weight factor is 0 to 16. The resulting weight is \n\nregister_value / 16. The host software has to limit the register value for each factor to 16. \n\nWeight registers are available, if histogram version 3 is implemented. \n\n\n\n"]
    #[inline(always)]
    pub const fn hist_weight_44(&self) -> &HistWeight44 {
        &self.hist_weight_44
    }
    #[doc = "0x2500 - mode control register for the filter block"]
    #[inline(always)]
    pub const fn filt_mode(&self) -> &FiltMode {
        &self.filt_mode
    }
    #[doc = "0x2528 - Blurring threshold 0\n\nNote: sum_grad is calculated by the texture detection unit as the sum of \n\n\n\nhorizontal and vertical gradients \n\n\n\n"]
    #[inline(always)]
    pub const fn filt_thresh_bl0(&self) -> &FiltThreshBl0 {
        &self.filt_thresh_bl0
    }
    #[doc = "0x252c - Blurring threshold 1\n\nNote: sum_grad is calculated by the texture detection unit as the sum of horizontal and \n\nvertical gradients \n\n\n\n"]
    #[inline(always)]
    pub const fn filt_thresh_bl1(&self) -> &FiltThreshBl1 {
        &self.filt_thresh_bl1
    }
    #[doc = "0x2530 - Sharpening threshold 0\n\nNote: sum_grad is calculated by the texture detection unit as the sum of horizontal and \n\nvertical gradients \n\n\n\n \n\n\n\n"]
    #[inline(always)]
    pub const fn filt_thresh_sh0(&self) -> &FiltThreshSh0 {
        &self.filt_thresh_sh0
    }
    #[doc = "0x2534 - Sharpening threshold 1\n\nNote: sum_grad is calculated by the texture detection unit as the sum of horizontal and \n\n\n\nvertical gradients \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn filt_thresh_sh1(&self) -> &FiltThreshSh1 {
        &self.filt_thresh_sh1
    }
    #[doc = "0x2538 - Parameters for luminance weight function"]
    #[inline(always)]
    pub const fn filt_lum_weight(&self) -> &FiltLumWeight {
        &self.filt_lum_weight
    }
    #[doc = "0x253c - filter factor sharp1"]
    #[inline(always)]
    pub const fn filt_fac_sh1(&self) -> &FiltFacSh1 {
        &self.filt_fac_sh1
    }
    #[doc = "0x2540 - filter factor sharp0"]
    #[inline(always)]
    pub const fn filt_fac_sh0(&self) -> &FiltFacSh0 {
        &self.filt_fac_sh0
    }
    #[doc = "0x2544 - filter factor middle"]
    #[inline(always)]
    pub const fn filt_fac_mid(&self) -> &FiltFacMid {
        &self.filt_fac_mid
    }
    #[doc = "0x2548 - Parameter for blur 0 filter"]
    #[inline(always)]
    pub const fn filt_fac_bl0(&self) -> &FiltFacBl0 {
        &self.filt_fac_bl0
    }
    #[doc = "0x254c - Parameter for blur 1 filter"]
    #[inline(always)]
    pub const fn filt_fac_bl1(&self) -> &FiltFacBl1 {
        &self.filt_fac_bl1
    }
    #[doc = "0x2580 - Control register for chromatic aberration correction\n\nNote: Clipping behavior can be controlled by clip_mode bits. If no clipping occurs, because \n\ndisplacement is below the maximum correctable displacement, then it does not matter which \n\nmode is selected. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn cac_ctrl(&self) -> &CacCtrl {
        &self.cac_ctrl
    }
    #[doc = "0x2584 - Preload values for CAC pixel and line counter\n\nNote: Reset value is valid for 8192 x 8192 image resolution with centered chromatic \n\naberration (no offset from image center). \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn cac_count_start(&self) -> &CacCountStart {
        &self.cac_count_start
    }
    #[doc = "0x2588 - Linear Parameters for radial shift calculation"]
    #[inline(always)]
    pub const fn cac_a(&self) -> &CacA {
        &self.cac_a
    }
    #[doc = "0x258c - Square Parameters for radial shift calculation"]
    #[inline(always)]
    pub const fn cac_b(&self) -> &CacB {
        &self.cac_b
    }
    #[doc = "0x2590 - Cubical Parameters for radial shift calculation"]
    #[inline(always)]
    pub const fn cac_c(&self) -> &CacC {
        &self.cac_c
    }
    #[doc = "0x2594 - Normalization parameters for calculation of image \n\n\n\ncoordinate x_d relative to optical center\n\nNote: These values need to be programmed according to the image resolution and the \n\ncenter offset of the chromatic aberration. \n\n\n\nThe parameters are necessary to avoid high gate count of the CAC hardware block. The \n\nreset value is valid for an image resolution of 2600 x 1950 and center offset 0. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn cac_x_norm(&self) -> &CacXNorm {
        &self.cac_x_norm
    }
    #[doc = "0x2598 - Normalization parameters for calculation of image \n\n\n\ncoordinate y_d relative to optical center\n\nNote: These values need to be programmed according to the image resolution and the \n\ncenter offset of the chromatic aberration. \n\n\n\nThe parameters are necessary to avoid high gate count of the CAC hardware block. The \n\n\n\nreset value is valid for an image resolution of 2600 x 1950 and center offset 0. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn cac_y_norm(&self) -> &CacYNorm {
        &self.cac_y_norm
    }
    #[doc = "0x2600 - Exposure control"]
    #[inline(always)]
    pub const fn exp_ctrl(&self) -> &ExpCtrl {
        &self.exp_ctrl
    }
    #[doc = "0x2604 - Horizontal offset for first block"]
    #[inline(always)]
    pub const fn exp_h_offset(&self) -> &ExpHOffset {
        &self.exp_h_offset
    }
    #[doc = "0x2608 - Vertical offset for first block"]
    #[inline(always)]
    pub const fn exp_v_offset(&self) -> &ExpVOffset {
        &self.exp_v_offset
    }
    #[doc = "0x260c - Horizontal size of one block\n\nNote: exp_h_size x 5 must be less (not equal) than the horizontal size of the picture \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn exp_h_size(&self) -> &ExpHSize {
        &self.exp_h_size
    }
    #[doc = "0x2610 - Vertical size of one block\n\nNote: The vertical size must be set in a way that after the last measurement window at \n\n\n\nleast two lines of the image will follow. In addition only even values for vertical size are \n\n\n\nallowed (vertical size must be a multiple of 2). \n\n\n\n"]
    #[inline(always)]
    pub const fn exp_v_size(&self) -> &ExpVSize {
        &self.exp_v_size
    }
    #[doc = "0x2614 - Mean luminance value of block 00"]
    #[inline(always)]
    pub const fn exp_mean_00(&self) -> &ExpMean00 {
        &self.exp_mean_00
    }
    #[doc = "0x2618 - Mean luminance value of block 10"]
    #[inline(always)]
    pub const fn exp_mean_10(&self) -> &ExpMean10 {
        &self.exp_mean_10
    }
    #[doc = "0x261c - Mean luminance value of block 20"]
    #[inline(always)]
    pub const fn exp_mean_20(&self) -> &ExpMean20 {
        &self.exp_mean_20
    }
    #[doc = "0x2620 - Mean luminance value of block 30"]
    #[inline(always)]
    pub const fn exp_mean_30(&self) -> &ExpMean30 {
        &self.exp_mean_30
    }
    #[doc = "0x2624 - Mean luminance value of block 40"]
    #[inline(always)]
    pub const fn exp_mean_40(&self) -> &ExpMean40 {
        &self.exp_mean_40
    }
    #[doc = "0x2628 - Mean luminance value of block 01"]
    #[inline(always)]
    pub const fn exp_mean_01(&self) -> &ExpMean01 {
        &self.exp_mean_01
    }
    #[doc = "0x262c - Mean luminance value of block 11"]
    #[inline(always)]
    pub const fn exp_mean_11(&self) -> &ExpMean11 {
        &self.exp_mean_11
    }
    #[doc = "0x2630 - Mean luminance value of block 21"]
    #[inline(always)]
    pub const fn exp_mean_21(&self) -> &ExpMean21 {
        &self.exp_mean_21
    }
    #[doc = "0x2634 - Mean luminance value of block 31"]
    #[inline(always)]
    pub const fn exp_mean_31(&self) -> &ExpMean31 {
        &self.exp_mean_31
    }
    #[doc = "0x2638 - Mean luminance value of block 41"]
    #[inline(always)]
    pub const fn exp_mean_41(&self) -> &ExpMean41 {
        &self.exp_mean_41
    }
    #[doc = "0x263c - Mean luminance value of block 02"]
    #[inline(always)]
    pub const fn exp_mean_02(&self) -> &ExpMean02 {
        &self.exp_mean_02
    }
    #[doc = "0x2640 - Mean luminance value of block 12"]
    #[inline(always)]
    pub const fn exp_mean_12(&self) -> &ExpMean12 {
        &self.exp_mean_12
    }
    #[doc = "0x2644 - Mean luminance value of block 22"]
    #[inline(always)]
    pub const fn exp_mean_22(&self) -> &ExpMean22 {
        &self.exp_mean_22
    }
    #[doc = "0x2648 - Mean luminance value of block 32"]
    #[inline(always)]
    pub const fn exp_mean_32(&self) -> &ExpMean32 {
        &self.exp_mean_32
    }
    #[doc = "0x264c - Mean luminance value of block 42"]
    #[inline(always)]
    pub const fn exp_mean_42(&self) -> &ExpMean42 {
        &self.exp_mean_42
    }
    #[doc = "0x2650 - Mean luminance value of block 03"]
    #[inline(always)]
    pub const fn exp_mean_03(&self) -> &ExpMean03 {
        &self.exp_mean_03
    }
    #[doc = "0x2654 - Mean luminance value of block 13"]
    #[inline(always)]
    pub const fn exp_mean_13(&self) -> &ExpMean13 {
        &self.exp_mean_13
    }
    #[doc = "0x2658 - Mean luminance value of block 23"]
    #[inline(always)]
    pub const fn exp_mean_23(&self) -> &ExpMean23 {
        &self.exp_mean_23
    }
    #[doc = "0x265c - Mean luminance value of block 33"]
    #[inline(always)]
    pub const fn exp_mean_33(&self) -> &ExpMean33 {
        &self.exp_mean_33
    }
    #[doc = "0x2660 - Mean luminance value of block 43"]
    #[inline(always)]
    pub const fn exp_mean_43(&self) -> &ExpMean43 {
        &self.exp_mean_43
    }
    #[doc = "0x2664 - Mean luminance value of block 04"]
    #[inline(always)]
    pub const fn exp_mean_04(&self) -> &ExpMean04 {
        &self.exp_mean_04
    }
    #[doc = "0x2668 - Mean luminance value of block 14"]
    #[inline(always)]
    pub const fn exp_mean_14(&self) -> &ExpMean14 {
        &self.exp_mean_14
    }
    #[doc = "0x266c - Mean luminance value of block 24"]
    #[inline(always)]
    pub const fn exp_mean_24(&self) -> &ExpMean24 {
        &self.exp_mean_24
    }
    #[doc = "0x2670 - Mean luminance value of block 34"]
    #[inline(always)]
    pub const fn exp_mean_34(&self) -> &ExpMean34 {
        &self.exp_mean_34
    }
    #[doc = "0x2674 - Mean luminance value of block 44"]
    #[inline(always)]
    pub const fn exp_mean_44(&self) -> &ExpMean44 {
        &self.exp_mean_44
    }
    #[doc = "0x2700 - global control register"]
    #[inline(always)]
    pub const fn bls_ctrl(&self) -> &BlsCtrl {
        &self.bls_ctrl
    }
    #[doc = "0x2704 - samples register"]
    #[inline(always)]
    pub const fn bls_samples(&self) -> &BlsSamples {
        &self.bls_samples
    }
    #[doc = "0x2708 - window 1 horizontal start"]
    #[inline(always)]
    pub const fn bls_h1_start(&self) -> &BlsH1Start {
        &self.bls_h1_start
    }
    #[doc = "0x270c - window 1 horizontal stop"]
    #[inline(always)]
    pub const fn bls_h1_stop(&self) -> &BlsH1Stop {
        &self.bls_h1_stop
    }
    #[doc = "0x2710 - window 1 vertical start"]
    #[inline(always)]
    pub const fn bls_v1_start(&self) -> &BlsV1Start {
        &self.bls_v1_start
    }
    #[doc = "0x2714 - window 1 vertical stop"]
    #[inline(always)]
    pub const fn bls_v1_stop(&self) -> &BlsV1Stop {
        &self.bls_v1_stop
    }
    #[doc = "0x2718 - window 2 horizontal start"]
    #[inline(always)]
    pub const fn bls_h2_start(&self) -> &BlsH2Start {
        &self.bls_h2_start
    }
    #[doc = "0x271c - window 2 horizontal stop"]
    #[inline(always)]
    pub const fn bls_h2_stop(&self) -> &BlsH2Stop {
        &self.bls_h2_stop
    }
    #[doc = "0x2720 - window 2 vertical start"]
    #[inline(always)]
    pub const fn bls_v2_start(&self) -> &BlsV2Start {
        &self.bls_v2_start
    }
    #[doc = "0x2724 - window 2 vertical stop"]
    #[inline(always)]
    pub const fn bls_v2_stop(&self) -> &BlsV2Stop {
        &self.bls_v2_stop
    }
    #[doc = "0x2728 - fixed black level A"]
    #[inline(always)]
    pub const fn bls_a_fixed(&self) -> &BlsAFixed {
        &self.bls_a_fixed
    }
    #[doc = "0x272c - fixed black level B"]
    #[inline(always)]
    pub const fn bls_b_fixed(&self) -> &BlsBFixed {
        &self.bls_b_fixed
    }
    #[doc = "0x2730 - fixed black level C"]
    #[inline(always)]
    pub const fn bls_c_fixed(&self) -> &BlsCFixed {
        &self.bls_c_fixed
    }
    #[doc = "0x2734 - fixed black level D"]
    #[inline(always)]
    pub const fn bls_d_fixed(&self) -> &BlsDFixed {
        &self.bls_d_fixed
    }
    #[doc = "0x2738 - measured black level A"]
    #[inline(always)]
    pub const fn bls_a_measured(&self) -> &BlsAMeasured {
        &self.bls_a_measured
    }
    #[doc = "0x273c - measured black level B"]
    #[inline(always)]
    pub const fn bls_b_measured(&self) -> &BlsBMeasured {
        &self.bls_b_measured
    }
    #[doc = "0x2740 - measured black level C"]
    #[inline(always)]
    pub const fn bls_c_measured(&self) -> &BlsCMeasured {
        &self.bls_c_measured
    }
    #[doc = "0x2744 - measured black level D"]
    #[inline(always)]
    pub const fn bls_d_measured(&self) -> &BlsDMeasured {
        &self.bls_d_measured
    }
    #[doc = "0x2800 - Mode control for Denoising Pre-Filter block"]
    #[inline(always)]
    pub const fn dpf_mode(&self) -> &DpfMode {
        &self.dpf_mode
    }
    #[doc = "0x2804 - filter strength of the RED filter"]
    #[inline(always)]
    pub const fn dpf_strength_r(&self) -> &DpfStrengthR {
        &self.dpf_strength_r
    }
    #[doc = "0x2808 - filter strength of the GREEN filter"]
    #[inline(always)]
    pub const fn dpf_strength_g(&self) -> &DpfStrengthG {
        &self.dpf_strength_g
    }
    #[doc = "0x280c - filter strength of the BLUE filter"]
    #[inline(always)]
    pub const fn dpf_strength_b(&self) -> &DpfStrengthB {
        &self.dpf_strength_b
    }
    #[doc = "0x2810 - Spatial Weights green channel 1 2 3 4\n\nNote: The value zero (0/16) disables the filter tap \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn dpf_s_weight_g_1_4(&self) -> &DpfSWeightG1_4 {
        &self.dpf_s_weight_g_1_4
    }
    #[doc = "0x2814 - Spatial Weights green channel 5 6\n\nNote: The value zero (0/16) disables the filter tap. \n\n\n\n"]
    #[inline(always)]
    pub const fn dpf_s_weight_g_5_6(&self) -> &DpfSWeightG5_6 {
        &self.dpf_s_weight_g_5_6
    }
    #[doc = "0x2818 - Spatial Weights red/blue channels 1 2 3 4\n\nNote: The value zero (0/16) disables the filter tap. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn dpf_s_weight_rb_1_4(&self) -> &DpfSWeightRb1_4 {
        &self.dpf_s_weight_rb_1_4
    }
    #[doc = "0x281c - Spatial Weights red/blue channels 5 6\n\nNote: The value zero (0/16) disables the filter tap. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn dpf_s_weight_rb_5_6(&self) -> &DpfSWeightRb5_6 {
        &self.dpf_s_weight_rb_5_6
    }
    #[doc = "0x2820..0x2864 - Noise Level Lookup Coefficient n (n=0..16)"]
    #[inline(always)]
    pub const fn dpf_nll_coeff(&self, n: usize) -> &DpfNllCoeff {
        &self.dpf_nll_coeff[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2820..0x2864 - Noise Level Lookup Coefficient n (n=0..16)"]
    #[inline(always)]
    pub fn dpf_nll_coeff_iter(&self) -> impl Iterator<Item = &DpfNllCoeff> {
        self.dpf_nll_coeff.iter()
    }
    #[doc = "0x2864 - noise function gain for red pixels"]
    #[inline(always)]
    pub const fn dpf_nf_gain_r(&self) -> &DpfNfGainR {
        &self.dpf_nf_gain_r
    }
    #[doc = "0x2868 - noise function gain for green in red pixels"]
    #[inline(always)]
    pub const fn dpf_nf_gain_gr(&self) -> &DpfNfGainGr {
        &self.dpf_nf_gain_gr
    }
    #[doc = "0x286c - noise function gain for green in blue pixels"]
    #[inline(always)]
    pub const fn dpf_nf_gain_gb(&self) -> &DpfNfGainGb {
        &self.dpf_nf_gain_gb
    }
    #[doc = "0x2870 - noise function gain for blue pixels"]
    #[inline(always)]
    pub const fn dpf_nf_gain_b(&self) -> &DpfNfGainB {
        &self.dpf_nf_gain_b
    }
    #[doc = "0x2900 - Mode control for DPCC detection unit"]
    #[inline(always)]
    pub const fn dpcc_mode(&self) -> &DpccMode {
        &self.dpcc_mode
    }
    #[doc = "0x2904 - Interpolation mode for correction unit"]
    #[inline(always)]
    pub const fn dpcc_output_mode(&self) -> &DpccOutputMode {
        &self.dpcc_output_mode
    }
    #[doc = "0x2908 - DPCC methods set usage for detection\n\nNote: methods sets can be used in parallel for each stage and the result is the logical OR \n\n\n\nof all selected sets \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn dpcc_set_use(&self) -> &DpccSetUse {
        &self.dpcc_set_use
    }
    #[doc = "0x290c - Methods enable bits for SET_1\n\nNote: different methods can be used in parallel, the result is the logical AND of all selected \n\n\n\nmethods \n\n \n\n"]
    #[inline(always)]
    pub const fn dpcc_methods_set_1(&self) -> &DpccMethodsSet1 {
        &self.dpcc_methods_set_1
    }
    #[doc = "0x2910 - Methods enable bits for SET_2\n\nNote: different methods can be used in parallel, the result is the logical AND of all selected \n\n\n\nmethods \n\n \n\n"]
    #[inline(always)]
    pub const fn dpcc_methods_set_2(&self) -> &DpccMethodsSet2 {
        &self.dpcc_methods_set_2
    }
    #[doc = "0x2914 - Methods enable bits for SET_3\n\nNote: different methods can be used in parallel, the result is the logical AND of all selected \n\n\n\nmethods \n\n \n\n"]
    #[inline(always)]
    pub const fn dpcc_methods_set_3(&self) -> &DpccMethodsSet3 {
        &self.dpcc_methods_set_3
    }
    #[doc = "0x2918 - Line threshold SET_1\n\nNote: all values are unsigned integer \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn dpcc_line_thresh_1(&self) -> &DpccLineThresh1 {
        &self.dpcc_line_thresh_1
    }
    #[doc = "0x291c - Mean Absolute Difference (MAD) factor for Line check set 1\n\nNote: all values are unsigned integer \n\n\n\n"]
    #[inline(always)]
    pub const fn dpcc_line_mad_fac_1(&self) -> &DpccLineMadFac1 {
        &self.dpcc_line_mad_fac_1
    }
    #[doc = "0x2920 - Peak gradient factor for set 1\n\nNote: all values are unsigned integer \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn dpcc_pg_fac_1(&self) -> &DpccPgFac1 {
        &self.dpcc_pg_fac_1
    }
    #[doc = "0x2924 - Rank Neighbor Difference threshold for set 1\n\nNote: all values are unsigned integer \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn dpcc_rnd_thresh_1(&self) -> &DpccRndThresh1 {
        &self.dpcc_rnd_thresh_1
    }
    #[doc = "0x2928 - Rank gradient factor for set 1\n\nNote: all values are unsigned integer \n\n\n\n"]
    #[inline(always)]
    pub const fn dpcc_rg_fac_1(&self) -> &DpccRgFac1 {
        &self.dpcc_rg_fac_1
    }
    #[doc = "0x292c - Line threshold set 2\n\nNote: all values are unsigned integer \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn dpcc_line_thresh_2(&self) -> &DpccLineThresh2 {
        &self.dpcc_line_thresh_2
    }
    #[doc = "0x2930 - Mean Absolute Difference (MAD) factor for Line check set 2\n\nNote: all values are unsigned integer \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn dpcc_line_mad_fac_2(&self) -> &DpccLineMadFac2 {
        &self.dpcc_line_mad_fac_2
    }
    #[doc = "0x2934 - Peak gradient factor for set 2\n\nNote: all values are unsigned integer \n\n\n\n"]
    #[inline(always)]
    pub const fn dpcc_pg_fac_2(&self) -> &DpccPgFac2 {
        &self.dpcc_pg_fac_2
    }
    #[doc = "0x2938 - Rank Neighbor Difference threshold for set 2\n\nNote: all values are unsigned integer \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn dpcc_rnd_thresh_2(&self) -> &DpccRndThresh2 {
        &self.dpcc_rnd_thresh_2
    }
    #[doc = "0x293c - Rank gradient factor for set 2\n\nNote: all values are unsigned integer \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn dpcc_rg_fac_2(&self) -> &DpccRgFac2 {
        &self.dpcc_rg_fac_2
    }
    #[doc = "0x2940 - Line threshold set 3\n\nNote: all values are unsigned integer \n\n\n\n"]
    #[inline(always)]
    pub const fn dpcc_line_thresh_3(&self) -> &DpccLineThresh3 {
        &self.dpcc_line_thresh_3
    }
    #[doc = "0x2944 - Mean Absolute Difference (MAD) factor for Line check set 3\n\nNote: all values are unsigned integer \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn dpcc_line_mad_fac_3(&self) -> &DpccLineMadFac3 {
        &self.dpcc_line_mad_fac_3
    }
    #[doc = "0x2948 - Peak gradient factor for set 3\n\nNote: all values are unsigned integer \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn dpcc_pg_fac_3(&self) -> &DpccPgFac3 {
        &self.dpcc_pg_fac_3
    }
    #[doc = "0x294c - Rank Neighbor Difference threshold for set 3\n\nNote: all values are unsigned integer \n\n\n\n"]
    #[inline(always)]
    pub const fn dpcc_rnd_thresh_3(&self) -> &DpccRndThresh3 {
        &self.dpcc_rnd_thresh_3
    }
    #[doc = "0x2950 - Rank gradient factor for set 3\n\nNote: all values are unsigned integer \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn dpcc_rg_fac_3(&self) -> &DpccRgFac3 {
        &self.dpcc_rg_fac_3
    }
    #[doc = "0x2954 - Rank Order Limits\n\nNote: all values are unsigned integer \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn dpcc_ro_limits(&self) -> &DpccRoLimits {
        &self.dpcc_ro_limits
    }
    #[doc = "0x2958 - Differential Rank Offsets for Rank Neighbor Difference\n\nNote: all values are unsigned integer \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn dpcc_rnd_offs(&self) -> &DpccRndOffs {
        &self.dpcc_rnd_offs
    }
    #[doc = "0x295c - bad pixel table settings\n\nNote: This register controls the behaviour of the table based bad pixel correction module. \n\nIt can be switched on and off independently of the DPCC detection and correction block. \n\nDifferent correction algorithms for the table based correction are available and are defined by \n\nthis register. The default setting after reset enables a correction algorithm with most accurate \n\ncorrelation to surrounding pixels. Detection for the table based correction can be configured \n\nindependently from the on-the-fly DPCC detection scheme. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn dpcc_bpt_ctrl(&self) -> &DpccBptCtrl {
        &self.dpcc_bpt_ctrl
    }
    #[doc = "0x2960 - Number of entries for bad pixel table (table based correction)\n\nNote: bit width of bp_number depends on size of BP RAM which is defined during chip \n\n\n\nsynthesis \n\n \n\n"]
    #[inline(always)]
    pub const fn dpcc_bpt_number(&self) -> &DpccBptNumber {
        &self.dpcc_bpt_number
    }
    #[doc = "0x2964 - TABLE Start Address for table-based correction algorithm\n\nNote: MKOE tbc: Orignial register mode was rwh which is no longer supported with new \n\n\n\nversion of SIG \n\n\n\n-> rwhh \n\n\n\n"]
    #[inline(always)]
    pub const fn dpcc_bpt_addr(&self) -> &DpccBptAddr {
        &self.dpcc_bpt_addr
    }
    #[doc = "0x2968 - TABLE DATA register for read and write access of table RAM\n\nNote: MKOE tbc: Orignial register mode was rwh which is no longer supported with new \n\n\n\nversion of SIG \n\n\n\n-> rwhh \n\n\n\nThe programmed table value is immediately written into the RAM. The RAM address is \n\n\n\ngenerated per auto-increment. The parameter RAMs for Lens Shade Correction and Bad \n\n\n\nPixel Correction can only be programmed, if the RGB Bayer path is switched on via ISP_CTRL \n\n"]
    #[inline(always)]
    pub const fn dpcc_bpt_data(&self) -> &DpccBptData {
        &self.dpcc_bpt_data
    }
    #[doc = "0x2a00 - Control Bits for Wide Dynamic Range Unit"]
    #[inline(always)]
    pub const fn wdr_ctrl(&self) -> &WdrCtrl {
        &self.wdr_ctrl
    }
    #[doc = "0x2a04 - Tone Curve sample points dYn definition (part 1)\n\nNote: The interval widths dYn are to be defined in a 2^(value+3) notation, where \n\n\n\n'value' has to be written to the register. So the steps would be \n\n\n\ndYn=0 -> 8 (2^3), dYn=1 -> 16 (2^4), dYn=2 \n\n\n\n-> 32 (2^5),... dYn=6 -> 512 (2^9), dYn=7 -> \n\n\n\n1024 (2^10). \n\n"]
    #[inline(always)]
    pub const fn wdr_tonecurve_1(&self) -> &WdrTonecurve1 {
        &self.wdr_tonecurve_1
    }
    #[doc = "0x2a08 - Tone Curve sample points dYn definition (part 2)\n\nNote: The interval widths dYn are to be defined in a 2^(value+3) notation, where \n\n\n\n'value' has to be written to the register. So the steps would be \n\n\n\ndYn=0 -> 8 (2^3), dYn=1 -> 16 (2^4), dYn=2 \n\n\n\n-> 32 (2^5),... dYn=6 -> 512 (2^9), dYn=7 -> \n\n\n\n1024 (2^10). \n\n"]
    #[inline(always)]
    pub const fn wdr_tonecurve_2(&self) -> &WdrTonecurve2 {
        &self.wdr_tonecurve_2
    }
    #[doc = "0x2a0c - Tone Curve sample points dYn definition (part 3)\n\nNote: The interval widths dYn are to be defined in a 2^(value+3) notation, where \n\n\n\n'value' has to be written to the register. So the steps would be \n\n\n\ndYn=0 -> 8 (2^3), dYn=1 -> 16 (2^4), dYn=2 \n\n\n\n-> 32 (2^5),... dYn=6 -> 512 (2^9), dYn=7 -> \n\n\n\n1024 (2^10). \n\n"]
    #[inline(always)]
    pub const fn wdr_tonecurve_3(&self) -> &WdrTonecurve3 {
        &self.wdr_tonecurve_3
    }
    #[doc = "0x2a10 - Tone Curve sample points dYn definition (part 4)\n\nNote: The interval widths dYn are to be defined in a 2^(value+3) notation, where 'value' \n\nhas to be written to the register. So the steps would be \n\n\n\ndYn=0 -> 8 (2^3), dYn=1 -> 16 (2^4), dYn=2 -> 32 (2^5),... dYn=6 -> 512 (2^9), \n\ndYn=7 -> 1024 (2^10). \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn wdr_tonecurve_4(&self) -> &WdrTonecurve4 {
        &self.wdr_tonecurve_4
    }
    #[doc = "0x2a14..0x2a98 - Tonemapping curve coefficient Ym_ n (n=0..32)\n\nNote: The reset values define a linear curve which has the same effect as bypass. Reset \n\nvalues are: Ym_00 = 0x0000, Ym_01 = 0x0080, Ym_02 = 0x0100, Ym_03 = 0x0180, Ym_04 \n\n= 0x0200, \n\n\n\nYm_05 = 0x0280, Ym_06 = 0x0300, Ym_07 = 0x0380, Ym_08 = 0x0400, Ym_09 = \n\n0x0480, Ym_10 = 0x0500, Ym_11 = 0x0580, Ym_12 = 0x0600, Ym_13 = 0x0680, Ym_14 = \n\n0x0700, Ym_15 = 0x0780, Ym_16 = 0x0800, Ym_17 = 0x0880, Ym_18 = 0x0900, Ym_19 = \n\n0x0980, Ym_20 = 0x0A00, Ym_21 = 0x0A80, Ym_22 = 0x0B00, Ym_23 = 0x0B80, Ym_24 = \n\n0x0C00, Ym_25 = 0x0C80, Ym_26 = 0x0D00, Ym_27 = 0x0D80, Ym_28 = 0x0E00, Ym_29 = \n\n0x0E80, Ym_30 = 0x0F00, Ym_31 = 0x0F80, Ym_32 = 0x1000 \n\n\n\n \n\n\n\nData format: 13 bit unsigned \n\n \n\nRESTRICTION: each Y must be in the +2047/-2048 range compared to its predecessor (so \n\n\n\nthat the difference between successive Y values is 12-bit signed !) \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn wdr_tonecurve_ym(&self, n: usize) -> &WdrTonecurveYm {
        &self.wdr_tonecurve_ym[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2a14..0x2a98 - Tonemapping curve coefficient Ym_ n (n=0..32)\n\nNote: The reset values define a linear curve which has the same effect as bypass. Reset \n\nvalues are: Ym_00 = 0x0000, Ym_01 = 0x0080, Ym_02 = 0x0100, Ym_03 = 0x0180, Ym_04 \n\n= 0x0200, \n\n\n\nYm_05 = 0x0280, Ym_06 = 0x0300, Ym_07 = 0x0380, Ym_08 = 0x0400, Ym_09 = \n\n0x0480, Ym_10 = 0x0500, Ym_11 = 0x0580, Ym_12 = 0x0600, Ym_13 = 0x0680, Ym_14 = \n\n0x0700, Ym_15 = 0x0780, Ym_16 = 0x0800, Ym_17 = 0x0880, Ym_18 = 0x0900, Ym_19 = \n\n0x0980, Ym_20 = 0x0A00, Ym_21 = 0x0A80, Ym_22 = 0x0B00, Ym_23 = 0x0B80, Ym_24 = \n\n0x0C00, Ym_25 = 0x0C80, Ym_26 = 0x0D00, Ym_27 = 0x0D80, Ym_28 = 0x0E00, Ym_29 = \n\n0x0E80, Ym_30 = 0x0F00, Ym_31 = 0x0F80, Ym_32 = 0x1000 \n\n\n\n \n\n\n\nData format: 13 bit unsigned \n\n \n\nRESTRICTION: each Y must be in the +2047/-2048 range compared to its predecessor (so \n\n\n\nthat the difference between successive Y values is 12-bit signed !) \n\n\n\n \n\n"]
    #[inline(always)]
    pub fn wdr_tonecurve_ym_iter(&self) -> impl Iterator<Item = &WdrTonecurveYm> {
        self.wdr_tonecurve_ym.iter()
    }
    #[doc = "0x2a98 - Offset values for RGB path"]
    #[inline(always)]
    pub const fn wdr_offset(&self) -> &WdrOffset {
        &self.wdr_offset
    }
    #[doc = "0x2a9c - DeltaMin Threshold and Strength factor"]
    #[inline(always)]
    pub const fn wdr_deltamin(&self) -> &WdrDeltamin {
        &self.wdr_deltamin
    }
    #[doc = "0x2aa0 - Tone Curve sample points dYn definition shadow register (part 1)\n\nNote: see register ISP_WDR_TONECURVE_1. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn wdr_tonecurve_1_shd(&self) -> &WdrTonecurve1Shd {
        &self.wdr_tonecurve_1_shd
    }
    #[doc = "0x2aa4 - Tone Curve sample points dYn definition shadow register (part 2)\n\nNote: see register ISP_WDR_TONECURVE_2. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn wdr_tonecurve_2_shd(&self) -> &WdrTonecurve2Shd {
        &self.wdr_tonecurve_2_shd
    }
    #[doc = "0x2aa8 - Tone Curve sample points dYn definition shadow register (part 3)\n\nNote: see register ISP_WDR_TONECURVE_3. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn wdr_tonecurve_3_shd(&self) -> &WdrTonecurve3Shd {
        &self.wdr_tonecurve_3_shd
    }
    #[doc = "0x2aac - Tone Curve sample points dYn definition shadow register(part 4)\n\nNote: see register ISP_WDR_TONECURVE_4. \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn wdr_tonecurve_4_shd(&self) -> &WdrTonecurve4Shd {
        &self.wdr_tonecurve_4_shd
    }
    #[doc = "0x2ab0..0x2b34 - Tonemapping curve coefficient shadow register n (n=0..32)\n\nNote: The reset values define a linear curve which has the same effect as bypass. Reset \n\nvalues are: Ym_00 = 0x0000, Ym_01 = 0x0080, Ym_02 = 0x0100, Ym_03 = 0x0180, Ym_04 \n\n= 0x0200, \n\n\n\nYm_05 = 0x0280, Ym_06 = 0x0300, Ym_07 = 0x0380, Ym_08 = 0x0400, Ym_09 = \n\n0x0480, Ym_10 = 0x0500, Ym_11 = 0x0580, Ym_12 = 0x0600, Ym_13 = 0x0680, Ym_14 = \n\n0x0700, Ym_15 = 0x0780, Ym_16 = 0x0800, Ym_17 = 0x0880, Ym_18 = 0x0900, Ym_19 = \n\n0x0980, Ym_20 = 0x0A00, Ym_21 = 0x0A80, Ym_22 = 0x0B00, Ym_23 = 0x0B80, Ym_24 = \n\n0x0C00, Ym_25 = 0x0C80, Ym_26 = 0x0D00, Ym_27 = 0x0D80, Ym_28 = 0x0E00, Ym_29 = \n\n0x0E80, Ym_30 = 0x0F00, Ym_31 = 0x0F80, Ym_32 = 0x1000 \n\n\n\n \n\nData format: 13 bit unsigned \n\n \n\n\n\nRESTRICTION: each Y must be in the +2047/-2048 range compared to its predecessor (so \n\nthat the difference between successive Y values is 12-bit signed !) \n\n\n\n \n\n"]
    #[inline(always)]
    pub const fn wdr_tonecurve_ym_shd(&self, n: usize) -> &WdrTonecurveYmShd {
        &self.wdr_tonecurve_ym_shd[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2ab0..0x2b34 - Tonemapping curve coefficient shadow register n (n=0..32)\n\nNote: The reset values define a linear curve which has the same effect as bypass. Reset \n\nvalues are: Ym_00 = 0x0000, Ym_01 = 0x0080, Ym_02 = 0x0100, Ym_03 = 0x0180, Ym_04 \n\n= 0x0200, \n\n\n\nYm_05 = 0x0280, Ym_06 = 0x0300, Ym_07 = 0x0380, Ym_08 = 0x0400, Ym_09 = \n\n0x0480, Ym_10 = 0x0500, Ym_11 = 0x0580, Ym_12 = 0x0600, Ym_13 = 0x0680, Ym_14 = \n\n0x0700, Ym_15 = 0x0780, Ym_16 = 0x0800, Ym_17 = 0x0880, Ym_18 = 0x0900, Ym_19 = \n\n0x0980, Ym_20 = 0x0A00, Ym_21 = 0x0A80, Ym_22 = 0x0B00, Ym_23 = 0x0B80, Ym_24 = \n\n0x0C00, Ym_25 = 0x0C80, Ym_26 = 0x0D00, Ym_27 = 0x0D80, Ym_28 = 0x0E00, Ym_29 = \n\n0x0E80, Ym_30 = 0x0F00, Ym_31 = 0x0F80, Ym_32 = 0x1000 \n\n\n\n \n\nData format: 13 bit unsigned \n\n \n\n\n\nRESTRICTION: each Y must be in the +2047/-2048 range compared to its predecessor (so \n\nthat the difference between successive Y values is 12-bit signed !) \n\n\n\n \n\n"]
    #[inline(always)]
    pub fn wdr_tonecurve_ym_shd_iter(&self) -> impl Iterator<Item = &WdrTonecurveYmShd> {
        self.wdr_tonecurve_ym_shd.iter()
    }
    #[doc = "0x2f00 - VS Measure Mode"]
    #[inline(always)]
    pub const fn vsm_mode(&self) -> &VsmMode {
        &self.vsm_mode
    }
    #[doc = "0x2f04 - VSM window horizontal offset"]
    #[inline(always)]
    pub const fn vsm_h_offs(&self) -> &VsmHOffs {
        &self.vsm_h_offs
    }
    #[doc = "0x2f08 - VSM window vertical offset"]
    #[inline(always)]
    pub const fn vsm_v_offs(&self) -> &VsmVOffs {
        &self.vsm_v_offs
    }
    #[doc = "0x2f0c - Horizontal measure window size\n\nNote: only even values are allowed: vsm_h_size\\[0\\]
not writable and read returns 0. \n\n\n\n \n\n\n\n"]
    #[inline(always)]
    pub const fn vsm_h_size(&self) -> &VsmHSize {
        &self.vsm_h_size
    }
    #[doc = "0x2f10 - Vertical measure window size\n\nNote: only even values are allowed: vsm_v_size\\[0\\]
not writable and read returns 0. \n\n\n\n \n\n\n\n"]
    #[inline(always)]
    pub const fn vsm_v_size(&self) -> &VsmVSize {
        &self.vsm_v_size
    }
    #[doc = "0x2f14 - Iteration 1 horizontal segments\n\nNote: number of 1st iteration sample points = vsm_h_segments + 1 \n\n\n\n \n\n\n\n"]
    #[inline(always)]
    pub const fn vsm_h_segments(&self) -> &VsmHSegments {
        &self.vsm_h_segments
    }
    #[doc = "0x2f18 - Iteration 1 vertical segments\n\nNote: number of 1st iteration sample points = vsm_v_segments + 1 \n\n\n\n \n\n\n\n"]
    #[inline(always)]
    pub const fn vsm_v_segments(&self) -> &VsmVSegments {
        &self.vsm_v_segments
    }
    #[doc = "0x2f1c - estimated horizontal displacement"]
    #[inline(always)]
    pub const fn vsm_delta_h(&self) -> &VsmDeltaH {
        &self.vsm_delta_h
    }
}
#[doc = "VI_CCL (rw) register accessor: Clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vi_ccl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vi_ccl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vi_ccl`]
module"]
#[doc(alias = "VI_CCL")]
pub type ViCcl = crate::Reg<vi_ccl::ViCclSpec>;
#[doc = "Clock control register"]
pub mod vi_ccl;
#[doc = "VI_ICCL (rw) register accessor: Internal clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vi_iccl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vi_iccl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vi_iccl`]
module"]
#[doc(alias = "VI_ICCL")]
pub type ViIccl = crate::Reg<vi_iccl::ViIcclSpec>;
#[doc = "Internal clock control register"]
pub mod vi_iccl;
#[doc = "VI_IRCL (rw) register accessor: Internal reset control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vi_ircl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vi_ircl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vi_ircl`]
module"]
#[doc(alias = "VI_IRCL")]
pub type ViIrcl = crate::Reg<vi_ircl::ViIrclSpec>;
#[doc = "Internal reset control register"]
pub mod vi_ircl;
#[doc = "VI_DPCL (rw) register accessor: Data path control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vi_dpcl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vi_dpcl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vi_dpcl`]
module"]
#[doc(alias = "VI_DPCL")]
pub type ViDpcl = crate::Reg<vi_dpcl::ViDpclSpec>;
#[doc = "Data path control register"]
pub mod vi_dpcl;
#[doc = "IMG_EFF_CTRL (rw) register accessor: Global control register\n\nNote: full_range for image effects is supported in ISP M5_v6, M5_v7 only \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`img_eff_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`img_eff_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@img_eff_ctrl`]
module"]
#[doc(alias = "IMG_EFF_CTRL")]
pub type ImgEffCtrl = crate::Reg<img_eff_ctrl::ImgEffCtrlSpec>;
#[doc = "Global control register\n\nNote: full_range for image effects is supported in ISP M5_v6, M5_v7 only \n\n\n\n"]
pub mod img_eff_ctrl;
#[doc = "IMG_EFF_COLOR_SEL (rw) register accessor: Color selection register (for color selection effect)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`img_eff_color_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`img_eff_color_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@img_eff_color_sel`]
module"]
#[doc(alias = "IMG_EFF_COLOR_SEL")]
pub type ImgEffColorSel = crate::Reg<img_eff_color_sel::ImgEffColorSelSpec>;
#[doc = "Color selection register (for color selection effect)"]
pub mod img_eff_color_sel;
#[doc = "IMG_EFF_MAT_1 (rw) register accessor: 3x3 matrix coefficients for emboss effect (1)\n\nNote: possible values for coefficients: 000 (1), 001 (2), 010 (4), 011 (8),100 (-1), 101 \n\n(-2), 110 (-4), 111 (-8) \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`img_eff_mat_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`img_eff_mat_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@img_eff_mat_1`]
module"]
#[doc(alias = "IMG_EFF_MAT_1")]
pub type ImgEffMat1 = crate::Reg<img_eff_mat_1::ImgEffMat1Spec>;
#[doc = "3x3 matrix coefficients for emboss effect (1)\n\nNote: possible values for coefficients: 000 (1), 001 (2), 010 (4), 011 (8),100 (-1), 101 \n\n(-2), 110 (-4), 111 (-8) \n\n\n\n"]
pub mod img_eff_mat_1;
#[doc = "IMG_EFF_MAT_2 (rw) register accessor: 3x3 matrix coefficients for emboss effect (2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`img_eff_mat_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`img_eff_mat_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@img_eff_mat_2`]
module"]
#[doc(alias = "IMG_EFF_MAT_2")]
pub type ImgEffMat2 = crate::Reg<img_eff_mat_2::ImgEffMat2Spec>;
#[doc = "3x3 matrix coefficients for emboss effect (2)"]
pub mod img_eff_mat_2;
#[doc = "IMG_EFF_MAT_3 (rw) register accessor: 3x3 matrix coefficients for emboss(3) effect / \n\n\n\nsketch/sharpen(1) effect\n\nNote: possible values for \n\n\n\ncoefficients: 000 (1), 001 (2), 010 \n\n\n\n(4), 011 (8), \n\n100 (-1), 101 (-2), 110 (-4), 111 (-8) \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`img_eff_mat_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`img_eff_mat_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@img_eff_mat_3`]
module"]
#[doc(alias = "IMG_EFF_MAT_3")]
pub type ImgEffMat3 = crate::Reg<img_eff_mat_3::ImgEffMat3Spec>;
#[doc = "3x3 matrix coefficients for emboss(3) effect / \n\n\n\nsketch/sharpen(1) effect\n\nNote: possible values for \n\n\n\ncoefficients: 000 (1), 001 (2), 010 \n\n\n\n(4), 011 (8), \n\n100 (-1), 101 (-2), 110 (-4), 111 (-8) \n\n\n\n \n\n"]
pub mod img_eff_mat_3;
#[doc = "IMG_EFF_MAT_4 (rw) register accessor: 3x3 matrix coefficients for sketch/sharpen effect (2)\n\nNote: possible values for coefficients: 000 (1), 001 (2), 010 (4), 011 (8), \n\n100 (-1), 101 (-2), 110 (-4), 111 (-8) \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`img_eff_mat_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`img_eff_mat_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@img_eff_mat_4`]
module"]
#[doc(alias = "IMG_EFF_MAT_4")]
pub type ImgEffMat4 = crate::Reg<img_eff_mat_4::ImgEffMat4Spec>;
#[doc = "3x3 matrix coefficients for sketch/sharpen effect (2)\n\nNote: possible values for coefficients: 000 (1), 001 (2), 010 (4), 011 (8), \n\n100 (-1), 101 (-2), 110 (-4), 111 (-8) \n\n\n\n \n\n"]
pub mod img_eff_mat_4;
#[doc = "IMG_EFF_MAT_5 (rw) register accessor: 3x3 matrix coefficients for sketch/sharpen effect (3)\n\nNote: possible values for coefficients: 000 (1), 001 (2), 010 (4), 011 (8), \n\n100 (-1), 101 (-2), 110 (-4), 111 (-8) \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`img_eff_mat_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`img_eff_mat_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@img_eff_mat_5`]
module"]
#[doc(alias = "IMG_EFF_MAT_5")]
pub type ImgEffMat5 = crate::Reg<img_eff_mat_5::ImgEffMat5Spec>;
#[doc = "3x3 matrix coefficients for sketch/sharpen effect (3)\n\nNote: possible values for coefficients: 000 (1), 001 (2), 010 (4), 011 (8), \n\n100 (-1), 101 (-2), 110 (-4), 111 (-8) \n\n\n\n"]
pub mod img_eff_mat_5;
#[doc = "IMG_EFF_TINT (rw) register accessor: Chrominance increment values of a tint (used for sepia effect)\n\nNote: Calculation process of incr_cr and incr_cb: \n\ntint values given in RGB format: R G B \n\nconverted to Cb and Cr: Cb = -0.148*R - 0.291*G + 0.439*B + 128 Cr = 0.439*R - \n\n\n\n0.368*G - 0.071*B + 128 \n\ncalculating of the increments inc_Cb = (128 – Cb)/110 inc_Cr = (128 – Cr)/110 \n\nregister entry of the increments with an accuracy of 1/64 incr_cb = inc_Cb * 64 \n\nincr_cr = inc_Cr * 64 \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`img_eff_tint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`img_eff_tint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@img_eff_tint`]
module"]
#[doc(alias = "IMG_EFF_TINT")]
pub type ImgEffTint = crate::Reg<img_eff_tint::ImgEffTintSpec>;
#[doc = "Chrominance increment values of a tint (used for sepia effect)\n\nNote: Calculation process of incr_cr and incr_cb: \n\ntint values given in RGB format: R G B \n\nconverted to Cb and Cr: Cb = -0.148*R - 0.291*G + 0.439*B + 128 Cr = 0.439*R - \n\n\n\n0.368*G - 0.071*B + 128 \n\ncalculating of the increments inc_Cb = (128 – Cb)/110 inc_Cr = (128 – Cr)/110 \n\nregister entry of the increments with an accuracy of 1/64 incr_cb = inc_Cb * 64 \n\nincr_cr = inc_Cr * 64 \n\n\n\n \n\n"]
pub mod img_eff_tint;
#[doc = "IMG_EFF_CTRL_SHD (r) register accessor: Shadow register for control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`img_eff_ctrl_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@img_eff_ctrl_shd`]
module"]
#[doc(alias = "IMG_EFF_CTRL_SHD")]
pub type ImgEffCtrlShd = crate::Reg<img_eff_ctrl_shd::ImgEffCtrlShdSpec>;
#[doc = "Shadow register for control register"]
pub mod img_eff_ctrl_shd;
#[doc = "IMG_EFF_SHARPEN (rw) register accessor: Factor and threshold for sharpen effect\n\nNote: For the sharpening effect the convolution mask must be set to the values \\[-1 -1 -1; \n\n-1 8 -1; -1 -1 -1\\]. \n\n\n\nThe convolution mask for sharpening is defined by the values sket_coef_xx in registers \n\nIMG_EFF_MAT_3 through IMG_EFF_MAT_5. Sketch effect and sharpening effect share the \n\nsame mask registers. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`img_eff_sharpen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`img_eff_sharpen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@img_eff_sharpen`]
module"]
#[doc(alias = "IMG_EFF_SHARPEN")]
pub type ImgEffSharpen = crate::Reg<img_eff_sharpen::ImgEffSharpenSpec>;
#[doc = "Factor and threshold for sharpen effect\n\nNote: For the sharpening effect the convolution mask must be set to the values \\[-1 -1 -1; \n\n-1 8 -1; -1 -1 -1\\]. \n\n\n\nThe convolution mask for sharpening is defined by the values sket_coef_xx in registers \n\nIMG_EFF_MAT_3 through IMG_EFF_MAT_5. Sketch effect and sharpening effect share the \n\nsame mask registers. \n\n\n\n \n\n"]
pub mod img_eff_sharpen;
#[doc = "SUPER_IMP_CTRL (rw) register accessor: Global control register\n\nNote: in the bypass mode the data stream from Image \n\n\n\nEffect is transmitted to MUX module without overlaying \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`super_imp_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`super_imp_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@super_imp_ctrl`]
module"]
#[doc(alias = "SUPER_IMP_CTRL")]
pub type SuperImpCtrl = crate::Reg<super_imp_ctrl::SuperImpCtrlSpec>;
#[doc = "Global control register\n\nNote: in the bypass mode the data stream from Image \n\n\n\nEffect is transmitted to MUX module without overlaying \n\n\n\n \n\n"]
pub mod super_imp_ctrl;
#[doc = "SUPER_IMP_OFFSET_X (rw) register accessor: Offset x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`super_imp_offset_x::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`super_imp_offset_x::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@super_imp_offset_x`]
module"]
#[doc(alias = "SUPER_IMP_OFFSET_X")]
pub type SuperImpOffsetX = crate::Reg<super_imp_offset_x::SuperImpOffsetXSpec>;
#[doc = "Offset x register"]
pub mod super_imp_offset_x;
#[doc = "SUPER_IMP_OFFSET_Y (rw) register accessor: Offset y register\n\nNote: the offset_y is positive and refers to the \n\n\n\nreference image \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`super_imp_offset_y::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`super_imp_offset_y::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@super_imp_offset_y`]
module"]
#[doc(alias = "SUPER_IMP_OFFSET_Y")]
pub type SuperImpOffsetY = crate::Reg<super_imp_offset_y::SuperImpOffsetYSpec>;
#[doc = "Offset y register\n\nNote: the offset_y is positive and refers to the \n\n\n\nreference image \n\n\n\n"]
pub mod super_imp_offset_y;
#[doc = "CTRL (rw) register accessor: global control register\n\nNote: partly write-only \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "global control register\n\nNote: partly write-only \n\n\n\n \n\n"]
pub mod ctrl;
#[doc = "ACQ_PROP (rw) register accessor: ISP acquisition properties\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acq_prop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acq_prop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acq_prop`]
module"]
#[doc(alias = "ACQ_PROP")]
pub type AcqProp = crate::Reg<acq_prop::AcqPropSpec>;
#[doc = "ISP acquisition properties"]
pub mod acq_prop;
#[doc = "ACQ_H_OFFS (rw) register accessor: horizontal input offset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acq_h_offs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acq_h_offs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acq_h_offs`]
module"]
#[doc(alias = "ACQ_H_OFFS")]
pub type AcqHOffs = crate::Reg<acq_h_offs::AcqHOffsSpec>;
#[doc = "horizontal input offset"]
pub mod acq_h_offs;
#[doc = "ACQ_V_OFFS (rw) register accessor: vertical input offset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acq_v_offs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acq_v_offs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acq_v_offs`]
module"]
#[doc(alias = "ACQ_V_OFFS")]
pub type AcqVOffs = crate::Reg<acq_v_offs::AcqVOffsSpec>;
#[doc = "vertical input offset"]
pub mod acq_v_offs;
#[doc = "ACQ_H_SIZE (rw) register accessor: horizontal input size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acq_h_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acq_h_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acq_h_size`]
module"]
#[doc(alias = "ACQ_H_SIZE")]
pub type AcqHSize = crate::Reg<acq_h_size::AcqHSizeSpec>;
#[doc = "horizontal input size"]
pub mod acq_h_size;
#[doc = "ACQ_V_SIZE (rw) register accessor: vertical input size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acq_v_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acq_v_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acq_v_size`]
module"]
#[doc(alias = "ACQ_V_SIZE")]
pub type AcqVSize = crate::Reg<acq_v_size::AcqVSizeSpec>;
#[doc = "vertical input size"]
pub mod acq_v_size;
#[doc = "ACQ_NR_FRAMES (rw) register accessor: Number of frames to be captured\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acq_nr_frames::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acq_nr_frames::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acq_nr_frames`]
module"]
#[doc(alias = "ACQ_NR_FRAMES")]
pub type AcqNrFrames = crate::Reg<acq_nr_frames::AcqNrFramesSpec>;
#[doc = "Number of frames to be captured"]
pub mod acq_nr_frames;
#[doc = "GAMMA_DX_LO (rw) register accessor: De-Gamma Curve definition lower x increments (sampling points)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gamma_dx_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_dx_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gamma_dx_lo`]
module"]
#[doc(alias = "GAMMA_DX_LO")]
pub type GammaDxLo = crate::Reg<gamma_dx_lo::GammaDxLoSpec>;
#[doc = "De-Gamma Curve definition lower x increments (sampling points)"]
pub mod gamma_dx_lo;
#[doc = "GAMMA_DX_HI (rw) register accessor: De-Gamma Curve definition higher x increments (sampling points)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gamma_dx_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_dx_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gamma_dx_hi`]
module"]
#[doc(alias = "GAMMA_DX_HI")]
pub type GammaDxHi = crate::Reg<gamma_dx_hi::GammaDxHiSpec>;
#[doc = "De-Gamma Curve definition higher x increments (sampling points)"]
pub mod gamma_dx_hi;
#[doc = "GAMMA_R_Y (rw) register accessor: De-Gamma Curve definition y red n (n=0..16)\n\nNote: The reset values define a linear curve which has the same effect as bypass. \n\n\n\nReset values are: Y_00 = 0x0000, Y_01 = 0x0100, Y_02 = 0x0200, Y_03 = 0x0300, Y_04 \n\n\n\n= 0x0400,Y_05 = 0x0500, Y_06 = 0x0600, Y_07 = 0x0700, Y_08 = 0x0800, Y_09 = \n\n\n\n0x0900, Y_10 = 0x0A00, Y_11 = 0x0B00, Y_12 = 0x0C00, Y_13 = 0x0D00, Y_14 = \n\n\n\n0x0E00, Y_15 = 0x0F00, Y_16 = 0x0FFF \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gamma_r_y::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_r_y::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gamma_r_y`]
module"]
#[doc(alias = "GAMMA_R_Y")]
pub type GammaRY = crate::Reg<gamma_r_y::GammaRYSpec>;
#[doc = "De-Gamma Curve definition y red n (n=0..16)\n\nNote: The reset values define a linear curve which has the same effect as bypass. \n\n\n\nReset values are: Y_00 = 0x0000, Y_01 = 0x0100, Y_02 = 0x0200, Y_03 = 0x0300, Y_04 \n\n\n\n= 0x0400,Y_05 = 0x0500, Y_06 = 0x0600, Y_07 = 0x0700, Y_08 = 0x0800, Y_09 = \n\n\n\n0x0900, Y_10 = 0x0A00, Y_11 = 0x0B00, Y_12 = 0x0C00, Y_13 = 0x0D00, Y_14 = \n\n\n\n0x0E00, Y_15 = 0x0F00, Y_16 = 0x0FFF \n\n\n\n \n\n"]
pub mod gamma_r_y;
#[doc = "GAMMA_G_Y (rw) register accessor: De-Gamma Curve definition y green n (n=0..16)\n\nNote: The reset values define a linear curve which has the same effect as bypass. \n\n\n\nReset values are: Y_00 = 0x0000, Y_01 = 0x0100, Y_02 = 0x0200, Y_03 = 0x0300, Y_04 \n\n\n\n= 0x0400,Y_05 = 0x0500, Y_06 = 0x0600, Y_07 = 0x0700, Y_08 = 0x0800, Y_09 = \n\n\n\n0x0900, Y_10 = 0x0A00, Y_11 = 0x0B00, Y_12 = 0x0C00, Y_13 = 0x0D00, Y_14 = \n\n\n\n0x0E00, Y_15 = 0x0F00, Y_16 = 0x0FFF \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gamma_g_y::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_g_y::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gamma_g_y`]
module"]
#[doc(alias = "GAMMA_G_Y")]
pub type GammaGY = crate::Reg<gamma_g_y::GammaGYSpec>;
#[doc = "De-Gamma Curve definition y green n (n=0..16)\n\nNote: The reset values define a linear curve which has the same effect as bypass. \n\n\n\nReset values are: Y_00 = 0x0000, Y_01 = 0x0100, Y_02 = 0x0200, Y_03 = 0x0300, Y_04 \n\n\n\n= 0x0400,Y_05 = 0x0500, Y_06 = 0x0600, Y_07 = 0x0700, Y_08 = 0x0800, Y_09 = \n\n\n\n0x0900, Y_10 = 0x0A00, Y_11 = 0x0B00, Y_12 = 0x0C00, Y_13 = 0x0D00, Y_14 = \n\n\n\n0x0E00, Y_15 = 0x0F00, Y_16 = 0x0FFF \n\n\n\n \n\n"]
pub mod gamma_g_y;
#[doc = "GAMMA_B_Y (rw) register accessor: De-Gamma Curve definition y blue n (n=0..16)\n\nNote: The reset values define a linear curve which has the same effect as bypass. \n\n\n\nReset values are: Y_00 = 0x0000, Y_01 = 0x0100, Y_02 = 0x0200, Y_03 = 0x0300, Y_04 \n\n\n\n= 0x0400, \n\n\n\nY_05 = 0x0500, Y_06 = 0x0600, Y_07 = 0x0700, Y_08 = 0x0800, \n\n\n\nY_09 = 0x0900, Y_10 = 0x0A00, Y_11 = 0x0B00, Y_12 = 0x0C00, Y_13 = \n\n\n\n0x0D00, Y_14 = 0x0E00, Y_15 = 0x0F00, Y_16 = 0x0FFF \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gamma_b_y::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_b_y::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gamma_b_y`]
module"]
#[doc(alias = "GAMMA_B_Y")]
pub type GammaBY = crate::Reg<gamma_b_y::GammaBYSpec>;
#[doc = "De-Gamma Curve definition y blue n (n=0..16)\n\nNote: The reset values define a linear curve which has the same effect as bypass. \n\n\n\nReset values are: Y_00 = 0x0000, Y_01 = 0x0100, Y_02 = 0x0200, Y_03 = 0x0300, Y_04 \n\n\n\n= 0x0400, \n\n\n\nY_05 = 0x0500, Y_06 = 0x0600, Y_07 = 0x0700, Y_08 = 0x0800, \n\n\n\nY_09 = 0x0900, Y_10 = 0x0A00, Y_11 = 0x0B00, Y_12 = 0x0C00, Y_13 = \n\n\n\n0x0D00, Y_14 = 0x0E00, Y_15 = 0x0F00, Y_16 = 0x0FFF \n\n"]
pub mod gamma_b_y;
#[doc = "AWB_PROP (rw) register accessor: Auto white balance properties\n\nNote: The following conversion matrix is used for calculating the YCbCr values: \n\n\n\nY = 16 + 0.2500 R + 0.5000 G + 0.1094 B \n\nCb = 128 - 0.1406 R - 0.2969 G + 0.4375 B \n\nCr = 128 + 0.4375 R - 0.3750 G - 0.0625 B \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awb_prop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awb_prop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awb_prop`]
module"]
#[doc(alias = "AWB_PROP")]
pub type AwbProp = crate::Reg<awb_prop::AwbPropSpec>;
#[doc = "Auto white balance properties\n\nNote: The following conversion matrix is used for calculating the YCbCr values: \n\n\n\nY = 16 + 0.2500 R + 0.5000 G + 0.1094 B \n\nCb = 128 - 0.1406 R - 0.2969 G + 0.4375 B \n\nCr = 128 + 0.4375 R - 0.3750 G - 0.0625 B \n\n\n\n"]
pub mod awb_prop;
#[doc = "AWB_H_OFFS (rw) register accessor: Auto white balance horizontal offset of measure window\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awb_h_offs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awb_h_offs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awb_h_offs`]
module"]
#[doc(alias = "AWB_H_OFFS")]
pub type AwbHOffs = crate::Reg<awb_h_offs::AwbHOffsSpec>;
#[doc = "Auto white balance horizontal offset of measure window"]
pub mod awb_h_offs;
#[doc = "AWB_V_OFFS (rw) register accessor: Auto white balance vertical offset of measure window\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awb_v_offs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awb_v_offs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awb_v_offs`]
module"]
#[doc(alias = "AWB_V_OFFS")]
pub type AwbVOffs = crate::Reg<awb_v_offs::AwbVOffsSpec>;
#[doc = "Auto white balance vertical offset of measure window"]
pub mod awb_v_offs;
#[doc = "AWB_H_SIZE (rw) register accessor: Auto white balance horizontal window size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awb_h_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awb_h_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awb_h_size`]
module"]
#[doc(alias = "AWB_H_SIZE")]
pub type AwbHSize = crate::Reg<awb_h_size::AwbHSizeSpec>;
#[doc = "Auto white balance horizontal window size"]
pub mod awb_h_size;
#[doc = "AWB_V_SIZE (rw) register accessor: Auto white balance vertical window size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awb_v_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awb_v_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awb_v_size`]
module"]
#[doc(alias = "AWB_V_SIZE")]
pub type AwbVSize = crate::Reg<awb_v_size::AwbVSizeSpec>;
#[doc = "Auto white balance vertical window size"]
pub mod awb_v_size;
#[doc = "AWB_FRAMES (rw) register accessor: Auto white balance mean value over multiple frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awb_frames::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awb_frames::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awb_frames`]
module"]
#[doc(alias = "AWB_FRAMES")]
pub type AwbFrames = crate::Reg<awb_frames::AwbFramesSpec>;
#[doc = "Auto white balance mean value over multiple frames"]
pub mod awb_frames;
#[doc = "AWB_REF (rw) register accessor: Auto white balance reference Cb/Cr values\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awb_ref::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awb_ref::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awb_ref`]
module"]
#[doc(alias = "AWB_REF")]
pub type AwbRef = crate::Reg<awb_ref::AwbRefSpec>;
#[doc = "Auto white balance reference Cb/Cr values"]
pub mod awb_ref;
#[doc = "AWB_THRESH (rw) register accessor: Auto white balance threshold values\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awb_thresh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awb_thresh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awb_thresh`]
module"]
#[doc(alias = "AWB_THRESH")]
pub type AwbThresh = crate::Reg<awb_thresh::AwbThreshSpec>;
#[doc = "Auto white balance threshold values"]
pub mod awb_thresh;
#[doc = "AWB_GAIN_G (rw) register accessor: Auto white balance gain green\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awb_gain_g::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awb_gain_g::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awb_gain_g`]
module"]
#[doc(alias = "AWB_GAIN_G")]
pub type AwbGainG = crate::Reg<awb_gain_g::AwbGainGSpec>;
#[doc = "Auto white balance gain green"]
pub mod awb_gain_g;
#[doc = "AWB_GAIN_RB (rw) register accessor: Auto white balance gain red and blue\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awb_gain_rb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awb_gain_rb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awb_gain_rb`]
module"]
#[doc(alias = "AWB_GAIN_RB")]
pub type AwbGainRb = crate::Reg<awb_gain_rb::AwbGainRbSpec>;
#[doc = "Auto white balance gain red and blue"]
pub mod awb_gain_rb;
#[doc = "AWB_WHITE_CNT (r) register accessor: Auto white balance white pixel count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awb_white_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awb_white_cnt`]
module"]
#[doc(alias = "AWB_WHITE_CNT")]
pub type AwbWhiteCnt = crate::Reg<awb_white_cnt::AwbWhiteCntSpec>;
#[doc = "Auto white balance white pixel count"]
pub mod awb_white_cnt;
#[doc = "AWB_MEAN (r) register accessor: Auto white balance measured mean value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awb_mean::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awb_mean`]
module"]
#[doc(alias = "AWB_MEAN")]
pub type AwbMean = crate::Reg<awb_mean::AwbMeanSpec>;
#[doc = "Auto white balance measured mean value"]
pub mod awb_mean;
#[doc = "CC_COEFF_0 (rw) register accessor: Color conversion coefficient 0\n\nNote: all color conversion coefficients are signed integer values with 7 bit fractional \n\n\n\npart, range -2 to 1.992 \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc_coeff_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc_coeff_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc_coeff_0`]
module"]
#[doc(alias = "CC_COEFF_0")]
pub type CcCoeff0 = crate::Reg<cc_coeff_0::CcCoeff0Spec>;
#[doc = "Color conversion coefficient 0\n\nNote: all color conversion coefficients are signed integer values with 7 bit fractional \n\n\n\npart, range -2 to 1.992 \n\n\n\n \n\n"]
pub mod cc_coeff_0;
#[doc = "CC_COEFF_1 (rw) register accessor: Color conversion coefficient 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc_coeff_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc_coeff_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc_coeff_1`]
module"]
#[doc(alias = "CC_COEFF_1")]
pub type CcCoeff1 = crate::Reg<cc_coeff_1::CcCoeff1Spec>;
#[doc = "Color conversion coefficient 1"]
pub mod cc_coeff_1;
#[doc = "CC_COEFF_2 (rw) register accessor: Color conversion coefficient 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc_coeff_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc_coeff_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc_coeff_2`]
module"]
#[doc(alias = "CC_COEFF_2")]
pub type CcCoeff2 = crate::Reg<cc_coeff_2::CcCoeff2Spec>;
#[doc = "Color conversion coefficient 2"]
pub mod cc_coeff_2;
#[doc = "CC_COEFF_3 (rw) register accessor: Color conversion coefficient 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc_coeff_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc_coeff_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc_coeff_3`]
module"]
#[doc(alias = "CC_COEFF_3")]
pub type CcCoeff3 = crate::Reg<cc_coeff_3::CcCoeff3Spec>;
#[doc = "Color conversion coefficient 3"]
pub mod cc_coeff_3;
#[doc = "CC_COEFF_4 (rw) register accessor: Color conversion coefficient 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc_coeff_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc_coeff_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc_coeff_4`]
module"]
#[doc(alias = "CC_COEFF_4")]
pub type CcCoeff4 = crate::Reg<cc_coeff_4::CcCoeff4Spec>;
#[doc = "Color conversion coefficient 4"]
pub mod cc_coeff_4;
#[doc = "CC_COEFF_5 (rw) register accessor: Color conversion coefficient 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc_coeff_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc_coeff_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc_coeff_5`]
module"]
#[doc(alias = "CC_COEFF_5")]
pub type CcCoeff5 = crate::Reg<cc_coeff_5::CcCoeff5Spec>;
#[doc = "Color conversion coefficient 5"]
pub mod cc_coeff_5;
#[doc = "CC_COEFF_6 (rw) register accessor: Color conversion coefficient 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc_coeff_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc_coeff_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc_coeff_6`]
module"]
#[doc(alias = "CC_COEFF_6")]
pub type CcCoeff6 = crate::Reg<cc_coeff_6::CcCoeff6Spec>;
#[doc = "Color conversion coefficient 6"]
pub mod cc_coeff_6;
#[doc = "CC_COEFF_7 (rw) register accessor: Color conversion coefficient 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc_coeff_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc_coeff_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc_coeff_7`]
module"]
#[doc(alias = "CC_COEFF_7")]
pub type CcCoeff7 = crate::Reg<cc_coeff_7::CcCoeff7Spec>;
#[doc = "Color conversion coefficient 7"]
pub mod cc_coeff_7;
#[doc = "CC_COEFF_8 (rw) register accessor: Color conversion coefficient 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc_coeff_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc_coeff_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc_coeff_8`]
module"]
#[doc(alias = "CC_COEFF_8")]
pub type CcCoeff8 = crate::Reg<cc_coeff_8::CcCoeff8Spec>;
#[doc = "Color conversion coefficient 8"]
pub mod cc_coeff_8;
#[doc = "OUT_H_OFFS (rw) register accessor: Horizontal offset of output window\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_h_offs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_h_offs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_h_offs`]
module"]
#[doc(alias = "OUT_H_OFFS")]
pub type OutHOffs = crate::Reg<out_h_offs::OutHOffsSpec>;
#[doc = "Horizontal offset of output window"]
pub mod out_h_offs;
#[doc = "OUT_V_OFFS (rw) register accessor: Vertical offset of output window\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_v_offs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_v_offs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_v_offs`]
module"]
#[doc(alias = "OUT_V_OFFS")]
pub type OutVOffs = crate::Reg<out_v_offs::OutVOffsSpec>;
#[doc = "Vertical offset of output window"]
pub mod out_v_offs;
#[doc = "OUT_H_SIZE (rw) register accessor: Output horizontal picture size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_h_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_h_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_h_size`]
module"]
#[doc(alias = "OUT_H_SIZE")]
pub type OutHSize = crate::Reg<out_h_size::OutHSizeSpec>;
#[doc = "Output horizontal picture size"]
pub mod out_h_size;
#[doc = "OUT_V_SIZE (rw) register accessor: Output vertical picture size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_v_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_v_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_v_size`]
module"]
#[doc(alias = "OUT_V_SIZE")]
pub type OutVSize = crate::Reg<out_v_size::OutVSizeSpec>;
#[doc = "Output vertical picture size"]
pub mod out_v_size;
#[doc = "DEMOSAIC (rw) register accessor: Demosaic parameters\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`demosaic::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`demosaic::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@demosaic`]
module"]
#[doc(alias = "DEMOSAIC")]
pub type Demosaic = crate::Reg<demosaic::DemosaicSpec>;
#[doc = "Demosaic parameters"]
pub mod demosaic;
#[doc = "FLAGS_SHD (r) register accessor: Flags (current status) of certain signals and Shadow regs \n\n\n\nfor enable signals\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flags_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flags_shd`]
module"]
#[doc(alias = "FLAGS_SHD")]
pub type FlagsShd = crate::Reg<flags_shd::FlagsShdSpec>;
#[doc = "Flags (current status) of certain signals and Shadow regs \n\n\n\nfor enable signals"]
pub mod flags_shd;
#[doc = "IMSC (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imsc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imsc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imsc`]
module"]
#[doc(alias = "IMSC")]
pub type Imsc = crate::Reg<imsc::ImscSpec>;
#[doc = "Interrupt mask"]
pub mod imsc;
#[doc = "RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"]
#[doc(alias = "RIS")]
pub type Ris = crate::Reg<ris::RisSpec>;
#[doc = "Raw interrupt status"]
pub mod ris;
#[doc = "MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`]
module"]
#[doc(alias = "MIS")]
pub type Mis = crate::Reg<mis::MisSpec>;
#[doc = "Masked interrupt status"]
pub mod mis;
#[doc = "ICR (w) register accessor: Interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "Interrupt clear register"]
pub mod icr;
#[doc = "ISR (w) register accessor: Interrupt set register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "Interrupt set register"]
pub mod isr;
#[doc = "CT_COEFF (rw) register accessor: cross-talk configuration register (color correction matrix) n (n=0..8)\n\nNote: Reset values generate a matrix which does not modify the pixel values. Reset \n\nvalues are: coeff_0 = 0x80, coeff_1 = 0x00, coeff_2 = 0x00, coeff_3 = 0x00, coeff_4 = 0x80, \n\n\n\ncoeff_5 = 0x00, coeff_6 = 0x00, coeff_7 = 0x00, coeff_8 = 0x80 \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ct_coeff::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ct_coeff::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ct_coeff`]
module"]
#[doc(alias = "CT_COEFF")]
pub type CtCoeff = crate::Reg<ct_coeff::CtCoeffSpec>;
#[doc = "cross-talk configuration register (color correction matrix) n (n=0..8)\n\nNote: Reset values generate a matrix which does not modify the pixel values. Reset \n\nvalues are: coeff_0 = 0x80, coeff_1 = 0x00, coeff_2 = 0x00, coeff_3 = 0x00, coeff_4 = 0x80, \n\n\n\ncoeff_5 = 0x00, coeff_6 = 0x00, coeff_7 = 0x00, coeff_8 = 0x80 \n\n\n\n \n\n"]
pub mod ct_coeff;
#[doc = "GAMMA_OUT_MODE (rw) register accessor: gamma segmentation mode register for output gamma\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gamma_out_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_out_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gamma_out_mode`]
module"]
#[doc(alias = "GAMMA_OUT_MODE")]
pub type GammaOutMode = crate::Reg<gamma_out_mode::GammaOutModeSpec>;
#[doc = "gamma segmentation mode register for output gamma"]
pub mod gamma_out_mode;
#[doc = "GAMMA_OUT_Y (rw) register accessor: Gamma Out Curve definition y_ n (n=0..16)\n\nNote: Reset values generate a standard gamma of 2.2. Reset values are: \n\ny_00 = 0x000, y_01 = 0x049, y_02 = 0x089, y_03 = 0x0B7, y_04 = 0x0DF, y_05 = \n\n\n\n0x11F, y_06 = 0x154, y_07 = 0x183, y_08 = 0x1AD, y_09 = 0x1F6, y_10 = 0x235, y_11 = \n\n0x26F, y_12 = 0x2D3, y_13 = 0x32A, y_14 = 0x378, y_15 = 0x3BF, y_16 = 0x3FF \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gamma_out_y::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_out_y::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gamma_out_y`]
module"]
#[doc(alias = "GAMMA_OUT_Y")]
pub type GammaOutY = crate::Reg<gamma_out_y::GammaOutYSpec>;
#[doc = "Gamma Out Curve definition y_ n (n=0..16)\n\nNote: Reset values generate a standard gamma of 2.2. Reset values are: \n\ny_00 = 0x000, y_01 = 0x049, y_02 = 0x089, y_03 = 0x0B7, y_04 = 0x0DF, y_05 = \n\n\n\n0x11F, y_06 = 0x154, y_07 = 0x183, y_08 = 0x1AD, y_09 = 0x1F6, y_10 = 0x235, y_11 = \n\n0x26F, y_12 = 0x2D3, y_13 = 0x32A, y_14 = 0x378, y_15 = 0x3BF, y_16 = 0x3FF \n\n\n\n \n\n"]
pub mod gamma_out_y;
#[doc = "ERR (r) register accessor: ISP error register\n\nNote: For debug purposes the ISP_ERR und ISP_ERR_CLR are implemented. For the case \n\nwhen a PIC_SIZE_ERR interrupt is signaled the SW is able to see in which submodule this error \n\nis generated. Writing to the ISP_ERR_CLR register clears this bit. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err`]
module"]
#[doc(alias = "ERR")]
pub type Err = crate::Reg<err::ErrSpec>;
#[doc = "ISP error register\n\nNote: For debug purposes the ISP_ERR und ISP_ERR_CLR are implemented. For the case \n\nwhen a PIC_SIZE_ERR interrupt is signaled the SW is able to see in which submodule this error \n\nis generated. Writing to the ISP_ERR_CLR register clears this bit. \n\n\n\n \n\n"]
pub mod err;
#[doc = "ERR_CLR (w) register accessor: ISP error clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_clr`]
module"]
#[doc(alias = "ERR_CLR")]
pub type ErrClr = crate::Reg<err_clr::ErrClrSpec>;
#[doc = "ISP error clear register"]
pub mod err_clr;
#[doc = "FRAME_COUNT (r) register accessor: Frame counter\n\nNote: In the ISP_FRAME_COUNT register the number of processed frames are displayed. \n\nFor example: If a 8 is programmed into the ISP_ACQ_NR_FRAMES register, a read access to \n\nthe ISP_FRAME_COUNT register during processing of the first picture shows a 7. \n\n\n\nAfter the entire frames are processed the ISP_OFF interrupt is generated and the \n\nISP_FRAME_COUNT has the count zero. In case a '0' is programmed into the \n\nISP_ACQ_NR_FRAMES register (continues mode) the ISP_FRAME_COUNT register keeps the \n\nvalue '0'. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frame_count::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frame_count`]
module"]
#[doc(alias = "FRAME_COUNT")]
pub type FrameCount = crate::Reg<frame_count::FrameCountSpec>;
#[doc = "Frame counter\n\nNote: In the ISP_FRAME_COUNT register the number of processed frames are displayed. \n\nFor example: If a 8 is programmed into the ISP_ACQ_NR_FRAMES register, a read access to \n\nthe ISP_FRAME_COUNT register during processing of the first picture shows a 7. \n\n\n\nAfter the entire frames are processed the ISP_OFF interrupt is generated and the \n\nISP_FRAME_COUNT has the count zero. In case a '0' is programmed into the \n\nISP_ACQ_NR_FRAMES register (continues mode) the ISP_FRAME_COUNT register keeps the \n\nvalue '0'. \n\n\n\n \n\n"]
pub mod frame_count;
#[doc = "CT_OFFSET_R (rw) register accessor: cross-talk offset red\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ct_offset_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ct_offset_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ct_offset_r`]
module"]
#[doc(alias = "CT_OFFSET_R")]
pub type CtOffsetR = crate::Reg<ct_offset_r::CtOffsetRSpec>;
#[doc = "cross-talk offset red"]
pub mod ct_offset_r;
#[doc = "CT_OFFSET_G (rw) register accessor: cross-talk offset green\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ct_offset_g::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ct_offset_g::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ct_offset_g`]
module"]
#[doc(alias = "CT_OFFSET_G")]
pub type CtOffsetG = crate::Reg<ct_offset_g::CtOffsetGSpec>;
#[doc = "cross-talk offset green"]
pub mod ct_offset_g;
#[doc = "CT_OFFSET_B (rw) register accessor: cross-talk offset blue\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ct_offset_b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ct_offset_b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ct_offset_b`]
module"]
#[doc(alias = "CT_OFFSET_B")]
pub type CtOffsetB = crate::Reg<ct_offset_b::CtOffsetBSpec>;
#[doc = "cross-talk offset blue"]
pub mod ct_offset_b;
#[doc = "FLASH_CMD (w) register accessor: Flash command\n\nNote: This is the command register for flash light and prelight activation. If the 'rw' bits \n\n(e.g. 'fl_cap_del') are re-programmed during operation, the following scheme shall be \n\napplied: \n\n\n\nprelight is active (prelight_on = 1 has been set before): Every write access to this register \n\nshall use prelight_on = 1 (to prevent undesired switch off of the prelight). \n\n\n\nprelight is off: Every write access to this register shall use prelight_on = 0 (to prevent \n\nundesired switch on of the prelight). \n\n\n\n\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_cmd`]
module"]
#[doc(alias = "FLASH_CMD")]
pub type FlashCmd = crate::Reg<flash_cmd::FlashCmdSpec>;
#[doc = "Flash command\n\nNote: This is the command register for flash light and prelight activation. If the 'rw' bits \n\n(e.g. 'fl_cap_del') are re-programmed during operation, the following scheme shall be \n\napplied: \n\n\n\nprelight is active (prelight_on = 1 has been set before): Every write access to this register \n\nshall use prelight_on = 1 (to prevent undesired switch off of the prelight). \n\n\n\nprelight is off: Every write access to this register shall use prelight_on = 0 (to prevent \n\nundesired switch on of the prelight). \n\n\n\n"]
pub mod flash_cmd;
#[doc = "FLASH_CONFIG (rw) register accessor: Flash config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_config`]
module"]
#[doc(alias = "FLASH_CONFIG")]
pub type FlashConfig = crate::Reg<flash_config::FlashConfigSpec>;
#[doc = "Flash config"]
pub mod flash_config;
#[doc = "FLASH_PREDIV (rw) register accessor: Flash Counter Pre-Divider\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_prediv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_prediv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prediv`]
module"]
#[doc(alias = "FLASH_PREDIV")]
pub type FlashPrediv = crate::Reg<flash_prediv::FlashPredivSpec>;
#[doc = "Flash Counter Pre-Divider"]
pub mod flash_prediv;
#[doc = "FLASH_DELAY (rw) register accessor: Flash Delay\n\nNote: Example: \n\nfl_delay = (10s * 100MHz) / (1023 + 1) – 1 = 976561 \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_delay::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_delay::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_delay`]
module"]
#[doc(alias = "FLASH_DELAY")]
pub type FlashDelay = crate::Reg<flash_delay::FlashDelaySpec>;
#[doc = "Flash Delay\n\nNote: Example: \n\nfl_delay = (10s * 100MHz) / (1023 + 1) – 1 = 976561 \n\n\n\n \n\n"]
pub mod flash_delay;
#[doc = "FLASH_TIME (rw) register accessor: Flash time\n\nNote: Example: \n\nfl_time = (500ms * 100MHz) / (700 + 1) – 1 = 71530 \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_time`]
module"]
#[doc(alias = "FLASH_TIME")]
pub type FlashTime = crate::Reg<flash_time::FlashTimeSpec>;
#[doc = "Flash time\n\nNote: Example: \n\nfl_time = (500ms * 100MHz) / (700 + 1) – 1 = 71530 \n\n\n\n \n\n"]
pub mod flash_time;
#[doc = "FLASH_MAXP (rw) register accessor: Maximum value for flash or preflash\n\nNote: Example: \n\n\n\nfl_maxp = (10s * 100MHz) / (16384) – 1 = 61034 \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_maxp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_maxp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_maxp`]
module"]
#[doc(alias = "FLASH_MAXP")]
pub type FlashMaxp = crate::Reg<flash_maxp::FlashMaxpSpec>;
#[doc = "Maximum value for flash or preflash\n\nNote: Example: \n\n\n\nfl_maxp = (10s * 100MHz) / (16384) – 1 = 61034 \n\n\n\n"]
pub mod flash_maxp;
#[doc = "SH_CTRL (rw) register accessor: mechanical shutter control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sh_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sh_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sh_ctrl`]
module"]
#[doc(alias = "SH_CTRL")]
pub type ShCtrl = crate::Reg<sh_ctrl::ShCtrlSpec>;
#[doc = "mechanical shutter control"]
pub mod sh_ctrl;
#[doc = "SH_PREDIV (rw) register accessor: Mech. Shutter Counter Pre-Divider\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sh_prediv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sh_prediv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sh_prediv`]
module"]
#[doc(alias = "SH_PREDIV")]
pub type ShPrediv = crate::Reg<sh_prediv::ShPredivSpec>;
#[doc = "Mech. Shutter Counter Pre-Divider"]
pub mod sh_prediv;
#[doc = "SH_DELAY (rw) register accessor: Delay register\n\nNote: Example: \n\nsh_delay = (250us * 100MHz) / (50 + 1) – 1 = 489 \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sh_delay::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sh_delay::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sh_delay`]
module"]
#[doc(alias = "SH_DELAY")]
pub type ShDelay = crate::Reg<sh_delay::ShDelaySpec>;
#[doc = "Delay register\n\nNote: Example: \n\nsh_delay = (250us * 100MHz) / (50 + 1) – 1 = 489 \n\n\n\n \n\n"]
pub mod sh_delay;
#[doc = "SH_TIME (rw) register accessor: Time register\n\nNote: Example: \n\n\n\nsh_time = (10s * 100MHz) / (1023 + 1) – 1 = 976561 \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sh_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sh_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sh_time`]
module"]
#[doc(alias = "SH_TIME")]
pub type ShTime = crate::Reg<sh_time::ShTimeSpec>;
#[doc = "Time register\n\nNote: Example: \n\n\n\nsh_time = (10s * 100MHz) / (1023 + 1) – 1 = 976561 \n\n\n\n \n\n"]
pub mod sh_time;
#[doc = "CPROC_CTRL (rw) register accessor: Global control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cproc_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cproc_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cproc_ctrl`]
module"]
#[doc(alias = "CPROC_CTRL")]
pub type CprocCtrl = crate::Reg<cproc_ctrl::CprocCtrlSpec>;
#[doc = "Global control register"]
pub mod cproc_ctrl;
#[doc = "CPROC_CONTRAST (rw) register accessor: Color Processing contrast register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cproc_contrast::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cproc_contrast::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cproc_contrast`]
module"]
#[doc(alias = "CPROC_CONTRAST")]
pub type CprocContrast = crate::Reg<cproc_contrast::CprocContrastSpec>;
#[doc = "Color Processing contrast register"]
pub mod cproc_contrast;
#[doc = "CPROC_BRIGHTNESS (rw) register accessor: Color Processing brightness register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cproc_brightness::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cproc_brightness::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cproc_brightness`]
module"]
#[doc(alias = "CPROC_BRIGHTNESS")]
pub type CprocBrightness = crate::Reg<cproc_brightness::CprocBrightnessSpec>;
#[doc = "Color Processing brightness register"]
pub mod cproc_brightness;
#[doc = "CPROC_SATURATION (rw) register accessor: Color Processing saturation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cproc_saturation::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cproc_saturation::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cproc_saturation`]
module"]
#[doc(alias = "CPROC_SATURATION")]
pub type CprocSaturation = crate::Reg<cproc_saturation::CprocSaturationSpec>;
#[doc = "Color Processing saturation register"]
pub mod cproc_saturation;
#[doc = "CPROC_HUE (rw) register accessor: Color Processing hue register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cproc_hue::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cproc_hue::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cproc_hue`]
module"]
#[doc(alias = "CPROC_HUE")]
pub type CprocHue = crate::Reg<cproc_hue::CprocHueSpec>;
#[doc = "Color Processing hue register"]
pub mod cproc_hue;
#[doc = "MRSZ_CTRL (rw) register accessor: global control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mrsz_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrsz_ctrl`]
module"]
#[doc(alias = "MRSZ_CTRL")]
pub type MrszCtrl = crate::Reg<mrsz_ctrl::MrszCtrlSpec>;
#[doc = "global control register"]
pub mod mrsz_ctrl;
#[doc = "MRSZ_SCALE_HY (rw) register accessor: horizontal luminance scale factor register\n\nNote: The size of the output picture is calculated as follows: \n\n\n\n \n\nupscaling: (size_in - 1) / (size_out - 1)) = scale downscaling: (size_out - 1) / (size_in - 1)) \n\n\n\n= scale, \n\n \n\nwhere size_in/out is the width or height of the in/output picture. The value of the \n\n\n\nrespective MRSZ_SCALE register then has to be \n\nint(scale x 2^14) for upscaling and \n\nint(scale x 2^14)+1 for downscaling. \n\nFor downscaling this formula has no restriction. In upscaling processes the limit is factor 5. \n\n\n\nThe output is at max. 5 MegaPixel. \n\nIf a format conversion is performed, the scale factors have to be different for the \n\n\n\nluminance and the chrominance component, respectively. For example, for a format \n\nconversion from 4:2:2 to 4:2:0 the scale register value for the vertical \n\n\n\nchrominance component should be half of the vertical luminance scale register value. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_scale_hy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mrsz_scale_hy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrsz_scale_hy`]
module"]
#[doc(alias = "MRSZ_SCALE_HY")]
pub type MrszScaleHy = crate::Reg<mrsz_scale_hy::MrszScaleHySpec>;
#[doc = "horizontal luminance scale factor register\n\nNote: The size of the output picture is calculated as follows: \n\n\n\n \n\nupscaling: (size_in - 1) / (size_out - 1)) = scale downscaling: (size_out - 1) / (size_in - 1)) \n\n\n\n= scale, \n\n \n\nwhere size_in/out is the width or height of the in/output picture. The value of the \n\n\n\nrespective MRSZ_SCALE register then has to be \n\nint(scale x 2^14) for upscaling and \n\nint(scale x 2^14)+1 for downscaling. \n\nFor downscaling this formula has no restriction. In upscaling processes the limit is factor 5. \n\n\n\nThe output is at max. 5 MegaPixel. \n\nIf a format conversion is performed, the scale factors have to be different for the \n\n\n\nluminance and the chrominance component, respectively. For example, for a format \n\nconversion from 4:2:2 to 4:2:0 the scale register value for the vertical \n\n\n\nchrominance component should be half of the vertical luminance scale register value. \n\n\n\n \n\n"]
pub mod mrsz_scale_hy;
#[doc = "MRSZ_SCALE_HCB (rw) register accessor: horizontal Cb scale factor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_scale_hcb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mrsz_scale_hcb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrsz_scale_hcb`]
module"]
#[doc(alias = "MRSZ_SCALE_HCB")]
pub type MrszScaleHcb = crate::Reg<mrsz_scale_hcb::MrszScaleHcbSpec>;
#[doc = "horizontal Cb scale factor register"]
pub mod mrsz_scale_hcb;
#[doc = "MRSZ_SCALE_HCR (rw) register accessor: horizontal Cr scale factor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_scale_hcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mrsz_scale_hcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrsz_scale_hcr`]
module"]
#[doc(alias = "MRSZ_SCALE_HCR")]
pub type MrszScaleHcr = crate::Reg<mrsz_scale_hcr::MrszScaleHcrSpec>;
#[doc = "horizontal Cr scale factor register"]
pub mod mrsz_scale_hcr;
#[doc = "MRSZ_SCALE_VY (rw) register accessor: vertical luminance scale factor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_scale_vy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mrsz_scale_vy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrsz_scale_vy`]
module"]
#[doc(alias = "MRSZ_SCALE_VY")]
pub type MrszScaleVy = crate::Reg<mrsz_scale_vy::MrszScaleVySpec>;
#[doc = "vertical luminance scale factor register"]
pub mod mrsz_scale_vy;
#[doc = "MRSZ_SCALE_VC (rw) register accessor: vertical chrominance scale factor register\n\nNote: The size of the output picture is calculated as follows: (size_out - 1) / (size_in - 1)) \n\n\n\n= scale, \n\nwhere size_in/out is the width or heigth of the in/output picture. The values of the \n\n\n\nMRSZ_SCALE registers then have to be int(scale x 2^14)+1 \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_scale_vc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mrsz_scale_vc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrsz_scale_vc`]
module"]
#[doc(alias = "MRSZ_SCALE_VC")]
pub type MrszScaleVc = crate::Reg<mrsz_scale_vc::MrszScaleVcSpec>;
#[doc = "vertical chrominance scale factor register\n\nNote: The size of the output picture is calculated as follows: (size_out - 1) / (size_in - 1)) \n\n\n\n= scale, \n\nwhere size_in/out is the width or heigth of the in/output picture. The values of the \n\n\n\nMRSZ_SCALE registers then have to be int(scale x 2^14)+1 \n\n\n\n \n\n"]
pub mod mrsz_scale_vc;
#[doc = "MRSZ_PHASE_HY (rw) register accessor: horizontal luminance phase register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_phase_hy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mrsz_phase_hy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrsz_phase_hy`]
module"]
#[doc(alias = "MRSZ_PHASE_HY")]
pub type MrszPhaseHy = crate::Reg<mrsz_phase_hy::MrszPhaseHySpec>;
#[doc = "horizontal luminance phase register"]
pub mod mrsz_phase_hy;
#[doc = "MRSZ_PHASE_HC (rw) register accessor: horizontal chrominance phase register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_phase_hc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mrsz_phase_hc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrsz_phase_hc`]
module"]
#[doc(alias = "MRSZ_PHASE_HC")]
pub type MrszPhaseHc = crate::Reg<mrsz_phase_hc::MrszPhaseHcSpec>;
#[doc = "horizontal chrominance phase register"]
pub mod mrsz_phase_hc;
#[doc = "MRSZ_PHASE_VY (rw) register accessor: vertical luminance phase register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_phase_vy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mrsz_phase_vy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrsz_phase_vy`]
module"]
#[doc(alias = "MRSZ_PHASE_VY")]
pub type MrszPhaseVy = crate::Reg<mrsz_phase_vy::MrszPhaseVySpec>;
#[doc = "vertical luminance phase register"]
pub mod mrsz_phase_vy;
#[doc = "MRSZ_PHASE_VC (rw) register accessor: vertical chrominance phase register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_phase_vc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mrsz_phase_vc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrsz_phase_vc`]
module"]
#[doc(alias = "MRSZ_PHASE_VC")]
pub type MrszPhaseVc = crate::Reg<mrsz_phase_vc::MrszPhaseVcSpec>;
#[doc = "vertical chrominance phase register"]
pub mod mrsz_phase_vc;
#[doc = "MRSZ_SCALE_LUT_ADDR (rw) register accessor: Address pointer of up-scaling look up table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_scale_lut_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mrsz_scale_lut_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrsz_scale_lut_addr`]
module"]
#[doc(alias = "MRSZ_SCALE_LUT_ADDR")]
pub type MrszScaleLutAddr = crate::Reg<mrsz_scale_lut_addr::MrszScaleLutAddrSpec>;
#[doc = "Address pointer of up-scaling look up table"]
pub mod mrsz_scale_lut_addr;
#[doc = "MRSZ_SCALE_LUT (rw) register accessor: Entry of up-scaling look up table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_scale_lut::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mrsz_scale_lut::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrsz_scale_lut`]
module"]
#[doc(alias = "MRSZ_SCALE_LUT")]
pub type MrszScaleLut = crate::Reg<mrsz_scale_lut::MrszScaleLutSpec>;
#[doc = "Entry of up-scaling look up table"]
pub mod mrsz_scale_lut;
#[doc = "MRSZ_CTRL_SHD (r) register accessor: global control shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_ctrl_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrsz_ctrl_shd`]
module"]
#[doc(alias = "MRSZ_CTRL_SHD")]
pub type MrszCtrlShd = crate::Reg<mrsz_ctrl_shd::MrszCtrlShdSpec>;
#[doc = "global control shadow register"]
pub mod mrsz_ctrl_shd;
#[doc = "MRSZ_SCALE_HY_SHD (r) register accessor: horizontal luminance scale factor shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_scale_hy_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrsz_scale_hy_shd`]
module"]
#[doc(alias = "MRSZ_SCALE_HY_SHD")]
pub type MrszScaleHyShd = crate::Reg<mrsz_scale_hy_shd::MrszScaleHyShdSpec>;
#[doc = "horizontal luminance scale factor shadow register"]
pub mod mrsz_scale_hy_shd;
#[doc = "MRSZ_SCALE_HCB_SHD (r) register accessor: horizontal Cb scale factor shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_scale_hcb_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrsz_scale_hcb_shd`]
module"]
#[doc(alias = "MRSZ_SCALE_HCB_SHD")]
pub type MrszScaleHcbShd = crate::Reg<mrsz_scale_hcb_shd::MrszScaleHcbShdSpec>;
#[doc = "horizontal Cb scale factor shadow register"]
pub mod mrsz_scale_hcb_shd;
#[doc = "MRSZ_SCALE_HCR_SHD (r) register accessor: horizontal Cr scale factor shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_scale_hcr_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrsz_scale_hcr_shd`]
module"]
#[doc(alias = "MRSZ_SCALE_HCR_SHD")]
pub type MrszScaleHcrShd = crate::Reg<mrsz_scale_hcr_shd::MrszScaleHcrShdSpec>;
#[doc = "horizontal Cr scale factor shadow register"]
pub mod mrsz_scale_hcr_shd;
#[doc = "MRSZ_SCALE_VY_SHD (r) register accessor: vertical luminance scale factor shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_scale_vy_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrsz_scale_vy_shd`]
module"]
#[doc(alias = "MRSZ_SCALE_VY_SHD")]
pub type MrszScaleVyShd = crate::Reg<mrsz_scale_vy_shd::MrszScaleVyShdSpec>;
#[doc = "vertical luminance scale factor shadow register"]
pub mod mrsz_scale_vy_shd;
#[doc = "MRSZ_SCALE_VC_SHD (r) register accessor: vertical chrominance scale factor shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_scale_vc_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrsz_scale_vc_shd`]
module"]
#[doc(alias = "MRSZ_SCALE_VC_SHD")]
pub type MrszScaleVcShd = crate::Reg<mrsz_scale_vc_shd::MrszScaleVcShdSpec>;
#[doc = "vertical chrominance scale factor shadow register"]
pub mod mrsz_scale_vc_shd;
#[doc = "MRSZ_PHASE_HY_SHD (r) register accessor: horizontal luminance phase shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_phase_hy_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrsz_phase_hy_shd`]
module"]
#[doc(alias = "MRSZ_PHASE_HY_SHD")]
pub type MrszPhaseHyShd = crate::Reg<mrsz_phase_hy_shd::MrszPhaseHyShdSpec>;
#[doc = "horizontal luminance phase shadow register"]
pub mod mrsz_phase_hy_shd;
#[doc = "MRSZ_PHASE_HC_SHD (r) register accessor: horizontal chrominance phase shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_phase_hc_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrsz_phase_hc_shd`]
module"]
#[doc(alias = "MRSZ_PHASE_HC_SHD")]
pub type MrszPhaseHcShd = crate::Reg<mrsz_phase_hc_shd::MrszPhaseHcShdSpec>;
#[doc = "horizontal chrominance phase shadow register"]
pub mod mrsz_phase_hc_shd;
#[doc = "MRSZ_PHASE_VY_SHD (r) register accessor: vertical luminance phase shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_phase_vy_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrsz_phase_vy_shd`]
module"]
#[doc(alias = "MRSZ_PHASE_VY_SHD")]
pub type MrszPhaseVyShd = crate::Reg<mrsz_phase_vy_shd::MrszPhaseVyShdSpec>;
#[doc = "vertical luminance phase shadow register"]
pub mod mrsz_phase_vy_shd;
#[doc = "MRSZ_PHASE_VC_SHD (r) register accessor: vertical chrominance phase shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_phase_vc_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrsz_phase_vc_shd`]
module"]
#[doc(alias = "MRSZ_PHASE_VC_SHD")]
pub type MrszPhaseVcShd = crate::Reg<mrsz_phase_vc_shd::MrszPhaseVcShdSpec>;
#[doc = "vertical chrominance phase shadow register"]
pub mod mrsz_phase_vc_shd;
#[doc = "SRSZ_CTRL (rw) register accessor: global control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srsz_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srsz_ctrl`]
module"]
#[doc(alias = "SRSZ_CTRL")]
pub type SrszCtrl = crate::Reg<srsz_ctrl::SrszCtrlSpec>;
#[doc = "global control register"]
pub mod srsz_ctrl;
#[doc = "SRSZ_SCALE_HY (rw) register accessor: horizontal luminance scale factor register\n\nNote: The size of the output picture is calculated as follows: \n\nupscaling: (size_in - 1) / (size_out - 1)) = scale downscaling: (size_out - 1) / (size_in - 1)) \n\n\n\n= scale, \n\nwhere size_in/out is the width or height of the in/output picture. The value of the \n\n\n\nrespective SRSZ_SCALE register then has to be \n\nint(scale x 2^14) for upscaling and \n\nint(scale x 2^14)+1 for downscaling. \n\nFor downscaling this formula has no restriction. In upscaling processes the limit is factor 5. \n\n\n\nIf a format conversion is performed, the scale factors have to be different for the luminance \n\nand the chrominance component, respectively. For example, for a \n\n\n\nformat conversion from 4:2:2 to 4:2:0 the scale register value for the vertical \n\nchrominance component should be half of the vertical luminance scale register value. \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_scale_hy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srsz_scale_hy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srsz_scale_hy`]
module"]
#[doc(alias = "SRSZ_SCALE_HY")]
pub type SrszScaleHy = crate::Reg<srsz_scale_hy::SrszScaleHySpec>;
#[doc = "horizontal luminance scale factor register\n\nNote: The size of the output picture is calculated as follows: \n\nupscaling: (size_in - 1) / (size_out - 1)) = scale downscaling: (size_out - 1) / (size_in - 1)) \n\n\n\n= scale, \n\nwhere size_in/out is the width or height of the in/output picture. The value of the \n\n\n\nrespective SRSZ_SCALE register then has to be \n\nint(scale x 2^14) for upscaling and \n\nint(scale x 2^14)+1 for downscaling. \n\nFor downscaling this formula has no restriction. In upscaling processes the limit is factor 5. \n\n\n\nIf a format conversion is performed, the scale factors have to be different for the luminance \n\nand the chrominance component, respectively. For example, for a \n\n\n\nformat conversion from 4:2:2 to 4:2:0 the scale register value for the vertical \n\nchrominance component should be half of the vertical luminance scale register value. \n\n\n\n"]
pub mod srsz_scale_hy;
#[doc = "SRSZ_SCALE_HCB (rw) register accessor: horizontal chrominance scale factor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_scale_hcb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srsz_scale_hcb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srsz_scale_hcb`]
module"]
#[doc(alias = "SRSZ_SCALE_HCB")]
pub type SrszScaleHcb = crate::Reg<srsz_scale_hcb::SrszScaleHcbSpec>;
#[doc = "horizontal chrominance scale factor register"]
pub mod srsz_scale_hcb;
#[doc = "SRSZ_SCALE_HCR (rw) register accessor: horizontal chrominance scale factor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_scale_hcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srsz_scale_hcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srsz_scale_hcr`]
module"]
#[doc(alias = "SRSZ_SCALE_HCR")]
pub type SrszScaleHcr = crate::Reg<srsz_scale_hcr::SrszScaleHcrSpec>;
#[doc = "horizontal chrominance scale factor register"]
pub mod srsz_scale_hcr;
#[doc = "SRSZ_SCALE_VY (rw) register accessor: vertical luminance scale factor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_scale_vy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srsz_scale_vy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srsz_scale_vy`]
module"]
#[doc(alias = "SRSZ_SCALE_VY")]
pub type SrszScaleVy = crate::Reg<srsz_scale_vy::SrszScaleVySpec>;
#[doc = "vertical luminance scale factor register"]
pub mod srsz_scale_vy;
#[doc = "SRSZ_SCALE_VC (rw) register accessor: vertical chrominance scale factor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_scale_vc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srsz_scale_vc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srsz_scale_vc`]
module"]
#[doc(alias = "SRSZ_SCALE_VC")]
pub type SrszScaleVc = crate::Reg<srsz_scale_vc::SrszScaleVcSpec>;
#[doc = "vertical chrominance scale factor register"]
pub mod srsz_scale_vc;
#[doc = "SRSZ_PHASE_HY (rw) register accessor: horizontal luminance phase register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_phase_hy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srsz_phase_hy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srsz_phase_hy`]
module"]
#[doc(alias = "SRSZ_PHASE_HY")]
pub type SrszPhaseHy = crate::Reg<srsz_phase_hy::SrszPhaseHySpec>;
#[doc = "horizontal luminance phase register"]
pub mod srsz_phase_hy;
#[doc = "SRSZ_PHASE_HC (rw) register accessor: horizontal chrominance phase register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_phase_hc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srsz_phase_hc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srsz_phase_hc`]
module"]
#[doc(alias = "SRSZ_PHASE_HC")]
pub type SrszPhaseHc = crate::Reg<srsz_phase_hc::SrszPhaseHcSpec>;
#[doc = "horizontal chrominance phase register"]
pub mod srsz_phase_hc;
#[doc = "SRSZ_PHASE_VY (rw) register accessor: vertical luminance phase register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_phase_vy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srsz_phase_vy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srsz_phase_vy`]
module"]
#[doc(alias = "SRSZ_PHASE_VY")]
pub type SrszPhaseVy = crate::Reg<srsz_phase_vy::SrszPhaseVySpec>;
#[doc = "vertical luminance phase register"]
pub mod srsz_phase_vy;
#[doc = "SRSZ_PHASE_VC (rw) register accessor: vertical chrominance phase register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_phase_vc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srsz_phase_vc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srsz_phase_vc`]
module"]
#[doc(alias = "SRSZ_PHASE_VC")]
pub type SrszPhaseVc = crate::Reg<srsz_phase_vc::SrszPhaseVcSpec>;
#[doc = "vertical chrominance phase register"]
pub mod srsz_phase_vc;
#[doc = "SRSZ_SCALE_LUT_ADDR (rw) register accessor: Address pointer of up-scaling look up table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_scale_lut_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srsz_scale_lut_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srsz_scale_lut_addr`]
module"]
#[doc(alias = "SRSZ_SCALE_LUT_ADDR")]
pub type SrszScaleLutAddr = crate::Reg<srsz_scale_lut_addr::SrszScaleLutAddrSpec>;
#[doc = "Address pointer of up-scaling look up table"]
pub mod srsz_scale_lut_addr;
#[doc = "SRSZ_SCALE_LUT (rw) register accessor: Entry of up-scaling look up table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_scale_lut::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srsz_scale_lut::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srsz_scale_lut`]
module"]
#[doc(alias = "SRSZ_SCALE_LUT")]
pub type SrszScaleLut = crate::Reg<srsz_scale_lut::SrszScaleLutSpec>;
#[doc = "Entry of up-scaling look up table"]
pub mod srsz_scale_lut;
#[doc = "SRSZ_CTRL_SHD (r) register accessor: global control shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_ctrl_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srsz_ctrl_shd`]
module"]
#[doc(alias = "SRSZ_CTRL_SHD")]
pub type SrszCtrlShd = crate::Reg<srsz_ctrl_shd::SrszCtrlShdSpec>;
#[doc = "global control shadow register"]
pub mod srsz_ctrl_shd;
#[doc = "SRSZ_SCALE_HY_SHD (r) register accessor: horizontal luminance scale factor shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_scale_hy_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srsz_scale_hy_shd`]
module"]
#[doc(alias = "SRSZ_SCALE_HY_SHD")]
pub type SrszScaleHyShd = crate::Reg<srsz_scale_hy_shd::SrszScaleHyShdSpec>;
#[doc = "horizontal luminance scale factor shadow register"]
pub mod srsz_scale_hy_shd;
#[doc = "SRSZ_SCALE_HCB_SHD (r) register accessor: horizontal Cb scale factor shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_scale_hcb_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srsz_scale_hcb_shd`]
module"]
#[doc(alias = "SRSZ_SCALE_HCB_SHD")]
pub type SrszScaleHcbShd = crate::Reg<srsz_scale_hcb_shd::SrszScaleHcbShdSpec>;
#[doc = "horizontal Cb scale factor shadow register"]
pub mod srsz_scale_hcb_shd;
#[doc = "SRSZ_SCALE_HCR_SHD (r) register accessor: horizontal Cr scale factor shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_scale_hcr_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srsz_scale_hcr_shd`]
module"]
#[doc(alias = "SRSZ_SCALE_HCR_SHD")]
pub type SrszScaleHcrShd = crate::Reg<srsz_scale_hcr_shd::SrszScaleHcrShdSpec>;
#[doc = "horizontal Cr scale factor shadow register"]
pub mod srsz_scale_hcr_shd;
#[doc = "SRSZ_SCALE_VY_SHD (r) register accessor: vertical luminance scale factor shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_scale_vy_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srsz_scale_vy_shd`]
module"]
#[doc(alias = "SRSZ_SCALE_VY_SHD")]
pub type SrszScaleVyShd = crate::Reg<srsz_scale_vy_shd::SrszScaleVyShdSpec>;
#[doc = "vertical luminance scale factor shadow register"]
pub mod srsz_scale_vy_shd;
#[doc = "SRSZ_SCALE_VC_SHD (r) register accessor: vertical chrominance scale factor shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_scale_vc_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srsz_scale_vc_shd`]
module"]
#[doc(alias = "SRSZ_SCALE_VC_SHD")]
pub type SrszScaleVcShd = crate::Reg<srsz_scale_vc_shd::SrszScaleVcShdSpec>;
#[doc = "vertical chrominance scale factor shadow register"]
pub mod srsz_scale_vc_shd;
#[doc = "SRSZ_PHASE_HY_SHD (r) register accessor: horizontal luminance phase shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_phase_hy_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srsz_phase_hy_shd`]
module"]
#[doc(alias = "SRSZ_PHASE_HY_SHD")]
pub type SrszPhaseHyShd = crate::Reg<srsz_phase_hy_shd::SrszPhaseHyShdSpec>;
#[doc = "horizontal luminance phase shadow register"]
pub mod srsz_phase_hy_shd;
#[doc = "SRSZ_PHASE_HC_SHD (r) register accessor: horizontal chrominance phase shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_phase_hc_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srsz_phase_hc_shd`]
module"]
#[doc(alias = "SRSZ_PHASE_HC_SHD")]
pub type SrszPhaseHcShd = crate::Reg<srsz_phase_hc_shd::SrszPhaseHcShdSpec>;
#[doc = "horizontal chrominance phase shadow register"]
pub mod srsz_phase_hc_shd;
#[doc = "SRSZ_PHASE_VY_SHD (r) register accessor: vertical luminance phase shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_phase_vy_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srsz_phase_vy_shd`]
module"]
#[doc(alias = "SRSZ_PHASE_VY_SHD")]
pub type SrszPhaseVyShd = crate::Reg<srsz_phase_vy_shd::SrszPhaseVyShdSpec>;
#[doc = "vertical luminance phase shadow register"]
pub mod srsz_phase_vy_shd;
#[doc = "SRSZ_PHASE_VC_SHD (r) register accessor: vertical chrominance phase shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_phase_vc_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srsz_phase_vc_shd`]
module"]
#[doc(alias = "SRSZ_PHASE_VC_SHD")]
pub type SrszPhaseVcShd = crate::Reg<srsz_phase_vc_shd::SrszPhaseVcShdSpec>;
#[doc = "vertical chrominance phase shadow register"]
pub mod srsz_phase_vc_shd;
#[doc = "MI_CTRL (rw) register accessor: Global control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_ctrl`]
module"]
#[doc(alias = "MI_CTRL")]
pub type MiCtrl = crate::Reg<mi_ctrl::MiCtrlSpec>;
#[doc = "Global control register"]
pub mod mi_ctrl;
#[doc = "MI_INIT (w) register accessor: Control register for address init and skip function\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_init::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_init`]
module"]
#[doc(alias = "MI_INIT")]
pub type MiInit = crate::Reg<mi_init::MiInitSpec>;
#[doc = "Control register for address init and skip function"]
pub mod mi_init;
#[doc = "MI_MP_Y_BASE_AD_INIT (rw) register accessor: Base address for main picture Y component, JPEG or raw data\n\nNote: This register protects from non-aligned access. The bits 0 to 2 are hard wired to \n\n'000'. As a consequence any byte address that is written to the register will automatically be \n\nre-mapped to the next lower 64 bit aligned address: write(MI_MP_Y_BASE_AD_INIT, \n\naddress_value) is equivalent to write(MI_Y_BASE_AD_INIT, address_value &amp; 0xFFFFFFF8). \n\nAnyhow, in order to avoid confusion it is NOT recommended to use non-aligned address values \n\nfor access. It is also NOT recommended to actively consider the register slice for register \n\naccess in order to avoid unneccessary mask and shift operations. \n\n\n\nIn addition, if ISP provides AXI interfaces the programmed base address shall be \n\n\n\nburst aligned with respect to the burst length configured in MI_CTRL . \n\n\n\nSet control bit init_base_en before updating so that a forced or automatic update can take \n\n\n\neffect. \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_y_base_ad_init::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_mp_y_base_ad_init::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_mp_y_base_ad_init`]
module"]
#[doc(alias = "MI_MP_Y_BASE_AD_INIT")]
pub type MiMpYBaseAdInit = crate::Reg<mi_mp_y_base_ad_init::MiMpYBaseAdInitSpec>;
#[doc = "Base address for main picture Y component, JPEG or raw data\n\nNote: This register protects from non-aligned access. The bits 0 to 2 are hard wired to \n\n'000'. As a consequence any byte address that is written to the register will automatically be \n\nre-mapped to the next lower 64 bit aligned address: write(MI_MP_Y_BASE_AD_INIT, \n\naddress_value) is equivalent to write(MI_Y_BASE_AD_INIT, address_value &amp; 0xFFFFFFF8). \n\nAnyhow, in order to avoid confusion it is NOT recommended to use non-aligned address values \n\nfor access. It is also NOT recommended to actively consider the register slice for register \n\naccess in order to avoid unneccessary mask and shift operations. \n\n\n\nIn addition, if ISP provides AXI interfaces the programmed base address shall be \n\n\n\nburst aligned with respect to the burst length configured in MI_CTRL . \n\n\n\nSet control bit init_base_en before updating so that a forced or automatic update can take \n\n\n\neffect. \n\n"]
pub mod mi_mp_y_base_ad_init;
#[doc = "MI_MP_Y_SIZE_INIT (rw) register accessor: Size of main picture Y component, JPEG or raw data\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nSet control bit init_base_en before updating so that a forced or automatic update can take \n\neffect. \n\n\n\n \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_y_size_init::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_mp_y_size_init::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_mp_y_size_init`]
module"]
#[doc(alias = "MI_MP_Y_SIZE_INIT")]
pub type MiMpYSizeInit = crate::Reg<mi_mp_y_size_init::MiMpYSizeInitSpec>;
#[doc = "Size of main picture Y component, JPEG or raw data\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nSet control bit init_base_en before updating so that a forced or automatic update can take \n\neffect. \n\n\n\n \n\n\n\n"]
pub mod mi_mp_y_size_init;
#[doc = "MI_MP_Y_OFFS_CNT_INIT (rw) register accessor: Offset counter init value for main picture Y, JPEG or raw data\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nSet control bit init_base_en before updating so that a forced or automatic update can \n\n\n\ntake effect. Check exceptional handling in skip modes. \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_y_offs_cnt_init::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_mp_y_offs_cnt_init::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_mp_y_offs_cnt_init`]
module"]
#[doc(alias = "MI_MP_Y_OFFS_CNT_INIT")]
pub type MiMpYOffsCntInit = crate::Reg<mi_mp_y_offs_cnt_init::MiMpYOffsCntInitSpec>;
#[doc = "Offset counter init value for main picture Y, JPEG or raw data\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nSet control bit init_base_en before updating so that a forced or automatic update can \n\n\n\ntake effect. Check exceptional handling in skip modes. \n\n\n\n"]
pub mod mi_mp_y_offs_cnt_init;
#[doc = "MI_MP_Y_OFFS_CNT_START (r) register accessor: Offset counter start value for main picture Y, JPEG or raw data\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_y_offs_cnt_start::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_mp_y_offs_cnt_start`]
module"]
#[doc(alias = "MI_MP_Y_OFFS_CNT_START")]
pub type MiMpYOffsCntStart = crate::Reg<mi_mp_y_offs_cnt_start::MiMpYOffsCntStartSpec>;
#[doc = "Offset counter start value for main picture Y, JPEG or raw data\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n"]
pub mod mi_mp_y_offs_cnt_start;
#[doc = "MI_MP_Y_IRQ_OFFS_INIT (rw) register accessor: Fill level interrupt offset value for main picture Y, JPEG or raw data\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_y_irq_offs_init::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_mp_y_irq_offs_init::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_mp_y_irq_offs_init`]
module"]
#[doc(alias = "MI_MP_Y_IRQ_OFFS_INIT")]
pub type MiMpYIrqOffsInit = crate::Reg<mi_mp_y_irq_offs_init::MiMpYIrqOffsInitSpec>;
#[doc = "Fill level interrupt offset value for main picture Y, JPEG or raw data\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n"]
pub mod mi_mp_y_irq_offs_init;
#[doc = "MI_MP_CB_BASE_AD_INIT (rw) register accessor: Base address for main picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_cb_base_ad_init::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_mp_cb_base_ad_init::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_mp_cb_base_ad_init`]
module"]
#[doc(alias = "MI_MP_CB_BASE_AD_INIT")]
pub type MiMpCbBaseAdInit = crate::Reg<mi_mp_cb_base_ad_init::MiMpCbBaseAdInitSpec>;
#[doc = "Base address for main picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n"]
pub mod mi_mp_cb_base_ad_init;
#[doc = "MI_MP_CB_SIZE_INIT (rw) register accessor: Size of main picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_cb_size_init::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_mp_cb_size_init::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_mp_cb_size_init`]
module"]
#[doc(alias = "MI_MP_CB_SIZE_INIT")]
pub type MiMpCbSizeInit = crate::Reg<mi_mp_cb_size_init::MiMpCbSizeInitSpec>;
#[doc = "Size of main picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n"]
pub mod mi_mp_cb_size_init;
#[doc = "MI_MP_CB_OFFS_CNT_INIT (rw) register accessor: Offset counter init value for main picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_cb_offs_cnt_init::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_mp_cb_offs_cnt_init::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_mp_cb_offs_cnt_init`]
module"]
#[doc(alias = "MI_MP_CB_OFFS_CNT_INIT")]
pub type MiMpCbOffsCntInit = crate::Reg<mi_mp_cb_offs_cnt_init::MiMpCbOffsCntInitSpec>;
#[doc = "Offset counter init value for main picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n"]
pub mod mi_mp_cb_offs_cnt_init;
#[doc = "MI_MP_CB_OFFS_CNT_START (r) register accessor: Offset counter start value for main picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_cb_offs_cnt_start::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_mp_cb_offs_cnt_start`]
module"]
#[doc(alias = "MI_MP_CB_OFFS_CNT_START")]
pub type MiMpCbOffsCntStart = crate::Reg<mi_mp_cb_offs_cnt_start::MiMpCbOffsCntStartSpec>;
#[doc = "Offset counter start value for main picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n"]
pub mod mi_mp_cb_offs_cnt_start;
#[doc = "MI_MP_CR_BASE_AD_INIT (rw) register accessor: Base address for main picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_cr_base_ad_init::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_mp_cr_base_ad_init::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_mp_cr_base_ad_init`]
module"]
#[doc(alias = "MI_MP_CR_BASE_AD_INIT")]
pub type MiMpCrBaseAdInit = crate::Reg<mi_mp_cr_base_ad_init::MiMpCrBaseAdInitSpec>;
#[doc = "Base address for main picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n"]
pub mod mi_mp_cr_base_ad_init;
#[doc = "MI_MP_CR_SIZE_INIT (rw) register accessor: Size of main picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_cr_size_init::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_mp_cr_size_init::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_mp_cr_size_init`]
module"]
#[doc(alias = "MI_MP_CR_SIZE_INIT")]
pub type MiMpCrSizeInit = crate::Reg<mi_mp_cr_size_init::MiMpCrSizeInitSpec>;
#[doc = "Size of main picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n"]
pub mod mi_mp_cr_size_init;
#[doc = "MI_MP_CR_OFFS_CNT_INIT (rw) register accessor: Offset counter init value for main picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_cr_offs_cnt_init::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_mp_cr_offs_cnt_init::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_mp_cr_offs_cnt_init`]
module"]
#[doc(alias = "MI_MP_CR_OFFS_CNT_INIT")]
pub type MiMpCrOffsCntInit = crate::Reg<mi_mp_cr_offs_cnt_init::MiMpCrOffsCntInitSpec>;
#[doc = "Offset counter init value for main picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n"]
pub mod mi_mp_cr_offs_cnt_init;
#[doc = "MI_MP_CR_OFFS_CNT_START (r) register accessor: Offset counter start value for main picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_cr_offs_cnt_start::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_mp_cr_offs_cnt_start`]
module"]
#[doc(alias = "MI_MP_CR_OFFS_CNT_START")]
pub type MiMpCrOffsCntStart = crate::Reg<mi_mp_cr_offs_cnt_start::MiMpCrOffsCntStartSpec>;
#[doc = "Offset counter start value for main picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n"]
pub mod mi_mp_cr_offs_cnt_start;
#[doc = "MI_SP_Y_BASE_AD_INIT (rw) register accessor: Base address for self picture Y component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_y_base_ad_init::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_sp_y_base_ad_init::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_sp_y_base_ad_init`]
module"]
#[doc(alias = "MI_SP_Y_BASE_AD_INIT")]
pub type MiSpYBaseAdInit = crate::Reg<mi_sp_y_base_ad_init::MiSpYBaseAdInitSpec>;
#[doc = "Base address for self picture Y component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n"]
pub mod mi_sp_y_base_ad_init;
#[doc = "MI_SP_Y_SIZE_INIT (rw) register accessor: Size of self picture Y component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_y_size_init::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_sp_y_size_init::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_sp_y_size_init`]
module"]
#[doc(alias = "MI_SP_Y_SIZE_INIT")]
pub type MiSpYSizeInit = crate::Reg<mi_sp_y_size_init::MiSpYSizeInitSpec>;
#[doc = "Size of self picture Y component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n"]
pub mod mi_sp_y_size_init;
#[doc = "MI_SP_Y_OFFS_CNT_INIT (rw) register accessor: Offset counter init value for self picture Y component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_y_offs_cnt_init::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_sp_y_offs_cnt_init::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_sp_y_offs_cnt_init`]
module"]
#[doc(alias = "MI_SP_Y_OFFS_CNT_INIT")]
pub type MiSpYOffsCntInit = crate::Reg<mi_sp_y_offs_cnt_init::MiSpYOffsCntInitSpec>;
#[doc = "Offset counter init value for self picture Y component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n"]
pub mod mi_sp_y_offs_cnt_init;
#[doc = "MI_SP_Y_OFFS_CNT_START (r) register accessor: Offset counter start value for self picture Y component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_y_offs_cnt_start::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_sp_y_offs_cnt_start`]
module"]
#[doc(alias = "MI_SP_Y_OFFS_CNT_START")]
pub type MiSpYOffsCntStart = crate::Reg<mi_sp_y_offs_cnt_start::MiSpYOffsCntStartSpec>;
#[doc = "Offset counter start value for self picture Y component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n"]
pub mod mi_sp_y_offs_cnt_start;
#[doc = "MI_SP_Y_LLENGTH (rw) register accessor: Line length of self picture Y component\n\nNote: Programmed value becomes effective \n\nimmediately. So write to the register only if no picture data \n\nis sent to the self path. \n\n\n\n \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_y_llength::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_sp_y_llength::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_sp_y_llength`]
module"]
#[doc(alias = "MI_SP_Y_LLENGTH")]
pub type MiSpYLlength = crate::Reg<mi_sp_y_llength::MiSpYLlengthSpec>;
#[doc = "Line length of self picture Y component\n\nNote: Programmed value becomes effective \n\nimmediately. So write to the register only if no picture data \n\nis sent to the self path. \n\n\n\n \n\n\n\n"]
pub mod mi_sp_y_llength;
#[doc = "MI_SP_CB_BASE_AD_INIT (rw) register accessor: Base address for self picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_cb_base_ad_init::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_sp_cb_base_ad_init::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_sp_cb_base_ad_init`]
module"]
#[doc(alias = "MI_SP_CB_BASE_AD_INIT")]
pub type MiSpCbBaseAdInit = crate::Reg<mi_sp_cb_base_ad_init::MiSpCbBaseAdInitSpec>;
#[doc = "Base address for self picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n"]
pub mod mi_sp_cb_base_ad_init;
#[doc = "MI_SP_CB_SIZE_INIT (rw) register accessor: Size of self picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_cb_size_init::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_sp_cb_size_init::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_sp_cb_size_init`]
module"]
#[doc(alias = "MI_SP_CB_SIZE_INIT")]
pub type MiSpCbSizeInit = crate::Reg<mi_sp_cb_size_init::MiSpCbSizeInitSpec>;
#[doc = "Size of self picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n"]
pub mod mi_sp_cb_size_init;
#[doc = "MI_SP_CB_OFFS_CNT_INIT (rw) register accessor: Offset counter init value for self picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_cb_offs_cnt_init::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_sp_cb_offs_cnt_init::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_sp_cb_offs_cnt_init`]
module"]
#[doc(alias = "MI_SP_CB_OFFS_CNT_INIT")]
pub type MiSpCbOffsCntInit = crate::Reg<mi_sp_cb_offs_cnt_init::MiSpCbOffsCntInitSpec>;
#[doc = "Offset counter init value for self picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n \n\n"]
pub mod mi_sp_cb_offs_cnt_init;
#[doc = "MI_SP_CB_OFFS_CNT_START (r) register accessor: Offset counter start value for self picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_cb_offs_cnt_start::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_sp_cb_offs_cnt_start`]
module"]
#[doc(alias = "MI_SP_CB_OFFS_CNT_START")]
pub type MiSpCbOffsCntStart = crate::Reg<mi_sp_cb_offs_cnt_start::MiSpCbOffsCntStartSpec>;
#[doc = "Offset counter start value for self picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n\n\n"]
pub mod mi_sp_cb_offs_cnt_start;
#[doc = "MI_SP_CR_BASE_AD_INIT (rw) register accessor: Base address for self picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_cr_base_ad_init::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_sp_cr_base_ad_init::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_sp_cr_base_ad_init`]
module"]
#[doc(alias = "MI_SP_CR_BASE_AD_INIT")]
pub type MiSpCrBaseAdInit = crate::Reg<mi_sp_cr_base_ad_init::MiSpCrBaseAdInitSpec>;
#[doc = "Base address for self picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n"]
pub mod mi_sp_cr_base_ad_init;
#[doc = "MI_SP_CR_SIZE_INIT (rw) register accessor: Size of self picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_cr_size_init::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_sp_cr_size_init::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_sp_cr_size_init`]
module"]
#[doc(alias = "MI_SP_CR_SIZE_INIT")]
pub type MiSpCrSizeInit = crate::Reg<mi_sp_cr_size_init::MiSpCrSizeInitSpec>;
#[doc = "Size of self picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n"]
pub mod mi_sp_cr_size_init;
#[doc = "MI_SP_CR_OFFS_CNT_INIT (rw) register accessor: Offset counter init value for self picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_cr_offs_cnt_init::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_sp_cr_offs_cnt_init::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_sp_cr_offs_cnt_init`]
module"]
#[doc(alias = "MI_SP_CR_OFFS_CNT_INIT")]
pub type MiSpCrOffsCntInit = crate::Reg<mi_sp_cr_offs_cnt_init::MiSpCrOffsCntInitSpec>;
#[doc = "Offset counter init value for self picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n"]
pub mod mi_sp_cr_offs_cnt_init;
#[doc = "MI_SP_CR_OFFS_CNT_START (r) register accessor: Offset counter start value for self picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_cr_offs_cnt_start::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_sp_cr_offs_cnt_start`]
module"]
#[doc(alias = "MI_SP_CR_OFFS_CNT_START")]
pub type MiSpCrOffsCntStart = crate::Reg<mi_sp_cr_offs_cnt_start::MiSpCrOffsCntStartSpec>;
#[doc = "Offset counter start value for self picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n \n\n\n\n \n\n"]
pub mod mi_sp_cr_offs_cnt_start;
#[doc = "MI_BYTE_CNT (r) register accessor: Counter value of JPEG or RAW data bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_byte_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_byte_cnt`]
module"]
#[doc(alias = "MI_BYTE_CNT")]
pub type MiByteCnt = crate::Reg<mi_byte_cnt::MiByteCntSpec>;
#[doc = "Counter value of JPEG or RAW data bytes"]
pub mod mi_byte_cnt;
#[doc = "MI_CTRL_SHD (r) register accessor: global control internal shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_ctrl_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_ctrl_shd`]
module"]
#[doc(alias = "MI_CTRL_SHD")]
pub type MiCtrlShd = crate::Reg<mi_ctrl_shd::MiCtrlShdSpec>;
#[doc = "global control internal shadow register"]
pub mod mi_ctrl_shd;
#[doc = "MI_MP_Y_BASE_AD_SHD (r) register accessor: Base address shadow register for main picture Y \n\n\n\ncomponent, JPEG or raw data ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_y_base_ad_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_mp_y_base_ad_shd`]
module"]
#[doc(alias = "MI_MP_Y_BASE_AD_SHD")]
pub type MiMpYBaseAdShd = crate::Reg<mi_mp_y_base_ad_shd::MiMpYBaseAdShdSpec>;
#[doc = "Base address shadow register for main picture Y \n\n\n\ncomponent, JPEG or raw data ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n"]
pub mod mi_mp_y_base_ad_shd;
#[doc = "MI_MP_Y_SIZE_SHD (r) register accessor: Size shadow register of main picture Y component, JPEG \n\n\n\nor raw data\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_y_size_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_mp_y_size_shd`]
module"]
#[doc(alias = "MI_MP_Y_SIZE_SHD")]
pub type MiMpYSizeShd = crate::Reg<mi_mp_y_size_shd::MiMpYSizeShdSpec>;
#[doc = "Size shadow register of main picture Y component, JPEG \n\n\n\nor raw data\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n"]
pub mod mi_mp_y_size_shd;
#[doc = "MI_MP_Y_OFFS_CNT_SHD (r) register accessor: Current offset counter of main picture Y component, JPEG \n\n\n\nor raw data ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_y_offs_cnt_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_mp_y_offs_cnt_shd`]
module"]
#[doc(alias = "MI_MP_Y_OFFS_CNT_SHD")]
pub type MiMpYOffsCntShd = crate::Reg<mi_mp_y_offs_cnt_shd::MiMpYOffsCntShdSpec>;
#[doc = "Current offset counter of main picture Y component, JPEG \n\n\n\nor raw data ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n"]
pub mod mi_mp_y_offs_cnt_shd;
#[doc = "MI_MP_Y_IRQ_OFFS_SHD (r) register accessor: Shadow register of fill level interrupt offset value for main \n\n\n\npicture Y component, JPEG or raw data\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_y_irq_offs_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_mp_y_irq_offs_shd`]
module"]
#[doc(alias = "MI_MP_Y_IRQ_OFFS_SHD")]
pub type MiMpYIrqOffsShd = crate::Reg<mi_mp_y_irq_offs_shd::MiMpYIrqOffsShdSpec>;
#[doc = "Shadow register of fill level interrupt offset value for main \n\n\n\npicture Y component, JPEG or raw data\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n"]
pub mod mi_mp_y_irq_offs_shd;
#[doc = "MI_MP_CB_BASE_AD_SHD (r) register accessor: Base address shadow register for main picture Cb \n\n\n\ncomponent ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_cb_base_ad_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_mp_cb_base_ad_shd`]
module"]
#[doc(alias = "MI_MP_CB_BASE_AD_SHD")]
pub type MiMpCbBaseAdShd = crate::Reg<mi_mp_cb_base_ad_shd::MiMpCbBaseAdShdSpec>;
#[doc = "Base address shadow register for main picture Cb \n\n\n\ncomponent ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n"]
pub mod mi_mp_cb_base_ad_shd;
#[doc = "MI_MP_CB_SIZE_SHD (r) register accessor: Size shadow register of main picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_cb_size_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_mp_cb_size_shd`]
module"]
#[doc(alias = "MI_MP_CB_SIZE_SHD")]
pub type MiMpCbSizeShd = crate::Reg<mi_mp_cb_size_shd::MiMpCbSizeShdSpec>;
#[doc = "Size shadow register of main picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n"]
pub mod mi_mp_cb_size_shd;
#[doc = "MI_MP_CB_OFFS_CNT_SHD (r) register accessor: Current offset counter of main picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_cb_offs_cnt_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_mp_cb_offs_cnt_shd`]
module"]
#[doc(alias = "MI_MP_CB_OFFS_CNT_SHD")]
pub type MiMpCbOffsCntShd = crate::Reg<mi_mp_cb_offs_cnt_shd::MiMpCbOffsCntShdSpec>;
#[doc = "Current offset counter of main picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n"]
pub mod mi_mp_cb_offs_cnt_shd;
#[doc = "MI_MP_CR_BASE_AD_SHD (r) register accessor: Base address shadow register for main picture Cr \n\n\n\ncomponent ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_cr_base_ad_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_mp_cr_base_ad_shd`]
module"]
#[doc(alias = "MI_MP_CR_BASE_AD_SHD")]
pub type MiMpCrBaseAdShd = crate::Reg<mi_mp_cr_base_ad_shd::MiMpCrBaseAdShdSpec>;
#[doc = "Base address shadow register for main picture Cr \n\n\n\ncomponent ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n"]
pub mod mi_mp_cr_base_ad_shd;
#[doc = "MI_MP_CR_SIZE_SHD (r) register accessor: Size shadow register of main picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_cr_size_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_mp_cr_size_shd`]
module"]
#[doc(alias = "MI_MP_CR_SIZE_SHD")]
pub type MiMpCrSizeShd = crate::Reg<mi_mp_cr_size_shd::MiMpCrSizeShdSpec>;
#[doc = "Size shadow register of main picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n"]
pub mod mi_mp_cr_size_shd;
#[doc = "MI_MP_CR_OFFS_CNT_SHD (r) register accessor: Current offset counter of main picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_cr_offs_cnt_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_mp_cr_offs_cnt_shd`]
module"]
#[doc(alias = "MI_MP_CR_OFFS_CNT_SHD")]
pub type MiMpCrOffsCntShd = crate::Reg<mi_mp_cr_offs_cnt_shd::MiMpCrOffsCntShdSpec>;
#[doc = "Current offset counter of main picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n"]
pub mod mi_mp_cr_offs_cnt_shd;
#[doc = "MI_SP_Y_BASE_AD_SHD (r) register accessor: Base address shadow register for self picture Y \n\n\n\ncomponent ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_y_base_ad_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_sp_y_base_ad_shd`]
module"]
#[doc(alias = "MI_SP_Y_BASE_AD_SHD")]
pub type MiSpYBaseAdShd = crate::Reg<mi_sp_y_base_ad_shd::MiSpYBaseAdShdSpec>;
#[doc = "Base address shadow register for self picture Y \n\n\n\ncomponent ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n"]
pub mod mi_sp_y_base_ad_shd;
#[doc = "MI_SP_Y_SIZE_SHD (r) register accessor: Size shadow register of self picture Y component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_y_size_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_sp_y_size_shd`]
module"]
#[doc(alias = "MI_SP_Y_SIZE_SHD")]
pub type MiSpYSizeShd = crate::Reg<mi_sp_y_size_shd::MiSpYSizeShdSpec>;
#[doc = "Size shadow register of self picture Y component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n"]
pub mod mi_sp_y_size_shd;
#[doc = "MI_SP_Y_OFFS_CNT_SHD (r) register accessor: Current offset counter of self picture Y component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_y_offs_cnt_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_sp_y_offs_cnt_shd`]
module"]
#[doc(alias = "MI_SP_Y_OFFS_CNT_SHD")]
pub type MiSpYOffsCntShd = crate::Reg<mi_sp_y_offs_cnt_shd::MiSpYOffsCntShdSpec>;
#[doc = "Current offset counter of self picture Y component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n"]
pub mod mi_sp_y_offs_cnt_shd;
#[doc = "MI_SP_CB_BASE_AD_SHD (r) register accessor: Base address shadow register for self picture Cb \n\n\n\ncomponent ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_cb_base_ad_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_sp_cb_base_ad_shd`]
module"]
#[doc(alias = "MI_SP_CB_BASE_AD_SHD")]
pub type MiSpCbBaseAdShd = crate::Reg<mi_sp_cb_base_ad_shd::MiSpCbBaseAdShdSpec>;
#[doc = "Base address shadow register for self picture Cb \n\n\n\ncomponent ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n"]
pub mod mi_sp_cb_base_ad_shd;
#[doc = "MI_SP_CB_SIZE_SHD (r) register accessor: Size shadow register of self picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_cb_size_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_sp_cb_size_shd`]
module"]
#[doc(alias = "MI_SP_CB_SIZE_SHD")]
pub type MiSpCbSizeShd = crate::Reg<mi_sp_cb_size_shd::MiSpCbSizeShdSpec>;
#[doc = "Size shadow register of self picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n"]
pub mod mi_sp_cb_size_shd;
#[doc = "MI_SP_CB_OFFS_CNT_SHD (r) register accessor: Current offset counter of self picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_cb_offs_cnt_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_sp_cb_offs_cnt_shd`]
module"]
#[doc(alias = "MI_SP_CB_OFFS_CNT_SHD")]
pub type MiSpCbOffsCntShd = crate::Reg<mi_sp_cb_offs_cnt_shd::MiSpCbOffsCntShdSpec>;
#[doc = "Current offset counter of self picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n"]
pub mod mi_sp_cb_offs_cnt_shd;
#[doc = "MI_SP_CR_BASE_AD_SHD (r) register accessor: Base address shadow register for self picture Cr \n\n\n\ncomponent ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_cr_base_ad_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_sp_cr_base_ad_shd`]
module"]
#[doc(alias = "MI_SP_CR_BASE_AD_SHD")]
pub type MiSpCrBaseAdShd = crate::Reg<mi_sp_cr_base_ad_shd::MiSpCrBaseAdShdSpec>;
#[doc = "Base address shadow register for self picture Cr \n\n\n\ncomponent ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n"]
pub mod mi_sp_cr_base_ad_shd;
#[doc = "MI_SP_CR_SIZE_SHD (r) register accessor: Size shadow register of self picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_cr_size_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_sp_cr_size_shd`]
module"]
#[doc(alias = "MI_SP_CR_SIZE_SHD")]
pub type MiSpCrSizeShd = crate::Reg<mi_sp_cr_size_shd::MiSpCrSizeShdSpec>;
#[doc = "Size shadow register of self picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n"]
pub mod mi_sp_cr_size_shd;
#[doc = "MI_SP_CR_OFFS_CNT_SHD (r) register accessor: Current offset counter of self picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_cr_offs_cnt_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_sp_cr_offs_cnt_shd`]
module"]
#[doc(alias = "MI_SP_CR_OFFS_CNT_SHD")]
pub type MiSpCrOffsCntShd = crate::Reg<mi_sp_cr_offs_cnt_shd::MiSpCrOffsCntShdSpec>;
#[doc = "Current offset counter of self picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n"]
pub mod mi_sp_cr_offs_cnt_shd;
#[doc = "MI_DMA_Y_PIC_START_AD (rw) register accessor: Y component image start address\n\nNote: Must be multiple of 4 in interleaved mode. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_dma_y_pic_start_ad::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_dma_y_pic_start_ad::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_dma_y_pic_start_ad`]
module"]
#[doc(alias = "MI_DMA_Y_PIC_START_AD")]
pub type MiDmaYPicStartAd = crate::Reg<mi_dma_y_pic_start_ad::MiDmaYPicStartAdSpec>;
#[doc = "Y component image start address\n\nNote: Must be multiple of 4 in interleaved mode. \n\n\n\n \n\n"]
pub mod mi_dma_y_pic_start_ad;
#[doc = "MI_DMA_Y_PIC_WIDTH (rw) register accessor: Y component image width\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_dma_y_pic_width::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_dma_y_pic_width::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_dma_y_pic_width`]
module"]
#[doc(alias = "MI_DMA_Y_PIC_WIDTH")]
pub type MiDmaYPicWidth = crate::Reg<mi_dma_y_pic_width::MiDmaYPicWidthSpec>;
#[doc = "Y component image width"]
pub mod mi_dma_y_pic_width;
#[doc = "MI_DMA_Y_LLENGTH (rw) register accessor: Y component original line length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_dma_y_llength::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_dma_y_llength::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_dma_y_llength`]
module"]
#[doc(alias = "MI_DMA_Y_LLENGTH")]
pub type MiDmaYLlength = crate::Reg<mi_dma_y_llength::MiDmaYLlengthSpec>;
#[doc = "Y component original line length"]
pub mod mi_dma_y_llength;
#[doc = "MI_DMA_Y_PIC_SIZE (rw) register accessor: Y component image size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_dma_y_pic_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_dma_y_pic_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_dma_y_pic_size`]
module"]
#[doc(alias = "MI_DMA_Y_PIC_SIZE")]
pub type MiDmaYPicSize = crate::Reg<mi_dma_y_pic_size::MiDmaYPicSizeSpec>;
#[doc = "Y component image size"]
pub mod mi_dma_y_pic_size;
#[doc = "MI_DMA_CB_PIC_START_AD (rw) register accessor: Cb component image start address\n\nNote: Must be multiple of 2 in semi-planar mode. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_dma_cb_pic_start_ad::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_dma_cb_pic_start_ad::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_dma_cb_pic_start_ad`]
module"]
#[doc(alias = "MI_DMA_CB_PIC_START_AD")]
pub type MiDmaCbPicStartAd = crate::Reg<mi_dma_cb_pic_start_ad::MiDmaCbPicStartAdSpec>;
#[doc = "Cb component image start address\n\nNote: Must be multiple of 2 in semi-planar mode. \n\n\n\n \n\n"]
pub mod mi_dma_cb_pic_start_ad;
#[doc = "MI_DMA_CR_PIC_START_AD (rw) register accessor: Cr component image start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_dma_cr_pic_start_ad::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_dma_cr_pic_start_ad::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_dma_cr_pic_start_ad`]
module"]
#[doc(alias = "MI_DMA_CR_PIC_START_AD")]
pub type MiDmaCrPicStartAd = crate::Reg<mi_dma_cr_pic_start_ad::MiDmaCrPicStartAdSpec>;
#[doc = "Cr component image start address"]
pub mod mi_dma_cr_pic_start_ad;
#[doc = "MI_IMSC (rw) register accessor: Interrupt Mask („1‟: interrupt active, „0‟: interrupt masked)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_imsc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_imsc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_imsc`]
module"]
#[doc(alias = "MI_IMSC")]
pub type MiImsc = crate::Reg<mi_imsc::MiImscSpec>;
#[doc = "Interrupt Mask („1‟: interrupt active, „0‟: interrupt masked)"]
pub mod mi_imsc;
#[doc = "MI_RIS (r) register accessor: Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_ris::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_ris`]
module"]
#[doc(alias = "MI_RIS")]
pub type MiRis = crate::Reg<mi_ris::MiRisSpec>;
#[doc = "Raw Interrupt Status"]
pub mod mi_ris;
#[doc = "MI_MIS (r) register accessor: Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mis::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_mis`]
module"]
#[doc(alias = "MI_MIS")]
pub type MiMis = crate::Reg<mi_mis::MiMisSpec>;
#[doc = "Masked Interrupt Status"]
pub mod mi_mis;
#[doc = "MI_ICR (w) register accessor: Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_icr`]
module"]
#[doc(alias = "MI_ICR")]
pub type MiIcr = crate::Reg<mi_icr::MiIcrSpec>;
#[doc = "Interrupt Clear Register"]
pub mod mi_icr;
#[doc = "MI_ISR (w) register accessor: Interrupt Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_isr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_isr`]
module"]
#[doc(alias = "MI_ISR")]
pub type MiIsr = crate::Reg<mi_isr::MiIsrSpec>;
#[doc = "Interrupt Set Register"]
pub mod mi_isr;
#[doc = "MI_STATUS (r) register accessor: MI Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_status`]
module"]
#[doc(alias = "MI_STATUS")]
pub type MiStatus = crate::Reg<mi_status::MiStatusSpec>;
#[doc = "MI Status Register"]
pub mod mi_status;
#[doc = "MI_STATUS_CLR (w) register accessor: MI Status Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_status_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_status_clr`]
module"]
#[doc(alias = "MI_STATUS_CLR")]
pub type MiStatusClr = crate::Reg<mi_status_clr::MiStatusClrSpec>;
#[doc = "MI Status Clear Register"]
pub mod mi_status_clr;
#[doc = "MI_SP_Y_PIC_WIDTH (rw) register accessor: Y component image width\n\nNote: Programmed value becomes effective \n\nimmediately. So write to the register only if no picture \n\ndata is sent to the self path. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_y_pic_width::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_sp_y_pic_width::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_sp_y_pic_width`]
module"]
#[doc(alias = "MI_SP_Y_PIC_WIDTH")]
pub type MiSpYPicWidth = crate::Reg<mi_sp_y_pic_width::MiSpYPicWidthSpec>;
#[doc = "Y component image width\n\nNote: Programmed value becomes effective \n\nimmediately. So write to the register only if no picture \n\ndata is sent to the self path. \n\n\n\n \n\n"]
pub mod mi_sp_y_pic_width;
#[doc = "MI_SP_Y_PIC_HEIGHT (rw) register accessor: Y component image height\n\nNote: Programmed value becomes effective \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_y_pic_height::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_sp_y_pic_height::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_sp_y_pic_height`]
module"]
#[doc(alias = "MI_SP_Y_PIC_HEIGHT")]
pub type MiSpYPicHeight = crate::Reg<mi_sp_y_pic_height::MiSpYPicHeightSpec>;
#[doc = "Y component image height\n\nNote: Programmed value becomes effective \n\n"]
pub mod mi_sp_y_pic_height;
#[doc = "MI_SP_Y_PIC_SIZE (rw) register accessor: Y component image size\n\nNote: Programmed value becomes effective \n\n\n\nimmediately. So write to the register only if no picture \n\n\n\ndata is sent to the self path. \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_y_pic_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_sp_y_pic_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_sp_y_pic_size`]
module"]
#[doc(alias = "MI_SP_Y_PIC_SIZE")]
pub type MiSpYPicSize = crate::Reg<mi_sp_y_pic_size::MiSpYPicSizeSpec>;
#[doc = "Y component image size\n\nNote: Programmed value becomes effective \n\n\n\nimmediately. So write to the register only if no picture \n\n\n\ndata is sent to the self path. \n\n"]
pub mod mi_sp_y_pic_size;
#[doc = "MI_DMA_CTRL (rw) register accessor: DMA control register\n\nNote: The dma_ready \n\ninterrupt is raised as usual, but the dma_frame_end \n\ninterrupt will not be generated until v_end has been \n\nenabled again. \n\n\n\n9 dma_continuous_en Enables continuous mode. If set the same frame is \n\nread back over and over. A start pulse on dma_start is \n\nneeded only for the first time. To stop continuous mode \n\nreset this bit (takes effect after the next frame end) or \n\nexecute a soft reset. This bit is intended to be used in \n\nconjunction with the Superimpose feature. \n\n\n\n8 dma_byte_swap Enables change of DMA byte order of the 32 bit \n\n\n\ninput word at read port \n\n1: byte order is mirrored but the bit order within one \n\n\n\nbyte doesn‟t change \n\n\n\n0: no byte mirroring \n\n\n\n7:6 dma_inout_format Selects input/output format of \n\n\n\nDMA picture. 11: YCbCr 4:4:4 \n\n\n\n10: YCbCr 4:2:2 \n\n\n\n01: YCbCr 4:2:0 \n\n\n\n00: YCbCr 4:0:0 \n\n\n\n5:4 dma_read_format Defines how YCbCr picture data is read from \n\n\n\nmemory. 00: planar \n\n01: semi planar, for YCbCr 4:2:x \n\n10: interleaved (combined), for YCbCr 4:2:2 and RGB \n\n\n\nonly 11: reserved \n\n\n\n3:2 dma_burst_len_chrom Burst length for Cb or Cr data affecting DMA \n\n\n\nread port. 00: 4-beat bursts \n\n\n\n01: 8-beat bursts \n\n\n\n10: 16-beat bursts \n\n11: reserved \n\nIgnored if 8- or 16-beat bursts are not supported. \n\n\n\nDMA control register Reset value: 0000'0000H \n\n \n\n\n\nAddress: ISP_MI_BASE + 0120H Mode : rw \n\n\n\nBit\n\n\n\ns \n\n\n\nName Description \n\n\n\n1:0 dma_burst_len_lum Burst length for Y data affecting DMA read port. \n\n\n\n00: 4-beat bursts \n\n\n\n01: 8-beat bursts \n\n10: 16-beat bursts \n\n\n\n11: reserved \n\n\n\nIgnored if 8- or 16-beat bursts are not supported. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_dma_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_dma_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_dma_ctrl`]
module"]
#[doc(alias = "MI_DMA_CTRL")]
pub type MiDmaCtrl = crate::Reg<mi_dma_ctrl::MiDmaCtrlSpec>;
#[doc = "DMA control register\n\nNote: The dma_ready \n\ninterrupt is raised as usual, but the dma_frame_end \n\ninterrupt will not be generated until v_end has been \n\nenabled again. \n\n\n\n9 dma_continuous_en Enables continuous mode. If set the same frame is \n\nread back over and over. A start pulse on dma_start is \n\nneeded only for the first time. To stop continuous mode \n\nreset this bit (takes effect after the next frame end) or \n\nexecute a soft reset. This bit is intended to be used in \n\nconjunction with the Superimpose feature. \n\n\n\n8 dma_byte_swap Enables change of DMA byte order of the 32 bit \n\n\n\ninput word at read port \n\n1: byte order is mirrored but the bit order within one \n\n\n\nbyte doesn‟t change \n\n\n\n0: no byte mirroring \n\n\n\n7:6 dma_inout_format Selects input/output format of \n\n\n\nDMA picture. 11: YCbCr 4:4:4 \n\n\n\n10: YCbCr 4:2:2 \n\n\n\n01: YCbCr 4:2:0 \n\n\n\n00: YCbCr 4:0:0 \n\n\n\n5:4 dma_read_format Defines how YCbCr picture data is read from \n\n\n\nmemory. 00: planar \n\n01: semi planar, for YCbCr 4:2:x \n\n10: interleaved (combined), for YCbCr 4:2:2 and RGB \n\n\n\nonly 11: reserved \n\n\n\n3:2 dma_burst_len_chrom Burst length for Cb or Cr data affecting DMA \n\n\n\nread port. 00: 4-beat bursts \n\n\n\n01: 8-beat bursts \n\n\n\n10: 16-beat bursts \n\n11: reserved \n\nIgnored if 8- or 16-beat bursts are not supported. \n\n\n\nDMA control register Reset value: 0000'0000H \n\n \n\n\n\nAddress: ISP_MI_BASE + 0120H Mode : rw \n\n\n\nBit\n\n\n\ns \n\n\n\nName Description \n\n\n\n1:0 dma_burst_len_lum Burst length for Y data affecting DMA read port. \n\n\n\n00: 4-beat bursts \n\n\n\n01: 8-beat bursts \n\n10: 16-beat bursts \n\n\n\n11: reserved \n\n\n\nIgnored if 8- or 16-beat bursts are not supported. \n\n\n\n \n\n"]
pub mod mi_dma_ctrl;
#[doc = "MI_DMA_START (w) register accessor: DMA start register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_dma_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_dma_start`]
module"]
#[doc(alias = "MI_DMA_START")]
pub type MiDmaStart = crate::Reg<mi_dma_start::MiDmaStartSpec>;
#[doc = "DMA start register"]
pub mod mi_dma_start;
#[doc = "MI_DMA_STATUS (r) register accessor: DMA status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_dma_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_dma_status`]
module"]
#[doc(alias = "MI_DMA_STATUS")]
pub type MiDmaStatus = crate::Reg<mi_dma_status::MiDmaStatusSpec>;
#[doc = "DMA status register"]
pub mod mi_dma_status;
#[doc = "MI_PIXEL_CNT (r) register accessor: Counter value for defect pixel list\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_pixel_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_pixel_cnt`]
module"]
#[doc(alias = "MI_PIXEL_CNT")]
pub type MiPixelCnt = crate::Reg<mi_pixel_cnt::MiPixelCntSpec>;
#[doc = "Counter value for defect pixel list"]
pub mod mi_pixel_cnt;
#[doc = "MI_MP_Y_BASE_AD_INIT2 (rw) register accessor: Base address 2 (ping pong) for main picture Y component, \n\n\n\nJPEG or raw data\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_y_base_ad_init2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_mp_y_base_ad_init2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_mp_y_base_ad_init2`]
module"]
#[doc(alias = "MI_MP_Y_BASE_AD_INIT2")]
pub type MiMpYBaseAdInit2 = crate::Reg<mi_mp_y_base_ad_init2::MiMpYBaseAdInit2Spec>;
#[doc = "Base address 2 (ping pong) for main picture Y component, \n\n\n\nJPEG or raw data\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n"]
pub mod mi_mp_y_base_ad_init2;
#[doc = "MI_MP_CB_BASE_AD_INIT2 (rw) register accessor: Base address 2 (pingpong) for main picture Cb component\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_cb_base_ad_init2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_mp_cb_base_ad_init2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_mp_cb_base_ad_init2`]
module"]
#[doc(alias = "MI_MP_CB_BASE_AD_INIT2")]
pub type MiMpCbBaseAdInit2 = crate::Reg<mi_mp_cb_base_ad_init2::MiMpCbBaseAdInit2Spec>;
#[doc = "Base address 2 (pingpong) for main picture Cb component\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n"]
pub mod mi_mp_cb_base_ad_init2;
#[doc = "MI_MP_CR_BASE_AD_INIT2 (rw) register accessor: Base address 2 (pingpong) for main picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_cr_base_ad_init2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_mp_cr_base_ad_init2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_mp_cr_base_ad_init2`]
module"]
#[doc(alias = "MI_MP_CR_BASE_AD_INIT2")]
pub type MiMpCrBaseAdInit2 = crate::Reg<mi_mp_cr_base_ad_init2::MiMpCrBaseAdInit2Spec>;
#[doc = "Base address 2 (pingpong) for main picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n"]
pub mod mi_mp_cr_base_ad_init2;
#[doc = "MI_SP_Y_BASE_AD_INIT2 (rw) register accessor: Base address 2 (ping pong) for self picture Y component\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_y_base_ad_init2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_sp_y_base_ad_init2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_sp_y_base_ad_init2`]
module"]
#[doc(alias = "MI_SP_Y_BASE_AD_INIT2")]
pub type MiSpYBaseAdInit2 = crate::Reg<mi_sp_y_base_ad_init2::MiSpYBaseAdInit2Spec>;
#[doc = "Base address 2 (ping pong) for self picture Y component\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n"]
pub mod mi_sp_y_base_ad_init2;
#[doc = "MI_SP_CB_BASE_AD_INIT2 (rw) register accessor: Base address 2 (pingpong) for self picture Cb component\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_cb_base_ad_init2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_sp_cb_base_ad_init2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_sp_cb_base_ad_init2`]
module"]
#[doc(alias = "MI_SP_CB_BASE_AD_INIT2")]
pub type MiSpCbBaseAdInit2 = crate::Reg<mi_sp_cb_base_ad_init2::MiSpCbBaseAdInit2Spec>;
#[doc = "Base address 2 (pingpong) for self picture Cb component\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n"]
pub mod mi_sp_cb_base_ad_init2;
#[doc = "MI_SP_CR_BASE_AD_INIT2 (rw) register accessor: Base address 2 (pingpong) for self picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_cr_base_ad_init2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_sp_cr_base_ad_init2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_sp_cr_base_ad_init2`]
module"]
#[doc(alias = "MI_SP_CR_BASE_AD_INIT2")]
pub type MiSpCrBaseAdInit2 = crate::Reg<mi_sp_cr_base_ad_init2::MiSpCrBaseAdInit2Spec>;
#[doc = "Base address 2 (pingpong) for self picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n"]
pub mod mi_sp_cr_base_ad_init2;
#[doc = "MI_XTD_FORMAT_CTRL (rw) register accessor: Extended Storage Format Control for main, self and dma read path\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_xtd_format_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_xtd_format_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi_xtd_format_ctrl`]
module"]
#[doc(alias = "MI_XTD_FORMAT_CTRL")]
pub type MiXtdFormatCtrl = crate::Reg<mi_xtd_format_ctrl::MiXtdFormatCtrlSpec>;
#[doc = "Extended Storage Format Control for main, self and dma read path"]
pub mod mi_xtd_format_ctrl;
#[doc = "MIPI_CTRL (rw) register accessor: global control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mipi_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_ctrl`]
module"]
#[doc(alias = "MIPI_CTRL")]
pub type MipiCtrl = crate::Reg<mipi_ctrl::MipiCtrlSpec>;
#[doc = "global control register"]
pub mod mipi_ctrl;
#[doc = "MIPI_STATUS (r) register accessor: global status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_status`]
module"]
#[doc(alias = "MIPI_STATUS")]
pub type MipiStatus = crate::Reg<mipi_status::MipiStatusSpec>;
#[doc = "global status register"]
pub mod mipi_status;
#[doc = "MIPI_IMSC (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_imsc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mipi_imsc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_imsc`]
module"]
#[doc(alias = "MIPI_IMSC")]
pub type MipiImsc = crate::Reg<mipi_imsc::MipiImscSpec>;
#[doc = "Interrupt mask"]
pub mod mipi_imsc;
#[doc = "MIPI_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_ris::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_ris`]
module"]
#[doc(alias = "MIPI_RIS")]
pub type MipiRis = crate::Reg<mipi_ris::MipiRisSpec>;
#[doc = "Raw interrupt status"]
pub mod mipi_ris;
#[doc = "MIPI_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_mis::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_mis`]
module"]
#[doc(alias = "MIPI_MIS")]
pub type MipiMis = crate::Reg<mipi_mis::MipiMisSpec>;
#[doc = "Masked interrupt status"]
pub mod mipi_mis;
#[doc = "MIPI_ICR (w) register accessor: Interrupt clear register\n\nNote: clears corresponding bits in MIPI_RIS register \n\n\n\n \n\n\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mipi_icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_icr`]
module"]
#[doc(alias = "MIPI_ICR")]
pub type MipiIcr = crate::Reg<mipi_icr::MipiIcrSpec>;
#[doc = "Interrupt clear register\n\nNote: clears corresponding bits in MIPI_RIS register \n\n\n\n \n\n"]
pub mod mipi_icr;
#[doc = "MIPI_ISR (w) register accessor: Interrupt set register\n\nNote: sets corresponding bits in MIPI_RIS register \n\n\n\n \n\n\n\n\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mipi_isr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_isr`]
module"]
#[doc(alias = "MIPI_ISR")]
pub type MipiIsr = crate::Reg<mipi_isr::MipiIsrSpec>;
#[doc = "Interrupt set register\n\nNote: sets corresponding bits in MIPI_RIS register \n\n\n\n \n\n\n\n"]
pub mod mipi_isr;
#[doc = "MIPI_CUR_DATA_ID (r) register accessor: Current Data Identifier\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_cur_data_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_cur_data_id`]
module"]
#[doc(alias = "MIPI_CUR_DATA_ID")]
pub type MipiCurDataId = crate::Reg<mipi_cur_data_id::MipiCurDataIdSpec>;
#[doc = "Current Data Identifier"]
pub mod mipi_cur_data_id;
#[doc = "MIPI_IMG_DATA_SEL (rw) register accessor: Image Data Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_img_data_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mipi_img_data_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_img_data_sel`]
module"]
#[doc(alias = "MIPI_IMG_DATA_SEL")]
pub type MipiImgDataSel = crate::Reg<mipi_img_data_sel::MipiImgDataSelSpec>;
#[doc = "Image Data Selector"]
pub mod mipi_img_data_sel;
#[doc = "MIPI_ADD_DATA_SEL_1 (rw) register accessor: Additional Data Selector 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_add_data_sel_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mipi_add_data_sel_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_add_data_sel_1`]
module"]
#[doc(alias = "MIPI_ADD_DATA_SEL_1")]
pub type MipiAddDataSel1 = crate::Reg<mipi_add_data_sel_1::MipiAddDataSel1Spec>;
#[doc = "Additional Data Selector 1"]
pub mod mipi_add_data_sel_1;
#[doc = "MIPI_ADD_DATA_SEL_2 (rw) register accessor: Additional Data Selector 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_add_data_sel_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mipi_add_data_sel_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_add_data_sel_2`]
module"]
#[doc(alias = "MIPI_ADD_DATA_SEL_2")]
pub type MipiAddDataSel2 = crate::Reg<mipi_add_data_sel_2::MipiAddDataSel2Spec>;
#[doc = "Additional Data Selector 2"]
pub mod mipi_add_data_sel_2;
#[doc = "MIPI_ADD_DATA_SEL_3 (rw) register accessor: Additional Data Selector 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_add_data_sel_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mipi_add_data_sel_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_add_data_sel_3`]
module"]
#[doc(alias = "MIPI_ADD_DATA_SEL_3")]
pub type MipiAddDataSel3 = crate::Reg<mipi_add_data_sel_3::MipiAddDataSel3Spec>;
#[doc = "Additional Data Selector 3"]
pub mod mipi_add_data_sel_3;
#[doc = "MIPI_ADD_DATA_SEL_4 (rw) register accessor: Additional Data Selector 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_add_data_sel_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mipi_add_data_sel_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_add_data_sel_4`]
module"]
#[doc(alias = "MIPI_ADD_DATA_SEL_4")]
pub type MipiAddDataSel4 = crate::Reg<mipi_add_data_sel_4::MipiAddDataSel4Spec>;
#[doc = "Additional Data Selector 4"]
pub mod mipi_add_data_sel_4;
#[doc = "MIPI_ADD_DATA_FIFO (r) register accessor: Additional Data Fifo\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_add_data_fifo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_add_data_fifo`]
module"]
#[doc(alias = "MIPI_ADD_DATA_FIFO")]
pub type MipiAddDataFifo = crate::Reg<mipi_add_data_fifo::MipiAddDataFifoSpec>;
#[doc = "Additional Data Fifo"]
pub mod mipi_add_data_fifo;
#[doc = "MIPI_COMPRESSED_MODE (rw) register accessor: controls processing of compressed raw data types\n\nNote: This register is only available in MIPI interface version 2 of ISP \n\n\n\n \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_compressed_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mipi_compressed_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_compressed_mode`]
module"]
#[doc(alias = "MIPI_COMPRESSED_MODE")]
pub type MipiCompressedMode = crate::Reg<mipi_compressed_mode::MipiCompressedModeSpec>;
#[doc = "controls processing of compressed raw data types\n\nNote: This register is only available in MIPI interface version 2 of ISP \n\n\n\n \n\n\n\n \n\n"]
pub mod mipi_compressed_mode;
#[doc = "MIPI_FRAME (r) register accessor: frame number from frame start and frame end short packets\n\nNote: This register is only available in MIPI interface version 2 of ISP \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_frame::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_frame`]
module"]
#[doc(alias = "MIPI_FRAME")]
pub type MipiFrame = crate::Reg<mipi_frame::MipiFrameSpec>;
#[doc = "frame number from frame start and frame end short packets\n\nNote: This register is only available in MIPI interface version 2 of ISP \n\n\n\n"]
pub mod mipi_frame;
#[doc = "MIPI_GEN_SHORT_DT (r) register accessor: data type flags for received generic short packets\n\nNote: This register is only available in MIPI interface version 2 of ISP \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_gen_short_dt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_gen_short_dt`]
module"]
#[doc(alias = "MIPI_GEN_SHORT_DT")]
pub type MipiGenShortDt = crate::Reg<mipi_gen_short_dt::MipiGenShortDtSpec>;
#[doc = "data type flags for received generic short packets\n\nNote: This register is only available in MIPI interface version 2 of ISP \n\n\n\n \n\n"]
pub mod mipi_gen_short_dt;
#[doc = "MIPI_GEN_SHORT_8_9 (r) register accessor: data field for generic short packets of data type 0x8 and 0x9\n\nNote: This register is only available in MIPI interface version 2 of ISP \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_gen_short_8_9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_gen_short_8_9`]
module"]
#[doc(alias = "MIPI_GEN_SHORT_8_9")]
pub type MipiGenShort8_9 = crate::Reg<mipi_gen_short_8_9::MipiGenShort8_9Spec>;
#[doc = "data field for generic short packets of data type 0x8 and 0x9\n\nNote: This register is only available in MIPI interface version 2 of ISP \n\n\n\n"]
pub mod mipi_gen_short_8_9;
#[doc = "MIPI_GEN_SHORT_A_B (r) register accessor: data field for generic short packets of data type 0xA and 0xB\n\nNote: This register is only available in MIPI interface version 2 of ISP \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_gen_short_a_b::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_gen_short_a_b`]
module"]
#[doc(alias = "MIPI_GEN_SHORT_A_B")]
pub type MipiGenShortAB = crate::Reg<mipi_gen_short_a_b::MipiGenShortABSpec>;
#[doc = "data field for generic short packets of data type 0xA and 0xB\n\nNote: This register is only available in MIPI interface version 2 of ISP \n\n\n\n \n\n"]
pub mod mipi_gen_short_a_b;
#[doc = "MIPI_GEN_SHORT_C_D (r) register accessor: data field for generic short packets of data type 0xC and 0xD\n\nNote: This register is only available in MIPI interface version 2 of ISP \n\n\n\n \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_gen_short_c_d::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_gen_short_c_d`]
module"]
#[doc(alias = "MIPI_GEN_SHORT_C_D")]
pub type MipiGenShortCD = crate::Reg<mipi_gen_short_c_d::MipiGenShortCDSpec>;
#[doc = "data field for generic short packets of data type 0xC and 0xD\n\nNote: This register is only available in MIPI interface version 2 of ISP \n\n\n\n \n\n\n\n"]
pub mod mipi_gen_short_c_d;
#[doc = "MIPI_GEN_SHORT_E_F (r) register accessor: data field for generic short packets of data type 0xE and 0xF\n\nNote: This register is only available in MIPI interface version 2 of ISP \n\n\n\n \n\n\n\n \n\n\n\n \n\n\n\n \n\n\n\n \n\n\n\n \n\n \n\n\n\nThis is the control register for AF measurement unit Reset value: 0000'0000H \n\n \n\n\n\nAddress: ISP_AFM_BASE + 0000H Mode : rw \n\n\n\nBit\n\n\n\ns \n\n\n\nName Description \n\n\n\n31:\n\n\n\n1 \n\n\n\n--- unused \n\n\n\n0 afm_en AF measurement enable \n\n\n\n0: AF measurement is \n\n\n\ndisabled 1: AF \n\n\n\nmeasurement is enabled \n\n\n\nWriting a 1 to this register starts a new measurement \n\n\n\nand resets the afm_fin (measurement finished) interrupt \n\n\n\nto 0. \n\n\n\nAs long as the afm_en is 1, the AFM unit \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_gen_short_e_f::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_gen_short_e_f`]
module"]
#[doc(alias = "MIPI_GEN_SHORT_E_F")]
pub type MipiGenShortEF = crate::Reg<mipi_gen_short_e_f::MipiGenShortEFSpec>;
#[doc = "data field for generic short packets of data type 0xE and 0xF\n\nNote: This register is only available in MIPI interface version 2 of ISP \n\n\n\n \n\n\n\n \n\n\n\n \n\n\n\n \n\n\n\n \n\n\n\n \n\n \n\n\n\nThis is the control register for AF measurement unit Reset value: 0000'0000H \n\n \n\n\n\nAddress: ISP_AFM_BASE + 0000H Mode : rw \n\n\n\nBit\n\n\n\ns \n\n\n\nName Description \n\n\n\n31:\n\n\n\n1 \n\n\n\n--- unused \n\n\n\n0 afm_en AF measurement enable \n\n\n\n0: AF measurement is \n\n\n\ndisabled 1: AF \n\n\n\nmeasurement is enabled \n\n\n\nWriting a 1 to this register starts a new measurement \n\n\n\nand resets the afm_fin (measurement finished) interrupt \n\n\n\nto 0. \n\n\n\nAs long as the afm_en is 1, the AFM unit \n\n"]
pub mod mipi_gen_short_e_f;
#[doc = "AFM_LT_A (rw) register accessor: Top Left corner of measure window A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afm_lt_a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afm_lt_a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afm_lt_a`]
module"]
#[doc(alias = "AFM_LT_A")]
pub type AfmLtA = crate::Reg<afm_lt_a::AfmLtASpec>;
#[doc = "Top Left corner of measure window A"]
pub mod afm_lt_a;
#[doc = "AFM_RB_A (rw) register accessor: Bottom right corner of measure window A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afm_rb_a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afm_rb_a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afm_rb_a`]
module"]
#[doc(alias = "AFM_RB_A")]
pub type AfmRbA = crate::Reg<afm_rb_a::AfmRbASpec>;
#[doc = "Bottom right corner of measure window A"]
pub mod afm_rb_a;
#[doc = "AFM_LT_B (rw) register accessor: Top left corner of measure window B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afm_lt_b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afm_lt_b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afm_lt_b`]
module"]
#[doc(alias = "AFM_LT_B")]
pub type AfmLtB = crate::Reg<afm_lt_b::AfmLtBSpec>;
#[doc = "Top left corner of measure window B"]
pub mod afm_lt_b;
#[doc = "AFM_RB_B (rw) register accessor: Bottom right corner of measure window B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afm_rb_b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afm_rb_b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afm_rb_b`]
module"]
#[doc(alias = "AFM_RB_B")]
pub type AfmRbB = crate::Reg<afm_rb_b::AfmRbBSpec>;
#[doc = "Bottom right corner of measure window B"]
pub mod afm_rb_b;
#[doc = "AFM_LT_C (rw) register accessor: Top left corner of measure window C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afm_lt_c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afm_lt_c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afm_lt_c`]
module"]
#[doc(alias = "AFM_LT_C")]
pub type AfmLtC = crate::Reg<afm_lt_c::AfmLtCSpec>;
#[doc = "Top left corner of measure window C"]
pub mod afm_lt_c;
#[doc = "AFM_RB_C (rw) register accessor: Bottom right corner of measure window C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afm_rb_c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afm_rb_c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afm_rb_c`]
module"]
#[doc(alias = "AFM_RB_C")]
pub type AfmRbC = crate::Reg<afm_rb_c::AfmRbCSpec>;
#[doc = "Bottom right corner of measure window C"]
pub mod afm_rb_c;
#[doc = "AFM_THRES (rw) register accessor: Threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afm_thres::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afm_thres::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afm_thres`]
module"]
#[doc(alias = "AFM_THRES")]
pub type AfmThres = crate::Reg<afm_thres::AfmThresSpec>;
#[doc = "Threshold register"]
pub mod afm_thres;
#[doc = "AFM_VAR_SHIFT (rw) register accessor: Variable shift register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afm_var_shift::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afm_var_shift::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afm_var_shift`]
module"]
#[doc(alias = "AFM_VAR_SHIFT")]
pub type AfmVarShift = crate::Reg<afm_var_shift::AfmVarShiftSpec>;
#[doc = "Variable shift register"]
pub mod afm_var_shift;
#[doc = "AFM_SUM_A (r) register accessor: Sharpness Value Status Register of Window A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afm_sum_a::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afm_sum_a`]
module"]
#[doc(alias = "AFM_SUM_A")]
pub type AfmSumA = crate::Reg<afm_sum_a::AfmSumASpec>;
#[doc = "Sharpness Value Status Register of Window A"]
pub mod afm_sum_a;
#[doc = "AFM_SUM_B (r) register accessor: Sharpness Value Status Register of Window B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afm_sum_b::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afm_sum_b`]
module"]
#[doc(alias = "AFM_SUM_B")]
pub type AfmSumB = crate::Reg<afm_sum_b::AfmSumBSpec>;
#[doc = "Sharpness Value Status Register of Window B"]
pub mod afm_sum_b;
#[doc = "AFM_SUM_C (r) register accessor: Sharpness Value Status Register of Window C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afm_sum_c::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afm_sum_c`]
module"]
#[doc(alias = "AFM_SUM_C")]
pub type AfmSumC = crate::Reg<afm_sum_c::AfmSumCSpec>;
#[doc = "Sharpness Value Status Register of Window C"]
pub mod afm_sum_c;
#[doc = "AFM_LUM_A (r) register accessor: Luminance Value Status Register of Window A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afm_lum_a::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afm_lum_a`]
module"]
#[doc(alias = "AFM_LUM_A")]
pub type AfmLumA = crate::Reg<afm_lum_a::AfmLumASpec>;
#[doc = "Luminance Value Status Register of Window A"]
pub mod afm_lum_a;
#[doc = "AFM_LUM_B (r) register accessor: Luminance Value Status Register of Window B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afm_lum_b::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afm_lum_b`]
module"]
#[doc(alias = "AFM_LUM_B")]
pub type AfmLumB = crate::Reg<afm_lum_b::AfmLumBSpec>;
#[doc = "Luminance Value Status Register of Window B"]
pub mod afm_lum_b;
#[doc = "AFM_LUM_C (r) register accessor: Luminance Value Status Register of Window C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afm_lum_c::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afm_lum_c`]
module"]
#[doc(alias = "AFM_LUM_C")]
pub type AfmLumC = crate::Reg<afm_lum_c::AfmLumCSpec>;
#[doc = "Luminance Value Status Register of Window C"]
pub mod afm_lum_c;
#[doc = "LSC_CTRL (rw) register accessor: Lens shade control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsc_ctrl`]
module"]
#[doc(alias = "LSC_CTRL")]
pub type LscCtrl = crate::Reg<lsc_ctrl::LscCtrlSpec>;
#[doc = "Lens shade control"]
pub mod lsc_ctrl;
#[doc = "LSC_R_TABLE_ADDR (rw) register accessor: Table RAM Address for red component\n\nNote: The table values are written into an internal RAM. The RAM Address is generated per \n\nauto- increment. The tables values will be read back by a continuous read access to the \n\ncorresponding register. The read address is auto-incremented for each read access to that \n\nregister and is reset to a specific value by a write access to the ISP_LSC_TABLE_ADDR \n\nregister. \n\n\n\nTable set 0 access by SW at table address 0...152. Table set 1 access at table address \n\n153...305. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_r_table_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_r_table_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsc_r_table_addr`]
module"]
#[doc(alias = "LSC_R_TABLE_ADDR")]
pub type LscRTableAddr = crate::Reg<lsc_r_table_addr::LscRTableAddrSpec>;
#[doc = "Table RAM Address for red component\n\nNote: The table values are written into an internal RAM. The RAM Address is generated per \n\nauto- increment. The tables values will be read back by a continuous read access to the \n\ncorresponding register. The read address is auto-incremented for each read access to that \n\nregister and is reset to a specific value by a write access to the ISP_LSC_TABLE_ADDR \n\nregister. \n\n\n\nTable set 0 access by SW at table address 0...152. Table set 1 access at table address \n\n153...305. \n\n\n\n \n\n"]
pub mod lsc_r_table_addr;
#[doc = "LSC_GR_TABLE_ADDR (rw) register accessor: Table RAM Address for green (red) component\n\nNote: MKOE tbc: Orignial register mode was rwh which is no longer supported with new \n\nversion of SIG-> rwhh Table set 0 access by SW at table address 0...153. Table set 1 access at \n\ntable address 154...307. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_gr_table_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_gr_table_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsc_gr_table_addr`]
module"]
#[doc(alias = "LSC_GR_TABLE_ADDR")]
pub type LscGrTableAddr = crate::Reg<lsc_gr_table_addr::LscGrTableAddrSpec>;
#[doc = "Table RAM Address for green (red) component\n\nNote: MKOE tbc: Orignial register mode was rwh which is no longer supported with new \n\nversion of SIG-> rwhh Table set 0 access by SW at table address 0...153. Table set 1 access at \n\ntable address 154...307. \n\n\n\n \n\n"]
pub mod lsc_gr_table_addr;
#[doc = "LSC_B_TABLE_ADDR (rw) register accessor: Table RAM Address for blue component\n\nNote: MKOE tbc: Orignial register mode was rwh which is no longer supported with new \n\nversion of SIG-> rwhh Table set 0 access by SW at table address 0...153. Table set 1 access at \n\ntable address 154...307. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_b_table_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_b_table_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsc_b_table_addr`]
module"]
#[doc(alias = "LSC_B_TABLE_ADDR")]
pub type LscBTableAddr = crate::Reg<lsc_b_table_addr::LscBTableAddrSpec>;
#[doc = "Table RAM Address for blue component\n\nNote: MKOE tbc: Orignial register mode was rwh which is no longer supported with new \n\nversion of SIG-> rwhh Table set 0 access by SW at table address 0...153. Table set 1 access at \n\ntable address 154...307. \n\n\n\n \n\n"]
pub mod lsc_b_table_addr;
#[doc = "LSC_GB_TABLE_ADDR (rw) register accessor: Table RAM Address for green (blue) component\n\nNote: MKOE tbc: Orignial register mode was rwh which is no longer supported with new \n\nversion of SIG-> rwhh Table set 0 access by SW at table address 0...153. Table set 1 access at \n\ntable address 154...307. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_gb_table_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_gb_table_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsc_gb_table_addr`]
module"]
#[doc(alias = "LSC_GB_TABLE_ADDR")]
pub type LscGbTableAddr = crate::Reg<lsc_gb_table_addr::LscGbTableAddrSpec>;
#[doc = "Table RAM Address for green (blue) component\n\nNote: MKOE tbc: Orignial register mode was rwh which is no longer supported with new \n\nversion of SIG-> rwhh Table set 0 access by SW at table address 0...153. Table set 1 access at \n\ntable address 154...307. \n\n\n\n \n\n"]
pub mod lsc_gb_table_addr;
#[doc = "LSC_R_TABLE_DATA (rw) register accessor: Sample table red\n\nNote: The programmed sample value is immediately written into the RAM. The RAM \n\naddress is generated per auto-increment. The parameter RAMs for Lens Shade Correction and \n\nBad Pixel Correction can only be programmed, if the RGB Bayer path is switched on via \n\nISP_CTRL register (ISP_MODE bits). \n\n\n\nTable set 0 access by SW at table address 0...153. Table set 1 access at table address \n\n\n\n154...307.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_r_table_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_r_table_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsc_r_table_data`]
module"]
#[doc(alias = "LSC_R_TABLE_DATA")]
pub type LscRTableData = crate::Reg<lsc_r_table_data::LscRTableDataSpec>;
#[doc = "Sample table red\n\nNote: The programmed sample value is immediately written into the RAM. The RAM \n\naddress is generated per auto-increment. The parameter RAMs for Lens Shade Correction and \n\nBad Pixel Correction can only be programmed, if the RGB Bayer path is switched on via \n\nISP_CTRL register (ISP_MODE bits). \n\n\n\nTable set 0 access by SW at table address 0...153. Table set 1 access at table address \n\n\n\n154...307."]
pub mod lsc_r_table_data;
#[doc = "LSC_GR_TABLE_DATA (rw) register accessor: Sample table green (red)\n\nNote: The programmed sample value is immediately written into the RAM. The RAM \n\naddress is generated per auto-increment. The parameter RAMs for Lens Shade Correction and \n\nBad Pixel Correction can only be programmed, if the RGB Bayer path is switched on via \n\nISP_CTRL register (ISP_MODE bits). \n\n\n\nTable set 0 access by SW at table address 0...153. Table set 1 access at table address \n\n\n\n154...307. \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_gr_table_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_gr_table_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsc_gr_table_data`]
module"]
#[doc(alias = "LSC_GR_TABLE_DATA")]
pub type LscGrTableData = crate::Reg<lsc_gr_table_data::LscGrTableDataSpec>;
#[doc = "Sample table green (red)\n\nNote: The programmed sample value is immediately written into the RAM. The RAM \n\naddress is generated per auto-increment. The parameter RAMs for Lens Shade Correction and \n\nBad Pixel Correction can only be programmed, if the RGB Bayer path is switched on via \n\nISP_CTRL register (ISP_MODE bits). \n\n\n\nTable set 0 access by SW at table address 0...153. Table set 1 access at table address \n\n\n\n154...307. \n\n\n\n"]
pub mod lsc_gr_table_data;
#[doc = "LSC_B_TABLE_DATA (rw) register accessor: Sample table blue\n\nNote: The programmed sample value is immediately written into the RAM. The \n\n\n\nRAM address is generated per auto-increment. The parameter RAMs for Lens Shade \n\n\n\nCorrection and Bad Pixel Correction can only be programmed, if the RGB Bayer path is \n\n\n\nswitched on via ISP_CTRL register (ISP_MODE bits). \n\n\n\nTable set 0 access by SW at table address 0...153. Table set 1 access at table address \n\n\n\n154...307. \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_b_table_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_b_table_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsc_b_table_data`]
module"]
#[doc(alias = "LSC_B_TABLE_DATA")]
pub type LscBTableData = crate::Reg<lsc_b_table_data::LscBTableDataSpec>;
#[doc = "Sample table blue\n\nNote: The programmed sample value is immediately written into the RAM. The \n\n\n\nRAM address is generated per auto-increment. The parameter RAMs for Lens Shade \n\n\n\nCorrection and Bad Pixel Correction can only be programmed, if the RGB Bayer path is \n\n\n\nswitched on via ISP_CTRL register (ISP_MODE bits). \n\n\n\nTable set 0 access by SW at table address 0...153. Table set 1 access at table address \n\n\n\n154...307. \n\n\n\n"]
pub mod lsc_b_table_data;
#[doc = "LSC_GB_TABLE_DATA (rw) register accessor: Sample table green (blue)\n\nNote: The programmed sample value is immediately written into the RAM. The RAM \n\naddress is generated per auto-increment. The parameter RAMs for Lens Shade Correction and \n\nBad Pixel Correction can only be programmed, if the RGB Bayer path is switched on via \n\nISP_CTRL register (ISP_MODE bits).Table set 0 access by SW at table address 0...153. Table \n\nset 1 access at table address 154...307. \n\n\n\n \n\n\n\n \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_gb_table_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_gb_table_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsc_gb_table_data`]
module"]
#[doc(alias = "LSC_GB_TABLE_DATA")]
pub type LscGbTableData = crate::Reg<lsc_gb_table_data::LscGbTableDataSpec>;
#[doc = "Sample table green (blue)\n\nNote: The programmed sample value is immediately written into the RAM. The RAM \n\naddress is generated per auto-increment. The parameter RAMs for Lens Shade Correction and \n\nBad Pixel Correction can only be programmed, if the RGB Bayer path is switched on via \n\nISP_CTRL register (ISP_MODE bits).Table set 0 access by SW at table address 0...153. Table \n\nset 1 access at table address 154...307. \n\n\n\n \n\n\n\n \n\n\n\n"]
pub mod lsc_gb_table_data;
#[doc = "LSC_XGRAD_01 (rw) register accessor: Gradient table x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_xgrad_01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_xgrad_01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsc_xgrad_01`]
module"]
#[doc(alias = "LSC_XGRAD_01")]
pub type LscXgrad01 = crate::Reg<lsc_xgrad_01::LscXgrad01Spec>;
#[doc = "Gradient table x"]
pub mod lsc_xgrad_01;
#[doc = "LSC_XGRAD_23 (rw) register accessor: Gradient table x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_xgrad_23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_xgrad_23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsc_xgrad_23`]
module"]
#[doc(alias = "LSC_XGRAD_23")]
pub type LscXgrad23 = crate::Reg<lsc_xgrad_23::LscXgrad23Spec>;
#[doc = "Gradient table x"]
pub mod lsc_xgrad_23;
#[doc = "LSC_XGRAD_45 (rw) register accessor: Gradient table x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_xgrad_45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_xgrad_45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsc_xgrad_45`]
module"]
#[doc(alias = "LSC_XGRAD_45")]
pub type LscXgrad45 = crate::Reg<lsc_xgrad_45::LscXgrad45Spec>;
#[doc = "Gradient table x"]
pub mod lsc_xgrad_45;
#[doc = "LSC_XGRAD_67 (rw) register accessor: Gradient table x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_xgrad_67::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_xgrad_67::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsc_xgrad_67`]
module"]
#[doc(alias = "LSC_XGRAD_67")]
pub type LscXgrad67 = crate::Reg<lsc_xgrad_67::LscXgrad67Spec>;
#[doc = "Gradient table x"]
pub mod lsc_xgrad_67;
#[doc = "LSC_YGRAD_01 (rw) register accessor: Gradient table y\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_ygrad_01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_ygrad_01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsc_ygrad_01`]
module"]
#[doc(alias = "LSC_YGRAD_01")]
pub type LscYgrad01 = crate::Reg<lsc_ygrad_01::LscYgrad01Spec>;
#[doc = "Gradient table y"]
pub mod lsc_ygrad_01;
#[doc = "LSC_YGRAD_23 (rw) register accessor: Gradient table y\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_ygrad_23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_ygrad_23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsc_ygrad_23`]
module"]
#[doc(alias = "LSC_YGRAD_23")]
pub type LscYgrad23 = crate::Reg<lsc_ygrad_23::LscYgrad23Spec>;
#[doc = "Gradient table y"]
pub mod lsc_ygrad_23;
#[doc = "LSC_YGRAD_45 (rw) register accessor: Gradient table y\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_ygrad_45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_ygrad_45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsc_ygrad_45`]
module"]
#[doc(alias = "LSC_YGRAD_45")]
pub type LscYgrad45 = crate::Reg<lsc_ygrad_45::LscYgrad45Spec>;
#[doc = "Gradient table y"]
pub mod lsc_ygrad_45;
#[doc = "LSC_YGRAD_67 (rw) register accessor: Gradient table y\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_ygrad_67::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_ygrad_67::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsc_ygrad_67`]
module"]
#[doc(alias = "LSC_YGRAD_67")]
pub type LscYgrad67 = crate::Reg<lsc_ygrad_67::LscYgrad67Spec>;
#[doc = "Gradient table y"]
pub mod lsc_ygrad_67;
#[doc = "LSC_XSIZE_01 (rw) register accessor: Size table\n\nNote: The sector size in x-direction must be greater than 12 pixels. The sum of the sector \n\nsizes in x- direction must be 'picture width / 2'. The sum of the sector sizes in y-direction must \n\nbe 'picture height / 2'. No interrupt is generated if above requirements are not fulfilled and the \n\nbehaviour of the hardware cannot be predicted. \n\n\n\nThe sector size in x-direction was defined to be 9 bits for preliminary ISP versions. \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_xsize_01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_xsize_01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsc_xsize_01`]
module"]
#[doc(alias = "LSC_XSIZE_01")]
pub type LscXsize01 = crate::Reg<lsc_xsize_01::LscXsize01Spec>;
#[doc = "Size table\n\nNote: The sector size in x-direction must be greater than 12 pixels. The sum of the sector \n\nsizes in x- direction must be 'picture width / 2'. The sum of the sector sizes in y-direction must \n\nbe 'picture height / 2'. No interrupt is generated if above requirements are not fulfilled and the \n\nbehaviour of the hardware cannot be predicted. \n\n\n\nThe sector size in x-direction was defined to be 9 bits for preliminary ISP versions. \n\n\n\n"]
pub mod lsc_xsize_01;
#[doc = "LSC_XSIZE_23 (rw) register accessor: Size table\n\nNote: minimum sector size is 10 in x direction \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_xsize_23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_xsize_23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsc_xsize_23`]
module"]
#[doc(alias = "LSC_XSIZE_23")]
pub type LscXsize23 = crate::Reg<lsc_xsize_23::LscXsize23Spec>;
#[doc = "Size table\n\nNote: minimum sector size is 10 in x direction \n\n\n\n \n\n"]
pub mod lsc_xsize_23;
#[doc = "LSC_XSIZE_45 (rw) register accessor: Size table\n\nNote: minimum sector size is 10 in x direction \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_xsize_45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_xsize_45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsc_xsize_45`]
module"]
#[doc(alias = "LSC_XSIZE_45")]
pub type LscXsize45 = crate::Reg<lsc_xsize_45::LscXsize45Spec>;
#[doc = "Size table\n\nNote: minimum sector size is 10 in x direction \n\n\n\n \n\n"]
pub mod lsc_xsize_45;
#[doc = "LSC_XSIZE_67 (rw) register accessor: Size table\n\nNote: minimum sector size is 10 in x direction \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_xsize_67::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_xsize_67::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsc_xsize_67`]
module"]
#[doc(alias = "LSC_XSIZE_67")]
pub type LscXsize67 = crate::Reg<lsc_xsize_67::LscXsize67Spec>;
#[doc = "Size table\n\nNote: minimum sector size is 10 in x direction \n\n\n\n"]
pub mod lsc_xsize_67;
#[doc = "LSC_YSIZE_01 (rw) register accessor: Size table\n\nNote: minimum sector size is 8 in y direction. \n\n\n\nThe sector size in y-direction was defined to be 9 bits for preliminary ISP versions. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_ysize_01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_ysize_01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsc_ysize_01`]
module"]
#[doc(alias = "LSC_YSIZE_01")]
pub type LscYsize01 = crate::Reg<lsc_ysize_01::LscYsize01Spec>;
#[doc = "Size table\n\nNote: minimum sector size is 8 in y direction. \n\n\n\nThe sector size in y-direction was defined to be 9 bits for preliminary ISP versions. \n\n\n\n \n\n"]
pub mod lsc_ysize_01;
#[doc = "LSC_YSIZE_23 (rw) register accessor: Size table\n\nNote: minimum sector size is 8 in y direction \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_ysize_23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_ysize_23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsc_ysize_23`]
module"]
#[doc(alias = "LSC_YSIZE_23")]
pub type LscYsize23 = crate::Reg<lsc_ysize_23::LscYsize23Spec>;
#[doc = "Size table\n\nNote: minimum sector size is 8 in y direction \n\n\n\n \n\n"]
pub mod lsc_ysize_23;
#[doc = "LSC_YSIZE_45 (rw) register accessor: Size table\n\nNote: minimum sector size is 8 in y direction \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_ysize_45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_ysize_45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsc_ysize_45`]
module"]
#[doc(alias = "LSC_YSIZE_45")]
pub type LscYsize45 = crate::Reg<lsc_ysize_45::LscYsize45Spec>;
#[doc = "Size table\n\nNote: minimum sector size is 8 in y direction \n\n\n\n"]
pub mod lsc_ysize_45;
#[doc = "LSC_YSIZE_67 (rw) register accessor: Size table\n\nNote: minimum sector size is 8 in y direction \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_ysize_67::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_ysize_67::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsc_ysize_67`]
module"]
#[doc(alias = "LSC_YSIZE_67")]
pub type LscYsize67 = crate::Reg<lsc_ysize_67::LscYsize67Spec>;
#[doc = "Size table\n\nNote: minimum sector size is 8 in y direction \n\n\n\n \n\n"]
pub mod lsc_ysize_67;
#[doc = "LSC_TABLE_SEL (rw) register accessor: Lens shade table set selection\n\nNote: Table set 0 access by SW at table address 0...153. Table set 1 access at table \n\naddress 154...307. For LSC4_MSZ the table set 1 is physically not available: \n\nISP_LSC_TABLE_SEL shall always be 0 for this HW configuration. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_table_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_table_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsc_table_sel`]
module"]
#[doc(alias = "LSC_TABLE_SEL")]
pub type LscTableSel = crate::Reg<lsc_table_sel::LscTableSelSpec>;
#[doc = "Lens shade table set selection\n\nNote: Table set 0 access by SW at table address 0...153. Table set 1 access at table \n\naddress 154...307. For LSC4_MSZ the table set 1 is physically not available: \n\nISP_LSC_TABLE_SEL shall always be 0 for this HW configuration. \n\n\n\n \n\n"]
pub mod lsc_table_sel;
#[doc = "LSC_STATUS (r) register accessor: Lens shade status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsc_status`]
module"]
#[doc(alias = "LSC_STATUS")]
pub type LscStatus = crate::Reg<lsc_status::LscStatusSpec>;
#[doc = "Lens shade status"]
pub mod lsc_status;
#[doc = "IS_CTRL (rw) register accessor: Image Stabilization Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`is_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`is_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@is_ctrl`]
module"]
#[doc(alias = "IS_CTRL")]
pub type IsCtrl = crate::Reg<is_ctrl::IsCtrlSpec>;
#[doc = "Image Stabilization Control Register"]
pub mod is_ctrl;
#[doc = "IS_RECENTER (rw) register accessor: Recenter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`is_recenter::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`is_recenter::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@is_recenter`]
module"]
#[doc(alias = "IS_RECENTER")]
pub type IsRecenter = crate::Reg<is_recenter::IsRecenterSpec>;
#[doc = "Recenter register"]
pub mod is_recenter;
#[doc = "IS_H_OFFS (rw) register accessor: Horizontal offset of output window\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`is_h_offs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`is_h_offs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@is_h_offs`]
module"]
#[doc(alias = "IS_H_OFFS")]
pub type IsHOffs = crate::Reg<is_h_offs::IsHOffsSpec>;
#[doc = "Horizontal offset of output window"]
pub mod is_h_offs;
#[doc = "IS_V_OFFS (rw) register accessor: Vertical offset of output window\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`is_v_offs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`is_v_offs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@is_v_offs`]
module"]
#[doc(alias = "IS_V_OFFS")]
pub type IsVOffs = crate::Reg<is_v_offs::IsVOffsSpec>;
#[doc = "Vertical offset of output window"]
pub mod is_v_offs;
#[doc = "IS_H_SIZE (rw) register accessor: Output horizontal picture size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`is_h_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`is_h_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@is_h_size`]
module"]
#[doc(alias = "IS_H_SIZE")]
pub type IsHSize = crate::Reg<is_h_size::IsHSizeSpec>;
#[doc = "Output horizontal picture size"]
pub mod is_h_size;
#[doc = "IS_V_SIZE (rw) register accessor: Output vertical picture size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`is_v_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`is_v_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@is_v_size`]
module"]
#[doc(alias = "IS_V_SIZE")]
pub type IsVSize = crate::Reg<is_v_size::IsVSizeSpec>;
#[doc = "Output vertical picture size"]
pub mod is_v_size;
#[doc = "IS_MAX_DX (rw) register accessor: Maximum Horizontal Displacement\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`is_max_dx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`is_max_dx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@is_max_dx`]
module"]
#[doc(alias = "IS_MAX_DX")]
pub type IsMaxDx = crate::Reg<is_max_dx::IsMaxDxSpec>;
#[doc = "Maximum Horizontal Displacement"]
pub mod is_max_dx;
#[doc = "IS_MAX_DY (rw) register accessor: Maximum Vertical Displacement\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`is_max_dy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`is_max_dy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@is_max_dy`]
module"]
#[doc(alias = "IS_MAX_DY")]
pub type IsMaxDy = crate::Reg<is_max_dy::IsMaxDySpec>;
#[doc = "Maximum Vertical Displacement"]
pub mod is_max_dy;
#[doc = "IS_DISPLACE (rw) register accessor: Camera displacement\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`is_displace::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`is_displace::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@is_displace`]
module"]
#[doc(alias = "IS_DISPLACE")]
pub type IsDisplace = crate::Reg<is_displace::IsDisplaceSpec>;
#[doc = "Camera displacement"]
pub mod is_displace;
#[doc = "IS_H_OFFS_SHD (r) register accessor: current horizontal offset of output window (shadow register)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`is_h_offs_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@is_h_offs_shd`]
module"]
#[doc(alias = "IS_H_OFFS_SHD")]
pub type IsHOffsShd = crate::Reg<is_h_offs_shd::IsHOffsShdSpec>;
#[doc = "current horizontal offset of output window (shadow register)"]
pub mod is_h_offs_shd;
#[doc = "IS_V_OFFS_SHD (r) register accessor: current vertical offset of output window (shadow register)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`is_v_offs_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@is_v_offs_shd`]
module"]
#[doc(alias = "IS_V_OFFS_SHD")]
pub type IsVOffsShd = crate::Reg<is_v_offs_shd::IsVOffsShdSpec>;
#[doc = "current vertical offset of output window (shadow register)"]
pub mod is_v_offs_shd;
#[doc = "IS_H_SIZE_SHD (r) register accessor: current output horizontal picture size (shadow register)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`is_h_size_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@is_h_size_shd`]
module"]
#[doc(alias = "IS_H_SIZE_SHD")]
pub type IsHSizeShd = crate::Reg<is_h_size_shd::IsHSizeShdSpec>;
#[doc = "current output horizontal picture size (shadow register)"]
pub mod is_h_size_shd;
#[doc = "IS_V_SIZE_SHD (r) register accessor: current output vertical picture size (shadow register)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`is_v_size_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@is_v_size_shd`]
module"]
#[doc(alias = "IS_V_SIZE_SHD")]
pub type IsVSizeShd = crate::Reg<is_v_size_shd::IsVSizeShdSpec>;
#[doc = "current output vertical picture size (shadow register)"]
pub mod is_v_size_shd;
#[doc = "HIST_PROP (rw) register accessor: Histogram properties\n\nNote: If RGB combined mode is used, then the 3 color components are sampled one \n\n\n\nafter the other. The software has to assure that all 3 color components are inside the \n\n\n\nselected window. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_prop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_prop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_prop`]
module"]
#[doc(alias = "HIST_PROP")]
pub type HistProp = crate::Reg<hist_prop::HistPropSpec>;
#[doc = "Histogram properties\n\nNote: If RGB combined mode is used, then the 3 color components are sampled one \n\n\n\nafter the other. The software has to assure that all 3 color components are inside the \n\n\n\nselected window. \n\n\n\n \n\n"]
pub mod hist_prop;
#[doc = "HIST_H_OFFS (rw) register accessor: Histogram window horizontal offset for first window of \n\n\n\n25 sub- windows\n\nNote: histogram measurement is done in 25 sub-windows like the exposure \n\n\n\nmeasurement, if histogram version 3 is implemented. All earlier versions use just one \n\n\n\nwindow. \n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_h_offs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_h_offs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_h_offs`]
module"]
#[doc(alias = "HIST_H_OFFS")]
pub type HistHOffs = crate::Reg<hist_h_offs::HistHOffsSpec>;
#[doc = "Histogram window horizontal offset for first window of \n\n\n\n25 sub- windows\n\nNote: histogram measurement is done in 25 sub-windows like the exposure \n\n\n\nmeasurement, if histogram version 3 is implemented. All earlier versions use just one \n\n\n\nwindow. \n\n \n\n"]
pub mod hist_h_offs;
#[doc = "HIST_V_OFFS (rw) register accessor: Histogram window vertical offset for first window of 25 sub-windows\n\nNote: histogram measurement is done in 25 sub-windows like the exposure \n\nmeasurement, if histogram version 3 is implemented. All earlier versions use just one window. \n\n\n\n \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_v_offs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_v_offs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_v_offs`]
module"]
#[doc(alias = "HIST_V_OFFS")]
pub type HistVOffs = crate::Reg<hist_v_offs::HistVOffsSpec>;
#[doc = "Histogram window vertical offset for first window of 25 sub-windows\n\nNote: histogram measurement is done in 25 sub-windows like the exposure \n\nmeasurement, if histogram version 3 is implemented. All earlier versions use just one window. \n\n\n\n \n\n\n\n"]
pub mod hist_v_offs;
#[doc = "HIST_H_SIZE (rw) register accessor: Horizontal (sub-)window size\n\nNote: hist_h_offset + hist_h_size x 5 should be less than or equal to the horizontal size \n\n\n\nof the picture, if histogram version 3 is implemented. Otherwise hist_h_size is the horizontal \n\n\n\nsize of the measurement window in pixels. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_h_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_h_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_h_size`]
module"]
#[doc(alias = "HIST_H_SIZE")]
pub type HistHSize = crate::Reg<hist_h_size::HistHSizeSpec>;
#[doc = "Horizontal (sub-)window size\n\nNote: hist_h_offset + hist_h_size x 5 should be less than or equal to the horizontal size \n\n\n\nof the picture, if histogram version 3 is implemented. Otherwise hist_h_size is the horizontal \n\n\n\nsize of the measurement window in pixels. \n\n\n\n \n\n"]
pub mod hist_h_size;
#[doc = "HIST_V_SIZE (rw) register accessor: Vertical (sub-)window size\n\nNote: hist_v_offset + hist_v_size x 5 should be less than or equal to the vertical size \n\n\n\nof the picture, if histogram version 3 is implemented. Otherwise hist_v_size is the vertical \n\n\n\nsize of the measurement window in lines. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_v_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_v_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_v_size`]
module"]
#[doc(alias = "HIST_V_SIZE")]
pub type HistVSize = crate::Reg<hist_v_size::HistVSizeSpec>;
#[doc = "Vertical (sub-)window size\n\nNote: hist_v_offset + hist_v_size x 5 should be less than or equal to the vertical size \n\n\n\nof the picture, if histogram version 3 is implemented. Otherwise hist_v_size is the vertical \n\n\n\nsize of the measurement window in lines. \n\n\n\n \n\n"]
pub mod hist_v_size;
#[doc = "HIST_BIN (r) register accessor: histogram measurement result bin n (n=0..15)\n\nNote: MKOE tbc: Orignial register mode was rh which is no longer supported with new \n\nversion of SIG -> r \n\n\n\n \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_bin::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_bin`]
module"]
#[doc(alias = "HIST_BIN")]
pub type HistBin = crate::Reg<hist_bin::HistBinSpec>;
#[doc = "histogram measurement result bin n (n=0..15)\n\nNote: MKOE tbc: Orignial register mode was rh which is no longer supported with new \n\nversion of SIG -> r \n\n\n\n \n\n\n\n"]
pub mod hist_bin;
#[doc = "HIST_WEIGHT_00TO30 (rw) register accessor: Weighting factor for sub-windows\n\nNote: Allowed value range for weight factor is 0 to 16. The resulting weight is \n\nregister_value / 16. The host software has to limit the register value for each factor to 16. \n\nWeight registers are available, if histogram version 3 is implemented. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_weight_00to30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_weight_00to30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_weight_00to30`]
module"]
#[doc(alias = "HIST_WEIGHT_00TO30")]
pub type HistWeight00to30 = crate::Reg<hist_weight_00to30::HistWeight00to30Spec>;
#[doc = "Weighting factor for sub-windows\n\nNote: Allowed value range for weight factor is 0 to 16. The resulting weight is \n\nregister_value / 16. The host software has to limit the register value for each factor to 16. \n\nWeight registers are available, if histogram version 3 is implemented. \n\n\n\n \n\n"]
pub mod hist_weight_00to30;
#[doc = "HIST_WEIGHT_40TO21 (rw) register accessor: Weighting factor for sub-windows\n\nNote: Allowed value range for weight factor is 0 to 16. The resulting weight is \n\n\n\nregister_value / 16. The host software has to limit the register value for each factor to 16. \n\n\n\nWeight registers are available, if histogram version 3 is implemented. \n\n\n\n \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_weight_40to21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_weight_40to21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_weight_40to21`]
module"]
#[doc(alias = "HIST_WEIGHT_40TO21")]
pub type HistWeight40to21 = crate::Reg<hist_weight_40to21::HistWeight40to21Spec>;
#[doc = "Weighting factor for sub-windows\n\nNote: Allowed value range for weight factor is 0 to 16. The resulting weight is \n\n\n\nregister_value / 16. The host software has to limit the register value for each factor to 16. \n\n\n\nWeight registers are available, if histogram version 3 is implemented. \n\n\n\n \n\n\n\n"]
pub mod hist_weight_40to21;
#[doc = "HIST_WEIGHT_31TO12 (rw) register accessor: Weighting factor for sub-windows\n\nNote: Allowed value range for weight factor is 0 to 16. The resulting weight is \n\nregister_value / 16. The host software has to limit the register value for each factor to 16. \n\nWeight registers are available, if histogram version 3 is implemented. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_weight_31to12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_weight_31to12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_weight_31to12`]
module"]
#[doc(alias = "HIST_WEIGHT_31TO12")]
pub type HistWeight31to12 = crate::Reg<hist_weight_31to12::HistWeight31to12Spec>;
#[doc = "Weighting factor for sub-windows\n\nNote: Allowed value range for weight factor is 0 to 16. The resulting weight is \n\nregister_value / 16. The host software has to limit the register value for each factor to 16. \n\nWeight registers are available, if histogram version 3 is implemented. \n\n\n\n \n\n"]
pub mod hist_weight_31to12;
#[doc = "HIST_WEIGHT_22TO03 (rw) register accessor: Weighting factor for sub-windows\n\nNote: Allowed value range for weight factor is 0 to 16. The resulting weight is \n\nregister_value / 16. The host software has to limit the register value for each factor to 16. \n\nWeight registers are available, if histogram version 3 is implemented. \n\n\n\n \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_weight_22to03::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_weight_22to03::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_weight_22to03`]
module"]
#[doc(alias = "HIST_WEIGHT_22TO03")]
pub type HistWeight22to03 = crate::Reg<hist_weight_22to03::HistWeight22to03Spec>;
#[doc = "Weighting factor for sub-windows\n\nNote: Allowed value range for weight factor is 0 to 16. The resulting weight is \n\nregister_value / 16. The host software has to limit the register value for each factor to 16. \n\nWeight registers are available, if histogram version 3 is implemented. \n\n\n\n \n\n\n\n"]
pub mod hist_weight_22to03;
#[doc = "HIST_WEIGHT_13TO43 (rw) register accessor: Weighting factor for sub-windows\n\nNote: Allowed value range for weight factor is 0 to 16. The resulting weight is \n\nregister_value / 16. The host software has to limit the register value for each factor to 16. \n\nWeight registers are available, if histogram version 3 is implemented. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_weight_13to43::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_weight_13to43::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_weight_13to43`]
module"]
#[doc(alias = "HIST_WEIGHT_13TO43")]
pub type HistWeight13to43 = crate::Reg<hist_weight_13to43::HistWeight13to43Spec>;
#[doc = "Weighting factor for sub-windows\n\nNote: Allowed value range for weight factor is 0 to 16. The resulting weight is \n\nregister_value / 16. The host software has to limit the register value for each factor to 16. \n\nWeight registers are available, if histogram version 3 is implemented. \n\n\n\n \n\n"]
pub mod hist_weight_13to43;
#[doc = "HIST_WEIGHT_04TO34 (rw) register accessor: Weighting factor for sub-windows\n\nNote: Allowed value range for weight factor is 0 to 16. The resulting weight is \n\nregister_value / 16. The host software has to limit the register value for each factor to 16. \n\nWeight registers are available, if histogram version 3 is implemented. \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_weight_04to34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_weight_04to34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_weight_04to34`]
module"]
#[doc(alias = "HIST_WEIGHT_04TO34")]
pub type HistWeight04to34 = crate::Reg<hist_weight_04to34::HistWeight04to34Spec>;
#[doc = "Weighting factor for sub-windows\n\nNote: Allowed value range for weight factor is 0 to 16. The resulting weight is \n\nregister_value / 16. The host software has to limit the register value for each factor to 16. \n\nWeight registers are available, if histogram version 3 is implemented. \n\n\n\n"]
pub mod hist_weight_04to34;
#[doc = "HIST_WEIGHT_44 (rw) register accessor: Weighting factor for sub-windows\n\nNote: Allowed value range for weight factor is 0 to 16. The resulting weight is \n\nregister_value / 16. The host software has to limit the register value for each factor to 16. \n\nWeight registers are available, if histogram version 3 is implemented. \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_weight_44::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_weight_44::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hist_weight_44`]
module"]
#[doc(alias = "HIST_WEIGHT_44")]
pub type HistWeight44 = crate::Reg<hist_weight_44::HistWeight44Spec>;
#[doc = "Weighting factor for sub-windows\n\nNote: Allowed value range for weight factor is 0 to 16. The resulting weight is \n\nregister_value / 16. The host software has to limit the register value for each factor to 16. \n\nWeight registers are available, if histogram version 3 is implemented. \n\n\n\n"]
pub mod hist_weight_44;
#[doc = "FILT_MODE (rw) register accessor: mode control register for the filter block\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filt_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filt_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filt_mode`]
module"]
#[doc(alias = "FILT_MODE")]
pub type FiltMode = crate::Reg<filt_mode::FiltModeSpec>;
#[doc = "mode control register for the filter block"]
pub mod filt_mode;
#[doc = "FILT_THRESH_BL0 (rw) register accessor: Blurring threshold 0\n\nNote: sum_grad is calculated by the texture detection unit as the sum of \n\n\n\nhorizontal and vertical gradients \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filt_thresh_bl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filt_thresh_bl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filt_thresh_bl0`]
module"]
#[doc(alias = "FILT_THRESH_BL0")]
pub type FiltThreshBl0 = crate::Reg<filt_thresh_bl0::FiltThreshBl0Spec>;
#[doc = "Blurring threshold 0\n\nNote: sum_grad is calculated by the texture detection unit as the sum of \n\n\n\nhorizontal and vertical gradients \n\n\n\n"]
pub mod filt_thresh_bl0;
#[doc = "FILT_THRESH_BL1 (rw) register accessor: Blurring threshold 1\n\nNote: sum_grad is calculated by the texture detection unit as the sum of horizontal and \n\nvertical gradients \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filt_thresh_bl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filt_thresh_bl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filt_thresh_bl1`]
module"]
#[doc(alias = "FILT_THRESH_BL1")]
pub type FiltThreshBl1 = crate::Reg<filt_thresh_bl1::FiltThreshBl1Spec>;
#[doc = "Blurring threshold 1\n\nNote: sum_grad is calculated by the texture detection unit as the sum of horizontal and \n\nvertical gradients \n\n\n\n"]
pub mod filt_thresh_bl1;
#[doc = "FILT_THRESH_SH0 (rw) register accessor: Sharpening threshold 0\n\nNote: sum_grad is calculated by the texture detection unit as the sum of horizontal and \n\nvertical gradients \n\n\n\n \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filt_thresh_sh0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filt_thresh_sh0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filt_thresh_sh0`]
module"]
#[doc(alias = "FILT_THRESH_SH0")]
pub type FiltThreshSh0 = crate::Reg<filt_thresh_sh0::FiltThreshSh0Spec>;
#[doc = "Sharpening threshold 0\n\nNote: sum_grad is calculated by the texture detection unit as the sum of horizontal and \n\nvertical gradients \n\n\n\n \n\n\n\n"]
pub mod filt_thresh_sh0;
#[doc = "FILT_THRESH_SH1 (rw) register accessor: Sharpening threshold 1\n\nNote: sum_grad is calculated by the texture detection unit as the sum of horizontal and \n\n\n\nvertical gradients \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filt_thresh_sh1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filt_thresh_sh1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filt_thresh_sh1`]
module"]
#[doc(alias = "FILT_THRESH_SH1")]
pub type FiltThreshSh1 = crate::Reg<filt_thresh_sh1::FiltThreshSh1Spec>;
#[doc = "Sharpening threshold 1\n\nNote: sum_grad is calculated by the texture detection unit as the sum of horizontal and \n\n\n\nvertical gradients \n\n\n\n \n\n"]
pub mod filt_thresh_sh1;
#[doc = "FILT_LUM_WEIGHT (rw) register accessor: Parameters for luminance weight function\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filt_lum_weight::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filt_lum_weight::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filt_lum_weight`]
module"]
#[doc(alias = "FILT_LUM_WEIGHT")]
pub type FiltLumWeight = crate::Reg<filt_lum_weight::FiltLumWeightSpec>;
#[doc = "Parameters for luminance weight function"]
pub mod filt_lum_weight;
#[doc = "FILT_FAC_SH1 (rw) register accessor: filter factor sharp1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filt_fac_sh1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filt_fac_sh1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filt_fac_sh1`]
module"]
#[doc(alias = "FILT_FAC_SH1")]
pub type FiltFacSh1 = crate::Reg<filt_fac_sh1::FiltFacSh1Spec>;
#[doc = "filter factor sharp1"]
pub mod filt_fac_sh1;
#[doc = "FILT_FAC_SH0 (rw) register accessor: filter factor sharp0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filt_fac_sh0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filt_fac_sh0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filt_fac_sh0`]
module"]
#[doc(alias = "FILT_FAC_SH0")]
pub type FiltFacSh0 = crate::Reg<filt_fac_sh0::FiltFacSh0Spec>;
#[doc = "filter factor sharp0"]
pub mod filt_fac_sh0;
#[doc = "FILT_FAC_MID (rw) register accessor: filter factor middle\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filt_fac_mid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filt_fac_mid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filt_fac_mid`]
module"]
#[doc(alias = "FILT_FAC_MID")]
pub type FiltFacMid = crate::Reg<filt_fac_mid::FiltFacMidSpec>;
#[doc = "filter factor middle"]
pub mod filt_fac_mid;
#[doc = "FILT_FAC_BL0 (rw) register accessor: Parameter for blur 0 filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filt_fac_bl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filt_fac_bl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filt_fac_bl0`]
module"]
#[doc(alias = "FILT_FAC_BL0")]
pub type FiltFacBl0 = crate::Reg<filt_fac_bl0::FiltFacBl0Spec>;
#[doc = "Parameter for blur 0 filter"]
pub mod filt_fac_bl0;
#[doc = "FILT_FAC_BL1 (rw) register accessor: Parameter for blur 1 filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filt_fac_bl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filt_fac_bl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filt_fac_bl1`]
module"]
#[doc(alias = "FILT_FAC_BL1")]
pub type FiltFacBl1 = crate::Reg<filt_fac_bl1::FiltFacBl1Spec>;
#[doc = "Parameter for blur 1 filter"]
pub mod filt_fac_bl1;
#[doc = "CAC_CTRL (rw) register accessor: Control register for chromatic aberration correction\n\nNote: Clipping behavior can be controlled by clip_mode bits. If no clipping occurs, because \n\ndisplacement is below the maximum correctable displacement, then it does not matter which \n\nmode is selected. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cac_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cac_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cac_ctrl`]
module"]
#[doc(alias = "CAC_CTRL")]
pub type CacCtrl = crate::Reg<cac_ctrl::CacCtrlSpec>;
#[doc = "Control register for chromatic aberration correction\n\nNote: Clipping behavior can be controlled by clip_mode bits. If no clipping occurs, because \n\ndisplacement is below the maximum correctable displacement, then it does not matter which \n\nmode is selected. \n\n\n\n \n\n"]
pub mod cac_ctrl;
#[doc = "CAC_COUNT_START (rw) register accessor: Preload values for CAC pixel and line counter\n\nNote: Reset value is valid for 8192 x 8192 image resolution with centered chromatic \n\naberration (no offset from image center). \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cac_count_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cac_count_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cac_count_start`]
module"]
#[doc(alias = "CAC_COUNT_START")]
pub type CacCountStart = crate::Reg<cac_count_start::CacCountStartSpec>;
#[doc = "Preload values for CAC pixel and line counter\n\nNote: Reset value is valid for 8192 x 8192 image resolution with centered chromatic \n\naberration (no offset from image center). \n\n\n\n \n\n"]
pub mod cac_count_start;
#[doc = "CAC_A (rw) register accessor: Linear Parameters for radial shift calculation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cac_a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cac_a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cac_a`]
module"]
#[doc(alias = "CAC_A")]
pub type CacA = crate::Reg<cac_a::CacASpec>;
#[doc = "Linear Parameters for radial shift calculation"]
pub mod cac_a;
#[doc = "CAC_B (rw) register accessor: Square Parameters for radial shift calculation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cac_b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cac_b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cac_b`]
module"]
#[doc(alias = "CAC_B")]
pub type CacB = crate::Reg<cac_b::CacBSpec>;
#[doc = "Square Parameters for radial shift calculation"]
pub mod cac_b;
#[doc = "CAC_C (rw) register accessor: Cubical Parameters for radial shift calculation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cac_c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cac_c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cac_c`]
module"]
#[doc(alias = "CAC_C")]
pub type CacC = crate::Reg<cac_c::CacCSpec>;
#[doc = "Cubical Parameters for radial shift calculation"]
pub mod cac_c;
#[doc = "CAC_X_NORM (rw) register accessor: Normalization parameters for calculation of image \n\n\n\ncoordinate x_d relative to optical center\n\nNote: These values need to be programmed according to the image resolution and the \n\ncenter offset of the chromatic aberration. \n\n\n\nThe parameters are necessary to avoid high gate count of the CAC hardware block. The \n\nreset value is valid for an image resolution of 2600 x 1950 and center offset 0. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cac_x_norm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cac_x_norm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cac_x_norm`]
module"]
#[doc(alias = "CAC_X_NORM")]
pub type CacXNorm = crate::Reg<cac_x_norm::CacXNormSpec>;
#[doc = "Normalization parameters for calculation of image \n\n\n\ncoordinate x_d relative to optical center\n\nNote: These values need to be programmed according to the image resolution and the \n\ncenter offset of the chromatic aberration. \n\n\n\nThe parameters are necessary to avoid high gate count of the CAC hardware block. The \n\nreset value is valid for an image resolution of 2600 x 1950 and center offset 0. \n\n\n\n \n\n"]
pub mod cac_x_norm;
#[doc = "CAC_Y_NORM (rw) register accessor: Normalization parameters for calculation of image \n\n\n\ncoordinate y_d relative to optical center\n\nNote: These values need to be programmed according to the image resolution and the \n\ncenter offset of the chromatic aberration. \n\n\n\nThe parameters are necessary to avoid high gate count of the CAC hardware block. The \n\n\n\nreset value is valid for an image resolution of 2600 x 1950 and center offset 0. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cac_y_norm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cac_y_norm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cac_y_norm`]
module"]
#[doc(alias = "CAC_Y_NORM")]
pub type CacYNorm = crate::Reg<cac_y_norm::CacYNormSpec>;
#[doc = "Normalization parameters for calculation of image \n\n\n\ncoordinate y_d relative to optical center\n\nNote: These values need to be programmed according to the image resolution and the \n\ncenter offset of the chromatic aberration. \n\n\n\nThe parameters are necessary to avoid high gate count of the CAC hardware block. The \n\n\n\nreset value is valid for an image resolution of 2600 x 1950 and center offset 0. \n\n\n\n \n\n"]
pub mod cac_y_norm;
#[doc = "EXP_CTRL (rw) register accessor: Exposure control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exp_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exp_ctrl`]
module"]
#[doc(alias = "EXP_CTRL")]
pub type ExpCtrl = crate::Reg<exp_ctrl::ExpCtrlSpec>;
#[doc = "Exposure control"]
pub mod exp_ctrl;
#[doc = "EXP_H_OFFSET (rw) register accessor: Horizontal offset for first block\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_h_offset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exp_h_offset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exp_h_offset`]
module"]
#[doc(alias = "EXP_H_OFFSET")]
pub type ExpHOffset = crate::Reg<exp_h_offset::ExpHOffsetSpec>;
#[doc = "Horizontal offset for first block"]
pub mod exp_h_offset;
#[doc = "EXP_V_OFFSET (rw) register accessor: Vertical offset for first block\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_v_offset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exp_v_offset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exp_v_offset`]
module"]
#[doc(alias = "EXP_V_OFFSET")]
pub type ExpVOffset = crate::Reg<exp_v_offset::ExpVOffsetSpec>;
#[doc = "Vertical offset for first block"]
pub mod exp_v_offset;
#[doc = "EXP_H_SIZE (rw) register accessor: Horizontal size of one block\n\nNote: exp_h_size x 5 must be less (not equal) than the horizontal size of the picture \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_h_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exp_h_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exp_h_size`]
module"]
#[doc(alias = "EXP_H_SIZE")]
pub type ExpHSize = crate::Reg<exp_h_size::ExpHSizeSpec>;
#[doc = "Horizontal size of one block\n\nNote: exp_h_size x 5 must be less (not equal) than the horizontal size of the picture \n\n\n\n \n\n"]
pub mod exp_h_size;
#[doc = "EXP_V_SIZE (rw) register accessor: Vertical size of one block\n\nNote: The vertical size must be set in a way that after the last measurement window at \n\n\n\nleast two lines of the image will follow. In addition only even values for vertical size are \n\n\n\nallowed (vertical size must be a multiple of 2). \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_v_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exp_v_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exp_v_size`]
module"]
#[doc(alias = "EXP_V_SIZE")]
pub type ExpVSize = crate::Reg<exp_v_size::ExpVSizeSpec>;
#[doc = "Vertical size of one block\n\nNote: The vertical size must be set in a way that after the last measurement window at \n\n\n\nleast two lines of the image will follow. In addition only even values for vertical size are \n\n\n\nallowed (vertical size must be a multiple of 2). \n\n\n\n"]
pub mod exp_v_size;
#[doc = "EXP_MEAN_00 (r) register accessor: Mean luminance value of block 00\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_00::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exp_mean_00`]
module"]
#[doc(alias = "EXP_MEAN_00")]
pub type ExpMean00 = crate::Reg<exp_mean_00::ExpMean00Spec>;
#[doc = "Mean luminance value of block 00"]
pub mod exp_mean_00;
#[doc = "EXP_MEAN_10 (r) register accessor: Mean luminance value of block 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_10::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exp_mean_10`]
module"]
#[doc(alias = "EXP_MEAN_10")]
pub type ExpMean10 = crate::Reg<exp_mean_10::ExpMean10Spec>;
#[doc = "Mean luminance value of block 10"]
pub mod exp_mean_10;
#[doc = "EXP_MEAN_20 (r) register accessor: Mean luminance value of block 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_20::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exp_mean_20`]
module"]
#[doc(alias = "EXP_MEAN_20")]
pub type ExpMean20 = crate::Reg<exp_mean_20::ExpMean20Spec>;
#[doc = "Mean luminance value of block 20"]
pub mod exp_mean_20;
#[doc = "EXP_MEAN_30 (r) register accessor: Mean luminance value of block 30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_30::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exp_mean_30`]
module"]
#[doc(alias = "EXP_MEAN_30")]
pub type ExpMean30 = crate::Reg<exp_mean_30::ExpMean30Spec>;
#[doc = "Mean luminance value of block 30"]
pub mod exp_mean_30;
#[doc = "EXP_MEAN_40 (r) register accessor: Mean luminance value of block 40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_40::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exp_mean_40`]
module"]
#[doc(alias = "EXP_MEAN_40")]
pub type ExpMean40 = crate::Reg<exp_mean_40::ExpMean40Spec>;
#[doc = "Mean luminance value of block 40"]
pub mod exp_mean_40;
#[doc = "EXP_MEAN_01 (r) register accessor: Mean luminance value of block 01\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_01::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exp_mean_01`]
module"]
#[doc(alias = "EXP_MEAN_01")]
pub type ExpMean01 = crate::Reg<exp_mean_01::ExpMean01Spec>;
#[doc = "Mean luminance value of block 01"]
pub mod exp_mean_01;
#[doc = "EXP_MEAN_11 (r) register accessor: Mean luminance value of block 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_11::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exp_mean_11`]
module"]
#[doc(alias = "EXP_MEAN_11")]
pub type ExpMean11 = crate::Reg<exp_mean_11::ExpMean11Spec>;
#[doc = "Mean luminance value of block 11"]
pub mod exp_mean_11;
#[doc = "EXP_MEAN_21 (r) register accessor: Mean luminance value of block 21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_21::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exp_mean_21`]
module"]
#[doc(alias = "EXP_MEAN_21")]
pub type ExpMean21 = crate::Reg<exp_mean_21::ExpMean21Spec>;
#[doc = "Mean luminance value of block 21"]
pub mod exp_mean_21;
#[doc = "EXP_MEAN_31 (r) register accessor: Mean luminance value of block 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_31::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exp_mean_31`]
module"]
#[doc(alias = "EXP_MEAN_31")]
pub type ExpMean31 = crate::Reg<exp_mean_31::ExpMean31Spec>;
#[doc = "Mean luminance value of block 31"]
pub mod exp_mean_31;
#[doc = "EXP_MEAN_41 (r) register accessor: Mean luminance value of block 41\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_41::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exp_mean_41`]
module"]
#[doc(alias = "EXP_MEAN_41")]
pub type ExpMean41 = crate::Reg<exp_mean_41::ExpMean41Spec>;
#[doc = "Mean luminance value of block 41"]
pub mod exp_mean_41;
#[doc = "EXP_MEAN_02 (r) register accessor: Mean luminance value of block 02\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_02::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exp_mean_02`]
module"]
#[doc(alias = "EXP_MEAN_02")]
pub type ExpMean02 = crate::Reg<exp_mean_02::ExpMean02Spec>;
#[doc = "Mean luminance value of block 02"]
pub mod exp_mean_02;
#[doc = "EXP_MEAN_12 (r) register accessor: Mean luminance value of block 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_12::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exp_mean_12`]
module"]
#[doc(alias = "EXP_MEAN_12")]
pub type ExpMean12 = crate::Reg<exp_mean_12::ExpMean12Spec>;
#[doc = "Mean luminance value of block 12"]
pub mod exp_mean_12;
#[doc = "EXP_MEAN_22 (r) register accessor: Mean luminance value of block 22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_22::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exp_mean_22`]
module"]
#[doc(alias = "EXP_MEAN_22")]
pub type ExpMean22 = crate::Reg<exp_mean_22::ExpMean22Spec>;
#[doc = "Mean luminance value of block 22"]
pub mod exp_mean_22;
#[doc = "EXP_MEAN_32 (r) register accessor: Mean luminance value of block 32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_32::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exp_mean_32`]
module"]
#[doc(alias = "EXP_MEAN_32")]
pub type ExpMean32 = crate::Reg<exp_mean_32::ExpMean32Spec>;
#[doc = "Mean luminance value of block 32"]
pub mod exp_mean_32;
#[doc = "EXP_MEAN_42 (r) register accessor: Mean luminance value of block 42\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_42::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exp_mean_42`]
module"]
#[doc(alias = "EXP_MEAN_42")]
pub type ExpMean42 = crate::Reg<exp_mean_42::ExpMean42Spec>;
#[doc = "Mean luminance value of block 42"]
pub mod exp_mean_42;
#[doc = "EXP_MEAN_03 (r) register accessor: Mean luminance value of block 03\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_03::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exp_mean_03`]
module"]
#[doc(alias = "EXP_MEAN_03")]
pub type ExpMean03 = crate::Reg<exp_mean_03::ExpMean03Spec>;
#[doc = "Mean luminance value of block 03"]
pub mod exp_mean_03;
#[doc = "EXP_MEAN_13 (r) register accessor: Mean luminance value of block 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_13::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exp_mean_13`]
module"]
#[doc(alias = "EXP_MEAN_13")]
pub type ExpMean13 = crate::Reg<exp_mean_13::ExpMean13Spec>;
#[doc = "Mean luminance value of block 13"]
pub mod exp_mean_13;
#[doc = "EXP_MEAN_23 (r) register accessor: Mean luminance value of block 23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_23::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exp_mean_23`]
module"]
#[doc(alias = "EXP_MEAN_23")]
pub type ExpMean23 = crate::Reg<exp_mean_23::ExpMean23Spec>;
#[doc = "Mean luminance value of block 23"]
pub mod exp_mean_23;
#[doc = "EXP_MEAN_33 (r) register accessor: Mean luminance value of block 33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_33::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exp_mean_33`]
module"]
#[doc(alias = "EXP_MEAN_33")]
pub type ExpMean33 = crate::Reg<exp_mean_33::ExpMean33Spec>;
#[doc = "Mean luminance value of block 33"]
pub mod exp_mean_33;
#[doc = "EXP_MEAN_43 (r) register accessor: Mean luminance value of block 43\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_43::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exp_mean_43`]
module"]
#[doc(alias = "EXP_MEAN_43")]
pub type ExpMean43 = crate::Reg<exp_mean_43::ExpMean43Spec>;
#[doc = "Mean luminance value of block 43"]
pub mod exp_mean_43;
#[doc = "EXP_MEAN_04 (r) register accessor: Mean luminance value of block 04\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_04::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exp_mean_04`]
module"]
#[doc(alias = "EXP_MEAN_04")]
pub type ExpMean04 = crate::Reg<exp_mean_04::ExpMean04Spec>;
#[doc = "Mean luminance value of block 04"]
pub mod exp_mean_04;
#[doc = "EXP_MEAN_14 (r) register accessor: Mean luminance value of block 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_14::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exp_mean_14`]
module"]
#[doc(alias = "EXP_MEAN_14")]
pub type ExpMean14 = crate::Reg<exp_mean_14::ExpMean14Spec>;
#[doc = "Mean luminance value of block 14"]
pub mod exp_mean_14;
#[doc = "EXP_MEAN_24 (r) register accessor: Mean luminance value of block 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_24::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exp_mean_24`]
module"]
#[doc(alias = "EXP_MEAN_24")]
pub type ExpMean24 = crate::Reg<exp_mean_24::ExpMean24Spec>;
#[doc = "Mean luminance value of block 24"]
pub mod exp_mean_24;
#[doc = "EXP_MEAN_34 (r) register accessor: Mean luminance value of block 34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_34::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exp_mean_34`]
module"]
#[doc(alias = "EXP_MEAN_34")]
pub type ExpMean34 = crate::Reg<exp_mean_34::ExpMean34Spec>;
#[doc = "Mean luminance value of block 34"]
pub mod exp_mean_34;
#[doc = "EXP_MEAN_44 (r) register accessor: Mean luminance value of block 44\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_44::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exp_mean_44`]
module"]
#[doc(alias = "EXP_MEAN_44")]
pub type ExpMean44 = crate::Reg<exp_mean_44::ExpMean44Spec>;
#[doc = "Mean luminance value of block 44"]
pub mod exp_mean_44;
#[doc = "BLS_CTRL (rw) register accessor: global control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bls_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bls_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bls_ctrl`]
module"]
#[doc(alias = "BLS_CTRL")]
pub type BlsCtrl = crate::Reg<bls_ctrl::BlsCtrlSpec>;
#[doc = "global control register"]
pub mod bls_ctrl;
#[doc = "BLS_SAMPLES (rw) register accessor: samples register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bls_samples::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bls_samples::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bls_samples`]
module"]
#[doc(alias = "BLS_SAMPLES")]
pub type BlsSamples = crate::Reg<bls_samples::BlsSamplesSpec>;
#[doc = "samples register"]
pub mod bls_samples;
#[doc = "BLS_H1_START (rw) register accessor: window 1 horizontal start\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bls_h1_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bls_h1_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bls_h1_start`]
module"]
#[doc(alias = "BLS_H1_START")]
pub type BlsH1Start = crate::Reg<bls_h1_start::BlsH1StartSpec>;
#[doc = "window 1 horizontal start"]
pub mod bls_h1_start;
#[doc = "BLS_H1_STOP (rw) register accessor: window 1 horizontal stop\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bls_h1_stop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bls_h1_stop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bls_h1_stop`]
module"]
#[doc(alias = "BLS_H1_STOP")]
pub type BlsH1Stop = crate::Reg<bls_h1_stop::BlsH1StopSpec>;
#[doc = "window 1 horizontal stop"]
pub mod bls_h1_stop;
#[doc = "BLS_V1_START (rw) register accessor: window 1 vertical start\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bls_v1_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bls_v1_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bls_v1_start`]
module"]
#[doc(alias = "BLS_V1_START")]
pub type BlsV1Start = crate::Reg<bls_v1_start::BlsV1StartSpec>;
#[doc = "window 1 vertical start"]
pub mod bls_v1_start;
#[doc = "BLS_V1_STOP (rw) register accessor: window 1 vertical stop\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bls_v1_stop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bls_v1_stop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bls_v1_stop`]
module"]
#[doc(alias = "BLS_V1_STOP")]
pub type BlsV1Stop = crate::Reg<bls_v1_stop::BlsV1StopSpec>;
#[doc = "window 1 vertical stop"]
pub mod bls_v1_stop;
#[doc = "BLS_H2_START (rw) register accessor: window 2 horizontal start\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bls_h2_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bls_h2_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bls_h2_start`]
module"]
#[doc(alias = "BLS_H2_START")]
pub type BlsH2Start = crate::Reg<bls_h2_start::BlsH2StartSpec>;
#[doc = "window 2 horizontal start"]
pub mod bls_h2_start;
#[doc = "BLS_H2_STOP (rw) register accessor: window 2 horizontal stop\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bls_h2_stop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bls_h2_stop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bls_h2_stop`]
module"]
#[doc(alias = "BLS_H2_STOP")]
pub type BlsH2Stop = crate::Reg<bls_h2_stop::BlsH2StopSpec>;
#[doc = "window 2 horizontal stop"]
pub mod bls_h2_stop;
#[doc = "BLS_V2_START (rw) register accessor: window 2 vertical start\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bls_v2_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bls_v2_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bls_v2_start`]
module"]
#[doc(alias = "BLS_V2_START")]
pub type BlsV2Start = crate::Reg<bls_v2_start::BlsV2StartSpec>;
#[doc = "window 2 vertical start"]
pub mod bls_v2_start;
#[doc = "BLS_V2_STOP (rw) register accessor: window 2 vertical stop\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bls_v2_stop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bls_v2_stop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bls_v2_stop`]
module"]
#[doc(alias = "BLS_V2_STOP")]
pub type BlsV2Stop = crate::Reg<bls_v2_stop::BlsV2StopSpec>;
#[doc = "window 2 vertical stop"]
pub mod bls_v2_stop;
#[doc = "BLS_A_FIXED (rw) register accessor: fixed black level A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bls_a_fixed::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bls_a_fixed::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bls_a_fixed`]
module"]
#[doc(alias = "BLS_A_FIXED")]
pub type BlsAFixed = crate::Reg<bls_a_fixed::BlsAFixedSpec>;
#[doc = "fixed black level A"]
pub mod bls_a_fixed;
#[doc = "BLS_B_FIXED (rw) register accessor: fixed black level B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bls_b_fixed::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bls_b_fixed::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bls_b_fixed`]
module"]
#[doc(alias = "BLS_B_FIXED")]
pub type BlsBFixed = crate::Reg<bls_b_fixed::BlsBFixedSpec>;
#[doc = "fixed black level B"]
pub mod bls_b_fixed;
#[doc = "BLS_C_FIXED (rw) register accessor: fixed black level C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bls_c_fixed::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bls_c_fixed::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bls_c_fixed`]
module"]
#[doc(alias = "BLS_C_FIXED")]
pub type BlsCFixed = crate::Reg<bls_c_fixed::BlsCFixedSpec>;
#[doc = "fixed black level C"]
pub mod bls_c_fixed;
#[doc = "BLS_D_FIXED (rw) register accessor: fixed black level D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bls_d_fixed::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bls_d_fixed::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bls_d_fixed`]
module"]
#[doc(alias = "BLS_D_FIXED")]
pub type BlsDFixed = crate::Reg<bls_d_fixed::BlsDFixedSpec>;
#[doc = "fixed black level D"]
pub mod bls_d_fixed;
#[doc = "BLS_A_MEASURED (r) register accessor: measured black level A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bls_a_measured::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bls_a_measured`]
module"]
#[doc(alias = "BLS_A_MEASURED")]
pub type BlsAMeasured = crate::Reg<bls_a_measured::BlsAMeasuredSpec>;
#[doc = "measured black level A"]
pub mod bls_a_measured;
#[doc = "BLS_B_MEASURED (r) register accessor: measured black level B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bls_b_measured::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bls_b_measured`]
module"]
#[doc(alias = "BLS_B_MEASURED")]
pub type BlsBMeasured = crate::Reg<bls_b_measured::BlsBMeasuredSpec>;
#[doc = "measured black level B"]
pub mod bls_b_measured;
#[doc = "BLS_C_MEASURED (r) register accessor: measured black level C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bls_c_measured::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bls_c_measured`]
module"]
#[doc(alias = "BLS_C_MEASURED")]
pub type BlsCMeasured = crate::Reg<bls_c_measured::BlsCMeasuredSpec>;
#[doc = "measured black level C"]
pub mod bls_c_measured;
#[doc = "BLS_D_MEASURED (r) register accessor: measured black level D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bls_d_measured::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bls_d_measured`]
module"]
#[doc(alias = "BLS_D_MEASURED")]
pub type BlsDMeasured = crate::Reg<bls_d_measured::BlsDMeasuredSpec>;
#[doc = "measured black level D"]
pub mod bls_d_measured;
#[doc = "DPF_MODE (rw) register accessor: Mode control for Denoising Pre-Filter block\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpf_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpf_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpf_mode`]
module"]
#[doc(alias = "DPF_MODE")]
pub type DpfMode = crate::Reg<dpf_mode::DpfModeSpec>;
#[doc = "Mode control for Denoising Pre-Filter block"]
pub mod dpf_mode;
#[doc = "DPF_STRENGTH_R (rw) register accessor: filter strength of the RED filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpf_strength_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpf_strength_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpf_strength_r`]
module"]
#[doc(alias = "DPF_STRENGTH_R")]
pub type DpfStrengthR = crate::Reg<dpf_strength_r::DpfStrengthRSpec>;
#[doc = "filter strength of the RED filter"]
pub mod dpf_strength_r;
#[doc = "DPF_STRENGTH_G (rw) register accessor: filter strength of the GREEN filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpf_strength_g::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpf_strength_g::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpf_strength_g`]
module"]
#[doc(alias = "DPF_STRENGTH_G")]
pub type DpfStrengthG = crate::Reg<dpf_strength_g::DpfStrengthGSpec>;
#[doc = "filter strength of the GREEN filter"]
pub mod dpf_strength_g;
#[doc = "DPF_STRENGTH_B (rw) register accessor: filter strength of the BLUE filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpf_strength_b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpf_strength_b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpf_strength_b`]
module"]
#[doc(alias = "DPF_STRENGTH_B")]
pub type DpfStrengthB = crate::Reg<dpf_strength_b::DpfStrengthBSpec>;
#[doc = "filter strength of the BLUE filter"]
pub mod dpf_strength_b;
#[doc = "DPF_S_WEIGHT_G_1_4 (rw) register accessor: Spatial Weights green channel 1 2 3 4\n\nNote: The value zero (0/16) disables the filter tap \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpf_s_weight_g_1_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpf_s_weight_g_1_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpf_s_weight_g_1_4`]
module"]
#[doc(alias = "DPF_S_WEIGHT_G_1_4")]
pub type DpfSWeightG1_4 = crate::Reg<dpf_s_weight_g_1_4::DpfSWeightG1_4Spec>;
#[doc = "Spatial Weights green channel 1 2 3 4\n\nNote: The value zero (0/16) disables the filter tap \n\n\n\n \n\n"]
pub mod dpf_s_weight_g_1_4;
#[doc = "DPF_S_WEIGHT_G_5_6 (rw) register accessor: Spatial Weights green channel 5 6\n\nNote: The value zero (0/16) disables the filter tap. \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpf_s_weight_g_5_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpf_s_weight_g_5_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpf_s_weight_g_5_6`]
module"]
#[doc(alias = "DPF_S_WEIGHT_G_5_6")]
pub type DpfSWeightG5_6 = crate::Reg<dpf_s_weight_g_5_6::DpfSWeightG5_6Spec>;
#[doc = "Spatial Weights green channel 5 6\n\nNote: The value zero (0/16) disables the filter tap. \n\n\n\n"]
pub mod dpf_s_weight_g_5_6;
#[doc = "DPF_S_WEIGHT_RB_1_4 (rw) register accessor: Spatial Weights red/blue channels 1 2 3 4\n\nNote: The value zero (0/16) disables the filter tap. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpf_s_weight_rb_1_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpf_s_weight_rb_1_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpf_s_weight_rb_1_4`]
module"]
#[doc(alias = "DPF_S_WEIGHT_RB_1_4")]
pub type DpfSWeightRb1_4 = crate::Reg<dpf_s_weight_rb_1_4::DpfSWeightRb1_4Spec>;
#[doc = "Spatial Weights red/blue channels 1 2 3 4\n\nNote: The value zero (0/16) disables the filter tap. \n\n\n\n \n\n"]
pub mod dpf_s_weight_rb_1_4;
#[doc = "DPF_S_WEIGHT_RB_5_6 (rw) register accessor: Spatial Weights red/blue channels 5 6\n\nNote: The value zero (0/16) disables the filter tap. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpf_s_weight_rb_5_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpf_s_weight_rb_5_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpf_s_weight_rb_5_6`]
module"]
#[doc(alias = "DPF_S_WEIGHT_RB_5_6")]
pub type DpfSWeightRb5_6 = crate::Reg<dpf_s_weight_rb_5_6::DpfSWeightRb5_6Spec>;
#[doc = "Spatial Weights red/blue channels 5 6\n\nNote: The value zero (0/16) disables the filter tap. \n\n\n\n \n\n"]
pub mod dpf_s_weight_rb_5_6;
#[doc = "DPF_NLL_COEFF (rw) register accessor: Noise Level Lookup Coefficient n (n=0..16)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpf_nll_coeff::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpf_nll_coeff::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpf_nll_coeff`]
module"]
#[doc(alias = "DPF_NLL_COEFF")]
pub type DpfNllCoeff = crate::Reg<dpf_nll_coeff::DpfNllCoeffSpec>;
#[doc = "Noise Level Lookup Coefficient n (n=0..16)"]
pub mod dpf_nll_coeff;
#[doc = "DPF_NF_GAIN_R (rw) register accessor: noise function gain for red pixels\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpf_nf_gain_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpf_nf_gain_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpf_nf_gain_r`]
module"]
#[doc(alias = "DPF_NF_GAIN_R")]
pub type DpfNfGainR = crate::Reg<dpf_nf_gain_r::DpfNfGainRSpec>;
#[doc = "noise function gain for red pixels"]
pub mod dpf_nf_gain_r;
#[doc = "DPF_NF_GAIN_GR (rw) register accessor: noise function gain for green in red pixels\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpf_nf_gain_gr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpf_nf_gain_gr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpf_nf_gain_gr`]
module"]
#[doc(alias = "DPF_NF_GAIN_GR")]
pub type DpfNfGainGr = crate::Reg<dpf_nf_gain_gr::DpfNfGainGrSpec>;
#[doc = "noise function gain for green in red pixels"]
pub mod dpf_nf_gain_gr;
#[doc = "DPF_NF_GAIN_GB (rw) register accessor: noise function gain for green in blue pixels\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpf_nf_gain_gb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpf_nf_gain_gb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpf_nf_gain_gb`]
module"]
#[doc(alias = "DPF_NF_GAIN_GB")]
pub type DpfNfGainGb = crate::Reg<dpf_nf_gain_gb::DpfNfGainGbSpec>;
#[doc = "noise function gain for green in blue pixels"]
pub mod dpf_nf_gain_gb;
#[doc = "DPF_NF_GAIN_B (rw) register accessor: noise function gain for blue pixels\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpf_nf_gain_b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpf_nf_gain_b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpf_nf_gain_b`]
module"]
#[doc(alias = "DPF_NF_GAIN_B")]
pub type DpfNfGainB = crate::Reg<dpf_nf_gain_b::DpfNfGainBSpec>;
#[doc = "noise function gain for blue pixels"]
pub mod dpf_nf_gain_b;
#[doc = "DPCC_MODE (rw) register accessor: Mode control for DPCC detection unit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpcc_mode`]
module"]
#[doc(alias = "DPCC_MODE")]
pub type DpccMode = crate::Reg<dpcc_mode::DpccModeSpec>;
#[doc = "Mode control for DPCC detection unit"]
pub mod dpcc_mode;
#[doc = "DPCC_OUTPUT_MODE (rw) register accessor: Interpolation mode for correction unit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_output_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_output_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpcc_output_mode`]
module"]
#[doc(alias = "DPCC_OUTPUT_MODE")]
pub type DpccOutputMode = crate::Reg<dpcc_output_mode::DpccOutputModeSpec>;
#[doc = "Interpolation mode for correction unit"]
pub mod dpcc_output_mode;
#[doc = "DPCC_SET_USE (rw) register accessor: DPCC methods set usage for detection\n\nNote: methods sets can be used in parallel for each stage and the result is the logical OR \n\n\n\nof all selected sets \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_set_use::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_set_use::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpcc_set_use`]
module"]
#[doc(alias = "DPCC_SET_USE")]
pub type DpccSetUse = crate::Reg<dpcc_set_use::DpccSetUseSpec>;
#[doc = "DPCC methods set usage for detection\n\nNote: methods sets can be used in parallel for each stage and the result is the logical OR \n\n\n\nof all selected sets \n\n\n\n \n\n"]
pub mod dpcc_set_use;
#[doc = "DPCC_METHODS_SET_1 (rw) register accessor: Methods enable bits for SET_1\n\nNote: different methods can be used in parallel, the result is the logical AND of all selected \n\n\n\nmethods \n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_methods_set_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_methods_set_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpcc_methods_set_1`]
module"]
#[doc(alias = "DPCC_METHODS_SET_1")]
pub type DpccMethodsSet1 = crate::Reg<dpcc_methods_set_1::DpccMethodsSet1Spec>;
#[doc = "Methods enable bits for SET_1\n\nNote: different methods can be used in parallel, the result is the logical AND of all selected \n\n\n\nmethods \n\n \n\n"]
pub mod dpcc_methods_set_1;
#[doc = "DPCC_METHODS_SET_2 (rw) register accessor: Methods enable bits for SET_2\n\nNote: different methods can be used in parallel, the result is the logical AND of all selected \n\n\n\nmethods \n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_methods_set_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_methods_set_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpcc_methods_set_2`]
module"]
#[doc(alias = "DPCC_METHODS_SET_2")]
pub type DpccMethodsSet2 = crate::Reg<dpcc_methods_set_2::DpccMethodsSet2Spec>;
#[doc = "Methods enable bits for SET_2\n\nNote: different methods can be used in parallel, the result is the logical AND of all selected \n\n\n\nmethods \n\n \n\n"]
pub mod dpcc_methods_set_2;
#[doc = "DPCC_METHODS_SET_3 (rw) register accessor: Methods enable bits for SET_3\n\nNote: different methods can be used in parallel, the result is the logical AND of all selected \n\n\n\nmethods \n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_methods_set_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_methods_set_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpcc_methods_set_3`]
module"]
#[doc(alias = "DPCC_METHODS_SET_3")]
pub type DpccMethodsSet3 = crate::Reg<dpcc_methods_set_3::DpccMethodsSet3Spec>;
#[doc = "Methods enable bits for SET_3\n\nNote: different methods can be used in parallel, the result is the logical AND of all selected \n\n\n\nmethods \n\n \n\n"]
pub mod dpcc_methods_set_3;
#[doc = "DPCC_LINE_THRESH_1 (rw) register accessor: Line threshold SET_1\n\nNote: all values are unsigned integer \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_line_thresh_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_line_thresh_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpcc_line_thresh_1`]
module"]
#[doc(alias = "DPCC_LINE_THRESH_1")]
pub type DpccLineThresh1 = crate::Reg<dpcc_line_thresh_1::DpccLineThresh1Spec>;
#[doc = "Line threshold SET_1\n\nNote: all values are unsigned integer \n\n\n\n \n\n"]
pub mod dpcc_line_thresh_1;
#[doc = "DPCC_LINE_MAD_FAC_1 (rw) register accessor: Mean Absolute Difference (MAD) factor for Line check set 1\n\nNote: all values are unsigned integer \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_line_mad_fac_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_line_mad_fac_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpcc_line_mad_fac_1`]
module"]
#[doc(alias = "DPCC_LINE_MAD_FAC_1")]
pub type DpccLineMadFac1 = crate::Reg<dpcc_line_mad_fac_1::DpccLineMadFac1Spec>;
#[doc = "Mean Absolute Difference (MAD) factor for Line check set 1\n\nNote: all values are unsigned integer \n\n\n\n"]
pub mod dpcc_line_mad_fac_1;
#[doc = "DPCC_PG_FAC_1 (rw) register accessor: Peak gradient factor for set 1\n\nNote: all values are unsigned integer \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_pg_fac_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_pg_fac_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpcc_pg_fac_1`]
module"]
#[doc(alias = "DPCC_PG_FAC_1")]
pub type DpccPgFac1 = crate::Reg<dpcc_pg_fac_1::DpccPgFac1Spec>;
#[doc = "Peak gradient factor for set 1\n\nNote: all values are unsigned integer \n\n\n\n \n\n"]
pub mod dpcc_pg_fac_1;
#[doc = "DPCC_RND_THRESH_1 (rw) register accessor: Rank Neighbor Difference threshold for set 1\n\nNote: all values are unsigned integer \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_rnd_thresh_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_rnd_thresh_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpcc_rnd_thresh_1`]
module"]
#[doc(alias = "DPCC_RND_THRESH_1")]
pub type DpccRndThresh1 = crate::Reg<dpcc_rnd_thresh_1::DpccRndThresh1Spec>;
#[doc = "Rank Neighbor Difference threshold for set 1\n\nNote: all values are unsigned integer \n\n\n\n \n\n"]
pub mod dpcc_rnd_thresh_1;
#[doc = "DPCC_RG_FAC_1 (rw) register accessor: Rank gradient factor for set 1\n\nNote: all values are unsigned integer \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_rg_fac_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_rg_fac_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpcc_rg_fac_1`]
module"]
#[doc(alias = "DPCC_RG_FAC_1")]
pub type DpccRgFac1 = crate::Reg<dpcc_rg_fac_1::DpccRgFac1Spec>;
#[doc = "Rank gradient factor for set 1\n\nNote: all values are unsigned integer \n\n\n\n"]
pub mod dpcc_rg_fac_1;
#[doc = "DPCC_LINE_THRESH_2 (rw) register accessor: Line threshold set 2\n\nNote: all values are unsigned integer \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_line_thresh_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_line_thresh_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpcc_line_thresh_2`]
module"]
#[doc(alias = "DPCC_LINE_THRESH_2")]
pub type DpccLineThresh2 = crate::Reg<dpcc_line_thresh_2::DpccLineThresh2Spec>;
#[doc = "Line threshold set 2\n\nNote: all values are unsigned integer \n\n\n\n \n\n"]
pub mod dpcc_line_thresh_2;
#[doc = "DPCC_LINE_MAD_FAC_2 (rw) register accessor: Mean Absolute Difference (MAD) factor for Line check set 2\n\nNote: all values are unsigned integer \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_line_mad_fac_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_line_mad_fac_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpcc_line_mad_fac_2`]
module"]
#[doc(alias = "DPCC_LINE_MAD_FAC_2")]
pub type DpccLineMadFac2 = crate::Reg<dpcc_line_mad_fac_2::DpccLineMadFac2Spec>;
#[doc = "Mean Absolute Difference (MAD) factor for Line check set 2\n\nNote: all values are unsigned integer \n\n\n\n \n\n"]
pub mod dpcc_line_mad_fac_2;
#[doc = "DPCC_PG_FAC_2 (rw) register accessor: Peak gradient factor for set 2\n\nNote: all values are unsigned integer \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_pg_fac_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_pg_fac_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpcc_pg_fac_2`]
module"]
#[doc(alias = "DPCC_PG_FAC_2")]
pub type DpccPgFac2 = crate::Reg<dpcc_pg_fac_2::DpccPgFac2Spec>;
#[doc = "Peak gradient factor for set 2\n\nNote: all values are unsigned integer \n\n\n\n"]
pub mod dpcc_pg_fac_2;
#[doc = "DPCC_RND_THRESH_2 (rw) register accessor: Rank Neighbor Difference threshold for set 2\n\nNote: all values are unsigned integer \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_rnd_thresh_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_rnd_thresh_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpcc_rnd_thresh_2`]
module"]
#[doc(alias = "DPCC_RND_THRESH_2")]
pub type DpccRndThresh2 = crate::Reg<dpcc_rnd_thresh_2::DpccRndThresh2Spec>;
#[doc = "Rank Neighbor Difference threshold for set 2\n\nNote: all values are unsigned integer \n\n\n\n \n\n"]
pub mod dpcc_rnd_thresh_2;
#[doc = "DPCC_RG_FAC_2 (rw) register accessor: Rank gradient factor for set 2\n\nNote: all values are unsigned integer \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_rg_fac_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_rg_fac_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpcc_rg_fac_2`]
module"]
#[doc(alias = "DPCC_RG_FAC_2")]
pub type DpccRgFac2 = crate::Reg<dpcc_rg_fac_2::DpccRgFac2Spec>;
#[doc = "Rank gradient factor for set 2\n\nNote: all values are unsigned integer \n\n\n\n \n\n"]
pub mod dpcc_rg_fac_2;
#[doc = "DPCC_LINE_THRESH_3 (rw) register accessor: Line threshold set 3\n\nNote: all values are unsigned integer \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_line_thresh_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_line_thresh_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpcc_line_thresh_3`]
module"]
#[doc(alias = "DPCC_LINE_THRESH_3")]
pub type DpccLineThresh3 = crate::Reg<dpcc_line_thresh_3::DpccLineThresh3Spec>;
#[doc = "Line threshold set 3\n\nNote: all values are unsigned integer \n\n\n\n"]
pub mod dpcc_line_thresh_3;
#[doc = "DPCC_LINE_MAD_FAC_3 (rw) register accessor: Mean Absolute Difference (MAD) factor for Line check set 3\n\nNote: all values are unsigned integer \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_line_mad_fac_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_line_mad_fac_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpcc_line_mad_fac_3`]
module"]
#[doc(alias = "DPCC_LINE_MAD_FAC_3")]
pub type DpccLineMadFac3 = crate::Reg<dpcc_line_mad_fac_3::DpccLineMadFac3Spec>;
#[doc = "Mean Absolute Difference (MAD) factor for Line check set 3\n\nNote: all values are unsigned integer \n\n\n\n \n\n"]
pub mod dpcc_line_mad_fac_3;
#[doc = "DPCC_PG_FAC_3 (rw) register accessor: Peak gradient factor for set 3\n\nNote: all values are unsigned integer \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_pg_fac_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_pg_fac_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpcc_pg_fac_3`]
module"]
#[doc(alias = "DPCC_PG_FAC_3")]
pub type DpccPgFac3 = crate::Reg<dpcc_pg_fac_3::DpccPgFac3Spec>;
#[doc = "Peak gradient factor for set 3\n\nNote: all values are unsigned integer \n\n\n\n \n\n"]
pub mod dpcc_pg_fac_3;
#[doc = "DPCC_RND_THRESH_3 (rw) register accessor: Rank Neighbor Difference threshold for set 3\n\nNote: all values are unsigned integer \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_rnd_thresh_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_rnd_thresh_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpcc_rnd_thresh_3`]
module"]
#[doc(alias = "DPCC_RND_THRESH_3")]
pub type DpccRndThresh3 = crate::Reg<dpcc_rnd_thresh_3::DpccRndThresh3Spec>;
#[doc = "Rank Neighbor Difference threshold for set 3\n\nNote: all values are unsigned integer \n\n\n\n"]
pub mod dpcc_rnd_thresh_3;
#[doc = "DPCC_RG_FAC_3 (rw) register accessor: Rank gradient factor for set 3\n\nNote: all values are unsigned integer \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_rg_fac_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_rg_fac_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpcc_rg_fac_3`]
module"]
#[doc(alias = "DPCC_RG_FAC_3")]
pub type DpccRgFac3 = crate::Reg<dpcc_rg_fac_3::DpccRgFac3Spec>;
#[doc = "Rank gradient factor for set 3\n\nNote: all values are unsigned integer \n\n\n\n \n\n"]
pub mod dpcc_rg_fac_3;
#[doc = "DPCC_RO_LIMITS (rw) register accessor: Rank Order Limits\n\nNote: all values are unsigned integer \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_ro_limits::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_ro_limits::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpcc_ro_limits`]
module"]
#[doc(alias = "DPCC_RO_LIMITS")]
pub type DpccRoLimits = crate::Reg<dpcc_ro_limits::DpccRoLimitsSpec>;
#[doc = "Rank Order Limits\n\nNote: all values are unsigned integer \n\n\n\n \n\n"]
pub mod dpcc_ro_limits;
#[doc = "DPCC_RND_OFFS (rw) register accessor: Differential Rank Offsets for Rank Neighbor Difference\n\nNote: all values are unsigned integer \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_rnd_offs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_rnd_offs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpcc_rnd_offs`]
module"]
#[doc(alias = "DPCC_RND_OFFS")]
pub type DpccRndOffs = crate::Reg<dpcc_rnd_offs::DpccRndOffsSpec>;
#[doc = "Differential Rank Offsets for Rank Neighbor Difference\n\nNote: all values are unsigned integer \n\n\n\n \n\n"]
pub mod dpcc_rnd_offs;
#[doc = "DPCC_BPT_CTRL (rw) register accessor: bad pixel table settings\n\nNote: This register controls the behaviour of the table based bad pixel correction module. \n\nIt can be switched on and off independently of the DPCC detection and correction block. \n\nDifferent correction algorithms for the table based correction are available and are defined by \n\nthis register. The default setting after reset enables a correction algorithm with most accurate \n\ncorrelation to surrounding pixels. Detection for the table based correction can be configured \n\nindependently from the on-the-fly DPCC detection scheme. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_bpt_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_bpt_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpcc_bpt_ctrl`]
module"]
#[doc(alias = "DPCC_BPT_CTRL")]
pub type DpccBptCtrl = crate::Reg<dpcc_bpt_ctrl::DpccBptCtrlSpec>;
#[doc = "bad pixel table settings\n\nNote: This register controls the behaviour of the table based bad pixel correction module. \n\nIt can be switched on and off independently of the DPCC detection and correction block. \n\nDifferent correction algorithms for the table based correction are available and are defined by \n\nthis register. The default setting after reset enables a correction algorithm with most accurate \n\ncorrelation to surrounding pixels. Detection for the table based correction can be configured \n\nindependently from the on-the-fly DPCC detection scheme. \n\n\n\n \n\n"]
pub mod dpcc_bpt_ctrl;
#[doc = "DPCC_BPT_NUMBER (rw) register accessor: Number of entries for bad pixel table (table based correction)\n\nNote: bit width of bp_number depends on size of BP RAM which is defined during chip \n\n\n\nsynthesis \n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_bpt_number::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_bpt_number::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpcc_bpt_number`]
module"]
#[doc(alias = "DPCC_BPT_NUMBER")]
pub type DpccBptNumber = crate::Reg<dpcc_bpt_number::DpccBptNumberSpec>;
#[doc = "Number of entries for bad pixel table (table based correction)\n\nNote: bit width of bp_number depends on size of BP RAM which is defined during chip \n\n\n\nsynthesis \n\n \n\n"]
pub mod dpcc_bpt_number;
#[doc = "DPCC_BPT_ADDR (rw) register accessor: TABLE Start Address for table-based correction algorithm\n\nNote: MKOE tbc: Orignial register mode was rwh which is no longer supported with new \n\n\n\nversion of SIG \n\n\n\n-> rwhh \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_bpt_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_bpt_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpcc_bpt_addr`]
module"]
#[doc(alias = "DPCC_BPT_ADDR")]
pub type DpccBptAddr = crate::Reg<dpcc_bpt_addr::DpccBptAddrSpec>;
#[doc = "TABLE Start Address for table-based correction algorithm\n\nNote: MKOE tbc: Orignial register mode was rwh which is no longer supported with new \n\n\n\nversion of SIG \n\n\n\n-> rwhh \n\n\n\n"]
pub mod dpcc_bpt_addr;
#[doc = "DPCC_BPT_DATA (rw) register accessor: TABLE DATA register for read and write access of table RAM\n\nNote: MKOE tbc: Orignial register mode was rwh which is no longer supported with new \n\n\n\nversion of SIG \n\n\n\n-> rwhh \n\n\n\nThe programmed table value is immediately written into the RAM. The RAM address is \n\n\n\ngenerated per auto-increment. The parameter RAMs for Lens Shade Correction and Bad \n\n\n\nPixel Correction can only be programmed, if the RGB Bayer path is switched on via ISP_CTRL \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_bpt_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_bpt_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpcc_bpt_data`]
module"]
#[doc(alias = "DPCC_BPT_DATA")]
pub type DpccBptData = crate::Reg<dpcc_bpt_data::DpccBptDataSpec>;
#[doc = "TABLE DATA register for read and write access of table RAM\n\nNote: MKOE tbc: Orignial register mode was rwh which is no longer supported with new \n\n\n\nversion of SIG \n\n\n\n-> rwhh \n\n\n\nThe programmed table value is immediately written into the RAM. The RAM address is \n\n\n\ngenerated per auto-increment. The parameter RAMs for Lens Shade Correction and Bad \n\n\n\nPixel Correction can only be programmed, if the RGB Bayer path is switched on via ISP_CTRL \n\n"]
pub mod dpcc_bpt_data;
#[doc = "WDR_CTRL (rw) register accessor: Control Bits for Wide Dynamic Range Unit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdr_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdr_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdr_ctrl`]
module"]
#[doc(alias = "WDR_CTRL")]
pub type WdrCtrl = crate::Reg<wdr_ctrl::WdrCtrlSpec>;
#[doc = "Control Bits for Wide Dynamic Range Unit"]
pub mod wdr_ctrl;
#[doc = "WDR_TONECURVE_1 (rw) register accessor: Tone Curve sample points dYn definition (part 1)\n\nNote: The interval widths dYn are to be defined in a 2^(value+3) notation, where \n\n\n\n'value' has to be written to the register. So the steps would be \n\n\n\ndYn=0 -> 8 (2^3), dYn=1 -> 16 (2^4), dYn=2 \n\n\n\n-> 32 (2^5),... dYn=6 -> 512 (2^9), dYn=7 -> \n\n\n\n1024 (2^10). \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdr_tonecurve_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdr_tonecurve_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdr_tonecurve_1`]
module"]
#[doc(alias = "WDR_TONECURVE_1")]
pub type WdrTonecurve1 = crate::Reg<wdr_tonecurve_1::WdrTonecurve1Spec>;
#[doc = "Tone Curve sample points dYn definition (part 1)\n\nNote: The interval widths dYn are to be defined in a 2^(value+3) notation, where \n\n\n\n'value' has to be written to the register. So the steps would be \n\n\n\ndYn=0 -> 8 (2^3), dYn=1 -> 16 (2^4), dYn=2 \n\n\n\n-> 32 (2^5),... dYn=6 -> 512 (2^9), dYn=7 -> \n\n\n\n1024 (2^10). \n\n"]
pub mod wdr_tonecurve_1;
#[doc = "WDR_TONECURVE_2 (rw) register accessor: Tone Curve sample points dYn definition (part 2)\n\nNote: The interval widths dYn are to be defined in a 2^(value+3) notation, where \n\n\n\n'value' has to be written to the register. So the steps would be \n\n\n\ndYn=0 -> 8 (2^3), dYn=1 -> 16 (2^4), dYn=2 \n\n\n\n-> 32 (2^5),... dYn=6 -> 512 (2^9), dYn=7 -> \n\n\n\n1024 (2^10). \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdr_tonecurve_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdr_tonecurve_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdr_tonecurve_2`]
module"]
#[doc(alias = "WDR_TONECURVE_2")]
pub type WdrTonecurve2 = crate::Reg<wdr_tonecurve_2::WdrTonecurve2Spec>;
#[doc = "Tone Curve sample points dYn definition (part 2)\n\nNote: The interval widths dYn are to be defined in a 2^(value+3) notation, where \n\n\n\n'value' has to be written to the register. So the steps would be \n\n\n\ndYn=0 -> 8 (2^3), dYn=1 -> 16 (2^4), dYn=2 \n\n\n\n-> 32 (2^5),... dYn=6 -> 512 (2^9), dYn=7 -> \n\n\n\n1024 (2^10). \n\n"]
pub mod wdr_tonecurve_2;
#[doc = "WDR_TONECURVE_3 (rw) register accessor: Tone Curve sample points dYn definition (part 3)\n\nNote: The interval widths dYn are to be defined in a 2^(value+3) notation, where \n\n\n\n'value' has to be written to the register. So the steps would be \n\n\n\ndYn=0 -> 8 (2^3), dYn=1 -> 16 (2^4), dYn=2 \n\n\n\n-> 32 (2^5),... dYn=6 -> 512 (2^9), dYn=7 -> \n\n\n\n1024 (2^10). \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdr_tonecurve_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdr_tonecurve_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdr_tonecurve_3`]
module"]
#[doc(alias = "WDR_TONECURVE_3")]
pub type WdrTonecurve3 = crate::Reg<wdr_tonecurve_3::WdrTonecurve3Spec>;
#[doc = "Tone Curve sample points dYn definition (part 3)\n\nNote: The interval widths dYn are to be defined in a 2^(value+3) notation, where \n\n\n\n'value' has to be written to the register. So the steps would be \n\n\n\ndYn=0 -> 8 (2^3), dYn=1 -> 16 (2^4), dYn=2 \n\n\n\n-> 32 (2^5),... dYn=6 -> 512 (2^9), dYn=7 -> \n\n\n\n1024 (2^10). \n\n"]
pub mod wdr_tonecurve_3;
#[doc = "WDR_TONECURVE_4 (rw) register accessor: Tone Curve sample points dYn definition (part 4)\n\nNote: The interval widths dYn are to be defined in a 2^(value+3) notation, where 'value' \n\nhas to be written to the register. So the steps would be \n\n\n\ndYn=0 -> 8 (2^3), dYn=1 -> 16 (2^4), dYn=2 -> 32 (2^5),... dYn=6 -> 512 (2^9), \n\ndYn=7 -> 1024 (2^10). \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdr_tonecurve_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdr_tonecurve_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdr_tonecurve_4`]
module"]
#[doc(alias = "WDR_TONECURVE_4")]
pub type WdrTonecurve4 = crate::Reg<wdr_tonecurve_4::WdrTonecurve4Spec>;
#[doc = "Tone Curve sample points dYn definition (part 4)\n\nNote: The interval widths dYn are to be defined in a 2^(value+3) notation, where 'value' \n\nhas to be written to the register. So the steps would be \n\n\n\ndYn=0 -> 8 (2^3), dYn=1 -> 16 (2^4), dYn=2 -> 32 (2^5),... dYn=6 -> 512 (2^9), \n\ndYn=7 -> 1024 (2^10). \n\n\n\n \n\n"]
pub mod wdr_tonecurve_4;
#[doc = "WDR_TONECURVE_YM (rw) register accessor: Tonemapping curve coefficient Ym_ n (n=0..32)\n\nNote: The reset values define a linear curve which has the same effect as bypass. Reset \n\nvalues are: Ym_00 = 0x0000, Ym_01 = 0x0080, Ym_02 = 0x0100, Ym_03 = 0x0180, Ym_04 \n\n= 0x0200, \n\n\n\nYm_05 = 0x0280, Ym_06 = 0x0300, Ym_07 = 0x0380, Ym_08 = 0x0400, Ym_09 = \n\n0x0480, Ym_10 = 0x0500, Ym_11 = 0x0580, Ym_12 = 0x0600, Ym_13 = 0x0680, Ym_14 = \n\n0x0700, Ym_15 = 0x0780, Ym_16 = 0x0800, Ym_17 = 0x0880, Ym_18 = 0x0900, Ym_19 = \n\n0x0980, Ym_20 = 0x0A00, Ym_21 = 0x0A80, Ym_22 = 0x0B00, Ym_23 = 0x0B80, Ym_24 = \n\n0x0C00, Ym_25 = 0x0C80, Ym_26 = 0x0D00, Ym_27 = 0x0D80, Ym_28 = 0x0E00, Ym_29 = \n\n0x0E80, Ym_30 = 0x0F00, Ym_31 = 0x0F80, Ym_32 = 0x1000 \n\n\n\n \n\n\n\nData format: 13 bit unsigned \n\n \n\nRESTRICTION: each Y must be in the +2047/-2048 range compared to its predecessor (so \n\n\n\nthat the difference between successive Y values is 12-bit signed !) \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdr_tonecurve_ym::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdr_tonecurve_ym::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdr_tonecurve_ym`]
module"]
#[doc(alias = "WDR_TONECURVE_YM")]
pub type WdrTonecurveYm = crate::Reg<wdr_tonecurve_ym::WdrTonecurveYmSpec>;
#[doc = "Tonemapping curve coefficient Ym_ n (n=0..32)\n\nNote: The reset values define a linear curve which has the same effect as bypass. Reset \n\nvalues are: Ym_00 = 0x0000, Ym_01 = 0x0080, Ym_02 = 0x0100, Ym_03 = 0x0180, Ym_04 \n\n= 0x0200, \n\n\n\nYm_05 = 0x0280, Ym_06 = 0x0300, Ym_07 = 0x0380, Ym_08 = 0x0400, Ym_09 = \n\n0x0480, Ym_10 = 0x0500, Ym_11 = 0x0580, Ym_12 = 0x0600, Ym_13 = 0x0680, Ym_14 = \n\n0x0700, Ym_15 = 0x0780, Ym_16 = 0x0800, Ym_17 = 0x0880, Ym_18 = 0x0900, Ym_19 = \n\n0x0980, Ym_20 = 0x0A00, Ym_21 = 0x0A80, Ym_22 = 0x0B00, Ym_23 = 0x0B80, Ym_24 = \n\n0x0C00, Ym_25 = 0x0C80, Ym_26 = 0x0D00, Ym_27 = 0x0D80, Ym_28 = 0x0E00, Ym_29 = \n\n0x0E80, Ym_30 = 0x0F00, Ym_31 = 0x0F80, Ym_32 = 0x1000 \n\n\n\n \n\n\n\nData format: 13 bit unsigned \n\n \n\nRESTRICTION: each Y must be in the +2047/-2048 range compared to its predecessor (so \n\n\n\nthat the difference between successive Y values is 12-bit signed !) \n\n\n\n \n\n"]
pub mod wdr_tonecurve_ym;
#[doc = "WDR_OFFSET (rw) register accessor: Offset values for RGB path\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdr_offset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdr_offset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdr_offset`]
module"]
#[doc(alias = "WDR_OFFSET")]
pub type WdrOffset = crate::Reg<wdr_offset::WdrOffsetSpec>;
#[doc = "Offset values for RGB path"]
pub mod wdr_offset;
#[doc = "WDR_DELTAMIN (rw) register accessor: DeltaMin Threshold and Strength factor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdr_deltamin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdr_deltamin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdr_deltamin`]
module"]
#[doc(alias = "WDR_DELTAMIN")]
pub type WdrDeltamin = crate::Reg<wdr_deltamin::WdrDeltaminSpec>;
#[doc = "DeltaMin Threshold and Strength factor"]
pub mod wdr_deltamin;
#[doc = "WDR_TONECURVE_1_SHD (r) register accessor: Tone Curve sample points dYn definition shadow register (part 1)\n\nNote: see register ISP_WDR_TONECURVE_1. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdr_tonecurve_1_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdr_tonecurve_1_shd`]
module"]
#[doc(alias = "WDR_TONECURVE_1_SHD")]
pub type WdrTonecurve1Shd = crate::Reg<wdr_tonecurve_1_shd::WdrTonecurve1ShdSpec>;
#[doc = "Tone Curve sample points dYn definition shadow register (part 1)\n\nNote: see register ISP_WDR_TONECURVE_1. \n\n\n\n \n\n"]
pub mod wdr_tonecurve_1_shd;
#[doc = "WDR_TONECURVE_2_SHD (r) register accessor: Tone Curve sample points dYn definition shadow register (part 2)\n\nNote: see register ISP_WDR_TONECURVE_2. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdr_tonecurve_2_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdr_tonecurve_2_shd`]
module"]
#[doc(alias = "WDR_TONECURVE_2_SHD")]
pub type WdrTonecurve2Shd = crate::Reg<wdr_tonecurve_2_shd::WdrTonecurve2ShdSpec>;
#[doc = "Tone Curve sample points dYn definition shadow register (part 2)\n\nNote: see register ISP_WDR_TONECURVE_2. \n\n\n\n \n\n"]
pub mod wdr_tonecurve_2_shd;
#[doc = "WDR_TONECURVE_3_SHD (r) register accessor: Tone Curve sample points dYn definition shadow register (part 3)\n\nNote: see register ISP_WDR_TONECURVE_3. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdr_tonecurve_3_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdr_tonecurve_3_shd`]
module"]
#[doc(alias = "WDR_TONECURVE_3_SHD")]
pub type WdrTonecurve3Shd = crate::Reg<wdr_tonecurve_3_shd::WdrTonecurve3ShdSpec>;
#[doc = "Tone Curve sample points dYn definition shadow register (part 3)\n\nNote: see register ISP_WDR_TONECURVE_3. \n\n\n\n \n\n"]
pub mod wdr_tonecurve_3_shd;
#[doc = "WDR_TONECURVE_4_SHD (r) register accessor: Tone Curve sample points dYn definition shadow register(part 4)\n\nNote: see register ISP_WDR_TONECURVE_4. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdr_tonecurve_4_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdr_tonecurve_4_shd`]
module"]
#[doc(alias = "WDR_TONECURVE_4_SHD")]
pub type WdrTonecurve4Shd = crate::Reg<wdr_tonecurve_4_shd::WdrTonecurve4ShdSpec>;
#[doc = "Tone Curve sample points dYn definition shadow register(part 4)\n\nNote: see register ISP_WDR_TONECURVE_4. \n\n\n\n \n\n"]
pub mod wdr_tonecurve_4_shd;
#[doc = "WDR_TONECURVE_YM_SHD (r) register accessor: Tonemapping curve coefficient shadow register n (n=0..32)\n\nNote: The reset values define a linear curve which has the same effect as bypass. Reset \n\nvalues are: Ym_00 = 0x0000, Ym_01 = 0x0080, Ym_02 = 0x0100, Ym_03 = 0x0180, Ym_04 \n\n= 0x0200, \n\n\n\nYm_05 = 0x0280, Ym_06 = 0x0300, Ym_07 = 0x0380, Ym_08 = 0x0400, Ym_09 = \n\n0x0480, Ym_10 = 0x0500, Ym_11 = 0x0580, Ym_12 = 0x0600, Ym_13 = 0x0680, Ym_14 = \n\n0x0700, Ym_15 = 0x0780, Ym_16 = 0x0800, Ym_17 = 0x0880, Ym_18 = 0x0900, Ym_19 = \n\n0x0980, Ym_20 = 0x0A00, Ym_21 = 0x0A80, Ym_22 = 0x0B00, Ym_23 = 0x0B80, Ym_24 = \n\n0x0C00, Ym_25 = 0x0C80, Ym_26 = 0x0D00, Ym_27 = 0x0D80, Ym_28 = 0x0E00, Ym_29 = \n\n0x0E80, Ym_30 = 0x0F00, Ym_31 = 0x0F80, Ym_32 = 0x1000 \n\n\n\n \n\nData format: 13 bit unsigned \n\n \n\n\n\nRESTRICTION: each Y must be in the +2047/-2048 range compared to its predecessor (so \n\nthat the difference between successive Y values is 12-bit signed !) \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdr_tonecurve_ym_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdr_tonecurve_ym_shd`]
module"]
#[doc(alias = "WDR_TONECURVE_YM_SHD")]
pub type WdrTonecurveYmShd = crate::Reg<wdr_tonecurve_ym_shd::WdrTonecurveYmShdSpec>;
#[doc = "Tonemapping curve coefficient shadow register n (n=0..32)\n\nNote: The reset values define a linear curve which has the same effect as bypass. Reset \n\nvalues are: Ym_00 = 0x0000, Ym_01 = 0x0080, Ym_02 = 0x0100, Ym_03 = 0x0180, Ym_04 \n\n= 0x0200, \n\n\n\nYm_05 = 0x0280, Ym_06 = 0x0300, Ym_07 = 0x0380, Ym_08 = 0x0400, Ym_09 = \n\n0x0480, Ym_10 = 0x0500, Ym_11 = 0x0580, Ym_12 = 0x0600, Ym_13 = 0x0680, Ym_14 = \n\n0x0700, Ym_15 = 0x0780, Ym_16 = 0x0800, Ym_17 = 0x0880, Ym_18 = 0x0900, Ym_19 = \n\n0x0980, Ym_20 = 0x0A00, Ym_21 = 0x0A80, Ym_22 = 0x0B00, Ym_23 = 0x0B80, Ym_24 = \n\n0x0C00, Ym_25 = 0x0C80, Ym_26 = 0x0D00, Ym_27 = 0x0D80, Ym_28 = 0x0E00, Ym_29 = \n\n0x0E80, Ym_30 = 0x0F00, Ym_31 = 0x0F80, Ym_32 = 0x1000 \n\n\n\n \n\nData format: 13 bit unsigned \n\n \n\n\n\nRESTRICTION: each Y must be in the +2047/-2048 range compared to its predecessor (so \n\nthat the difference between successive Y values is 12-bit signed !) \n\n\n\n \n\n"]
pub mod wdr_tonecurve_ym_shd;
#[doc = "VSM_MODE (rw) register accessor: VS Measure Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vsm_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vsm_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vsm_mode`]
module"]
#[doc(alias = "VSM_MODE")]
pub type VsmMode = crate::Reg<vsm_mode::VsmModeSpec>;
#[doc = "VS Measure Mode"]
pub mod vsm_mode;
#[doc = "VSM_H_OFFS (rw) register accessor: VSM window horizontal offset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vsm_h_offs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vsm_h_offs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vsm_h_offs`]
module"]
#[doc(alias = "VSM_H_OFFS")]
pub type VsmHOffs = crate::Reg<vsm_h_offs::VsmHOffsSpec>;
#[doc = "VSM window horizontal offset"]
pub mod vsm_h_offs;
#[doc = "VSM_V_OFFS (rw) register accessor: VSM window vertical offset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vsm_v_offs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vsm_v_offs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vsm_v_offs`]
module"]
#[doc(alias = "VSM_V_OFFS")]
pub type VsmVOffs = crate::Reg<vsm_v_offs::VsmVOffsSpec>;
#[doc = "VSM window vertical offset"]
pub mod vsm_v_offs;
#[doc = "VSM_H_SIZE (rw) register accessor: Horizontal measure window size\n\nNote: only even values are allowed: vsm_h_size\\[0\\]
not writable and read returns 0. \n\n\n\n \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vsm_h_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vsm_h_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vsm_h_size`]
module"]
#[doc(alias = "VSM_H_SIZE")]
pub type VsmHSize = crate::Reg<vsm_h_size::VsmHSizeSpec>;
#[doc = "Horizontal measure window size\n\nNote: only even values are allowed: vsm_h_size\\[0\\]
not writable and read returns 0. \n\n\n\n \n\n\n\n"]
pub mod vsm_h_size;
#[doc = "VSM_V_SIZE (rw) register accessor: Vertical measure window size\n\nNote: only even values are allowed: vsm_v_size\\[0\\]
not writable and read returns 0. \n\n\n\n \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vsm_v_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vsm_v_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vsm_v_size`]
module"]
#[doc(alias = "VSM_V_SIZE")]
pub type VsmVSize = crate::Reg<vsm_v_size::VsmVSizeSpec>;
#[doc = "Vertical measure window size\n\nNote: only even values are allowed: vsm_v_size\\[0\\]
not writable and read returns 0. \n\n\n\n \n\n\n\n"]
pub mod vsm_v_size;
#[doc = "VSM_H_SEGMENTS (rw) register accessor: Iteration 1 horizontal segments\n\nNote: number of 1st iteration sample points = vsm_h_segments + 1 \n\n\n\n \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vsm_h_segments::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vsm_h_segments::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vsm_h_segments`]
module"]
#[doc(alias = "VSM_H_SEGMENTS")]
pub type VsmHSegments = crate::Reg<vsm_h_segments::VsmHSegmentsSpec>;
#[doc = "Iteration 1 horizontal segments\n\nNote: number of 1st iteration sample points = vsm_h_segments + 1 \n\n\n\n \n\n\n\n"]
pub mod vsm_h_segments;
#[doc = "VSM_V_SEGMENTS (rw) register accessor: Iteration 1 vertical segments\n\nNote: number of 1st iteration sample points = vsm_v_segments + 1 \n\n\n\n \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vsm_v_segments::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vsm_v_segments::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vsm_v_segments`]
module"]
#[doc(alias = "VSM_V_SEGMENTS")]
pub type VsmVSegments = crate::Reg<vsm_v_segments::VsmVSegmentsSpec>;
#[doc = "Iteration 1 vertical segments\n\nNote: number of 1st iteration sample points = vsm_v_segments + 1 \n\n\n\n \n\n\n\n"]
pub mod vsm_v_segments;
#[doc = "VSM_DELTA_H (r) register accessor: estimated horizontal displacement\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vsm_delta_h::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vsm_delta_h`]
module"]
#[doc(alias = "VSM_DELTA_H")]
pub type VsmDeltaH = crate::Reg<vsm_delta_h::VsmDeltaHSpec>;
#[doc = "estimated horizontal displacement"]
pub mod vsm_delta_h;
