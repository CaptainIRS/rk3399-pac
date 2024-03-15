#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    dp_tx_version: DpTxVersion,
    _reserved1: [u8; 0x04],
    func_en_1: FuncEn1,
    func_en_2: FuncEn2,
    video_ctl_1: VideoCtl1,
    video_ctl_2: VideoCtl2,
    video_ctl_3: VideoCtl3,
    video_ctl_4: VideoCtl4,
    _reserved7: [u8; 0x0c],
    video_ctl_8: VideoCtl8,
    _reserved8: [u8; 0x04],
    video_ctl_10: VideoCtl10,
    total_line_cfg_l: TotalLineCfgL,
    total_line_cfg_h: TotalLineCfgH,
    active_line_cfg_l: ActiveLineCfgL,
    active_line_cfg_h: ActiveLineCfgH,
    v_f_porch_cfg: VFPorchCfg,
    v_sync_width_cfg: VSyncWidthCfg,
    v_b_porch_cfg: VBPorchCfg,
    total_pixel_cfg_l: TotalPixelCfgL,
    total_pixel_cfg_h: TotalPixelCfgH,
    active_pixel_cfg_l: ActivePixelCfgL,
    active_pixel_cfg_h: ActivePixelCfgH,
    h_f_porch_cfg_l: HFPorchCfgL,
    h_f_porch_cfg_h: HFPorchCfgH,
    h_sync_cfg_l: HSyncCfgL,
    h_sync_cfg_h: HSyncCfgH,
    h_b_porch_cfg_l: HBPorchCfgL,
    h_b_porch_cfg_h: HBPorchCfgH,
    video_status: VideoStatus,
    total_line_sta_l: TotalLineStaL,
    total_line_sta_h: TotalLineStaH,
    active_line_sta_l: ActiveLineStaL,
    active_line_sta_h: ActiveLineStaH,
    v_f_porch_sta: VFPorchSta,
    v_sync_sta: VSyncSta,
    v_b_porch_sta: VBPorchSta,
    total_pixel_sta_l: TotalPixelStaL,
    total_pixel_sta_h: TotalPixelStaH,
    active_pixel_sta_l: ActivePixelStaL,
    active_pixel_sta_h: ActivePixelStaH,
    h_f_porch_sta_l: HFPorchStaL,
    h_f_porch_sta_h: HFPorchStaH,
    h_sync_sta_l: HSyncStaL,
    h_sync_sta_h: HSyncStaH,
    h_b_porch_sta_l: HBPorchStaL,
    h_b_porch_sta_h: HBPorchStaH,
    _reserved44: [u8; 0x28],
    pll_reg_1: PllReg1,
    _reserved45: [u8; 0x04],
    ssc_reg: SscReg,
    _reserved46: [u8; 0x0c],
    tx_common: TxCommon,
    tx_common2: TxCommon2,
    _reserved48: [u8; 0x04],
    dp_aux: DpAux,
    dp_bias: DpBias,
    dp_test: DpTest,
    dp_pd: DpPd,
    dp_reserv1: DpReserv1,
    dp_reserv2: DpReserv2,
    _reserved54: [u8; 0x98],
    avi_db: [AviDb; 13],
    _reserved55: [u8; 0x40],
    if_type: IfType,
    _reserved56: [u8; 0x0c],
    if_pkt_db: [IfPktDb; 25],
    _reserved57: [u8; 0x18],
    mpeg_db: [MpegDb; 10],
    _reserved58: [u8; 0x20],
    psr_frame_updata_ctrl: PsrFrameUpdataCtrl,
    vsc_shadow_db: [VscShadowDb; 8],
    vsc_shadow_pb: [VscShadowPb; 2],
    _reserved61: [u8; 0x18],
    lane_map: LaneMap,
    _reserved62: [u8; 0x14],
    analog_ctl_2: AnalogCtl2,
    _reserved63: [u8; 0x18],
    int_state_0: IntState0,
    _reserved64: [u8; 0x2c],
    int_state_1: IntState1,
    common_int_sta_1: CommonIntSta1,
    _reserved66: [u8; 0x04],
    common_int_sta_3: CommonIntSta3,
    common_int_sta_4: CommonIntSta4,
    _reserved68: [u8; 0x08],
    dp_int_sta: DpIntSta,
    common_int_mask_1: CommonIntMask1,
    _reserved70: [u8; 0x04],
    common_int_mask_3: CommonIntMask3,
    common_int_mask_4: CommonIntMask4,
    _reserved72: [u8; 0x08],
    dp_int_sta_mask: DpIntStaMask,
    int_ctl: IntCtl,
    _reserved74: [u8; 0x0200],
    sys_ctl_1: SysCtl1,
    sys_ctl_2: SysCtl2,
    sys_ctl_3: SysCtl3,
    sys_ctl_4: SysCtl4,
    dp_vid_ctl: DpVidCtl,
    _reserved79: [u8; 0x2c],
    pkt_send_ctl: PktSendCtl,
    _reserved80: [u8; 0x3c],
    link_bw_set: LinkBwSet,
    lane_count_set: LaneCountSet,
    dp_training_ptn_set: DpTrainingPtnSet,
    dp_ln0_link_training_ctl: DpLn0LinkTrainingCtl,
    dp_ln1_link_training_ctl: DpLn1LinkTrainingCtl,
    dp_ln2_link_training_ctl: DpLn2LinkTrainingCtl,
    dp_ln3_link_training_ctl: DpLn3LinkTrainingCtl,
    _reserved87: [u8; 0x04],
    dp_hw_link_training_ctl: DpHwLinkTrainingCtl,
    _reserved88: [u8; 0x1c],
    dp_debug_ctl: DpDebugCtl,
    hpd_deglitch_l: HpdDeglitchL,
    hpd_deglitch_h: HpdDeglitchH,
    polling_period: PollingPeriod,
    _reserved92: [u8; 0x10],
    dp_link_debug_ctl: DpLinkDebugCtl,
    dp_sink_count: DpSinkCount,
    dp_irq_vector: DpIrqVector,
    dp_link_status0: DpLinkStatus0,
    dp_link_status1: DpLinkStatus1,
    dp_align_status: DpAlignStatus,
    dp_sink_status: DpSinkStatus,
    _reserved99: [u8; 0x04],
    m_vid_0: MVid0,
    m_vid_1: MVid1,
    m_vid_2: MVid2,
    n_vid_0: NVid0,
    n_vid_1: NVid1,
    n_vid_2: NVid2,
    m_vid_mon: MVidMon,
    _reserved106: [u8; 0x14],
    dp_video_fifo_thrd: DpVideoFifoThrd,
    _reserved107: [u8; 0x2c],
    dp_m_cal_ctl: DpMCalCtl,
    m_vid_gen_filter_th: MVidGenFilterTh,
    _reserved109: [u8; 0x18],
    aux_ch_sta: AuxChSta,
    aux_err_num: AuxErrNum,
    aux_ch_defer_ctl: AuxChDeferCtl,
    aux_rx_comm: AuxRxComm,
    buffer_data_ctl: BufferDataCtl,
    aux_ch_ctl_1: AuxChCtl1,
    aux_addr_7_0: AuxAddr7_0,
    aux_addr_15_8: AuxAddr15_8,
    aux_addr_19_16: AuxAddr19_16,
    aux_ch_ctl_2: AuxChCtl2,
    _reserved119: [u8; 0x18],
    buf_data_: [BufData_; 16],
    _reserved120: [u8; 0x04],
    ate_test_ctl: AteTestCtl,
    ate_test_status: AteTestStatus,
    ate_test_err_cnt: [AteTestErrCnt; 4],
    dp_test_80b_pattern0: DpTest80bPattern0,
    dp_test_80b_pattern1: DpTest80bPattern1,
    dp_test_80b_pattern2: DpTest80bPattern2,
    dp_test_hbr2_pattern: DpTestHbr2Pattern,
    _reserved127: [u8; 0x64],
    crc_con: CrcCon,
    _reserved128: [u8; 0x80],
    analog_ctl_5: AnalogCtl5,
    analog_ctl_6: AnalogCtl6,
    analog_ctl_7: AnalogCtl7,
    analog_ctl_8: AnalogCtl8,
    analog_ctl_9: AnalogCtl9,
    analog_ctl_10: AnalogCtl10,
    analog_ctl_11: AnalogCtl11,
    analog_ctl_12: AnalogCtl12,
    analog_ctl_13: AnalogCtl13,
    analog_ctl_14: AnalogCtl14,
    analog_ctl_15: AnalogCtl15,
    analog_ctl_16: AnalogCtl16,
    analog_ctl_17: AnalogCtl17,
    analog_ctl_18: AnalogCtl18,
    analog_ctl_19: AnalogCtl19,
    analog_ctl_20: AnalogCtl20,
    analog_ctl_21: AnalogCtl21,
    analog_ctl_22: AnalogCtl22,
    analog_ctl_23: AnalogCtl23,
    analog_ctl_24: AnalogCtl24,
    analog_ctl_25: AnalogCtl25,
    analog_ctl_26: AnalogCtl26,
    analog_ctl_27: AnalogCtl27,
    analog_ctl_28: AnalogCtl28,
    analog_ctl_29: AnalogCtl29,
    analog_ctl_30: AnalogCtl30,
    analog_ctl_31: AnalogCtl31,
    analog_ctl_32: AnalogCtl32,
    analog_ctl_33: AnalogCtl33,
    analog_ctl_34: AnalogCtl34,
    analog_ctl_35: AnalogCtl35,
    analog_ctl_36: AnalogCtl36,
    analog_ctl_37: AnalogCtl37,
    analog_ctl_38: AnalogCtl38,
    analog_ctl_39: AnalogCtl39,
    analog_ctl_40: AnalogCtl40,
    analog_ctl_41: AnalogCtl41,
    analog_ctl_42: AnalogCtl42,
    analog_ctl_43: AnalogCtl43,
    analog_ctl_44: AnalogCtl44,
    analog_ctl_45: AnalogCtl45,
    analog_ctl_46: AnalogCtl46,
    analog_ctl_47: AnalogCtl47,
    analog_ctl_48: AnalogCtl48,
    analog_ctl_49: AnalogCtl49,
    _reserved173: [u8; 0x10],
    link_policy: LinkPolicy,
    _reserved174: [u8; 0x08],
    pll_reg_2: PllReg2,
    pll_reg_3: PllReg3,
    pll_reg_4: PllReg4,
    _reserved177: [u8; 0x10],
    pll_reg_5: PllReg5,
    pll_reg_mac: PllRegMac,
    tx_common3: TxCommon3,
    _reserved180: [u8; 0x04],
    freq_in_reg: FreqInReg,
    p_reg_frq: PRegFrq,
    p_reg_frq_count_rdy: PRegFrqCountRdy,
    p_band_dec_reset: PBandDecReset,
}
impl RegisterBlock {
    #[doc = "0x10 - DP_TX version register"]
    #[inline(always)]
    pub const fn dp_tx_version(&self) -> &DpTxVersion {
        &self.dp_tx_version
    }
    #[doc = "0x18 - Function Enable Register 1"]
    #[inline(always)]
    pub const fn func_en_1(&self) -> &FuncEn1 {
        &self.func_en_1
    }
    #[doc = "0x1c - Function Enable Register 2"]
    #[inline(always)]
    pub const fn func_en_2(&self) -> &FuncEn2 {
        &self.func_en_2
    }
    #[doc = "0x20 - Video Control 1"]
    #[inline(always)]
    pub const fn video_ctl_1(&self) -> &VideoCtl1 {
        &self.video_ctl_1
    }
    #[doc = "0x24 - Video Control 2"]
    #[inline(always)]
    pub const fn video_ctl_2(&self) -> &VideoCtl2 {
        &self.video_ctl_2
    }
    #[doc = "0x28 - Video Control 3"]
    #[inline(always)]
    pub const fn video_ctl_3(&self) -> &VideoCtl3 {
        &self.video_ctl_3
    }
    #[doc = "0x2c - Video Control 4"]
    #[inline(always)]
    pub const fn video_ctl_4(&self) -> &VideoCtl4 {
        &self.video_ctl_4
    }
    #[doc = "0x3c - Video Control 8"]
    #[inline(always)]
    pub const fn video_ctl_8(&self) -> &VideoCtl8 {
        &self.video_ctl_8
    }
    #[doc = "0x44 - Video Control 10"]
    #[inline(always)]
    pub const fn video_ctl_10(&self) -> &VideoCtl10 {
        &self.video_ctl_10
    }
    #[doc = "0x48 - Total Line Low Byte Configure Register"]
    #[inline(always)]
    pub const fn total_line_cfg_l(&self) -> &TotalLineCfgL {
        &self.total_line_cfg_l
    }
    #[doc = "0x4c - Total Line High Byte Configure Register"]
    #[inline(always)]
    pub const fn total_line_cfg_h(&self) -> &TotalLineCfgH {
        &self.total_line_cfg_h
    }
    #[doc = "0x50 - Active Line Low Byte Configure Register"]
    #[inline(always)]
    pub const fn active_line_cfg_l(&self) -> &ActiveLineCfgL {
        &self.active_line_cfg_l
    }
    #[doc = "0x54 - Active Line High Byte Configure Register"]
    #[inline(always)]
    pub const fn active_line_cfg_h(&self) -> &ActiveLineCfgH {
        &self.active_line_cfg_h
    }
    #[doc = "0x58 - Vertical Front Porch Configure Register"]
    #[inline(always)]
    pub const fn v_f_porch_cfg(&self) -> &VFPorchCfg {
        &self.v_f_porch_cfg
    }
    #[doc = "0x5c - Vertical Sync Width Configure Register"]
    #[inline(always)]
    pub const fn v_sync_width_cfg(&self) -> &VSyncWidthCfg {
        &self.v_sync_width_cfg
    }
    #[doc = "0x60 - Vertical Back Porch Configure Register"]
    #[inline(always)]
    pub const fn v_b_porch_cfg(&self) -> &VBPorchCfg {
        &self.v_b_porch_cfg
    }
    #[doc = "0x64 - Total Pixel Low Byte Configure Register"]
    #[inline(always)]
    pub const fn total_pixel_cfg_l(&self) -> &TotalPixelCfgL {
        &self.total_pixel_cfg_l
    }
    #[doc = "0x68 - Total Pixel High Byte Configure Register"]
    #[inline(always)]
    pub const fn total_pixel_cfg_h(&self) -> &TotalPixelCfgH {
        &self.total_pixel_cfg_h
    }
    #[doc = "0x6c - Active Pixel Low Byte Configure Register"]
    #[inline(always)]
    pub const fn active_pixel_cfg_l(&self) -> &ActivePixelCfgL {
        &self.active_pixel_cfg_l
    }
    #[doc = "0x70 - Active Pixel High Byte Configure Register"]
    #[inline(always)]
    pub const fn active_pixel_cfg_h(&self) -> &ActivePixelCfgH {
        &self.active_pixel_cfg_h
    }
    #[doc = "0x74 - Horizon Front Porch Low Byte Configure Register"]
    #[inline(always)]
    pub const fn h_f_porch_cfg_l(&self) -> &HFPorchCfgL {
        &self.h_f_porch_cfg_l
    }
    #[doc = "0x78 - Horizon Front Porch High Byte Configure Register"]
    #[inline(always)]
    pub const fn h_f_porch_cfg_h(&self) -> &HFPorchCfgH {
        &self.h_f_porch_cfg_h
    }
    #[doc = "0x7c - Horizon Sync Width Low Byte Configure Register"]
    #[inline(always)]
    pub const fn h_sync_cfg_l(&self) -> &HSyncCfgL {
        &self.h_sync_cfg_l
    }
    #[doc = "0x80 - Horizon Sync Width High Byte Configure Register"]
    #[inline(always)]
    pub const fn h_sync_cfg_h(&self) -> &HSyncCfgH {
        &self.h_sync_cfg_h
    }
    #[doc = "0x84 - Horizon Back Porch Low Byte Configure Register"]
    #[inline(always)]
    pub const fn h_b_porch_cfg_l(&self) -> &HBPorchCfgL {
        &self.h_b_porch_cfg_l
    }
    #[doc = "0x88 - Horizon Back Porch High Byte Configure Register"]
    #[inline(always)]
    pub const fn h_b_porch_cfg_h(&self) -> &HBPorchCfgH {
        &self.h_b_porch_cfg_h
    }
    #[doc = "0x8c - Video Status Register"]
    #[inline(always)]
    pub const fn video_status(&self) -> &VideoStatus {
        &self.video_status
    }
    #[doc = "0x90 - Total Line Status Low Byte Register"]
    #[inline(always)]
    pub const fn total_line_sta_l(&self) -> &TotalLineStaL {
        &self.total_line_sta_l
    }
    #[doc = "0x94 - Total Line Status High Byte Register"]
    #[inline(always)]
    pub const fn total_line_sta_h(&self) -> &TotalLineStaH {
        &self.total_line_sta_h
    }
    #[doc = "0x98 - Active Line Status Low Byte Register"]
    #[inline(always)]
    pub const fn active_line_sta_l(&self) -> &ActiveLineStaL {
        &self.active_line_sta_l
    }
    #[doc = "0x9c - Active Line Status High Byte Register"]
    #[inline(always)]
    pub const fn active_line_sta_h(&self) -> &ActiveLineStaH {
        &self.active_line_sta_h
    }
    #[doc = "0xa0 - Vertical Front Porch Status Register"]
    #[inline(always)]
    pub const fn v_f_porch_sta(&self) -> &VFPorchSta {
        &self.v_f_porch_sta
    }
    #[doc = "0xa4 - Vertical Sync Width Status Register"]
    #[inline(always)]
    pub const fn v_sync_sta(&self) -> &VSyncSta {
        &self.v_sync_sta
    }
    #[doc = "0xa8 - Vertical Back Porch Status Register"]
    #[inline(always)]
    pub const fn v_b_porch_sta(&self) -> &VBPorchSta {
        &self.v_b_porch_sta
    }
    #[doc = "0xac - Total Pixel Status Low"]
    #[inline(always)]
    pub const fn total_pixel_sta_l(&self) -> &TotalPixelStaL {
        &self.total_pixel_sta_l
    }
    #[doc = "0xb0 - Total Pixel Status High Byte Register"]
    #[inline(always)]
    pub const fn total_pixel_sta_h(&self) -> &TotalPixelStaH {
        &self.total_pixel_sta_h
    }
    #[doc = "0xb4 - Active Pixel Status Low Byte Register"]
    #[inline(always)]
    pub const fn active_pixel_sta_l(&self) -> &ActivePixelStaL {
        &self.active_pixel_sta_l
    }
    #[doc = "0xb8 - Active Pixel Status High Byte Register"]
    #[inline(always)]
    pub const fn active_pixel_sta_h(&self) -> &ActivePixelStaH {
        &self.active_pixel_sta_h
    }
    #[doc = "0xbc - Horizon Front Porch Status Low Byte Register"]
    #[inline(always)]
    pub const fn h_f_porch_sta_l(&self) -> &HFPorchStaL {
        &self.h_f_porch_sta_l
    }
    #[doc = "0xc0 - Horizon Front Porch Status High Byte Register"]
    #[inline(always)]
    pub const fn h_f_porch_sta_h(&self) -> &HFPorchStaH {
        &self.h_f_porch_sta_h
    }
    #[doc = "0xc4 - Horizon Sync Width Status Low Byte Register"]
    #[inline(always)]
    pub const fn h_sync_sta_l(&self) -> &HSyncStaL {
        &self.h_sync_sta_l
    }
    #[doc = "0xc8 - Horizon Sync Width Status High Byte Register"]
    #[inline(always)]
    pub const fn h_sync_sta_h(&self) -> &HSyncStaH {
        &self.h_sync_sta_h
    }
    #[doc = "0xcc - Horizon Back Porch Status Low Byte Register"]
    #[inline(always)]
    pub const fn h_b_porch_sta_l(&self) -> &HBPorchStaL {
        &self.h_b_porch_sta_l
    }
    #[doc = "0xd0 - Horizon Back Porch Status High Byte Register"]
    #[inline(always)]
    pub const fn h_b_porch_sta_h(&self) -> &HBPorchStaH {
        &self.h_b_porch_sta_h
    }
    #[doc = "0xfc - Pll_control_1"]
    #[inline(always)]
    pub const fn pll_reg_1(&self) -> &PllReg1 {
        &self.pll_reg_1
    }
    #[doc = "0x104 - SSC control"]
    #[inline(always)]
    pub const fn ssc_reg(&self) -> &SscReg {
        &self.ssc_reg
    }
    #[doc = "0x114 - Tx terminal resistor control"]
    #[inline(always)]
    pub const fn tx_common(&self) -> &TxCommon {
        &self.tx_common
    }
    #[doc = "0x118 - Tx terminal resistor control2"]
    #[inline(always)]
    pub const fn tx_common2(&self) -> &TxCommon2 {
        &self.tx_common2
    }
    #[doc = "0x120 - Aux control"]
    #[inline(always)]
    pub const fn dp_aux(&self) -> &DpAux {
        &self.dp_aux
    }
    #[doc = "0x124 - Bias control"]
    #[inline(always)]
    pub const fn dp_bias(&self) -> &DpBias {
        &self.dp_bias
    }
    #[doc = "0x128 - Test mode"]
    #[inline(always)]
    pub const fn dp_test(&self) -> &DpTest {
        &self.dp_test
    }
    #[doc = "0x12c - Power down control"]
    #[inline(always)]
    pub const fn dp_pd(&self) -> &DpPd {
        &self.dp_pd
    }
    #[doc = "0x130 - RESERVD1"]
    #[inline(always)]
    pub const fn dp_reserv1(&self) -> &DpReserv1 {
        &self.dp_reserv1
    }
    #[doc = "0x134 - RESERVD2"]
    #[inline(always)]
    pub const fn dp_reserv2(&self) -> &DpReserv2 {
        &self.dp_reserv2
    }
    #[doc = "0x1d0..0x204 - AVI InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn avi_db(&self, n: usize) -> &AviDb {
        &self.avi_db[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1d0..0x204 - AVI InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub fn avi_db_iter(&self) -> impl Iterator<Item = &AviDb> {
        self.avi_db.iter()
    }
    #[doc = "0x1d0 - AVI InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn avi_db1(&self) -> &AviDb {
        self.avi_db(0)
    }
    #[doc = "0x1d4 - AVI InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn avi_db2(&self) -> &AviDb {
        self.avi_db(1)
    }
    #[doc = "0x1d8 - AVI InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn avi_db3(&self) -> &AviDb {
        self.avi_db(2)
    }
    #[doc = "0x1dc - AVI InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn avi_db4(&self) -> &AviDb {
        self.avi_db(3)
    }
    #[doc = "0x1e0 - AVI InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn avi_db5(&self) -> &AviDb {
        self.avi_db(4)
    }
    #[doc = "0x1e4 - AVI InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn avi_db6(&self) -> &AviDb {
        self.avi_db(5)
    }
    #[doc = "0x1e8 - AVI InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn avi_db7(&self) -> &AviDb {
        self.avi_db(6)
    }
    #[doc = "0x1ec - AVI InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn avi_db8(&self) -> &AviDb {
        self.avi_db(7)
    }
    #[doc = "0x1f0 - AVI InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn avi_db9(&self) -> &AviDb {
        self.avi_db(8)
    }
    #[doc = "0x1f4 - AVI InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn avi_db10(&self) -> &AviDb {
        self.avi_db(9)
    }
    #[doc = "0x1f8 - AVI InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn avi_db11(&self) -> &AviDb {
        self.avi_db(10)
    }
    #[doc = "0x1fc - AVI InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn avi_db12(&self) -> &AviDb {
        self.avi_db(11)
    }
    #[doc = "0x200 - AVI InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn avi_db13(&self) -> &AviDb {
        self.avi_db(12)
    }
    #[doc = "0x244 - InfoFrame Packet Type Code."]
    #[inline(always)]
    pub const fn if_type(&self) -> &IfType {
        &self.if_type
    }
    #[doc = "0x254..0x2b8 - InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn if_pkt_db(&self, n: usize) -> &IfPktDb {
        &self.if_pkt_db[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x254..0x2b8 - InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub fn if_pkt_db_iter(&self) -> impl Iterator<Item = &IfPktDb> {
        self.if_pkt_db.iter()
    }
    #[doc = "0x254 - InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn if_pkt_db1(&self) -> &IfPktDb {
        self.if_pkt_db(0)
    }
    #[doc = "0x258 - InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn if_pkt_db2(&self) -> &IfPktDb {
        self.if_pkt_db(1)
    }
    #[doc = "0x25c - InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn if_pkt_db3(&self) -> &IfPktDb {
        self.if_pkt_db(2)
    }
    #[doc = "0x260 - InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn if_pkt_db4(&self) -> &IfPktDb {
        self.if_pkt_db(3)
    }
    #[doc = "0x264 - InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn if_pkt_db5(&self) -> &IfPktDb {
        self.if_pkt_db(4)
    }
    #[doc = "0x268 - InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn if_pkt_db6(&self) -> &IfPktDb {
        self.if_pkt_db(5)
    }
    #[doc = "0x26c - InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn if_pkt_db7(&self) -> &IfPktDb {
        self.if_pkt_db(6)
    }
    #[doc = "0x270 - InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn if_pkt_db8(&self) -> &IfPktDb {
        self.if_pkt_db(7)
    }
    #[doc = "0x274 - InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn if_pkt_db9(&self) -> &IfPktDb {
        self.if_pkt_db(8)
    }
    #[doc = "0x278 - InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn if_pkt_db10(&self) -> &IfPktDb {
        self.if_pkt_db(9)
    }
    #[doc = "0x27c - InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn if_pkt_db11(&self) -> &IfPktDb {
        self.if_pkt_db(10)
    }
    #[doc = "0x280 - InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn if_pkt_db12(&self) -> &IfPktDb {
        self.if_pkt_db(11)
    }
    #[doc = "0x284 - InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn if_pkt_db13(&self) -> &IfPktDb {
        self.if_pkt_db(12)
    }
    #[doc = "0x288 - InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn if_pkt_db14(&self) -> &IfPktDb {
        self.if_pkt_db(13)
    }
    #[doc = "0x28c - InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn if_pkt_db15(&self) -> &IfPktDb {
        self.if_pkt_db(14)
    }
    #[doc = "0x290 - InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn if_pkt_db16(&self) -> &IfPktDb {
        self.if_pkt_db(15)
    }
    #[doc = "0x294 - InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn if_pkt_db17(&self) -> &IfPktDb {
        self.if_pkt_db(16)
    }
    #[doc = "0x298 - InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn if_pkt_db18(&self) -> &IfPktDb {
        self.if_pkt_db(17)
    }
    #[doc = "0x29c - InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn if_pkt_db19(&self) -> &IfPktDb {
        self.if_pkt_db(18)
    }
    #[doc = "0x2a0 - InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn if_pkt_db20(&self) -> &IfPktDb {
        self.if_pkt_db(19)
    }
    #[doc = "0x2a4 - InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn if_pkt_db21(&self) -> &IfPktDb {
        self.if_pkt_db(20)
    }
    #[doc = "0x2a8 - InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn if_pkt_db22(&self) -> &IfPktDb {
        self.if_pkt_db(21)
    }
    #[doc = "0x2ac - InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn if_pkt_db23(&self) -> &IfPktDb {
        self.if_pkt_db(22)
    }
    #[doc = "0x2b0 - InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn if_pkt_db24(&self) -> &IfPktDb {
        self.if_pkt_db(23)
    }
    #[doc = "0x2b4 - InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn if_pkt_db25(&self) -> &IfPktDb {
        self.if_pkt_db(24)
    }
    #[doc = "0x2d0..0x2f8 - MPEG Source InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn mpeg_db(&self, n: usize) -> &MpegDb {
        &self.mpeg_db[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2d0..0x2f8 - MPEG Source InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub fn mpeg_db_iter(&self) -> impl Iterator<Item = &MpegDb> {
        self.mpeg_db.iter()
    }
    #[doc = "0x2d0 - MPEG Source InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn mpeg_db1(&self) -> &MpegDb {
        self.mpeg_db(0)
    }
    #[doc = "0x2d4 - MPEG Source InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn mpeg_db2(&self) -> &MpegDb {
        self.mpeg_db(1)
    }
    #[doc = "0x2d8 - MPEG Source InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn mpeg_db3(&self) -> &MpegDb {
        self.mpeg_db(2)
    }
    #[doc = "0x2dc - MPEG Source InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn mpeg_db4(&self) -> &MpegDb {
        self.mpeg_db(3)
    }
    #[doc = "0x2e0 - MPEG Source InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn mpeg_db5(&self) -> &MpegDb {
        self.mpeg_db(4)
    }
    #[doc = "0x2e4 - MPEG Source InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn mpeg_db6(&self) -> &MpegDb {
        self.mpeg_db(5)
    }
    #[doc = "0x2e8 - MPEG Source InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn mpeg_db7(&self) -> &MpegDb {
        self.mpeg_db(6)
    }
    #[doc = "0x2ec - MPEG Source InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn mpeg_db8(&self) -> &MpegDb {
        self.mpeg_db(7)
    }
    #[doc = "0x2f0 - MPEG Source InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn mpeg_db9(&self) -> &MpegDb {
        self.mpeg_db(8)
    }
    #[doc = "0x2f4 - MPEG Source InfoFrame Packet Data Byte"]
    #[inline(always)]
    pub const fn mpeg_db10(&self) -> &MpegDb {
        self.mpeg_db(9)
    }
    #[doc = "0x318 - Frame update control for PSR"]
    #[inline(always)]
    pub const fn psr_frame_updata_ctrl(&self) -> &PsrFrameUpdataCtrl {
        &self.psr_frame_updata_ctrl
    }
    #[doc = "0x31c..0x33c - VSC shadow data bytes 0 ~ 7"]
    #[inline(always)]
    pub const fn vsc_shadow_db(&self, n: usize) -> &VscShadowDb {
        &self.vsc_shadow_db[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x31c..0x33c - VSC shadow data bytes 0 ~ 7"]
    #[inline(always)]
    pub fn vsc_shadow_db_iter(&self) -> impl Iterator<Item = &VscShadowDb> {
        self.vsc_shadow_db.iter()
    }
    #[doc = "0x33c..0x344 - VSC shadow parity byte 0 ~ 1"]
    #[inline(always)]
    pub const fn vsc_shadow_pb(&self, n: usize) -> &VscShadowPb {
        &self.vsc_shadow_pb[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x33c..0x344 - VSC shadow parity byte 0 ~ 1"]
    #[inline(always)]
    pub fn vsc_shadow_pb_iter(&self) -> impl Iterator<Item = &VscShadowPb> {
        self.vsc_shadow_pb.iter()
    }
    #[doc = "0x35c - Lane Map Register"]
    #[inline(always)]
    pub const fn lane_map(&self) -> &LaneMap {
        &self.lane_map
    }
    #[doc = "0x374 - Analog Control Register 2"]
    #[inline(always)]
    pub const fn analog_ctl_2(&self) -> &AnalogCtl2 {
        &self.analog_ctl_2
    }
    #[doc = "0x390 - Debug Register"]
    #[inline(always)]
    pub const fn int_state_0(&self) -> &IntState0 {
        &self.int_state_0
    }
    #[doc = "0x3c0 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn int_state_1(&self) -> &IntState1 {
        &self.int_state_1
    }
    #[doc = "0x3c4 - Common Interrupt Status Register 1"]
    #[inline(always)]
    pub const fn common_int_sta_1(&self) -> &CommonIntSta1 {
        &self.common_int_sta_1
    }
    #[doc = "0x3cc - Common Interrupt Status Register 3"]
    #[inline(always)]
    pub const fn common_int_sta_3(&self) -> &CommonIntSta3 {
        &self.common_int_sta_3
    }
    #[doc = "0x3d0 - Common Interrupt Status Register 4"]
    #[inline(always)]
    pub const fn common_int_sta_4(&self) -> &CommonIntSta4 {
        &self.common_int_sta_4
    }
    #[doc = "0x3dc - DisplayPort Interrupt Status Register"]
    #[inline(always)]
    pub const fn dp_int_sta(&self) -> &DpIntSta {
        &self.dp_int_sta
    }
    #[doc = "0x3e0 - Common Interrupt Mask Register1"]
    #[inline(always)]
    pub const fn common_int_mask_1(&self) -> &CommonIntMask1 {
        &self.common_int_mask_1
    }
    #[doc = "0x3e8 - Common Interrupt Mask Register3"]
    #[inline(always)]
    pub const fn common_int_mask_3(&self) -> &CommonIntMask3 {
        &self.common_int_mask_3
    }
    #[doc = "0x3ec - Common Interrupt Mask Register4"]
    #[inline(always)]
    pub const fn common_int_mask_4(&self) -> &CommonIntMask4 {
        &self.common_int_mask_4
    }
    #[doc = "0x3f8 - DisplayPort Interrupt enable Register"]
    #[inline(always)]
    pub const fn dp_int_sta_mask(&self) -> &DpIntStaMask {
        &self.dp_int_sta_mask
    }
    #[doc = "0x3fc - Interrupt Control Register"]
    #[inline(always)]
    pub const fn int_ctl(&self) -> &IntCtl {
        &self.int_ctl
    }
    #[doc = "0x600 - System Control Register #1"]
    #[inline(always)]
    pub const fn sys_ctl_1(&self) -> &SysCtl1 {
        &self.sys_ctl_1
    }
    #[doc = "0x604 - System Control Register #2"]
    #[inline(always)]
    pub const fn sys_ctl_2(&self) -> &SysCtl2 {
        &self.sys_ctl_2
    }
    #[doc = "0x608 - System Control Register #3"]
    #[inline(always)]
    pub const fn sys_ctl_3(&self) -> &SysCtl3 {
        &self.sys_ctl_3
    }
    #[doc = "0x60c - System Control Register #4"]
    #[inline(always)]
    pub const fn sys_ctl_4(&self) -> &SysCtl4 {
        &self.sys_ctl_4
    }
    #[doc = "0x610 - DP Video Control Register"]
    #[inline(always)]
    pub const fn dp_vid_ctl(&self) -> &DpVidCtl {
        &self.dp_vid_ctl
    }
    #[doc = "0x640 - Packet Send Control Register"]
    #[inline(always)]
    pub const fn pkt_send_ctl(&self) -> &PktSendCtl {
        &self.pkt_send_ctl
    }
    #[doc = "0x680 - Main Link Bandwidth Setting Register"]
    #[inline(always)]
    pub const fn link_bw_set(&self) -> &LinkBwSet {
        &self.link_bw_set
    }
    #[doc = "0x684 - DP Main Link Lane Number Register"]
    #[inline(always)]
    pub const fn lane_count_set(&self) -> &LaneCountSet {
        &self.lane_count_set
    }
    #[doc = "0x688 - DP Training Pattern Set Register"]
    #[inline(always)]
    pub const fn dp_training_ptn_set(&self) -> &DpTrainingPtnSet {
        &self.dp_training_ptn_set
    }
    #[doc = "0x68c - DP Lane 0 Link Training Control Register"]
    #[inline(always)]
    pub const fn dp_ln0_link_training_ctl(&self) -> &DpLn0LinkTrainingCtl {
        &self.dp_ln0_link_training_ctl
    }
    #[doc = "0x690 - DP Lane 1 Link Training Control Register"]
    #[inline(always)]
    pub const fn dp_ln1_link_training_ctl(&self) -> &DpLn1LinkTrainingCtl {
        &self.dp_ln1_link_training_ctl
    }
    #[doc = "0x694 - DP Lane 2 Link Training Control Register"]
    #[inline(always)]
    pub const fn dp_ln2_link_training_ctl(&self) -> &DpLn2LinkTrainingCtl {
        &self.dp_ln2_link_training_ctl
    }
    #[doc = "0x698 - DP Lane 3 Link Training Control Register"]
    #[inline(always)]
    pub const fn dp_ln3_link_training_ctl(&self) -> &DpLn3LinkTrainingCtl {
        &self.dp_ln3_link_training_ctl
    }
    #[doc = "0x6a0 - DP HW LINK TRAINING_CONTROL Register"]
    #[inline(always)]
    pub const fn dp_hw_link_training_ctl(&self) -> &DpHwLinkTrainingCtl {
        &self.dp_hw_link_training_ctl
    }
    #[doc = "0x6c0 - DP Debug Control Register #1"]
    #[inline(always)]
    pub const fn dp_debug_ctl(&self) -> &DpDebugCtl {
        &self.dp_debug_ctl
    }
    #[doc = "0x6c4 - DP HPD De-glitch Low Byte Register"]
    #[inline(always)]
    pub const fn hpd_deglitch_l(&self) -> &HpdDeglitchL {
        &self.hpd_deglitch_l
    }
    #[doc = "0x6c8 - DP HPD De-glitch High Byte Register"]
    #[inline(always)]
    pub const fn hpd_deglitch_h(&self) -> &HpdDeglitchH {
        &self.hpd_deglitch_h
    }
    #[doc = "0x6cc - DP polling period"]
    #[inline(always)]
    pub const fn polling_period(&self) -> &PollingPeriod {
        &self.polling_period
    }
    #[doc = "0x6e0 - DP Link Debug Control Register"]
    #[inline(always)]
    pub const fn dp_link_debug_ctl(&self) -> &DpLinkDebugCtl {
        &self.dp_link_debug_ctl
    }
    #[doc = "0x6e4 - DP Sink Count"]
    #[inline(always)]
    pub const fn dp_sink_count(&self) -> &DpSinkCount {
        &self.dp_sink_count
    }
    #[doc = "0x6e8 - DP Irq Vector"]
    #[inline(always)]
    pub const fn dp_irq_vector(&self) -> &DpIrqVector {
        &self.dp_irq_vector
    }
    #[doc = "0x6ec - DP Lane0 and Lane1 Status"]
    #[inline(always)]
    pub const fn dp_link_status0(&self) -> &DpLinkStatus0 {
        &self.dp_link_status0
    }
    #[doc = "0x6f0 - DP Lane2 and Lane3 Status"]
    #[inline(always)]
    pub const fn dp_link_status1(&self) -> &DpLinkStatus1 {
        &self.dp_link_status1
    }
    #[doc = "0x6f4 - DP Align Status"]
    #[inline(always)]
    pub const fn dp_align_status(&self) -> &DpAlignStatus {
        &self.dp_align_status
    }
    #[doc = "0x6f8 - DP Sink Status"]
    #[inline(always)]
    pub const fn dp_sink_status(&self) -> &DpSinkStatus {
        &self.dp_sink_status
    }
    #[doc = "0x700 - DP M_VID Configure Register #0"]
    #[inline(always)]
    pub const fn m_vid_0(&self) -> &MVid0 {
        &self.m_vid_0
    }
    #[doc = "0x704 - DP M_VID Configure Register #1"]
    #[inline(always)]
    pub const fn m_vid_1(&self) -> &MVid1 {
        &self.m_vid_1
    }
    #[doc = "0x708 - DP M_VID Configure Register #2"]
    #[inline(always)]
    pub const fn m_vid_2(&self) -> &MVid2 {
        &self.m_vid_2
    }
    #[doc = "0x70c - DP N_VID Configure Register #0"]
    #[inline(always)]
    pub const fn n_vid_0(&self) -> &NVid0 {
        &self.n_vid_0
    }
    #[doc = "0x710 - DP N_VID Configure Register #1"]
    #[inline(always)]
    pub const fn n_vid_1(&self) -> &NVid1 {
        &self.n_vid_1
    }
    #[doc = "0x714 - DP N_VID Configure Register #2"]
    #[inline(always)]
    pub const fn n_vid_2(&self) -> &NVid2 {
        &self.n_vid_2
    }
    #[doc = "0x718 - DP M_VID value monitoring register"]
    #[inline(always)]
    pub const fn m_vid_mon(&self) -> &MVidMon {
        &self.m_vid_mon
    }
    #[doc = "0x730 - DP FIFO Threshold Register"]
    #[inline(always)]
    pub const fn dp_video_fifo_thrd(&self) -> &DpVideoFifoThrd {
        &self.dp_video_fifo_thrd
    }
    #[doc = "0x760 - DP M Value Calculation Control Register"]
    #[inline(always)]
    pub const fn dp_m_cal_ctl(&self) -> &DpMCalCtl {
        &self.dp_m_cal_ctl
    }
    #[doc = "0x764 - DP M_VID Value Calculation Control Register"]
    #[inline(always)]
    pub const fn m_vid_gen_filter_th(&self) -> &MVidGenFilterTh {
        &self.m_vid_gen_filter_th
    }
    #[doc = "0x780 - AUX Channel Access Status Register"]
    #[inline(always)]
    pub const fn aux_ch_sta(&self) -> &AuxChSta {
        &self.aux_ch_sta
    }
    #[doc = "0x784 - AUX Channel Access Error Code Register"]
    #[inline(always)]
    pub const fn aux_err_num(&self) -> &AuxErrNum {
        &self.aux_err_num
    }
    #[doc = "0x788 - DP AUX CH DEFER Control Register"]
    #[inline(always)]
    pub const fn aux_ch_defer_ctl(&self) -> &AuxChDeferCtl {
        &self.aux_ch_defer_ctl
    }
    #[doc = "0x78c - DP AUX RX Command Register"]
    #[inline(always)]
    pub const fn aux_rx_comm(&self) -> &AuxRxComm {
        &self.aux_rx_comm
    }
    #[doc = "0x790 - DP Buffer Data Count Register"]
    #[inline(always)]
    pub const fn buffer_data_ctl(&self) -> &BufferDataCtl {
        &self.buffer_data_ctl
    }
    #[doc = "0x794 - DP AUX Channel Control Register 1"]
    #[inline(always)]
    pub const fn aux_ch_ctl_1(&self) -> &AuxChCtl1 {
        &self.aux_ch_ctl_1
    }
    #[doc = "0x798 - DP AUX CH Address Register #0"]
    #[inline(always)]
    pub const fn aux_addr_7_0(&self) -> &AuxAddr7_0 {
        &self.aux_addr_7_0
    }
    #[doc = "0x79c - DP AUX CH Address Register #1"]
    #[inline(always)]
    pub const fn aux_addr_15_8(&self) -> &AuxAddr15_8 {
        &self.aux_addr_15_8
    }
    #[doc = "0x7a0 - DP AUX CH Address Register #2"]
    #[inline(always)]
    pub const fn aux_addr_19_16(&self) -> &AuxAddr19_16 {
        &self.aux_addr_19_16
    }
    #[doc = "0x7a4 - DP AUX CH Control Register 2"]
    #[inline(always)]
    pub const fn aux_ch_ctl_2(&self) -> &AuxChCtl2 {
        &self.aux_ch_ctl_2
    }
    #[doc = "0x7c0..0x800 - AUX CH buffer data 0 ~ 15"]
    #[inline(always)]
    pub const fn buf_data_(&self, n: usize) -> &BufData_ {
        &self.buf_data_[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x7c0..0x800 - AUX CH buffer data 0 ~ 15"]
    #[inline(always)]
    pub fn buf_data__iter(&self) -> impl Iterator<Item = &BufData_> {
        self.buf_data_.iter()
    }
    #[doc = "0x804 - ATE test control register"]
    #[inline(always)]
    pub const fn ate_test_ctl(&self) -> &AteTestCtl {
        &self.ate_test_ctl
    }
    #[doc = "0x808 - ATE test status register"]
    #[inline(always)]
    pub const fn ate_test_status(&self) -> &AteTestStatus {
        &self.ate_test_status
    }
    #[doc = "0x80c..0x81c - ATE test error counter register"]
    #[inline(always)]
    pub const fn ate_test_err_cnt(&self, n: usize) -> &AteTestErrCnt {
        &self.ate_test_err_cnt[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80c..0x81c - ATE test error counter register"]
    #[inline(always)]
    pub fn ate_test_err_cnt_iter(&self) -> impl Iterator<Item = &AteTestErrCnt> {
        self.ate_test_err_cnt.iter()
    }
    #[doc = "0x81c - 80b pattern \\[29:0\\]"]
    #[inline(always)]
    pub const fn dp_test_80b_pattern0(&self) -> &DpTest80bPattern0 {
        &self.dp_test_80b_pattern0
    }
    #[doc = "0x820 - 80b pattern \\[59:30\\]"]
    #[inline(always)]
    pub const fn dp_test_80b_pattern1(&self) -> &DpTest80bPattern1 {
        &self.dp_test_80b_pattern1
    }
    #[doc = "0x824 - 80b pattern \\[79:60\\]"]
    #[inline(always)]
    pub const fn dp_test_80b_pattern2(&self) -> &DpTest80bPattern2 {
        &self.dp_test_80b_pattern2
    }
    #[doc = "0x828 - Hbr2 compliance SR count"]
    #[inline(always)]
    pub const fn dp_test_hbr2_pattern(&self) -> &DpTestHbr2Pattern {
        &self.dp_test_hbr2_pattern
    }
    #[doc = "0x890 - CRC control register"]
    #[inline(always)]
    pub const fn crc_con(&self) -> &CrcCon {
        &self.crc_con
    }
    #[doc = "0x914 - PC2 Control Register"]
    #[inline(always)]
    pub const fn analog_ctl_5(&self) -> &AnalogCtl5 {
        &self.analog_ctl_5
    }
    #[doc = "0x918 - AMP_400MV_0DB"]
    #[inline(always)]
    pub const fn analog_ctl_6(&self) -> &AnalogCtl6 {
        &self.analog_ctl_6
    }
    #[doc = "0x91c - AMP_600MV_0DB"]
    #[inline(always)]
    pub const fn analog_ctl_7(&self) -> &AnalogCtl7 {
        &self.analog_ctl_7
    }
    #[doc = "0x920 - AMP_800MV_0DB"]
    #[inline(always)]
    pub const fn analog_ctl_8(&self) -> &AnalogCtl8 {
        &self.analog_ctl_8
    }
    #[doc = "0x924 - AMP_1200MV_0DB"]
    #[inline(always)]
    pub const fn analog_ctl_9(&self) -> &AnalogCtl9 {
        &self.analog_ctl_9
    }
    #[doc = "0x928 - AMP_400MV_3P5DB"]
    #[inline(always)]
    pub const fn analog_ctl_10(&self) -> &AnalogCtl10 {
        &self.analog_ctl_10
    }
    #[doc = "0x92c - AMP_600MV_3P5DB"]
    #[inline(always)]
    pub const fn analog_ctl_11(&self) -> &AnalogCtl11 {
        &self.analog_ctl_11
    }
    #[doc = "0x930 - AMP_800MV_3P5DB"]
    #[inline(always)]
    pub const fn analog_ctl_12(&self) -> &AnalogCtl12 {
        &self.analog_ctl_12
    }
    #[doc = "0x934 - AMP_400MV_6DB"]
    #[inline(always)]
    pub const fn analog_ctl_13(&self) -> &AnalogCtl13 {
        &self.analog_ctl_13
    }
    #[doc = "0x938 - AMP_600MV_6DB"]
    #[inline(always)]
    pub const fn analog_ctl_14(&self) -> &AnalogCtl14 {
        &self.analog_ctl_14
    }
    #[doc = "0x93c - AMP_400MV_9DB"]
    #[inline(always)]
    pub const fn analog_ctl_15(&self) -> &AnalogCtl15 {
        &self.analog_ctl_15
    }
    #[doc = "0x940 - EMP_400MV_0DB"]
    #[inline(always)]
    pub const fn analog_ctl_16(&self) -> &AnalogCtl16 {
        &self.analog_ctl_16
    }
    #[doc = "0x944 - EMP_600MV_0DB"]
    #[inline(always)]
    pub const fn analog_ctl_17(&self) -> &AnalogCtl17 {
        &self.analog_ctl_17
    }
    #[doc = "0x948 - EMP_800MV_0DB"]
    #[inline(always)]
    pub const fn analog_ctl_18(&self) -> &AnalogCtl18 {
        &self.analog_ctl_18
    }
    #[doc = "0x94c - EMP_1200MV_0DB"]
    #[inline(always)]
    pub const fn analog_ctl_19(&self) -> &AnalogCtl19 {
        &self.analog_ctl_19
    }
    #[doc = "0x950 - EMP_400MV_3P5DB"]
    #[inline(always)]
    pub const fn analog_ctl_20(&self) -> &AnalogCtl20 {
        &self.analog_ctl_20
    }
    #[doc = "0x954 - EMP_600MV_3P5DB"]
    #[inline(always)]
    pub const fn analog_ctl_21(&self) -> &AnalogCtl21 {
        &self.analog_ctl_21
    }
    #[doc = "0x958 - EMP_800MV_3P5DB"]
    #[inline(always)]
    pub const fn analog_ctl_22(&self) -> &AnalogCtl22 {
        &self.analog_ctl_22
    }
    #[doc = "0x95c - EMP_400MV_6DB"]
    #[inline(always)]
    pub const fn analog_ctl_23(&self) -> &AnalogCtl23 {
        &self.analog_ctl_23
    }
    #[doc = "0x960 - EMP_600MV_6DB"]
    #[inline(always)]
    pub const fn analog_ctl_24(&self) -> &AnalogCtl24 {
        &self.analog_ctl_24
    }
    #[doc = "0x964 - EMP_400MV_9DB"]
    #[inline(always)]
    pub const fn analog_ctl_25(&self) -> &AnalogCtl25 {
        &self.analog_ctl_25
    }
    #[doc = "0x968 - PC2_400MV_0DB"]
    #[inline(always)]
    pub const fn analog_ctl_26(&self) -> &AnalogCtl26 {
        &self.analog_ctl_26
    }
    #[doc = "0x96c - PC2_600MV_0DB"]
    #[inline(always)]
    pub const fn analog_ctl_27(&self) -> &AnalogCtl27 {
        &self.analog_ctl_27
    }
    #[doc = "0x970 - PC2_800MV_0DB"]
    #[inline(always)]
    pub const fn analog_ctl_28(&self) -> &AnalogCtl28 {
        &self.analog_ctl_28
    }
    #[doc = "0x974 - PC2_1200MV_0DB"]
    #[inline(always)]
    pub const fn analog_ctl_29(&self) -> &AnalogCtl29 {
        &self.analog_ctl_29
    }
    #[doc = "0x978 - PC2_400MV_3P5DB"]
    #[inline(always)]
    pub const fn analog_ctl_30(&self) -> &AnalogCtl30 {
        &self.analog_ctl_30
    }
    #[doc = "0x97c - PC2_600MV_3P5DB"]
    #[inline(always)]
    pub const fn analog_ctl_31(&self) -> &AnalogCtl31 {
        &self.analog_ctl_31
    }
    #[doc = "0x980 - PC2_800MV_3P5DB"]
    #[inline(always)]
    pub const fn analog_ctl_32(&self) -> &AnalogCtl32 {
        &self.analog_ctl_32
    }
    #[doc = "0x984 - PC2_400MV_6DB"]
    #[inline(always)]
    pub const fn analog_ctl_33(&self) -> &AnalogCtl33 {
        &self.analog_ctl_33
    }
    #[doc = "0x988 - PC2_600MV_6DB"]
    #[inline(always)]
    pub const fn analog_ctl_34(&self) -> &AnalogCtl34 {
        &self.analog_ctl_34
    }
    #[doc = "0x98c - PC2_400MV_9DB"]
    #[inline(always)]
    pub const fn analog_ctl_35(&self) -> &AnalogCtl35 {
        &self.analog_ctl_35
    }
    #[doc = "0x990 - CH0_AMP_FORCE_VALUE"]
    #[inline(always)]
    pub const fn analog_ctl_36(&self) -> &AnalogCtl36 {
        &self.analog_ctl_36
    }
    #[doc = "0x994 - CH0_EMP_FORCE_VALUE"]
    #[inline(always)]
    pub const fn analog_ctl_37(&self) -> &AnalogCtl37 {
        &self.analog_ctl_37
    }
    #[doc = "0x998 - CH0_PC2_FORCE_VALUE"]
    #[inline(always)]
    pub const fn analog_ctl_38(&self) -> &AnalogCtl38 {
        &self.analog_ctl_38
    }
    #[doc = "0x99c - CH1_AMP_FORCE_VALUE"]
    #[inline(always)]
    pub const fn analog_ctl_39(&self) -> &AnalogCtl39 {
        &self.analog_ctl_39
    }
    #[doc = "0x9a0 - CH1_EMP_FORCE_VALUE"]
    #[inline(always)]
    pub const fn analog_ctl_40(&self) -> &AnalogCtl40 {
        &self.analog_ctl_40
    }
    #[doc = "0x9a4 - CH1_PC2_FORCE_VALUE"]
    #[inline(always)]
    pub const fn analog_ctl_41(&self) -> &AnalogCtl41 {
        &self.analog_ctl_41
    }
    #[doc = "0x9a8 - CH0_CH1_FORCE_CTRL"]
    #[inline(always)]
    pub const fn analog_ctl_42(&self) -> &AnalogCtl42 {
        &self.analog_ctl_42
    }
    #[doc = "0x9ac - CH2_AMP_FORCE_VALUE"]
    #[inline(always)]
    pub const fn analog_ctl_43(&self) -> &AnalogCtl43 {
        &self.analog_ctl_43
    }
    #[doc = "0x9b0 - CH2_EMP_FORCE_VALUE"]
    #[inline(always)]
    pub const fn analog_ctl_44(&self) -> &AnalogCtl44 {
        &self.analog_ctl_44
    }
    #[doc = "0x9b4 - CH2_PC2_FORCE_VALUE"]
    #[inline(always)]
    pub const fn analog_ctl_45(&self) -> &AnalogCtl45 {
        &self.analog_ctl_45
    }
    #[doc = "0x9b8 - CH3_AMP_FORCE_VALUE"]
    #[inline(always)]
    pub const fn analog_ctl_46(&self) -> &AnalogCtl46 {
        &self.analog_ctl_46
    }
    #[doc = "0x9bc - CH3_EMP_FORCE_VALUE"]
    #[inline(always)]
    pub const fn analog_ctl_47(&self) -> &AnalogCtl47 {
        &self.analog_ctl_47
    }
    #[doc = "0x9c0 - CH3_PC2_FORCE_VALUE"]
    #[inline(always)]
    pub const fn analog_ctl_48(&self) -> &AnalogCtl48 {
        &self.analog_ctl_48
    }
    #[doc = "0x9c4 - CH2_CH3_FORCE_CTRL"]
    #[inline(always)]
    pub const fn analog_ctl_49(&self) -> &AnalogCtl49 {
        &self.analog_ctl_49
    }
    #[doc = "0x9d8 - Dp Link Policy"]
    #[inline(always)]
    pub const fn link_policy(&self) -> &LinkPolicy {
        &self.link_policy
    }
    #[doc = "0x9e4 - Pll_control_2"]
    #[inline(always)]
    pub const fn pll_reg_2(&self) -> &PllReg2 {
        &self.pll_reg_2
    }
    #[doc = "0x9e8 - Pll_control_3"]
    #[inline(always)]
    pub const fn pll_reg_3(&self) -> &PllReg3 {
        &self.pll_reg_3
    }
    #[doc = "0x9ec - Pll_control_4"]
    #[inline(always)]
    pub const fn pll_reg_4(&self) -> &PllReg4 {
        &self.pll_reg_4
    }
    #[doc = "0xa00 - Pll_control_5"]
    #[inline(always)]
    pub const fn pll_reg_5(&self) -> &PllReg5 {
        &self.pll_reg_5
    }
    #[doc = "0xa04 - Pll_control_MAC"]
    #[inline(always)]
    pub const fn pll_reg_mac(&self) -> &PllRegMac {
        &self.pll_reg_mac
    }
    #[doc = "0xa08 - Tx terminal resistor control3"]
    #[inline(always)]
    pub const fn tx_common3(&self) -> &TxCommon3 {
        &self.tx_common3
    }
    #[doc = "0xa10 - freq_in_reg"]
    #[inline(always)]
    pub const fn freq_in_reg(&self) -> &FreqInReg {
        &self.freq_in_reg
    }
    #[doc = "0xa14 - frequency counter ,digital output for debug"]
    #[inline(always)]
    pub const fn p_reg_frq(&self) -> &PRegFrq {
        &self.p_reg_frq
    }
    #[doc = "0xa18 - frequency counter ready indicator"]
    #[inline(always)]
    pub const fn p_reg_frq_count_rdy(&self) -> &PRegFrqCountRdy {
        &self.p_reg_frq_count_rdy
    }
    #[doc = "0xa1c - reset band decoder"]
    #[inline(always)]
    pub const fn p_band_dec_reset(&self) -> &PBandDecReset {
        &self.p_band_dec_reset
    }
}
#[doc = "DP_TX_VERSION (rw) register accessor: DP_TX version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_tx_version::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_tx_version::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_tx_version`]
module"]
#[doc(alias = "DP_TX_VERSION")]
pub type DpTxVersion = crate::Reg<dp_tx_version::DpTxVersionSpec>;
#[doc = "DP_TX version register"]
pub mod dp_tx_version;
#[doc = "FUNC_EN_1 (rw) register accessor: Function Enable Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_en_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_en_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_en_1`]
module"]
#[doc(alias = "FUNC_EN_1")]
pub type FuncEn1 = crate::Reg<func_en_1::FuncEn1Spec>;
#[doc = "Function Enable Register 1"]
pub mod func_en_1;
#[doc = "FUNC_EN_2 (rw) register accessor: Function Enable Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_en_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_en_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_en_2`]
module"]
#[doc(alias = "FUNC_EN_2")]
pub type FuncEn2 = crate::Reg<func_en_2::FuncEn2Spec>;
#[doc = "Function Enable Register 2"]
pub mod func_en_2;
#[doc = "VIDEO_CTL_1 (rw) register accessor: Video Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`video_ctl_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`video_ctl_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@video_ctl_1`]
module"]
#[doc(alias = "VIDEO_CTL_1")]
pub type VideoCtl1 = crate::Reg<video_ctl_1::VideoCtl1Spec>;
#[doc = "Video Control 1"]
pub mod video_ctl_1;
#[doc = "VIDEO_CTL_2 (rw) register accessor: Video Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`video_ctl_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`video_ctl_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@video_ctl_2`]
module"]
#[doc(alias = "VIDEO_CTL_2")]
pub type VideoCtl2 = crate::Reg<video_ctl_2::VideoCtl2Spec>;
#[doc = "Video Control 2"]
pub mod video_ctl_2;
#[doc = "VIDEO_CTL_3 (rw) register accessor: Video Control 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`video_ctl_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`video_ctl_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@video_ctl_3`]
module"]
#[doc(alias = "VIDEO_CTL_3")]
pub type VideoCtl3 = crate::Reg<video_ctl_3::VideoCtl3Spec>;
#[doc = "Video Control 3"]
pub mod video_ctl_3;
#[doc = "VIDEO_CTL_4 (rw) register accessor: Video Control 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`video_ctl_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`video_ctl_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@video_ctl_4`]
module"]
#[doc(alias = "VIDEO_CTL_4")]
pub type VideoCtl4 = crate::Reg<video_ctl_4::VideoCtl4Spec>;
#[doc = "Video Control 4"]
pub mod video_ctl_4;
#[doc = "VIDEO_CTL_8 (rw) register accessor: Video Control 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`video_ctl_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`video_ctl_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@video_ctl_8`]
module"]
#[doc(alias = "VIDEO_CTL_8")]
pub type VideoCtl8 = crate::Reg<video_ctl_8::VideoCtl8Spec>;
#[doc = "Video Control 8"]
pub mod video_ctl_8;
#[doc = "VIDEO_CTL_10 (rw) register accessor: Video Control 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`video_ctl_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`video_ctl_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@video_ctl_10`]
module"]
#[doc(alias = "VIDEO_CTL_10")]
pub type VideoCtl10 = crate::Reg<video_ctl_10::VideoCtl10Spec>;
#[doc = "Video Control 10"]
pub mod video_ctl_10;
#[doc = "TOTAL_LINE_CFG_L (rw) register accessor: Total Line Low Byte Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`total_line_cfg_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`total_line_cfg_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@total_line_cfg_l`]
module"]
#[doc(alias = "TOTAL_LINE_CFG_L")]
pub type TotalLineCfgL = crate::Reg<total_line_cfg_l::TotalLineCfgLSpec>;
#[doc = "Total Line Low Byte Configure Register"]
pub mod total_line_cfg_l;
#[doc = "TOTAL_LINE_CFG_H (rw) register accessor: Total Line High Byte Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`total_line_cfg_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`total_line_cfg_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@total_line_cfg_h`]
module"]
#[doc(alias = "TOTAL_LINE_CFG_H")]
pub type TotalLineCfgH = crate::Reg<total_line_cfg_h::TotalLineCfgHSpec>;
#[doc = "Total Line High Byte Configure Register"]
pub mod total_line_cfg_h;
#[doc = "ACTIVE_LINE_CFG_L (rw) register accessor: Active Line Low Byte Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`active_line_cfg_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`active_line_cfg_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@active_line_cfg_l`]
module"]
#[doc(alias = "ACTIVE_LINE_CFG_L")]
pub type ActiveLineCfgL = crate::Reg<active_line_cfg_l::ActiveLineCfgLSpec>;
#[doc = "Active Line Low Byte Configure Register"]
pub mod active_line_cfg_l;
#[doc = "ACTIVE_LINE_CFG_H (rw) register accessor: Active Line High Byte Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`active_line_cfg_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`active_line_cfg_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@active_line_cfg_h`]
module"]
#[doc(alias = "ACTIVE_LINE_CFG_H")]
pub type ActiveLineCfgH = crate::Reg<active_line_cfg_h::ActiveLineCfgHSpec>;
#[doc = "Active Line High Byte Configure Register"]
pub mod active_line_cfg_h;
#[doc = "V_F_PORCH_CFG (rw) register accessor: Vertical Front Porch Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`v_f_porch_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`v_f_porch_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@v_f_porch_cfg`]
module"]
#[doc(alias = "V_F_PORCH_CFG")]
pub type VFPorchCfg = crate::Reg<v_f_porch_cfg::VFPorchCfgSpec>;
#[doc = "Vertical Front Porch Configure Register"]
pub mod v_f_porch_cfg;
#[doc = "V_SYNC_WIDTH_CFG (rw) register accessor: Vertical Sync Width Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`v_sync_width_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`v_sync_width_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@v_sync_width_cfg`]
module"]
#[doc(alias = "V_SYNC_WIDTH_CFG")]
pub type VSyncWidthCfg = crate::Reg<v_sync_width_cfg::VSyncWidthCfgSpec>;
#[doc = "Vertical Sync Width Configure Register"]
pub mod v_sync_width_cfg;
#[doc = "V_B_PORCH_CFG (rw) register accessor: Vertical Back Porch Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`v_b_porch_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`v_b_porch_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@v_b_porch_cfg`]
module"]
#[doc(alias = "V_B_PORCH_CFG")]
pub type VBPorchCfg = crate::Reg<v_b_porch_cfg::VBPorchCfgSpec>;
#[doc = "Vertical Back Porch Configure Register"]
pub mod v_b_porch_cfg;
#[doc = "TOTAL_PIXEL_CFG_L (rw) register accessor: Total Pixel Low Byte Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`total_pixel_cfg_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`total_pixel_cfg_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@total_pixel_cfg_l`]
module"]
#[doc(alias = "TOTAL_PIXEL_CFG_L")]
pub type TotalPixelCfgL = crate::Reg<total_pixel_cfg_l::TotalPixelCfgLSpec>;
#[doc = "Total Pixel Low Byte Configure Register"]
pub mod total_pixel_cfg_l;
#[doc = "TOTAL_PIXEL_CFG_H (rw) register accessor: Total Pixel High Byte Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`total_pixel_cfg_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`total_pixel_cfg_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@total_pixel_cfg_h`]
module"]
#[doc(alias = "TOTAL_PIXEL_CFG_H")]
pub type TotalPixelCfgH = crate::Reg<total_pixel_cfg_h::TotalPixelCfgHSpec>;
#[doc = "Total Pixel High Byte Configure Register"]
pub mod total_pixel_cfg_h;
#[doc = "ACTIVE_PIXEL_CFG_L (rw) register accessor: Active Pixel Low Byte Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`active_pixel_cfg_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`active_pixel_cfg_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@active_pixel_cfg_l`]
module"]
#[doc(alias = "ACTIVE_PIXEL_CFG_L")]
pub type ActivePixelCfgL = crate::Reg<active_pixel_cfg_l::ActivePixelCfgLSpec>;
#[doc = "Active Pixel Low Byte Configure Register"]
pub mod active_pixel_cfg_l;
#[doc = "ACTIVE_PIXEL_CFG_H (rw) register accessor: Active Pixel High Byte Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`active_pixel_cfg_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`active_pixel_cfg_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@active_pixel_cfg_h`]
module"]
#[doc(alias = "ACTIVE_PIXEL_CFG_H")]
pub type ActivePixelCfgH = crate::Reg<active_pixel_cfg_h::ActivePixelCfgHSpec>;
#[doc = "Active Pixel High Byte Configure Register"]
pub mod active_pixel_cfg_h;
#[doc = "H_F_PORCH_CFG_L (rw) register accessor: Horizon Front Porch Low Byte Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h_f_porch_cfg_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h_f_porch_cfg_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@h_f_porch_cfg_l`]
module"]
#[doc(alias = "H_F_PORCH_CFG_L")]
pub type HFPorchCfgL = crate::Reg<h_f_porch_cfg_l::HFPorchCfgLSpec>;
#[doc = "Horizon Front Porch Low Byte Configure Register"]
pub mod h_f_porch_cfg_l;
#[doc = "H_F_PORCH_CFG_H (rw) register accessor: Horizon Front Porch High Byte Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h_f_porch_cfg_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h_f_porch_cfg_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@h_f_porch_cfg_h`]
module"]
#[doc(alias = "H_F_PORCH_CFG_H")]
pub type HFPorchCfgH = crate::Reg<h_f_porch_cfg_h::HFPorchCfgHSpec>;
#[doc = "Horizon Front Porch High Byte Configure Register"]
pub mod h_f_porch_cfg_h;
#[doc = "H_SYNC_CFG_L (rw) register accessor: Horizon Sync Width Low Byte Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h_sync_cfg_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h_sync_cfg_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@h_sync_cfg_l`]
module"]
#[doc(alias = "H_SYNC_CFG_L")]
pub type HSyncCfgL = crate::Reg<h_sync_cfg_l::HSyncCfgLSpec>;
#[doc = "Horizon Sync Width Low Byte Configure Register"]
pub mod h_sync_cfg_l;
#[doc = "H_SYNC_CFG_H (rw) register accessor: Horizon Sync Width High Byte Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h_sync_cfg_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h_sync_cfg_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@h_sync_cfg_h`]
module"]
#[doc(alias = "H_SYNC_CFG_H")]
pub type HSyncCfgH = crate::Reg<h_sync_cfg_h::HSyncCfgHSpec>;
#[doc = "Horizon Sync Width High Byte Configure Register"]
pub mod h_sync_cfg_h;
#[doc = "H_B_PORCH_CFG_L (rw) register accessor: Horizon Back Porch Low Byte Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h_b_porch_cfg_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h_b_porch_cfg_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@h_b_porch_cfg_l`]
module"]
#[doc(alias = "H_B_PORCH_CFG_L")]
pub type HBPorchCfgL = crate::Reg<h_b_porch_cfg_l::HBPorchCfgLSpec>;
#[doc = "Horizon Back Porch Low Byte Configure Register"]
pub mod h_b_porch_cfg_l;
#[doc = "H_B_PORCH_CFG_H (rw) register accessor: Horizon Back Porch High Byte Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h_b_porch_cfg_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h_b_porch_cfg_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@h_b_porch_cfg_h`]
module"]
#[doc(alias = "H_B_PORCH_CFG_H")]
pub type HBPorchCfgH = crate::Reg<h_b_porch_cfg_h::HBPorchCfgHSpec>;
#[doc = "Horizon Back Porch High Byte Configure Register"]
pub mod h_b_porch_cfg_h;
#[doc = "VIDEO_STATUS (rw) register accessor: Video Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`video_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`video_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@video_status`]
module"]
#[doc(alias = "VIDEO_STATUS")]
pub type VideoStatus = crate::Reg<video_status::VideoStatusSpec>;
#[doc = "Video Status Register"]
pub mod video_status;
#[doc = "TOTAL_LINE_STA_L (rw) register accessor: Total Line Status Low Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`total_line_sta_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`total_line_sta_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@total_line_sta_l`]
module"]
#[doc(alias = "TOTAL_LINE_STA_L")]
pub type TotalLineStaL = crate::Reg<total_line_sta_l::TotalLineStaLSpec>;
#[doc = "Total Line Status Low Byte Register"]
pub mod total_line_sta_l;
#[doc = "TOTAL_LINE_STA_H (rw) register accessor: Total Line Status High Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`total_line_sta_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`total_line_sta_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@total_line_sta_h`]
module"]
#[doc(alias = "TOTAL_LINE_STA_H")]
pub type TotalLineStaH = crate::Reg<total_line_sta_h::TotalLineStaHSpec>;
#[doc = "Total Line Status High Byte Register"]
pub mod total_line_sta_h;
#[doc = "ACTIVE_LINE_STA_L (rw) register accessor: Active Line Status Low Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`active_line_sta_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`active_line_sta_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@active_line_sta_l`]
module"]
#[doc(alias = "ACTIVE_LINE_STA_L")]
pub type ActiveLineStaL = crate::Reg<active_line_sta_l::ActiveLineStaLSpec>;
#[doc = "Active Line Status Low Byte Register"]
pub mod active_line_sta_l;
#[doc = "ACTIVE_LINE_STA_H (rw) register accessor: Active Line Status High Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`active_line_sta_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`active_line_sta_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@active_line_sta_h`]
module"]
#[doc(alias = "ACTIVE_LINE_STA_H")]
pub type ActiveLineStaH = crate::Reg<active_line_sta_h::ActiveLineStaHSpec>;
#[doc = "Active Line Status High Byte Register"]
pub mod active_line_sta_h;
#[doc = "V_F_PORCH_STA (rw) register accessor: Vertical Front Porch Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`v_f_porch_sta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`v_f_porch_sta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@v_f_porch_sta`]
module"]
#[doc(alias = "V_F_PORCH_STA")]
pub type VFPorchSta = crate::Reg<v_f_porch_sta::VFPorchStaSpec>;
#[doc = "Vertical Front Porch Status Register"]
pub mod v_f_porch_sta;
#[doc = "V_SYNC_STA (rw) register accessor: Vertical Sync Width Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`v_sync_sta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`v_sync_sta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@v_sync_sta`]
module"]
#[doc(alias = "V_SYNC_STA")]
pub type VSyncSta = crate::Reg<v_sync_sta::VSyncStaSpec>;
#[doc = "Vertical Sync Width Status Register"]
pub mod v_sync_sta;
#[doc = "V_B_PORCH_STA (rw) register accessor: Vertical Back Porch Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`v_b_porch_sta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`v_b_porch_sta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@v_b_porch_sta`]
module"]
#[doc(alias = "V_B_PORCH_STA")]
pub type VBPorchSta = crate::Reg<v_b_porch_sta::VBPorchStaSpec>;
#[doc = "Vertical Back Porch Status Register"]
pub mod v_b_porch_sta;
#[doc = "TOTAL_PIXEL_STA_L (rw) register accessor: Total Pixel Status Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`total_pixel_sta_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`total_pixel_sta_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@total_pixel_sta_l`]
module"]
#[doc(alias = "TOTAL_PIXEL_STA_L")]
pub type TotalPixelStaL = crate::Reg<total_pixel_sta_l::TotalPixelStaLSpec>;
#[doc = "Total Pixel Status Low"]
pub mod total_pixel_sta_l;
#[doc = "TOTAL_PIXEL_STA_H (rw) register accessor: Total Pixel Status High Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`total_pixel_sta_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`total_pixel_sta_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@total_pixel_sta_h`]
module"]
#[doc(alias = "TOTAL_PIXEL_STA_H")]
pub type TotalPixelStaH = crate::Reg<total_pixel_sta_h::TotalPixelStaHSpec>;
#[doc = "Total Pixel Status High Byte Register"]
pub mod total_pixel_sta_h;
#[doc = "ACTIVE_PIXEL_STA_L (rw) register accessor: Active Pixel Status Low Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`active_pixel_sta_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`active_pixel_sta_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@active_pixel_sta_l`]
module"]
#[doc(alias = "ACTIVE_PIXEL_STA_L")]
pub type ActivePixelStaL = crate::Reg<active_pixel_sta_l::ActivePixelStaLSpec>;
#[doc = "Active Pixel Status Low Byte Register"]
pub mod active_pixel_sta_l;
#[doc = "ACTIVE_PIXEL_STA_H (rw) register accessor: Active Pixel Status High Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`active_pixel_sta_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`active_pixel_sta_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@active_pixel_sta_h`]
module"]
#[doc(alias = "ACTIVE_PIXEL_STA_H")]
pub type ActivePixelStaH = crate::Reg<active_pixel_sta_h::ActivePixelStaHSpec>;
#[doc = "Active Pixel Status High Byte Register"]
pub mod active_pixel_sta_h;
#[doc = "H_F_PORCH_STA_L (rw) register accessor: Horizon Front Porch Status Low Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h_f_porch_sta_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h_f_porch_sta_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@h_f_porch_sta_l`]
module"]
#[doc(alias = "H_F_PORCH_STA_L")]
pub type HFPorchStaL = crate::Reg<h_f_porch_sta_l::HFPorchStaLSpec>;
#[doc = "Horizon Front Porch Status Low Byte Register"]
pub mod h_f_porch_sta_l;
#[doc = "H_F_PORCH_STA_H (rw) register accessor: Horizon Front Porch Status High Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h_f_porch_sta_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h_f_porch_sta_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@h_f_porch_sta_h`]
module"]
#[doc(alias = "H_F_PORCH_STA_H")]
pub type HFPorchStaH = crate::Reg<h_f_porch_sta_h::HFPorchStaHSpec>;
#[doc = "Horizon Front Porch Status High Byte Register"]
pub mod h_f_porch_sta_h;
#[doc = "H_SYNC_STA_L (rw) register accessor: Horizon Sync Width Status Low Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h_sync_sta_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h_sync_sta_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@h_sync_sta_l`]
module"]
#[doc(alias = "H_SYNC_STA_L")]
pub type HSyncStaL = crate::Reg<h_sync_sta_l::HSyncStaLSpec>;
#[doc = "Horizon Sync Width Status Low Byte Register"]
pub mod h_sync_sta_l;
#[doc = "H_SYNC_STA_H (rw) register accessor: Horizon Sync Width Status High Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h_sync_sta_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h_sync_sta_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@h_sync_sta_h`]
module"]
#[doc(alias = "H_SYNC_STA_H")]
pub type HSyncStaH = crate::Reg<h_sync_sta_h::HSyncStaHSpec>;
#[doc = "Horizon Sync Width Status High Byte Register"]
pub mod h_sync_sta_h;
#[doc = "H_B_PORCH_STA_L (rw) register accessor: Horizon Back Porch Status Low Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h_b_porch_sta_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h_b_porch_sta_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@h_b_porch_sta_l`]
module"]
#[doc(alias = "H_B_PORCH_STA_L")]
pub type HBPorchStaL = crate::Reg<h_b_porch_sta_l::HBPorchStaLSpec>;
#[doc = "Horizon Back Porch Status Low Byte Register"]
pub mod h_b_porch_sta_l;
#[doc = "H_B_PORCH_STA_H (rw) register accessor: Horizon Back Porch Status High Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h_b_porch_sta_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h_b_porch_sta_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@h_b_porch_sta_h`]
module"]
#[doc(alias = "H_B_PORCH_STA_H")]
pub type HBPorchStaH = crate::Reg<h_b_porch_sta_h::HBPorchStaHSpec>;
#[doc = "Horizon Back Porch Status High Byte Register"]
pub mod h_b_porch_sta_h;
#[doc = "AVI_DB (rw) register accessor: AVI InfoFrame Packet Data Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`avi_db::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`avi_db::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@avi_db`]
module"]
#[doc(alias = "AVI_DB")]
pub type AviDb = crate::Reg<avi_db::AviDbSpec>;
#[doc = "AVI InfoFrame Packet Data Byte"]
pub mod avi_db;
#[doc = "IF_TYPE (rw) register accessor: InfoFrame Packet Type Code.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_type::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if_type::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_type`]
module"]
#[doc(alias = "IF_TYPE")]
pub type IfType = crate::Reg<if_type::IfTypeSpec>;
#[doc = "InfoFrame Packet Type Code."]
pub mod if_type;
#[doc = "IF_PKT_DB (rw) register accessor: InfoFrame Packet Data Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_pkt_db::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if_pkt_db::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_pkt_db`]
module"]
#[doc(alias = "IF_PKT_DB")]
pub type IfPktDb = crate::Reg<if_pkt_db::IfPktDbSpec>;
#[doc = "InfoFrame Packet Data Byte"]
pub mod if_pkt_db;
#[doc = "MPEG_DB (rw) register accessor: MPEG Source InfoFrame Packet Data Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpeg_db::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpeg_db::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpeg_db`]
module"]
#[doc(alias = "MPEG_DB")]
pub type MpegDb = crate::Reg<mpeg_db::MpegDbSpec>;
#[doc = "MPEG Source InfoFrame Packet Data Byte"]
pub mod mpeg_db;
#[doc = "PSR_FRAME_UPDATA_CTRL (rw) register accessor: Frame update control for PSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr_frame_updata_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr_frame_updata_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psr_frame_updata_ctrl`]
module"]
#[doc(alias = "PSR_FRAME_UPDATA_CTRL")]
pub type PsrFrameUpdataCtrl = crate::Reg<psr_frame_updata_ctrl::PsrFrameUpdataCtrlSpec>;
#[doc = "Frame update control for PSR"]
pub mod psr_frame_updata_ctrl;
#[doc = "VSC_SHADOW_DB (rw) register accessor: VSC shadow data bytes 0 ~ 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vsc_shadow_db::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vsc_shadow_db::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vsc_shadow_db`]
module"]
#[doc(alias = "VSC_SHADOW_DB")]
pub type VscShadowDb = crate::Reg<vsc_shadow_db::VscShadowDbSpec>;
#[doc = "VSC shadow data bytes 0 ~ 7"]
pub mod vsc_shadow_db;
#[doc = "VSC_SHADOW_PB (rw) register accessor: VSC shadow parity byte 0 ~ 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vsc_shadow_pb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vsc_shadow_pb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vsc_shadow_pb`]
module"]
#[doc(alias = "VSC_SHADOW_PB")]
pub type VscShadowPb = crate::Reg<vsc_shadow_pb::VscShadowPbSpec>;
#[doc = "VSC shadow parity byte 0 ~ 1"]
pub mod vsc_shadow_pb;
#[doc = "LANE_MAP (rw) register accessor: Lane Map Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lane_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lane_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lane_map`]
module"]
#[doc(alias = "LANE_MAP")]
pub type LaneMap = crate::Reg<lane_map::LaneMapSpec>;
#[doc = "Lane Map Register"]
pub mod lane_map;
#[doc = "ANALOG_CTL_2 (rw) register accessor: Analog Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_2`]
module"]
#[doc(alias = "ANALOG_CTL_2")]
pub type AnalogCtl2 = crate::Reg<analog_ctl_2::AnalogCtl2Spec>;
#[doc = "Analog Control Register 2"]
pub mod analog_ctl_2;
#[doc = "INT_STATE_0 (rw) register accessor: Debug Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_state_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_state_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_state_0`]
module"]
#[doc(alias = "INT_STATE_0")]
pub type IntState0 = crate::Reg<int_state_0::IntState0Spec>;
#[doc = "Debug Register"]
pub mod int_state_0;
#[doc = "INT_STATE_1 (rw) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_state_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_state_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_state_1`]
module"]
#[doc(alias = "INT_STATE_1")]
pub type IntState1 = crate::Reg<int_state_1::IntState1Spec>;
#[doc = "Interrupt Status Register"]
pub mod int_state_1;
#[doc = "COMMON_INT_STA_1 (rw) register accessor: Common Interrupt Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`common_int_sta_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`common_int_sta_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@common_int_sta_1`]
module"]
#[doc(alias = "COMMON_INT_STA_1")]
pub type CommonIntSta1 = crate::Reg<common_int_sta_1::CommonIntSta1Spec>;
#[doc = "Common Interrupt Status Register 1"]
pub mod common_int_sta_1;
#[doc = "COMMON_INT_STA_3 (rw) register accessor: Common Interrupt Status Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`common_int_sta_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`common_int_sta_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@common_int_sta_3`]
module"]
#[doc(alias = "COMMON_INT_STA_3")]
pub type CommonIntSta3 = crate::Reg<common_int_sta_3::CommonIntSta3Spec>;
#[doc = "Common Interrupt Status Register 3"]
pub mod common_int_sta_3;
#[doc = "COMMON_INT_STA_4 (rw) register accessor: Common Interrupt Status Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`common_int_sta_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`common_int_sta_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@common_int_sta_4`]
module"]
#[doc(alias = "COMMON_INT_STA_4")]
pub type CommonIntSta4 = crate::Reg<common_int_sta_4::CommonIntSta4Spec>;
#[doc = "Common Interrupt Status Register 4"]
pub mod common_int_sta_4;
#[doc = "DP_INT_STA (rw) register accessor: DisplayPort Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_int_sta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_int_sta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_int_sta`]
module"]
#[doc(alias = "DP_INT_STA")]
pub type DpIntSta = crate::Reg<dp_int_sta::DpIntStaSpec>;
#[doc = "DisplayPort Interrupt Status Register"]
pub mod dp_int_sta;
#[doc = "COMMON_INT_MASK_1 (rw) register accessor: Common Interrupt Mask Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`common_int_mask_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`common_int_mask_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@common_int_mask_1`]
module"]
#[doc(alias = "COMMON_INT_MASK_1")]
pub type CommonIntMask1 = crate::Reg<common_int_mask_1::CommonIntMask1Spec>;
#[doc = "Common Interrupt Mask Register1"]
pub mod common_int_mask_1;
#[doc = "COMMON_INT_MASK_3 (rw) register accessor: Common Interrupt Mask Register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`common_int_mask_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`common_int_mask_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@common_int_mask_3`]
module"]
#[doc(alias = "COMMON_INT_MASK_3")]
pub type CommonIntMask3 = crate::Reg<common_int_mask_3::CommonIntMask3Spec>;
#[doc = "Common Interrupt Mask Register3"]
pub mod common_int_mask_3;
#[doc = "COMMON_INT_MASK_4 (rw) register accessor: Common Interrupt Mask Register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`common_int_mask_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`common_int_mask_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@common_int_mask_4`]
module"]
#[doc(alias = "COMMON_INT_MASK_4")]
pub type CommonIntMask4 = crate::Reg<common_int_mask_4::CommonIntMask4Spec>;
#[doc = "Common Interrupt Mask Register4"]
pub mod common_int_mask_4;
#[doc = "DP_INT_STA_MASK (rw) register accessor: DisplayPort Interrupt enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_int_sta_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_int_sta_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_int_sta_mask`]
module"]
#[doc(alias = "DP_INT_STA_MASK")]
pub type DpIntStaMask = crate::Reg<dp_int_sta_mask::DpIntStaMaskSpec>;
#[doc = "DisplayPort Interrupt enable Register"]
pub mod dp_int_sta_mask;
#[doc = "INT_CTL (rw) register accessor: Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ctl`]
module"]
#[doc(alias = "INT_CTL")]
pub type IntCtl = crate::Reg<int_ctl::IntCtlSpec>;
#[doc = "Interrupt Control Register"]
pub mod int_ctl;
#[doc = "SYS_CTL_1 (rw) register accessor: System Control Register #1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_ctl_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_ctl_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_ctl_1`]
module"]
#[doc(alias = "SYS_CTL_1")]
pub type SysCtl1 = crate::Reg<sys_ctl_1::SysCtl1Spec>;
#[doc = "System Control Register #1"]
pub mod sys_ctl_1;
#[doc = "SYS_CTL_2 (rw) register accessor: System Control Register #2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_ctl_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_ctl_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_ctl_2`]
module"]
#[doc(alias = "SYS_CTL_2")]
pub type SysCtl2 = crate::Reg<sys_ctl_2::SysCtl2Spec>;
#[doc = "System Control Register #2"]
pub mod sys_ctl_2;
#[doc = "SYS_CTL_3 (rw) register accessor: System Control Register #3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_ctl_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_ctl_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_ctl_3`]
module"]
#[doc(alias = "SYS_CTL_3")]
pub type SysCtl3 = crate::Reg<sys_ctl_3::SysCtl3Spec>;
#[doc = "System Control Register #3"]
pub mod sys_ctl_3;
#[doc = "SYS_CTL_4 (rw) register accessor: System Control Register #4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_ctl_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_ctl_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_ctl_4`]
module"]
#[doc(alias = "SYS_CTL_4")]
pub type SysCtl4 = crate::Reg<sys_ctl_4::SysCtl4Spec>;
#[doc = "System Control Register #4"]
pub mod sys_ctl_4;
#[doc = "DP_VID_CTL (rw) register accessor: DP Video Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_vid_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_vid_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_vid_ctl`]
module"]
#[doc(alias = "DP_VID_CTL")]
pub type DpVidCtl = crate::Reg<dp_vid_ctl::DpVidCtlSpec>;
#[doc = "DP Video Control Register"]
pub mod dp_vid_ctl;
#[doc = "PKT_SEND_CTL (rw) register accessor: Packet Send Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pkt_send_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pkt_send_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkt_send_ctl`]
module"]
#[doc(alias = "PKT_SEND_CTL")]
pub type PktSendCtl = crate::Reg<pkt_send_ctl::PktSendCtlSpec>;
#[doc = "Packet Send Control Register"]
pub mod pkt_send_ctl;
#[doc = "LINK_BW_SET (rw) register accessor: Main Link Bandwidth Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`link_bw_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`link_bw_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link_bw_set`]
module"]
#[doc(alias = "LINK_BW_SET")]
pub type LinkBwSet = crate::Reg<link_bw_set::LinkBwSetSpec>;
#[doc = "Main Link Bandwidth Setting Register"]
pub mod link_bw_set;
#[doc = "LANE_COUNT_SET (rw) register accessor: DP Main Link Lane Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lane_count_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lane_count_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lane_count_set`]
module"]
#[doc(alias = "LANE_COUNT_SET")]
pub type LaneCountSet = crate::Reg<lane_count_set::LaneCountSetSpec>;
#[doc = "DP Main Link Lane Number Register"]
pub mod lane_count_set;
#[doc = "DP_TRAINING_PTN_SET (rw) register accessor: DP Training Pattern Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_training_ptn_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_training_ptn_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_training_ptn_set`]
module"]
#[doc(alias = "DP_TRAINING_PTN_SET")]
pub type DpTrainingPtnSet = crate::Reg<dp_training_ptn_set::DpTrainingPtnSetSpec>;
#[doc = "DP Training Pattern Set Register"]
pub mod dp_training_ptn_set;
#[doc = "DP_LN0_LINK_TRAINING_CTL (rw) register accessor: DP Lane 0 Link Training Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_ln0_link_training_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_ln0_link_training_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_ln0_link_training_ctl`]
module"]
#[doc(alias = "DP_LN0_LINK_TRAINING_CTL")]
pub type DpLn0LinkTrainingCtl = crate::Reg<dp_ln0_link_training_ctl::DpLn0LinkTrainingCtlSpec>;
#[doc = "DP Lane 0 Link Training Control Register"]
pub mod dp_ln0_link_training_ctl;
#[doc = "DP_LN1_LINK_TRAINING_CTL (rw) register accessor: DP Lane 1 Link Training Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_ln1_link_training_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_ln1_link_training_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_ln1_link_training_ctl`]
module"]
#[doc(alias = "DP_LN1_LINK_TRAINING_CTL")]
pub type DpLn1LinkTrainingCtl = crate::Reg<dp_ln1_link_training_ctl::DpLn1LinkTrainingCtlSpec>;
#[doc = "DP Lane 1 Link Training Control Register"]
pub mod dp_ln1_link_training_ctl;
#[doc = "DP_LN2_LINK_TRAINING_CTL (rw) register accessor: DP Lane 2 Link Training Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_ln2_link_training_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_ln2_link_training_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_ln2_link_training_ctl`]
module"]
#[doc(alias = "DP_LN2_LINK_TRAINING_CTL")]
pub type DpLn2LinkTrainingCtl = crate::Reg<dp_ln2_link_training_ctl::DpLn2LinkTrainingCtlSpec>;
#[doc = "DP Lane 2 Link Training Control Register"]
pub mod dp_ln2_link_training_ctl;
#[doc = "DP_LN3_LINK_TRAINING_CTL (rw) register accessor: DP Lane 3 Link Training Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_ln3_link_training_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_ln3_link_training_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_ln3_link_training_ctl`]
module"]
#[doc(alias = "DP_LN3_LINK_TRAINING_CTL")]
pub type DpLn3LinkTrainingCtl = crate::Reg<dp_ln3_link_training_ctl::DpLn3LinkTrainingCtlSpec>;
#[doc = "DP Lane 3 Link Training Control Register"]
pub mod dp_ln3_link_training_ctl;
#[doc = "DP_HW_LINK_TRAINING_CTL (rw) register accessor: DP HW LINK TRAINING_CONTROL Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_hw_link_training_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_hw_link_training_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_hw_link_training_ctl`]
module"]
#[doc(alias = "DP_HW_LINK_TRAINING_CTL")]
pub type DpHwLinkTrainingCtl = crate::Reg<dp_hw_link_training_ctl::DpHwLinkTrainingCtlSpec>;
#[doc = "DP HW LINK TRAINING_CONTROL Register"]
pub mod dp_hw_link_training_ctl;
#[doc = "DP_DEBUG_CTL (rw) register accessor: DP Debug Control Register #1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_debug_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_debug_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_debug_ctl`]
module"]
#[doc(alias = "DP_DEBUG_CTL")]
pub type DpDebugCtl = crate::Reg<dp_debug_ctl::DpDebugCtlSpec>;
#[doc = "DP Debug Control Register #1"]
pub mod dp_debug_ctl;
#[doc = "HPD_DEGLITCH_L (rw) register accessor: DP HPD De-glitch Low Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpd_deglitch_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hpd_deglitch_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpd_deglitch_l`]
module"]
#[doc(alias = "HPD_DEGLITCH_L")]
pub type HpdDeglitchL = crate::Reg<hpd_deglitch_l::HpdDeglitchLSpec>;
#[doc = "DP HPD De-glitch Low Byte Register"]
pub mod hpd_deglitch_l;
#[doc = "HPD_DEGLITCH_H (rw) register accessor: DP HPD De-glitch High Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpd_deglitch_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hpd_deglitch_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpd_deglitch_h`]
module"]
#[doc(alias = "HPD_DEGLITCH_H")]
pub type HpdDeglitchH = crate::Reg<hpd_deglitch_h::HpdDeglitchHSpec>;
#[doc = "DP HPD De-glitch High Byte Register"]
pub mod hpd_deglitch_h;
#[doc = "POLLING_PERIOD (rw) register accessor: DP polling period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`polling_period::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`polling_period::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@polling_period`]
module"]
#[doc(alias = "POLLING_PERIOD")]
pub type PollingPeriod = crate::Reg<polling_period::PollingPeriodSpec>;
#[doc = "DP polling period"]
pub mod polling_period;
#[doc = "DP_LINK_DEBUG_CTL (rw) register accessor: DP Link Debug Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_link_debug_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_link_debug_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_link_debug_ctl`]
module"]
#[doc(alias = "DP_LINK_DEBUG_CTL")]
pub type DpLinkDebugCtl = crate::Reg<dp_link_debug_ctl::DpLinkDebugCtlSpec>;
#[doc = "DP Link Debug Control Register"]
pub mod dp_link_debug_ctl;
#[doc = "DP_SINK_COUNT (rw) register accessor: DP Sink Count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_sink_count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_sink_count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_sink_count`]
module"]
#[doc(alias = "DP_SINK_COUNT")]
pub type DpSinkCount = crate::Reg<dp_sink_count::DpSinkCountSpec>;
#[doc = "DP Sink Count"]
pub mod dp_sink_count;
#[doc = "DP_IRQ_VECTOR (rw) register accessor: DP Irq Vector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_irq_vector::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_irq_vector::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_irq_vector`]
module"]
#[doc(alias = "DP_IRQ_VECTOR")]
pub type DpIrqVector = crate::Reg<dp_irq_vector::DpIrqVectorSpec>;
#[doc = "DP Irq Vector"]
pub mod dp_irq_vector;
#[doc = "DP_LINK_STATUS0 (rw) register accessor: DP Lane0 and Lane1 Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_link_status0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_link_status0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_link_status0`]
module"]
#[doc(alias = "DP_LINK_STATUS0")]
pub type DpLinkStatus0 = crate::Reg<dp_link_status0::DpLinkStatus0Spec>;
#[doc = "DP Lane0 and Lane1 Status"]
pub mod dp_link_status0;
#[doc = "DP_LINK_STATUS1 (rw) register accessor: DP Lane2 and Lane3 Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_link_status1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_link_status1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_link_status1`]
module"]
#[doc(alias = "DP_LINK_STATUS1")]
pub type DpLinkStatus1 = crate::Reg<dp_link_status1::DpLinkStatus1Spec>;
#[doc = "DP Lane2 and Lane3 Status"]
pub mod dp_link_status1;
#[doc = "DP_ALIGN_STATUS (rw) register accessor: DP Align Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_align_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_align_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_align_status`]
module"]
#[doc(alias = "DP_ALIGN_STATUS")]
pub type DpAlignStatus = crate::Reg<dp_align_status::DpAlignStatusSpec>;
#[doc = "DP Align Status"]
pub mod dp_align_status;
#[doc = "DP_SINK_STATUS (rw) register accessor: DP Sink Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_sink_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_sink_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_sink_status`]
module"]
#[doc(alias = "DP_SINK_STATUS")]
pub type DpSinkStatus = crate::Reg<dp_sink_status::DpSinkStatusSpec>;
#[doc = "DP Sink Status"]
pub mod dp_sink_status;
#[doc = "M_VID_0 (rw) register accessor: DP M_VID Configure Register #0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m_vid_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m_vid_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m_vid_0`]
module"]
#[doc(alias = "M_VID_0")]
pub type MVid0 = crate::Reg<m_vid_0::MVid0Spec>;
#[doc = "DP M_VID Configure Register #0"]
pub mod m_vid_0;
#[doc = "M_VID_1 (rw) register accessor: DP M_VID Configure Register #1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m_vid_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m_vid_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m_vid_1`]
module"]
#[doc(alias = "M_VID_1")]
pub type MVid1 = crate::Reg<m_vid_1::MVid1Spec>;
#[doc = "DP M_VID Configure Register #1"]
pub mod m_vid_1;
#[doc = "M_VID_2 (rw) register accessor: DP M_VID Configure Register #2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m_vid_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m_vid_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m_vid_2`]
module"]
#[doc(alias = "M_VID_2")]
pub type MVid2 = crate::Reg<m_vid_2::MVid2Spec>;
#[doc = "DP M_VID Configure Register #2"]
pub mod m_vid_2;
#[doc = "N_VID_0 (rw) register accessor: DP N_VID Configure Register #0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`n_vid_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`n_vid_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@n_vid_0`]
module"]
#[doc(alias = "N_VID_0")]
pub type NVid0 = crate::Reg<n_vid_0::NVid0Spec>;
#[doc = "DP N_VID Configure Register #0"]
pub mod n_vid_0;
#[doc = "N_VID_1 (rw) register accessor: DP N_VID Configure Register #1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`n_vid_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`n_vid_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@n_vid_1`]
module"]
#[doc(alias = "N_VID_1")]
pub type NVid1 = crate::Reg<n_vid_1::NVid1Spec>;
#[doc = "DP N_VID Configure Register #1"]
pub mod n_vid_1;
#[doc = "N_VID_2 (rw) register accessor: DP N_VID Configure Register #2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`n_vid_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`n_vid_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@n_vid_2`]
module"]
#[doc(alias = "N_VID_2")]
pub type NVid2 = crate::Reg<n_vid_2::NVid2Spec>;
#[doc = "DP N_VID Configure Register #2"]
pub mod n_vid_2;
#[doc = "M_VID_MON (rw) register accessor: DP M_VID value monitoring register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m_vid_mon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m_vid_mon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m_vid_mon`]
module"]
#[doc(alias = "M_VID_MON")]
pub type MVidMon = crate::Reg<m_vid_mon::MVidMonSpec>;
#[doc = "DP M_VID value monitoring register"]
pub mod m_vid_mon;
#[doc = "DP_VIDEO_FIFO_THRD (rw) register accessor: DP FIFO Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_video_fifo_thrd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_video_fifo_thrd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_video_fifo_thrd`]
module"]
#[doc(alias = "DP_VIDEO_FIFO_THRD")]
pub type DpVideoFifoThrd = crate::Reg<dp_video_fifo_thrd::DpVideoFifoThrdSpec>;
#[doc = "DP FIFO Threshold Register"]
pub mod dp_video_fifo_thrd;
#[doc = "DP_M_CAL_CTL (rw) register accessor: DP M Value Calculation Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_m_cal_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_m_cal_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_m_cal_ctl`]
module"]
#[doc(alias = "DP_M_CAL_CTL")]
pub type DpMCalCtl = crate::Reg<dp_m_cal_ctl::DpMCalCtlSpec>;
#[doc = "DP M Value Calculation Control Register"]
pub mod dp_m_cal_ctl;
#[doc = "M_VID_GEN_FILTER_TH (rw) register accessor: DP M_VID Value Calculation Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m_vid_gen_filter_th::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m_vid_gen_filter_th::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m_vid_gen_filter_th`]
module"]
#[doc(alias = "M_VID_GEN_FILTER_TH")]
pub type MVidGenFilterTh = crate::Reg<m_vid_gen_filter_th::MVidGenFilterThSpec>;
#[doc = "DP M_VID Value Calculation Control Register"]
pub mod m_vid_gen_filter_th;
#[doc = "AUX_CH_STA (rw) register accessor: AUX Channel Access Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aux_ch_sta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aux_ch_sta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aux_ch_sta`]
module"]
#[doc(alias = "AUX_CH_STA")]
pub type AuxChSta = crate::Reg<aux_ch_sta::AuxChStaSpec>;
#[doc = "AUX Channel Access Status Register"]
pub mod aux_ch_sta;
#[doc = "AUX_ERR_NUM (rw) register accessor: AUX Channel Access Error Code Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aux_err_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aux_err_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aux_err_num`]
module"]
#[doc(alias = "AUX_ERR_NUM")]
pub type AuxErrNum = crate::Reg<aux_err_num::AuxErrNumSpec>;
#[doc = "AUX Channel Access Error Code Register"]
pub mod aux_err_num;
#[doc = "AUX_CH_DEFER_CTL (rw) register accessor: DP AUX CH DEFER Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aux_ch_defer_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aux_ch_defer_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aux_ch_defer_ctl`]
module"]
#[doc(alias = "AUX_CH_DEFER_CTL")]
pub type AuxChDeferCtl = crate::Reg<aux_ch_defer_ctl::AuxChDeferCtlSpec>;
#[doc = "DP AUX CH DEFER Control Register"]
pub mod aux_ch_defer_ctl;
#[doc = "AUX_RX_COMM (rw) register accessor: DP AUX RX Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aux_rx_comm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aux_rx_comm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aux_rx_comm`]
module"]
#[doc(alias = "AUX_RX_COMM")]
pub type AuxRxComm = crate::Reg<aux_rx_comm::AuxRxCommSpec>;
#[doc = "DP AUX RX Command Register"]
pub mod aux_rx_comm;
#[doc = "BUFFER_DATA_CTL (rw) register accessor: DP Buffer Data Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buffer_data_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buffer_data_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buffer_data_ctl`]
module"]
#[doc(alias = "BUFFER_DATA_CTL")]
pub type BufferDataCtl = crate::Reg<buffer_data_ctl::BufferDataCtlSpec>;
#[doc = "DP Buffer Data Count Register"]
pub mod buffer_data_ctl;
#[doc = "AUX_CH_CTL_1 (rw) register accessor: DP AUX Channel Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aux_ch_ctl_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aux_ch_ctl_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aux_ch_ctl_1`]
module"]
#[doc(alias = "AUX_CH_CTL_1")]
pub type AuxChCtl1 = crate::Reg<aux_ch_ctl_1::AuxChCtl1Spec>;
#[doc = "DP AUX Channel Control Register 1"]
pub mod aux_ch_ctl_1;
#[doc = "AUX_ADDR_7_0 (rw) register accessor: DP AUX CH Address Register #0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aux_addr_7_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aux_addr_7_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aux_addr_7_0`]
module"]
#[doc(alias = "AUX_ADDR_7_0")]
pub type AuxAddr7_0 = crate::Reg<aux_addr_7_0::AuxAddr7_0Spec>;
#[doc = "DP AUX CH Address Register #0"]
pub mod aux_addr_7_0;
#[doc = "AUX_ADDR_15_8 (rw) register accessor: DP AUX CH Address Register #1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aux_addr_15_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aux_addr_15_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aux_addr_15_8`]
module"]
#[doc(alias = "AUX_ADDR_15_8")]
pub type AuxAddr15_8 = crate::Reg<aux_addr_15_8::AuxAddr15_8Spec>;
#[doc = "DP AUX CH Address Register #1"]
pub mod aux_addr_15_8;
#[doc = "AUX_ADDR_19_16 (rw) register accessor: DP AUX CH Address Register #2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aux_addr_19_16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aux_addr_19_16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aux_addr_19_16`]
module"]
#[doc(alias = "AUX_ADDR_19_16")]
pub type AuxAddr19_16 = crate::Reg<aux_addr_19_16::AuxAddr19_16Spec>;
#[doc = "DP AUX CH Address Register #2"]
pub mod aux_addr_19_16;
#[doc = "AUX_CH_CTL_2 (rw) register accessor: DP AUX CH Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aux_ch_ctl_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aux_ch_ctl_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aux_ch_ctl_2`]
module"]
#[doc(alias = "AUX_CH_CTL_2")]
pub type AuxChCtl2 = crate::Reg<aux_ch_ctl_2::AuxChCtl2Spec>;
#[doc = "DP AUX CH Control Register 2"]
pub mod aux_ch_ctl_2;
#[doc = "BUF_DATA_ (rw) register accessor: AUX CH buffer data 0 ~ 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_data_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_data_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf_data_`]
module"]
#[doc(alias = "BUF_DATA_")]
pub type BufData_ = crate::Reg<buf_data_::BufData_Spec>;
#[doc = "AUX CH buffer data 0 ~ 15"]
pub mod buf_data_;
#[doc = "ATE_TEST_CTL (rw) register accessor: ATE test control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ate_test_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ate_test_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ate_test_ctl`]
module"]
#[doc(alias = "ATE_TEST_CTL")]
pub type AteTestCtl = crate::Reg<ate_test_ctl::AteTestCtlSpec>;
#[doc = "ATE test control register"]
pub mod ate_test_ctl;
#[doc = "ATE_TEST_STATUS (rw) register accessor: ATE test status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ate_test_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ate_test_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ate_test_status`]
module"]
#[doc(alias = "ATE_TEST_STATUS")]
pub type AteTestStatus = crate::Reg<ate_test_status::AteTestStatusSpec>;
#[doc = "ATE test status register"]
pub mod ate_test_status;
#[doc = "ATE_TEST_ERR_CNT (rw) register accessor: ATE test error counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ate_test_err_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ate_test_err_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ate_test_err_cnt`]
module"]
#[doc(alias = "ATE_TEST_ERR_CNT")]
pub type AteTestErrCnt = crate::Reg<ate_test_err_cnt::AteTestErrCntSpec>;
#[doc = "ATE test error counter register"]
pub mod ate_test_err_cnt;
#[doc = "DP_TEST_80B_PATTERN0 (rw) register accessor: 80b pattern \\[29:0\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_test_80b_pattern0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_test_80b_pattern0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_test_80b_pattern0`]
module"]
#[doc(alias = "DP_TEST_80B_PATTERN0")]
pub type DpTest80bPattern0 = crate::Reg<dp_test_80b_pattern0::DpTest80bPattern0Spec>;
#[doc = "80b pattern \\[29:0\\]"]
pub mod dp_test_80b_pattern0;
#[doc = "DP_TEST_80B_PATTERN1 (rw) register accessor: 80b pattern \\[59:30\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_test_80b_pattern1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_test_80b_pattern1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_test_80b_pattern1`]
module"]
#[doc(alias = "DP_TEST_80B_PATTERN1")]
pub type DpTest80bPattern1 = crate::Reg<dp_test_80b_pattern1::DpTest80bPattern1Spec>;
#[doc = "80b pattern \\[59:30\\]"]
pub mod dp_test_80b_pattern1;
#[doc = "DP_TEST_80B_PATTERN2 (rw) register accessor: 80b pattern \\[79:60\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_test_80b_pattern2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_test_80b_pattern2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_test_80b_pattern2`]
module"]
#[doc(alias = "DP_TEST_80B_PATTERN2")]
pub type DpTest80bPattern2 = crate::Reg<dp_test_80b_pattern2::DpTest80bPattern2Spec>;
#[doc = "80b pattern \\[79:60\\]"]
pub mod dp_test_80b_pattern2;
#[doc = "DP_TEST_HBR2_PATTERN (rw) register accessor: Hbr2 compliance SR count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_test_hbr2_pattern::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_test_hbr2_pattern::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_test_hbr2_pattern`]
module"]
#[doc(alias = "DP_TEST_HBR2_PATTERN")]
pub type DpTestHbr2Pattern = crate::Reg<dp_test_hbr2_pattern::DpTestHbr2PatternSpec>;
#[doc = "Hbr2 compliance SR count"]
pub mod dp_test_hbr2_pattern;
#[doc = "CRC_CON (rw) register accessor: CRC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_con`]
module"]
#[doc(alias = "CRC_CON")]
pub type CrcCon = crate::Reg<crc_con::CrcConSpec>;
#[doc = "CRC control register"]
pub mod crc_con;
#[doc = "ANALOG_CTL_5 (rw) register accessor: PC2 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_5`]
module"]
#[doc(alias = "ANALOG_CTL_5")]
pub type AnalogCtl5 = crate::Reg<analog_ctl_5::AnalogCtl5Spec>;
#[doc = "PC2 Control Register"]
pub mod analog_ctl_5;
#[doc = "ANALOG_CTL_6 (rw) register accessor: AMP_400MV_0DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_6`]
module"]
#[doc(alias = "ANALOG_CTL_6")]
pub type AnalogCtl6 = crate::Reg<analog_ctl_6::AnalogCtl6Spec>;
#[doc = "AMP_400MV_0DB"]
pub mod analog_ctl_6;
#[doc = "ANALOG_CTL_7 (rw) register accessor: AMP_600MV_0DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_7`]
module"]
#[doc(alias = "ANALOG_CTL_7")]
pub type AnalogCtl7 = crate::Reg<analog_ctl_7::AnalogCtl7Spec>;
#[doc = "AMP_600MV_0DB"]
pub mod analog_ctl_7;
#[doc = "ANALOG_CTL_8 (rw) register accessor: AMP_800MV_0DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_8`]
module"]
#[doc(alias = "ANALOG_CTL_8")]
pub type AnalogCtl8 = crate::Reg<analog_ctl_8::AnalogCtl8Spec>;
#[doc = "AMP_800MV_0DB"]
pub mod analog_ctl_8;
#[doc = "ANALOG_CTL_9 (rw) register accessor: AMP_1200MV_0DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_9`]
module"]
#[doc(alias = "ANALOG_CTL_9")]
pub type AnalogCtl9 = crate::Reg<analog_ctl_9::AnalogCtl9Spec>;
#[doc = "AMP_1200MV_0DB"]
pub mod analog_ctl_9;
#[doc = "ANALOG_CTL_10 (rw) register accessor: AMP_400MV_3P5DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_10`]
module"]
#[doc(alias = "ANALOG_CTL_10")]
pub type AnalogCtl10 = crate::Reg<analog_ctl_10::AnalogCtl10Spec>;
#[doc = "AMP_400MV_3P5DB"]
pub mod analog_ctl_10;
#[doc = "ANALOG_CTL_11 (rw) register accessor: AMP_600MV_3P5DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_11`]
module"]
#[doc(alias = "ANALOG_CTL_11")]
pub type AnalogCtl11 = crate::Reg<analog_ctl_11::AnalogCtl11Spec>;
#[doc = "AMP_600MV_3P5DB"]
pub mod analog_ctl_11;
#[doc = "ANALOG_CTL_12 (rw) register accessor: AMP_800MV_3P5DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_12`]
module"]
#[doc(alias = "ANALOG_CTL_12")]
pub type AnalogCtl12 = crate::Reg<analog_ctl_12::AnalogCtl12Spec>;
#[doc = "AMP_800MV_3P5DB"]
pub mod analog_ctl_12;
#[doc = "ANALOG_CTL_13 (rw) register accessor: AMP_400MV_6DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_13`]
module"]
#[doc(alias = "ANALOG_CTL_13")]
pub type AnalogCtl13 = crate::Reg<analog_ctl_13::AnalogCtl13Spec>;
#[doc = "AMP_400MV_6DB"]
pub mod analog_ctl_13;
#[doc = "ANALOG_CTL_14 (rw) register accessor: AMP_600MV_6DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_14`]
module"]
#[doc(alias = "ANALOG_CTL_14")]
pub type AnalogCtl14 = crate::Reg<analog_ctl_14::AnalogCtl14Spec>;
#[doc = "AMP_600MV_6DB"]
pub mod analog_ctl_14;
#[doc = "ANALOG_CTL_15 (rw) register accessor: AMP_400MV_9DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_15`]
module"]
#[doc(alias = "ANALOG_CTL_15")]
pub type AnalogCtl15 = crate::Reg<analog_ctl_15::AnalogCtl15Spec>;
#[doc = "AMP_400MV_9DB"]
pub mod analog_ctl_15;
#[doc = "ANALOG_CTL_16 (rw) register accessor: EMP_400MV_0DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_16`]
module"]
#[doc(alias = "ANALOG_CTL_16")]
pub type AnalogCtl16 = crate::Reg<analog_ctl_16::AnalogCtl16Spec>;
#[doc = "EMP_400MV_0DB"]
pub mod analog_ctl_16;
#[doc = "ANALOG_CTL_17 (rw) register accessor: EMP_600MV_0DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_17`]
module"]
#[doc(alias = "ANALOG_CTL_17")]
pub type AnalogCtl17 = crate::Reg<analog_ctl_17::AnalogCtl17Spec>;
#[doc = "EMP_600MV_0DB"]
pub mod analog_ctl_17;
#[doc = "ANALOG_CTL_18 (rw) register accessor: EMP_800MV_0DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_18`]
module"]
#[doc(alias = "ANALOG_CTL_18")]
pub type AnalogCtl18 = crate::Reg<analog_ctl_18::AnalogCtl18Spec>;
#[doc = "EMP_800MV_0DB"]
pub mod analog_ctl_18;
#[doc = "ANALOG_CTL_19 (rw) register accessor: EMP_1200MV_0DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_19`]
module"]
#[doc(alias = "ANALOG_CTL_19")]
pub type AnalogCtl19 = crate::Reg<analog_ctl_19::AnalogCtl19Spec>;
#[doc = "EMP_1200MV_0DB"]
pub mod analog_ctl_19;
#[doc = "ANALOG_CTL_20 (rw) register accessor: EMP_400MV_3P5DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_20`]
module"]
#[doc(alias = "ANALOG_CTL_20")]
pub type AnalogCtl20 = crate::Reg<analog_ctl_20::AnalogCtl20Spec>;
#[doc = "EMP_400MV_3P5DB"]
pub mod analog_ctl_20;
#[doc = "ANALOG_CTL_21 (rw) register accessor: EMP_600MV_3P5DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_21`]
module"]
#[doc(alias = "ANALOG_CTL_21")]
pub type AnalogCtl21 = crate::Reg<analog_ctl_21::AnalogCtl21Spec>;
#[doc = "EMP_600MV_3P5DB"]
pub mod analog_ctl_21;
#[doc = "ANALOG_CTL_22 (rw) register accessor: EMP_800MV_3P5DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_22`]
module"]
#[doc(alias = "ANALOG_CTL_22")]
pub type AnalogCtl22 = crate::Reg<analog_ctl_22::AnalogCtl22Spec>;
#[doc = "EMP_800MV_3P5DB"]
pub mod analog_ctl_22;
#[doc = "ANALOG_CTL_23 (rw) register accessor: EMP_400MV_6DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_23`]
module"]
#[doc(alias = "ANALOG_CTL_23")]
pub type AnalogCtl23 = crate::Reg<analog_ctl_23::AnalogCtl23Spec>;
#[doc = "EMP_400MV_6DB"]
pub mod analog_ctl_23;
#[doc = "ANALOG_CTL_24 (rw) register accessor: EMP_600MV_6DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_24`]
module"]
#[doc(alias = "ANALOG_CTL_24")]
pub type AnalogCtl24 = crate::Reg<analog_ctl_24::AnalogCtl24Spec>;
#[doc = "EMP_600MV_6DB"]
pub mod analog_ctl_24;
#[doc = "ANALOG_CTL_25 (rw) register accessor: EMP_400MV_9DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_25`]
module"]
#[doc(alias = "ANALOG_CTL_25")]
pub type AnalogCtl25 = crate::Reg<analog_ctl_25::AnalogCtl25Spec>;
#[doc = "EMP_400MV_9DB"]
pub mod analog_ctl_25;
#[doc = "ANALOG_CTL_26 (rw) register accessor: PC2_400MV_0DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_26`]
module"]
#[doc(alias = "ANALOG_CTL_26")]
pub type AnalogCtl26 = crate::Reg<analog_ctl_26::AnalogCtl26Spec>;
#[doc = "PC2_400MV_0DB"]
pub mod analog_ctl_26;
#[doc = "ANALOG_CTL_27 (rw) register accessor: PC2_600MV_0DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_27`]
module"]
#[doc(alias = "ANALOG_CTL_27")]
pub type AnalogCtl27 = crate::Reg<analog_ctl_27::AnalogCtl27Spec>;
#[doc = "PC2_600MV_0DB"]
pub mod analog_ctl_27;
#[doc = "ANALOG_CTL_28 (rw) register accessor: PC2_800MV_0DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_28`]
module"]
#[doc(alias = "ANALOG_CTL_28")]
pub type AnalogCtl28 = crate::Reg<analog_ctl_28::AnalogCtl28Spec>;
#[doc = "PC2_800MV_0DB"]
pub mod analog_ctl_28;
#[doc = "ANALOG_CTL_29 (rw) register accessor: PC2_1200MV_0DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_29`]
module"]
#[doc(alias = "ANALOG_CTL_29")]
pub type AnalogCtl29 = crate::Reg<analog_ctl_29::AnalogCtl29Spec>;
#[doc = "PC2_1200MV_0DB"]
pub mod analog_ctl_29;
#[doc = "ANALOG_CTL_30 (rw) register accessor: PC2_400MV_3P5DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_30`]
module"]
#[doc(alias = "ANALOG_CTL_30")]
pub type AnalogCtl30 = crate::Reg<analog_ctl_30::AnalogCtl30Spec>;
#[doc = "PC2_400MV_3P5DB"]
pub mod analog_ctl_30;
#[doc = "ANALOG_CTL_31 (rw) register accessor: PC2_600MV_3P5DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_31`]
module"]
#[doc(alias = "ANALOG_CTL_31")]
pub type AnalogCtl31 = crate::Reg<analog_ctl_31::AnalogCtl31Spec>;
#[doc = "PC2_600MV_3P5DB"]
pub mod analog_ctl_31;
#[doc = "ANALOG_CTL_32 (rw) register accessor: PC2_800MV_3P5DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_32`]
module"]
#[doc(alias = "ANALOG_CTL_32")]
pub type AnalogCtl32 = crate::Reg<analog_ctl_32::AnalogCtl32Spec>;
#[doc = "PC2_800MV_3P5DB"]
pub mod analog_ctl_32;
#[doc = "ANALOG_CTL_33 (rw) register accessor: PC2_400MV_6DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_33`]
module"]
#[doc(alias = "ANALOG_CTL_33")]
pub type AnalogCtl33 = crate::Reg<analog_ctl_33::AnalogCtl33Spec>;
#[doc = "PC2_400MV_6DB"]
pub mod analog_ctl_33;
#[doc = "ANALOG_CTL_34 (rw) register accessor: PC2_600MV_6DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_34`]
module"]
#[doc(alias = "ANALOG_CTL_34")]
pub type AnalogCtl34 = crate::Reg<analog_ctl_34::AnalogCtl34Spec>;
#[doc = "PC2_600MV_6DB"]
pub mod analog_ctl_34;
#[doc = "ANALOG_CTL_35 (rw) register accessor: PC2_400MV_9DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_35`]
module"]
#[doc(alias = "ANALOG_CTL_35")]
pub type AnalogCtl35 = crate::Reg<analog_ctl_35::AnalogCtl35Spec>;
#[doc = "PC2_400MV_9DB"]
pub mod analog_ctl_35;
#[doc = "ANALOG_CTL_36 (rw) register accessor: CH0_AMP_FORCE_VALUE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_36`]
module"]
#[doc(alias = "ANALOG_CTL_36")]
pub type AnalogCtl36 = crate::Reg<analog_ctl_36::AnalogCtl36Spec>;
#[doc = "CH0_AMP_FORCE_VALUE"]
pub mod analog_ctl_36;
#[doc = "ANALOG_CTL_37 (rw) register accessor: CH0_EMP_FORCE_VALUE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_37::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_37::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_37`]
module"]
#[doc(alias = "ANALOG_CTL_37")]
pub type AnalogCtl37 = crate::Reg<analog_ctl_37::AnalogCtl37Spec>;
#[doc = "CH0_EMP_FORCE_VALUE"]
pub mod analog_ctl_37;
#[doc = "ANALOG_CTL_38 (rw) register accessor: CH0_PC2_FORCE_VALUE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_38::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_38::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_38`]
module"]
#[doc(alias = "ANALOG_CTL_38")]
pub type AnalogCtl38 = crate::Reg<analog_ctl_38::AnalogCtl38Spec>;
#[doc = "CH0_PC2_FORCE_VALUE"]
pub mod analog_ctl_38;
#[doc = "ANALOG_CTL_39 (rw) register accessor: CH1_AMP_FORCE_VALUE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_39::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_39::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_39`]
module"]
#[doc(alias = "ANALOG_CTL_39")]
pub type AnalogCtl39 = crate::Reg<analog_ctl_39::AnalogCtl39Spec>;
#[doc = "CH1_AMP_FORCE_VALUE"]
pub mod analog_ctl_39;
#[doc = "ANALOG_CTL_40 (rw) register accessor: CH1_EMP_FORCE_VALUE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_40::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_40::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_40`]
module"]
#[doc(alias = "ANALOG_CTL_40")]
pub type AnalogCtl40 = crate::Reg<analog_ctl_40::AnalogCtl40Spec>;
#[doc = "CH1_EMP_FORCE_VALUE"]
pub mod analog_ctl_40;
#[doc = "ANALOG_CTL_41 (rw) register accessor: CH1_PC2_FORCE_VALUE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_41::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_41::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_41`]
module"]
#[doc(alias = "ANALOG_CTL_41")]
pub type AnalogCtl41 = crate::Reg<analog_ctl_41::AnalogCtl41Spec>;
#[doc = "CH1_PC2_FORCE_VALUE"]
pub mod analog_ctl_41;
#[doc = "ANALOG_CTL_42 (rw) register accessor: CH0_CH1_FORCE_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_42::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_42::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_42`]
module"]
#[doc(alias = "ANALOG_CTL_42")]
pub type AnalogCtl42 = crate::Reg<analog_ctl_42::AnalogCtl42Spec>;
#[doc = "CH0_CH1_FORCE_CTRL"]
pub mod analog_ctl_42;
#[doc = "ANALOG_CTL_43 (rw) register accessor: CH2_AMP_FORCE_VALUE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_43::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_43::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_43`]
module"]
#[doc(alias = "ANALOG_CTL_43")]
pub type AnalogCtl43 = crate::Reg<analog_ctl_43::AnalogCtl43Spec>;
#[doc = "CH2_AMP_FORCE_VALUE"]
pub mod analog_ctl_43;
#[doc = "ANALOG_CTL_44 (rw) register accessor: CH2_EMP_FORCE_VALUE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_44::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_44::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_44`]
module"]
#[doc(alias = "ANALOG_CTL_44")]
pub type AnalogCtl44 = crate::Reg<analog_ctl_44::AnalogCtl44Spec>;
#[doc = "CH2_EMP_FORCE_VALUE"]
pub mod analog_ctl_44;
#[doc = "ANALOG_CTL_45 (rw) register accessor: CH2_PC2_FORCE_VALUE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_45`]
module"]
#[doc(alias = "ANALOG_CTL_45")]
pub type AnalogCtl45 = crate::Reg<analog_ctl_45::AnalogCtl45Spec>;
#[doc = "CH2_PC2_FORCE_VALUE"]
pub mod analog_ctl_45;
#[doc = "ANALOG_CTL_46 (rw) register accessor: CH3_AMP_FORCE_VALUE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_46::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_46::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_46`]
module"]
#[doc(alias = "ANALOG_CTL_46")]
pub type AnalogCtl46 = crate::Reg<analog_ctl_46::AnalogCtl46Spec>;
#[doc = "CH3_AMP_FORCE_VALUE"]
pub mod analog_ctl_46;
#[doc = "ANALOG_CTL_47 (rw) register accessor: CH3_EMP_FORCE_VALUE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_47::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_47::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_47`]
module"]
#[doc(alias = "ANALOG_CTL_47")]
pub type AnalogCtl47 = crate::Reg<analog_ctl_47::AnalogCtl47Spec>;
#[doc = "CH3_EMP_FORCE_VALUE"]
pub mod analog_ctl_47;
#[doc = "ANALOG_CTL_48 (rw) register accessor: CH3_PC2_FORCE_VALUE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_48::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_48::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_48`]
module"]
#[doc(alias = "ANALOG_CTL_48")]
pub type AnalogCtl48 = crate::Reg<analog_ctl_48::AnalogCtl48Spec>;
#[doc = "CH3_PC2_FORCE_VALUE"]
pub mod analog_ctl_48;
#[doc = "ANALOG_CTL_49 (rw) register accessor: CH2_CH3_FORCE_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_49::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_49::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctl_49`]
module"]
#[doc(alias = "ANALOG_CTL_49")]
pub type AnalogCtl49 = crate::Reg<analog_ctl_49::AnalogCtl49Spec>;
#[doc = "CH2_CH3_FORCE_CTRL"]
pub mod analog_ctl_49;
#[doc = "LINK_POLICY (rw) register accessor: Dp Link Policy\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`link_policy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`link_policy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link_policy`]
module"]
#[doc(alias = "LINK_POLICY")]
pub type LinkPolicy = crate::Reg<link_policy::LinkPolicySpec>;
#[doc = "Dp Link Policy"]
pub mod link_policy;
#[doc = "PLL_REG_1 (rw) register accessor: Pll_control_1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_reg_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_reg_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_reg_1`]
module"]
#[doc(alias = "PLL_REG_1")]
pub type PllReg1 = crate::Reg<pll_reg_1::PllReg1Spec>;
#[doc = "Pll_control_1"]
pub mod pll_reg_1;
#[doc = "PLL_REG_2 (rw) register accessor: Pll_control_2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_reg_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_reg_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_reg_2`]
module"]
#[doc(alias = "PLL_REG_2")]
pub type PllReg2 = crate::Reg<pll_reg_2::PllReg2Spec>;
#[doc = "Pll_control_2"]
pub mod pll_reg_2;
#[doc = "PLL_REG_3 (rw) register accessor: Pll_control_3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_reg_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_reg_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_reg_3`]
module"]
#[doc(alias = "PLL_REG_3")]
pub type PllReg3 = crate::Reg<pll_reg_3::PllReg3Spec>;
#[doc = "Pll_control_3"]
pub mod pll_reg_3;
#[doc = "PLL_REG_4 (r) register accessor: Pll_control_4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_reg_4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_reg_4`]
module"]
#[doc(alias = "PLL_REG_4")]
pub type PllReg4 = crate::Reg<pll_reg_4::PllReg4Spec>;
#[doc = "Pll_control_4"]
pub mod pll_reg_4;
#[doc = "PLL_REG_5 (rw) register accessor: Pll_control_5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_reg_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_reg_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_reg_5`]
module"]
#[doc(alias = "PLL_REG_5")]
pub type PllReg5 = crate::Reg<pll_reg_5::PllReg5Spec>;
#[doc = "Pll_control_5"]
pub mod pll_reg_5;
#[doc = "PLL_REG_MAC (rw) register accessor: Pll_control_MAC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_reg_mac::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_reg_mac::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_reg_mac`]
module"]
#[doc(alias = "PLL_REG_MAC")]
pub type PllRegMac = crate::Reg<pll_reg_mac::PllRegMacSpec>;
#[doc = "Pll_control_MAC"]
pub mod pll_reg_mac;
#[doc = "FREQ_IN_REG (rw) register accessor: freq_in_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`freq_in_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`freq_in_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freq_in_reg`]
module"]
#[doc(alias = "FREQ_IN_REG")]
pub type FreqInReg = crate::Reg<freq_in_reg::FreqInRegSpec>;
#[doc = "freq_in_reg"]
pub mod freq_in_reg;
#[doc = "P_REG_FRQ (rw) register accessor: frequency counter ,digital output for debug\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p_reg_frq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p_reg_frq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p_reg_frq`]
module"]
#[doc(alias = "P_REG_FRQ")]
pub type PRegFrq = crate::Reg<p_reg_frq::PRegFrqSpec>;
#[doc = "frequency counter ,digital output for debug"]
pub mod p_reg_frq;
#[doc = "P_REG_FRQ_COUNT_RDY (rw) register accessor: frequency counter ready indicator\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p_reg_frq_count_rdy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p_reg_frq_count_rdy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p_reg_frq_count_rdy`]
module"]
#[doc(alias = "P_REG_FRQ_COUNT_RDY")]
pub type PRegFrqCountRdy = crate::Reg<p_reg_frq_count_rdy::PRegFrqCountRdySpec>;
#[doc = "frequency counter ready indicator"]
pub mod p_reg_frq_count_rdy;
#[doc = "P_BAND_DEC_RESET (rw) register accessor: reset band decoder\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p_band_dec_reset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p_band_dec_reset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p_band_dec_reset`]
module"]
#[doc(alias = "P_BAND_DEC_RESET")]
pub type PBandDecReset = crate::Reg<p_band_dec_reset::PBandDecResetSpec>;
#[doc = "reset band decoder"]
pub mod p_band_dec_reset;
#[doc = "SSC_REG (rw) register accessor: SSC control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssc_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssc_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssc_reg`]
module"]
#[doc(alias = "SSC_REG")]
pub type SscReg = crate::Reg<ssc_reg::SscRegSpec>;
#[doc = "SSC control"]
pub mod ssc_reg;
#[doc = "TX_COMMON (rw) register accessor: Tx terminal resistor control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_common::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_common::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_common`]
module"]
#[doc(alias = "TX_COMMON")]
pub type TxCommon = crate::Reg<tx_common::TxCommonSpec>;
#[doc = "Tx terminal resistor control"]
pub mod tx_common;
#[doc = "TX_COMMON2 (rw) register accessor: Tx terminal resistor control2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_common2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_common2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_common2`]
module"]
#[doc(alias = "TX_COMMON2")]
pub type TxCommon2 = crate::Reg<tx_common2::TxCommon2Spec>;
#[doc = "Tx terminal resistor control2"]
pub mod tx_common2;
#[doc = "TX_COMMON3 (rw) register accessor: Tx terminal resistor control3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_common3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_common3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_common3`]
module"]
#[doc(alias = "TX_COMMON3")]
pub type TxCommon3 = crate::Reg<tx_common3::TxCommon3Spec>;
#[doc = "Tx terminal resistor control3"]
pub mod tx_common3;
#[doc = "DP_AUX (rw) register accessor: Aux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_aux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_aux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_aux`]
module"]
#[doc(alias = "DP_AUX")]
pub type DpAux = crate::Reg<dp_aux::DpAuxSpec>;
#[doc = "Aux control"]
pub mod dp_aux;
#[doc = "DP_BIAS (rw) register accessor: Bias control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_bias::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_bias::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_bias`]
module"]
#[doc(alias = "DP_BIAS")]
pub type DpBias = crate::Reg<dp_bias::DpBiasSpec>;
#[doc = "Bias control"]
pub mod dp_bias;
#[doc = "DP_TEST (rw) register accessor: Test mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_test`]
module"]
#[doc(alias = "DP_TEST")]
pub type DpTest = crate::Reg<dp_test::DpTestSpec>;
#[doc = "Test mode"]
pub mod dp_test;
#[doc = "DP_PD (rw) register accessor: Power down control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_pd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_pd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_pd`]
module"]
#[doc(alias = "DP_PD")]
pub type DpPd = crate::Reg<dp_pd::DpPdSpec>;
#[doc = "Power down control"]
pub mod dp_pd;
#[doc = "DP_RESERV1 (rw) register accessor: RESERVD1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_reserv1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_reserv1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_reserv1`]
module"]
#[doc(alias = "DP_RESERV1")]
pub type DpReserv1 = crate::Reg<dp_reserv1::DpReserv1Spec>;
#[doc = "RESERVD1"]
pub mod dp_reserv1;
#[doc = "DP_RESERV2 (rw) register accessor: RESERVD2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_reserv2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_reserv2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_reserv2`]
module"]
#[doc(alias = "DP_RESERV2")]
pub type DpReserv2 = crate::Reg<dp_reserv2::DpReserv2Spec>;
#[doc = "RESERVD2"]
pub mod dp_reserv2;
