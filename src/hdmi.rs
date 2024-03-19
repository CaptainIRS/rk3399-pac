#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    design_id: DesignId,
    revision_id: RevisionId,
    product_id0: ProductId0,
    product_id1: ProductId1,
    config0_id: Config0Id,
    config1_id: Config1Id,
    config2_id: Config2Id,
    config3_id: Config3Id,
    _reserved8: [u8; 0xf8],
    ih_fc_stat0: IhFcStat0,
    ih_fc_stat1: IhFcStat1,
    ih_fc_stat2: IhFcStat2,
    ih_as_stat0: IhAsStat0,
    ih_phy_stat0: IhPhyStat0,
    ih_i2cm_stat0: IhI2cmStat0,
    ih_cec_stat0: IhCecStat0,
    ih_vp_stat0: IhVpStat0,
    ih_i2cmphy_stat0: IhI2cmphyStat0,
    ih_ahbdmaaud_stat0: IhAhbdmaaudStat0,
    _reserved18: [u8; 0x66],
    ih_decode: IhDecode,
    _reserved19: [u8; 0x0f],
    ih_mute_fc_stat0: IhMuteFcStat0,
    ih_mute_fc_stat1: IhMuteFcStat1,
    ih_mute_fc_stat2: IhMuteFcStat2,
    ih_mute_as_stat0: IhMuteAsStat0,
    ih_mute_phy_stat0: IhMutePhyStat0,
    ih_mute_i2cm_stat0: IhMuteI2cmStat0,
    ih_mute_cec_stat0: IhMuteCecStat0,
    ih_mute_vp_stat0: IhMuteVpStat0,
    ih_mute_i2cmphy_stat0: IhMuteI2cmphyStat0,
    ih_mute_ahbdmaaud_stat0: IhMuteAhbdmaaudStat0,
    _reserved29: [u8; 0x75],
    ih_mute: IhMute,
    tx_invid0: TxInvid0,
    tx_instuffing: TxInstuffing,
    tx_gydata0: TxGydata0,
    tx_gydata1: TxGydata1,
    tx_rcrdata0: TxRcrdata0,
    tx_rcrdata1: TxRcrdata1,
    tx_bcbdata0: TxBcbdata0,
    tx_bcbdata1: TxBcbdata1,
    _reserved38: [u8; 0x05f8],
    vp_status: VpStatus,
    vp_pr_cd: VpPrCd,
    vp_stuff: VpStuff,
    vp_remap: VpRemap,
    vp_conf: VpConf,
    _reserved43: [u8; 0x02],
    vp_mask: VpMask,
    _reserved44: [u8; 0x07f8],
    fc_invidconf: FcInvidconf,
    fc_inhactiv0: FcInhactiv0,
    fc_inhactiv1: FcInhactiv1,
    fc_inhblank0: FcInhblank0,
    fc_inhblank1: FcInhblank1,
    fc_invactiv0: FcInvactiv0,
    fc_invactiv1: FcInvactiv1,
    fc_invblank: FcInvblank,
    fc_hsyncindelay0: FcHsyncindelay0,
    fc_hsyncindelay1: FcHsyncindelay1,
    fc_hsyncinwidth0: FcHsyncinwidth0,
    fc_hsyncinwidth1: FcHsyncinwidth1,
    fc_vsyncindelay: FcVsyncindelay,
    fc_vsyncinwidth: FcVsyncinwidth,
    fc_infreq0: FcInfreq0,
    fc_infreq1: FcInfreq1,
    fc_infreq2: FcInfreq2,
    fc_ctrldur: FcCtrldur,
    fc_exctrldur: FcExctrldur,
    fc_exctrlspac: FcExctrlspac,
    fc_ch0pream: FcCh0pream,
    fc_ch1pream: FcCh1pream,
    fc_ch2pream: FcCh2pream,
    fc_aviconf3: FcAviconf3,
    fc_gcp: FcGcp,
    fc_aviconf0: FcAviconf0,
    fc_aviconf1: FcAviconf1,
    fc_aviconf2: FcAviconf2,
    fc_avivid: FcAvivid,
    fc_avietb: [FcAvietb; 2],
    fc_avisbb: [FcAvisbb; 2],
    fc_avielb: [FcAvielb; 2],
    fc_avisrb: [FcAvisrb; 2],
    fc_audiconf0: FcAudiconf0,
    fc_audiconf1: FcAudiconf1,
    fc_audiconf2: FcAudiconf2,
    fc_audiconf3: FcAudiconf3,
    fc_vsdieeeid2: FcVsdieeeid2,
    fc_vsdsize: FcVsdsize,
    _reserved83: [u8; 0x05],
    fc_vsdieeeid1: FcVsdieeeid1,
    fc_vsdieeeid0: FcVsdieeeid0,
    fc_vsdpayload: [FcVsdpayload; 24],
    fc_spdvendorname: [FcSpdvendorname; 8],
    fc_spdproductname: [FcSpdproductname; 16],
    fc_spddeviceinf: FcSpddeviceinf,
    fc_audsconf: FcAudsconf,
    fc_audsstat: FcAudsstat,
    fc_audsv: FcAudsv,
    fc_audsu: FcAudsu,
    fc_audschnl0: FcAudschnl0,
    fc_audschnl1: FcAudschnl1,
    fc_audschnl2: FcAudschnl2,
    fc_audschnl3: FcAudschnl3,
    fc_audschnl4: FcAudschnl4,
    fc_audschnl5: FcAudschnl5,
    fc_audschnl6: FcAudschnl6,
    fc_audschnl7: FcAudschnl7,
    fc_audschnl8: FcAudschnl8,
    _reserved102: [u8; 0x03],
    fc_ctrlqhigh: FcCtrlqhigh,
    fc_ctrlqlow: FcCtrlqlow,
    fc_acp0: FcAcp0,
    _reserved105: [u8; 0x0c],
    fc_acp16: FcAcp16,
    fc_acp15: FcAcp15,
    fc_acp14: FcAcp14,
    fc_acp13: FcAcp13,
    fc_acp12: FcAcp12,
    fc_acp11: FcAcp11,
    fc_acp10: FcAcp10,
    fc_acp9: FcAcp9,
    fc_acp8: FcAcp8,
    fc_acp7: FcAcp7,
    fc_acp6: FcAcp6,
    fc_acp5: FcAcp5,
    fc_acp4: FcAcp4,
    fc_acp3: FcAcp3,
    fc_acp2: FcAcp2,
    fc_acp1: FcAcp1,
    fc_iscr1_0: FcIscr1_0,
    fc_iscr1_16: FcIscr1_16,
    fc_iscr1_15: FcIscr1_15,
    fc_iscr1_14: FcIscr1_14,
    fc_iscr1_13: FcIscr1_13,
    fc_iscr1_12: FcIscr1_12,
    fc_iscr1_11: FcIscr1_11,
    fc_iscr1_10: FcIscr1_10,
    fc_iscr1_9: FcIscr1_9,
    fc_iscr1_8: FcIscr1_8,
    fc_iscr1_7: FcIscr1_7,
    fc_iscr1_6: FcIscr1_6,
    fc_iscr1_5: FcIscr1_5,
    fc_iscr1_4: FcIscr1_4,
    fc_iscr1_3: FcIscr1_3,
    fc_iscr1_2: FcIscr1_2,
    fc_iscr1_1: FcIscr1_1,
    fc_iscr2_15: FcIscr2_15,
    fc_iscr2_14: FcIscr2_14,
    fc_iscr2_13: FcIscr2_13,
    fc_iscr2_12: FcIscr2_12,
    fc_iscr2_11: FcIscr2_11,
    fc_iscr2_10: FcIscr2_10,
    fc_iscr2_9: FcIscr2_9,
    fc_iscr2_8: FcIscr2_8,
    fc_iscr2_7: FcIscr2_7,
    fc_iscr2_6: FcIscr2_6,
    fc_iscr2_5: FcIscr2_5,
    fc_iscr2_4: FcIscr2_4,
    fc_iscr2_3: FcIscr2_3,
    fc_iscr2_2: FcIscr2_2,
    fc_iscr2_1: FcIscr2_1,
    fc_iscr2_0: FcIscr2_0,
    fc_datauto0: FcDatauto0,
    fc_datauto1: FcDatauto1,
    fc_datauto2: FcDatauto2,
    fc_datman: FcDatman,
    fc_datauto3: FcDatauto3,
    fc_rdrb0: FcRdrb0,
    fc_rdrb1: FcRdrb1,
    fc_rdrb2: FcRdrb2,
    fc_rdrb3: FcRdrb3,
    fc_rdrb4: FcRdrb4,
    fc_rdrb5: FcRdrb5,
    fc_rdrb6: FcRdrb6,
    fc_rdrb7: FcRdrb7,
    fc_rdrb8: FcRdrb8,
    fc_rdrb9: FcRdrb9,
    fc_rdrb10: FcRdrb10,
    fc_rdrb11: FcRdrb11,
    fc_rdrb12: FcRdrb12,
    fc_rdrb13: FcRdrb13,
    _reserved173: [u8; 0x0c],
    fc_mask0: FcMask0,
    _reserved174: [u8; 0x03],
    fc_mask1: FcMask1,
    _reserved175: [u8; 0x03],
    fc_mask2: FcMask2,
    _reserved176: [u8; 0x05],
    fc_prconf: FcPrconf,
    fc_scrambler_ctrl: FcScramblerCtrl,
    fc_multistream_ctrl: FcMultistreamCtrl,
    fc_packet_tx_en: FcPacketTxEn,
    _reserved180: [u8; 0x04],
    fc_actspc_hdlr_cfg: FcActspcHdlrCfg,
    fc_invact_2d_0: FcInvact2d0,
    fc_invact_2d_1: FcInvact2d1,
    _reserved183: [u8; 0x15],
    fc_gmd_stat: FcGmdStat,
    fc_gmd_en: FcGmdEn,
    fc_gmd_up: FcGmdUp,
    fc_gmd_conf: FcGmdConf,
    fc_gmd_hb: FcGmdHb,
    fc_gmd_pb: [FcGmdPb; 28],
    _reserved189: [u8; 0x07],
    fc_amp_hb1: FcAmpHb1,
    fc_amp_hb2: FcAmpHb2,
    fc_amp_pb: [FcAmpPb; 28],
    _reserved192: [u8; 0x02],
    fc_nvbi_hb1: FcNvbiHb1,
    fc_nvbi_hb2: FcNvbiHb2,
    fc_nvbi_pb: [FcNvbiPb; 27],
    _reserved195: [u8; 0x02],
    fc_drm_up: FcDrmUp,
    fc_drm_hb: [FcDrmHb; 2],
    fc_drm_pb: [FcDrmPb; 27],
    _reserved198: [u8; 0x7b],
    fc_dbgforce: FcDbgforce,
    fc_dbgaud0ch0: FcDbgaud0ch0,
    fc_dbgaud1ch0: FcDbgaud1ch0,
    fc_dbgaud2ch0: FcDbgaud2ch0,
    fc_dbgaud0ch1: FcDbgaud0ch1,
    fc_dbgaud1ch1: FcDbgaud1ch1,
    fc_dbgaud2ch1: FcDbgaud2ch1,
    fc_dbgaud0ch2: FcDbgaud0ch2,
    fc_dbgaud1ch2: FcDbgaud1ch2,
    fc_dbgaud2ch2: FcDbgaud2ch2,
    fc_dbgaud0ch3: FcDbgaud0ch3,
    fc_dbgaud1ch3: FcDbgaud1ch3,
    fc_dbgaud2ch3: FcDbgaud2ch3,
    fc_dbgaud0ch4: FcDbgaud0ch4,
    fc_dbgaud1ch4: FcDbgaud1ch4,
    fc_dbgaud2ch4: FcDbgaud2ch4,
    fc_dbgaud0ch5: FcDbgaud0ch5,
    fc_dbgaud1ch5: FcDbgaud1ch5,
    fc_dbgaud2ch5: FcDbgaud2ch5,
    fc_dbgaud0ch6: FcDbgaud0ch6,
    fc_dbgaud1ch6: FcDbgaud1ch6,
    fc_dbgaud2ch6: FcDbgaud2ch6,
    fc_dbgaud0ch7: FcDbgaud0ch7,
    fc_dbgaud1ch7: FcDbgaud1ch7,
    fc_dbgaud2ch7: FcDbgaud2ch7,
    fc_dbgtmds: [FcDbgtmds; 3],
    _reserved224: [u8; 0x1de4],
    phy_conf0: PhyConf0,
    phy_tst0: PhyTst0,
    phy_tst1: PhyTst1,
    phy_tst2: PhyTst2,
    phy_stat0: PhyStat0,
    phy_int0: PhyInt0,
    phy_mask0: PhyMask0,
    phy_pol0: PhyPol0,
    phy_pclfreq0: PhyPclfreq0,
    phy_pclfreq1: PhyPclfreq1,
    phy_pllcfgfreq0: PhyPllcfgfreq0,
    phy_pllcfgfreq1: PhyPllcfgfreq1,
    phy_pllcfgfreq2: PhyPllcfgfreq2,
    _reserved237: [u8; 0x13],
    phy_i2cm_slave: PhyI2cmSlave,
    phy_i2cm_address: PhyI2cmAddress,
    phy_i2cm_datao_1: PhyI2cmDatao1,
    phy_i2cm_datao_0: PhyI2cmDatao0,
    phy_i2cm_datai_1: PhyI2cmDatai1,
    phy_i2cm_datai_0: PhyI2cmDatai0,
    phy_i2cm_operation: PhyI2cmOperation,
    phy_i2cm_int: PhyI2cmInt,
    phy_i2cm_ctlint: PhyI2cmCtlint,
    phy_i2cm_div: PhyI2cmDiv,
    phy_i2cm_softrstz: PhyI2cmSoftrstz,
    phy_i2cm_ss_scl_hcnt_1_addr: PhyI2cmSsSclHcnt1Addr,
    phy_i2cm_ss_scl_hcnt_0_addr: PhyI2cmSsSclHcnt0Addr,
    phy_i2cm_ss_scl_lcnt_1_addr: PhyI2cmSsSclLcnt1Addr,
    phy_i2cm_ss_scl_lcnt_0_addr: PhyI2cmSsSclLcnt0Addr,
    phy_i2cm_fs_scl_hcnt_1_addr: PhyI2cmFsSclHcnt1Addr,
    phy_i2cm_fs_scl_hcnt_0_addr: PhyI2cmFsSclHcnt0Addr,
    phy_i2cm_fs_scl_lcnt_1_addr: PhyI2cmFsSclLcnt1Addr,
    phy_i2cm_fs_scl_lcnt_0_addr: PhyI2cmFsSclLcnt0Addr,
    phy_i2cm_sda_hold: PhyI2cmSdaHold,
    jtag_phy_config: JtagPhyConfig,
    jtag_phy_tap_tck: JtagPhyTapTck,
    jtag_phy_tap_in: JtagPhyTapIn,
    jtag_phy_tap_out: JtagPhyTapOut,
    jtag_phy_addr: JtagPhyAddr,
    _reserved262: [u8; 0xc7],
    aud_conf0: AudConf0,
    aud_conf1: AudConf1,
    aud_int: AudInt,
    aud_conf2: AudConf2,
    _reserved266: [u8; 0xfc],
    aud_n1: AudN1,
    aud_n2: AudN2,
    aud_n3: AudN3,
    aud_cts1: AudCts1,
    aud_cts2: AudCts2,
    aud_cts3: AudCts3,
    aud_inputclkfs: AudInputclkfs,
    aud_cts_dither: AudCtsDither,
    _reserved274: [u8; 0xf8],
    aud_spdif0: AudSpdif0,
    aud_spdif1: AudSpdif1,
    aud_spdifint: AudSpdifint,
    aud_spdifint1: AudSpdifint1,
    aud_spdif2: AudSpdif2,
    _reserved279: [u8; 0x01fb],
    gp_conf0: GpConf0,
    gp_conf1: GpConf1,
    gp_conf2: GpConf2,
    _reserved282: [u8; 0x03],
    gp_mask: GpMask,
    _reserved283: [u8; 0xf9],
    ahb_dma_conf0: AhbDmaConf0,
    ahb_dma_start: AhbDmaStart,
    ahb_dma_stop: AhbDmaStop,
    ahb_dma_thrsld: AhbDmaThrsld,
    ahb_dma_straddr_set0: [AhbDmaStraddrSet0; 4],
    ahb_dma_stpaddr_set0: [AhbDmaStpaddrSet0; 4],
    ahb_dma_bstraddr: [AhbDmaBstraddr; 4],
    ahb_dma_mblength0: AhbDmaMblength0,
    ahb_dma_mblength1: AhbDmaMblength1,
    _reserved292: [u8; 0x02],
    ahb_dma_mask: AhbDmaMask,
    _reserved293: [u8; 0x01],
    ahb_dma_conf1: AhbDmaConf1,
    _reserved294: [u8; 0x02],
    ahb_dma_buffmask: AhbDmaBuffmask,
    _reserved295: [u8; 0x01],
    ahb_dma_mask1: AhbDmaMask1,
    ahb_dma_status: AhbDmaStatus,
    ahb_dma_conf2: AhbDmaConf2,
    _reserved298: [u8; 0x02],
    ahb_dma_straddr_set1: [AhbDmaStraddrSet1; 4],
    ahb_dma_stpaddr_set1: [AhbDmaStpaddrSet1; 4],
    _reserved300: [u8; 0x09d9],
    mc_clkdis: McClkdis,
    mc_swrstzreq: McSwrstzreq,
    mc_opctrl: McOpctrl,
    mc_flowctrl: McFlowctrl,
    mc_phyrstz: McPhyrstz,
    mc_lockonclock: McLockonclock,
    mc_heacphy_rst: McHeacphyRst,
    _reserved307: [u8; 0x01],
    mc_lockonclock_2: McLockonclock2,
    mc_swrstzreq_2: McSwrstzreq2,
    _reserved309: [u8; 0x05],
    mc_opsts: McOpsts,
    _reserved310: [u8; 0x07],
    base_sfrdivlow: BaseSfrdivlow,
    base_sfrdivhigh: BaseSfrdivhigh,
    _reserved312: [u8; 0xe6],
    csc_cfg: CscCfg,
    csc_scale: CscScale,
    csc_coef_a1_msb: CscCoefA1Msb,
    csc_coef_a1_lsb: CscCoefA1Lsb,
    csc_coef_a2_msb: CscCoefA2Msb,
    csc_coef_a2_lsb: CscCoefA2Lsb,
    csc_coef_a3_msb: CscCoefA3Msb,
    csc_coef_a3_lsb: CscCoefA3Lsb,
    csc_coef_a4_msb: CscCoefA4Msb,
    csc_coef_a4_lsb: CscCoefA4Lsb,
    csc_coef_b1_msb: CscCoefB1Msb,
    csc_coef_b1_lsb: CscCoefB1Lsb,
    csc_coef_b2_msb: CscCoefB2Msb,
    csc_coef_b2_lsb: CscCoefB2Lsb,
    csc_coef_b3_msb: CscCoefB3Msb,
    csc_coef_b3_lsb: CscCoefB3Lsb,
    csc_coef_b4_msb: CscCoefB4Msb,
    csc_coef_b4_lsb: CscCoefB4Lsb,
    csc_coef_c1_msb: CscCoefC1Msb,
    csc_coef_c1_lsb: CscCoefC1Lsb,
    csc_coef_c2_msb: CscCoefC2Msb,
    csc_coef_c2_lsb: CscCoefC2Lsb,
    csc_coef_c3_msb: CscCoefC3Msb,
    csc_coef_c3_lsb: CscCoefC3Lsb,
    csc_coef_c4_msb: CscCoefC4Msb,
    csc_coef_c4_lsb: CscCoefC4Lsb,
    csc_limit_up_msb: CscLimitUpMsb,
    csc_limit_up_lsb: CscLimitUpLsb,
    csc_limit_dn_msb: CscLimitDnMsb,
    csc_limit_dn_lsb: CscLimitDnLsb,
    _reserved342: [u8; 0x0ee2],
    a_hdcpcfg0: AHdcpcfg0,
    a_hdcpcfg1: AHdcpcfg1,
    a_hdcpobs0: AHdcpobs0,
    a_hdcpobs1: AHdcpobs1,
    a_hdcpobs2: AHdcpobs2,
    a_hdcpobs3: AHdcpobs3,
    a_apiintclr: AApiintclr,
    a_apiintstat: AApiintstat,
    a_apiintmsk: AApiintmsk,
    a_vidpolcfg: AVidpolcfg,
    a_oesswcfg: AOesswcfg,
    _reserved353: [u8; 0x09],
    a_coreverlsb: ACoreverlsb,
    a_corevermsb: ACorevermsb,
    a_ksvmemctrl: AKsvmemctrl,
    _reserved356: [u8; 0x09],
    hdcp_bstatus: [HdcpBstatus; 2],
    hdcp_m0: [HdcpM0; 8],
    hdcp_ksv: [HdcpKsv; 635],
    hdcp_vh: [HdcpVh; 20],
    hdcp_revoc_size_0: HdcpRevocSize0,
    hdcp_revoc_size_1: HdcpRevocSize1,
    hdcp_revoc_list: [HdcpRevocList; 5060],
    _reserved363: [u8; 0x1181],
    hdcpreg_bksv0: HdcpregBksv0,
    hdcpreg_bksv1: HdcpregBksv1,
    hdcpreg_bksv2: HdcpregBksv2,
    hdcpreg_bksv3: HdcpregBksv3,
    hdcpreg_bksv4: HdcpregBksv4,
    hdcpreg_anconf: HdcpregAnconf,
    hdcpreg_an0: HdcpregAn0,
    hdcpreg_an1: HdcpregAn1,
    hdcpreg_an2: HdcpregAn2,
    hdcpreg_an3: HdcpregAn3,
    hdcpreg_an4: HdcpregAn4,
    hdcpreg_an5: HdcpregAn5,
    hdcpreg_an6: HdcpregAn6,
    hdcpreg_an7: HdcpregAn7,
    hdcpreg_rmlctl: HdcpregRmlctl,
    hdcpreg_rmlsts: HdcpregRmlsts,
    hdcpreg_seed0: HdcpregSeed0,
    hdcpreg_seed1: HdcpregSeed1,
    hdcpreg_dpk0: HdcpregDpk0,
    hdcpreg_dpk1: HdcpregDpk1,
    hdcpreg_dpk2: HdcpregDpk2,
    hdcpreg_dpk3: HdcpregDpk3,
    hdcpreg_dpk4: HdcpregDpk4,
    hdcpreg_dpk5: HdcpregDpk5,
    hdcpreg_dpk6: HdcpregDpk6,
    _reserved388: [u8; 0xe7],
    hdcp22reg_id: Hdcp22regId,
    _reserved389: [u8; 0x03],
    hdcp22reg_ctrl: Hdcp22regCtrl,
    hdcp22reg_ctrl1: Hdcp22regCtrl1,
    _reserved391: [u8; 0x02],
    hdcp22reg_sts: Hdcp22regSts,
    _reserved392: [u8; 0x03],
    hdcp22reg_mask: Hdcp22regMask,
    hdcp22reg_stat: Hdcp22regStat,
    hdcp22reg_mute: Hdcp22regMute,
    _reserved395: [u8; 0x03f1],
    cec_ctrl: CecCtrl,
    _reserved396: [u8; 0x01],
    cec_mask: CecMask,
    _reserved397: [u8; 0x02],
    cec_addr_l: CecAddrL,
    cec_addr_h: CecAddrH,
    cec_tx_cnt: CecTxCnt,
    cec_rx_cnt: CecRxCnt,
    _reserved401: [u8; 0x07],
    cec_tx_data: [CecTxData; 16],
    cec_rx_data: [CecRxData; 16],
    cec_lock: CecLock,
    cec_wakeupctrl: CecWakeupctrl,
    _reserved405: [u8; 0xce],
    i2cm_slave: I2cmSlave,
    i2cm_address: I2cmAddress,
    i2cm_datao: I2cmDatao,
    i2cm_datai: I2cmDatai,
    i2cm_operation: I2cmOperation,
    i2cm_int: I2cmInt,
    i2cm_ctlint: I2cmCtlint,
    i2cm_div: I2cmDiv,
    i2cm_segaddr: I2cmSegaddr,
    i2cm_softrstz: I2cmSoftrstz,
    i2cm_segptr: I2cmSegptr,
    i2cm_ss_scl_hcnt_1_addr: I2cmSsSclHcnt1Addr,
    i2cm_ss_scl_hcnt_0_addr: I2cmSsSclHcnt0Addr,
    i2cm_ss_scl_lcnt_1_addr: I2cmSsSclLcnt1Addr,
    i2cm_ss_scl_lcnt_0_addr: I2cmSsSclLcnt0Addr,
    i2cm_fs_scl_hcnt_1_addr: I2cmFsSclHcnt1Addr,
    i2cm_fs_scl_hcnt_0_addr: I2cmFsSclHcnt0Addr,
    i2cm_fs_scl_lcnt_1_addr: I2cmFsSclLcnt1Addr,
    i2cm_fs_scl_lcnt_0_addr: I2cmFsSclLcnt0Addr,
    i2cm_sda_hold: I2cmSdaHold,
    i2cm_scdc_read_update: I2cmScdcReadUpdate,
    _reserved426: [u8; 0x0b],
    i2cm_read_buff0: I2cmReadBuff0,
    i2cm_read_buff1: I2cmReadBuff1,
    i2cm_read_buff2: I2cmReadBuff2,
    i2cm_read_buff3: I2cmReadBuff3,
    i2cm_read_buff4: I2cmReadBuff4,
    i2cm_read_buff5: I2cmReadBuff5,
    i2cm_read_buff6: I2cmReadBuff6,
    i2cm_read_buff7: I2cmReadBuff7,
    _reserved434: [u8; 0x08],
    i2cm_scdc_update0: I2cmScdcUpdate0,
    i2cm_scdc_update1: I2cmScdcUpdate1,
}
impl RegisterBlock {
    #[doc = "0x00 - Design Identification Register"]
    #[inline(always)]
    pub const fn design_id(&self) -> &DesignId {
        &self.design_id
    }
    #[doc = "0x01 - Revision Identification Register"]
    #[inline(always)]
    pub const fn revision_id(&self) -> &RevisionId {
        &self.revision_id
    }
    #[doc = "0x02 - Product Identification Register 0"]
    #[inline(always)]
    pub const fn product_id0(&self) -> &ProductId0 {
        &self.product_id0
    }
    #[doc = "0x03 - Product Identification Register 1"]
    #[inline(always)]
    pub const fn product_id1(&self) -> &ProductId1 {
        &self.product_id1
    }
    #[doc = "0x04 - Configuration Identification Register 0"]
    #[inline(always)]
    pub const fn config0_id(&self) -> &Config0Id {
        &self.config0_id
    }
    #[doc = "0x05 - Configuration Identification Register 1"]
    #[inline(always)]
    pub const fn config1_id(&self) -> &Config1Id {
        &self.config1_id
    }
    #[doc = "0x06 - Configuration Identification Register 2"]
    #[inline(always)]
    pub const fn config2_id(&self) -> &Config2Id {
        &self.config2_id
    }
    #[doc = "0x07 - Configuration Identification Register 3"]
    #[inline(always)]
    pub const fn config3_id(&self) -> &Config3Id {
        &self.config3_id
    }
    #[doc = "0x100 - Frame Composer Interrupt Status Register 0 (Packet Interrupts)"]
    #[inline(always)]
    pub const fn ih_fc_stat0(&self) -> &IhFcStat0 {
        &self.ih_fc_stat0
    }
    #[doc = "0x101 - Frame Composer Interrupt Status Register 1 (Packet Interrupts)"]
    #[inline(always)]
    pub const fn ih_fc_stat1(&self) -> &IhFcStat1 {
        &self.ih_fc_stat1
    }
    #[doc = "0x102 - Frame Composer Interrupt Status Register 2 (Packet Interrupts)"]
    #[inline(always)]
    pub const fn ih_fc_stat2(&self) -> &IhFcStat2 {
        &self.ih_fc_stat2
    }
    #[doc = "0x103 - Audio Sampler Interrupt Status Register (FIFO Threshold, Underflow and\n\nOverflow Interrupts)"]
    #[inline(always)]
    pub const fn ih_as_stat0(&self) -> &IhAsStat0 {
        &self.ih_as_stat0
    }
    #[doc = "0x104 - PHY Interface Interrupt Status Register (RXSENSE, PLL Lock and HPD\n\nInterrupts)"]
    #[inline(always)]
    pub const fn ih_phy_stat0(&self) -> &IhPhyStat0 {
        &self.ih_phy_stat0
    }
    #[doc = "0x105 - E-DDC I2C Master Interrupt Status Register (Done and Error Interrupts)"]
    #[inline(always)]
    pub const fn ih_i2cm_stat0(&self) -> &IhI2cmStat0 {
        &self.ih_i2cm_stat0
    }
    #[doc = "0x106 - CEC Interrupt Status Register (Functional Operation Interrupts)"]
    #[inline(always)]
    pub const fn ih_cec_stat0(&self) -> &IhCecStat0 {
        &self.ih_cec_stat0
    }
    #[doc = "0x107 - Video Packetizer Interrupt Status Register (FIFO Full and Empty Interrupts)"]
    #[inline(always)]
    pub const fn ih_vp_stat0(&self) -> &IhVpStat0 {
        &self.ih_vp_stat0
    }
    #[doc = "0x108 - PHY GEN2 I2C Master Interrupt Status Register (Done and Error Interrupts)"]
    #[inline(always)]
    pub const fn ih_i2cmphy_stat0(&self) -> &IhI2cmphyStat0 {
        &self.ih_i2cmphy_stat0
    }
    #[doc = "0x109 - AHB Audio DMA Interrupt Status Register (Functional Operation, Buffer Full\n\nand Empty Interrupts)"]
    #[inline(always)]
    pub const fn ih_ahbdmaaud_stat0(&self) -> &IhAhbdmaaudStat0 {
        &self.ih_ahbdmaaud_stat0
    }
    #[doc = "0x170 - Interruption Handler Decode Assist Register"]
    #[inline(always)]
    pub const fn ih_decode(&self) -> &IhDecode {
        &self.ih_decode
    }
    #[doc = "0x180 - Frame Composer Interrupt Mute Control Register 0"]
    #[inline(always)]
    pub const fn ih_mute_fc_stat0(&self) -> &IhMuteFcStat0 {
        &self.ih_mute_fc_stat0
    }
    #[doc = "0x181 - Frame Composer Interrupt Mute Control Register 1"]
    #[inline(always)]
    pub const fn ih_mute_fc_stat1(&self) -> &IhMuteFcStat1 {
        &self.ih_mute_fc_stat1
    }
    #[doc = "0x182 - Frame Composer Interrupt Mute Control Register 2"]
    #[inline(always)]
    pub const fn ih_mute_fc_stat2(&self) -> &IhMuteFcStat2 {
        &self.ih_mute_fc_stat2
    }
    #[doc = "0x183 - Audio Sampler Interrupt Mute Control Register"]
    #[inline(always)]
    pub const fn ih_mute_as_stat0(&self) -> &IhMuteAsStat0 {
        &self.ih_mute_as_stat0
    }
    #[doc = "0x184 - PHY Interface Interrupt Mute Control Register"]
    #[inline(always)]
    pub const fn ih_mute_phy_stat0(&self) -> &IhMutePhyStat0 {
        &self.ih_mute_phy_stat0
    }
    #[doc = "0x185 - E-DDC I2C Master Interrupt Mute Control Register"]
    #[inline(always)]
    pub const fn ih_mute_i2cm_stat0(&self) -> &IhMuteI2cmStat0 {
        &self.ih_mute_i2cm_stat0
    }
    #[doc = "0x186 - CEC Interrupt Mute Control Register"]
    #[inline(always)]
    pub const fn ih_mute_cec_stat0(&self) -> &IhMuteCecStat0 {
        &self.ih_mute_cec_stat0
    }
    #[doc = "0x187 - Video Packetizer Interrupt Mute Control Register"]
    #[inline(always)]
    pub const fn ih_mute_vp_stat0(&self) -> &IhMuteVpStat0 {
        &self.ih_mute_vp_stat0
    }
    #[doc = "0x188 - PHY GEN2 I2C Master Interrupt Mute Control Register"]
    #[inline(always)]
    pub const fn ih_mute_i2cmphy_stat0(&self) -> &IhMuteI2cmphyStat0 {
        &self.ih_mute_i2cmphy_stat0
    }
    #[doc = "0x189 - AHB Audio DMA Interrupt Mute Control Register"]
    #[inline(always)]
    pub const fn ih_mute_ahbdmaaud_stat0(&self) -> &IhMuteAhbdmaaudStat0 {
        &self.ih_mute_ahbdmaaud_stat0
    }
    #[doc = "0x1ff - Global Interrupt Mute Control Register"]
    #[inline(always)]
    pub const fn ih_mute(&self) -> &IhMute {
        &self.ih_mute
    }
    #[doc = "0x200 - Video Input Mapping and Internal Data Enable Configuration Register"]
    #[inline(always)]
    pub const fn tx_invid0(&self) -> &TxInvid0 {
        &self.tx_invid0
    }
    #[doc = "0x201 - Video Input Stuffing Enable Register"]
    #[inline(always)]
    pub const fn tx_instuffing(&self) -> &TxInstuffing {
        &self.tx_instuffing
    }
    #[doc = "0x202 - Video Input gy Data Channel Stuffing Register 0"]
    #[inline(always)]
    pub const fn tx_gydata0(&self) -> &TxGydata0 {
        &self.tx_gydata0
    }
    #[doc = "0x203 - Video Input gy Data Channel Stuffing Register 1"]
    #[inline(always)]
    pub const fn tx_gydata1(&self) -> &TxGydata1 {
        &self.tx_gydata1
    }
    #[doc = "0x204 - Video Input rcr Data Channel Stuffing Register 0"]
    #[inline(always)]
    pub const fn tx_rcrdata0(&self) -> &TxRcrdata0 {
        &self.tx_rcrdata0
    }
    #[doc = "0x205 - Video Input rcr Data Channel Stuffing Register 1"]
    #[inline(always)]
    pub const fn tx_rcrdata1(&self) -> &TxRcrdata1 {
        &self.tx_rcrdata1
    }
    #[doc = "0x206 - Video Input bcb Data Channel Stuffing Register 0"]
    #[inline(always)]
    pub const fn tx_bcbdata0(&self) -> &TxBcbdata0 {
        &self.tx_bcbdata0
    }
    #[doc = "0x207 - Video Input bcb Data Channel Stuffing Register 1"]
    #[inline(always)]
    pub const fn tx_bcbdata1(&self) -> &TxBcbdata1 {
        &self.tx_bcbdata1
    }
    #[doc = "0x800 - Video Packetizer Packing Phase Status Register"]
    #[inline(always)]
    pub const fn vp_status(&self) -> &VpStatus {
        &self.vp_status
    }
    #[doc = "0x801 - Video Packetizer Pixel Repetition and Color Depth Register"]
    #[inline(always)]
    pub const fn vp_pr_cd(&self) -> &VpPrCd {
        &self.vp_pr_cd
    }
    #[doc = "0x802 - Video Packetizer Stuffing and Default Packing Phase Register"]
    #[inline(always)]
    pub const fn vp_stuff(&self) -> &VpStuff {
        &self.vp_stuff
    }
    #[doc = "0x803 - Video Packetizer YCC422 Remapping Register"]
    #[inline(always)]
    pub const fn vp_remap(&self) -> &VpRemap {
        &self.vp_remap
    }
    #[doc = "0x804 - Video Packetizer Output and Enable Configuration Register"]
    #[inline(always)]
    pub const fn vp_conf(&self) -> &VpConf {
        &self.vp_conf
    }
    #[doc = "0x807 - Video Packetizer Interrupt Mask Register"]
    #[inline(always)]
    pub const fn vp_mask(&self) -> &VpMask {
        &self.vp_mask
    }
    #[doc = "0x1000 - Frame Composer Input Video Configuration and HDCP Keepout Register"]
    #[inline(always)]
    pub const fn fc_invidconf(&self) -> &FcInvidconf {
        &self.fc_invidconf
    }
    #[doc = "0x1001 - Frame Composer Input Video HActive Pixels Register 0"]
    #[inline(always)]
    pub const fn fc_inhactiv0(&self) -> &FcInhactiv0 {
        &self.fc_inhactiv0
    }
    #[doc = "0x1002 - Frame Composer Input Video HActive Pixels Register 1"]
    #[inline(always)]
    pub const fn fc_inhactiv1(&self) -> &FcInhactiv1 {
        &self.fc_inhactiv1
    }
    #[doc = "0x1003 - Frame Composer Input Video HBlank Pixels Register 0"]
    #[inline(always)]
    pub const fn fc_inhblank0(&self) -> &FcInhblank0 {
        &self.fc_inhblank0
    }
    #[doc = "0x1004 - Frame Composer Input Video HBlank Pixels Register 1"]
    #[inline(always)]
    pub const fn fc_inhblank1(&self) -> &FcInhblank1 {
        &self.fc_inhblank1
    }
    #[doc = "0x1005 - Frame Composer Input Video VActive Pixels Register 0"]
    #[inline(always)]
    pub const fn fc_invactiv0(&self) -> &FcInvactiv0 {
        &self.fc_invactiv0
    }
    #[doc = "0x1006 - Frame Composer Input Video VActive Pixels Register 1"]
    #[inline(always)]
    pub const fn fc_invactiv1(&self) -> &FcInvactiv1 {
        &self.fc_invactiv1
    }
    #[doc = "0x1007 - Frame Composer Input Video VBlank Pixels Register"]
    #[inline(always)]
    pub const fn fc_invblank(&self) -> &FcInvblank {
        &self.fc_invblank
    }
    #[doc = "0x1008 - Frame Composer Input Video HSync Front Porch Register 0"]
    #[inline(always)]
    pub const fn fc_hsyncindelay0(&self) -> &FcHsyncindelay0 {
        &self.fc_hsyncindelay0
    }
    #[doc = "0x1009 - Frame Composer Input Video HSync Front Porch Register 1"]
    #[inline(always)]
    pub const fn fc_hsyncindelay1(&self) -> &FcHsyncindelay1 {
        &self.fc_hsyncindelay1
    }
    #[doc = "0x100a - Frame Composer Input Video HSync Width Register 0"]
    #[inline(always)]
    pub const fn fc_hsyncinwidth0(&self) -> &FcHsyncinwidth0 {
        &self.fc_hsyncinwidth0
    }
    #[doc = "0x100b - Frame Composer Input Video HSync Width Register 1"]
    #[inline(always)]
    pub const fn fc_hsyncinwidth1(&self) -> &FcHsyncinwidth1 {
        &self.fc_hsyncinwidth1
    }
    #[doc = "0x100c - Frame Composer Input Video VSync Front Porch Register"]
    #[inline(always)]
    pub const fn fc_vsyncindelay(&self) -> &FcVsyncindelay {
        &self.fc_vsyncindelay
    }
    #[doc = "0x100d - Frame Composer Input Video VSync Width Register"]
    #[inline(always)]
    pub const fn fc_vsyncinwidth(&self) -> &FcVsyncinwidth {
        &self.fc_vsyncinwidth
    }
    #[doc = "0x100e - Frame Composer Input Video Refresh Rate Register 0"]
    #[inline(always)]
    pub const fn fc_infreq0(&self) -> &FcInfreq0 {
        &self.fc_infreq0
    }
    #[doc = "0x100f - Frame Composer Input Video Refresh Rate Register 1"]
    #[inline(always)]
    pub const fn fc_infreq1(&self) -> &FcInfreq1 {
        &self.fc_infreq1
    }
    #[doc = "0x1010 - Frame Composer Input Video Refresh Rate Register 2"]
    #[inline(always)]
    pub const fn fc_infreq2(&self) -> &FcInfreq2 {
        &self.fc_infreq2
    }
    #[doc = "0x1011 - Frame Composer Control Period Duration Register"]
    #[inline(always)]
    pub const fn fc_ctrldur(&self) -> &FcCtrldur {
        &self.fc_ctrldur
    }
    #[doc = "0x1012 - Frame Composer Extended Control Period Duration Register"]
    #[inline(always)]
    pub const fn fc_exctrldur(&self) -> &FcExctrldur {
        &self.fc_exctrldur
    }
    #[doc = "0x1013 - Frame Composer Extended Control Period Maximum Spacing Register"]
    #[inline(always)]
    pub const fn fc_exctrlspac(&self) -> &FcExctrlspac {
        &self.fc_exctrlspac
    }
    #[doc = "0x1014 - Frame Composer Channel 0 Non-Preamble Data Register"]
    #[inline(always)]
    pub const fn fc_ch0pream(&self) -> &FcCh0pream {
        &self.fc_ch0pream
    }
    #[doc = "0x1015 - Frame Composer Channel 1 Non-Preamble Data Register"]
    #[inline(always)]
    pub const fn fc_ch1pream(&self) -> &FcCh1pream {
        &self.fc_ch1pream
    }
    #[doc = "0x1016 - Frame Composer Channel 2 Non-Preamble Data Register"]
    #[inline(always)]
    pub const fn fc_ch2pream(&self) -> &FcCh2pream {
        &self.fc_ch2pream
    }
    #[doc = "0x1017 - Frame Composer AVI Packet Configuration Register 3"]
    #[inline(always)]
    pub const fn fc_aviconf3(&self) -> &FcAviconf3 {
        &self.fc_aviconf3
    }
    #[doc = "0x1018 - Frame Composer GCP Packet Configuration Register"]
    #[inline(always)]
    pub const fn fc_gcp(&self) -> &FcGcp {
        &self.fc_gcp
    }
    #[doc = "0x1019 - Frame Composer AVI Packet Configuration Register 0"]
    #[inline(always)]
    pub const fn fc_aviconf0(&self) -> &FcAviconf0 {
        &self.fc_aviconf0
    }
    #[doc = "0x101a - Frame Composer AVI Packet Configuration Register 1"]
    #[inline(always)]
    pub const fn fc_aviconf1(&self) -> &FcAviconf1 {
        &self.fc_aviconf1
    }
    #[doc = "0x101b - Frame Composer AVI Packet Configuration Register 2"]
    #[inline(always)]
    pub const fn fc_aviconf2(&self) -> &FcAviconf2 {
        &self.fc_aviconf2
    }
    #[doc = "0x101c - Frame Composer AVI Packet VIC Register"]
    #[inline(always)]
    pub const fn fc_avivid(&self) -> &FcAvivid {
        &self.fc_avivid
    }
    #[doc = "0x101d - Frame Composer AVI Packet End of Top Bar Register Array"]
    #[inline(always)]
    pub const fn fc_avietb(&self, n: usize) -> &FcAvietb {
        &self.fc_avietb[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x101d - Frame Composer AVI Packet End of Top Bar Register Array"]
    #[inline(always)]
    pub fn fc_avietb_iter(&self) -> impl Iterator<Item = &FcAvietb> {
        self.fc_avietb.iter()
    }
    #[doc = "0x101f - Frame Composer AVI Packet Start of Bottom Bar Register Array"]
    #[inline(always)]
    pub const fn fc_avisbb(&self, n: usize) -> &FcAvisbb {
        &self.fc_avisbb[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x101f - Frame Composer AVI Packet Start of Bottom Bar Register Array"]
    #[inline(always)]
    pub fn fc_avisbb_iter(&self) -> impl Iterator<Item = &FcAvisbb> {
        self.fc_avisbb.iter()
    }
    #[doc = "0x1021 - Frame Composer AVI Packet End of Left Bar Register Array"]
    #[inline(always)]
    pub const fn fc_avielb(&self, n: usize) -> &FcAvielb {
        &self.fc_avielb[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1021 - Frame Composer AVI Packet End of Left Bar Register Array"]
    #[inline(always)]
    pub fn fc_avielb_iter(&self) -> impl Iterator<Item = &FcAvielb> {
        self.fc_avielb.iter()
    }
    #[doc = "0x1023 - Frame Composer AVI Packet Start of Right Bar Register Array"]
    #[inline(always)]
    pub const fn fc_avisrb(&self, n: usize) -> &FcAvisrb {
        &self.fc_avisrb[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1023 - Frame Composer AVI Packet Start of Right Bar Register Array"]
    #[inline(always)]
    pub fn fc_avisrb_iter(&self) -> impl Iterator<Item = &FcAvisrb> {
        self.fc_avisrb.iter()
    }
    #[doc = "0x1025 - Frame Composer AUD Packet Configuration Register 0"]
    #[inline(always)]
    pub const fn fc_audiconf0(&self) -> &FcAudiconf0 {
        &self.fc_audiconf0
    }
    #[doc = "0x1026 - Frame Composer AUD Packet Configuration Register 1"]
    #[inline(always)]
    pub const fn fc_audiconf1(&self) -> &FcAudiconf1 {
        &self.fc_audiconf1
    }
    #[doc = "0x1027 - Frame Composer AUD Packet Configuration Register 2"]
    #[inline(always)]
    pub const fn fc_audiconf2(&self) -> &FcAudiconf2 {
        &self.fc_audiconf2
    }
    #[doc = "0x1028 - Frame Composer AUD Packet Configuration Register 3"]
    #[inline(always)]
    pub const fn fc_audiconf3(&self) -> &FcAudiconf3 {
        &self.fc_audiconf3
    }
    #[doc = "0x1029 - Frame Composer VSI Packet Data IEEE Register 2"]
    #[inline(always)]
    pub const fn fc_vsdieeeid2(&self) -> &FcVsdieeeid2 {
        &self.fc_vsdieeeid2
    }
    #[doc = "0x102a - Frame Composer VSI Packet Data Size Register"]
    #[inline(always)]
    pub const fn fc_vsdsize(&self) -> &FcVsdsize {
        &self.fc_vsdsize
    }
    #[doc = "0x1030 - Frame Composer VSI Packet Data IEEE Register 1"]
    #[inline(always)]
    pub const fn fc_vsdieeeid1(&self) -> &FcVsdieeeid1 {
        &self.fc_vsdieeeid1
    }
    #[doc = "0x1031 - Frame Composer VSI Packet Data IEEE Register 0"]
    #[inline(always)]
    pub const fn fc_vsdieeeid0(&self) -> &FcVsdieeeid0 {
        &self.fc_vsdieeeid0
    }
    #[doc = "0x1032..0x104a - Frame Composer VSI Packet Data Payload Register Array"]
    #[inline(always)]
    pub const fn fc_vsdpayload(&self, n: usize) -> &FcVsdpayload {
        &self.fc_vsdpayload[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1032..0x104a - Frame Composer VSI Packet Data Payload Register Array"]
    #[inline(always)]
    pub fn fc_vsdpayload_iter(&self) -> impl Iterator<Item = &FcVsdpayload> {
        self.fc_vsdpayload.iter()
    }
    #[doc = "0x104a..0x1052 - Frame Composer SPD Packet Data Vendor Name Register Array"]
    #[inline(always)]
    pub const fn fc_spdvendorname(&self, n: usize) -> &FcSpdvendorname {
        &self.fc_spdvendorname[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x104a..0x1052 - Frame Composer SPD Packet Data Vendor Name Register Array"]
    #[inline(always)]
    pub fn fc_spdvendorname_iter(&self) -> impl Iterator<Item = &FcSpdvendorname> {
        self.fc_spdvendorname.iter()
    }
    #[doc = "0x1052..0x1062 - Frame Composer SPD packet Data Product Name Register Array"]
    #[inline(always)]
    pub const fn fc_spdproductname(&self, n: usize) -> &FcSpdproductname {
        &self.fc_spdproductname[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1052..0x1062 - Frame Composer SPD packet Data Product Name Register Array"]
    #[inline(always)]
    pub fn fc_spdproductname_iter(&self) -> impl Iterator<Item = &FcSpdproductname> {
        self.fc_spdproductname.iter()
    }
    #[doc = "0x1062 - Frame Composer SPD Packet Data Source Product Descriptor Register"]
    #[inline(always)]
    pub const fn fc_spddeviceinf(&self) -> &FcSpddeviceinf {
        &self.fc_spddeviceinf
    }
    #[doc = "0x1063 - Frame Composer Audio Sample Flat and Layout Configuration Register"]
    #[inline(always)]
    pub const fn fc_audsconf(&self) -> &FcAudsconf {
        &self.fc_audsconf
    }
    #[doc = "0x1064 - Frame Composer Audio Sample Flat and Layout Status Register"]
    #[inline(always)]
    pub const fn fc_audsstat(&self) -> &FcAudsstat {
        &self.fc_audsstat
    }
    #[doc = "0x1065 - Frame Composer Audio Sample Validity Flag Register"]
    #[inline(always)]
    pub const fn fc_audsv(&self) -> &FcAudsv {
        &self.fc_audsv
    }
    #[doc = "0x1066 - Frame Composer Audio Sample User Flag Register"]
    #[inline(always)]
    pub const fn fc_audsu(&self) -> &FcAudsu {
        &self.fc_audsu
    }
    #[doc = "0x1067 - Frame Composer Audio Sample Channel Status Configuration Register 0"]
    #[inline(always)]
    pub const fn fc_audschnl0(&self) -> &FcAudschnl0 {
        &self.fc_audschnl0
    }
    #[doc = "0x1068 - Frame Composer Audio Sample Channel Status Configuration Register 1"]
    #[inline(always)]
    pub const fn fc_audschnl1(&self) -> &FcAudschnl1 {
        &self.fc_audschnl1
    }
    #[doc = "0x1069 - Frame Composer Audio Sample Channel Status Configuration Register 2"]
    #[inline(always)]
    pub const fn fc_audschnl2(&self) -> &FcAudschnl2 {
        &self.fc_audschnl2
    }
    #[doc = "0x106a - Frame Composer Audio Sample Channel Status Configuration Register 3"]
    #[inline(always)]
    pub const fn fc_audschnl3(&self) -> &FcAudschnl3 {
        &self.fc_audschnl3
    }
    #[doc = "0x106b - Frame Composer Audio Sample Channel Status Configuration Register 4"]
    #[inline(always)]
    pub const fn fc_audschnl4(&self) -> &FcAudschnl4 {
        &self.fc_audschnl4
    }
    #[doc = "0x106c - Frame Composer Audio Sample Channel Status Configuration Register 5"]
    #[inline(always)]
    pub const fn fc_audschnl5(&self) -> &FcAudschnl5 {
        &self.fc_audschnl5
    }
    #[doc = "0x106d - Frame Composer Audio Sample Channel Status Configuration Register 6"]
    #[inline(always)]
    pub const fn fc_audschnl6(&self) -> &FcAudschnl6 {
        &self.fc_audschnl6
    }
    #[doc = "0x106e - Frame Composer Audio Sample Channel Status Configuration Register 7"]
    #[inline(always)]
    pub const fn fc_audschnl7(&self) -> &FcAudschnl7 {
        &self.fc_audschnl7
    }
    #[doc = "0x106f - Frame Composer Audio Sample Channel Status Configuration Register 8"]
    #[inline(always)]
    pub const fn fc_audschnl8(&self) -> &FcAudschnl8 {
        &self.fc_audschnl8
    }
    #[doc = "0x1073 - Frame Composer Number of High Priority Packets Attended Configuration\n\nRegister"]
    #[inline(always)]
    pub const fn fc_ctrlqhigh(&self) -> &FcCtrlqhigh {
        &self.fc_ctrlqhigh
    }
    #[doc = "0x1074 - Frame Composer Number of Low Priority Packets Attended Configuration\n\nRegister"]
    #[inline(always)]
    pub const fn fc_ctrlqlow(&self) -> &FcCtrlqlow {
        &self.fc_ctrlqlow
    }
    #[doc = "0x1075 - Frame Composer ACP Packet Type Configuration Register 0"]
    #[inline(always)]
    pub const fn fc_acp0(&self) -> &FcAcp0 {
        &self.fc_acp0
    }
    #[doc = "0x1082 - Frame Composer ACP Packet Body Configuration Register 16"]
    #[inline(always)]
    pub const fn fc_acp16(&self) -> &FcAcp16 {
        &self.fc_acp16
    }
    #[doc = "0x1083 - Frame Composer ACP Packet Body Configuration Register 15"]
    #[inline(always)]
    pub const fn fc_acp15(&self) -> &FcAcp15 {
        &self.fc_acp15
    }
    #[doc = "0x1084 - Frame Composer ACP Packet Body Configuration Register 14"]
    #[inline(always)]
    pub const fn fc_acp14(&self) -> &FcAcp14 {
        &self.fc_acp14
    }
    #[doc = "0x1085 - Frame Composer ACP Packet Body Configuration Register 13"]
    #[inline(always)]
    pub const fn fc_acp13(&self) -> &FcAcp13 {
        &self.fc_acp13
    }
    #[doc = "0x1086 - Frame Composer ACP Packet Body Configuration Register 12"]
    #[inline(always)]
    pub const fn fc_acp12(&self) -> &FcAcp12 {
        &self.fc_acp12
    }
    #[doc = "0x1087 - Frame Composer ACP Packet Body Configuration Register 11"]
    #[inline(always)]
    pub const fn fc_acp11(&self) -> &FcAcp11 {
        &self.fc_acp11
    }
    #[doc = "0x1088 - Frame Composer ACP Packet Body Configuration Register 10"]
    #[inline(always)]
    pub const fn fc_acp10(&self) -> &FcAcp10 {
        &self.fc_acp10
    }
    #[doc = "0x1089 - Frame Composer ACP Packet Body Configuration Register 9"]
    #[inline(always)]
    pub const fn fc_acp9(&self) -> &FcAcp9 {
        &self.fc_acp9
    }
    #[doc = "0x108a - Frame Composer ACP Packet Body Configuration Register 8"]
    #[inline(always)]
    pub const fn fc_acp8(&self) -> &FcAcp8 {
        &self.fc_acp8
    }
    #[doc = "0x108b - Frame Composer ACP Packet Body Configuration Register 7"]
    #[inline(always)]
    pub const fn fc_acp7(&self) -> &FcAcp7 {
        &self.fc_acp7
    }
    #[doc = "0x108c - Frame Composer ACP Packet Body Configuration Register 6"]
    #[inline(always)]
    pub const fn fc_acp6(&self) -> &FcAcp6 {
        &self.fc_acp6
    }
    #[doc = "0x108d - Frame Composer ACP Packet Body Configuration Register 5"]
    #[inline(always)]
    pub const fn fc_acp5(&self) -> &FcAcp5 {
        &self.fc_acp5
    }
    #[doc = "0x108e - Frame Composer ACP Packet Body Configuration Register 4"]
    #[inline(always)]
    pub const fn fc_acp4(&self) -> &FcAcp4 {
        &self.fc_acp4
    }
    #[doc = "0x108f - Frame Composer ACP Packet Body Configuration Register 3"]
    #[inline(always)]
    pub const fn fc_acp3(&self) -> &FcAcp3 {
        &self.fc_acp3
    }
    #[doc = "0x1090 - Frame Composer ACP Packet Body Configuration Register 2"]
    #[inline(always)]
    pub const fn fc_acp2(&self) -> &FcAcp2 {
        &self.fc_acp2
    }
    #[doc = "0x1091 - Frame Composer ACP Packet Body Configuration Register 1"]
    #[inline(always)]
    pub const fn fc_acp1(&self) -> &FcAcp1 {
        &self.fc_acp1
    }
    #[doc = "0x1092 - Frame Composer ISRC1 Packet Status, Valid, and Continue Configuration\n\nRegister"]
    #[inline(always)]
    pub const fn fc_iscr1_0(&self) -> &FcIscr1_0 {
        &self.fc_iscr1_0
    }
    #[doc = "0x1093 - Frame Composer ISRC1 Packet Body Register 16"]
    #[inline(always)]
    pub const fn fc_iscr1_16(&self) -> &FcIscr1_16 {
        &self.fc_iscr1_16
    }
    #[doc = "0x1094 - Frame Composer ISRC1 Packet Body Register 15"]
    #[inline(always)]
    pub const fn fc_iscr1_15(&self) -> &FcIscr1_15 {
        &self.fc_iscr1_15
    }
    #[doc = "0x1095 - Frame Composer ISRC1 Packet Body Register 14"]
    #[inline(always)]
    pub const fn fc_iscr1_14(&self) -> &FcIscr1_14 {
        &self.fc_iscr1_14
    }
    #[doc = "0x1096 - Frame Composer ISRC1 Packet Body Register 13"]
    #[inline(always)]
    pub const fn fc_iscr1_13(&self) -> &FcIscr1_13 {
        &self.fc_iscr1_13
    }
    #[doc = "0x1097 - Frame Composer ISRC1 Packet Body Register 12"]
    #[inline(always)]
    pub const fn fc_iscr1_12(&self) -> &FcIscr1_12 {
        &self.fc_iscr1_12
    }
    #[doc = "0x1098 - Frame Composer ISRC1 Packet Body Register 11"]
    #[inline(always)]
    pub const fn fc_iscr1_11(&self) -> &FcIscr1_11 {
        &self.fc_iscr1_11
    }
    #[doc = "0x1099 - Frame Composer ISRC1 Packet Body Register 10"]
    #[inline(always)]
    pub const fn fc_iscr1_10(&self) -> &FcIscr1_10 {
        &self.fc_iscr1_10
    }
    #[doc = "0x109a - Frame Composer ISRC1 Packet Body Register 9"]
    #[inline(always)]
    pub const fn fc_iscr1_9(&self) -> &FcIscr1_9 {
        &self.fc_iscr1_9
    }
    #[doc = "0x109b - Frame Composer ISRC1 Packet Body Register 8"]
    #[inline(always)]
    pub const fn fc_iscr1_8(&self) -> &FcIscr1_8 {
        &self.fc_iscr1_8
    }
    #[doc = "0x109c - Frame Composer ISRC1 Packet Body Register 7"]
    #[inline(always)]
    pub const fn fc_iscr1_7(&self) -> &FcIscr1_7 {
        &self.fc_iscr1_7
    }
    #[doc = "0x109d - Frame Composer ISRC1 Packet Body Register 6"]
    #[inline(always)]
    pub const fn fc_iscr1_6(&self) -> &FcIscr1_6 {
        &self.fc_iscr1_6
    }
    #[doc = "0x109e - Frame Composer ISRC1 Packet Body Register 5"]
    #[inline(always)]
    pub const fn fc_iscr1_5(&self) -> &FcIscr1_5 {
        &self.fc_iscr1_5
    }
    #[doc = "0x109f - Frame Composer ISRC1 Packet Body Register 4"]
    #[inline(always)]
    pub const fn fc_iscr1_4(&self) -> &FcIscr1_4 {
        &self.fc_iscr1_4
    }
    #[doc = "0x10a0 - Frame Composer ISRC1 Packet Body Register 3"]
    #[inline(always)]
    pub const fn fc_iscr1_3(&self) -> &FcIscr1_3 {
        &self.fc_iscr1_3
    }
    #[doc = "0x10a1 - Frame Composer ISRC1 Packet Body Register 2"]
    #[inline(always)]
    pub const fn fc_iscr1_2(&self) -> &FcIscr1_2 {
        &self.fc_iscr1_2
    }
    #[doc = "0x10a2 - Frame Composer ISRC1 Packet Body Register 1"]
    #[inline(always)]
    pub const fn fc_iscr1_1(&self) -> &FcIscr1_1 {
        &self.fc_iscr1_1
    }
    #[doc = "0x10a3 - Frame Composer ISRC2 Packet Body Register 15"]
    #[inline(always)]
    pub const fn fc_iscr2_15(&self) -> &FcIscr2_15 {
        &self.fc_iscr2_15
    }
    #[doc = "0x10a4 - Frame Composer ISRC2 Packet Body Register 14"]
    #[inline(always)]
    pub const fn fc_iscr2_14(&self) -> &FcIscr2_14 {
        &self.fc_iscr2_14
    }
    #[doc = "0x10a5 - Frame Composer ISRC2 Packet Body Register 13"]
    #[inline(always)]
    pub const fn fc_iscr2_13(&self) -> &FcIscr2_13 {
        &self.fc_iscr2_13
    }
    #[doc = "0x10a6 - Frame Composer ISRC2 Packet Body Register 12"]
    #[inline(always)]
    pub const fn fc_iscr2_12(&self) -> &FcIscr2_12 {
        &self.fc_iscr2_12
    }
    #[doc = "0x10a7 - Frame Composer ISRC2 Packet Body Register 11"]
    #[inline(always)]
    pub const fn fc_iscr2_11(&self) -> &FcIscr2_11 {
        &self.fc_iscr2_11
    }
    #[doc = "0x10a8 - Frame Composer ISRC2 Packet Body Register 10"]
    #[inline(always)]
    pub const fn fc_iscr2_10(&self) -> &FcIscr2_10 {
        &self.fc_iscr2_10
    }
    #[doc = "0x10a9 - Frame Composer ISRC2 Packet Body Register 9"]
    #[inline(always)]
    pub const fn fc_iscr2_9(&self) -> &FcIscr2_9 {
        &self.fc_iscr2_9
    }
    #[doc = "0x10aa - Frame Composer ISRC2 Packet Body Register 8"]
    #[inline(always)]
    pub const fn fc_iscr2_8(&self) -> &FcIscr2_8 {
        &self.fc_iscr2_8
    }
    #[doc = "0x10ab - Frame Composer ISRC2 Packet Body Register 7"]
    #[inline(always)]
    pub const fn fc_iscr2_7(&self) -> &FcIscr2_7 {
        &self.fc_iscr2_7
    }
    #[doc = "0x10ac - Frame Composer ISRC2 Packet Body Register 6"]
    #[inline(always)]
    pub const fn fc_iscr2_6(&self) -> &FcIscr2_6 {
        &self.fc_iscr2_6
    }
    #[doc = "0x10ad - Frame Composer ISRC2 Packet Body Register 5"]
    #[inline(always)]
    pub const fn fc_iscr2_5(&self) -> &FcIscr2_5 {
        &self.fc_iscr2_5
    }
    #[doc = "0x10ae - Frame Composer ISRC2 Packet Body Register 4"]
    #[inline(always)]
    pub const fn fc_iscr2_4(&self) -> &FcIscr2_4 {
        &self.fc_iscr2_4
    }
    #[doc = "0x10af - Frame Composer ISRC2 Packet Body Register 3"]
    #[inline(always)]
    pub const fn fc_iscr2_3(&self) -> &FcIscr2_3 {
        &self.fc_iscr2_3
    }
    #[doc = "0x10b0 - Frame Composer ISRC2 Packet Body Register 2"]
    #[inline(always)]
    pub const fn fc_iscr2_2(&self) -> &FcIscr2_2 {
        &self.fc_iscr2_2
    }
    #[doc = "0x10b1 - Frame Composer ISRC2 Packet Body Register 1"]
    #[inline(always)]
    pub const fn fc_iscr2_1(&self) -> &FcIscr2_1 {
        &self.fc_iscr2_1
    }
    #[doc = "0x10b2 - Frame Composer ISRC2 Packet Body Register 0"]
    #[inline(always)]
    pub const fn fc_iscr2_0(&self) -> &FcIscr2_0 {
        &self.fc_iscr2_0
    }
    #[doc = "0x10b3 - Frame Composer Data Island Auto Packet Scheduling Register 0\n\nConfigures the Frame Composer RDRB(1)/Manual(0) data island packet insertion for SPD,\n\nVSD, ISRC2, ISRC1 and ACP packets. On RDRB mode the described packet scheduling is\n\ncontrolled by registers FC_DATAUTO1 and FC_DATAUTO2, while in Manual mode register\n\nFC_DATMAN requests to FC the insertion of the requested packet."]
    #[inline(always)]
    pub const fn fc_datauto0(&self) -> &FcDatauto0 {
        &self.fc_datauto0
    }
    #[doc = "0x10b4 - Frame Composer Data Island Auto Packet Scheduling Register 1\n\nConfigures the Frame Composer (FC) RDRB frame interpolation for SPD, VSD, ISRC2,\n\nISRC1 and ACP packet insertion on data island when FC is on RDRB mode for the listed\n\npackets."]
    #[inline(always)]
    pub const fn fc_datauto1(&self) -> &FcDatauto1 {
        &self.fc_datauto1
    }
    #[doc = "0x10b5 - Frame Composer Data Island Auto packet scheduling Register 2\n\nConfigures the Frame Composer (FC) RDRB line interpolation and number of packets in\n\nframe for SPD, VSD, ISRC2, ISRC1 and ACP packet insertion on data island when FC is on\n\nRDRB mode for the listed packets."]
    #[inline(always)]
    pub const fn fc_datauto2(&self) -> &FcDatauto2 {
        &self.fc_datauto2
    }
    #[doc = "0x10b6 - Frame Composer Data Island Manual Packet Request Register\n\nRequests to the Frame Composer the data island packet insertion for NULL, SPD, VSD,\n\nISRC2, ISRC1 and ACP packets when FC_DATAUTO0 bit is in manual mode for the packet\n\nrequested."]
    #[inline(always)]
    pub const fn fc_datman(&self) -> &FcDatman {
        &self.fc_datman
    }
    #[doc = "0x10b7 - Frame Composer Data Island Auto Packet Scheduling Register 3\n\nConfigures the Frame Composer Automatic(1)/RDRB(0) data island packet insertion for\n\nAVI, GCP, AUDI and ACR packets. In Automatic mode, the packet is inserted on Vblanking\n\nwhen first line with active Vsync appears."]
    #[inline(always)]
    pub const fn fc_datauto3(&self) -> &FcDatauto3 {
        &self.fc_datauto3
    }
    #[doc = "0x10b8 - Frame Composer Round Robin ACR Packet Insertion Register 0\n\nConfigures the Frame Composer (FC) RDRB frame interpolation for ACR packet insertion on\n\ndata island when FC is on RDRB mode for this packet."]
    #[inline(always)]
    pub const fn fc_rdrb0(&self) -> &FcRdrb0 {
        &self.fc_rdrb0
    }
    #[doc = "0x10b9 - Frame Composer Round Robin ACR Packet Insertion Register 1\n\nConfigures the Frame Composer (FC) RDRB line interpolation and number of packets in\n\nframe for the ACR packet insertion on data island when FC is on RDRB mode this packet."]
    #[inline(always)]
    pub const fn fc_rdrb1(&self) -> &FcRdrb1 {
        &self.fc_rdrb1
    }
    #[doc = "0x10ba - Frame Composer Round Robin AUDI Packet Insertion Register 2\n\nConfigures the Frame Composer (FC) RDRB frame interpolation for AUDI packet insertion\n\non data island when FC is on RDRB mode for this packet."]
    #[inline(always)]
    pub const fn fc_rdrb2(&self) -> &FcRdrb2 {
        &self.fc_rdrb2
    }
    #[doc = "0x10bb - Frame Composer Round Robin AUDI Packet Insertion Register 3\n\nConfigures the Frame Composer (FC) RDRB line interpolation and number of packets in\n\nframe for the AUDI packet insertion on data island when FC is on RDRB mode this packet."]
    #[inline(always)]
    pub const fn fc_rdrb3(&self) -> &FcRdrb3 {
        &self.fc_rdrb3
    }
    #[doc = "0x10bc - Frame Composer Round Robin GCP Packet Insertion Register 4\n\nConfigures the Frame Composer (FC) RDRB frame interpolation for GCP packet insertion on\n\ndata island when FC is on RDRB mode for this packet."]
    #[inline(always)]
    pub const fn fc_rdrb4(&self) -> &FcRdrb4 {
        &self.fc_rdrb4
    }
    #[doc = "0x10bd - Frame Composer Round Robin GCP Packet Insertion Register 5\n\nConfigures the Frame Composer (FC) RDRB line interpolation and number of packets in\n\nframe for the GCP packet insertion on data island when FC is on RDRB mode this packet."]
    #[inline(always)]
    pub const fn fc_rdrb5(&self) -> &FcRdrb5 {
        &self.fc_rdrb5
    }
    #[doc = "0x10be - Frame Composer Round Robin AVI Packet Insertion Register 6\n\nConfigures the Frame Composer (FC) RDRB frame interpolation for AVI packet insertion on\n\ndata island when FC is on RDRB mode for this packet."]
    #[inline(always)]
    pub const fn fc_rdrb6(&self) -> &FcRdrb6 {
        &self.fc_rdrb6
    }
    #[doc = "0x10bf - Frame Composer Round Robin AVI Packet Insertion Register 7\n\nConfigures the Frame Composer (FC) RDRB line interpolation and number of packets in\n\nframe for the AVI packet insertion on data island when FC is on RDRB mode this packet."]
    #[inline(always)]
    pub const fn fc_rdrb7(&self) -> &FcRdrb7 {
        &self.fc_rdrb7
    }
    #[doc = "0x10c0 - Frame Composer Round Robin AMP Packet Insertion Register 8"]
    #[inline(always)]
    pub const fn fc_rdrb8(&self) -> &FcRdrb8 {
        &self.fc_rdrb8
    }
    #[doc = "0x10c1 - Frame Composer Round Robin AMP Packet Insertion Register 9"]
    #[inline(always)]
    pub const fn fc_rdrb9(&self) -> &FcRdrb9 {
        &self.fc_rdrb9
    }
    #[doc = "0x10c2 - Frame Composer Round Robin NTSC VBI Packet Insertion Register 10"]
    #[inline(always)]
    pub const fn fc_rdrb10(&self) -> &FcRdrb10 {
        &self.fc_rdrb10
    }
    #[doc = "0x10c3 - Frame Composer Round Robin NTSC VBI Packet Insertion Register 11"]
    #[inline(always)]
    pub const fn fc_rdrb11(&self) -> &FcRdrb11 {
        &self.fc_rdrb11
    }
    #[doc = "0x10c4 - Frame Composer Round Robin DRM Packet Insertion Register 12"]
    #[inline(always)]
    pub const fn fc_rdrb12(&self) -> &FcRdrb12 {
        &self.fc_rdrb12
    }
    #[doc = "0x10c5 - Frame Composer Round Robin DRM Packet Insertion Register 13"]
    #[inline(always)]
    pub const fn fc_rdrb13(&self) -> &FcRdrb13 {
        &self.fc_rdrb13
    }
    #[doc = "0x10d2 - Frame Composer Packet Interrupt Mask Register 0"]
    #[inline(always)]
    pub const fn fc_mask0(&self) -> &FcMask0 {
        &self.fc_mask0
    }
    #[doc = "0x10d6 - Frame Composer Packet Interrupt Mask Register 1"]
    #[inline(always)]
    pub const fn fc_mask1(&self) -> &FcMask1 {
        &self.fc_mask1
    }
    #[doc = "0x10da - Frame Composer High/Low Priority Overflow and DRM Interrupt Mask Register\n\n2"]
    #[inline(always)]
    pub const fn fc_mask2(&self) -> &FcMask2 {
        &self.fc_mask2
    }
    #[doc = "0x10e0 - Frame Composer Pixel Repetition Configuration Register"]
    #[inline(always)]
    pub const fn fc_prconf(&self) -> &FcPrconf {
        &self.fc_prconf
    }
    #[doc = "0x10e1 - Frame Composer Scrambler Control"]
    #[inline(always)]
    pub const fn fc_scrambler_ctrl(&self) -> &FcScramblerCtrl {
        &self.fc_scrambler_ctrl
    }
    #[doc = "0x10e2 - Frame Composer Multi-Stream Audio Control"]
    #[inline(always)]
    pub const fn fc_multistream_ctrl(&self) -> &FcMultistreamCtrl {
        &self.fc_multistream_ctrl
    }
    #[doc = "0x10e3 - Frame Composer Packet Transmission Control"]
    #[inline(always)]
    pub const fn fc_packet_tx_en(&self) -> &FcPacketTxEn {
        &self.fc_packet_tx_en
    }
    #[doc = "0x10e8 - Frame Composer Active Space Control"]
    #[inline(always)]
    pub const fn fc_actspc_hdlr_cfg(&self) -> &FcActspcHdlrCfg {
        &self.fc_actspc_hdlr_cfg
    }
    #[doc = "0x10e9 - Frame Composer Input Video 2D VActive Pixels Register 0"]
    #[inline(always)]
    pub const fn fc_invact_2d_0(&self) -> &FcInvact2d0 {
        &self.fc_invact_2d_0
    }
    #[doc = "0x10ea - Frame Composer Input Video VActive pixels Register 1"]
    #[inline(always)]
    pub const fn fc_invact_2d_1(&self) -> &FcInvact2d1 {
        &self.fc_invact_2d_1
    }
    #[doc = "0x1100 - Frame Composer GMD Packet Status Register\n\nGamut metadata packet status bit information for no_current_gmd, next_gmd_field,\n\ngmd_packet_sequence and current_gamut_seq_num. For more information, refer to the\n\nHDMI 1.4b specification."]
    #[inline(always)]
    pub const fn fc_gmd_stat(&self) -> &FcGmdStat {
        &self.fc_gmd_stat
    }
    #[doc = "0x1101 - Frame Composer GMD Packet Enable Register\n\nThis register enables Gamut metadata (GMD) packet transmission. Packets are inserted in\n\nthe incoming frame, starting in the line where active Vsync indication starts. After enable of\n\nGMD packets the outgoing packet is sent with no_current_gmd active indication until\n\nupdate GMD request is performed in the controller."]
    #[inline(always)]
    pub const fn fc_gmd_en(&self) -> &FcGmdEn {
        &self.fc_gmd_en
    }
    #[doc = "0x1102 - Frame Composer GMD Packet Update Register\n\nThis register performs an GMD packet content update according to the configured packet\n\nbody (FC_GMD_PB0 to FC_GMD_PB27) and packet header (FC_GMD_HB). This active high\n\nauto clear register reflects the body and header configurations on the GMD packets sent\n\narbitrating the current_gamut_seq_num, gmd_packet_sequence and next_gmd_field bits\n\non packet to correctly indicate to source the Gamut change to be performed. After enable\n\nGMD packets the first update request is also responsible for deactivating the\n\nno_current_gmd indication bit.\n\nAttention packet update request must only be done after correct configuration of GMD\n\npacket body and header registers. Correct affected_gamut_seq_num and gmd_profile\n\nconfiguration is user responsibility and must convey with HDMI 1.4b standard gamut rules."]
    #[inline(always)]
    pub const fn fc_gmd_up(&self) -> &FcGmdUp {
        &self.fc_gmd_up
    }
    #[doc = "0x1103 - Frame Composer GMD Packet Schedule Configuration Register\n\nThis register configures the number of GMD packets to be inserted per frame (starting\n\nalways in the line where the active Vsync appears) and the line spacing between the\n\ntransmitted GMD packets.\n\nNote that for profile P0 (refer to the HDMI 1.4b specification) this register should only\n\nindicate one GMD packet to be inserted per video field."]
    #[inline(always)]
    pub const fn fc_gmd_conf(&self) -> &FcGmdConf {
        &self.fc_gmd_conf
    }
    #[doc = "0x1104 - Frame Composer GMD Packet Profile and Gamut Sequence Configuration\n\nRegister\n\nThis register configures the GMD packet header affected_gamut_seq_num and gmd_profile\n\nbits. For more information, refer to the HDMI 1.4b specification."]
    #[inline(always)]
    pub const fn fc_gmd_hb(&self) -> &FcGmdHb {
        &self.fc_gmd_hb
    }
    #[doc = "0x1105..0x1121 - Frame Composer GMD Packet Body Register Array Configures the GMD packet\n\nbody of the GMD packet."]
    #[inline(always)]
    pub const fn fc_gmd_pb(&self, n: usize) -> &FcGmdPb {
        &self.fc_gmd_pb[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1105..0x1121 - Frame Composer GMD Packet Body Register Array Configures the GMD packet\n\nbody of the GMD packet."]
    #[inline(always)]
    pub fn fc_gmd_pb_iter(&self) -> impl Iterator<Item = &FcGmdPb> {
        self.fc_gmd_pb.iter()
    }
    #[doc = "0x1128 - Frame Composer AMP Packet Header Register 1"]
    #[inline(always)]
    pub const fn fc_amp_hb1(&self) -> &FcAmpHb1 {
        &self.fc_amp_hb1
    }
    #[doc = "0x1129 - Frame Composer AMP Packet Header Register 2"]
    #[inline(always)]
    pub const fn fc_amp_hb2(&self) -> &FcAmpHb2 {
        &self.fc_amp_hb2
    }
    #[doc = "0x112a..0x1146 - Frame Composer AMP Packet Body Register Array"]
    #[inline(always)]
    pub const fn fc_amp_pb(&self, n: usize) -> &FcAmpPb {
        &self.fc_amp_pb[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x112a..0x1146 - Frame Composer AMP Packet Body Register Array"]
    #[inline(always)]
    pub fn fc_amp_pb_iter(&self) -> impl Iterator<Item = &FcAmpPb> {
        self.fc_amp_pb.iter()
    }
    #[doc = "0x1148 - Frame Composer NTSC VBI Packet Header Register 1"]
    #[inline(always)]
    pub const fn fc_nvbi_hb1(&self) -> &FcNvbiHb1 {
        &self.fc_nvbi_hb1
    }
    #[doc = "0x1149 - Frame Composer NTSC VBI Packet Header Register 2"]
    #[inline(always)]
    pub const fn fc_nvbi_hb2(&self) -> &FcNvbiHb2 {
        &self.fc_nvbi_hb2
    }
    #[doc = "0x114a..0x1165 - Frame Composer NTSC VBI Packet Body Register Array"]
    #[inline(always)]
    pub const fn fc_nvbi_pb(&self, n: usize) -> &FcNvbiPb {
        &self.fc_nvbi_pb[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x114a..0x1165 - Frame Composer NTSC VBI Packet Body Register Array"]
    #[inline(always)]
    pub fn fc_nvbi_pb_iter(&self) -> impl Iterator<Item = &FcNvbiPb> {
        self.fc_nvbi_pb.iter()
    }
    #[doc = "0x1167 - Frame Composer DRM Packet Update Register\n\nThis register performs an DRM packet content update according to the configured packet\n\nbody (FC_DRM_PB0 to FC_DRM_PB27) and packet header (FC_DRM_HB). This active high\n\nauto clear register reflects the body and header configurations on the DRM packets change\n\nto be performed.\n\nAttention packet update request must only be done after correct configuration of DRM\n\npacket body and header registers."]
    #[inline(always)]
    pub const fn fc_drm_up(&self) -> &FcDrmUp {
        &self.fc_drm_up
    }
    #[doc = "0x1168 - Frame Composer DRM Packet Header Register Array"]
    #[inline(always)]
    pub const fn fc_drm_hb(&self, n: usize) -> &FcDrmHb {
        &self.fc_drm_hb[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1168 - Frame Composer DRM Packet Header Register Array"]
    #[inline(always)]
    pub fn fc_drm_hb_iter(&self) -> impl Iterator<Item = &FcDrmHb> {
        self.fc_drm_hb.iter()
    }
    #[doc = "0x116a..0x1185 - Frame Composer DRM Packet Body Register Array"]
    #[inline(always)]
    pub const fn fc_drm_pb(&self, n: usize) -> &FcDrmPb {
        &self.fc_drm_pb[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x116a..0x1185 - Frame Composer DRM Packet Body Register Array"]
    #[inline(always)]
    pub fn fc_drm_pb_iter(&self) -> impl Iterator<Item = &FcDrmPb> {
        self.fc_drm_pb.iter()
    }
    #[doc = "0x1200 - Frame Composer video/audio Force Enable Register\n\nThis register allows to force the controller to output audio and video data the values\n\nconfigured in the FC_DBGAUD and FC_DBGTMDS registers."]
    #[inline(always)]
    pub const fn fc_dbgforce(&self) -> &FcDbgforce {
        &self.fc_dbgforce
    }
    #[doc = "0x1201 - Frame Composer Audio Data Channel 0 Register 0\n\nConfigures the audio fixed data to be used in channel 0 when in fixed audio selection."]
    #[inline(always)]
    pub const fn fc_dbgaud0ch0(&self) -> &FcDbgaud0ch0 {
        &self.fc_dbgaud0ch0
    }
    #[doc = "0x1202 - Frame Composer Audio Data Channel 0 Register 1\n\nConfigures the audio fixed data to be used in channel 0 when in fixed audio selection."]
    #[inline(always)]
    pub const fn fc_dbgaud1ch0(&self) -> &FcDbgaud1ch0 {
        &self.fc_dbgaud1ch0
    }
    #[doc = "0x1203 - Frame Composer Audio Data Channel 0 Register 2\n\nConfigures the audio fixed data to be used in channel 0 when in fixed audio selection."]
    #[inline(always)]
    pub const fn fc_dbgaud2ch0(&self) -> &FcDbgaud2ch0 {
        &self.fc_dbgaud2ch0
    }
    #[doc = "0x1204 - Frame Composer Audio Data Channel 1 Register 0\n\nConfigures the audio fixed data to be used in channel 1 when in fixed audio selection."]
    #[inline(always)]
    pub const fn fc_dbgaud0ch1(&self) -> &FcDbgaud0ch1 {
        &self.fc_dbgaud0ch1
    }
    #[doc = "0x1205 - Frame Composer Audio Data Channel 1 Register 1\n\nConfigures the audio fixed data to be used in channel 1 when in fixed audio selection."]
    #[inline(always)]
    pub const fn fc_dbgaud1ch1(&self) -> &FcDbgaud1ch1 {
        &self.fc_dbgaud1ch1
    }
    #[doc = "0x1206 - Frame Composer Audio Data Channel 1 Register 2\n\nConfigures the audio fixed data to be used in channel 1 when in fixed audio selection."]
    #[inline(always)]
    pub const fn fc_dbgaud2ch1(&self) -> &FcDbgaud2ch1 {
        &self.fc_dbgaud2ch1
    }
    #[doc = "0x1207 - Frame Composer Audio Data Channel 2 Register 0\n\nConfigures the audio fixed data to be used in channel 2 when in fixed audio selection."]
    #[inline(always)]
    pub const fn fc_dbgaud0ch2(&self) -> &FcDbgaud0ch2 {
        &self.fc_dbgaud0ch2
    }
    #[doc = "0x1208 - Frame Composer Audio Data Channel 2 Register 1\n\nConfigures the audio fixed data to be used in channel 2 when in fixed audio selection."]
    #[inline(always)]
    pub const fn fc_dbgaud1ch2(&self) -> &FcDbgaud1ch2 {
        &self.fc_dbgaud1ch2
    }
    #[doc = "0x1209 - Frame Composer Audio Data Channel 2 Register 2\n\nConfigures the audio fixed data to be used in channel 2 when in fixed audio selection."]
    #[inline(always)]
    pub const fn fc_dbgaud2ch2(&self) -> &FcDbgaud2ch2 {
        &self.fc_dbgaud2ch2
    }
    #[doc = "0x120a - Frame Composer Audio Data Channel 3 Register 0\n\nConfigures the audio fixed data to be used in channel 3 when in fixed audio selection."]
    #[inline(always)]
    pub const fn fc_dbgaud0ch3(&self) -> &FcDbgaud0ch3 {
        &self.fc_dbgaud0ch3
    }
    #[doc = "0x120b - Frame Composer Audio Data Channel 3 Register 1\n\nConfigures the audio fixed data to be used in channel 3 when in fixed audio selection."]
    #[inline(always)]
    pub const fn fc_dbgaud1ch3(&self) -> &FcDbgaud1ch3 {
        &self.fc_dbgaud1ch3
    }
    #[doc = "0x120c - Frame Composer Audio Data Channel 3 Register 2\n\nConfigures the audio fixed data to be used in channel 3 when in fixed audio selection."]
    #[inline(always)]
    pub const fn fc_dbgaud2ch3(&self) -> &FcDbgaud2ch3 {
        &self.fc_dbgaud2ch3
    }
    #[doc = "0x120d - Frame Composer Audio Data Channel 4 Register 0\n\nConfigures the audio fixed data to be used in channel 4 when in fixed audio selection."]
    #[inline(always)]
    pub const fn fc_dbgaud0ch4(&self) -> &FcDbgaud0ch4 {
        &self.fc_dbgaud0ch4
    }
    #[doc = "0x120e - Frame Composer Audio Data Channel 4 Register 1\n\nConfigures the audio fixed data to be used in channel 4 when in fixed audio selection."]
    #[inline(always)]
    pub const fn fc_dbgaud1ch4(&self) -> &FcDbgaud1ch4 {
        &self.fc_dbgaud1ch4
    }
    #[doc = "0x120f - Frame Composer Audio Data Channel 4 Register 2\n\nConfigures the audio fixed data to be used in channel 4 when in fixed audio selection."]
    #[inline(always)]
    pub const fn fc_dbgaud2ch4(&self) -> &FcDbgaud2ch4 {
        &self.fc_dbgaud2ch4
    }
    #[doc = "0x1210 - Frame Composer Audio Data Channel 5 Register 0\n\nConfigures the audio fixed data to be used in channel 5 when in fixed audio selection."]
    #[inline(always)]
    pub const fn fc_dbgaud0ch5(&self) -> &FcDbgaud0ch5 {
        &self.fc_dbgaud0ch5
    }
    #[doc = "0x1211 - Frame Composer Audio Data Channel 5 Register 1\n\nConfigures the audio fixed data to be used in channel 5 when in fixed audio selection."]
    #[inline(always)]
    pub const fn fc_dbgaud1ch5(&self) -> &FcDbgaud1ch5 {
        &self.fc_dbgaud1ch5
    }
    #[doc = "0x1212 - Frame Composer Audio Data Channel 5 Register 2\n\nConfigures the audio fixed data to be used in channel 5 when in fixed audio selection."]
    #[inline(always)]
    pub const fn fc_dbgaud2ch5(&self) -> &FcDbgaud2ch5 {
        &self.fc_dbgaud2ch5
    }
    #[doc = "0x1213 - Frame Composer Audio Data Channel 6 Register 0\n\nConfigures the audio fixed data to be used in channel 6 when in fixed audio selection."]
    #[inline(always)]
    pub const fn fc_dbgaud0ch6(&self) -> &FcDbgaud0ch6 {
        &self.fc_dbgaud0ch6
    }
    #[doc = "0x1214 - Frame Composer Audio Data Channel 6 Register 1\n\nConfigures the audio fixed data to be used in channel 6 when in fixed audio selection."]
    #[inline(always)]
    pub const fn fc_dbgaud1ch6(&self) -> &FcDbgaud1ch6 {
        &self.fc_dbgaud1ch6
    }
    #[doc = "0x1215 - Frame Composer Audio Data Channel 6 Register 2\n\nConfigures the audio fixed data to be used in channel 6 when in fixed audio selection."]
    #[inline(always)]
    pub const fn fc_dbgaud2ch6(&self) -> &FcDbgaud2ch6 {
        &self.fc_dbgaud2ch6
    }
    #[doc = "0x1216 - Frame Composer Audio Data Channel 7 Register 0\n\nConfigures the audio fixed data to be used in channel 7 when in fixed audio selection."]
    #[inline(always)]
    pub const fn fc_dbgaud0ch7(&self) -> &FcDbgaud0ch7 {
        &self.fc_dbgaud0ch7
    }
    #[doc = "0x1217 - Frame Composer Audio Data Channel 7 Register 1\n\nConfigures the audio fixed data to be used in channel 7 when in fixed audio selection."]
    #[inline(always)]
    pub const fn fc_dbgaud1ch7(&self) -> &FcDbgaud1ch7 {
        &self.fc_dbgaud1ch7
    }
    #[doc = "0x1218 - Frame Composer Audio Data Channel 7 Register 2\n\nConfigures the audio fixed data to be used in channel 7 when in fixed audio selection."]
    #[inline(always)]
    pub const fn fc_dbgaud2ch7(&self) -> &FcDbgaud2ch7 {
        &self.fc_dbgaud2ch7
    }
    #[doc = "0x1219 - Frame Composer TMDS Data Channel Register Array\n\nConfigures the video fixed data to be used in TMDS channel x (where x is 0 to 2) when in\n\nfixed video selection.\n\nFor Channel 0, this equals to set B pixel component value in RGB video or Cb pixel\n\ncomponent value in YCbCr.\n\nFor Channel 1, this equals set G pixel component value in RGB video or Y pixel component\n\nvalue in YCbCr.\n\nFor Channel 2, this equals to set R pixel component value in RGB video or Cr pixel\n\ncomponent value in YCbCr."]
    #[inline(always)]
    pub const fn fc_dbgtmds(&self, n: usize) -> &FcDbgtmds {
        &self.fc_dbgtmds[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1219 - Frame Composer TMDS Data Channel Register Array\n\nConfigures the video fixed data to be used in TMDS channel x (where x is 0 to 2) when in\n\nfixed video selection.\n\nFor Channel 0, this equals to set B pixel component value in RGB video or Cb pixel\n\ncomponent value in YCbCr.\n\nFor Channel 1, this equals set G pixel component value in RGB video or Y pixel component\n\nvalue in YCbCr.\n\nFor Channel 2, this equals to set R pixel component value in RGB video or Cr pixel\n\ncomponent value in YCbCr."]
    #[inline(always)]
    pub fn fc_dbgtmds_iter(&self) -> impl Iterator<Item = &FcDbgtmds> {
        self.fc_dbgtmds.iter()
    }
    #[doc = "0x3000 - PHY Configuration Register\n\nThis register holds the power down, data enable polarity, and interface control of the HDMI\n\nSource PHY control."]
    #[inline(always)]
    pub const fn phy_conf0(&self) -> &PhyConf0 {
        &self.phy_conf0
    }
    #[doc = "0x3001 - PHY Test Interface Register 0\n\nPHY TX mapped test interface (control)."]
    #[inline(always)]
    pub const fn phy_tst0(&self) -> &PhyTst0 {
        &self.phy_tst0
    }
    #[doc = "0x3002 - PHY Test Interface Register 1\n\nPHY TX mapped text interface (data in)."]
    #[inline(always)]
    pub const fn phy_tst1(&self) -> &PhyTst1 {
        &self.phy_tst1
    }
    #[doc = "0x3003 - PHY Test Interface Register 2\n\nPHY TX mapped text interface (data out)."]
    #[inline(always)]
    pub const fn phy_tst2(&self) -> &PhyTst2 {
        &self.phy_tst2
    }
    #[doc = "0x3004 - PHY RXSENSE, PLL Lock, and HPD Status Register\n\nThis register contains the following active high packet sent status indications."]
    #[inline(always)]
    pub const fn phy_stat0(&self) -> &PhyStat0 {
        &self.phy_stat0
    }
    #[doc = "0x3005 - PHY RXSENSE, PLL Lock, and HPD Interrupt Register\n\nThis register contains the interrupt indication of the PHY_STAT0 status interrupts. Interrupt\n\ngeneration is accomplished in the following way:\n\ninterrupt = (mask == 1'b0) &amp;&amp; (polarity == status);\n\nAll these interrupts are forwarded to the Interrupt Handler sticky bit register ih_phy_stat0\n\nand after ORed to a single main interrupt line to micro controller. Assertion of this interrupt\n\nimplies that data related with the corresponding packet has been sent through the HDMI\n\ninterface."]
    #[inline(always)]
    pub const fn phy_int0(&self) -> &PhyInt0 {
        &self.phy_int0
    }
    #[doc = "0x3006 - PHY RXSENSE, PLL Lock, and HPD Mask Register Mask register for generation\n\nof PHY_INT0 interrupts."]
    #[inline(always)]
    pub const fn phy_mask0(&self) -> &PhyMask0 {
        &self.phy_mask0
    }
    #[doc = "0x3007 - PHY RXSENSE, PLL Lock, and HPD Polarity Register Polarity register for\n\ngeneration of PHY_INT0 interrupts."]
    #[inline(always)]
    pub const fn phy_pol0(&self) -> &PhyPol0 {
        &self.phy_pol0
    }
    #[doc = "0x3008 - PHY Test Interface Register 0"]
    #[inline(always)]
    pub const fn phy_pclfreq0(&self) -> &PhyPclfreq0 {
        &self.phy_pclfreq0
    }
    #[doc = "0x3009 - PHY Test Interface Register 1"]
    #[inline(always)]
    pub const fn phy_pclfreq1(&self) -> &PhyPclfreq1 {
        &self.phy_pclfreq1
    }
    #[doc = "0x300a - PHY PLL Test Interface Register 0"]
    #[inline(always)]
    pub const fn phy_pllcfgfreq0(&self) -> &PhyPllcfgfreq0 {
        &self.phy_pllcfgfreq0
    }
    #[doc = "0x300b - PHY PLL Test Interface Register 1"]
    #[inline(always)]
    pub const fn phy_pllcfgfreq1(&self) -> &PhyPllcfgfreq1 {
        &self.phy_pllcfgfreq1
    }
    #[doc = "0x300c - PHY PLL Test Interface Register 2"]
    #[inline(always)]
    pub const fn phy_pllcfgfreq2(&self) -> &PhyPllcfgfreq2 {
        &self.phy_pllcfgfreq2
    }
    #[doc = "0x3020 - PHY I2C Slave Address Configuration Register"]
    #[inline(always)]
    pub const fn phy_i2cm_slave(&self) -> &PhyI2cmSlave {
        &self.phy_i2cm_slave
    }
    #[doc = "0x3021 - PHY I2C Address Configuration Register\n\nThis register writes the address for read and write operations."]
    #[inline(always)]
    pub const fn phy_i2cm_address(&self) -> &PhyI2cmAddress {
        &self.phy_i2cm_address
    }
    #[doc = "0x3022 - PHY I2C Data Write Register 1"]
    #[inline(always)]
    pub const fn phy_i2cm_datao_1(&self) -> &PhyI2cmDatao1 {
        &self.phy_i2cm_datao_1
    }
    #[doc = "0x3023 - PHY I2C Data Write Register 0"]
    #[inline(always)]
    pub const fn phy_i2cm_datao_0(&self) -> &PhyI2cmDatao0 {
        &self.phy_i2cm_datao_0
    }
    #[doc = "0x3024 - PHY I2C Data Read Register 1"]
    #[inline(always)]
    pub const fn phy_i2cm_datai_1(&self) -> &PhyI2cmDatai1 {
        &self.phy_i2cm_datai_1
    }
    #[doc = "0x3025 - PHY I2C Data Read Register 0"]
    #[inline(always)]
    pub const fn phy_i2cm_datai_0(&self) -> &PhyI2cmDatai0 {
        &self.phy_i2cm_datai_0
    }
    #[doc = "0x3026 - PHY I2C RD/RD_EXT/WR Operation Register\n\nThis register requests read and write operations from the I2C Master PHY. This register can\n\nonly be written; reading this register always results in 00h. Writing 1'b1 simultaneously to\n\nread and write requests is considered a read request."]
    #[inline(always)]
    pub const fn phy_i2cm_operation(&self) -> &PhyI2cmOperation {
        &self.phy_i2cm_operation
    }
    #[doc = "0x3027 - PHY I2C Done Interrupt Register\n\nThis register contains and configures I2C master PHY done interrupt."]
    #[inline(always)]
    pub const fn phy_i2cm_int(&self) -> &PhyI2cmInt {
        &self.phy_i2cm_int
    }
    #[doc = "0x3028 - PHY I2C error Interrupt Register\n\nThis register contains and configures the I2C master PHY error interrupts."]
    #[inline(always)]
    pub const fn phy_i2cm_ctlint(&self) -> &PhyI2cmCtlint {
        &self.phy_i2cm_ctlint
    }
    #[doc = "0x3029 - PHY I2C Speed control Register\n\nThis register wets the I2C Master PHY to work in either Fast or Standard mode."]
    #[inline(always)]
    pub const fn phy_i2cm_div(&self) -> &PhyI2cmDiv {
        &self.phy_i2cm_div
    }
    #[doc = "0x302a - PHY I2C SW reset control register\n\nThis register sets the I2C Master PHY software reset."]
    #[inline(always)]
    pub const fn phy_i2cm_softrstz(&self) -> &PhyI2cmSoftrstz {
        &self.phy_i2cm_softrstz
    }
    #[doc = "0x302b - PHY I2C Slow Speed SCL High Level Control Register 1"]
    #[inline(always)]
    pub const fn phy_i2cm_ss_scl_hcnt_1_addr(&self) -> &PhyI2cmSsSclHcnt1Addr {
        &self.phy_i2cm_ss_scl_hcnt_1_addr
    }
    #[doc = "0x302c - PHY I2C Slow Speed SCL High Level Control Register 0"]
    #[inline(always)]
    pub const fn phy_i2cm_ss_scl_hcnt_0_addr(&self) -> &PhyI2cmSsSclHcnt0Addr {
        &self.phy_i2cm_ss_scl_hcnt_0_addr
    }
    #[doc = "0x302d - PHY I2C Slow Speed SCL Low Level Control Register 1"]
    #[inline(always)]
    pub const fn phy_i2cm_ss_scl_lcnt_1_addr(&self) -> &PhyI2cmSsSclLcnt1Addr {
        &self.phy_i2cm_ss_scl_lcnt_1_addr
    }
    #[doc = "0x302e - PHY I2C Slow Speed SCL Low Level Control Register 0"]
    #[inline(always)]
    pub const fn phy_i2cm_ss_scl_lcnt_0_addr(&self) -> &PhyI2cmSsSclLcnt0Addr {
        &self.phy_i2cm_ss_scl_lcnt_0_addr
    }
    #[doc = "0x302f - PHY I2C Fast Speed SCL High Level Control Register 1"]
    #[inline(always)]
    pub const fn phy_i2cm_fs_scl_hcnt_1_addr(&self) -> &PhyI2cmFsSclHcnt1Addr {
        &self.phy_i2cm_fs_scl_hcnt_1_addr
    }
    #[doc = "0x3030 - PHY I2C Fast Speed SCL High Level Control Register 0"]
    #[inline(always)]
    pub const fn phy_i2cm_fs_scl_hcnt_0_addr(&self) -> &PhyI2cmFsSclHcnt0Addr {
        &self.phy_i2cm_fs_scl_hcnt_0_addr
    }
    #[doc = "0x3031 - PHY I2C Fast Speed SCL Low Level Control Register 1"]
    #[inline(always)]
    pub const fn phy_i2cm_fs_scl_lcnt_1_addr(&self) -> &PhyI2cmFsSclLcnt1Addr {
        &self.phy_i2cm_fs_scl_lcnt_1_addr
    }
    #[doc = "0x3032 - PHY I2C Fast Speed SCL Low Level Control Register 0"]
    #[inline(always)]
    pub const fn phy_i2cm_fs_scl_lcnt_0_addr(&self) -> &PhyI2cmFsSclLcnt0Addr {
        &self.phy_i2cm_fs_scl_lcnt_0_addr
    }
    #[doc = "0x3033 - PHY I2C SDA HOLD Control Register"]
    #[inline(always)]
    pub const fn phy_i2cm_sda_hold(&self) -> &PhyI2cmSdaHold {
        &self.phy_i2cm_sda_hold
    }
    #[doc = "0x3034 - PHY I2C/JTAG I/O Configuration Control Register"]
    #[inline(always)]
    pub const fn jtag_phy_config(&self) -> &JtagPhyConfig {
        &self.jtag_phy_config
    }
    #[doc = "0x3035 - PHY JTAG Clock Control Register"]
    #[inline(always)]
    pub const fn jtag_phy_tap_tck(&self) -> &JtagPhyTapTck {
        &self.jtag_phy_tap_tck
    }
    #[doc = "0x3036 - PHY JTAG TAP In Control Register"]
    #[inline(always)]
    pub const fn jtag_phy_tap_in(&self) -> &JtagPhyTapIn {
        &self.jtag_phy_tap_in
    }
    #[doc = "0x3037 - PHY JTAG TAP Out Control Register"]
    #[inline(always)]
    pub const fn jtag_phy_tap_out(&self) -> &JtagPhyTapOut {
        &self.jtag_phy_tap_out
    }
    #[doc = "0x3038 - PHY JTAG Address Control Register"]
    #[inline(always)]
    pub const fn jtag_phy_addr(&self) -> &JtagPhyAddr {
        &self.jtag_phy_addr
    }
    #[doc = "0x3100 - Audio I2S Software FIFO Reset, Select, and Enable Control Register 0\n\nThis register configures the I2S input enable that indicates which input I2S channels have\n\nvalid data. It also allows the system processor to reset audio FIFOs upon\n\nunderflow/overflow error detection."]
    #[inline(always)]
    pub const fn aud_conf0(&self) -> &AudConf0 {
        &self.aud_conf0
    }
    #[doc = "0x3101 - Audio I2S Width Configuration Register 1 This register configures the data\n\nwidth of the input data."]
    #[inline(always)]
    pub const fn aud_conf1(&self) -> &AudConf1 {
        &self.aud_conf1
    }
    #[doc = "0x3102 - I2S FIFO status and interrupts.\n\nThis register configures the I2S FIFO status and interrupts."]
    #[inline(always)]
    pub const fn aud_int(&self) -> &AudInt {
        &self.aud_int
    }
    #[doc = "0x3103 - Audio I2S PCUV, NLPCM and HBR configuration Register 2\n\nThis register configures the I2S Audio Data mapping. By default, audio data mapping is the\n\nstandard I2S Linear PCM (L-PCM) mapping. You can choose to use the I2S interface to\n\ntransport HBR or Non- Linear PCM (NL-PCM) audio, by setting the relevant bit in this\n\nregister."]
    #[inline(always)]
    pub const fn aud_conf2(&self) -> &AudConf2 {
        &self.aud_conf2
    }
    #[doc = "0x3200 - Audio Clock Regenerator N Value Register 1 For N expected values, refer to\n\nthe HDMI 1.4b specification."]
    #[inline(always)]
    pub const fn aud_n1(&self) -> &AudN1 {
        &self.aud_n1
    }
    #[doc = "0x3201 - Audio Clock Regenerator N Value Register 2 For N expected values, refer to\n\nthe HDMI 1.4b specification."]
    #[inline(always)]
    pub const fn aud_n2(&self) -> &AudN2 {
        &self.aud_n2
    }
    #[doc = "0x3202 - Audio Clock Regenerator N Value Register 3 For N expected values, refer to\n\nthe HDMI 1.4b specification."]
    #[inline(always)]
    pub const fn aud_n3(&self) -> &AudN3 {
        &self.aud_n3
    }
    #[doc = "0x3203 - Audio Clock Regenerator CTS Value Register 1 For CTS expected values, refer\n\nto the HDMI 1.4b specification."]
    #[inline(always)]
    pub const fn aud_cts1(&self) -> &AudCts1 {
        &self.aud_cts1
    }
    #[doc = "0x3204 - Audio Clock Regenerator CTS Register 2\n\nFor CTS expected values, refer to the HDMI 1.4b specification."]
    #[inline(always)]
    pub const fn aud_cts2(&self) -> &AudCts2 {
        &self.aud_cts2
    }
    #[doc = "0x3205 - Audio Clock Regenerator CTS value Register 3. For CTS expected values, refer\n\nto the HDMI 1.4b specification."]
    #[inline(always)]
    pub const fn aud_cts3(&self) -> &AudCts3 {
        &self.aud_cts3
    }
    #[doc = "0x3206 - Audio Input Clock FS Factor Register"]
    #[inline(always)]
    pub const fn aud_inputclkfs(&self) -> &AudInputclkfs {
        &self.aud_inputclkfs
    }
    #[doc = "0x3207 - Audio CTS Dither Register"]
    #[inline(always)]
    pub const fn aud_cts_dither(&self) -> &AudCtsDither {
        &self.aud_cts_dither
    }
    #[doc = "0x3300 - Audio SPDIF Software FIFO Reset Control Register 0\n\nThis register allows the system processor to reset audio FIFOs upon underflow/overflow\n\nerror detection."]
    #[inline(always)]
    pub const fn aud_spdif0(&self) -> &AudSpdif0 {
        &self.aud_spdif0
    }
    #[doc = "0x3301 - Audio SPDIF NLPCM and Width Configuration Register 1 This register\n\nconfigures the SPDIF data width."]
    #[inline(always)]
    pub const fn aud_spdif1(&self) -> &AudSpdif1 {
        &self.aud_spdif1
    }
    #[doc = "0x3302 - Audio SPDIF FIFO Empty/Full Mask Register"]
    #[inline(always)]
    pub const fn aud_spdifint(&self) -> &AudSpdifint {
        &self.aud_spdifint
    }
    #[doc = "0x3303 - Audio SPDIF Mask Interrupt Register 1\n\nThis register masks interrupts present in the SPDIF module."]
    #[inline(always)]
    pub const fn aud_spdifint1(&self) -> &AudSpdifint1 {
        &self.aud_spdifint1
    }
    #[doc = "0x3304 - Audio SPDIF Enable Confiiguration Register 2\n\nThis register configures the SPDIF input enable that indicates which input SPDIF channels\n\nhave valid data."]
    #[inline(always)]
    pub const fn aud_spdif2(&self) -> &AudSpdif2 {
        &self.aud_spdif2
    }
    #[doc = "0x3500 - Audio GPA Software FIFO Reset Control Register 0"]
    #[inline(always)]
    pub const fn gp_conf0(&self) -> &GpConf0 {
        &self.gp_conf0
    }
    #[doc = "0x3501 - Audio GPA Channel Enable Configuration Register 1"]
    #[inline(always)]
    pub const fn gp_conf1(&self) -> &GpConf1 {
        &self.gp_conf1
    }
    #[doc = "0x3502 - Audio GPA HBR Enable Register 2"]
    #[inline(always)]
    pub const fn gp_conf2(&self) -> &GpConf2 {
        &self.gp_conf2
    }
    #[doc = "0x3506 - Audio GPA FIFO Full and Empty Mask Interrupt Register"]
    #[inline(always)]
    pub const fn gp_mask(&self) -> &GpMask {
        &self.gp_mask
    }
    #[doc = "0x3600 - Audio DMA SW FIFO reset and DMA Configuration Register 0\n\nThis register contains the software reset bit for the audio FIFOs. It also configures\n\noperating modes of the AHB master."]
    #[inline(always)]
    pub const fn ahb_dma_conf0(&self) -> &AhbDmaConf0 {
        &self.ahb_dma_conf0
    }
    #[doc = "0x3601 - Audio DMA Start Register\n\nThe start_dma_transaction bit field signals the AHB audio DMA to start accessing system\n\nmemory in order to fetch data samples to store in the FIFO. After the operation starts, a\n\nnew request for a DMA start is ignored until the DMA is stopped or it reaches the end\n\naddress. Only in one of these situations is a new start request acknowledged.\n\nThe first DMA burst request after start_dma_transaction configuration uses\n\ninitial_addr\\[31:0\\]
as ohaddr\\[31:0\\]
value; mburstlength\\[8:0\\]
is set to the maximum\n\nadmissible value. This maximum value is constrained by the size of buffer provided, the\n\ninstantiated FIFO depth, or/and the number of words up to the next 1 Kbyte boundary."]
    #[inline(always)]
    pub const fn ahb_dma_start(&self) -> &AhbDmaStart {
        &self.ahb_dma_start
    }
    #[doc = "0x3602 - Audio DMA Stop Register\n\nThe stop_dma_transaction bit field signals the AHB audio DMA to stop current Attr. After it\n\nstops, if a new start DMA operation is requested, the DMA engine restarts the Attr using\n\nthe initial_addr\\[31:0\\], which is programmed at ahb_dma_straddr0 to ahb_dma_straddr3."]
    #[inline(always)]
    pub const fn ahb_dma_stop(&self) -> &AhbDmaStop {
        &self.ahb_dma_stop
    }
    #[doc = "0x3603 - Audio DMA FIFO Threshold Register\n\nThis register defines the FIFO medium threshold occupation value.\n\nAfter the AHB master completes a burst transaction successfully, the FIFO may remain full\n\ntill the data fetch interface requests samples. Each data fetch operation reduces the\n\nnumber of samples stored in the FIFO by the number of channels enabled.\n\nTherefore, the fifo_threshold\\[7:0\\]
is the medium number of samples that should be\n\navailable in the audio FIFO across the DMA operation.\n\nAs soon as the number of samples in the FIFO drops lower than the fifo_threshold\\[7:0\\], the\n\nDMA engine requests a new burst of samples for the AHB master. The length is constrained\n\nby the size of buffer provided, the instantiated FIFO depth minus fifo_threshold\\[7:0\\],\n\nand/or the number of words up to the next 1 kbyte boundary."]
    #[inline(always)]
    pub const fn ahb_dma_thrsld(&self) -> &AhbDmaThrsld {
        &self.ahb_dma_thrsld
    }
    #[doc = "0x3604 - Audio DMA Start Address Set0 Register Array Address offset: i = 0 to 3\n\nThese registers define the initial_addr\\[31:0\\]
used to initiate the DMA burst read\n\ntransactions upon start_dma_transaction configuration."]
    #[inline(always)]
    pub const fn ahb_dma_straddr_set0(&self, n: usize) -> &AhbDmaStraddrSet0 {
        &self.ahb_dma_straddr_set0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3604 - Audio DMA Start Address Set0 Register Array Address offset: i = 0 to 3\n\nThese registers define the initial_addr\\[31:0\\]
used to initiate the DMA burst read\n\ntransactions upon start_dma_transaction configuration."]
    #[inline(always)]
    pub fn ahb_dma_straddr_set0_iter(&self) -> impl Iterator<Item = &AhbDmaStraddrSet0> {
        self.ahb_dma_straddr_set0.iter()
    }
    #[doc = "0x3608 - Audio DMA Stop Address Set0 Register Array Address offset: i = 0 to 3\n\nThis registers define the final_addr\\[31:0\\]
used as the final point to the DMA burst read\n\ntransactions.\n\nUpon start_dma_transaction configuration, the DMA engine starts requesting burst reads\n\nfrom the external system memory. Each burst read can have a maximum theoretical length\n\nof 256 words (due to the AMBA AHB specification 1 Kbyte boundary burst limitation).\n\nThe DMA engine is responsible for incrementing the burst starting address and defining its\n\ncorresponding burst length to reach the final_addr\\[31:0\\]
address. The last burst request\n\nissued by the DMA engine takes into account that it should only request data until the\n\nfinal_addr\\[31:0\\]
address (included) and for that should calculate the correct burst length.\n\nAfter reaching the final_addr\\[31:0\\]
address, the done interrupt is active to signal\n\ncompletion of DMA operation."]
    #[inline(always)]
    pub const fn ahb_dma_stpaddr_set0(&self, n: usize) -> &AhbDmaStpaddrSet0 {
        &self.ahb_dma_stpaddr_set0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3608 - Audio DMA Stop Address Set0 Register Array Address offset: i = 0 to 3\n\nThis registers define the final_addr\\[31:0\\]
used as the final point to the DMA burst read\n\ntransactions.\n\nUpon start_dma_transaction configuration, the DMA engine starts requesting burst reads\n\nfrom the external system memory. Each burst read can have a maximum theoretical length\n\nof 256 words (due to the AMBA AHB specification 1 Kbyte boundary burst limitation).\n\nThe DMA engine is responsible for incrementing the burst starting address and defining its\n\ncorresponding burst length to reach the final_addr\\[31:0\\]
address. The last burst request\n\nissued by the DMA engine takes into account that it should only request data until the\n\nfinal_addr\\[31:0\\]
address (included) and for that should calculate the correct burst length.\n\nAfter reaching the final_addr\\[31:0\\]
address, the done interrupt is active to signal\n\ncompletion of DMA operation."]
    #[inline(always)]
    pub fn ahb_dma_stpaddr_set0_iter(&self) -> impl Iterator<Item = &AhbDmaStpaddrSet0> {
        self.ahb_dma_stpaddr_set0.iter()
    }
    #[doc = "0x360c - Audio DMA Burst Start Address Register Array Address offset: i = 0 to 3\n\nThese read-only registers compose the start address of the current burst operation.\n\nburst_start_addr\\[31:0\\]
= haddr\\[31:0\\]
= initial_addr\\[31:0\\]
+ 16."]
    #[inline(always)]
    pub const fn ahb_dma_bstraddr(&self, n: usize) -> &AhbDmaBstraddr {
        &self.ahb_dma_bstraddr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x360c - Audio DMA Burst Start Address Register Array Address offset: i = 0 to 3\n\nThese read-only registers compose the start address of the current burst operation.\n\nburst_start_addr\\[31:0\\]
= haddr\\[31:0\\]
= initial_addr\\[31:0\\]
+ 16."]
    #[inline(always)]
    pub fn ahb_dma_bstraddr_iter(&self) -> impl Iterator<Item = &AhbDmaBstraddr> {
        self.ahb_dma_bstraddr.iter()
    }
    #[doc = "0x3610 - Audio DMA Burst Length Register 0\n\nThis registers holds the length of the current burst operation. As an example, if the first\n\nburst transaction of the AHB audio DMA is a length of 8, then the second burst should start\n\nat address ohaddr\\[31:0\\]
= initial_addr\\[31:0\\]
+ 32."]
    #[inline(always)]
    pub const fn ahb_dma_mblength0(&self) -> &AhbDmaMblength0 {
        &self.ahb_dma_mblength0
    }
    #[doc = "0x3611 - Audio DMA Burst Length Register 1\n\nThis registers holds the length of the current burst operation. As an example, if the first\n\nburst transaction of the AHB audio DMA is a length of 8, then the second burst should start\n\nat address ohaddr\\[31:0\\]
= initial_addr\\[31:0\\]
+ 32."]
    #[inline(always)]
    pub const fn ahb_dma_mblength1(&self) -> &AhbDmaMblength1 {
        &self.ahb_dma_mblength1
    }
    #[doc = "0x3614 - Audio DMA Mask Interrupt Register\n\nThis register masks each of the interrupts present in the AHB audio DMA module."]
    #[inline(always)]
    pub const fn ahb_dma_mask(&self) -> &AhbDmaMask {
        &self.ahb_dma_mask
    }
    #[doc = "0x3616 - Audio DMA Channel Enable Configuration Register 1\n\nIn AUDS packet configuration with layout 0 selected, the maximum number of active\n\nchannels is 2."]
    #[inline(always)]
    pub const fn ahb_dma_conf1(&self) -> &AhbDmaConf1 {
        &self.ahb_dma_conf1
    }
    #[doc = "0x3619 - Audio DMA Buffer Mask Interrupt Register"]
    #[inline(always)]
    pub const fn ahb_dma_buffmask(&self) -> &AhbDmaBuffmask {
        &self.ahb_dma_buffmask
    }
    #[doc = "0x361b - Audio DMA Mask Interrupt Register 1\n\nThis register masks interrupts present in the AHB audio DMA module."]
    #[inline(always)]
    pub const fn ahb_dma_mask1(&self) -> &AhbDmaMask1 {
        &self.ahb_dma_mask1
    }
    #[doc = "0x361c - Audio DMA Status"]
    #[inline(always)]
    pub const fn ahb_dma_status(&self) -> &AhbDmaStatus {
        &self.ahb_dma_status
    }
    #[doc = "0x361d - Audio DMA Configuration Register 2"]
    #[inline(always)]
    pub const fn ahb_dma_conf2(&self) -> &AhbDmaConf2 {
        &self.ahb_dma_conf2
    }
    #[doc = "0x3620 - Audio DMA Start Address Set 1 Register Array Address offset: i = 0 to 3\n\nThese registers define the initial_addr_1\\[31:0\\]
used to initiate the DMA burst read\n\ntransactions upon start_dma_transaction configuration."]
    #[inline(always)]
    pub const fn ahb_dma_straddr_set1(&self, n: usize) -> &AhbDmaStraddrSet1 {
        &self.ahb_dma_straddr_set1[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3620 - Audio DMA Start Address Set 1 Register Array Address offset: i = 0 to 3\n\nThese registers define the initial_addr_1\\[31:0\\]
used to initiate the DMA burst read\n\ntransactions upon start_dma_transaction configuration."]
    #[inline(always)]
    pub fn ahb_dma_straddr_set1_iter(&self) -> impl Iterator<Item = &AhbDmaStraddrSet1> {
        self.ahb_dma_straddr_set1.iter()
    }
    #[doc = "0x3624 - Audio DMA Stop Address Set 1 Register Array Address offset: i = 0 to 3\n\nThese registers define the final_addr_1\\[31:0\\]
used as the final point to the DMA burst read\n\ntransactions. Upon start_dma_transaction configuration, the DMA engine starts requesting\n\nburst reads from the external system memory. Each burst read can have a maximum\n\ntheoretical length of 256 words (due to the AMBA AHB specification 1 Kbyte boundary burst\n\nlimitation).\n\nThe DMA engine is responsible for incrementing the burst starting address and defining its\n\ncorresponding burst length to reach the final_addr\\[31:0\\]
address. The last burst request\n\nissued by the DMA engine takes into account that it should only request data until the\n\nfinal_addr\\[31:0\\]
address (included) and for that should calculate the correct burst length.\n\nAfter reaching the final_addr_1\\[31:0\\]
address, the done interrupt is active to indicate\n\ncompletion of the DMA operation."]
    #[inline(always)]
    pub const fn ahb_dma_stpaddr_set1(&self, n: usize) -> &AhbDmaStpaddrSet1 {
        &self.ahb_dma_stpaddr_set1[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3624 - Audio DMA Stop Address Set 1 Register Array Address offset: i = 0 to 3\n\nThese registers define the final_addr_1\\[31:0\\]
used as the final point to the DMA burst read\n\ntransactions. Upon start_dma_transaction configuration, the DMA engine starts requesting\n\nburst reads from the external system memory. Each burst read can have a maximum\n\ntheoretical length of 256 words (due to the AMBA AHB specification 1 Kbyte boundary burst\n\nlimitation).\n\nThe DMA engine is responsible for incrementing the burst starting address and defining its\n\ncorresponding burst length to reach the final_addr\\[31:0\\]
address. The last burst request\n\nissued by the DMA engine takes into account that it should only request data until the\n\nfinal_addr\\[31:0\\]
address (included) and for that should calculate the correct burst length.\n\nAfter reaching the final_addr_1\\[31:0\\]
address, the done interrupt is active to indicate\n\ncompletion of the DMA operation."]
    #[inline(always)]
    pub fn ahb_dma_stpaddr_set1_iter(&self) -> impl Iterator<Item = &AhbDmaStpaddrSet1> {
        self.ahb_dma_stpaddr_set1.iter()
    }
    #[doc = "0x4001 - Main Controller Synchronous Clock Domain Disable Register"]
    #[inline(always)]
    pub const fn mc_clkdis(&self) -> &McClkdis {
        &self.mc_clkdis
    }
    #[doc = "0x4002 - Main Controller Software Reset Register\n\nMain controller software reset request per clock domain. Writing zero to a bit of this\n\nregister results in an NRZ signal toggle at sfrclk rate to an output signal that indicates a\n\nsoftware reset request. This toggle must be used to generate a synchronized reset to de\n\ncorresponding domain, with at least 1 clock cycle."]
    #[inline(always)]
    pub const fn mc_swrstzreq(&self) -> &McSwrstzreq {
        &self.mc_swrstzreq
    }
    #[doc = "0x4003 - Main Controller HDCP Bypass Control Register"]
    #[inline(always)]
    pub const fn mc_opctrl(&self) -> &McOpctrl {
        &self.mc_opctrl
    }
    #[doc = "0x4004 - Main Controller Feed Through Control Register"]
    #[inline(always)]
    pub const fn mc_flowctrl(&self) -> &McFlowctrl {
        &self.mc_flowctrl
    }
    #[doc = "0x4005 - Main Controller PHY Reset Register"]
    #[inline(always)]
    pub const fn mc_phyrstz(&self) -> &McPhyrstz {
        &self.mc_phyrstz
    }
    #[doc = "0x4006 - Main Controller Clock Present Register"]
    #[inline(always)]
    pub const fn mc_lockonclock(&self) -> &McLockonclock {
        &self.mc_lockonclock
    }
    #[doc = "0x4007 - Main Controller HEAC PHY Reset Register"]
    #[inline(always)]
    pub const fn mc_heacphy_rst(&self) -> &McHeacphyRst {
        &self.mc_heacphy_rst
    }
    #[doc = "0x4009 - Main Controller Clock Present Register 2"]
    #[inline(always)]
    pub const fn mc_lockonclock_2(&self) -> &McLockonclock2 {
        &self.mc_lockonclock_2
    }
    #[doc = "0x400a - Main Controller Software Reset Register 2\n\nMain controller software reset request per clock domain. Writing zero to a bit of this\n\nregister results in a signal toggle that indicates a software reset request. This toggle is used\n\nto generate a synchronized reset to the corresponding domain, with one or more clock\n\ncycles."]
    #[inline(always)]
    pub const fn mc_swrstzreq_2(&self) -> &McSwrstzreq2 {
        &self.mc_swrstzreq_2
    }
    #[doc = "0x4010 - Main Controller Status Register\n\nThis register contains the information regarding the status of the HDCP SNPS 2.2 versus\n\n1.4 switch."]
    #[inline(always)]
    pub const fn mc_opsts(&self) -> &McOpsts {
        &self.mc_opsts
    }
    #[doc = "0x4018 - SFR Clock Base Time Register Low"]
    #[inline(always)]
    pub const fn base_sfrdivlow(&self) -> &BaseSfrdivlow {
        &self.base_sfrdivlow
    }
    #[doc = "0x4019 - SFR Clock Base Time Register High"]
    #[inline(always)]
    pub const fn base_sfrdivhigh(&self) -> &BaseSfrdivhigh {
        &self.base_sfrdivhigh
    }
    #[doc = "0x4100 - Color Space Converter Interpolation and Decimation Configuration Register"]
    #[inline(always)]
    pub const fn csc_cfg(&self) -> &CscCfg {
        &self.csc_cfg
    }
    #[doc = "0x4101 - Color Space Converter Scale and Deep Color Configuration Register"]
    #[inline(always)]
    pub const fn csc_scale(&self) -> &CscScale {
        &self.csc_scale
    }
    #[doc = "0x4102 - Color Space Converter Matrix A1 Coefficient Register MSB Notes:\n\nThe coefficients used in the CSC matrix use only 15 bits for the internal computations.\n\nCoefficients are represented in 2's complementary format and stored in two registers:\n\ncsc_coef_*_lsb\\[7:0\\]: coefficient bits 7 to 0\n\ncsc_coef_*_msb\\[7\\]: spare bit\n\ncsc_coef_*_msb\\[6:0\\]: coefficient bits 14 to 8\n\nExamples for standard ITU601 and ITU709 RGB/YCC conversion CSC coefficients exist in\n\nthe Video Datapath Application Note."]
    #[inline(always)]
    pub const fn csc_coef_a1_msb(&self) -> &CscCoefA1Msb {
        &self.csc_coef_a1_msb
    }
    #[doc = "0x4103 - Color Space Converter Matrix A1 Coefficient Register LSB Notes:\n\nThe coefficients used in the CSC matrix use only 15 bits for the internal computations.\n\nCoefficients are represented in 2's complementary format and stored in two registers:\n\ncsc_coef_*_lsb\\[7:0\\]: coefficient bits 7 to 0\n\ncsc_coef_*_msb\\[7\\]: spare bit\n\ncsc_coef_*_msb\\[6:0\\]: coefficient bits 14 to 8\n\nExamples for standard ITU601 and ITU709 RGB/YCC conversion CSC coefficients exist in\n\nthe Video Datapath Application Note."]
    #[inline(always)]
    pub const fn csc_coef_a1_lsb(&self) -> &CscCoefA1Lsb {
        &self.csc_coef_a1_lsb
    }
    #[doc = "0x4104 - Color Space Converter Matrix A2 Coefficient Register MSB Color Space\n\nConversion A2 coefficient."]
    #[inline(always)]
    pub const fn csc_coef_a2_msb(&self) -> &CscCoefA2Msb {
        &self.csc_coef_a2_msb
    }
    #[doc = "0x4105 - Color Space Converter Matrix A2 Coefficient Register LSB Color Space\n\nConversion A2 coefficient."]
    #[inline(always)]
    pub const fn csc_coef_a2_lsb(&self) -> &CscCoefA2Lsb {
        &self.csc_coef_a2_lsb
    }
    #[doc = "0x4106 - Color Space Converter Matrix A3 Coefficient Register MSB Color Space\n\nConversion A3 coefficient."]
    #[inline(always)]
    pub const fn csc_coef_a3_msb(&self) -> &CscCoefA3Msb {
        &self.csc_coef_a3_msb
    }
    #[doc = "0x4107 - Color Space Converter Matrix A3 Coefficient Register LSB Color Space\n\nConversion A3 coefficient."]
    #[inline(always)]
    pub const fn csc_coef_a3_lsb(&self) -> &CscCoefA3Lsb {
        &self.csc_coef_a3_lsb
    }
    #[doc = "0x4108 - Color Space Converter Matrix A4 Coefficient Register MSB Color Space\n\nConversion A4 coefficient."]
    #[inline(always)]
    pub const fn csc_coef_a4_msb(&self) -> &CscCoefA4Msb {
        &self.csc_coef_a4_msb
    }
    #[doc = "0x4109 - Color Space Converter Matrix A4 Coefficient Register LSB Color Space\n\nConversion A4 coefficient."]
    #[inline(always)]
    pub const fn csc_coef_a4_lsb(&self) -> &CscCoefA4Lsb {
        &self.csc_coef_a4_lsb
    }
    #[doc = "0x410a - Color Space Converter Matrix B1 Coefficient Register MSB Color Space\n\nConversion B1 coefficient."]
    #[inline(always)]
    pub const fn csc_coef_b1_msb(&self) -> &CscCoefB1Msb {
        &self.csc_coef_b1_msb
    }
    #[doc = "0x410b - Color Space Converter Matrix B1 Coefficient Register LSB Color Space\n\nConversion B1 coefficient."]
    #[inline(always)]
    pub const fn csc_coef_b1_lsb(&self) -> &CscCoefB1Lsb {
        &self.csc_coef_b1_lsb
    }
    #[doc = "0x410c - Color Space Converter Matrix B2 Coefficient Register MSB Color Space\n\nConversion B2 coefficient."]
    #[inline(always)]
    pub const fn csc_coef_b2_msb(&self) -> &CscCoefB2Msb {
        &self.csc_coef_b2_msb
    }
    #[doc = "0x410d - Color Space Converter Matrix B2 Coefficient Register LSB Color Space\n\nConversion B2 coefficient."]
    #[inline(always)]
    pub const fn csc_coef_b2_lsb(&self) -> &CscCoefB2Lsb {
        &self.csc_coef_b2_lsb
    }
    #[doc = "0x410e - Color Space Converter Matrix B3 Coefficient Register MSB Color Space\n\nConversion B3 coefficient."]
    #[inline(always)]
    pub const fn csc_coef_b3_msb(&self) -> &CscCoefB3Msb {
        &self.csc_coef_b3_msb
    }
    #[doc = "0x410f - Color Space Converter Matrix B3 Coefficient Register LSB Color Space\n\nConversion B3 coefficient."]
    #[inline(always)]
    pub const fn csc_coef_b3_lsb(&self) -> &CscCoefB3Lsb {
        &self.csc_coef_b3_lsb
    }
    #[doc = "0x4110 - Color Space Converter Matrix B4 Coefficient Register MSB Color Space\n\nConversion B4 coefficient."]
    #[inline(always)]
    pub const fn csc_coef_b4_msb(&self) -> &CscCoefB4Msb {
        &self.csc_coef_b4_msb
    }
    #[doc = "0x4111 - Color Space Converter Matrix B4 Coefficient Register LSB Color Space\n\nConversion B4 coefficient."]
    #[inline(always)]
    pub const fn csc_coef_b4_lsb(&self) -> &CscCoefB4Lsb {
        &self.csc_coef_b4_lsb
    }
    #[doc = "0x4112 - Color Space Converter Matrix C1 Coefficient Register MSB Color Space\n\nConversion C1 coefficient."]
    #[inline(always)]
    pub const fn csc_coef_c1_msb(&self) -> &CscCoefC1Msb {
        &self.csc_coef_c1_msb
    }
    #[doc = "0x4113 - Color Space Converter Matrix C1 Coefficient Register LSB Color Space\n\nConversion C1 coefficient."]
    #[inline(always)]
    pub const fn csc_coef_c1_lsb(&self) -> &CscCoefC1Lsb {
        &self.csc_coef_c1_lsb
    }
    #[doc = "0x4114 - Color Space Converter Matrix C2 Coefficient Register MSB Color Space\n\nConversion C2 coefficient."]
    #[inline(always)]
    pub const fn csc_coef_c2_msb(&self) -> &CscCoefC2Msb {
        &self.csc_coef_c2_msb
    }
    #[doc = "0x4115 - Color Space Converter Matrix C2 Coefficient Register LSB Color Space\n\nConversion C2 coefficient."]
    #[inline(always)]
    pub const fn csc_coef_c2_lsb(&self) -> &CscCoefC2Lsb {
        &self.csc_coef_c2_lsb
    }
    #[doc = "0x4116 - Color Space Converter Matrix C3 Coefficient Register MSB Color Space\n\nConversion C3 coefficient."]
    #[inline(always)]
    pub const fn csc_coef_c3_msb(&self) -> &CscCoefC3Msb {
        &self.csc_coef_c3_msb
    }
    #[doc = "0x4117 - Color Space Converter Matrix C3 Coefficient Register LSB Color Space\n\nConversion C3 coefficient."]
    #[inline(always)]
    pub const fn csc_coef_c3_lsb(&self) -> &CscCoefC3Lsb {
        &self.csc_coef_c3_lsb
    }
    #[doc = "0x4118 - Color Space Converter Matrix C4 Coefficient Register MSB Color Space\n\nConversion C4 coefficient."]
    #[inline(always)]
    pub const fn csc_coef_c4_msb(&self) -> &CscCoefC4Msb {
        &self.csc_coef_c4_msb
    }
    #[doc = "0x4119 - Color Space Converter Matrix C4 Coefficient Register LSB Color Space\n\nConversion C4 coefficient."]
    #[inline(always)]
    pub const fn csc_coef_c4_lsb(&self) -> &CscCoefC4Lsb {
        &self.csc_coef_c4_lsb
    }
    #[doc = "0x411a - Color Space Converter Matrix Output Up Limit Register MSB\n\nFor more details, refer to the HDMI 1.4 specification, paragraph 6.6 Video Quantization\n\nRanges. For an RGB output of 8 bits, the expected limit is 254, and this register must be\n\nconfigured with 0x00."]
    #[inline(always)]
    pub const fn csc_limit_up_msb(&self) -> &CscLimitUpMsb {
        &self.csc_limit_up_msb
    }
    #[doc = "0x411b - Color Space Converter Matrix output Up Limit Register LSB\n\nFor more details, refer to the HDMI 1.4 specification, paragraph 6.6 Video Quantization\n\nRanges. For an RGB output of 8 bits, the expected limit is 254, and this register must be\n\nconfigured with 0xFE."]
    #[inline(always)]
    pub const fn csc_limit_up_lsb(&self) -> &CscLimitUpLsb {
        &self.csc_limit_up_lsb
    }
    #[doc = "0x411c - Color Space Converter Matrix output Down Limit Register MSB\n\nFor more details, refer to the HDMI 1.4 specification, paragraph 6.6 Video Quantization\n\nRanges. For an RGB output of 8 bits, the expected limit is 1, and this register must be\n\nconfigured with 0x00."]
    #[inline(always)]
    pub const fn csc_limit_dn_msb(&self) -> &CscLimitDnMsb {
        &self.csc_limit_dn_msb
    }
    #[doc = "0x411d - Color Space Converter Matrix output Down Limit Register LSB\n\nFor more details, refer to the HDMI 1.4 specification, paragraph 6.6 Video Quantization\n\nRanges. For an RGB output of 8 bits, the expected limit is 1, and this register must be\n\nconfigured with 0x01."]
    #[inline(always)]
    pub const fn csc_limit_dn_lsb(&self) -> &CscLimitDnLsb {
        &self.csc_limit_dn_lsb
    }
    #[doc = "0x5000 - HDCP Enable and Functional Control Configuration Register 0"]
    #[inline(always)]
    pub const fn a_hdcpcfg0(&self) -> &AHdcpcfg0 {
        &self.a_hdcpcfg0
    }
    #[doc = "0x5001 - HDCP Software Reset and Functional Control Configuration Register 1"]
    #[inline(always)]
    pub const fn a_hdcpcfg1(&self) -> &AHdcpcfg1 {
        &self.a_hdcpcfg1
    }
    #[doc = "0x5002 - HDCP Observation Register 0"]
    #[inline(always)]
    pub const fn a_hdcpobs0(&self) -> &AHdcpobs0 {
        &self.a_hdcpobs0
    }
    #[doc = "0x5003 - HDCP Observation Register 1"]
    #[inline(always)]
    pub const fn a_hdcpobs1(&self) -> &AHdcpobs1 {
        &self.a_hdcpobs1
    }
    #[doc = "0x5004 - HDCP Observation Register 2"]
    #[inline(always)]
    pub const fn a_hdcpobs2(&self) -> &AHdcpobs2 {
        &self.a_hdcpobs2
    }
    #[doc = "0x5005 - HDCP Observation Register 3"]
    #[inline(always)]
    pub const fn a_hdcpobs3(&self) -> &AHdcpobs3 {
        &self.a_hdcpobs3
    }
    #[doc = "0x5006 - HDCP Interrupt Clear Register\n\nWrite only register, active high and auto cleared, cleans the respective interruption in the\n\ninterrupt status register."]
    #[inline(always)]
    pub const fn a_apiintclr(&self) -> &AApiintclr {
        &self.a_apiintclr
    }
    #[doc = "0x5007 - HDCP Interrupt Status Register\n\nRead only register, reports the interruption which caused the activation of the interruption\n\noutput pin."]
    #[inline(always)]
    pub const fn a_apiintstat(&self) -> &AApiintstat {
        &self.a_apiintstat
    }
    #[doc = "0x5008 - HDCP Interrupt Mask Register\n\nThe configuration of this register mask a given setup of interruption, disabling them from\n\ngenerating interruption pulses in the interruption output pin."]
    #[inline(always)]
    pub const fn a_apiintmsk(&self) -> &AApiintmsk {
        &self.a_apiintmsk
    }
    #[doc = "0x5009 - HDCP Video Polarity Configuration Register"]
    #[inline(always)]
    pub const fn a_vidpolcfg(&self) -> &AVidpolcfg {
        &self.a_vidpolcfg
    }
    #[doc = "0x500a - HDCP OESS WOO Configuration Register\n\nPulse width of the encryption enable (CTL3) signal in the HDCP OESS mode. The window of\n\nopportunity for the Original Encryption Status Signaling starts at the active edge of the\n\nVertical synchronism and stops after oesswindowoffset\\[7:0\\]*4 clock cycles of TMDS clock.\n\nAccording to the HDCP specification, the CTL3 signal must be asserted at least for eight\n\nTMDS clock cycles (oesswindowoffset\\[7:0\\]
must be greater than 1), and it is recommended\n\nto transmit a larger pulse width for enhanced link reliability."]
    #[inline(always)]
    pub const fn a_oesswcfg(&self) -> &AOesswcfg {
        &self.a_oesswcfg
    }
    #[doc = "0x5014 - HDCP Controller Version Register LSB Design ID number."]
    #[inline(always)]
    pub const fn a_coreverlsb(&self) -> &ACoreverlsb {
        &self.a_coreverlsb
    }
    #[doc = "0x5015 - HDCP Controller Version Register MSB Revision ID number."]
    #[inline(always)]
    pub const fn a_corevermsb(&self) -> &ACorevermsb {
        &self.a_corevermsb
    }
    #[doc = "0x5016 - HDCP KSV Memory Control Register\n\nThe KSVCTRLupd bit is a notification flag. This flag changes polarity whenever the register\n\nis written. This flag acts as a trigger to other blocks that processes this data. Upon reset\n\nthe flag returns to low default value."]
    #[inline(always)]
    pub const fn a_ksvmemctrl(&self) -> &AKsvmemctrl {
        &self.a_ksvmemctrl
    }
    #[doc = "0x5020 - HDCP BStatus Register Array"]
    #[inline(always)]
    pub const fn hdcp_bstatus(&self, n: usize) -> &HdcpBstatus {
        &self.hdcp_bstatus[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x5020 - HDCP BStatus Register Array"]
    #[inline(always)]
    pub fn hdcp_bstatus_iter(&self) -> impl Iterator<Item = &HdcpBstatus> {
        self.hdcp_bstatus.iter()
    }
    #[doc = "0x5022..0x502a - HDCP M0 Register Array"]
    #[inline(always)]
    pub const fn hdcp_m0(&self, n: usize) -> &HdcpM0 {
        &self.hdcp_m0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x5022..0x502a - HDCP M0 Register Array"]
    #[inline(always)]
    pub fn hdcp_m0_iter(&self) -> impl Iterator<Item = &HdcpM0> {
        self.hdcp_m0.iter()
    }
    #[doc = "0x502a..0x52a5 - HDCP KSV Registers."]
    #[inline(always)]
    pub const fn hdcp_ksv(&self, n: usize) -> &HdcpKsv {
        &self.hdcp_ksv[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x502a..0x52a5 - HDCP KSV Registers."]
    #[inline(always)]
    pub fn hdcp_ksv_iter(&self) -> impl Iterator<Item = &HdcpKsv> {
        self.hdcp_ksv.iter()
    }
    #[doc = "0x52a5..0x52b9 - HDCP SHA-1 VH Registers."]
    #[inline(always)]
    pub const fn hdcp_vh(&self, n: usize) -> &HdcpVh {
        &self.hdcp_vh[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x52a5..0x52b9 - HDCP SHA-1 VH Registers."]
    #[inline(always)]
    pub fn hdcp_vh_iter(&self) -> impl Iterator<Item = &HdcpVh> {
        self.hdcp_vh.iter()
    }
    #[doc = "0x52b9 - HDCP Revocation KSV List Size Register 0"]
    #[inline(always)]
    pub const fn hdcp_revoc_size_0(&self) -> &HdcpRevocSize0 {
        &self.hdcp_revoc_size_0
    }
    #[doc = "0x52ba - HDCP Revocation KSV List Size Register 1"]
    #[inline(always)]
    pub const fn hdcp_revoc_size_1(&self) -> &HdcpRevocSize1 {
        &self.hdcp_revoc_size_1
    }
    #[doc = "0x52bb..0x667f - HDCP Revocation KSV Registers."]
    #[inline(always)]
    pub const fn hdcp_revoc_list(&self, n: usize) -> &HdcpRevocList {
        &self.hdcp_revoc_list[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x52bb..0x667f - HDCP Revocation KSV Registers."]
    #[inline(always)]
    pub fn hdcp_revoc_list_iter(&self) -> impl Iterator<Item = &HdcpRevocList> {
        self.hdcp_revoc_list.iter()
    }
    #[doc = "0x7800 - HDCP KSV Status Register 0"]
    #[inline(always)]
    pub const fn hdcpreg_bksv0(&self) -> &HdcpregBksv0 {
        &self.hdcpreg_bksv0
    }
    #[doc = "0x7801 - HDCP KSV Status Register 1"]
    #[inline(always)]
    pub const fn hdcpreg_bksv1(&self) -> &HdcpregBksv1 {
        &self.hdcpreg_bksv1
    }
    #[doc = "0x7802 - HDCP KSV Status Register 2"]
    #[inline(always)]
    pub const fn hdcpreg_bksv2(&self) -> &HdcpregBksv2 {
        &self.hdcpreg_bksv2
    }
    #[doc = "0x7803 - HDCP KSV Status Register 3"]
    #[inline(always)]
    pub const fn hdcpreg_bksv3(&self) -> &HdcpregBksv3 {
        &self.hdcpreg_bksv3
    }
    #[doc = "0x7804 - HDCP KSV Status Register 4"]
    #[inline(always)]
    pub const fn hdcpreg_bksv4(&self) -> &HdcpregBksv4 {
        &self.hdcpreg_bksv4
    }
    #[doc = "0x7805 - HDCP AN Bypass Control Register"]
    #[inline(always)]
    pub const fn hdcpreg_anconf(&self) -> &HdcpregAnconf {
        &self.hdcpreg_anconf
    }
    #[doc = "0x7806 - HDCP Forced AN Register 0"]
    #[inline(always)]
    pub const fn hdcpreg_an0(&self) -> &HdcpregAn0 {
        &self.hdcpreg_an0
    }
    #[doc = "0x7807 - HDCP Forced AN Register 1"]
    #[inline(always)]
    pub const fn hdcpreg_an1(&self) -> &HdcpregAn1 {
        &self.hdcpreg_an1
    }
    #[doc = "0x7808 - HDCP forced AN Register 2"]
    #[inline(always)]
    pub const fn hdcpreg_an2(&self) -> &HdcpregAn2 {
        &self.hdcpreg_an2
    }
    #[doc = "0x7809 - HDCP Forced AN Register 3"]
    #[inline(always)]
    pub const fn hdcpreg_an3(&self) -> &HdcpregAn3 {
        &self.hdcpreg_an3
    }
    #[doc = "0x780a - HDCP Forced AN Register 4"]
    #[inline(always)]
    pub const fn hdcpreg_an4(&self) -> &HdcpregAn4 {
        &self.hdcpreg_an4
    }
    #[doc = "0x780b - HDCP Forced AN Register 5"]
    #[inline(always)]
    pub const fn hdcpreg_an5(&self) -> &HdcpregAn5 {
        &self.hdcpreg_an5
    }
    #[doc = "0x780c - HDCP Forced AN Register 6"]
    #[inline(always)]
    pub const fn hdcpreg_an6(&self) -> &HdcpregAn6 {
        &self.hdcpreg_an6
    }
    #[doc = "0x780d - HDCP Forced AN Register 7"]
    #[inline(always)]
    pub const fn hdcpreg_an7(&self) -> &HdcpregAn7 {
        &self.hdcpreg_an7
    }
    #[doc = "0x780e - HDCP Encrypted Device Private Keys Control Register\n\nThis register is the control register for the software programmable encrypted DPK\n\nembedded storage feature. The required software configuration sequence is documented in\n\nthe Cores HDMI Transmitter User Guide in the \"Programming\" chapter, Section 3.2.4,\n\n\"Configure HDCP.\""]
    #[inline(always)]
    pub const fn hdcpreg_rmlctl(&self) -> &HdcpregRmlctl {
        &self.hdcpreg_rmlctl
    }
    #[doc = "0x780f - HDCP Encrypted DPK Status Register\n\nThe required software configuration sequence is documented in the Cores HDMI\n\nTransmitter User Guide in the \"Programming\" chapter, Section 3.2.4, \"Configure HDCP.\""]
    #[inline(always)]
    pub const fn hdcpreg_rmlsts(&self) -> &HdcpregRmlsts {
        &self.hdcpreg_rmlsts
    }
    #[doc = "0x7810 - HDCP Encrypted DPK Seed Register 0\n\nThis register contains a byte of the HDCP Encrypted DPK seed value used to encrypt the\n\nDevice Private Keys. The required software configuration sequence is documented in the\n\nCores HDMI Transmitter User Guide in the \"Programming\" chapter, Section 3.2.4,\n\n\"Configure HDCP.\""]
    #[inline(always)]
    pub const fn hdcpreg_seed0(&self) -> &HdcpregSeed0 {
        &self.hdcpreg_seed0
    }
    #[doc = "0x7811 - HDCP Encrypted DPK Seed Register 1\n\nThis register contains a byte of the HDCP Encrypted DPK seed value used to encrypt the\n\nDevice Private Keys. The required software configuration sequence is documented in the\n\nCores HDMI Transmitter User Guide in the \"Programming\" chapter, Section 3.2.4,\n\n\"Configure HDCP.\""]
    #[inline(always)]
    pub const fn hdcpreg_seed1(&self) -> &HdcpregSeed1 {
        &self.hdcpreg_seed1
    }
    #[doc = "0x7812 - HDCP Encrypted DPK Data Register 0\n\nThis register contains an HDCP DPK byte. The required software configuration sequence is\n\ndocumented in the Cores HDMI Transmitter User Guide in the \"Programming\" chapter,\n\nSection 3.2.4, \"Configure HDCP.\""]
    #[inline(always)]
    pub const fn hdcpreg_dpk0(&self) -> &HdcpregDpk0 {
        &self.hdcpreg_dpk0
    }
    #[doc = "0x7813 - HDCP Encrypted DPK Data Register 1\n\nThis register contains an HDCP DPK byte. The required software configuration sequence is\n\ndocumented in the Cores HDMI Transmitter User Guide in the \"Programming\" chapter,\n\nSection 3.2.4, \"Configure HDCP.\""]
    #[inline(always)]
    pub const fn hdcpreg_dpk1(&self) -> &HdcpregDpk1 {
        &self.hdcpreg_dpk1
    }
    #[doc = "0x7814 - HDCP Encrypted DPK Data Register 2\n\nThis register contains an HDCP DPK byte. The required software configuration sequence is\n\ndocumented in the Cores HDMI Transmitter User Guide in the \"Programming\" chapter,\n\nSection 3.2.4, \"Configure HDCP.\""]
    #[inline(always)]
    pub const fn hdcpreg_dpk2(&self) -> &HdcpregDpk2 {
        &self.hdcpreg_dpk2
    }
    #[doc = "0x7815 - HDCP Encrypted DPK Data Register 3\n\nThis register contains an HDCP DPK byte. The required software configuration sequence is\n\ndocumented in the Cores HDMI Transmitter User Guide in the \"Programming\" chapter,\n\nSection 3.2.4, \"Configure HDCP.\""]
    #[inline(always)]
    pub const fn hdcpreg_dpk3(&self) -> &HdcpregDpk3 {
        &self.hdcpreg_dpk3
    }
    #[doc = "0x7816 - HDCP Encrypted DPK Data Register 4\n\nThis register contains an HDCP DPK byte."]
    #[inline(always)]
    pub const fn hdcpreg_dpk4(&self) -> &HdcpregDpk4 {
        &self.hdcpreg_dpk4
    }
    #[doc = "0x7817 - HDCP Encrypted DPK Data Register 5\n\nThis register contains an HDCP DPK byte. The required software configuration sequence is\n\ndocumented in the Cores HDMI Transmitter User Guide in the \"Programming\" chapter,\n\nSection 3.2.4, \"Configure HDCP.\""]
    #[inline(always)]
    pub const fn hdcpreg_dpk5(&self) -> &HdcpregDpk5 {
        &self.hdcpreg_dpk5
    }
    #[doc = "0x7818 - HDCP Encrypted DPK Data Register 6\n\nThis register contains an HDCP DPK byte. The required software configuration sequence is\n\ndocumented in the Cores HDMI Transmitter User Guide in the \"Programming\" chapter,\n\nSection 3.2.4, \"Configure HDCP.\""]
    #[inline(always)]
    pub const fn hdcpreg_dpk6(&self) -> &HdcpregDpk6 {
        &self.hdcpreg_dpk6
    }
    #[doc = "0x7900 - HDCP 2.2 Identification Register"]
    #[inline(always)]
    pub const fn hdcp22reg_id(&self) -> &Hdcp22regId {
        &self.hdcp22reg_id
    }
    #[doc = "0x7904 - HDCP 2.2 Control Register"]
    #[inline(always)]
    pub const fn hdcp22reg_ctrl(&self) -> &Hdcp22regCtrl {
        &self.hdcp22reg_ctrl
    }
    #[doc = "0x7905 - HDCP 2.2 Control Register 1"]
    #[inline(always)]
    pub const fn hdcp22reg_ctrl1(&self) -> &Hdcp22regCtrl1 {
        &self.hdcp22reg_ctrl1
    }
    #[doc = "0x7908 - HDCP 2.2 Status Register"]
    #[inline(always)]
    pub const fn hdcp22reg_sts(&self) -> &Hdcp22regSts {
        &self.hdcp22reg_sts
    }
    #[doc = "0x790c - HDCP 2.2 Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hdcp22reg_mask(&self) -> &Hdcp22regMask {
        &self.hdcp22reg_mask
    }
    #[doc = "0x790d - HDCP 2.2 interrupt Sticky Bit Status Register"]
    #[inline(always)]
    pub const fn hdcp22reg_stat(&self) -> &Hdcp22regStat {
        &self.hdcp22reg_stat
    }
    #[doc = "0x790e - HDCP 2.2 Interrupt Mute Vector"]
    #[inline(always)]
    pub const fn hdcp22reg_mute(&self) -> &Hdcp22regMute {
        &self.hdcp22reg_mute
    }
    #[doc = "0x7d00 - CEC Control Register\n\nThis register handles the main control of the CEC initiator."]
    #[inline(always)]
    pub const fn cec_ctrl(&self) -> &CecCtrl {
        &self.cec_ctrl
    }
    #[doc = "0x7d02 - CEC Interrupt Mask Register\n\nThis read/write register masks/unmasks the interrupt events. When the bit is set to 1\n\n(masked), the corresponding event does not trigger an interrupt signal at the system\n\ninterface. When the bit is reset to 0, the interrupt event is unmasked."]
    #[inline(always)]
    pub const fn cec_mask(&self) -> &CecMask {
        &self.cec_mask
    }
    #[doc = "0x7d05 - CEC Logical Address Register Low\n\nThis register indicates the logical address(es) allocated to the CEC device.\n\nThis register is written by software when the logical allocation is finished. Bit value 1 means\n\nthe corresponding logical address is allocated to this device. Bit value 0 means the\n\ncorresponding logical address is not allocated to this device."]
    #[inline(always)]
    pub const fn cec_addr_l(&self) -> &CecAddrL {
        &self.cec_addr_l
    }
    #[doc = "0x7d06 - CEC Logical Address Register High\n\nThis register indicates the logical address(es) allocated to the CEC device.\n\nThis register is written by software when the logical allocation is finished. Bit value 1 means\n\nthe corresponding logical address is allocated to this device. Bit value 0 means the\n\ncorresponding logical address is not allocated to this device."]
    #[inline(always)]
    pub const fn cec_addr_h(&self) -> &CecAddrH {
        &self.cec_addr_h
    }
    #[doc = "0x7d07 - CEC TX Frame Size Register\n\nThis register indicates the size of the frame in bytes (including header and data blocks),\n\nwhich are available in the transmitter data buffer.\n\nNote: When the value is zero, the CEC controller ignores the send command triggered by\n\nsoftware. When the transmission is done (no matter success or not), the current value is\n\nheld until it is overwritten by software."]
    #[inline(always)]
    pub const fn cec_tx_cnt(&self) -> &CecTxCnt {
        &self.cec_tx_cnt
    }
    #[doc = "0x7d08 - CEC RX Frame Size Register\n\nThis register indicates the size of the frame in bytes (including header and data blocks),\n\nwhich are available in the receiver data buffer.\n\nNote: Only after the whole receiving process is finished successfully, the counter is\n\nrefreshed to the value which indicates the total number of data bytes in the Receiver Data\n\nRegister."]
    #[inline(always)]
    pub const fn cec_rx_cnt(&self) -> &CecRxCnt {
        &self.cec_rx_cnt
    }
    #[doc = "0x7d10..0x7d20 - CEC TX Data Register Array Address offset: i = 0 to 15\n\nThese registers (8 bits each) are the buffers used for storing the data waiting for\n\ntransmission (including header and data blocks)."]
    #[inline(always)]
    pub const fn cec_tx_data(&self, n: usize) -> &CecTxData {
        &self.cec_tx_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x7d10..0x7d20 - CEC TX Data Register Array Address offset: i = 0 to 15\n\nThese registers (8 bits each) are the buffers used for storing the data waiting for\n\ntransmission (including header and data blocks)."]
    #[inline(always)]
    pub fn cec_tx_data_iter(&self) -> impl Iterator<Item = &CecTxData> {
        self.cec_tx_data.iter()
    }
    #[doc = "0x7d20..0x7d30 - CEC RX Data Register Array Address offset: i =0 to 15\n\nThese registers (8 bit each) are the buffers used for storing the received data (including\n\nheader and data blocks)."]
    #[inline(always)]
    pub const fn cec_rx_data(&self, n: usize) -> &CecRxData {
        &self.cec_rx_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x7d20..0x7d30 - CEC RX Data Register Array Address offset: i =0 to 15\n\nThese registers (8 bit each) are the buffers used for storing the received data (including\n\nheader and data blocks)."]
    #[inline(always)]
    pub fn cec_rx_data_iter(&self) -> impl Iterator<Item = &CecRxData> {
        self.cec_rx_data.iter()
    }
    #[doc = "0x7d30 - CEC Buffer Lock Register"]
    #[inline(always)]
    pub const fn cec_lock(&self) -> &CecLock {
        &self.cec_lock
    }
    #[doc = "0x7d31 - CEC Wake-up Control Register\n\nAfter receiving a message in the CEC_RX_DATA1 (OPCODE) registers, the CEC engine\n\nverifies the message opcode\\[7:0\\]
against one of the previously defined values to generate\n\nthe wake-up status:\n\nWakeupstatus is 1 when:\n\nreceived opcode is 0x04 and opcode0x04en is 1 or\n\nreceived opcode is 0x0D and opcode0x0Den is 1 or\n\nreceived opcode is 0x41 and opcode0x41en is 1 or\n\nreceived opcode is 0x42 and opcode0x42en is 1 or\n\nreceived opcode is 0x44 and opcode0x44en is 1 or\n\nreceived opcode is 0x70 and opcode0x70en is 1 or\n\nreceived opcode is 0x82 and opcode0x82en is 1 or\n\nreceived opcode is 0x86 and opcode0x86en is 1\n\nWakeupstatus is 0 when none of the previous conditions are true.\n\nThis formula means that the wake-up status (on CEC_STAT\\[6\\]
register) is only '1' if the\n\nopcode\\[7:0\\]
received is equal to one of the defined values and the corresponding enable bit\n\nof that defined value is set to '1'."]
    #[inline(always)]
    pub const fn cec_wakeupctrl(&self) -> &CecWakeupctrl {
        &self.cec_wakeupctrl
    }
    #[doc = "0x7e00 - I2C DDC Slave address Configuration Register"]
    #[inline(always)]
    pub const fn i2cm_slave(&self) -> &I2cmSlave {
        &self.i2cm_slave
    }
    #[doc = "0x7e01 - I2C DDC Address Configuration Register"]
    #[inline(always)]
    pub const fn i2cm_address(&self) -> &I2cmAddress {
        &self.i2cm_address
    }
    #[doc = "0x7e02 - I2C DDC Data Write Register"]
    #[inline(always)]
    pub const fn i2cm_datao(&self) -> &I2cmDatao {
        &self.i2cm_datao
    }
    #[doc = "0x7e03 - I2C DDC Data read Register"]
    #[inline(always)]
    pub const fn i2cm_datai(&self) -> &I2cmDatai {
        &self.i2cm_datai
    }
    #[doc = "0x7e04 - I2C DDC RD/RD_EXT/WR Operation Register\n\nRead and write operation request. This register can only be written; reading this register\n\nalways results in 00h. Writing 1'b1 simultaneously to rd, rd_ext and wr requests is\n\nconsidered as a read (rd) request."]
    #[inline(always)]
    pub const fn i2cm_operation(&self) -> &I2cmOperation {
        &self.i2cm_operation
    }
    #[doc = "0x7e05 - I2C DDC Done Interrupt Register This register configures the I2C master\n\ninterrupts."]
    #[inline(always)]
    pub const fn i2cm_int(&self) -> &I2cmInt {
        &self.i2cm_int
    }
    #[doc = "0x7e06 - I2C DDC error Interrupt Register\n\nThis register configures the I2C master arbitration lost and not acknowledge error\n\ninterrupts."]
    #[inline(always)]
    pub const fn i2cm_ctlint(&self) -> &I2cmCtlint {
        &self.i2cm_ctlint
    }
    #[doc = "0x7e07 - I2C DDC Speed Control Register\n\nThis register configures the division relation between master and scl clock."]
    #[inline(always)]
    pub const fn i2cm_div(&self) -> &I2cmDiv {
        &self.i2cm_div
    }
    #[doc = "0x7e08 - I2C DDC Segment Address Configuration Register\n\nThis register configures the segment address for extended R/W destination and is used for\n\nEDID reading operations, particularly for the Extended Data Read Operation for Enhanced\n\nDDC."]
    #[inline(always)]
    pub const fn i2cm_segaddr(&self) -> &I2cmSegaddr {
        &self.i2cm_segaddr
    }
    #[doc = "0x7e09 - I2C DDC Software Reset Control Register This register resets the I2C master."]
    #[inline(always)]
    pub const fn i2cm_softrstz(&self) -> &I2cmSoftrstz {
        &self.i2cm_softrstz
    }
    #[doc = "0x7e0a - I2C DDC Segment Pointer Register\n\nThis register configures the segment pointer for extended RD/WR request."]
    #[inline(always)]
    pub const fn i2cm_segptr(&self) -> &I2cmSegptr {
        &self.i2cm_segptr
    }
    #[doc = "0x7e0b - I2C DDC Slow Speed SCL High Level Control Register 1"]
    #[inline(always)]
    pub const fn i2cm_ss_scl_hcnt_1_addr(&self) -> &I2cmSsSclHcnt1Addr {
        &self.i2cm_ss_scl_hcnt_1_addr
    }
    #[doc = "0x7e0c - I2C DDC Slow Speed SCL High Level Control Register 0"]
    #[inline(always)]
    pub const fn i2cm_ss_scl_hcnt_0_addr(&self) -> &I2cmSsSclHcnt0Addr {
        &self.i2cm_ss_scl_hcnt_0_addr
    }
    #[doc = "0x7e0d - I2C DDC Slow Speed SCL Low Level Control Register 1"]
    #[inline(always)]
    pub const fn i2cm_ss_scl_lcnt_1_addr(&self) -> &I2cmSsSclLcnt1Addr {
        &self.i2cm_ss_scl_lcnt_1_addr
    }
    #[doc = "0x7e0e - I2C DDC Slow Speed SCL Low Level Control Register 0"]
    #[inline(always)]
    pub const fn i2cm_ss_scl_lcnt_0_addr(&self) -> &I2cmSsSclLcnt0Addr {
        &self.i2cm_ss_scl_lcnt_0_addr
    }
    #[doc = "0x7e0f - I2C DDC Fast Speed SCL High Level Control Register 1"]
    #[inline(always)]
    pub const fn i2cm_fs_scl_hcnt_1_addr(&self) -> &I2cmFsSclHcnt1Addr {
        &self.i2cm_fs_scl_hcnt_1_addr
    }
    #[doc = "0x7e10 - I2C DDC Fast Speed SCL High Level Control Register 0"]
    #[inline(always)]
    pub const fn i2cm_fs_scl_hcnt_0_addr(&self) -> &I2cmFsSclHcnt0Addr {
        &self.i2cm_fs_scl_hcnt_0_addr
    }
    #[doc = "0x7e11 - I2C DDC Fast Speed SCL Low Level Control Register 1"]
    #[inline(always)]
    pub const fn i2cm_fs_scl_lcnt_1_addr(&self) -> &I2cmFsSclLcnt1Addr {
        &self.i2cm_fs_scl_lcnt_1_addr
    }
    #[doc = "0x7e12 - I2C DDC Fast Speed SCL Low Level Control Register 0"]
    #[inline(always)]
    pub const fn i2cm_fs_scl_lcnt_0_addr(&self) -> &I2cmFsSclLcnt0Addr {
        &self.i2cm_fs_scl_lcnt_0_addr
    }
    #[doc = "0x7e13 - I2C DDC SDA Hold Register"]
    #[inline(always)]
    pub const fn i2cm_sda_hold(&self) -> &I2cmSdaHold {
        &self.i2cm_sda_hold
    }
    #[doc = "0x7e14 - SCDC Control Register\n\nThis register configures the SCDC update status read through the I2C master interface."]
    #[inline(always)]
    pub const fn i2cm_scdc_read_update(&self) -> &I2cmScdcReadUpdate {
        &self.i2cm_scdc_read_update
    }
    #[doc = "0x7e20 - I2C Master Sequential Read Buffer Register 0"]
    #[inline(always)]
    pub const fn i2cm_read_buff0(&self) -> &I2cmReadBuff0 {
        &self.i2cm_read_buff0
    }
    #[doc = "0x7e21 - I2C Master Sequential Read Buffer Register 1"]
    #[inline(always)]
    pub const fn i2cm_read_buff1(&self) -> &I2cmReadBuff1 {
        &self.i2cm_read_buff1
    }
    #[doc = "0x7e22 - I2C Master Sequential Read Buffer Register 2"]
    #[inline(always)]
    pub const fn i2cm_read_buff2(&self) -> &I2cmReadBuff2 {
        &self.i2cm_read_buff2
    }
    #[doc = "0x7e23 - I2C Master Sequential Read Buffer Register 3"]
    #[inline(always)]
    pub const fn i2cm_read_buff3(&self) -> &I2cmReadBuff3 {
        &self.i2cm_read_buff3
    }
    #[doc = "0x7e24 - I2C Master Sequential Read Buffer Register 4"]
    #[inline(always)]
    pub const fn i2cm_read_buff4(&self) -> &I2cmReadBuff4 {
        &self.i2cm_read_buff4
    }
    #[doc = "0x7e25 - I2C Master Sequential Read Buffer Register 5"]
    #[inline(always)]
    pub const fn i2cm_read_buff5(&self) -> &I2cmReadBuff5 {
        &self.i2cm_read_buff5
    }
    #[doc = "0x7e26 - I2C Master Sequential Read Buffer Register 6"]
    #[inline(always)]
    pub const fn i2cm_read_buff6(&self) -> &I2cmReadBuff6 {
        &self.i2cm_read_buff6
    }
    #[doc = "0x7e27 - I2C Master Sequential Read Buffer Register 7"]
    #[inline(always)]
    pub const fn i2cm_read_buff7(&self) -> &I2cmReadBuff7 {
        &self.i2cm_read_buff7
    }
    #[doc = "0x7e30 - I2C SCDC Read Update Register 0"]
    #[inline(always)]
    pub const fn i2cm_scdc_update0(&self) -> &I2cmScdcUpdate0 {
        &self.i2cm_scdc_update0
    }
    #[doc = "0x7e31 - I2C SCDC Read Update Register 1"]
    #[inline(always)]
    pub const fn i2cm_scdc_update1(&self) -> &I2cmScdcUpdate1 {
        &self.i2cm_scdc_update1
    }
}
#[doc = "DESIGN_ID (r) register accessor: Design Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`design_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@design_id`]
module"]
#[doc(alias = "DESIGN_ID")]
pub type DesignId = crate::Reg<design_id::DesignIdSpec>;
#[doc = "Design Identification Register"]
pub mod design_id;
#[doc = "REVISION_ID (r) register accessor: Revision Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`revision_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@revision_id`]
module"]
#[doc(alias = "REVISION_ID")]
pub type RevisionId = crate::Reg<revision_id::RevisionIdSpec>;
#[doc = "Revision Identification Register"]
pub mod revision_id;
#[doc = "PRODUCT_ID0 (r) register accessor: Product Identification Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`product_id0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@product_id0`]
module"]
#[doc(alias = "PRODUCT_ID0")]
pub type ProductId0 = crate::Reg<product_id0::ProductId0Spec>;
#[doc = "Product Identification Register 0"]
pub mod product_id0;
#[doc = "PRODUCT_ID1 (r) register accessor: Product Identification Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`product_id1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@product_id1`]
module"]
#[doc(alias = "PRODUCT_ID1")]
pub type ProductId1 = crate::Reg<product_id1::ProductId1Spec>;
#[doc = "Product Identification Register 1"]
pub mod product_id1;
#[doc = "CONFIG0_ID (r) register accessor: Configuration Identification Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config0_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config0_id`]
module"]
#[doc(alias = "CONFIG0_ID")]
pub type Config0Id = crate::Reg<config0_id::Config0IdSpec>;
#[doc = "Configuration Identification Register 0"]
pub mod config0_id;
#[doc = "CONFIG1_ID (r) register accessor: Configuration Identification Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config1_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config1_id`]
module"]
#[doc(alias = "CONFIG1_ID")]
pub type Config1Id = crate::Reg<config1_id::Config1IdSpec>;
#[doc = "Configuration Identification Register 1"]
pub mod config1_id;
#[doc = "CONFIG2_ID (r) register accessor: Configuration Identification Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config2_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config2_id`]
module"]
#[doc(alias = "CONFIG2_ID")]
pub type Config2Id = crate::Reg<config2_id::Config2IdSpec>;
#[doc = "Configuration Identification Register 2"]
pub mod config2_id;
#[doc = "CONFIG3_ID (r) register accessor: Configuration Identification Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config3_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config3_id`]
module"]
#[doc(alias = "CONFIG3_ID")]
pub type Config3Id = crate::Reg<config3_id::Config3IdSpec>;
#[doc = "Configuration Identification Register 3"]
pub mod config3_id;
#[doc = "IH_FC_STAT0 (rw) register accessor: Frame Composer Interrupt Status Register 0 (Packet Interrupts)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_fc_stat0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_fc_stat0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ih_fc_stat0`]
module"]
#[doc(alias = "IH_FC_STAT0")]
pub type IhFcStat0 = crate::Reg<ih_fc_stat0::IhFcStat0Spec>;
#[doc = "Frame Composer Interrupt Status Register 0 (Packet Interrupts)"]
pub mod ih_fc_stat0;
#[doc = "IH_FC_STAT1 (rw) register accessor: Frame Composer Interrupt Status Register 1 (Packet Interrupts)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_fc_stat1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_fc_stat1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ih_fc_stat1`]
module"]
#[doc(alias = "IH_FC_STAT1")]
pub type IhFcStat1 = crate::Reg<ih_fc_stat1::IhFcStat1Spec>;
#[doc = "Frame Composer Interrupt Status Register 1 (Packet Interrupts)"]
pub mod ih_fc_stat1;
#[doc = "IH_FC_STAT2 (rw) register accessor: Frame Composer Interrupt Status Register 2 (Packet Interrupts)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_fc_stat2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_fc_stat2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ih_fc_stat2`]
module"]
#[doc(alias = "IH_FC_STAT2")]
pub type IhFcStat2 = crate::Reg<ih_fc_stat2::IhFcStat2Spec>;
#[doc = "Frame Composer Interrupt Status Register 2 (Packet Interrupts)"]
pub mod ih_fc_stat2;
#[doc = "IH_AS_STAT0 (rw) register accessor: Audio Sampler Interrupt Status Register (FIFO Threshold, Underflow and\n\nOverflow Interrupts)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_as_stat0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_as_stat0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ih_as_stat0`]
module"]
#[doc(alias = "IH_AS_STAT0")]
pub type IhAsStat0 = crate::Reg<ih_as_stat0::IhAsStat0Spec>;
#[doc = "Audio Sampler Interrupt Status Register (FIFO Threshold, Underflow and\n\nOverflow Interrupts)"]
pub mod ih_as_stat0;
#[doc = "IH_PHY_STAT0 (rw) register accessor: PHY Interface Interrupt Status Register (RXSENSE, PLL Lock and HPD\n\nInterrupts)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_phy_stat0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_phy_stat0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ih_phy_stat0`]
module"]
#[doc(alias = "IH_PHY_STAT0")]
pub type IhPhyStat0 = crate::Reg<ih_phy_stat0::IhPhyStat0Spec>;
#[doc = "PHY Interface Interrupt Status Register (RXSENSE, PLL Lock and HPD\n\nInterrupts)"]
pub mod ih_phy_stat0;
#[doc = "IH_I2CM_STAT0 (rw) register accessor: E-DDC I2C Master Interrupt Status Register (Done and Error Interrupts)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_i2cm_stat0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_i2cm_stat0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ih_i2cm_stat0`]
module"]
#[doc(alias = "IH_I2CM_STAT0")]
pub type IhI2cmStat0 = crate::Reg<ih_i2cm_stat0::IhI2cmStat0Spec>;
#[doc = "E-DDC I2C Master Interrupt Status Register (Done and Error Interrupts)"]
pub mod ih_i2cm_stat0;
#[doc = "IH_CEC_STAT0 (rw) register accessor: CEC Interrupt Status Register (Functional Operation Interrupts)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_cec_stat0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_cec_stat0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ih_cec_stat0`]
module"]
#[doc(alias = "IH_CEC_STAT0")]
pub type IhCecStat0 = crate::Reg<ih_cec_stat0::IhCecStat0Spec>;
#[doc = "CEC Interrupt Status Register (Functional Operation Interrupts)"]
pub mod ih_cec_stat0;
#[doc = "IH_VP_STAT0 (rw) register accessor: Video Packetizer Interrupt Status Register (FIFO Full and Empty Interrupts)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_vp_stat0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_vp_stat0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ih_vp_stat0`]
module"]
#[doc(alias = "IH_VP_STAT0")]
pub type IhVpStat0 = crate::Reg<ih_vp_stat0::IhVpStat0Spec>;
#[doc = "Video Packetizer Interrupt Status Register (FIFO Full and Empty Interrupts)"]
pub mod ih_vp_stat0;
#[doc = "IH_I2CMPHY_STAT0 (rw) register accessor: PHY GEN2 I2C Master Interrupt Status Register (Done and Error Interrupts)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_i2cmphy_stat0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_i2cmphy_stat0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ih_i2cmphy_stat0`]
module"]
#[doc(alias = "IH_I2CMPHY_STAT0")]
pub type IhI2cmphyStat0 = crate::Reg<ih_i2cmphy_stat0::IhI2cmphyStat0Spec>;
#[doc = "PHY GEN2 I2C Master Interrupt Status Register (Done and Error Interrupts)"]
pub mod ih_i2cmphy_stat0;
#[doc = "IH_AHBDMAAUD_STAT0 (rw) register accessor: AHB Audio DMA Interrupt Status Register (Functional Operation, Buffer Full\n\nand Empty Interrupts)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_ahbdmaaud_stat0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_ahbdmaaud_stat0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ih_ahbdmaaud_stat0`]
module"]
#[doc(alias = "IH_AHBDMAAUD_STAT0")]
pub type IhAhbdmaaudStat0 = crate::Reg<ih_ahbdmaaud_stat0::IhAhbdmaaudStat0Spec>;
#[doc = "AHB Audio DMA Interrupt Status Register (Functional Operation, Buffer Full\n\nand Empty Interrupts)"]
pub mod ih_ahbdmaaud_stat0;
#[doc = "IH_DECODE (r) register accessor: Interruption Handler Decode Assist Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_decode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ih_decode`]
module"]
#[doc(alias = "IH_DECODE")]
pub type IhDecode = crate::Reg<ih_decode::IhDecodeSpec>;
#[doc = "Interruption Handler Decode Assist Register"]
pub mod ih_decode;
#[doc = "IH_MUTE_FC_STAT0 (rw) register accessor: Frame Composer Interrupt Mute Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_mute_fc_stat0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_mute_fc_stat0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ih_mute_fc_stat0`]
module"]
#[doc(alias = "IH_MUTE_FC_STAT0")]
pub type IhMuteFcStat0 = crate::Reg<ih_mute_fc_stat0::IhMuteFcStat0Spec>;
#[doc = "Frame Composer Interrupt Mute Control Register 0"]
pub mod ih_mute_fc_stat0;
#[doc = "IH_MUTE_FC_STAT1 (rw) register accessor: Frame Composer Interrupt Mute Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_mute_fc_stat1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_mute_fc_stat1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ih_mute_fc_stat1`]
module"]
#[doc(alias = "IH_MUTE_FC_STAT1")]
pub type IhMuteFcStat1 = crate::Reg<ih_mute_fc_stat1::IhMuteFcStat1Spec>;
#[doc = "Frame Composer Interrupt Mute Control Register 1"]
pub mod ih_mute_fc_stat1;
#[doc = "IH_MUTE_FC_STAT2 (rw) register accessor: Frame Composer Interrupt Mute Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_mute_fc_stat2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_mute_fc_stat2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ih_mute_fc_stat2`]
module"]
#[doc(alias = "IH_MUTE_FC_STAT2")]
pub type IhMuteFcStat2 = crate::Reg<ih_mute_fc_stat2::IhMuteFcStat2Spec>;
#[doc = "Frame Composer Interrupt Mute Control Register 2"]
pub mod ih_mute_fc_stat2;
#[doc = "IH_MUTE_AS_STAT0 (rw) register accessor: Audio Sampler Interrupt Mute Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_mute_as_stat0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_mute_as_stat0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ih_mute_as_stat0`]
module"]
#[doc(alias = "IH_MUTE_AS_STAT0")]
pub type IhMuteAsStat0 = crate::Reg<ih_mute_as_stat0::IhMuteAsStat0Spec>;
#[doc = "Audio Sampler Interrupt Mute Control Register"]
pub mod ih_mute_as_stat0;
#[doc = "IH_MUTE_PHY_STAT0 (rw) register accessor: PHY Interface Interrupt Mute Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_mute_phy_stat0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_mute_phy_stat0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ih_mute_phy_stat0`]
module"]
#[doc(alias = "IH_MUTE_PHY_STAT0")]
pub type IhMutePhyStat0 = crate::Reg<ih_mute_phy_stat0::IhMutePhyStat0Spec>;
#[doc = "PHY Interface Interrupt Mute Control Register"]
pub mod ih_mute_phy_stat0;
#[doc = "IH_MUTE_I2CM_STAT0 (rw) register accessor: E-DDC I2C Master Interrupt Mute Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_mute_i2cm_stat0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_mute_i2cm_stat0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ih_mute_i2cm_stat0`]
module"]
#[doc(alias = "IH_MUTE_I2CM_STAT0")]
pub type IhMuteI2cmStat0 = crate::Reg<ih_mute_i2cm_stat0::IhMuteI2cmStat0Spec>;
#[doc = "E-DDC I2C Master Interrupt Mute Control Register"]
pub mod ih_mute_i2cm_stat0;
#[doc = "IH_MUTE_CEC_STAT0 (rw) register accessor: CEC Interrupt Mute Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_mute_cec_stat0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_mute_cec_stat0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ih_mute_cec_stat0`]
module"]
#[doc(alias = "IH_MUTE_CEC_STAT0")]
pub type IhMuteCecStat0 = crate::Reg<ih_mute_cec_stat0::IhMuteCecStat0Spec>;
#[doc = "CEC Interrupt Mute Control Register"]
pub mod ih_mute_cec_stat0;
#[doc = "IH_MUTE_VP_STAT0 (rw) register accessor: Video Packetizer Interrupt Mute Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_mute_vp_stat0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_mute_vp_stat0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ih_mute_vp_stat0`]
module"]
#[doc(alias = "IH_MUTE_VP_STAT0")]
pub type IhMuteVpStat0 = crate::Reg<ih_mute_vp_stat0::IhMuteVpStat0Spec>;
#[doc = "Video Packetizer Interrupt Mute Control Register"]
pub mod ih_mute_vp_stat0;
#[doc = "IH_MUTE_I2CMPHY_STAT0 (rw) register accessor: PHY GEN2 I2C Master Interrupt Mute Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_mute_i2cmphy_stat0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_mute_i2cmphy_stat0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ih_mute_i2cmphy_stat0`]
module"]
#[doc(alias = "IH_MUTE_I2CMPHY_STAT0")]
pub type IhMuteI2cmphyStat0 = crate::Reg<ih_mute_i2cmphy_stat0::IhMuteI2cmphyStat0Spec>;
#[doc = "PHY GEN2 I2C Master Interrupt Mute Control Register"]
pub mod ih_mute_i2cmphy_stat0;
#[doc = "IH_MUTE_AHBDMAAUD_STAT0 (rw) register accessor: AHB Audio DMA Interrupt Mute Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_mute_ahbdmaaud_stat0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_mute_ahbdmaaud_stat0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ih_mute_ahbdmaaud_stat0`]
module"]
#[doc(alias = "IH_MUTE_AHBDMAAUD_STAT0")]
pub type IhMuteAhbdmaaudStat0 = crate::Reg<ih_mute_ahbdmaaud_stat0::IhMuteAhbdmaaudStat0Spec>;
#[doc = "AHB Audio DMA Interrupt Mute Control Register"]
pub mod ih_mute_ahbdmaaud_stat0;
#[doc = "IH_MUTE (rw) register accessor: Global Interrupt Mute Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_mute::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_mute::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ih_mute`]
module"]
#[doc(alias = "IH_MUTE")]
pub type IhMute = crate::Reg<ih_mute::IhMuteSpec>;
#[doc = "Global Interrupt Mute Control Register"]
pub mod ih_mute;
#[doc = "TX_INVID0 (rw) register accessor: Video Input Mapping and Internal Data Enable Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_invid0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_invid0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_invid0`]
module"]
#[doc(alias = "TX_INVID0")]
pub type TxInvid0 = crate::Reg<tx_invid0::TxInvid0Spec>;
#[doc = "Video Input Mapping and Internal Data Enable Configuration Register"]
pub mod tx_invid0;
#[doc = "TX_INSTUFFING (rw) register accessor: Video Input Stuffing Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_instuffing::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_instuffing::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_instuffing`]
module"]
#[doc(alias = "TX_INSTUFFING")]
pub type TxInstuffing = crate::Reg<tx_instuffing::TxInstuffingSpec>;
#[doc = "Video Input Stuffing Enable Register"]
pub mod tx_instuffing;
#[doc = "TX_GYDATA0 (rw) register accessor: Video Input gy Data Channel Stuffing Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_gydata0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_gydata0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_gydata0`]
module"]
#[doc(alias = "TX_GYDATA0")]
pub type TxGydata0 = crate::Reg<tx_gydata0::TxGydata0Spec>;
#[doc = "Video Input gy Data Channel Stuffing Register 0"]
pub mod tx_gydata0;
#[doc = "TX_GYDATA1 (rw) register accessor: Video Input gy Data Channel Stuffing Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_gydata1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_gydata1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_gydata1`]
module"]
#[doc(alias = "TX_GYDATA1")]
pub type TxGydata1 = crate::Reg<tx_gydata1::TxGydata1Spec>;
#[doc = "Video Input gy Data Channel Stuffing Register 1"]
pub mod tx_gydata1;
#[doc = "TX_RCRDATA0 (rw) register accessor: Video Input rcr Data Channel Stuffing Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_rcrdata0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_rcrdata0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_rcrdata0`]
module"]
#[doc(alias = "TX_RCRDATA0")]
pub type TxRcrdata0 = crate::Reg<tx_rcrdata0::TxRcrdata0Spec>;
#[doc = "Video Input rcr Data Channel Stuffing Register 0"]
pub mod tx_rcrdata0;
#[doc = "TX_RCRDATA1 (rw) register accessor: Video Input rcr Data Channel Stuffing Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_rcrdata1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_rcrdata1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_rcrdata1`]
module"]
#[doc(alias = "TX_RCRDATA1")]
pub type TxRcrdata1 = crate::Reg<tx_rcrdata1::TxRcrdata1Spec>;
#[doc = "Video Input rcr Data Channel Stuffing Register 1"]
pub mod tx_rcrdata1;
#[doc = "TX_BCBDATA0 (rw) register accessor: Video Input bcb Data Channel Stuffing Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_bcbdata0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_bcbdata0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_bcbdata0`]
module"]
#[doc(alias = "TX_BCBDATA0")]
pub type TxBcbdata0 = crate::Reg<tx_bcbdata0::TxBcbdata0Spec>;
#[doc = "Video Input bcb Data Channel Stuffing Register 0"]
pub mod tx_bcbdata0;
#[doc = "TX_BCBDATA1 (rw) register accessor: Video Input bcb Data Channel Stuffing Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_bcbdata1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_bcbdata1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_bcbdata1`]
module"]
#[doc(alias = "TX_BCBDATA1")]
pub type TxBcbdata1 = crate::Reg<tx_bcbdata1::TxBcbdata1Spec>;
#[doc = "Video Input bcb Data Channel Stuffing Register 1"]
pub mod tx_bcbdata1;
#[doc = "VP_STATUS (r) register accessor: Video Packetizer Packing Phase Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vp_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vp_status`]
module"]
#[doc(alias = "VP_STATUS")]
pub type VpStatus = crate::Reg<vp_status::VpStatusSpec>;
#[doc = "Video Packetizer Packing Phase Status Register"]
pub mod vp_status;
#[doc = "VP_PR_CD (rw) register accessor: Video Packetizer Pixel Repetition and Color Depth Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vp_pr_cd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vp_pr_cd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vp_pr_cd`]
module"]
#[doc(alias = "VP_PR_CD")]
pub type VpPrCd = crate::Reg<vp_pr_cd::VpPrCdSpec>;
#[doc = "Video Packetizer Pixel Repetition and Color Depth Register"]
pub mod vp_pr_cd;
#[doc = "VP_STUFF (rw) register accessor: Video Packetizer Stuffing and Default Packing Phase Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vp_stuff::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vp_stuff::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vp_stuff`]
module"]
#[doc(alias = "VP_STUFF")]
pub type VpStuff = crate::Reg<vp_stuff::VpStuffSpec>;
#[doc = "Video Packetizer Stuffing and Default Packing Phase Register"]
pub mod vp_stuff;
#[doc = "VP_REMAP (rw) register accessor: Video Packetizer YCC422 Remapping Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vp_remap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vp_remap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vp_remap`]
module"]
#[doc(alias = "VP_REMAP")]
pub type VpRemap = crate::Reg<vp_remap::VpRemapSpec>;
#[doc = "Video Packetizer YCC422 Remapping Register"]
pub mod vp_remap;
#[doc = "VP_CONF (rw) register accessor: Video Packetizer Output and Enable Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vp_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vp_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vp_conf`]
module"]
#[doc(alias = "VP_CONF")]
pub type VpConf = crate::Reg<vp_conf::VpConfSpec>;
#[doc = "Video Packetizer Output and Enable Configuration Register"]
pub mod vp_conf;
#[doc = "VP_MASK (rw) register accessor: Video Packetizer Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vp_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vp_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vp_mask`]
module"]
#[doc(alias = "VP_MASK")]
pub type VpMask = crate::Reg<vp_mask::VpMaskSpec>;
#[doc = "Video Packetizer Interrupt Mask Register"]
pub mod vp_mask;
#[doc = "FC_INVIDCONF (rw) register accessor: Frame Composer Input Video Configuration and HDCP Keepout Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_invidconf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_invidconf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_invidconf`]
module"]
#[doc(alias = "FC_INVIDCONF")]
pub type FcInvidconf = crate::Reg<fc_invidconf::FcInvidconfSpec>;
#[doc = "Frame Composer Input Video Configuration and HDCP Keepout Register"]
pub mod fc_invidconf;
#[doc = "FC_INHACTIV0 (rw) register accessor: Frame Composer Input Video HActive Pixels Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_inhactiv0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_inhactiv0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_inhactiv0`]
module"]
#[doc(alias = "FC_INHACTIV0")]
pub type FcInhactiv0 = crate::Reg<fc_inhactiv0::FcInhactiv0Spec>;
#[doc = "Frame Composer Input Video HActive Pixels Register 0"]
pub mod fc_inhactiv0;
#[doc = "FC_INHACTIV1 (rw) register accessor: Frame Composer Input Video HActive Pixels Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_inhactiv1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_inhactiv1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_inhactiv1`]
module"]
#[doc(alias = "FC_INHACTIV1")]
pub type FcInhactiv1 = crate::Reg<fc_inhactiv1::FcInhactiv1Spec>;
#[doc = "Frame Composer Input Video HActive Pixels Register 1"]
pub mod fc_inhactiv1;
#[doc = "FC_INHBLANK0 (rw) register accessor: Frame Composer Input Video HBlank Pixels Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_inhblank0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_inhblank0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_inhblank0`]
module"]
#[doc(alias = "FC_INHBLANK0")]
pub type FcInhblank0 = crate::Reg<fc_inhblank0::FcInhblank0Spec>;
#[doc = "Frame Composer Input Video HBlank Pixels Register 0"]
pub mod fc_inhblank0;
#[doc = "FC_INHBLANK1 (rw) register accessor: Frame Composer Input Video HBlank Pixels Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_inhblank1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_inhblank1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_inhblank1`]
module"]
#[doc(alias = "FC_INHBLANK1")]
pub type FcInhblank1 = crate::Reg<fc_inhblank1::FcInhblank1Spec>;
#[doc = "Frame Composer Input Video HBlank Pixels Register 1"]
pub mod fc_inhblank1;
#[doc = "FC_INVACTIV0 (rw) register accessor: Frame Composer Input Video VActive Pixels Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_invactiv0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_invactiv0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_invactiv0`]
module"]
#[doc(alias = "FC_INVACTIV0")]
pub type FcInvactiv0 = crate::Reg<fc_invactiv0::FcInvactiv0Spec>;
#[doc = "Frame Composer Input Video VActive Pixels Register 0"]
pub mod fc_invactiv0;
#[doc = "FC_INVACTIV1 (rw) register accessor: Frame Composer Input Video VActive Pixels Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_invactiv1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_invactiv1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_invactiv1`]
module"]
#[doc(alias = "FC_INVACTIV1")]
pub type FcInvactiv1 = crate::Reg<fc_invactiv1::FcInvactiv1Spec>;
#[doc = "Frame Composer Input Video VActive Pixels Register 1"]
pub mod fc_invactiv1;
#[doc = "FC_INVBLANK (rw) register accessor: Frame Composer Input Video VBlank Pixels Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_invblank::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_invblank::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_invblank`]
module"]
#[doc(alias = "FC_INVBLANK")]
pub type FcInvblank = crate::Reg<fc_invblank::FcInvblankSpec>;
#[doc = "Frame Composer Input Video VBlank Pixels Register"]
pub mod fc_invblank;
#[doc = "FC_HSYNCINDELAY0 (rw) register accessor: Frame Composer Input Video HSync Front Porch Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_hsyncindelay0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_hsyncindelay0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_hsyncindelay0`]
module"]
#[doc(alias = "FC_HSYNCINDELAY0")]
pub type FcHsyncindelay0 = crate::Reg<fc_hsyncindelay0::FcHsyncindelay0Spec>;
#[doc = "Frame Composer Input Video HSync Front Porch Register 0"]
pub mod fc_hsyncindelay0;
#[doc = "FC_HSYNCINDELAY1 (rw) register accessor: Frame Composer Input Video HSync Front Porch Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_hsyncindelay1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_hsyncindelay1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_hsyncindelay1`]
module"]
#[doc(alias = "FC_HSYNCINDELAY1")]
pub type FcHsyncindelay1 = crate::Reg<fc_hsyncindelay1::FcHsyncindelay1Spec>;
#[doc = "Frame Composer Input Video HSync Front Porch Register 1"]
pub mod fc_hsyncindelay1;
#[doc = "FC_HSYNCINWIDTH0 (rw) register accessor: Frame Composer Input Video HSync Width Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_hsyncinwidth0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_hsyncinwidth0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_hsyncinwidth0`]
module"]
#[doc(alias = "FC_HSYNCINWIDTH0")]
pub type FcHsyncinwidth0 = crate::Reg<fc_hsyncinwidth0::FcHsyncinwidth0Spec>;
#[doc = "Frame Composer Input Video HSync Width Register 0"]
pub mod fc_hsyncinwidth0;
#[doc = "FC_HSYNCINWIDTH1 (rw) register accessor: Frame Composer Input Video HSync Width Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_hsyncinwidth1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_hsyncinwidth1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_hsyncinwidth1`]
module"]
#[doc(alias = "FC_HSYNCINWIDTH1")]
pub type FcHsyncinwidth1 = crate::Reg<fc_hsyncinwidth1::FcHsyncinwidth1Spec>;
#[doc = "Frame Composer Input Video HSync Width Register 1"]
pub mod fc_hsyncinwidth1;
#[doc = "FC_VSYNCINDELAY (rw) register accessor: Frame Composer Input Video VSync Front Porch Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_vsyncindelay::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_vsyncindelay::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_vsyncindelay`]
module"]
#[doc(alias = "FC_VSYNCINDELAY")]
pub type FcVsyncindelay = crate::Reg<fc_vsyncindelay::FcVsyncindelaySpec>;
#[doc = "Frame Composer Input Video VSync Front Porch Register"]
pub mod fc_vsyncindelay;
#[doc = "FC_VSYNCINWIDTH (rw) register accessor: Frame Composer Input Video VSync Width Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_vsyncinwidth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_vsyncinwidth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_vsyncinwidth`]
module"]
#[doc(alias = "FC_VSYNCINWIDTH")]
pub type FcVsyncinwidth = crate::Reg<fc_vsyncinwidth::FcVsyncinwidthSpec>;
#[doc = "Frame Composer Input Video VSync Width Register"]
pub mod fc_vsyncinwidth;
#[doc = "FC_INFREQ0 (rw) register accessor: Frame Composer Input Video Refresh Rate Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_infreq0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_infreq0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_infreq0`]
module"]
#[doc(alias = "FC_INFREQ0")]
pub type FcInfreq0 = crate::Reg<fc_infreq0::FcInfreq0Spec>;
#[doc = "Frame Composer Input Video Refresh Rate Register 0"]
pub mod fc_infreq0;
#[doc = "FC_INFREQ1 (rw) register accessor: Frame Composer Input Video Refresh Rate Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_infreq1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_infreq1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_infreq1`]
module"]
#[doc(alias = "FC_INFREQ1")]
pub type FcInfreq1 = crate::Reg<fc_infreq1::FcInfreq1Spec>;
#[doc = "Frame Composer Input Video Refresh Rate Register 1"]
pub mod fc_infreq1;
#[doc = "FC_INFREQ2 (rw) register accessor: Frame Composer Input Video Refresh Rate Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_infreq2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_infreq2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_infreq2`]
module"]
#[doc(alias = "FC_INFREQ2")]
pub type FcInfreq2 = crate::Reg<fc_infreq2::FcInfreq2Spec>;
#[doc = "Frame Composer Input Video Refresh Rate Register 2"]
pub mod fc_infreq2;
#[doc = "FC_CTRLDUR (rw) register accessor: Frame Composer Control Period Duration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_ctrldur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_ctrldur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_ctrldur`]
module"]
#[doc(alias = "FC_CTRLDUR")]
pub type FcCtrldur = crate::Reg<fc_ctrldur::FcCtrldurSpec>;
#[doc = "Frame Composer Control Period Duration Register"]
pub mod fc_ctrldur;
#[doc = "FC_EXCTRLDUR (rw) register accessor: Frame Composer Extended Control Period Duration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_exctrldur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_exctrldur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_exctrldur`]
module"]
#[doc(alias = "FC_EXCTRLDUR")]
pub type FcExctrldur = crate::Reg<fc_exctrldur::FcExctrldurSpec>;
#[doc = "Frame Composer Extended Control Period Duration Register"]
pub mod fc_exctrldur;
#[doc = "FC_EXCTRLSPAC (rw) register accessor: Frame Composer Extended Control Period Maximum Spacing Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_exctrlspac::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_exctrlspac::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_exctrlspac`]
module"]
#[doc(alias = "FC_EXCTRLSPAC")]
pub type FcExctrlspac = crate::Reg<fc_exctrlspac::FcExctrlspacSpec>;
#[doc = "Frame Composer Extended Control Period Maximum Spacing Register"]
pub mod fc_exctrlspac;
#[doc = "FC_CH0PREAM (rw) register accessor: Frame Composer Channel 0 Non-Preamble Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_ch0pream::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_ch0pream::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_ch0pream`]
module"]
#[doc(alias = "FC_CH0PREAM")]
pub type FcCh0pream = crate::Reg<fc_ch0pream::FcCh0preamSpec>;
#[doc = "Frame Composer Channel 0 Non-Preamble Data Register"]
pub mod fc_ch0pream;
#[doc = "FC_CH1PREAM (rw) register accessor: Frame Composer Channel 1 Non-Preamble Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_ch1pream::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_ch1pream::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_ch1pream`]
module"]
#[doc(alias = "FC_CH1PREAM")]
pub type FcCh1pream = crate::Reg<fc_ch1pream::FcCh1preamSpec>;
#[doc = "Frame Composer Channel 1 Non-Preamble Data Register"]
pub mod fc_ch1pream;
#[doc = "FC_CH2PREAM (rw) register accessor: Frame Composer Channel 2 Non-Preamble Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_ch2pream::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_ch2pream::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_ch2pream`]
module"]
#[doc(alias = "FC_CH2PREAM")]
pub type FcCh2pream = crate::Reg<fc_ch2pream::FcCh2preamSpec>;
#[doc = "Frame Composer Channel 2 Non-Preamble Data Register"]
pub mod fc_ch2pream;
#[doc = "FC_AVICONF3 (rw) register accessor: Frame Composer AVI Packet Configuration Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_aviconf3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_aviconf3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_aviconf3`]
module"]
#[doc(alias = "FC_AVICONF3")]
pub type FcAviconf3 = crate::Reg<fc_aviconf3::FcAviconf3Spec>;
#[doc = "Frame Composer AVI Packet Configuration Register 3"]
pub mod fc_aviconf3;
#[doc = "FC_GCP (rw) register accessor: Frame Composer GCP Packet Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_gcp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_gcp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_gcp`]
module"]
#[doc(alias = "FC_GCP")]
pub type FcGcp = crate::Reg<fc_gcp::FcGcpSpec>;
#[doc = "Frame Composer GCP Packet Configuration Register"]
pub mod fc_gcp;
#[doc = "FC_AVICONF0 (rw) register accessor: Frame Composer AVI Packet Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_aviconf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_aviconf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_aviconf0`]
module"]
#[doc(alias = "FC_AVICONF0")]
pub type FcAviconf0 = crate::Reg<fc_aviconf0::FcAviconf0Spec>;
#[doc = "Frame Composer AVI Packet Configuration Register 0"]
pub mod fc_aviconf0;
#[doc = "FC_AVICONF1 (rw) register accessor: Frame Composer AVI Packet Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_aviconf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_aviconf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_aviconf1`]
module"]
#[doc(alias = "FC_AVICONF1")]
pub type FcAviconf1 = crate::Reg<fc_aviconf1::FcAviconf1Spec>;
#[doc = "Frame Composer AVI Packet Configuration Register 1"]
pub mod fc_aviconf1;
#[doc = "FC_AVICONF2 (rw) register accessor: Frame Composer AVI Packet Configuration Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_aviconf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_aviconf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_aviconf2`]
module"]
#[doc(alias = "FC_AVICONF2")]
pub type FcAviconf2 = crate::Reg<fc_aviconf2::FcAviconf2Spec>;
#[doc = "Frame Composer AVI Packet Configuration Register 2"]
pub mod fc_aviconf2;
#[doc = "FC_AVIVID (rw) register accessor: Frame Composer AVI Packet VIC Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_avivid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_avivid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_avivid`]
module"]
#[doc(alias = "FC_AVIVID")]
pub type FcAvivid = crate::Reg<fc_avivid::FcAvividSpec>;
#[doc = "Frame Composer AVI Packet VIC Register"]
pub mod fc_avivid;
#[doc = "FC_AVIETB (rw) register accessor: Frame Composer AVI Packet End of Top Bar Register Array\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_avietb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_avietb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_avietb`]
module"]
#[doc(alias = "FC_AVIETB")]
pub type FcAvietb = crate::Reg<fc_avietb::FcAvietbSpec>;
#[doc = "Frame Composer AVI Packet End of Top Bar Register Array"]
pub mod fc_avietb;
#[doc = "FC_AVISBB (rw) register accessor: Frame Composer AVI Packet Start of Bottom Bar Register Array\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_avisbb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_avisbb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_avisbb`]
module"]
#[doc(alias = "FC_AVISBB")]
pub type FcAvisbb = crate::Reg<fc_avisbb::FcAvisbbSpec>;
#[doc = "Frame Composer AVI Packet Start of Bottom Bar Register Array"]
pub mod fc_avisbb;
#[doc = "FC_AVIELB (rw) register accessor: Frame Composer AVI Packet End of Left Bar Register Array\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_avielb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_avielb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_avielb`]
module"]
#[doc(alias = "FC_AVIELB")]
pub type FcAvielb = crate::Reg<fc_avielb::FcAvielbSpec>;
#[doc = "Frame Composer AVI Packet End of Left Bar Register Array"]
pub mod fc_avielb;
#[doc = "FC_AVISRB (rw) register accessor: Frame Composer AVI Packet Start of Right Bar Register Array\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_avisrb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_avisrb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_avisrb`]
module"]
#[doc(alias = "FC_AVISRB")]
pub type FcAvisrb = crate::Reg<fc_avisrb::FcAvisrbSpec>;
#[doc = "Frame Composer AVI Packet Start of Right Bar Register Array"]
pub mod fc_avisrb;
#[doc = "FC_AUDICONF0 (rw) register accessor: Frame Composer AUD Packet Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audiconf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_audiconf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_audiconf0`]
module"]
#[doc(alias = "FC_AUDICONF0")]
pub type FcAudiconf0 = crate::Reg<fc_audiconf0::FcAudiconf0Spec>;
#[doc = "Frame Composer AUD Packet Configuration Register 0"]
pub mod fc_audiconf0;
#[doc = "FC_AUDICONF1 (rw) register accessor: Frame Composer AUD Packet Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audiconf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_audiconf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_audiconf1`]
module"]
#[doc(alias = "FC_AUDICONF1")]
pub type FcAudiconf1 = crate::Reg<fc_audiconf1::FcAudiconf1Spec>;
#[doc = "Frame Composer AUD Packet Configuration Register 1"]
pub mod fc_audiconf1;
#[doc = "FC_AUDICONF2 (rw) register accessor: Frame Composer AUD Packet Configuration Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audiconf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_audiconf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_audiconf2`]
module"]
#[doc(alias = "FC_AUDICONF2")]
pub type FcAudiconf2 = crate::Reg<fc_audiconf2::FcAudiconf2Spec>;
#[doc = "Frame Composer AUD Packet Configuration Register 2"]
pub mod fc_audiconf2;
#[doc = "FC_AUDICONF3 (rw) register accessor: Frame Composer AUD Packet Configuration Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audiconf3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_audiconf3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_audiconf3`]
module"]
#[doc(alias = "FC_AUDICONF3")]
pub type FcAudiconf3 = crate::Reg<fc_audiconf3::FcAudiconf3Spec>;
#[doc = "Frame Composer AUD Packet Configuration Register 3"]
pub mod fc_audiconf3;
#[doc = "FC_VSDIEEEID2 (rw) register accessor: Frame Composer VSI Packet Data IEEE Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_vsdieeeid2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_vsdieeeid2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_vsdieeeid2`]
module"]
#[doc(alias = "FC_VSDIEEEID2")]
pub type FcVsdieeeid2 = crate::Reg<fc_vsdieeeid2::FcVsdieeeid2Spec>;
#[doc = "Frame Composer VSI Packet Data IEEE Register 2"]
pub mod fc_vsdieeeid2;
#[doc = "FC_VSDSIZE (rw) register accessor: Frame Composer VSI Packet Data Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_vsdsize::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_vsdsize::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_vsdsize`]
module"]
#[doc(alias = "FC_VSDSIZE")]
pub type FcVsdsize = crate::Reg<fc_vsdsize::FcVsdsizeSpec>;
#[doc = "Frame Composer VSI Packet Data Size Register"]
pub mod fc_vsdsize;
#[doc = "FC_VSDIEEEID1 (rw) register accessor: Frame Composer VSI Packet Data IEEE Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_vsdieeeid1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_vsdieeeid1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_vsdieeeid1`]
module"]
#[doc(alias = "FC_VSDIEEEID1")]
pub type FcVsdieeeid1 = crate::Reg<fc_vsdieeeid1::FcVsdieeeid1Spec>;
#[doc = "Frame Composer VSI Packet Data IEEE Register 1"]
pub mod fc_vsdieeeid1;
#[doc = "FC_VSDIEEEID0 (rw) register accessor: Frame Composer VSI Packet Data IEEE Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_vsdieeeid0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_vsdieeeid0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_vsdieeeid0`]
module"]
#[doc(alias = "FC_VSDIEEEID0")]
pub type FcVsdieeeid0 = crate::Reg<fc_vsdieeeid0::FcVsdieeeid0Spec>;
#[doc = "Frame Composer VSI Packet Data IEEE Register 0"]
pub mod fc_vsdieeeid0;
#[doc = "FC_VSDPAYLOAD (rw) register accessor: Frame Composer VSI Packet Data Payload Register Array\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_vsdpayload::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_vsdpayload::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_vsdpayload`]
module"]
#[doc(alias = "FC_VSDPAYLOAD")]
pub type FcVsdpayload = crate::Reg<fc_vsdpayload::FcVsdpayloadSpec>;
#[doc = "Frame Composer VSI Packet Data Payload Register Array"]
pub mod fc_vsdpayload;
#[doc = "FC_SPDVENDORNAME (rw) register accessor: Frame Composer SPD Packet Data Vendor Name Register Array\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_spdvendorname::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_spdvendorname::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_spdvendorname`]
module"]
#[doc(alias = "FC_SPDVENDORNAME")]
pub type FcSpdvendorname = crate::Reg<fc_spdvendorname::FcSpdvendornameSpec>;
#[doc = "Frame Composer SPD Packet Data Vendor Name Register Array"]
pub mod fc_spdvendorname;
#[doc = "FC_SPDPRODUCTNAME (rw) register accessor: Frame Composer SPD packet Data Product Name Register Array\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_spdproductname::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_spdproductname::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_spdproductname`]
module"]
#[doc(alias = "FC_SPDPRODUCTNAME")]
pub type FcSpdproductname = crate::Reg<fc_spdproductname::FcSpdproductnameSpec>;
#[doc = "Frame Composer SPD packet Data Product Name Register Array"]
pub mod fc_spdproductname;
#[doc = "FC_SPDDEVICEINF (rw) register accessor: Frame Composer SPD Packet Data Source Product Descriptor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_spddeviceinf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_spddeviceinf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_spddeviceinf`]
module"]
#[doc(alias = "FC_SPDDEVICEINF")]
pub type FcSpddeviceinf = crate::Reg<fc_spddeviceinf::FcSpddeviceinfSpec>;
#[doc = "Frame Composer SPD Packet Data Source Product Descriptor Register"]
pub mod fc_spddeviceinf;
#[doc = "FC_AUDSCONF (rw) register accessor: Frame Composer Audio Sample Flat and Layout Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audsconf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_audsconf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_audsconf`]
module"]
#[doc(alias = "FC_AUDSCONF")]
pub type FcAudsconf = crate::Reg<fc_audsconf::FcAudsconfSpec>;
#[doc = "Frame Composer Audio Sample Flat and Layout Configuration Register"]
pub mod fc_audsconf;
#[doc = "FC_AUDSSTAT (r) register accessor: Frame Composer Audio Sample Flat and Layout Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audsstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_audsstat`]
module"]
#[doc(alias = "FC_AUDSSTAT")]
pub type FcAudsstat = crate::Reg<fc_audsstat::FcAudsstatSpec>;
#[doc = "Frame Composer Audio Sample Flat and Layout Status Register"]
pub mod fc_audsstat;
#[doc = "FC_AUDSV (rw) register accessor: Frame Composer Audio Sample Validity Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audsv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_audsv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_audsv`]
module"]
#[doc(alias = "FC_AUDSV")]
pub type FcAudsv = crate::Reg<fc_audsv::FcAudsvSpec>;
#[doc = "Frame Composer Audio Sample Validity Flag Register"]
pub mod fc_audsv;
#[doc = "FC_AUDSU (rw) register accessor: Frame Composer Audio Sample User Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audsu::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_audsu::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_audsu`]
module"]
#[doc(alias = "FC_AUDSU")]
pub type FcAudsu = crate::Reg<fc_audsu::FcAudsuSpec>;
#[doc = "Frame Composer Audio Sample User Flag Register"]
pub mod fc_audsu;
#[doc = "FC_AUDSCHNL0 (rw) register accessor: Frame Composer Audio Sample Channel Status Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audschnl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_audschnl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_audschnl0`]
module"]
#[doc(alias = "FC_AUDSCHNL0")]
pub type FcAudschnl0 = crate::Reg<fc_audschnl0::FcAudschnl0Spec>;
#[doc = "Frame Composer Audio Sample Channel Status Configuration Register 0"]
pub mod fc_audschnl0;
#[doc = "FC_AUDSCHNL1 (rw) register accessor: Frame Composer Audio Sample Channel Status Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audschnl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_audschnl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_audschnl1`]
module"]
#[doc(alias = "FC_AUDSCHNL1")]
pub type FcAudschnl1 = crate::Reg<fc_audschnl1::FcAudschnl1Spec>;
#[doc = "Frame Composer Audio Sample Channel Status Configuration Register 1"]
pub mod fc_audschnl1;
#[doc = "FC_AUDSCHNL2 (rw) register accessor: Frame Composer Audio Sample Channel Status Configuration Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audschnl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_audschnl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_audschnl2`]
module"]
#[doc(alias = "FC_AUDSCHNL2")]
pub type FcAudschnl2 = crate::Reg<fc_audschnl2::FcAudschnl2Spec>;
#[doc = "Frame Composer Audio Sample Channel Status Configuration Register 2"]
pub mod fc_audschnl2;
#[doc = "FC_AUDSCHNL3 (rw) register accessor: Frame Composer Audio Sample Channel Status Configuration Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audschnl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_audschnl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_audschnl3`]
module"]
#[doc(alias = "FC_AUDSCHNL3")]
pub type FcAudschnl3 = crate::Reg<fc_audschnl3::FcAudschnl3Spec>;
#[doc = "Frame Composer Audio Sample Channel Status Configuration Register 3"]
pub mod fc_audschnl3;
#[doc = "FC_AUDSCHNL4 (rw) register accessor: Frame Composer Audio Sample Channel Status Configuration Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audschnl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_audschnl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_audschnl4`]
module"]
#[doc(alias = "FC_AUDSCHNL4")]
pub type FcAudschnl4 = crate::Reg<fc_audschnl4::FcAudschnl4Spec>;
#[doc = "Frame Composer Audio Sample Channel Status Configuration Register 4"]
pub mod fc_audschnl4;
#[doc = "FC_AUDSCHNL5 (rw) register accessor: Frame Composer Audio Sample Channel Status Configuration Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audschnl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_audschnl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_audschnl5`]
module"]
#[doc(alias = "FC_AUDSCHNL5")]
pub type FcAudschnl5 = crate::Reg<fc_audschnl5::FcAudschnl5Spec>;
#[doc = "Frame Composer Audio Sample Channel Status Configuration Register 5"]
pub mod fc_audschnl5;
#[doc = "FC_AUDSCHNL6 (rw) register accessor: Frame Composer Audio Sample Channel Status Configuration Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audschnl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_audschnl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_audschnl6`]
module"]
#[doc(alias = "FC_AUDSCHNL6")]
pub type FcAudschnl6 = crate::Reg<fc_audschnl6::FcAudschnl6Spec>;
#[doc = "Frame Composer Audio Sample Channel Status Configuration Register 6"]
pub mod fc_audschnl6;
#[doc = "FC_AUDSCHNL7 (rw) register accessor: Frame Composer Audio Sample Channel Status Configuration Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audschnl7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_audschnl7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_audschnl7`]
module"]
#[doc(alias = "FC_AUDSCHNL7")]
pub type FcAudschnl7 = crate::Reg<fc_audschnl7::FcAudschnl7Spec>;
#[doc = "Frame Composer Audio Sample Channel Status Configuration Register 7"]
pub mod fc_audschnl7;
#[doc = "FC_AUDSCHNL8 (rw) register accessor: Frame Composer Audio Sample Channel Status Configuration Register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audschnl8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_audschnl8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_audschnl8`]
module"]
#[doc(alias = "FC_AUDSCHNL8")]
pub type FcAudschnl8 = crate::Reg<fc_audschnl8::FcAudschnl8Spec>;
#[doc = "Frame Composer Audio Sample Channel Status Configuration Register 8"]
pub mod fc_audschnl8;
#[doc = "FC_CTRLQHIGH (rw) register accessor: Frame Composer Number of High Priority Packets Attended Configuration\n\nRegister\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_ctrlqhigh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_ctrlqhigh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_ctrlqhigh`]
module"]
#[doc(alias = "FC_CTRLQHIGH")]
pub type FcCtrlqhigh = crate::Reg<fc_ctrlqhigh::FcCtrlqhighSpec>;
#[doc = "Frame Composer Number of High Priority Packets Attended Configuration\n\nRegister"]
pub mod fc_ctrlqhigh;
#[doc = "FC_CTRLQLOW (rw) register accessor: Frame Composer Number of Low Priority Packets Attended Configuration\n\nRegister\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_ctrlqlow::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_ctrlqlow::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_ctrlqlow`]
module"]
#[doc(alias = "FC_CTRLQLOW")]
pub type FcCtrlqlow = crate::Reg<fc_ctrlqlow::FcCtrlqlowSpec>;
#[doc = "Frame Composer Number of Low Priority Packets Attended Configuration\n\nRegister"]
pub mod fc_ctrlqlow;
#[doc = "FC_ACP0 (rw) register accessor: Frame Composer ACP Packet Type Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_acp0`]
module"]
#[doc(alias = "FC_ACP0")]
pub type FcAcp0 = crate::Reg<fc_acp0::FcAcp0Spec>;
#[doc = "Frame Composer ACP Packet Type Configuration Register 0"]
pub mod fc_acp0;
#[doc = "FC_ACP16 (rw) register accessor: Frame Composer ACP Packet Body Configuration Register 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_acp16`]
module"]
#[doc(alias = "FC_ACP16")]
pub type FcAcp16 = crate::Reg<fc_acp16::FcAcp16Spec>;
#[doc = "Frame Composer ACP Packet Body Configuration Register 16"]
pub mod fc_acp16;
#[doc = "FC_ACP15 (rw) register accessor: Frame Composer ACP Packet Body Configuration Register 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_acp15`]
module"]
#[doc(alias = "FC_ACP15")]
pub type FcAcp15 = crate::Reg<fc_acp15::FcAcp15Spec>;
#[doc = "Frame Composer ACP Packet Body Configuration Register 15"]
pub mod fc_acp15;
#[doc = "FC_ACP14 (rw) register accessor: Frame Composer ACP Packet Body Configuration Register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_acp14`]
module"]
#[doc(alias = "FC_ACP14")]
pub type FcAcp14 = crate::Reg<fc_acp14::FcAcp14Spec>;
#[doc = "Frame Composer ACP Packet Body Configuration Register 14"]
pub mod fc_acp14;
#[doc = "FC_ACP13 (rw) register accessor: Frame Composer ACP Packet Body Configuration Register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_acp13`]
module"]
#[doc(alias = "FC_ACP13")]
pub type FcAcp13 = crate::Reg<fc_acp13::FcAcp13Spec>;
#[doc = "Frame Composer ACP Packet Body Configuration Register 13"]
pub mod fc_acp13;
#[doc = "FC_ACP12 (rw) register accessor: Frame Composer ACP Packet Body Configuration Register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_acp12`]
module"]
#[doc(alias = "FC_ACP12")]
pub type FcAcp12 = crate::Reg<fc_acp12::FcAcp12Spec>;
#[doc = "Frame Composer ACP Packet Body Configuration Register 12"]
pub mod fc_acp12;
#[doc = "FC_ACP11 (rw) register accessor: Frame Composer ACP Packet Body Configuration Register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_acp11`]
module"]
#[doc(alias = "FC_ACP11")]
pub type FcAcp11 = crate::Reg<fc_acp11::FcAcp11Spec>;
#[doc = "Frame Composer ACP Packet Body Configuration Register 11"]
pub mod fc_acp11;
#[doc = "FC_ACP10 (rw) register accessor: Frame Composer ACP Packet Body Configuration Register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_acp10`]
module"]
#[doc(alias = "FC_ACP10")]
pub type FcAcp10 = crate::Reg<fc_acp10::FcAcp10Spec>;
#[doc = "Frame Composer ACP Packet Body Configuration Register 10"]
pub mod fc_acp10;
#[doc = "FC_ACP9 (rw) register accessor: Frame Composer ACP Packet Body Configuration Register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_acp9`]
module"]
#[doc(alias = "FC_ACP9")]
pub type FcAcp9 = crate::Reg<fc_acp9::FcAcp9Spec>;
#[doc = "Frame Composer ACP Packet Body Configuration Register 9"]
pub mod fc_acp9;
#[doc = "FC_ACP8 (rw) register accessor: Frame Composer ACP Packet Body Configuration Register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_acp8`]
module"]
#[doc(alias = "FC_ACP8")]
pub type FcAcp8 = crate::Reg<fc_acp8::FcAcp8Spec>;
#[doc = "Frame Composer ACP Packet Body Configuration Register 8"]
pub mod fc_acp8;
#[doc = "FC_ACP7 (rw) register accessor: Frame Composer ACP Packet Body Configuration Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_acp7`]
module"]
#[doc(alias = "FC_ACP7")]
pub type FcAcp7 = crate::Reg<fc_acp7::FcAcp7Spec>;
#[doc = "Frame Composer ACP Packet Body Configuration Register 7"]
pub mod fc_acp7;
#[doc = "FC_ACP6 (rw) register accessor: Frame Composer ACP Packet Body Configuration Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_acp6`]
module"]
#[doc(alias = "FC_ACP6")]
pub type FcAcp6 = crate::Reg<fc_acp6::FcAcp6Spec>;
#[doc = "Frame Composer ACP Packet Body Configuration Register 6"]
pub mod fc_acp6;
#[doc = "FC_ACP5 (rw) register accessor: Frame Composer ACP Packet Body Configuration Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_acp5`]
module"]
#[doc(alias = "FC_ACP5")]
pub type FcAcp5 = crate::Reg<fc_acp5::FcAcp5Spec>;
#[doc = "Frame Composer ACP Packet Body Configuration Register 5"]
pub mod fc_acp5;
#[doc = "FC_ACP4 (rw) register accessor: Frame Composer ACP Packet Body Configuration Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_acp4`]
module"]
#[doc(alias = "FC_ACP4")]
pub type FcAcp4 = crate::Reg<fc_acp4::FcAcp4Spec>;
#[doc = "Frame Composer ACP Packet Body Configuration Register 4"]
pub mod fc_acp4;
#[doc = "FC_ACP3 (rw) register accessor: Frame Composer ACP Packet Body Configuration Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_acp3`]
module"]
#[doc(alias = "FC_ACP3")]
pub type FcAcp3 = crate::Reg<fc_acp3::FcAcp3Spec>;
#[doc = "Frame Composer ACP Packet Body Configuration Register 3"]
pub mod fc_acp3;
#[doc = "FC_ACP2 (rw) register accessor: Frame Composer ACP Packet Body Configuration Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_acp2`]
module"]
#[doc(alias = "FC_ACP2")]
pub type FcAcp2 = crate::Reg<fc_acp2::FcAcp2Spec>;
#[doc = "Frame Composer ACP Packet Body Configuration Register 2"]
pub mod fc_acp2;
#[doc = "FC_ACP1 (rw) register accessor: Frame Composer ACP Packet Body Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_acp1`]
module"]
#[doc(alias = "FC_ACP1")]
pub type FcAcp1 = crate::Reg<fc_acp1::FcAcp1Spec>;
#[doc = "Frame Composer ACP Packet Body Configuration Register 1"]
pub mod fc_acp1;
#[doc = "FC_ISCR1_0 (rw) register accessor: Frame Composer ISRC1 Packet Status, Valid, and Continue Configuration\n\nRegister\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr1_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr1_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_iscr1_0`]
module"]
#[doc(alias = "FC_ISCR1_0")]
pub type FcIscr1_0 = crate::Reg<fc_iscr1_0::FcIscr1_0Spec>;
#[doc = "Frame Composer ISRC1 Packet Status, Valid, and Continue Configuration\n\nRegister"]
pub mod fc_iscr1_0;
#[doc = "FC_ISCR1_16 (rw) register accessor: Frame Composer ISRC1 Packet Body Register 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr1_16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr1_16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_iscr1_16`]
module"]
#[doc(alias = "FC_ISCR1_16")]
pub type FcIscr1_16 = crate::Reg<fc_iscr1_16::FcIscr1_16Spec>;
#[doc = "Frame Composer ISRC1 Packet Body Register 16"]
pub mod fc_iscr1_16;
#[doc = "FC_ISCR1_15 (rw) register accessor: Frame Composer ISRC1 Packet Body Register 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr1_15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr1_15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_iscr1_15`]
module"]
#[doc(alias = "FC_ISCR1_15")]
pub type FcIscr1_15 = crate::Reg<fc_iscr1_15::FcIscr1_15Spec>;
#[doc = "Frame Composer ISRC1 Packet Body Register 15"]
pub mod fc_iscr1_15;
#[doc = "FC_ISCR1_14 (rw) register accessor: Frame Composer ISRC1 Packet Body Register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr1_14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr1_14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_iscr1_14`]
module"]
#[doc(alias = "FC_ISCR1_14")]
pub type FcIscr1_14 = crate::Reg<fc_iscr1_14::FcIscr1_14Spec>;
#[doc = "Frame Composer ISRC1 Packet Body Register 14"]
pub mod fc_iscr1_14;
#[doc = "FC_ISCR1_13 (rw) register accessor: Frame Composer ISRC1 Packet Body Register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr1_13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr1_13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_iscr1_13`]
module"]
#[doc(alias = "FC_ISCR1_13")]
pub type FcIscr1_13 = crate::Reg<fc_iscr1_13::FcIscr1_13Spec>;
#[doc = "Frame Composer ISRC1 Packet Body Register 13"]
pub mod fc_iscr1_13;
#[doc = "FC_ISCR1_12 (rw) register accessor: Frame Composer ISRC1 Packet Body Register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr1_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr1_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_iscr1_12`]
module"]
#[doc(alias = "FC_ISCR1_12")]
pub type FcIscr1_12 = crate::Reg<fc_iscr1_12::FcIscr1_12Spec>;
#[doc = "Frame Composer ISRC1 Packet Body Register 12"]
pub mod fc_iscr1_12;
#[doc = "FC_ISCR1_11 (rw) register accessor: Frame Composer ISRC1 Packet Body Register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr1_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr1_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_iscr1_11`]
module"]
#[doc(alias = "FC_ISCR1_11")]
pub type FcIscr1_11 = crate::Reg<fc_iscr1_11::FcIscr1_11Spec>;
#[doc = "Frame Composer ISRC1 Packet Body Register 11"]
pub mod fc_iscr1_11;
#[doc = "FC_ISCR1_10 (rw) register accessor: Frame Composer ISRC1 Packet Body Register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr1_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr1_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_iscr1_10`]
module"]
#[doc(alias = "FC_ISCR1_10")]
pub type FcIscr1_10 = crate::Reg<fc_iscr1_10::FcIscr1_10Spec>;
#[doc = "Frame Composer ISRC1 Packet Body Register 10"]
pub mod fc_iscr1_10;
#[doc = "FC_ISCR1_9 (rw) register accessor: Frame Composer ISRC1 Packet Body Register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr1_9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr1_9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_iscr1_9`]
module"]
#[doc(alias = "FC_ISCR1_9")]
pub type FcIscr1_9 = crate::Reg<fc_iscr1_9::FcIscr1_9Spec>;
#[doc = "Frame Composer ISRC1 Packet Body Register 9"]
pub mod fc_iscr1_9;
#[doc = "FC_ISCR1_8 (rw) register accessor: Frame Composer ISRC1 Packet Body Register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr1_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr1_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_iscr1_8`]
module"]
#[doc(alias = "FC_ISCR1_8")]
pub type FcIscr1_8 = crate::Reg<fc_iscr1_8::FcIscr1_8Spec>;
#[doc = "Frame Composer ISRC1 Packet Body Register 8"]
pub mod fc_iscr1_8;
#[doc = "FC_ISCR1_7 (rw) register accessor: Frame Composer ISRC1 Packet Body Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr1_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr1_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_iscr1_7`]
module"]
#[doc(alias = "FC_ISCR1_7")]
pub type FcIscr1_7 = crate::Reg<fc_iscr1_7::FcIscr1_7Spec>;
#[doc = "Frame Composer ISRC1 Packet Body Register 7"]
pub mod fc_iscr1_7;
#[doc = "FC_ISCR1_6 (rw) register accessor: Frame Composer ISRC1 Packet Body Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr1_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr1_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_iscr1_6`]
module"]
#[doc(alias = "FC_ISCR1_6")]
pub type FcIscr1_6 = crate::Reg<fc_iscr1_6::FcIscr1_6Spec>;
#[doc = "Frame Composer ISRC1 Packet Body Register 6"]
pub mod fc_iscr1_6;
#[doc = "FC_ISCR1_5 (rw) register accessor: Frame Composer ISRC1 Packet Body Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr1_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr1_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_iscr1_5`]
module"]
#[doc(alias = "FC_ISCR1_5")]
pub type FcIscr1_5 = crate::Reg<fc_iscr1_5::FcIscr1_5Spec>;
#[doc = "Frame Composer ISRC1 Packet Body Register 5"]
pub mod fc_iscr1_5;
#[doc = "FC_ISCR1_4 (rw) register accessor: Frame Composer ISRC1 Packet Body Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr1_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr1_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_iscr1_4`]
module"]
#[doc(alias = "FC_ISCR1_4")]
pub type FcIscr1_4 = crate::Reg<fc_iscr1_4::FcIscr1_4Spec>;
#[doc = "Frame Composer ISRC1 Packet Body Register 4"]
pub mod fc_iscr1_4;
#[doc = "FC_ISCR1_3 (rw) register accessor: Frame Composer ISRC1 Packet Body Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr1_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr1_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_iscr1_3`]
module"]
#[doc(alias = "FC_ISCR1_3")]
pub type FcIscr1_3 = crate::Reg<fc_iscr1_3::FcIscr1_3Spec>;
#[doc = "Frame Composer ISRC1 Packet Body Register 3"]
pub mod fc_iscr1_3;
#[doc = "FC_ISCR1_2 (rw) register accessor: Frame Composer ISRC1 Packet Body Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr1_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr1_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_iscr1_2`]
module"]
#[doc(alias = "FC_ISCR1_2")]
pub type FcIscr1_2 = crate::Reg<fc_iscr1_2::FcIscr1_2Spec>;
#[doc = "Frame Composer ISRC1 Packet Body Register 2"]
pub mod fc_iscr1_2;
#[doc = "FC_ISCR1_1 (rw) register accessor: Frame Composer ISRC1 Packet Body Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr1_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr1_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_iscr1_1`]
module"]
#[doc(alias = "FC_ISCR1_1")]
pub type FcIscr1_1 = crate::Reg<fc_iscr1_1::FcIscr1_1Spec>;
#[doc = "Frame Composer ISRC1 Packet Body Register 1"]
pub mod fc_iscr1_1;
#[doc = "FC_ISCR2_15 (rw) register accessor: Frame Composer ISRC2 Packet Body Register 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr2_15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr2_15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_iscr2_15`]
module"]
#[doc(alias = "FC_ISCR2_15")]
pub type FcIscr2_15 = crate::Reg<fc_iscr2_15::FcIscr2_15Spec>;
#[doc = "Frame Composer ISRC2 Packet Body Register 15"]
pub mod fc_iscr2_15;
#[doc = "FC_ISCR2_14 (rw) register accessor: Frame Composer ISRC2 Packet Body Register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr2_14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr2_14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_iscr2_14`]
module"]
#[doc(alias = "FC_ISCR2_14")]
pub type FcIscr2_14 = crate::Reg<fc_iscr2_14::FcIscr2_14Spec>;
#[doc = "Frame Composer ISRC2 Packet Body Register 14"]
pub mod fc_iscr2_14;
#[doc = "FC_ISCR2_13 (rw) register accessor: Frame Composer ISRC2 Packet Body Register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr2_13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr2_13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_iscr2_13`]
module"]
#[doc(alias = "FC_ISCR2_13")]
pub type FcIscr2_13 = crate::Reg<fc_iscr2_13::FcIscr2_13Spec>;
#[doc = "Frame Composer ISRC2 Packet Body Register 13"]
pub mod fc_iscr2_13;
#[doc = "FC_ISCR2_12 (rw) register accessor: Frame Composer ISRC2 Packet Body Register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr2_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr2_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_iscr2_12`]
module"]
#[doc(alias = "FC_ISCR2_12")]
pub type FcIscr2_12 = crate::Reg<fc_iscr2_12::FcIscr2_12Spec>;
#[doc = "Frame Composer ISRC2 Packet Body Register 12"]
pub mod fc_iscr2_12;
#[doc = "FC_ISCR2_11 (rw) register accessor: Frame Composer ISRC2 Packet Body Register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr2_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr2_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_iscr2_11`]
module"]
#[doc(alias = "FC_ISCR2_11")]
pub type FcIscr2_11 = crate::Reg<fc_iscr2_11::FcIscr2_11Spec>;
#[doc = "Frame Composer ISRC2 Packet Body Register 11"]
pub mod fc_iscr2_11;
#[doc = "FC_ISCR2_10 (rw) register accessor: Frame Composer ISRC2 Packet Body Register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr2_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr2_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_iscr2_10`]
module"]
#[doc(alias = "FC_ISCR2_10")]
pub type FcIscr2_10 = crate::Reg<fc_iscr2_10::FcIscr2_10Spec>;
#[doc = "Frame Composer ISRC2 Packet Body Register 10"]
pub mod fc_iscr2_10;
#[doc = "FC_ISCR2_9 (rw) register accessor: Frame Composer ISRC2 Packet Body Register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr2_9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr2_9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_iscr2_9`]
module"]
#[doc(alias = "FC_ISCR2_9")]
pub type FcIscr2_9 = crate::Reg<fc_iscr2_9::FcIscr2_9Spec>;
#[doc = "Frame Composer ISRC2 Packet Body Register 9"]
pub mod fc_iscr2_9;
#[doc = "FC_ISCR2_8 (rw) register accessor: Frame Composer ISRC2 Packet Body Register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr2_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr2_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_iscr2_8`]
module"]
#[doc(alias = "FC_ISCR2_8")]
pub type FcIscr2_8 = crate::Reg<fc_iscr2_8::FcIscr2_8Spec>;
#[doc = "Frame Composer ISRC2 Packet Body Register 8"]
pub mod fc_iscr2_8;
#[doc = "FC_ISCR2_7 (rw) register accessor: Frame Composer ISRC2 Packet Body Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr2_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr2_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_iscr2_7`]
module"]
#[doc(alias = "FC_ISCR2_7")]
pub type FcIscr2_7 = crate::Reg<fc_iscr2_7::FcIscr2_7Spec>;
#[doc = "Frame Composer ISRC2 Packet Body Register 7"]
pub mod fc_iscr2_7;
#[doc = "FC_ISCR2_6 (rw) register accessor: Frame Composer ISRC2 Packet Body Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr2_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr2_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_iscr2_6`]
module"]
#[doc(alias = "FC_ISCR2_6")]
pub type FcIscr2_6 = crate::Reg<fc_iscr2_6::FcIscr2_6Spec>;
#[doc = "Frame Composer ISRC2 Packet Body Register 6"]
pub mod fc_iscr2_6;
#[doc = "FC_ISCR2_5 (rw) register accessor: Frame Composer ISRC2 Packet Body Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr2_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr2_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_iscr2_5`]
module"]
#[doc(alias = "FC_ISCR2_5")]
pub type FcIscr2_5 = crate::Reg<fc_iscr2_5::FcIscr2_5Spec>;
#[doc = "Frame Composer ISRC2 Packet Body Register 5"]
pub mod fc_iscr2_5;
#[doc = "FC_ISCR2_4 (rw) register accessor: Frame Composer ISRC2 Packet Body Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr2_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr2_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_iscr2_4`]
module"]
#[doc(alias = "FC_ISCR2_4")]
pub type FcIscr2_4 = crate::Reg<fc_iscr2_4::FcIscr2_4Spec>;
#[doc = "Frame Composer ISRC2 Packet Body Register 4"]
pub mod fc_iscr2_4;
#[doc = "FC_ISCR2_3 (rw) register accessor: Frame Composer ISRC2 Packet Body Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr2_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr2_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_iscr2_3`]
module"]
#[doc(alias = "FC_ISCR2_3")]
pub type FcIscr2_3 = crate::Reg<fc_iscr2_3::FcIscr2_3Spec>;
#[doc = "Frame Composer ISRC2 Packet Body Register 3"]
pub mod fc_iscr2_3;
#[doc = "FC_ISCR2_2 (rw) register accessor: Frame Composer ISRC2 Packet Body Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr2_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr2_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_iscr2_2`]
module"]
#[doc(alias = "FC_ISCR2_2")]
pub type FcIscr2_2 = crate::Reg<fc_iscr2_2::FcIscr2_2Spec>;
#[doc = "Frame Composer ISRC2 Packet Body Register 2"]
pub mod fc_iscr2_2;
#[doc = "FC_ISCR2_1 (rw) register accessor: Frame Composer ISRC2 Packet Body Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr2_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr2_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_iscr2_1`]
module"]
#[doc(alias = "FC_ISCR2_1")]
pub type FcIscr2_1 = crate::Reg<fc_iscr2_1::FcIscr2_1Spec>;
#[doc = "Frame Composer ISRC2 Packet Body Register 1"]
pub mod fc_iscr2_1;
#[doc = "FC_ISCR2_0 (rw) register accessor: Frame Composer ISRC2 Packet Body Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr2_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr2_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_iscr2_0`]
module"]
#[doc(alias = "FC_ISCR2_0")]
pub type FcIscr2_0 = crate::Reg<fc_iscr2_0::FcIscr2_0Spec>;
#[doc = "Frame Composer ISRC2 Packet Body Register 0"]
pub mod fc_iscr2_0;
#[doc = "FC_DATAUTO0 (rw) register accessor: Frame Composer Data Island Auto Packet Scheduling Register 0\n\nConfigures the Frame Composer RDRB(1)/Manual(0) data island packet insertion for SPD,\n\nVSD, ISRC2, ISRC1 and ACP packets. On RDRB mode the described packet scheduling is\n\ncontrolled by registers FC_DATAUTO1 and FC_DATAUTO2, while in Manual mode register\n\nFC_DATMAN requests to FC the insertion of the requested packet.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_datauto0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_datauto0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_datauto0`]
module"]
#[doc(alias = "FC_DATAUTO0")]
pub type FcDatauto0 = crate::Reg<fc_datauto0::FcDatauto0Spec>;
#[doc = "Frame Composer Data Island Auto Packet Scheduling Register 0\n\nConfigures the Frame Composer RDRB(1)/Manual(0) data island packet insertion for SPD,\n\nVSD, ISRC2, ISRC1 and ACP packets. On RDRB mode the described packet scheduling is\n\ncontrolled by registers FC_DATAUTO1 and FC_DATAUTO2, while in Manual mode register\n\nFC_DATMAN requests to FC the insertion of the requested packet."]
pub mod fc_datauto0;
#[doc = "FC_DATAUTO1 (rw) register accessor: Frame Composer Data Island Auto Packet Scheduling Register 1\n\nConfigures the Frame Composer (FC) RDRB frame interpolation for SPD, VSD, ISRC2,\n\nISRC1 and ACP packet insertion on data island when FC is on RDRB mode for the listed\n\npackets.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_datauto1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_datauto1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_datauto1`]
module"]
#[doc(alias = "FC_DATAUTO1")]
pub type FcDatauto1 = crate::Reg<fc_datauto1::FcDatauto1Spec>;
#[doc = "Frame Composer Data Island Auto Packet Scheduling Register 1\n\nConfigures the Frame Composer (FC) RDRB frame interpolation for SPD, VSD, ISRC2,\n\nISRC1 and ACP packet insertion on data island when FC is on RDRB mode for the listed\n\npackets."]
pub mod fc_datauto1;
#[doc = "FC_DATAUTO2 (rw) register accessor: Frame Composer Data Island Auto packet scheduling Register 2\n\nConfigures the Frame Composer (FC) RDRB line interpolation and number of packets in\n\nframe for SPD, VSD, ISRC2, ISRC1 and ACP packet insertion on data island when FC is on\n\nRDRB mode for the listed packets.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_datauto2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_datauto2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_datauto2`]
module"]
#[doc(alias = "FC_DATAUTO2")]
pub type FcDatauto2 = crate::Reg<fc_datauto2::FcDatauto2Spec>;
#[doc = "Frame Composer Data Island Auto packet scheduling Register 2\n\nConfigures the Frame Composer (FC) RDRB line interpolation and number of packets in\n\nframe for SPD, VSD, ISRC2, ISRC1 and ACP packet insertion on data island when FC is on\n\nRDRB mode for the listed packets."]
pub mod fc_datauto2;
#[doc = "FC_DATMAN (w) register accessor: Frame Composer Data Island Manual Packet Request Register\n\nRequests to the Frame Composer the data island packet insertion for NULL, SPD, VSD,\n\nISRC2, ISRC1 and ACP packets when FC_DATAUTO0 bit is in manual mode for the packet\n\nrequested.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_datman::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_datman`]
module"]
#[doc(alias = "FC_DATMAN")]
pub type FcDatman = crate::Reg<fc_datman::FcDatmanSpec>;
#[doc = "Frame Composer Data Island Manual Packet Request Register\n\nRequests to the Frame Composer the data island packet insertion for NULL, SPD, VSD,\n\nISRC2, ISRC1 and ACP packets when FC_DATAUTO0 bit is in manual mode for the packet\n\nrequested."]
pub mod fc_datman;
#[doc = "FC_DATAUTO3 (rw) register accessor: Frame Composer Data Island Auto Packet Scheduling Register 3\n\nConfigures the Frame Composer Automatic(1)/RDRB(0) data island packet insertion for\n\nAVI, GCP, AUDI and ACR packets. In Automatic mode, the packet is inserted on Vblanking\n\nwhen first line with active Vsync appears.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_datauto3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_datauto3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_datauto3`]
module"]
#[doc(alias = "FC_DATAUTO3")]
pub type FcDatauto3 = crate::Reg<fc_datauto3::FcDatauto3Spec>;
#[doc = "Frame Composer Data Island Auto Packet Scheduling Register 3\n\nConfigures the Frame Composer Automatic(1)/RDRB(0) data island packet insertion for\n\nAVI, GCP, AUDI and ACR packets. In Automatic mode, the packet is inserted on Vblanking\n\nwhen first line with active Vsync appears."]
pub mod fc_datauto3;
#[doc = "FC_RDRB0 (rw) register accessor: Frame Composer Round Robin ACR Packet Insertion Register 0\n\nConfigures the Frame Composer (FC) RDRB frame interpolation for ACR packet insertion on\n\ndata island when FC is on RDRB mode for this packet.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_rdrb0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_rdrb0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_rdrb0`]
module"]
#[doc(alias = "FC_RDRB0")]
pub type FcRdrb0 = crate::Reg<fc_rdrb0::FcRdrb0Spec>;
#[doc = "Frame Composer Round Robin ACR Packet Insertion Register 0\n\nConfigures the Frame Composer (FC) RDRB frame interpolation for ACR packet insertion on\n\ndata island when FC is on RDRB mode for this packet."]
pub mod fc_rdrb0;
#[doc = "FC_RDRB1 (rw) register accessor: Frame Composer Round Robin ACR Packet Insertion Register 1\n\nConfigures the Frame Composer (FC) RDRB line interpolation and number of packets in\n\nframe for the ACR packet insertion on data island when FC is on RDRB mode this packet.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_rdrb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_rdrb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_rdrb1`]
module"]
#[doc(alias = "FC_RDRB1")]
pub type FcRdrb1 = crate::Reg<fc_rdrb1::FcRdrb1Spec>;
#[doc = "Frame Composer Round Robin ACR Packet Insertion Register 1\n\nConfigures the Frame Composer (FC) RDRB line interpolation and number of packets in\n\nframe for the ACR packet insertion on data island when FC is on RDRB mode this packet."]
pub mod fc_rdrb1;
#[doc = "FC_RDRB2 (rw) register accessor: Frame Composer Round Robin AUDI Packet Insertion Register 2\n\nConfigures the Frame Composer (FC) RDRB frame interpolation for AUDI packet insertion\n\non data island when FC is on RDRB mode for this packet.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_rdrb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_rdrb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_rdrb2`]
module"]
#[doc(alias = "FC_RDRB2")]
pub type FcRdrb2 = crate::Reg<fc_rdrb2::FcRdrb2Spec>;
#[doc = "Frame Composer Round Robin AUDI Packet Insertion Register 2\n\nConfigures the Frame Composer (FC) RDRB frame interpolation for AUDI packet insertion\n\non data island when FC is on RDRB mode for this packet."]
pub mod fc_rdrb2;
#[doc = "FC_RDRB3 (rw) register accessor: Frame Composer Round Robin AUDI Packet Insertion Register 3\n\nConfigures the Frame Composer (FC) RDRB line interpolation and number of packets in\n\nframe for the AUDI packet insertion on data island when FC is on RDRB mode this packet.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_rdrb3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_rdrb3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_rdrb3`]
module"]
#[doc(alias = "FC_RDRB3")]
pub type FcRdrb3 = crate::Reg<fc_rdrb3::FcRdrb3Spec>;
#[doc = "Frame Composer Round Robin AUDI Packet Insertion Register 3\n\nConfigures the Frame Composer (FC) RDRB line interpolation and number of packets in\n\nframe for the AUDI packet insertion on data island when FC is on RDRB mode this packet."]
pub mod fc_rdrb3;
#[doc = "FC_RDRB4 (rw) register accessor: Frame Composer Round Robin GCP Packet Insertion Register 4\n\nConfigures the Frame Composer (FC) RDRB frame interpolation for GCP packet insertion on\n\ndata island when FC is on RDRB mode for this packet.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_rdrb4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_rdrb4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_rdrb4`]
module"]
#[doc(alias = "FC_RDRB4")]
pub type FcRdrb4 = crate::Reg<fc_rdrb4::FcRdrb4Spec>;
#[doc = "Frame Composer Round Robin GCP Packet Insertion Register 4\n\nConfigures the Frame Composer (FC) RDRB frame interpolation for GCP packet insertion on\n\ndata island when FC is on RDRB mode for this packet."]
pub mod fc_rdrb4;
#[doc = "FC_RDRB5 (rw) register accessor: Frame Composer Round Robin GCP Packet Insertion Register 5\n\nConfigures the Frame Composer (FC) RDRB line interpolation and number of packets in\n\nframe for the GCP packet insertion on data island when FC is on RDRB mode this packet.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_rdrb5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_rdrb5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_rdrb5`]
module"]
#[doc(alias = "FC_RDRB5")]
pub type FcRdrb5 = crate::Reg<fc_rdrb5::FcRdrb5Spec>;
#[doc = "Frame Composer Round Robin GCP Packet Insertion Register 5\n\nConfigures the Frame Composer (FC) RDRB line interpolation and number of packets in\n\nframe for the GCP packet insertion on data island when FC is on RDRB mode this packet."]
pub mod fc_rdrb5;
#[doc = "FC_RDRB6 (rw) register accessor: Frame Composer Round Robin AVI Packet Insertion Register 6\n\nConfigures the Frame Composer (FC) RDRB frame interpolation for AVI packet insertion on\n\ndata island when FC is on RDRB mode for this packet.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_rdrb6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_rdrb6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_rdrb6`]
module"]
#[doc(alias = "FC_RDRB6")]
pub type FcRdrb6 = crate::Reg<fc_rdrb6::FcRdrb6Spec>;
#[doc = "Frame Composer Round Robin AVI Packet Insertion Register 6\n\nConfigures the Frame Composer (FC) RDRB frame interpolation for AVI packet insertion on\n\ndata island when FC is on RDRB mode for this packet."]
pub mod fc_rdrb6;
#[doc = "FC_RDRB7 (rw) register accessor: Frame Composer Round Robin AVI Packet Insertion Register 7\n\nConfigures the Frame Composer (FC) RDRB line interpolation and number of packets in\n\nframe for the AVI packet insertion on data island when FC is on RDRB mode this packet.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_rdrb7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_rdrb7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_rdrb7`]
module"]
#[doc(alias = "FC_RDRB7")]
pub type FcRdrb7 = crate::Reg<fc_rdrb7::FcRdrb7Spec>;
#[doc = "Frame Composer Round Robin AVI Packet Insertion Register 7\n\nConfigures the Frame Composer (FC) RDRB line interpolation and number of packets in\n\nframe for the AVI packet insertion on data island when FC is on RDRB mode this packet."]
pub mod fc_rdrb7;
#[doc = "FC_RDRB8 (rw) register accessor: Frame Composer Round Robin AMP Packet Insertion Register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_rdrb8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_rdrb8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_rdrb8`]
module"]
#[doc(alias = "FC_RDRB8")]
pub type FcRdrb8 = crate::Reg<fc_rdrb8::FcRdrb8Spec>;
#[doc = "Frame Composer Round Robin AMP Packet Insertion Register 8"]
pub mod fc_rdrb8;
#[doc = "FC_RDRB9 (rw) register accessor: Frame Composer Round Robin AMP Packet Insertion Register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_rdrb9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_rdrb9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_rdrb9`]
module"]
#[doc(alias = "FC_RDRB9")]
pub type FcRdrb9 = crate::Reg<fc_rdrb9::FcRdrb9Spec>;
#[doc = "Frame Composer Round Robin AMP Packet Insertion Register 9"]
pub mod fc_rdrb9;
#[doc = "FC_RDRB10 (rw) register accessor: Frame Composer Round Robin NTSC VBI Packet Insertion Register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_rdrb10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_rdrb10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_rdrb10`]
module"]
#[doc(alias = "FC_RDRB10")]
pub type FcRdrb10 = crate::Reg<fc_rdrb10::FcRdrb10Spec>;
#[doc = "Frame Composer Round Robin NTSC VBI Packet Insertion Register 10"]
pub mod fc_rdrb10;
#[doc = "FC_RDRB11 (rw) register accessor: Frame Composer Round Robin NTSC VBI Packet Insertion Register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_rdrb11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_rdrb11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_rdrb11`]
module"]
#[doc(alias = "FC_RDRB11")]
pub type FcRdrb11 = crate::Reg<fc_rdrb11::FcRdrb11Spec>;
#[doc = "Frame Composer Round Robin NTSC VBI Packet Insertion Register 11"]
pub mod fc_rdrb11;
#[doc = "FC_RDRB12 (rw) register accessor: Frame Composer Round Robin DRM Packet Insertion Register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_rdrb12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_rdrb12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_rdrb12`]
module"]
#[doc(alias = "FC_RDRB12")]
pub type FcRdrb12 = crate::Reg<fc_rdrb12::FcRdrb12Spec>;
#[doc = "Frame Composer Round Robin DRM Packet Insertion Register 12"]
pub mod fc_rdrb12;
#[doc = "FC_RDRB13 (rw) register accessor: Frame Composer Round Robin DRM Packet Insertion Register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_rdrb13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_rdrb13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_rdrb13`]
module"]
#[doc(alias = "FC_RDRB13")]
pub type FcRdrb13 = crate::Reg<fc_rdrb13::FcRdrb13Spec>;
#[doc = "Frame Composer Round Robin DRM Packet Insertion Register 13"]
pub mod fc_rdrb13;
#[doc = "FC_MASK0 (rw) register accessor: Frame Composer Packet Interrupt Mask Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_mask0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_mask0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_mask0`]
module"]
#[doc(alias = "FC_MASK0")]
pub type FcMask0 = crate::Reg<fc_mask0::FcMask0Spec>;
#[doc = "Frame Composer Packet Interrupt Mask Register 0"]
pub mod fc_mask0;
#[doc = "FC_MASK1 (rw) register accessor: Frame Composer Packet Interrupt Mask Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_mask1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_mask1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_mask1`]
module"]
#[doc(alias = "FC_MASK1")]
pub type FcMask1 = crate::Reg<fc_mask1::FcMask1Spec>;
#[doc = "Frame Composer Packet Interrupt Mask Register 1"]
pub mod fc_mask1;
#[doc = "FC_MASK2 (rw) register accessor: Frame Composer High/Low Priority Overflow and DRM Interrupt Mask Register\n\n2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_mask2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_mask2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_mask2`]
module"]
#[doc(alias = "FC_MASK2")]
pub type FcMask2 = crate::Reg<fc_mask2::FcMask2Spec>;
#[doc = "Frame Composer High/Low Priority Overflow and DRM Interrupt Mask Register\n\n2"]
pub mod fc_mask2;
#[doc = "FC_PRCONF (rw) register accessor: Frame Composer Pixel Repetition Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_prconf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_prconf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_prconf`]
module"]
#[doc(alias = "FC_PRCONF")]
pub type FcPrconf = crate::Reg<fc_prconf::FcPrconfSpec>;
#[doc = "Frame Composer Pixel Repetition Configuration Register"]
pub mod fc_prconf;
#[doc = "FC_SCRAMBLER_CTRL (rw) register accessor: Frame Composer Scrambler Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_scrambler_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_scrambler_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_scrambler_ctrl`]
module"]
#[doc(alias = "FC_SCRAMBLER_CTRL")]
pub type FcScramblerCtrl = crate::Reg<fc_scrambler_ctrl::FcScramblerCtrlSpec>;
#[doc = "Frame Composer Scrambler Control"]
pub mod fc_scrambler_ctrl;
#[doc = "FC_MULTISTREAM_CTRL (rw) register accessor: Frame Composer Multi-Stream Audio Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_multistream_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_multistream_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_multistream_ctrl`]
module"]
#[doc(alias = "FC_MULTISTREAM_CTRL")]
pub type FcMultistreamCtrl = crate::Reg<fc_multistream_ctrl::FcMultistreamCtrlSpec>;
#[doc = "Frame Composer Multi-Stream Audio Control"]
pub mod fc_multistream_ctrl;
#[doc = "FC_PACKET_TX_EN (rw) register accessor: Frame Composer Packet Transmission Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_packet_tx_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_packet_tx_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_packet_tx_en`]
module"]
#[doc(alias = "FC_PACKET_TX_EN")]
pub type FcPacketTxEn = crate::Reg<fc_packet_tx_en::FcPacketTxEnSpec>;
#[doc = "Frame Composer Packet Transmission Control"]
pub mod fc_packet_tx_en;
#[doc = "FC_ACTSPC_HDLR_CFG (rw) register accessor: Frame Composer Active Space Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_actspc_hdlr_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_actspc_hdlr_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_actspc_hdlr_cfg`]
module"]
#[doc(alias = "FC_ACTSPC_HDLR_CFG")]
pub type FcActspcHdlrCfg = crate::Reg<fc_actspc_hdlr_cfg::FcActspcHdlrCfgSpec>;
#[doc = "Frame Composer Active Space Control"]
pub mod fc_actspc_hdlr_cfg;
#[doc = "FC_INVACT_2D_0 (rw) register accessor: Frame Composer Input Video 2D VActive Pixels Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_invact_2d_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_invact_2d_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_invact_2d_0`]
module"]
#[doc(alias = "FC_INVACT_2D_0")]
pub type FcInvact2d0 = crate::Reg<fc_invact_2d_0::FcInvact2d0Spec>;
#[doc = "Frame Composer Input Video 2D VActive Pixels Register 0"]
pub mod fc_invact_2d_0;
#[doc = "FC_INVACT_2D_1 (rw) register accessor: Frame Composer Input Video VActive pixels Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_invact_2d_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_invact_2d_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_invact_2d_1`]
module"]
#[doc(alias = "FC_INVACT_2D_1")]
pub type FcInvact2d1 = crate::Reg<fc_invact_2d_1::FcInvact2d1Spec>;
#[doc = "Frame Composer Input Video VActive pixels Register 1"]
pub mod fc_invact_2d_1;
#[doc = "FC_GMD_STAT (r) register accessor: Frame Composer GMD Packet Status Register\n\nGamut metadata packet status bit information for no_current_gmd, next_gmd_field,\n\ngmd_packet_sequence and current_gamut_seq_num. For more information, refer to the\n\nHDMI 1.4b specification.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_gmd_stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_gmd_stat`]
module"]
#[doc(alias = "FC_GMD_STAT")]
pub type FcGmdStat = crate::Reg<fc_gmd_stat::FcGmdStatSpec>;
#[doc = "Frame Composer GMD Packet Status Register\n\nGamut metadata packet status bit information for no_current_gmd, next_gmd_field,\n\ngmd_packet_sequence and current_gamut_seq_num. For more information, refer to the\n\nHDMI 1.4b specification."]
pub mod fc_gmd_stat;
#[doc = "FC_GMD_EN (rw) register accessor: Frame Composer GMD Packet Enable Register\n\nThis register enables Gamut metadata (GMD) packet transmission. Packets are inserted in\n\nthe incoming frame, starting in the line where active Vsync indication starts. After enable of\n\nGMD packets the outgoing packet is sent with no_current_gmd active indication until\n\nupdate GMD request is performed in the controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_gmd_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_gmd_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_gmd_en`]
module"]
#[doc(alias = "FC_GMD_EN")]
pub type FcGmdEn = crate::Reg<fc_gmd_en::FcGmdEnSpec>;
#[doc = "Frame Composer GMD Packet Enable Register\n\nThis register enables Gamut metadata (GMD) packet transmission. Packets are inserted in\n\nthe incoming frame, starting in the line where active Vsync indication starts. After enable of\n\nGMD packets the outgoing packet is sent with no_current_gmd active indication until\n\nupdate GMD request is performed in the controller."]
pub mod fc_gmd_en;
#[doc = "FC_GMD_UP (w) register accessor: Frame Composer GMD Packet Update Register\n\nThis register performs an GMD packet content update according to the configured packet\n\nbody (FC_GMD_PB0 to FC_GMD_PB27) and packet header (FC_GMD_HB). This active high\n\nauto clear register reflects the body and header configurations on the GMD packets sent\n\narbitrating the current_gamut_seq_num, gmd_packet_sequence and next_gmd_field bits\n\non packet to correctly indicate to source the Gamut change to be performed. After enable\n\nGMD packets the first update request is also responsible for deactivating the\n\nno_current_gmd indication bit.\n\nAttention packet update request must only be done after correct configuration of GMD\n\npacket body and header registers. Correct affected_gamut_seq_num and gmd_profile\n\nconfiguration is user responsibility and must convey with HDMI 1.4b standard gamut rules.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_gmd_up::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_gmd_up`]
module"]
#[doc(alias = "FC_GMD_UP")]
pub type FcGmdUp = crate::Reg<fc_gmd_up::FcGmdUpSpec>;
#[doc = "Frame Composer GMD Packet Update Register\n\nThis register performs an GMD packet content update according to the configured packet\n\nbody (FC_GMD_PB0 to FC_GMD_PB27) and packet header (FC_GMD_HB). This active high\n\nauto clear register reflects the body and header configurations on the GMD packets sent\n\narbitrating the current_gamut_seq_num, gmd_packet_sequence and next_gmd_field bits\n\non packet to correctly indicate to source the Gamut change to be performed. After enable\n\nGMD packets the first update request is also responsible for deactivating the\n\nno_current_gmd indication bit.\n\nAttention packet update request must only be done after correct configuration of GMD\n\npacket body and header registers. Correct affected_gamut_seq_num and gmd_profile\n\nconfiguration is user responsibility and must convey with HDMI 1.4b standard gamut rules."]
pub mod fc_gmd_up;
#[doc = "FC_GMD_CONF (rw) register accessor: Frame Composer GMD Packet Schedule Configuration Register\n\nThis register configures the number of GMD packets to be inserted per frame (starting\n\nalways in the line where the active Vsync appears) and the line spacing between the\n\ntransmitted GMD packets.\n\nNote that for profile P0 (refer to the HDMI 1.4b specification) this register should only\n\nindicate one GMD packet to be inserted per video field.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_gmd_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_gmd_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_gmd_conf`]
module"]
#[doc(alias = "FC_GMD_CONF")]
pub type FcGmdConf = crate::Reg<fc_gmd_conf::FcGmdConfSpec>;
#[doc = "Frame Composer GMD Packet Schedule Configuration Register\n\nThis register configures the number of GMD packets to be inserted per frame (starting\n\nalways in the line where the active Vsync appears) and the line spacing between the\n\ntransmitted GMD packets.\n\nNote that for profile P0 (refer to the HDMI 1.4b specification) this register should only\n\nindicate one GMD packet to be inserted per video field."]
pub mod fc_gmd_conf;
#[doc = "FC_GMD_HB (rw) register accessor: Frame Composer GMD Packet Profile and Gamut Sequence Configuration\n\nRegister\n\nThis register configures the GMD packet header affected_gamut_seq_num and gmd_profile\n\nbits. For more information, refer to the HDMI 1.4b specification.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_gmd_hb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_gmd_hb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_gmd_hb`]
module"]
#[doc(alias = "FC_GMD_HB")]
pub type FcGmdHb = crate::Reg<fc_gmd_hb::FcGmdHbSpec>;
#[doc = "Frame Composer GMD Packet Profile and Gamut Sequence Configuration\n\nRegister\n\nThis register configures the GMD packet header affected_gamut_seq_num and gmd_profile\n\nbits. For more information, refer to the HDMI 1.4b specification."]
pub mod fc_gmd_hb;
#[doc = "FC_GMD_PB (rw) register accessor: Frame Composer GMD Packet Body Register Array Configures the GMD packet\n\nbody of the GMD packet.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_gmd_pb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_gmd_pb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_gmd_pb`]
module"]
#[doc(alias = "FC_GMD_PB")]
pub type FcGmdPb = crate::Reg<fc_gmd_pb::FcGmdPbSpec>;
#[doc = "Frame Composer GMD Packet Body Register Array Configures the GMD packet\n\nbody of the GMD packet."]
pub mod fc_gmd_pb;
#[doc = "FC_AMP_HB1 (rw) register accessor: Frame Composer AMP Packet Header Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_amp_hb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_amp_hb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_amp_hb1`]
module"]
#[doc(alias = "FC_AMP_HB1")]
pub type FcAmpHb1 = crate::Reg<fc_amp_hb1::FcAmpHb1Spec>;
#[doc = "Frame Composer AMP Packet Header Register 1"]
pub mod fc_amp_hb1;
#[doc = "FC_AMP_HB2 (rw) register accessor: Frame Composer AMP Packet Header Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_amp_hb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_amp_hb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_amp_hb2`]
module"]
#[doc(alias = "FC_AMP_HB2")]
pub type FcAmpHb2 = crate::Reg<fc_amp_hb2::FcAmpHb2Spec>;
#[doc = "Frame Composer AMP Packet Header Register 2"]
pub mod fc_amp_hb2;
#[doc = "FC_AMP_PB (rw) register accessor: Frame Composer AMP Packet Body Register Array\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_amp_pb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_amp_pb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_amp_pb`]
module"]
#[doc(alias = "FC_AMP_PB")]
pub type FcAmpPb = crate::Reg<fc_amp_pb::FcAmpPbSpec>;
#[doc = "Frame Composer AMP Packet Body Register Array"]
pub mod fc_amp_pb;
#[doc = "FC_NVBI_HB1 (rw) register accessor: Frame Composer NTSC VBI Packet Header Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_nvbi_hb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_nvbi_hb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_nvbi_hb1`]
module"]
#[doc(alias = "FC_NVBI_HB1")]
pub type FcNvbiHb1 = crate::Reg<fc_nvbi_hb1::FcNvbiHb1Spec>;
#[doc = "Frame Composer NTSC VBI Packet Header Register 1"]
pub mod fc_nvbi_hb1;
#[doc = "FC_NVBI_HB2 (rw) register accessor: Frame Composer NTSC VBI Packet Header Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_nvbi_hb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_nvbi_hb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_nvbi_hb2`]
module"]
#[doc(alias = "FC_NVBI_HB2")]
pub type FcNvbiHb2 = crate::Reg<fc_nvbi_hb2::FcNvbiHb2Spec>;
#[doc = "Frame Composer NTSC VBI Packet Header Register 2"]
pub mod fc_nvbi_hb2;
#[doc = "FC_NVBI_PB (rw) register accessor: Frame Composer NTSC VBI Packet Body Register Array\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_nvbi_pb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_nvbi_pb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_nvbi_pb`]
module"]
#[doc(alias = "FC_NVBI_PB")]
pub type FcNvbiPb = crate::Reg<fc_nvbi_pb::FcNvbiPbSpec>;
#[doc = "Frame Composer NTSC VBI Packet Body Register Array"]
pub mod fc_nvbi_pb;
#[doc = "FC_DRM_UP (w) register accessor: Frame Composer DRM Packet Update Register\n\nThis register performs an DRM packet content update according to the configured packet\n\nbody (FC_DRM_PB0 to FC_DRM_PB27) and packet header (FC_DRM_HB). This active high\n\nauto clear register reflects the body and header configurations on the DRM packets change\n\nto be performed.\n\nAttention packet update request must only be done after correct configuration of DRM\n\npacket body and header registers.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_drm_up::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_drm_up`]
module"]
#[doc(alias = "FC_DRM_UP")]
pub type FcDrmUp = crate::Reg<fc_drm_up::FcDrmUpSpec>;
#[doc = "Frame Composer DRM Packet Update Register\n\nThis register performs an DRM packet content update according to the configured packet\n\nbody (FC_DRM_PB0 to FC_DRM_PB27) and packet header (FC_DRM_HB). This active high\n\nauto clear register reflects the body and header configurations on the DRM packets change\n\nto be performed.\n\nAttention packet update request must only be done after correct configuration of DRM\n\npacket body and header registers."]
pub mod fc_drm_up;
#[doc = "FC_DRM_HB (rw) register accessor: Frame Composer DRM Packet Header Register Array\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_drm_hb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_drm_hb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_drm_hb`]
module"]
#[doc(alias = "FC_DRM_HB")]
pub type FcDrmHb = crate::Reg<fc_drm_hb::FcDrmHbSpec>;
#[doc = "Frame Composer DRM Packet Header Register Array"]
pub mod fc_drm_hb;
#[doc = "FC_DRM_PB (rw) register accessor: Frame Composer DRM Packet Body Register Array\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_drm_pb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_drm_pb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_drm_pb`]
module"]
#[doc(alias = "FC_DRM_PB")]
pub type FcDrmPb = crate::Reg<fc_drm_pb::FcDrmPbSpec>;
#[doc = "Frame Composer DRM Packet Body Register Array"]
pub mod fc_drm_pb;
#[doc = "FC_DBGFORCE (rw) register accessor: Frame Composer video/audio Force Enable Register\n\nThis register allows to force the controller to output audio and video data the values\n\nconfigured in the FC_DBGAUD and FC_DBGTMDS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgforce::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgforce::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_dbgforce`]
module"]
#[doc(alias = "FC_DBGFORCE")]
pub type FcDbgforce = crate::Reg<fc_dbgforce::FcDbgforceSpec>;
#[doc = "Frame Composer video/audio Force Enable Register\n\nThis register allows to force the controller to output audio and video data the values\n\nconfigured in the FC_DBGAUD and FC_DBGTMDS registers."]
pub mod fc_dbgforce;
#[doc = "FC_DBGAUD0CH0 (rw) register accessor: Frame Composer Audio Data Channel 0 Register 0\n\nConfigures the audio fixed data to be used in channel 0 when in fixed audio selection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgaud0ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgaud0ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_dbgaud0ch0`]
module"]
#[doc(alias = "FC_DBGAUD0CH0")]
pub type FcDbgaud0ch0 = crate::Reg<fc_dbgaud0ch0::FcDbgaud0ch0Spec>;
#[doc = "Frame Composer Audio Data Channel 0 Register 0\n\nConfigures the audio fixed data to be used in channel 0 when in fixed audio selection."]
pub mod fc_dbgaud0ch0;
#[doc = "FC_DBGAUD1CH0 (rw) register accessor: Frame Composer Audio Data Channel 0 Register 1\n\nConfigures the audio fixed data to be used in channel 0 when in fixed audio selection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgaud1ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgaud1ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_dbgaud1ch0`]
module"]
#[doc(alias = "FC_DBGAUD1CH0")]
pub type FcDbgaud1ch0 = crate::Reg<fc_dbgaud1ch0::FcDbgaud1ch0Spec>;
#[doc = "Frame Composer Audio Data Channel 0 Register 1\n\nConfigures the audio fixed data to be used in channel 0 when in fixed audio selection."]
pub mod fc_dbgaud1ch0;
#[doc = "FC_DBGAUD2CH0 (rw) register accessor: Frame Composer Audio Data Channel 0 Register 2\n\nConfigures the audio fixed data to be used in channel 0 when in fixed audio selection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgaud2ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgaud2ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_dbgaud2ch0`]
module"]
#[doc(alias = "FC_DBGAUD2CH0")]
pub type FcDbgaud2ch0 = crate::Reg<fc_dbgaud2ch0::FcDbgaud2ch0Spec>;
#[doc = "Frame Composer Audio Data Channel 0 Register 2\n\nConfigures the audio fixed data to be used in channel 0 when in fixed audio selection."]
pub mod fc_dbgaud2ch0;
#[doc = "FC_DBGAUD0CH1 (rw) register accessor: Frame Composer Audio Data Channel 1 Register 0\n\nConfigures the audio fixed data to be used in channel 1 when in fixed audio selection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgaud0ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgaud0ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_dbgaud0ch1`]
module"]
#[doc(alias = "FC_DBGAUD0CH1")]
pub type FcDbgaud0ch1 = crate::Reg<fc_dbgaud0ch1::FcDbgaud0ch1Spec>;
#[doc = "Frame Composer Audio Data Channel 1 Register 0\n\nConfigures the audio fixed data to be used in channel 1 when in fixed audio selection."]
pub mod fc_dbgaud0ch1;
#[doc = "FC_DBGAUD1CH1 (rw) register accessor: Frame Composer Audio Data Channel 1 Register 1\n\nConfigures the audio fixed data to be used in channel 1 when in fixed audio selection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgaud1ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgaud1ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_dbgaud1ch1`]
module"]
#[doc(alias = "FC_DBGAUD1CH1")]
pub type FcDbgaud1ch1 = crate::Reg<fc_dbgaud1ch1::FcDbgaud1ch1Spec>;
#[doc = "Frame Composer Audio Data Channel 1 Register 1\n\nConfigures the audio fixed data to be used in channel 1 when in fixed audio selection."]
pub mod fc_dbgaud1ch1;
#[doc = "FC_DBGAUD2CH1 (rw) register accessor: Frame Composer Audio Data Channel 1 Register 2\n\nConfigures the audio fixed data to be used in channel 1 when in fixed audio selection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgaud2ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgaud2ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_dbgaud2ch1`]
module"]
#[doc(alias = "FC_DBGAUD2CH1")]
pub type FcDbgaud2ch1 = crate::Reg<fc_dbgaud2ch1::FcDbgaud2ch1Spec>;
#[doc = "Frame Composer Audio Data Channel 1 Register 2\n\nConfigures the audio fixed data to be used in channel 1 when in fixed audio selection."]
pub mod fc_dbgaud2ch1;
#[doc = "FC_DBGAUD0CH2 (rw) register accessor: Frame Composer Audio Data Channel 2 Register 0\n\nConfigures the audio fixed data to be used in channel 2 when in fixed audio selection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgaud0ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgaud0ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_dbgaud0ch2`]
module"]
#[doc(alias = "FC_DBGAUD0CH2")]
pub type FcDbgaud0ch2 = crate::Reg<fc_dbgaud0ch2::FcDbgaud0ch2Spec>;
#[doc = "Frame Composer Audio Data Channel 2 Register 0\n\nConfigures the audio fixed data to be used in channel 2 when in fixed audio selection."]
pub mod fc_dbgaud0ch2;
#[doc = "FC_DBGAUD1CH2 (rw) register accessor: Frame Composer Audio Data Channel 2 Register 1\n\nConfigures the audio fixed data to be used in channel 2 when in fixed audio selection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgaud1ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgaud1ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_dbgaud1ch2`]
module"]
#[doc(alias = "FC_DBGAUD1CH2")]
pub type FcDbgaud1ch2 = crate::Reg<fc_dbgaud1ch2::FcDbgaud1ch2Spec>;
#[doc = "Frame Composer Audio Data Channel 2 Register 1\n\nConfigures the audio fixed data to be used in channel 2 when in fixed audio selection."]
pub mod fc_dbgaud1ch2;
#[doc = "FC_DBGAUD2CH2 (rw) register accessor: Frame Composer Audio Data Channel 2 Register 2\n\nConfigures the audio fixed data to be used in channel 2 when in fixed audio selection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgaud2ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgaud2ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_dbgaud2ch2`]
module"]
#[doc(alias = "FC_DBGAUD2CH2")]
pub type FcDbgaud2ch2 = crate::Reg<fc_dbgaud2ch2::FcDbgaud2ch2Spec>;
#[doc = "Frame Composer Audio Data Channel 2 Register 2\n\nConfigures the audio fixed data to be used in channel 2 when in fixed audio selection."]
pub mod fc_dbgaud2ch2;
#[doc = "FC_DBGAUD0CH3 (rw) register accessor: Frame Composer Audio Data Channel 3 Register 0\n\nConfigures the audio fixed data to be used in channel 3 when in fixed audio selection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgaud0ch3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgaud0ch3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_dbgaud0ch3`]
module"]
#[doc(alias = "FC_DBGAUD0CH3")]
pub type FcDbgaud0ch3 = crate::Reg<fc_dbgaud0ch3::FcDbgaud0ch3Spec>;
#[doc = "Frame Composer Audio Data Channel 3 Register 0\n\nConfigures the audio fixed data to be used in channel 3 when in fixed audio selection."]
pub mod fc_dbgaud0ch3;
#[doc = "FC_DBGAUD1CH3 (rw) register accessor: Frame Composer Audio Data Channel 3 Register 1\n\nConfigures the audio fixed data to be used in channel 3 when in fixed audio selection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgaud1ch3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgaud1ch3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_dbgaud1ch3`]
module"]
#[doc(alias = "FC_DBGAUD1CH3")]
pub type FcDbgaud1ch3 = crate::Reg<fc_dbgaud1ch3::FcDbgaud1ch3Spec>;
#[doc = "Frame Composer Audio Data Channel 3 Register 1\n\nConfigures the audio fixed data to be used in channel 3 when in fixed audio selection."]
pub mod fc_dbgaud1ch3;
#[doc = "FC_DBGAUD2CH3 (rw) register accessor: Frame Composer Audio Data Channel 3 Register 2\n\nConfigures the audio fixed data to be used in channel 3 when in fixed audio selection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgaud2ch3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgaud2ch3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_dbgaud2ch3`]
module"]
#[doc(alias = "FC_DBGAUD2CH3")]
pub type FcDbgaud2ch3 = crate::Reg<fc_dbgaud2ch3::FcDbgaud2ch3Spec>;
#[doc = "Frame Composer Audio Data Channel 3 Register 2\n\nConfigures the audio fixed data to be used in channel 3 when in fixed audio selection."]
pub mod fc_dbgaud2ch3;
#[doc = "FC_DBGAUD0CH4 (rw) register accessor: Frame Composer Audio Data Channel 4 Register 0\n\nConfigures the audio fixed data to be used in channel 4 when in fixed audio selection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgaud0ch4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgaud0ch4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_dbgaud0ch4`]
module"]
#[doc(alias = "FC_DBGAUD0CH4")]
pub type FcDbgaud0ch4 = crate::Reg<fc_dbgaud0ch4::FcDbgaud0ch4Spec>;
#[doc = "Frame Composer Audio Data Channel 4 Register 0\n\nConfigures the audio fixed data to be used in channel 4 when in fixed audio selection."]
pub mod fc_dbgaud0ch4;
#[doc = "FC_DBGAUD1CH4 (rw) register accessor: Frame Composer Audio Data Channel 4 Register 1\n\nConfigures the audio fixed data to be used in channel 4 when in fixed audio selection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgaud1ch4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgaud1ch4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_dbgaud1ch4`]
module"]
#[doc(alias = "FC_DBGAUD1CH4")]
pub type FcDbgaud1ch4 = crate::Reg<fc_dbgaud1ch4::FcDbgaud1ch4Spec>;
#[doc = "Frame Composer Audio Data Channel 4 Register 1\n\nConfigures the audio fixed data to be used in channel 4 when in fixed audio selection."]
pub mod fc_dbgaud1ch4;
#[doc = "FC_DBGAUD2CH4 (rw) register accessor: Frame Composer Audio Data Channel 4 Register 2\n\nConfigures the audio fixed data to be used in channel 4 when in fixed audio selection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgaud2ch4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgaud2ch4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_dbgaud2ch4`]
module"]
#[doc(alias = "FC_DBGAUD2CH4")]
pub type FcDbgaud2ch4 = crate::Reg<fc_dbgaud2ch4::FcDbgaud2ch4Spec>;
#[doc = "Frame Composer Audio Data Channel 4 Register 2\n\nConfigures the audio fixed data to be used in channel 4 when in fixed audio selection."]
pub mod fc_dbgaud2ch4;
#[doc = "FC_DBGAUD0CH5 (rw) register accessor: Frame Composer Audio Data Channel 5 Register 0\n\nConfigures the audio fixed data to be used in channel 5 when in fixed audio selection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgaud0ch5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgaud0ch5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_dbgaud0ch5`]
module"]
#[doc(alias = "FC_DBGAUD0CH5")]
pub type FcDbgaud0ch5 = crate::Reg<fc_dbgaud0ch5::FcDbgaud0ch5Spec>;
#[doc = "Frame Composer Audio Data Channel 5 Register 0\n\nConfigures the audio fixed data to be used in channel 5 when in fixed audio selection."]
pub mod fc_dbgaud0ch5;
#[doc = "FC_DBGAUD1CH5 (rw) register accessor: Frame Composer Audio Data Channel 5 Register 1\n\nConfigures the audio fixed data to be used in channel 5 when in fixed audio selection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgaud1ch5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgaud1ch5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_dbgaud1ch5`]
module"]
#[doc(alias = "FC_DBGAUD1CH5")]
pub type FcDbgaud1ch5 = crate::Reg<fc_dbgaud1ch5::FcDbgaud1ch5Spec>;
#[doc = "Frame Composer Audio Data Channel 5 Register 1\n\nConfigures the audio fixed data to be used in channel 5 when in fixed audio selection."]
pub mod fc_dbgaud1ch5;
#[doc = "FC_DBGAUD2CH5 (rw) register accessor: Frame Composer Audio Data Channel 5 Register 2\n\nConfigures the audio fixed data to be used in channel 5 when in fixed audio selection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgaud2ch5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgaud2ch5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_dbgaud2ch5`]
module"]
#[doc(alias = "FC_DBGAUD2CH5")]
pub type FcDbgaud2ch5 = crate::Reg<fc_dbgaud2ch5::FcDbgaud2ch5Spec>;
#[doc = "Frame Composer Audio Data Channel 5 Register 2\n\nConfigures the audio fixed data to be used in channel 5 when in fixed audio selection."]
pub mod fc_dbgaud2ch5;
#[doc = "FC_DBGAUD0CH6 (rw) register accessor: Frame Composer Audio Data Channel 6 Register 0\n\nConfigures the audio fixed data to be used in channel 6 when in fixed audio selection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgaud0ch6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgaud0ch6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_dbgaud0ch6`]
module"]
#[doc(alias = "FC_DBGAUD0CH6")]
pub type FcDbgaud0ch6 = crate::Reg<fc_dbgaud0ch6::FcDbgaud0ch6Spec>;
#[doc = "Frame Composer Audio Data Channel 6 Register 0\n\nConfigures the audio fixed data to be used in channel 6 when in fixed audio selection."]
pub mod fc_dbgaud0ch6;
#[doc = "FC_DBGAUD1CH6 (rw) register accessor: Frame Composer Audio Data Channel 6 Register 1\n\nConfigures the audio fixed data to be used in channel 6 when in fixed audio selection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgaud1ch6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgaud1ch6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_dbgaud1ch6`]
module"]
#[doc(alias = "FC_DBGAUD1CH6")]
pub type FcDbgaud1ch6 = crate::Reg<fc_dbgaud1ch6::FcDbgaud1ch6Spec>;
#[doc = "Frame Composer Audio Data Channel 6 Register 1\n\nConfigures the audio fixed data to be used in channel 6 when in fixed audio selection."]
pub mod fc_dbgaud1ch6;
#[doc = "FC_DBGAUD2CH6 (rw) register accessor: Frame Composer Audio Data Channel 6 Register 2\n\nConfigures the audio fixed data to be used in channel 6 when in fixed audio selection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgaud2ch6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgaud2ch6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_dbgaud2ch6`]
module"]
#[doc(alias = "FC_DBGAUD2CH6")]
pub type FcDbgaud2ch6 = crate::Reg<fc_dbgaud2ch6::FcDbgaud2ch6Spec>;
#[doc = "Frame Composer Audio Data Channel 6 Register 2\n\nConfigures the audio fixed data to be used in channel 6 when in fixed audio selection."]
pub mod fc_dbgaud2ch6;
#[doc = "FC_DBGAUD0CH7 (rw) register accessor: Frame Composer Audio Data Channel 7 Register 0\n\nConfigures the audio fixed data to be used in channel 7 when in fixed audio selection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgaud0ch7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgaud0ch7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_dbgaud0ch7`]
module"]
#[doc(alias = "FC_DBGAUD0CH7")]
pub type FcDbgaud0ch7 = crate::Reg<fc_dbgaud0ch7::FcDbgaud0ch7Spec>;
#[doc = "Frame Composer Audio Data Channel 7 Register 0\n\nConfigures the audio fixed data to be used in channel 7 when in fixed audio selection."]
pub mod fc_dbgaud0ch7;
#[doc = "FC_DBGAUD1CH7 (rw) register accessor: Frame Composer Audio Data Channel 7 Register 1\n\nConfigures the audio fixed data to be used in channel 7 when in fixed audio selection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgaud1ch7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgaud1ch7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_dbgaud1ch7`]
module"]
#[doc(alias = "FC_DBGAUD1CH7")]
pub type FcDbgaud1ch7 = crate::Reg<fc_dbgaud1ch7::FcDbgaud1ch7Spec>;
#[doc = "Frame Composer Audio Data Channel 7 Register 1\n\nConfigures the audio fixed data to be used in channel 7 when in fixed audio selection."]
pub mod fc_dbgaud1ch7;
#[doc = "FC_DBGAUD2CH7 (rw) register accessor: Frame Composer Audio Data Channel 7 Register 2\n\nConfigures the audio fixed data to be used in channel 7 when in fixed audio selection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgaud2ch7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgaud2ch7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_dbgaud2ch7`]
module"]
#[doc(alias = "FC_DBGAUD2CH7")]
pub type FcDbgaud2ch7 = crate::Reg<fc_dbgaud2ch7::FcDbgaud2ch7Spec>;
#[doc = "Frame Composer Audio Data Channel 7 Register 2\n\nConfigures the audio fixed data to be used in channel 7 when in fixed audio selection."]
pub mod fc_dbgaud2ch7;
#[doc = "FC_DBGTMDS (rw) register accessor: Frame Composer TMDS Data Channel Register Array\n\nConfigures the video fixed data to be used in TMDS channel x (where x is 0 to 2) when in\n\nfixed video selection.\n\nFor Channel 0, this equals to set B pixel component value in RGB video or Cb pixel\n\ncomponent value in YCbCr.\n\nFor Channel 1, this equals set G pixel component value in RGB video or Y pixel component\n\nvalue in YCbCr.\n\nFor Channel 2, this equals to set R pixel component value in RGB video or Cr pixel\n\ncomponent value in YCbCr.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgtmds::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgtmds::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_dbgtmds`]
module"]
#[doc(alias = "FC_DBGTMDS")]
pub type FcDbgtmds = crate::Reg<fc_dbgtmds::FcDbgtmdsSpec>;
#[doc = "Frame Composer TMDS Data Channel Register Array\n\nConfigures the video fixed data to be used in TMDS channel x (where x is 0 to 2) when in\n\nfixed video selection.\n\nFor Channel 0, this equals to set B pixel component value in RGB video or Cb pixel\n\ncomponent value in YCbCr.\n\nFor Channel 1, this equals set G pixel component value in RGB video or Y pixel component\n\nvalue in YCbCr.\n\nFor Channel 2, this equals to set R pixel component value in RGB video or Cr pixel\n\ncomponent value in YCbCr."]
pub mod fc_dbgtmds;
#[doc = "PHY_CONF0 (rw) register accessor: PHY Configuration Register\n\nThis register holds the power down, data enable polarity, and interface control of the HDMI\n\nSource PHY control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_conf0`]
module"]
#[doc(alias = "PHY_CONF0")]
pub type PhyConf0 = crate::Reg<phy_conf0::PhyConf0Spec>;
#[doc = "PHY Configuration Register\n\nThis register holds the power down, data enable polarity, and interface control of the HDMI\n\nSource PHY control."]
pub mod phy_conf0;
#[doc = "PHY_TST0 (rw) register accessor: PHY Test Interface Register 0\n\nPHY TX mapped test interface (control).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_tst0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_tst0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_tst0`]
module"]
#[doc(alias = "PHY_TST0")]
pub type PhyTst0 = crate::Reg<phy_tst0::PhyTst0Spec>;
#[doc = "PHY Test Interface Register 0\n\nPHY TX mapped test interface (control)."]
pub mod phy_tst0;
#[doc = "PHY_TST1 (rw) register accessor: PHY Test Interface Register 1\n\nPHY TX mapped text interface (data in).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_tst1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_tst1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_tst1`]
module"]
#[doc(alias = "PHY_TST1")]
pub type PhyTst1 = crate::Reg<phy_tst1::PhyTst1Spec>;
#[doc = "PHY Test Interface Register 1\n\nPHY TX mapped text interface (data in)."]
pub mod phy_tst1;
#[doc = "PHY_TST2 (r) register accessor: PHY Test Interface Register 2\n\nPHY TX mapped text interface (data out).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_tst2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_tst2`]
module"]
#[doc(alias = "PHY_TST2")]
pub type PhyTst2 = crate::Reg<phy_tst2::PhyTst2Spec>;
#[doc = "PHY Test Interface Register 2\n\nPHY TX mapped text interface (data out)."]
pub mod phy_tst2;
#[doc = "PHY_STAT0 (r) register accessor: PHY RXSENSE, PLL Lock, and HPD Status Register\n\nThis register contains the following active high packet sent status indications.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_stat0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_stat0`]
module"]
#[doc(alias = "PHY_STAT0")]
pub type PhyStat0 = crate::Reg<phy_stat0::PhyStat0Spec>;
#[doc = "PHY RXSENSE, PLL Lock, and HPD Status Register\n\nThis register contains the following active high packet sent status indications."]
pub mod phy_stat0;
#[doc = "PHY_INT0 (r) register accessor: PHY RXSENSE, PLL Lock, and HPD Interrupt Register\n\nThis register contains the interrupt indication of the PHY_STAT0 status interrupts. Interrupt\n\ngeneration is accomplished in the following way:\n\ninterrupt = (mask == 1'b0) &amp;&amp; (polarity == status);\n\nAll these interrupts are forwarded to the Interrupt Handler sticky bit register ih_phy_stat0\n\nand after ORed to a single main interrupt line to micro controller. Assertion of this interrupt\n\nimplies that data related with the corresponding packet has been sent through the HDMI\n\ninterface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_int0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_int0`]
module"]
#[doc(alias = "PHY_INT0")]
pub type PhyInt0 = crate::Reg<phy_int0::PhyInt0Spec>;
#[doc = "PHY RXSENSE, PLL Lock, and HPD Interrupt Register\n\nThis register contains the interrupt indication of the PHY_STAT0 status interrupts. Interrupt\n\ngeneration is accomplished in the following way:\n\ninterrupt = (mask == 1'b0) &amp;&amp; (polarity == status);\n\nAll these interrupts are forwarded to the Interrupt Handler sticky bit register ih_phy_stat0\n\nand after ORed to a single main interrupt line to micro controller. Assertion of this interrupt\n\nimplies that data related with the corresponding packet has been sent through the HDMI\n\ninterface."]
pub mod phy_int0;
#[doc = "PHY_MASK0 (rw) register accessor: PHY RXSENSE, PLL Lock, and HPD Mask Register Mask register for generation\n\nof PHY_INT0 interrupts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_mask0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_mask0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_mask0`]
module"]
#[doc(alias = "PHY_MASK0")]
pub type PhyMask0 = crate::Reg<phy_mask0::PhyMask0Spec>;
#[doc = "PHY RXSENSE, PLL Lock, and HPD Mask Register Mask register for generation\n\nof PHY_INT0 interrupts."]
pub mod phy_mask0;
#[doc = "PHY_POL0 (rw) register accessor: PHY RXSENSE, PLL Lock, and HPD Polarity Register Polarity register for\n\ngeneration of PHY_INT0 interrupts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_pol0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_pol0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_pol0`]
module"]
#[doc(alias = "PHY_POL0")]
pub type PhyPol0 = crate::Reg<phy_pol0::PhyPol0Spec>;
#[doc = "PHY RXSENSE, PLL Lock, and HPD Polarity Register Polarity register for\n\ngeneration of PHY_INT0 interrupts."]
pub mod phy_pol0;
#[doc = "PHY_PCLFREQ0 (rw) register accessor: PHY Test Interface Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_pclfreq0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_pclfreq0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_pclfreq0`]
module"]
#[doc(alias = "PHY_PCLFREQ0")]
pub type PhyPclfreq0 = crate::Reg<phy_pclfreq0::PhyPclfreq0Spec>;
#[doc = "PHY Test Interface Register 0"]
pub mod phy_pclfreq0;
#[doc = "PHY_PCLFREQ1 (rw) register accessor: PHY Test Interface Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_pclfreq1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_pclfreq1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_pclfreq1`]
module"]
#[doc(alias = "PHY_PCLFREQ1")]
pub type PhyPclfreq1 = crate::Reg<phy_pclfreq1::PhyPclfreq1Spec>;
#[doc = "PHY Test Interface Register 1"]
pub mod phy_pclfreq1;
#[doc = "PHY_PLLCFGFREQ0 (rw) register accessor: PHY PLL Test Interface Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_pllcfgfreq0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_pllcfgfreq0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_pllcfgfreq0`]
module"]
#[doc(alias = "PHY_PLLCFGFREQ0")]
pub type PhyPllcfgfreq0 = crate::Reg<phy_pllcfgfreq0::PhyPllcfgfreq0Spec>;
#[doc = "PHY PLL Test Interface Register 0"]
pub mod phy_pllcfgfreq0;
#[doc = "PHY_PLLCFGFREQ1 (rw) register accessor: PHY PLL Test Interface Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_pllcfgfreq1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_pllcfgfreq1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_pllcfgfreq1`]
module"]
#[doc(alias = "PHY_PLLCFGFREQ1")]
pub type PhyPllcfgfreq1 = crate::Reg<phy_pllcfgfreq1::PhyPllcfgfreq1Spec>;
#[doc = "PHY PLL Test Interface Register 1"]
pub mod phy_pllcfgfreq1;
#[doc = "PHY_PLLCFGFREQ2 (rw) register accessor: PHY PLL Test Interface Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_pllcfgfreq2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_pllcfgfreq2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_pllcfgfreq2`]
module"]
#[doc(alias = "PHY_PLLCFGFREQ2")]
pub type PhyPllcfgfreq2 = crate::Reg<phy_pllcfgfreq2::PhyPllcfgfreq2Spec>;
#[doc = "PHY PLL Test Interface Register 2"]
pub mod phy_pllcfgfreq2;
#[doc = "PHY_I2CM_SLAVE (rw) register accessor: PHY I2C Slave Address Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_i2cm_slave::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_i2cm_slave::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_i2cm_slave`]
module"]
#[doc(alias = "PHY_I2CM_SLAVE")]
pub type PhyI2cmSlave = crate::Reg<phy_i2cm_slave::PhyI2cmSlaveSpec>;
#[doc = "PHY I2C Slave Address Configuration Register"]
pub mod phy_i2cm_slave;
#[doc = "PHY_I2CM_ADDRESS (rw) register accessor: PHY I2C Address Configuration Register\n\nThis register writes the address for read and write operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_i2cm_address::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_i2cm_address::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_i2cm_address`]
module"]
#[doc(alias = "PHY_I2CM_ADDRESS")]
pub type PhyI2cmAddress = crate::Reg<phy_i2cm_address::PhyI2cmAddressSpec>;
#[doc = "PHY I2C Address Configuration Register\n\nThis register writes the address for read and write operations."]
pub mod phy_i2cm_address;
#[doc = "PHY_I2CM_DATAO_1 (rw) register accessor: PHY I2C Data Write Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_i2cm_datao_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_i2cm_datao_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_i2cm_datao_1`]
module"]
#[doc(alias = "PHY_I2CM_DATAO_1")]
pub type PhyI2cmDatao1 = crate::Reg<phy_i2cm_datao_1::PhyI2cmDatao1Spec>;
#[doc = "PHY I2C Data Write Register 1"]
pub mod phy_i2cm_datao_1;
#[doc = "PHY_I2CM_DATAO_0 (rw) register accessor: PHY I2C Data Write Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_i2cm_datao_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_i2cm_datao_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_i2cm_datao_0`]
module"]
#[doc(alias = "PHY_I2CM_DATAO_0")]
pub type PhyI2cmDatao0 = crate::Reg<phy_i2cm_datao_0::PhyI2cmDatao0Spec>;
#[doc = "PHY I2C Data Write Register 0"]
pub mod phy_i2cm_datao_0;
#[doc = "PHY_I2CM_DATAI_1 (r) register accessor: PHY I2C Data Read Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_i2cm_datai_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_i2cm_datai_1`]
module"]
#[doc(alias = "PHY_I2CM_DATAI_1")]
pub type PhyI2cmDatai1 = crate::Reg<phy_i2cm_datai_1::PhyI2cmDatai1Spec>;
#[doc = "PHY I2C Data Read Register 1"]
pub mod phy_i2cm_datai_1;
#[doc = "PHY_I2CM_DATAI_0 (r) register accessor: PHY I2C Data Read Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_i2cm_datai_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_i2cm_datai_0`]
module"]
#[doc(alias = "PHY_I2CM_DATAI_0")]
pub type PhyI2cmDatai0 = crate::Reg<phy_i2cm_datai_0::PhyI2cmDatai0Spec>;
#[doc = "PHY I2C Data Read Register 0"]
pub mod phy_i2cm_datai_0;
#[doc = "PHY_I2CM_OPERATION (w) register accessor: PHY I2C RD/RD_EXT/WR Operation Register\n\nThis register requests read and write operations from the I2C Master PHY. This register can\n\nonly be written; reading this register always results in 00h. Writing 1'b1 simultaneously to\n\nread and write requests is considered a read request.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_i2cm_operation::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_i2cm_operation`]
module"]
#[doc(alias = "PHY_I2CM_OPERATION")]
pub type PhyI2cmOperation = crate::Reg<phy_i2cm_operation::PhyI2cmOperationSpec>;
#[doc = "PHY I2C RD/RD_EXT/WR Operation Register\n\nThis register requests read and write operations from the I2C Master PHY. This register can\n\nonly be written; reading this register always results in 00h. Writing 1'b1 simultaneously to\n\nread and write requests is considered a read request."]
pub mod phy_i2cm_operation;
#[doc = "PHY_I2CM_INT (rw) register accessor: PHY I2C Done Interrupt Register\n\nThis register contains and configures I2C master PHY done interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_i2cm_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_i2cm_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_i2cm_int`]
module"]
#[doc(alias = "PHY_I2CM_INT")]
pub type PhyI2cmInt = crate::Reg<phy_i2cm_int::PhyI2cmIntSpec>;
#[doc = "PHY I2C Done Interrupt Register\n\nThis register contains and configures I2C master PHY done interrupt."]
pub mod phy_i2cm_int;
#[doc = "PHY_I2CM_CTLINT (rw) register accessor: PHY I2C error Interrupt Register\n\nThis register contains and configures the I2C master PHY error interrupts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_i2cm_ctlint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_i2cm_ctlint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_i2cm_ctlint`]
module"]
#[doc(alias = "PHY_I2CM_CTLINT")]
pub type PhyI2cmCtlint = crate::Reg<phy_i2cm_ctlint::PhyI2cmCtlintSpec>;
#[doc = "PHY I2C error Interrupt Register\n\nThis register contains and configures the I2C master PHY error interrupts."]
pub mod phy_i2cm_ctlint;
#[doc = "PHY_I2CM_DIV (rw) register accessor: PHY I2C Speed control Register\n\nThis register wets the I2C Master PHY to work in either Fast or Standard mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_i2cm_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_i2cm_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_i2cm_div`]
module"]
#[doc(alias = "PHY_I2CM_DIV")]
pub type PhyI2cmDiv = crate::Reg<phy_i2cm_div::PhyI2cmDivSpec>;
#[doc = "PHY I2C Speed control Register\n\nThis register wets the I2C Master PHY to work in either Fast or Standard mode."]
pub mod phy_i2cm_div;
#[doc = "PHY_I2CM_SOFTRSTZ (rw) register accessor: PHY I2C SW reset control register\n\nThis register sets the I2C Master PHY software reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_i2cm_softrstz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_i2cm_softrstz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_i2cm_softrstz`]
module"]
#[doc(alias = "PHY_I2CM_SOFTRSTZ")]
pub type PhyI2cmSoftrstz = crate::Reg<phy_i2cm_softrstz::PhyI2cmSoftrstzSpec>;
#[doc = "PHY I2C SW reset control register\n\nThis register sets the I2C Master PHY software reset."]
pub mod phy_i2cm_softrstz;
#[doc = "PHY_I2CM_SS_SCL_HCNT_1_ADDR (rw) register accessor: PHY I2C Slow Speed SCL High Level Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_i2cm_ss_scl_hcnt_1_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_i2cm_ss_scl_hcnt_1_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_i2cm_ss_scl_hcnt_1_addr`]
module"]
#[doc(alias = "PHY_I2CM_SS_SCL_HCNT_1_ADDR")]
pub type PhyI2cmSsSclHcnt1Addr = crate::Reg<phy_i2cm_ss_scl_hcnt_1_addr::PhyI2cmSsSclHcnt1AddrSpec>;
#[doc = "PHY I2C Slow Speed SCL High Level Control Register 1"]
pub mod phy_i2cm_ss_scl_hcnt_1_addr;
#[doc = "PHY_I2CM_SS_SCL_HCNT_0_ADDR (rw) register accessor: PHY I2C Slow Speed SCL High Level Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_i2cm_ss_scl_hcnt_0_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_i2cm_ss_scl_hcnt_0_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_i2cm_ss_scl_hcnt_0_addr`]
module"]
#[doc(alias = "PHY_I2CM_SS_SCL_HCNT_0_ADDR")]
pub type PhyI2cmSsSclHcnt0Addr = crate::Reg<phy_i2cm_ss_scl_hcnt_0_addr::PhyI2cmSsSclHcnt0AddrSpec>;
#[doc = "PHY I2C Slow Speed SCL High Level Control Register 0"]
pub mod phy_i2cm_ss_scl_hcnt_0_addr;
#[doc = "PHY_I2CM_SS_SCL_LCNT_1_ADDR (rw) register accessor: PHY I2C Slow Speed SCL Low Level Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_i2cm_ss_scl_lcnt_1_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_i2cm_ss_scl_lcnt_1_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_i2cm_ss_scl_lcnt_1_addr`]
module"]
#[doc(alias = "PHY_I2CM_SS_SCL_LCNT_1_ADDR")]
pub type PhyI2cmSsSclLcnt1Addr = crate::Reg<phy_i2cm_ss_scl_lcnt_1_addr::PhyI2cmSsSclLcnt1AddrSpec>;
#[doc = "PHY I2C Slow Speed SCL Low Level Control Register 1"]
pub mod phy_i2cm_ss_scl_lcnt_1_addr;
#[doc = "PHY_I2CM_SS_SCL_LCNT_0_ADDR (rw) register accessor: PHY I2C Slow Speed SCL Low Level Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_i2cm_ss_scl_lcnt_0_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_i2cm_ss_scl_lcnt_0_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_i2cm_ss_scl_lcnt_0_addr`]
module"]
#[doc(alias = "PHY_I2CM_SS_SCL_LCNT_0_ADDR")]
pub type PhyI2cmSsSclLcnt0Addr = crate::Reg<phy_i2cm_ss_scl_lcnt_0_addr::PhyI2cmSsSclLcnt0AddrSpec>;
#[doc = "PHY I2C Slow Speed SCL Low Level Control Register 0"]
pub mod phy_i2cm_ss_scl_lcnt_0_addr;
#[doc = "PHY_I2CM_FS_SCL_HCNT_1_ADDR (rw) register accessor: PHY I2C Fast Speed SCL High Level Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_i2cm_fs_scl_hcnt_1_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_i2cm_fs_scl_hcnt_1_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_i2cm_fs_scl_hcnt_1_addr`]
module"]
#[doc(alias = "PHY_I2CM_FS_SCL_HCNT_1_ADDR")]
pub type PhyI2cmFsSclHcnt1Addr = crate::Reg<phy_i2cm_fs_scl_hcnt_1_addr::PhyI2cmFsSclHcnt1AddrSpec>;
#[doc = "PHY I2C Fast Speed SCL High Level Control Register 1"]
pub mod phy_i2cm_fs_scl_hcnt_1_addr;
#[doc = "PHY_I2CM_FS_SCL_HCNT_0_ADDR (rw) register accessor: PHY I2C Fast Speed SCL High Level Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_i2cm_fs_scl_hcnt_0_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_i2cm_fs_scl_hcnt_0_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_i2cm_fs_scl_hcnt_0_addr`]
module"]
#[doc(alias = "PHY_I2CM_FS_SCL_HCNT_0_ADDR")]
pub type PhyI2cmFsSclHcnt0Addr = crate::Reg<phy_i2cm_fs_scl_hcnt_0_addr::PhyI2cmFsSclHcnt0AddrSpec>;
#[doc = "PHY I2C Fast Speed SCL High Level Control Register 0"]
pub mod phy_i2cm_fs_scl_hcnt_0_addr;
#[doc = "PHY_I2CM_FS_SCL_LCNT_1_ADDR (rw) register accessor: PHY I2C Fast Speed SCL Low Level Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_i2cm_fs_scl_lcnt_1_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_i2cm_fs_scl_lcnt_1_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_i2cm_fs_scl_lcnt_1_addr`]
module"]
#[doc(alias = "PHY_I2CM_FS_SCL_LCNT_1_ADDR")]
pub type PhyI2cmFsSclLcnt1Addr = crate::Reg<phy_i2cm_fs_scl_lcnt_1_addr::PhyI2cmFsSclLcnt1AddrSpec>;
#[doc = "PHY I2C Fast Speed SCL Low Level Control Register 1"]
pub mod phy_i2cm_fs_scl_lcnt_1_addr;
#[doc = "PHY_I2CM_FS_SCL_LCNT_0_ADDR (rw) register accessor: PHY I2C Fast Speed SCL Low Level Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_i2cm_fs_scl_lcnt_0_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_i2cm_fs_scl_lcnt_0_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_i2cm_fs_scl_lcnt_0_addr`]
module"]
#[doc(alias = "PHY_I2CM_FS_SCL_LCNT_0_ADDR")]
pub type PhyI2cmFsSclLcnt0Addr = crate::Reg<phy_i2cm_fs_scl_lcnt_0_addr::PhyI2cmFsSclLcnt0AddrSpec>;
#[doc = "PHY I2C Fast Speed SCL Low Level Control Register 0"]
pub mod phy_i2cm_fs_scl_lcnt_0_addr;
#[doc = "PHY_I2CM_SDA_HOLD (rw) register accessor: PHY I2C SDA HOLD Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_i2cm_sda_hold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_i2cm_sda_hold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_i2cm_sda_hold`]
module"]
#[doc(alias = "PHY_I2CM_SDA_HOLD")]
pub type PhyI2cmSdaHold = crate::Reg<phy_i2cm_sda_hold::PhyI2cmSdaHoldSpec>;
#[doc = "PHY I2C SDA HOLD Control Register"]
pub mod phy_i2cm_sda_hold;
#[doc = "JTAG_PHY_CONFIG (rw) register accessor: PHY I2C/JTAG I/O Configuration Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jtag_phy_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jtag_phy_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jtag_phy_config`]
module"]
#[doc(alias = "JTAG_PHY_CONFIG")]
pub type JtagPhyConfig = crate::Reg<jtag_phy_config::JtagPhyConfigSpec>;
#[doc = "PHY I2C/JTAG I/O Configuration Control Register"]
pub mod jtag_phy_config;
#[doc = "JTAG_PHY_TAP_TCK (rw) register accessor: PHY JTAG Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jtag_phy_tap_tck::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jtag_phy_tap_tck::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jtag_phy_tap_tck`]
module"]
#[doc(alias = "JTAG_PHY_TAP_TCK")]
pub type JtagPhyTapTck = crate::Reg<jtag_phy_tap_tck::JtagPhyTapTckSpec>;
#[doc = "PHY JTAG Clock Control Register"]
pub mod jtag_phy_tap_tck;
#[doc = "JTAG_PHY_TAP_IN (rw) register accessor: PHY JTAG TAP In Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jtag_phy_tap_in::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jtag_phy_tap_in::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jtag_phy_tap_in`]
module"]
#[doc(alias = "JTAG_PHY_TAP_IN")]
pub type JtagPhyTapIn = crate::Reg<jtag_phy_tap_in::JtagPhyTapInSpec>;
#[doc = "PHY JTAG TAP In Control Register"]
pub mod jtag_phy_tap_in;
#[doc = "JTAG_PHY_TAP_OUT (r) register accessor: PHY JTAG TAP Out Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jtag_phy_tap_out::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jtag_phy_tap_out`]
module"]
#[doc(alias = "JTAG_PHY_TAP_OUT")]
pub type JtagPhyTapOut = crate::Reg<jtag_phy_tap_out::JtagPhyTapOutSpec>;
#[doc = "PHY JTAG TAP Out Control Register"]
pub mod jtag_phy_tap_out;
#[doc = "JTAG_PHY_ADDR (rw) register accessor: PHY JTAG Address Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jtag_phy_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jtag_phy_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jtag_phy_addr`]
module"]
#[doc(alias = "JTAG_PHY_ADDR")]
pub type JtagPhyAddr = crate::Reg<jtag_phy_addr::JtagPhyAddrSpec>;
#[doc = "PHY JTAG Address Control Register"]
pub mod jtag_phy_addr;
#[doc = "AUD_CONF0 (rw) register accessor: Audio I2S Software FIFO Reset, Select, and Enable Control Register 0\n\nThis register configures the I2S input enable that indicates which input I2S channels have\n\nvalid data. It also allows the system processor to reset audio FIFOs upon\n\nunderflow/overflow error detection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aud_conf0`]
module"]
#[doc(alias = "AUD_CONF0")]
pub type AudConf0 = crate::Reg<aud_conf0::AudConf0Spec>;
#[doc = "Audio I2S Software FIFO Reset, Select, and Enable Control Register 0\n\nThis register configures the I2S input enable that indicates which input I2S channels have\n\nvalid data. It also allows the system processor to reset audio FIFOs upon\n\nunderflow/overflow error detection."]
pub mod aud_conf0;
#[doc = "AUD_CONF1 (rw) register accessor: Audio I2S Width Configuration Register 1 This register configures the data\n\nwidth of the input data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aud_conf1`]
module"]
#[doc(alias = "AUD_CONF1")]
pub type AudConf1 = crate::Reg<aud_conf1::AudConf1Spec>;
#[doc = "Audio I2S Width Configuration Register 1 This register configures the data\n\nwidth of the input data."]
pub mod aud_conf1;
#[doc = "AUD_INT (rw) register accessor: I2S FIFO status and interrupts.\n\nThis register configures the I2S FIFO status and interrupts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aud_int`]
module"]
#[doc(alias = "AUD_INT")]
pub type AudInt = crate::Reg<aud_int::AudIntSpec>;
#[doc = "I2S FIFO status and interrupts.\n\nThis register configures the I2S FIFO status and interrupts."]
pub mod aud_int;
#[doc = "AUD_CONF2 (rw) register accessor: Audio I2S PCUV, NLPCM and HBR configuration Register 2\n\nThis register configures the I2S Audio Data mapping. By default, audio data mapping is the\n\nstandard I2S Linear PCM (L-PCM) mapping. You can choose to use the I2S interface to\n\ntransport HBR or Non- Linear PCM (NL-PCM) audio, by setting the relevant bit in this\n\nregister.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aud_conf2`]
module"]
#[doc(alias = "AUD_CONF2")]
pub type AudConf2 = crate::Reg<aud_conf2::AudConf2Spec>;
#[doc = "Audio I2S PCUV, NLPCM and HBR configuration Register 2\n\nThis register configures the I2S Audio Data mapping. By default, audio data mapping is the\n\nstandard I2S Linear PCM (L-PCM) mapping. You can choose to use the I2S interface to\n\ntransport HBR or Non- Linear PCM (NL-PCM) audio, by setting the relevant bit in this\n\nregister."]
pub mod aud_conf2;
#[doc = "AUD_N1 (rw) register accessor: Audio Clock Regenerator N Value Register 1 For N expected values, refer to\n\nthe HDMI 1.4b specification.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_n1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_n1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aud_n1`]
module"]
#[doc(alias = "AUD_N1")]
pub type AudN1 = crate::Reg<aud_n1::AudN1Spec>;
#[doc = "Audio Clock Regenerator N Value Register 1 For N expected values, refer to\n\nthe HDMI 1.4b specification."]
pub mod aud_n1;
#[doc = "AUD_N2 (rw) register accessor: Audio Clock Regenerator N Value Register 2 For N expected values, refer to\n\nthe HDMI 1.4b specification.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_n2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_n2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aud_n2`]
module"]
#[doc(alias = "AUD_N2")]
pub type AudN2 = crate::Reg<aud_n2::AudN2Spec>;
#[doc = "Audio Clock Regenerator N Value Register 2 For N expected values, refer to\n\nthe HDMI 1.4b specification."]
pub mod aud_n2;
#[doc = "AUD_N3 (rw) register accessor: Audio Clock Regenerator N Value Register 3 For N expected values, refer to\n\nthe HDMI 1.4b specification.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_n3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_n3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aud_n3`]
module"]
#[doc(alias = "AUD_N3")]
pub type AudN3 = crate::Reg<aud_n3::AudN3Spec>;
#[doc = "Audio Clock Regenerator N Value Register 3 For N expected values, refer to\n\nthe HDMI 1.4b specification."]
pub mod aud_n3;
#[doc = "AUD_CTS1 (rw) register accessor: Audio Clock Regenerator CTS Value Register 1 For CTS expected values, refer\n\nto the HDMI 1.4b specification.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_cts1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_cts1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aud_cts1`]
module"]
#[doc(alias = "AUD_CTS1")]
pub type AudCts1 = crate::Reg<aud_cts1::AudCts1Spec>;
#[doc = "Audio Clock Regenerator CTS Value Register 1 For CTS expected values, refer\n\nto the HDMI 1.4b specification."]
pub mod aud_cts1;
#[doc = "AUD_CTS2 (rw) register accessor: Audio Clock Regenerator CTS Register 2\n\nFor CTS expected values, refer to the HDMI 1.4b specification.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_cts2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_cts2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aud_cts2`]
module"]
#[doc(alias = "AUD_CTS2")]
pub type AudCts2 = crate::Reg<aud_cts2::AudCts2Spec>;
#[doc = "Audio Clock Regenerator CTS Register 2\n\nFor CTS expected values, refer to the HDMI 1.4b specification."]
pub mod aud_cts2;
#[doc = "AUD_CTS3 (rw) register accessor: Audio Clock Regenerator CTS value Register 3. For CTS expected values, refer\n\nto the HDMI 1.4b specification.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_cts3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_cts3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aud_cts3`]
module"]
#[doc(alias = "AUD_CTS3")]
pub type AudCts3 = crate::Reg<aud_cts3::AudCts3Spec>;
#[doc = "Audio Clock Regenerator CTS value Register 3. For CTS expected values, refer\n\nto the HDMI 1.4b specification."]
pub mod aud_cts3;
#[doc = "AUD_INPUTCLKFS (rw) register accessor: Audio Input Clock FS Factor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_inputclkfs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_inputclkfs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aud_inputclkfs`]
module"]
#[doc(alias = "AUD_INPUTCLKFS")]
pub type AudInputclkfs = crate::Reg<aud_inputclkfs::AudInputclkfsSpec>;
#[doc = "Audio Input Clock FS Factor Register"]
pub mod aud_inputclkfs;
#[doc = "AUD_CTS_DITHER (rw) register accessor: Audio CTS Dither Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_cts_dither::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_cts_dither::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aud_cts_dither`]
module"]
#[doc(alias = "AUD_CTS_DITHER")]
pub type AudCtsDither = crate::Reg<aud_cts_dither::AudCtsDitherSpec>;
#[doc = "Audio CTS Dither Register"]
pub mod aud_cts_dither;
#[doc = "AUD_SPDIF0 (rw) register accessor: Audio SPDIF Software FIFO Reset Control Register 0\n\nThis register allows the system processor to reset audio FIFOs upon underflow/overflow\n\nerror detection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_spdif0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_spdif0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aud_spdif0`]
module"]
#[doc(alias = "AUD_SPDIF0")]
pub type AudSpdif0 = crate::Reg<aud_spdif0::AudSpdif0Spec>;
#[doc = "Audio SPDIF Software FIFO Reset Control Register 0\n\nThis register allows the system processor to reset audio FIFOs upon underflow/overflow\n\nerror detection."]
pub mod aud_spdif0;
#[doc = "AUD_SPDIF1 (rw) register accessor: Audio SPDIF NLPCM and Width Configuration Register 1 This register\n\nconfigures the SPDIF data width.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_spdif1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_spdif1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aud_spdif1`]
module"]
#[doc(alias = "AUD_SPDIF1")]
pub type AudSpdif1 = crate::Reg<aud_spdif1::AudSpdif1Spec>;
#[doc = "Audio SPDIF NLPCM and Width Configuration Register 1 This register\n\nconfigures the SPDIF data width."]
pub mod aud_spdif1;
#[doc = "AUD_SPDIFINT (rw) register accessor: Audio SPDIF FIFO Empty/Full Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_spdifint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_spdifint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aud_spdifint`]
module"]
#[doc(alias = "AUD_SPDIFINT")]
pub type AudSpdifint = crate::Reg<aud_spdifint::AudSpdifintSpec>;
#[doc = "Audio SPDIF FIFO Empty/Full Mask Register"]
pub mod aud_spdifint;
#[doc = "AUD_SPDIFINT1 (rw) register accessor: Audio SPDIF Mask Interrupt Register 1\n\nThis register masks interrupts present in the SPDIF module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_spdifint1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_spdifint1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aud_spdifint1`]
module"]
#[doc(alias = "AUD_SPDIFINT1")]
pub type AudSpdifint1 = crate::Reg<aud_spdifint1::AudSpdifint1Spec>;
#[doc = "Audio SPDIF Mask Interrupt Register 1\n\nThis register masks interrupts present in the SPDIF module."]
pub mod aud_spdifint1;
#[doc = "AUD_SPDIF2 (rw) register accessor: Audio SPDIF Enable Confiiguration Register 2\n\nThis register configures the SPDIF input enable that indicates which input SPDIF channels\n\nhave valid data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_spdif2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_spdif2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aud_spdif2`]
module"]
#[doc(alias = "AUD_SPDIF2")]
pub type AudSpdif2 = crate::Reg<aud_spdif2::AudSpdif2Spec>;
#[doc = "Audio SPDIF Enable Confiiguration Register 2\n\nThis register configures the SPDIF input enable that indicates which input SPDIF channels\n\nhave valid data."]
pub mod aud_spdif2;
#[doc = "GP_CONF0 (rw) register accessor: Audio GPA Software FIFO Reset Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gp_conf0`]
module"]
#[doc(alias = "GP_CONF0")]
pub type GpConf0 = crate::Reg<gp_conf0::GpConf0Spec>;
#[doc = "Audio GPA Software FIFO Reset Control Register 0"]
pub mod gp_conf0;
#[doc = "GP_CONF1 (rw) register accessor: Audio GPA Channel Enable Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gp_conf1`]
module"]
#[doc(alias = "GP_CONF1")]
pub type GpConf1 = crate::Reg<gp_conf1::GpConf1Spec>;
#[doc = "Audio GPA Channel Enable Configuration Register 1"]
pub mod gp_conf1;
#[doc = "GP_CONF2 (rw) register accessor: Audio GPA HBR Enable Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gp_conf2`]
module"]
#[doc(alias = "GP_CONF2")]
pub type GpConf2 = crate::Reg<gp_conf2::GpConf2Spec>;
#[doc = "Audio GPA HBR Enable Register 2"]
pub mod gp_conf2;
#[doc = "GP_MASK (rw) register accessor: Audio GPA FIFO Full and Empty Mask Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gp_mask`]
module"]
#[doc(alias = "GP_MASK")]
pub type GpMask = crate::Reg<gp_mask::GpMaskSpec>;
#[doc = "Audio GPA FIFO Full and Empty Mask Interrupt Register"]
pub mod gp_mask;
#[doc = "AHB_DMA_CONF0 (rw) register accessor: Audio DMA SW FIFO reset and DMA Configuration Register 0\n\nThis register contains the software reset bit for the audio FIFOs. It also configures\n\noperating modes of the AHB master.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_dma_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_dma_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_conf0`]
module"]
#[doc(alias = "AHB_DMA_CONF0")]
pub type AhbDmaConf0 = crate::Reg<ahb_dma_conf0::AhbDmaConf0Spec>;
#[doc = "Audio DMA SW FIFO reset and DMA Configuration Register 0\n\nThis register contains the software reset bit for the audio FIFOs. It also configures\n\noperating modes of the AHB master."]
pub mod ahb_dma_conf0;
#[doc = "AHB_DMA_START (rw) register accessor: Audio DMA Start Register\n\nThe start_dma_transaction bit field signals the AHB audio DMA to start accessing system\n\nmemory in order to fetch data samples to store in the FIFO. After the operation starts, a\n\nnew request for a DMA start is ignored until the DMA is stopped or it reaches the end\n\naddress. Only in one of these situations is a new start request acknowledged.\n\nThe first DMA burst request after start_dma_transaction configuration uses\n\ninitial_addr\\[31:0\\]
as ohaddr\\[31:0\\]
value; mburstlength\\[8:0\\]
is set to the maximum\n\nadmissible value. This maximum value is constrained by the size of buffer provided, the\n\ninstantiated FIFO depth, or/and the number of words up to the next 1 Kbyte boundary.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_dma_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_dma_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_start`]
module"]
#[doc(alias = "AHB_DMA_START")]
pub type AhbDmaStart = crate::Reg<ahb_dma_start::AhbDmaStartSpec>;
#[doc = "Audio DMA Start Register\n\nThe start_dma_transaction bit field signals the AHB audio DMA to start accessing system\n\nmemory in order to fetch data samples to store in the FIFO. After the operation starts, a\n\nnew request for a DMA start is ignored until the DMA is stopped or it reaches the end\n\naddress. Only in one of these situations is a new start request acknowledged.\n\nThe first DMA burst request after start_dma_transaction configuration uses\n\ninitial_addr\\[31:0\\]
as ohaddr\\[31:0\\]
value; mburstlength\\[8:0\\]
is set to the maximum\n\nadmissible value. This maximum value is constrained by the size of buffer provided, the\n\ninstantiated FIFO depth, or/and the number of words up to the next 1 Kbyte boundary."]
pub mod ahb_dma_start;
#[doc = "AHB_DMA_STOP (rw) register accessor: Audio DMA Stop Register\n\nThe stop_dma_transaction bit field signals the AHB audio DMA to stop current Attr. After it\n\nstops, if a new start DMA operation is requested, the DMA engine restarts the Attr using\n\nthe initial_addr\\[31:0\\], which is programmed at ahb_dma_straddr0 to ahb_dma_straddr3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_dma_stop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_dma_stop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_stop`]
module"]
#[doc(alias = "AHB_DMA_STOP")]
pub type AhbDmaStop = crate::Reg<ahb_dma_stop::AhbDmaStopSpec>;
#[doc = "Audio DMA Stop Register\n\nThe stop_dma_transaction bit field signals the AHB audio DMA to stop current Attr. After it\n\nstops, if a new start DMA operation is requested, the DMA engine restarts the Attr using\n\nthe initial_addr\\[31:0\\], which is programmed at ahb_dma_straddr0 to ahb_dma_straddr3."]
pub mod ahb_dma_stop;
#[doc = "AHB_DMA_THRSLD (rw) register accessor: Audio DMA FIFO Threshold Register\n\nThis register defines the FIFO medium threshold occupation value.\n\nAfter the AHB master completes a burst transaction successfully, the FIFO may remain full\n\ntill the data fetch interface requests samples. Each data fetch operation reduces the\n\nnumber of samples stored in the FIFO by the number of channels enabled.\n\nTherefore, the fifo_threshold\\[7:0\\]
is the medium number of samples that should be\n\navailable in the audio FIFO across the DMA operation.\n\nAs soon as the number of samples in the FIFO drops lower than the fifo_threshold\\[7:0\\], the\n\nDMA engine requests a new burst of samples for the AHB master. The length is constrained\n\nby the size of buffer provided, the instantiated FIFO depth minus fifo_threshold\\[7:0\\],\n\nand/or the number of words up to the next 1 kbyte boundary.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_dma_thrsld::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_dma_thrsld::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_thrsld`]
module"]
#[doc(alias = "AHB_DMA_THRSLD")]
pub type AhbDmaThrsld = crate::Reg<ahb_dma_thrsld::AhbDmaThrsldSpec>;
#[doc = "Audio DMA FIFO Threshold Register\n\nThis register defines the FIFO medium threshold occupation value.\n\nAfter the AHB master completes a burst transaction successfully, the FIFO may remain full\n\ntill the data fetch interface requests samples. Each data fetch operation reduces the\n\nnumber of samples stored in the FIFO by the number of channels enabled.\n\nTherefore, the fifo_threshold\\[7:0\\]
is the medium number of samples that should be\n\navailable in the audio FIFO across the DMA operation.\n\nAs soon as the number of samples in the FIFO drops lower than the fifo_threshold\\[7:0\\], the\n\nDMA engine requests a new burst of samples for the AHB master. The length is constrained\n\nby the size of buffer provided, the instantiated FIFO depth minus fifo_threshold\\[7:0\\],\n\nand/or the number of words up to the next 1 kbyte boundary."]
pub mod ahb_dma_thrsld;
#[doc = "AHB_DMA_STRADDR_SET0 (rw) register accessor: Audio DMA Start Address Set0 Register Array Address offset: i = 0 to 3\n\nThese registers define the initial_addr\\[31:0\\]
used to initiate the DMA burst read\n\ntransactions upon start_dma_transaction configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_dma_straddr_set0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_dma_straddr_set0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_straddr_set0`]
module"]
#[doc(alias = "AHB_DMA_STRADDR_SET0")]
pub type AhbDmaStraddrSet0 = crate::Reg<ahb_dma_straddr_set0::AhbDmaStraddrSet0Spec>;
#[doc = "Audio DMA Start Address Set0 Register Array Address offset: i = 0 to 3\n\nThese registers define the initial_addr\\[31:0\\]
used to initiate the DMA burst read\n\ntransactions upon start_dma_transaction configuration."]
pub mod ahb_dma_straddr_set0;
#[doc = "AHB_DMA_STPADDR_SET0 (rw) register accessor: Audio DMA Stop Address Set0 Register Array Address offset: i = 0 to 3\n\nThis registers define the final_addr\\[31:0\\]
used as the final point to the DMA burst read\n\ntransactions.\n\nUpon start_dma_transaction configuration, the DMA engine starts requesting burst reads\n\nfrom the external system memory. Each burst read can have a maximum theoretical length\n\nof 256 words (due to the AMBA AHB specification 1 Kbyte boundary burst limitation).\n\nThe DMA engine is responsible for incrementing the burst starting address and defining its\n\ncorresponding burst length to reach the final_addr\\[31:0\\]
address. The last burst request\n\nissued by the DMA engine takes into account that it should only request data until the\n\nfinal_addr\\[31:0\\]
address (included) and for that should calculate the correct burst length.\n\nAfter reaching the final_addr\\[31:0\\]
address, the done interrupt is active to signal\n\ncompletion of DMA operation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_dma_stpaddr_set0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_dma_stpaddr_set0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_stpaddr_set0`]
module"]
#[doc(alias = "AHB_DMA_STPADDR_SET0")]
pub type AhbDmaStpaddrSet0 = crate::Reg<ahb_dma_stpaddr_set0::AhbDmaStpaddrSet0Spec>;
#[doc = "Audio DMA Stop Address Set0 Register Array Address offset: i = 0 to 3\n\nThis registers define the final_addr\\[31:0\\]
used as the final point to the DMA burst read\n\ntransactions.\n\nUpon start_dma_transaction configuration, the DMA engine starts requesting burst reads\n\nfrom the external system memory. Each burst read can have a maximum theoretical length\n\nof 256 words (due to the AMBA AHB specification 1 Kbyte boundary burst limitation).\n\nThe DMA engine is responsible for incrementing the burst starting address and defining its\n\ncorresponding burst length to reach the final_addr\\[31:0\\]
address. The last burst request\n\nissued by the DMA engine takes into account that it should only request data until the\n\nfinal_addr\\[31:0\\]
address (included) and for that should calculate the correct burst length.\n\nAfter reaching the final_addr\\[31:0\\]
address, the done interrupt is active to signal\n\ncompletion of DMA operation."]
pub mod ahb_dma_stpaddr_set0;
#[doc = "AHB_DMA_BSTRADDR (r) register accessor: Audio DMA Burst Start Address Register Array Address offset: i = 0 to 3\n\nThese read-only registers compose the start address of the current burst operation.\n\nburst_start_addr\\[31:0\\]
= haddr\\[31:0\\]
= initial_addr\\[31:0\\]
+ 16.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_dma_bstraddr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_bstraddr`]
module"]
#[doc(alias = "AHB_DMA_BSTRADDR")]
pub type AhbDmaBstraddr = crate::Reg<ahb_dma_bstraddr::AhbDmaBstraddrSpec>;
#[doc = "Audio DMA Burst Start Address Register Array Address offset: i = 0 to 3\n\nThese read-only registers compose the start address of the current burst operation.\n\nburst_start_addr\\[31:0\\]
= haddr\\[31:0\\]
= initial_addr\\[31:0\\]
+ 16."]
pub mod ahb_dma_bstraddr;
#[doc = "AHB_DMA_MBLENGTH0 (r) register accessor: Audio DMA Burst Length Register 0\n\nThis registers holds the length of the current burst operation. As an example, if the first\n\nburst transaction of the AHB audio DMA is a length of 8, then the second burst should start\n\nat address ohaddr\\[31:0\\]
= initial_addr\\[31:0\\]
+ 32.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_dma_mblength0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_mblength0`]
module"]
#[doc(alias = "AHB_DMA_MBLENGTH0")]
pub type AhbDmaMblength0 = crate::Reg<ahb_dma_mblength0::AhbDmaMblength0Spec>;
#[doc = "Audio DMA Burst Length Register 0\n\nThis registers holds the length of the current burst operation. As an example, if the first\n\nburst transaction of the AHB audio DMA is a length of 8, then the second burst should start\n\nat address ohaddr\\[31:0\\]
= initial_addr\\[31:0\\]
+ 32."]
pub mod ahb_dma_mblength0;
#[doc = "AHB_DMA_MBLENGTH1 (r) register accessor: Audio DMA Burst Length Register 1\n\nThis registers holds the length of the current burst operation. As an example, if the first\n\nburst transaction of the AHB audio DMA is a length of 8, then the second burst should start\n\nat address ohaddr\\[31:0\\]
= initial_addr\\[31:0\\]
+ 32.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_dma_mblength1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_mblength1`]
module"]
#[doc(alias = "AHB_DMA_MBLENGTH1")]
pub type AhbDmaMblength1 = crate::Reg<ahb_dma_mblength1::AhbDmaMblength1Spec>;
#[doc = "Audio DMA Burst Length Register 1\n\nThis registers holds the length of the current burst operation. As an example, if the first\n\nburst transaction of the AHB audio DMA is a length of 8, then the second burst should start\n\nat address ohaddr\\[31:0\\]
= initial_addr\\[31:0\\]
+ 32."]
pub mod ahb_dma_mblength1;
#[doc = "AHB_DMA_MASK (rw) register accessor: Audio DMA Mask Interrupt Register\n\nThis register masks each of the interrupts present in the AHB audio DMA module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_dma_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_dma_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_mask`]
module"]
#[doc(alias = "AHB_DMA_MASK")]
pub type AhbDmaMask = crate::Reg<ahb_dma_mask::AhbDmaMaskSpec>;
#[doc = "Audio DMA Mask Interrupt Register\n\nThis register masks each of the interrupts present in the AHB audio DMA module."]
pub mod ahb_dma_mask;
#[doc = "AHB_DMA_CONF1 (rw) register accessor: Audio DMA Channel Enable Configuration Register 1\n\nIn AUDS packet configuration with layout 0 selected, the maximum number of active\n\nchannels is 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_dma_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_dma_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_conf1`]
module"]
#[doc(alias = "AHB_DMA_CONF1")]
pub type AhbDmaConf1 = crate::Reg<ahb_dma_conf1::AhbDmaConf1Spec>;
#[doc = "Audio DMA Channel Enable Configuration Register 1\n\nIn AUDS packet configuration with layout 0 selected, the maximum number of active\n\nchannels is 2."]
pub mod ahb_dma_conf1;
#[doc = "AHB_DMA_BUFFMASK (rw) register accessor: Audio DMA Buffer Mask Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_dma_buffmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_dma_buffmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_buffmask`]
module"]
#[doc(alias = "AHB_DMA_BUFFMASK")]
pub type AhbDmaBuffmask = crate::Reg<ahb_dma_buffmask::AhbDmaBuffmaskSpec>;
#[doc = "Audio DMA Buffer Mask Interrupt Register"]
pub mod ahb_dma_buffmask;
#[doc = "AHB_DMA_MASK1 (rw) register accessor: Audio DMA Mask Interrupt Register 1\n\nThis register masks interrupts present in the AHB audio DMA module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_dma_mask1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_dma_mask1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_mask1`]
module"]
#[doc(alias = "AHB_DMA_MASK1")]
pub type AhbDmaMask1 = crate::Reg<ahb_dma_mask1::AhbDmaMask1Spec>;
#[doc = "Audio DMA Mask Interrupt Register 1\n\nThis register masks interrupts present in the AHB audio DMA module."]
pub mod ahb_dma_mask1;
#[doc = "AHB_DMA_STATUS (r) register accessor: Audio DMA Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_dma_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_status`]
module"]
#[doc(alias = "AHB_DMA_STATUS")]
pub type AhbDmaStatus = crate::Reg<ahb_dma_status::AhbDmaStatusSpec>;
#[doc = "Audio DMA Status"]
pub mod ahb_dma_status;
#[doc = "AHB_DMA_CONF2 (rw) register accessor: Audio DMA Configuration Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_dma_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_dma_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_conf2`]
module"]
#[doc(alias = "AHB_DMA_CONF2")]
pub type AhbDmaConf2 = crate::Reg<ahb_dma_conf2::AhbDmaConf2Spec>;
#[doc = "Audio DMA Configuration Register 2"]
pub mod ahb_dma_conf2;
#[doc = "AHB_DMA_STRADDR_SET1 (rw) register accessor: Audio DMA Start Address Set 1 Register Array Address offset: i = 0 to 3\n\nThese registers define the initial_addr_1\\[31:0\\]
used to initiate the DMA burst read\n\ntransactions upon start_dma_transaction configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_dma_straddr_set1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_dma_straddr_set1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_straddr_set1`]
module"]
#[doc(alias = "AHB_DMA_STRADDR_SET1")]
pub type AhbDmaStraddrSet1 = crate::Reg<ahb_dma_straddr_set1::AhbDmaStraddrSet1Spec>;
#[doc = "Audio DMA Start Address Set 1 Register Array Address offset: i = 0 to 3\n\nThese registers define the initial_addr_1\\[31:0\\]
used to initiate the DMA burst read\n\ntransactions upon start_dma_transaction configuration."]
pub mod ahb_dma_straddr_set1;
#[doc = "AHB_DMA_STPADDR_SET1 (rw) register accessor: Audio DMA Stop Address Set 1 Register Array Address offset: i = 0 to 3\n\nThese registers define the final_addr_1\\[31:0\\]
used as the final point to the DMA burst read\n\ntransactions. Upon start_dma_transaction configuration, the DMA engine starts requesting\n\nburst reads from the external system memory. Each burst read can have a maximum\n\ntheoretical length of 256 words (due to the AMBA AHB specification 1 Kbyte boundary burst\n\nlimitation).\n\nThe DMA engine is responsible for incrementing the burst starting address and defining its\n\ncorresponding burst length to reach the final_addr\\[31:0\\]
address. The last burst request\n\nissued by the DMA engine takes into account that it should only request data until the\n\nfinal_addr\\[31:0\\]
address (included) and for that should calculate the correct burst length.\n\nAfter reaching the final_addr_1\\[31:0\\]
address, the done interrupt is active to indicate\n\ncompletion of the DMA operation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_dma_stpaddr_set1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_dma_stpaddr_set1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_stpaddr_set1`]
module"]
#[doc(alias = "AHB_DMA_STPADDR_SET1")]
pub type AhbDmaStpaddrSet1 = crate::Reg<ahb_dma_stpaddr_set1::AhbDmaStpaddrSet1Spec>;
#[doc = "Audio DMA Stop Address Set 1 Register Array Address offset: i = 0 to 3\n\nThese registers define the final_addr_1\\[31:0\\]
used as the final point to the DMA burst read\n\ntransactions. Upon start_dma_transaction configuration, the DMA engine starts requesting\n\nburst reads from the external system memory. Each burst read can have a maximum\n\ntheoretical length of 256 words (due to the AMBA AHB specification 1 Kbyte boundary burst\n\nlimitation).\n\nThe DMA engine is responsible for incrementing the burst starting address and defining its\n\ncorresponding burst length to reach the final_addr\\[31:0\\]
address. The last burst request\n\nissued by the DMA engine takes into account that it should only request data until the\n\nfinal_addr\\[31:0\\]
address (included) and for that should calculate the correct burst length.\n\nAfter reaching the final_addr_1\\[31:0\\]
address, the done interrupt is active to indicate\n\ncompletion of the DMA operation."]
pub mod ahb_dma_stpaddr_set1;
#[doc = "MC_CLKDIS (rw) register accessor: Main Controller Synchronous Clock Domain Disable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mc_clkdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mc_clkdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mc_clkdis`]
module"]
#[doc(alias = "MC_CLKDIS")]
pub type McClkdis = crate::Reg<mc_clkdis::McClkdisSpec>;
#[doc = "Main Controller Synchronous Clock Domain Disable Register"]
pub mod mc_clkdis;
#[doc = "MC_SWRSTZREQ (rw) register accessor: Main Controller Software Reset Register\n\nMain controller software reset request per clock domain. Writing zero to a bit of this\n\nregister results in an NRZ signal toggle at sfrclk rate to an output signal that indicates a\n\nsoftware reset request. This toggle must be used to generate a synchronized reset to de\n\ncorresponding domain, with at least 1 clock cycle.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mc_swrstzreq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mc_swrstzreq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mc_swrstzreq`]
module"]
#[doc(alias = "MC_SWRSTZREQ")]
pub type McSwrstzreq = crate::Reg<mc_swrstzreq::McSwrstzreqSpec>;
#[doc = "Main Controller Software Reset Register\n\nMain controller software reset request per clock domain. Writing zero to a bit of this\n\nregister results in an NRZ signal toggle at sfrclk rate to an output signal that indicates a\n\nsoftware reset request. This toggle must be used to generate a synchronized reset to de\n\ncorresponding domain, with at least 1 clock cycle."]
pub mod mc_swrstzreq;
#[doc = "MC_OPCTRL (rw) register accessor: Main Controller HDCP Bypass Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mc_opctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mc_opctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mc_opctrl`]
module"]
#[doc(alias = "MC_OPCTRL")]
pub type McOpctrl = crate::Reg<mc_opctrl::McOpctrlSpec>;
#[doc = "Main Controller HDCP Bypass Control Register"]
pub mod mc_opctrl;
#[doc = "MC_FLOWCTRL (rw) register accessor: Main Controller Feed Through Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mc_flowctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mc_flowctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mc_flowctrl`]
module"]
#[doc(alias = "MC_FLOWCTRL")]
pub type McFlowctrl = crate::Reg<mc_flowctrl::McFlowctrlSpec>;
#[doc = "Main Controller Feed Through Control Register"]
pub mod mc_flowctrl;
#[doc = "MC_PHYRSTZ (rw) register accessor: Main Controller PHY Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mc_phyrstz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mc_phyrstz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mc_phyrstz`]
module"]
#[doc(alias = "MC_PHYRSTZ")]
pub type McPhyrstz = crate::Reg<mc_phyrstz::McPhyrstzSpec>;
#[doc = "Main Controller PHY Reset Register"]
pub mod mc_phyrstz;
#[doc = "MC_LOCKONCLOCK (rw) register accessor: Main Controller Clock Present Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mc_lockonclock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mc_lockonclock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mc_lockonclock`]
module"]
#[doc(alias = "MC_LOCKONCLOCK")]
pub type McLockonclock = crate::Reg<mc_lockonclock::McLockonclockSpec>;
#[doc = "Main Controller Clock Present Register"]
pub mod mc_lockonclock;
#[doc = "MC_HEACPHY_RST (rw) register accessor: Main Controller HEAC PHY Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mc_heacphy_rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mc_heacphy_rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mc_heacphy_rst`]
module"]
#[doc(alias = "MC_HEACPHY_RST")]
pub type McHeacphyRst = crate::Reg<mc_heacphy_rst::McHeacphyRstSpec>;
#[doc = "Main Controller HEAC PHY Reset Register"]
pub mod mc_heacphy_rst;
#[doc = "MC_LOCKONCLOCK_2 (rw) register accessor: Main Controller Clock Present Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mc_lockonclock_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mc_lockonclock_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mc_lockonclock_2`]
module"]
#[doc(alias = "MC_LOCKONCLOCK_2")]
pub type McLockonclock2 = crate::Reg<mc_lockonclock_2::McLockonclock2Spec>;
#[doc = "Main Controller Clock Present Register 2"]
pub mod mc_lockonclock_2;
#[doc = "MC_SWRSTZREQ_2 (rw) register accessor: Main Controller Software Reset Register 2\n\nMain controller software reset request per clock domain. Writing zero to a bit of this\n\nregister results in a signal toggle that indicates a software reset request. This toggle is used\n\nto generate a synchronized reset to the corresponding domain, with one or more clock\n\ncycles.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mc_swrstzreq_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mc_swrstzreq_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mc_swrstzreq_2`]
module"]
#[doc(alias = "MC_SWRSTZREQ_2")]
pub type McSwrstzreq2 = crate::Reg<mc_swrstzreq_2::McSwrstzreq2Spec>;
#[doc = "Main Controller Software Reset Register 2\n\nMain controller software reset request per clock domain. Writing zero to a bit of this\n\nregister results in a signal toggle that indicates a software reset request. This toggle is used\n\nto generate a synchronized reset to the corresponding domain, with one or more clock\n\ncycles."]
pub mod mc_swrstzreq_2;
#[doc = "MC_OPSTS (r) register accessor: Main Controller Status Register\n\nThis register contains the information regarding the status of the HDCP SNPS 2.2 versus\n\n1.4 switch.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mc_opsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mc_opsts`]
module"]
#[doc(alias = "MC_OPSTS")]
pub type McOpsts = crate::Reg<mc_opsts::McOpstsSpec>;
#[doc = "Main Controller Status Register\n\nThis register contains the information regarding the status of the HDCP SNPS 2.2 versus\n\n1.4 switch."]
pub mod mc_opsts;
#[doc = "BASE_SFRDIVLOW (rw) register accessor: SFR Clock Base Time Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`base_sfrdivlow::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`base_sfrdivlow::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base_sfrdivlow`]
module"]
#[doc(alias = "BASE_SFRDIVLOW")]
pub type BaseSfrdivlow = crate::Reg<base_sfrdivlow::BaseSfrdivlowSpec>;
#[doc = "SFR Clock Base Time Register Low"]
pub mod base_sfrdivlow;
#[doc = "BASE_SFRDIVHIGH (rw) register accessor: SFR Clock Base Time Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`base_sfrdivhigh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`base_sfrdivhigh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base_sfrdivhigh`]
module"]
#[doc(alias = "BASE_SFRDIVHIGH")]
pub type BaseSfrdivhigh = crate::Reg<base_sfrdivhigh::BaseSfrdivhighSpec>;
#[doc = "SFR Clock Base Time Register High"]
pub mod base_sfrdivhigh;
#[doc = "CSC_CFG (rw) register accessor: Color Space Converter Interpolation and Decimation Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csc_cfg`]
module"]
#[doc(alias = "CSC_CFG")]
pub type CscCfg = crate::Reg<csc_cfg::CscCfgSpec>;
#[doc = "Color Space Converter Interpolation and Decimation Configuration Register"]
pub mod csc_cfg;
#[doc = "CSC_SCALE (rw) register accessor: Color Space Converter Scale and Deep Color Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_scale::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_scale::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csc_scale`]
module"]
#[doc(alias = "CSC_SCALE")]
pub type CscScale = crate::Reg<csc_scale::CscScaleSpec>;
#[doc = "Color Space Converter Scale and Deep Color Configuration Register"]
pub mod csc_scale;
#[doc = "CSC_COEF_A1_MSB (rw) register accessor: Color Space Converter Matrix A1 Coefficient Register MSB Notes:\n\nThe coefficients used in the CSC matrix use only 15 bits for the internal computations.\n\nCoefficients are represented in 2's complementary format and stored in two registers:\n\ncsc_coef_*_lsb\\[7:0\\]: coefficient bits 7 to 0\n\ncsc_coef_*_msb\\[7\\]: spare bit\n\ncsc_coef_*_msb\\[6:0\\]: coefficient bits 14 to 8\n\nExamples for standard ITU601 and ITU709 RGB/YCC conversion CSC coefficients exist in\n\nthe Video Datapath Application Note.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_a1_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_a1_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csc_coef_a1_msb`]
module"]
#[doc(alias = "CSC_COEF_A1_MSB")]
pub type CscCoefA1Msb = crate::Reg<csc_coef_a1_msb::CscCoefA1MsbSpec>;
#[doc = "Color Space Converter Matrix A1 Coefficient Register MSB Notes:\n\nThe coefficients used in the CSC matrix use only 15 bits for the internal computations.\n\nCoefficients are represented in 2's complementary format and stored in two registers:\n\ncsc_coef_*_lsb\\[7:0\\]: coefficient bits 7 to 0\n\ncsc_coef_*_msb\\[7\\]: spare bit\n\ncsc_coef_*_msb\\[6:0\\]: coefficient bits 14 to 8\n\nExamples for standard ITU601 and ITU709 RGB/YCC conversion CSC coefficients exist in\n\nthe Video Datapath Application Note."]
pub mod csc_coef_a1_msb;
#[doc = "CSC_COEF_A1_LSB (rw) register accessor: Color Space Converter Matrix A1 Coefficient Register LSB Notes:\n\nThe coefficients used in the CSC matrix use only 15 bits for the internal computations.\n\nCoefficients are represented in 2's complementary format and stored in two registers:\n\ncsc_coef_*_lsb\\[7:0\\]: coefficient bits 7 to 0\n\ncsc_coef_*_msb\\[7\\]: spare bit\n\ncsc_coef_*_msb\\[6:0\\]: coefficient bits 14 to 8\n\nExamples for standard ITU601 and ITU709 RGB/YCC conversion CSC coefficients exist in\n\nthe Video Datapath Application Note.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_a1_lsb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_a1_lsb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csc_coef_a1_lsb`]
module"]
#[doc(alias = "CSC_COEF_A1_LSB")]
pub type CscCoefA1Lsb = crate::Reg<csc_coef_a1_lsb::CscCoefA1LsbSpec>;
#[doc = "Color Space Converter Matrix A1 Coefficient Register LSB Notes:\n\nThe coefficients used in the CSC matrix use only 15 bits for the internal computations.\n\nCoefficients are represented in 2's complementary format and stored in two registers:\n\ncsc_coef_*_lsb\\[7:0\\]: coefficient bits 7 to 0\n\ncsc_coef_*_msb\\[7\\]: spare bit\n\ncsc_coef_*_msb\\[6:0\\]: coefficient bits 14 to 8\n\nExamples for standard ITU601 and ITU709 RGB/YCC conversion CSC coefficients exist in\n\nthe Video Datapath Application Note."]
pub mod csc_coef_a1_lsb;
#[doc = "CSC_COEF_A2_MSB (rw) register accessor: Color Space Converter Matrix A2 Coefficient Register MSB Color Space\n\nConversion A2 coefficient.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_a2_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_a2_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csc_coef_a2_msb`]
module"]
#[doc(alias = "CSC_COEF_A2_MSB")]
pub type CscCoefA2Msb = crate::Reg<csc_coef_a2_msb::CscCoefA2MsbSpec>;
#[doc = "Color Space Converter Matrix A2 Coefficient Register MSB Color Space\n\nConversion A2 coefficient."]
pub mod csc_coef_a2_msb;
#[doc = "CSC_COEF_A2_LSB (rw) register accessor: Color Space Converter Matrix A2 Coefficient Register LSB Color Space\n\nConversion A2 coefficient.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_a2_lsb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_a2_lsb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csc_coef_a2_lsb`]
module"]
#[doc(alias = "CSC_COEF_A2_LSB")]
pub type CscCoefA2Lsb = crate::Reg<csc_coef_a2_lsb::CscCoefA2LsbSpec>;
#[doc = "Color Space Converter Matrix A2 Coefficient Register LSB Color Space\n\nConversion A2 coefficient."]
pub mod csc_coef_a2_lsb;
#[doc = "CSC_COEF_A3_MSB (rw) register accessor: Color Space Converter Matrix A3 Coefficient Register MSB Color Space\n\nConversion A3 coefficient.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_a3_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_a3_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csc_coef_a3_msb`]
module"]
#[doc(alias = "CSC_COEF_A3_MSB")]
pub type CscCoefA3Msb = crate::Reg<csc_coef_a3_msb::CscCoefA3MsbSpec>;
#[doc = "Color Space Converter Matrix A3 Coefficient Register MSB Color Space\n\nConversion A3 coefficient."]
pub mod csc_coef_a3_msb;
#[doc = "CSC_COEF_A3_LSB (rw) register accessor: Color Space Converter Matrix A3 Coefficient Register LSB Color Space\n\nConversion A3 coefficient.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_a3_lsb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_a3_lsb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csc_coef_a3_lsb`]
module"]
#[doc(alias = "CSC_COEF_A3_LSB")]
pub type CscCoefA3Lsb = crate::Reg<csc_coef_a3_lsb::CscCoefA3LsbSpec>;
#[doc = "Color Space Converter Matrix A3 Coefficient Register LSB Color Space\n\nConversion A3 coefficient."]
pub mod csc_coef_a3_lsb;
#[doc = "CSC_COEF_A4_MSB (rw) register accessor: Color Space Converter Matrix A4 Coefficient Register MSB Color Space\n\nConversion A4 coefficient.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_a4_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_a4_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csc_coef_a4_msb`]
module"]
#[doc(alias = "CSC_COEF_A4_MSB")]
pub type CscCoefA4Msb = crate::Reg<csc_coef_a4_msb::CscCoefA4MsbSpec>;
#[doc = "Color Space Converter Matrix A4 Coefficient Register MSB Color Space\n\nConversion A4 coefficient."]
pub mod csc_coef_a4_msb;
#[doc = "CSC_COEF_A4_LSB (rw) register accessor: Color Space Converter Matrix A4 Coefficient Register LSB Color Space\n\nConversion A4 coefficient.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_a4_lsb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_a4_lsb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csc_coef_a4_lsb`]
module"]
#[doc(alias = "CSC_COEF_A4_LSB")]
pub type CscCoefA4Lsb = crate::Reg<csc_coef_a4_lsb::CscCoefA4LsbSpec>;
#[doc = "Color Space Converter Matrix A4 Coefficient Register LSB Color Space\n\nConversion A4 coefficient."]
pub mod csc_coef_a4_lsb;
#[doc = "CSC_COEF_B1_MSB (rw) register accessor: Color Space Converter Matrix B1 Coefficient Register MSB Color Space\n\nConversion B1 coefficient.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_b1_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_b1_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csc_coef_b1_msb`]
module"]
#[doc(alias = "CSC_COEF_B1_MSB")]
pub type CscCoefB1Msb = crate::Reg<csc_coef_b1_msb::CscCoefB1MsbSpec>;
#[doc = "Color Space Converter Matrix B1 Coefficient Register MSB Color Space\n\nConversion B1 coefficient."]
pub mod csc_coef_b1_msb;
#[doc = "CSC_COEF_B1_LSB (rw) register accessor: Color Space Converter Matrix B1 Coefficient Register LSB Color Space\n\nConversion B1 coefficient.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_b1_lsb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_b1_lsb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csc_coef_b1_lsb`]
module"]
#[doc(alias = "CSC_COEF_B1_LSB")]
pub type CscCoefB1Lsb = crate::Reg<csc_coef_b1_lsb::CscCoefB1LsbSpec>;
#[doc = "Color Space Converter Matrix B1 Coefficient Register LSB Color Space\n\nConversion B1 coefficient."]
pub mod csc_coef_b1_lsb;
#[doc = "CSC_COEF_B2_MSB (rw) register accessor: Color Space Converter Matrix B2 Coefficient Register MSB Color Space\n\nConversion B2 coefficient.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_b2_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_b2_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csc_coef_b2_msb`]
module"]
#[doc(alias = "CSC_COEF_B2_MSB")]
pub type CscCoefB2Msb = crate::Reg<csc_coef_b2_msb::CscCoefB2MsbSpec>;
#[doc = "Color Space Converter Matrix B2 Coefficient Register MSB Color Space\n\nConversion B2 coefficient."]
pub mod csc_coef_b2_msb;
#[doc = "CSC_COEF_B2_LSB (rw) register accessor: Color Space Converter Matrix B2 Coefficient Register LSB Color Space\n\nConversion B2 coefficient.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_b2_lsb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_b2_lsb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csc_coef_b2_lsb`]
module"]
#[doc(alias = "CSC_COEF_B2_LSB")]
pub type CscCoefB2Lsb = crate::Reg<csc_coef_b2_lsb::CscCoefB2LsbSpec>;
#[doc = "Color Space Converter Matrix B2 Coefficient Register LSB Color Space\n\nConversion B2 coefficient."]
pub mod csc_coef_b2_lsb;
#[doc = "CSC_COEF_B3_MSB (rw) register accessor: Color Space Converter Matrix B3 Coefficient Register MSB Color Space\n\nConversion B3 coefficient.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_b3_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_b3_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csc_coef_b3_msb`]
module"]
#[doc(alias = "CSC_COEF_B3_MSB")]
pub type CscCoefB3Msb = crate::Reg<csc_coef_b3_msb::CscCoefB3MsbSpec>;
#[doc = "Color Space Converter Matrix B3 Coefficient Register MSB Color Space\n\nConversion B3 coefficient."]
pub mod csc_coef_b3_msb;
#[doc = "CSC_COEF_B3_LSB (rw) register accessor: Color Space Converter Matrix B3 Coefficient Register LSB Color Space\n\nConversion B3 coefficient.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_b3_lsb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_b3_lsb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csc_coef_b3_lsb`]
module"]
#[doc(alias = "CSC_COEF_B3_LSB")]
pub type CscCoefB3Lsb = crate::Reg<csc_coef_b3_lsb::CscCoefB3LsbSpec>;
#[doc = "Color Space Converter Matrix B3 Coefficient Register LSB Color Space\n\nConversion B3 coefficient."]
pub mod csc_coef_b3_lsb;
#[doc = "CSC_COEF_B4_MSB (rw) register accessor: Color Space Converter Matrix B4 Coefficient Register MSB Color Space\n\nConversion B4 coefficient.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_b4_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_b4_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csc_coef_b4_msb`]
module"]
#[doc(alias = "CSC_COEF_B4_MSB")]
pub type CscCoefB4Msb = crate::Reg<csc_coef_b4_msb::CscCoefB4MsbSpec>;
#[doc = "Color Space Converter Matrix B4 Coefficient Register MSB Color Space\n\nConversion B4 coefficient."]
pub mod csc_coef_b4_msb;
#[doc = "CSC_COEF_B4_LSB (rw) register accessor: Color Space Converter Matrix B4 Coefficient Register LSB Color Space\n\nConversion B4 coefficient.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_b4_lsb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_b4_lsb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csc_coef_b4_lsb`]
module"]
#[doc(alias = "CSC_COEF_B4_LSB")]
pub type CscCoefB4Lsb = crate::Reg<csc_coef_b4_lsb::CscCoefB4LsbSpec>;
#[doc = "Color Space Converter Matrix B4 Coefficient Register LSB Color Space\n\nConversion B4 coefficient."]
pub mod csc_coef_b4_lsb;
#[doc = "CSC_COEF_C1_MSB (rw) register accessor: Color Space Converter Matrix C1 Coefficient Register MSB Color Space\n\nConversion C1 coefficient.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_c1_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_c1_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csc_coef_c1_msb`]
module"]
#[doc(alias = "CSC_COEF_C1_MSB")]
pub type CscCoefC1Msb = crate::Reg<csc_coef_c1_msb::CscCoefC1MsbSpec>;
#[doc = "Color Space Converter Matrix C1 Coefficient Register MSB Color Space\n\nConversion C1 coefficient."]
pub mod csc_coef_c1_msb;
#[doc = "CSC_COEF_C1_LSB (rw) register accessor: Color Space Converter Matrix C1 Coefficient Register LSB Color Space\n\nConversion C1 coefficient.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_c1_lsb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_c1_lsb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csc_coef_c1_lsb`]
module"]
#[doc(alias = "CSC_COEF_C1_LSB")]
pub type CscCoefC1Lsb = crate::Reg<csc_coef_c1_lsb::CscCoefC1LsbSpec>;
#[doc = "Color Space Converter Matrix C1 Coefficient Register LSB Color Space\n\nConversion C1 coefficient."]
pub mod csc_coef_c1_lsb;
#[doc = "CSC_COEF_C2_MSB (rw) register accessor: Color Space Converter Matrix C2 Coefficient Register MSB Color Space\n\nConversion C2 coefficient.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_c2_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_c2_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csc_coef_c2_msb`]
module"]
#[doc(alias = "CSC_COEF_C2_MSB")]
pub type CscCoefC2Msb = crate::Reg<csc_coef_c2_msb::CscCoefC2MsbSpec>;
#[doc = "Color Space Converter Matrix C2 Coefficient Register MSB Color Space\n\nConversion C2 coefficient."]
pub mod csc_coef_c2_msb;
#[doc = "CSC_COEF_C2_LSB (rw) register accessor: Color Space Converter Matrix C2 Coefficient Register LSB Color Space\n\nConversion C2 coefficient.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_c2_lsb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_c2_lsb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csc_coef_c2_lsb`]
module"]
#[doc(alias = "CSC_COEF_C2_LSB")]
pub type CscCoefC2Lsb = crate::Reg<csc_coef_c2_lsb::CscCoefC2LsbSpec>;
#[doc = "Color Space Converter Matrix C2 Coefficient Register LSB Color Space\n\nConversion C2 coefficient."]
pub mod csc_coef_c2_lsb;
#[doc = "CSC_COEF_C3_MSB (rw) register accessor: Color Space Converter Matrix C3 Coefficient Register MSB Color Space\n\nConversion C3 coefficient.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_c3_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_c3_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csc_coef_c3_msb`]
module"]
#[doc(alias = "CSC_COEF_C3_MSB")]
pub type CscCoefC3Msb = crate::Reg<csc_coef_c3_msb::CscCoefC3MsbSpec>;
#[doc = "Color Space Converter Matrix C3 Coefficient Register MSB Color Space\n\nConversion C3 coefficient."]
pub mod csc_coef_c3_msb;
#[doc = "CSC_COEF_C3_LSB (rw) register accessor: Color Space Converter Matrix C3 Coefficient Register LSB Color Space\n\nConversion C3 coefficient.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_c3_lsb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_c3_lsb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csc_coef_c3_lsb`]
module"]
#[doc(alias = "CSC_COEF_C3_LSB")]
pub type CscCoefC3Lsb = crate::Reg<csc_coef_c3_lsb::CscCoefC3LsbSpec>;
#[doc = "Color Space Converter Matrix C3 Coefficient Register LSB Color Space\n\nConversion C3 coefficient."]
pub mod csc_coef_c3_lsb;
#[doc = "CSC_COEF_C4_MSB (rw) register accessor: Color Space Converter Matrix C4 Coefficient Register MSB Color Space\n\nConversion C4 coefficient.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_c4_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_c4_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csc_coef_c4_msb`]
module"]
#[doc(alias = "CSC_COEF_C4_MSB")]
pub type CscCoefC4Msb = crate::Reg<csc_coef_c4_msb::CscCoefC4MsbSpec>;
#[doc = "Color Space Converter Matrix C4 Coefficient Register MSB Color Space\n\nConversion C4 coefficient."]
pub mod csc_coef_c4_msb;
#[doc = "CSC_COEF_C4_LSB (rw) register accessor: Color Space Converter Matrix C4 Coefficient Register LSB Color Space\n\nConversion C4 coefficient.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_c4_lsb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_c4_lsb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csc_coef_c4_lsb`]
module"]
#[doc(alias = "CSC_COEF_C4_LSB")]
pub type CscCoefC4Lsb = crate::Reg<csc_coef_c4_lsb::CscCoefC4LsbSpec>;
#[doc = "Color Space Converter Matrix C4 Coefficient Register LSB Color Space\n\nConversion C4 coefficient."]
pub mod csc_coef_c4_lsb;
#[doc = "CSC_LIMIT_UP_MSB (rw) register accessor: Color Space Converter Matrix Output Up Limit Register MSB\n\nFor more details, refer to the HDMI 1.4 specification, paragraph 6.6 Video Quantization\n\nRanges. For an RGB output of 8 bits, the expected limit is 254, and this register must be\n\nconfigured with 0x00.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_limit_up_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_limit_up_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csc_limit_up_msb`]
module"]
#[doc(alias = "CSC_LIMIT_UP_MSB")]
pub type CscLimitUpMsb = crate::Reg<csc_limit_up_msb::CscLimitUpMsbSpec>;
#[doc = "Color Space Converter Matrix Output Up Limit Register MSB\n\nFor more details, refer to the HDMI 1.4 specification, paragraph 6.6 Video Quantization\n\nRanges. For an RGB output of 8 bits, the expected limit is 254, and this register must be\n\nconfigured with 0x00."]
pub mod csc_limit_up_msb;
#[doc = "CSC_LIMIT_UP_LSB (rw) register accessor: Color Space Converter Matrix output Up Limit Register LSB\n\nFor more details, refer to the HDMI 1.4 specification, paragraph 6.6 Video Quantization\n\nRanges. For an RGB output of 8 bits, the expected limit is 254, and this register must be\n\nconfigured with 0xFE.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_limit_up_lsb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_limit_up_lsb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csc_limit_up_lsb`]
module"]
#[doc(alias = "CSC_LIMIT_UP_LSB")]
pub type CscLimitUpLsb = crate::Reg<csc_limit_up_lsb::CscLimitUpLsbSpec>;
#[doc = "Color Space Converter Matrix output Up Limit Register LSB\n\nFor more details, refer to the HDMI 1.4 specification, paragraph 6.6 Video Quantization\n\nRanges. For an RGB output of 8 bits, the expected limit is 254, and this register must be\n\nconfigured with 0xFE."]
pub mod csc_limit_up_lsb;
#[doc = "CSC_LIMIT_DN_MSB (rw) register accessor: Color Space Converter Matrix output Down Limit Register MSB\n\nFor more details, refer to the HDMI 1.4 specification, paragraph 6.6 Video Quantization\n\nRanges. For an RGB output of 8 bits, the expected limit is 1, and this register must be\n\nconfigured with 0x00.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_limit_dn_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_limit_dn_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csc_limit_dn_msb`]
module"]
#[doc(alias = "CSC_LIMIT_DN_MSB")]
pub type CscLimitDnMsb = crate::Reg<csc_limit_dn_msb::CscLimitDnMsbSpec>;
#[doc = "Color Space Converter Matrix output Down Limit Register MSB\n\nFor more details, refer to the HDMI 1.4 specification, paragraph 6.6 Video Quantization\n\nRanges. For an RGB output of 8 bits, the expected limit is 1, and this register must be\n\nconfigured with 0x00."]
pub mod csc_limit_dn_msb;
#[doc = "CSC_LIMIT_DN_LSB (rw) register accessor: Color Space Converter Matrix output Down Limit Register LSB\n\nFor more details, refer to the HDMI 1.4 specification, paragraph 6.6 Video Quantization\n\nRanges. For an RGB output of 8 bits, the expected limit is 1, and this register must be\n\nconfigured with 0x01.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_limit_dn_lsb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_limit_dn_lsb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csc_limit_dn_lsb`]
module"]
#[doc(alias = "CSC_LIMIT_DN_LSB")]
pub type CscLimitDnLsb = crate::Reg<csc_limit_dn_lsb::CscLimitDnLsbSpec>;
#[doc = "Color Space Converter Matrix output Down Limit Register LSB\n\nFor more details, refer to the HDMI 1.4 specification, paragraph 6.6 Video Quantization\n\nRanges. For an RGB output of 8 bits, the expected limit is 1, and this register must be\n\nconfigured with 0x01."]
pub mod csc_limit_dn_lsb;
#[doc = "A_HDCPCFG0 (rw) register accessor: HDCP Enable and Functional Control Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_hdcpcfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_hdcpcfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a_hdcpcfg0`]
module"]
#[doc(alias = "A_HDCPCFG0")]
pub type AHdcpcfg0 = crate::Reg<a_hdcpcfg0::AHdcpcfg0Spec>;
#[doc = "HDCP Enable and Functional Control Configuration Register 0"]
pub mod a_hdcpcfg0;
#[doc = "A_HDCPCFG1 (rw) register accessor: HDCP Software Reset and Functional Control Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_hdcpcfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_hdcpcfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a_hdcpcfg1`]
module"]
#[doc(alias = "A_HDCPCFG1")]
pub type AHdcpcfg1 = crate::Reg<a_hdcpcfg1::AHdcpcfg1Spec>;
#[doc = "HDCP Software Reset and Functional Control Configuration Register 1"]
pub mod a_hdcpcfg1;
#[doc = "A_HDCPOBS0 (r) register accessor: HDCP Observation Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_hdcpobs0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a_hdcpobs0`]
module"]
#[doc(alias = "A_HDCPOBS0")]
pub type AHdcpobs0 = crate::Reg<a_hdcpobs0::AHdcpobs0Spec>;
#[doc = "HDCP Observation Register 0"]
pub mod a_hdcpobs0;
#[doc = "A_HDCPOBS1 (r) register accessor: HDCP Observation Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_hdcpobs1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a_hdcpobs1`]
module"]
#[doc(alias = "A_HDCPOBS1")]
pub type AHdcpobs1 = crate::Reg<a_hdcpobs1::AHdcpobs1Spec>;
#[doc = "HDCP Observation Register 1"]
pub mod a_hdcpobs1;
#[doc = "A_HDCPOBS2 (r) register accessor: HDCP Observation Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_hdcpobs2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a_hdcpobs2`]
module"]
#[doc(alias = "A_HDCPOBS2")]
pub type AHdcpobs2 = crate::Reg<a_hdcpobs2::AHdcpobs2Spec>;
#[doc = "HDCP Observation Register 2"]
pub mod a_hdcpobs2;
#[doc = "A_HDCPOBS3 (r) register accessor: HDCP Observation Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_hdcpobs3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a_hdcpobs3`]
module"]
#[doc(alias = "A_HDCPOBS3")]
pub type AHdcpobs3 = crate::Reg<a_hdcpobs3::AHdcpobs3Spec>;
#[doc = "HDCP Observation Register 3"]
pub mod a_hdcpobs3;
#[doc = "A_APIINTCLR (w) register accessor: HDCP Interrupt Clear Register\n\nWrite only register, active high and auto cleared, cleans the respective interruption in the\n\ninterrupt status register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_apiintclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a_apiintclr`]
module"]
#[doc(alias = "A_APIINTCLR")]
pub type AApiintclr = crate::Reg<a_apiintclr::AApiintclrSpec>;
#[doc = "HDCP Interrupt Clear Register\n\nWrite only register, active high and auto cleared, cleans the respective interruption in the\n\ninterrupt status register."]
pub mod a_apiintclr;
#[doc = "A_APIINTSTAT (r) register accessor: HDCP Interrupt Status Register\n\nRead only register, reports the interruption which caused the activation of the interruption\n\noutput pin.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_apiintstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a_apiintstat`]
module"]
#[doc(alias = "A_APIINTSTAT")]
pub type AApiintstat = crate::Reg<a_apiintstat::AApiintstatSpec>;
#[doc = "HDCP Interrupt Status Register\n\nRead only register, reports the interruption which caused the activation of the interruption\n\noutput pin."]
pub mod a_apiintstat;
#[doc = "A_APIINTMSK (rw) register accessor: HDCP Interrupt Mask Register\n\nThe configuration of this register mask a given setup of interruption, disabling them from\n\ngenerating interruption pulses in the interruption output pin.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_apiintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_apiintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a_apiintmsk`]
module"]
#[doc(alias = "A_APIINTMSK")]
pub type AApiintmsk = crate::Reg<a_apiintmsk::AApiintmskSpec>;
#[doc = "HDCP Interrupt Mask Register\n\nThe configuration of this register mask a given setup of interruption, disabling them from\n\ngenerating interruption pulses in the interruption output pin."]
pub mod a_apiintmsk;
#[doc = "A_VIDPOLCFG (rw) register accessor: HDCP Video Polarity Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_vidpolcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_vidpolcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a_vidpolcfg`]
module"]
#[doc(alias = "A_VIDPOLCFG")]
pub type AVidpolcfg = crate::Reg<a_vidpolcfg::AVidpolcfgSpec>;
#[doc = "HDCP Video Polarity Configuration Register"]
pub mod a_vidpolcfg;
#[doc = "A_OESSWCFG (rw) register accessor: HDCP OESS WOO Configuration Register\n\nPulse width of the encryption enable (CTL3) signal in the HDCP OESS mode. The window of\n\nopportunity for the Original Encryption Status Signaling starts at the active edge of the\n\nVertical synchronism and stops after oesswindowoffset\\[7:0\\]*4 clock cycles of TMDS clock.\n\nAccording to the HDCP specification, the CTL3 signal must be asserted at least for eight\n\nTMDS clock cycles (oesswindowoffset\\[7:0\\]
must be greater than 1), and it is recommended\n\nto transmit a larger pulse width for enhanced link reliability.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_oesswcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_oesswcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a_oesswcfg`]
module"]
#[doc(alias = "A_OESSWCFG")]
pub type AOesswcfg = crate::Reg<a_oesswcfg::AOesswcfgSpec>;
#[doc = "HDCP OESS WOO Configuration Register\n\nPulse width of the encryption enable (CTL3) signal in the HDCP OESS mode. The window of\n\nopportunity for the Original Encryption Status Signaling starts at the active edge of the\n\nVertical synchronism and stops after oesswindowoffset\\[7:0\\]*4 clock cycles of TMDS clock.\n\nAccording to the HDCP specification, the CTL3 signal must be asserted at least for eight\n\nTMDS clock cycles (oesswindowoffset\\[7:0\\]
must be greater than 1), and it is recommended\n\nto transmit a larger pulse width for enhanced link reliability."]
pub mod a_oesswcfg;
#[doc = "A_COREVERLSB (r) register accessor: HDCP Controller Version Register LSB Design ID number.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_coreverlsb::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a_coreverlsb`]
module"]
#[doc(alias = "A_COREVERLSB")]
pub type ACoreverlsb = crate::Reg<a_coreverlsb::ACoreverlsbSpec>;
#[doc = "HDCP Controller Version Register LSB Design ID number."]
pub mod a_coreverlsb;
#[doc = "A_COREVERMSB (r) register accessor: HDCP Controller Version Register MSB Revision ID number.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_corevermsb::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a_corevermsb`]
module"]
#[doc(alias = "A_COREVERMSB")]
pub type ACorevermsb = crate::Reg<a_corevermsb::ACorevermsbSpec>;
#[doc = "HDCP Controller Version Register MSB Revision ID number."]
pub mod a_corevermsb;
#[doc = "A_KSVMEMCTRL (rw) register accessor: HDCP KSV Memory Control Register\n\nThe KSVCTRLupd bit is a notification flag. This flag changes polarity whenever the register\n\nis written. This flag acts as a trigger to other blocks that processes this data. Upon reset\n\nthe flag returns to low default value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_ksvmemctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_ksvmemctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a_ksvmemctrl`]
module"]
#[doc(alias = "A_KSVMEMCTRL")]
pub type AKsvmemctrl = crate::Reg<a_ksvmemctrl::AKsvmemctrlSpec>;
#[doc = "HDCP KSV Memory Control Register\n\nThe KSVCTRLupd bit is a notification flag. This flag changes polarity whenever the register\n\nis written. This flag acts as a trigger to other blocks that processes this data. Upon reset\n\nthe flag returns to low default value."]
pub mod a_ksvmemctrl;
#[doc = "HDCP_BSTATUS (rw) register accessor: HDCP BStatus Register Array\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp_bstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcp_bstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcp_bstatus`]
module"]
#[doc(alias = "HDCP_BSTATUS")]
pub type HdcpBstatus = crate::Reg<hdcp_bstatus::HdcpBstatusSpec>;
#[doc = "HDCP BStatus Register Array"]
pub mod hdcp_bstatus;
#[doc = "HDCP_M0 (rw) register accessor: HDCP M0 Register Array\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp_m0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcp_m0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcp_m0`]
module"]
#[doc(alias = "HDCP_M0")]
pub type HdcpM0 = crate::Reg<hdcp_m0::HdcpM0Spec>;
#[doc = "HDCP M0 Register Array"]
pub mod hdcp_m0;
#[doc = "HDCP_KSV (rw) register accessor: HDCP KSV Registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp_ksv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcp_ksv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcp_ksv`]
module"]
#[doc(alias = "HDCP_KSV")]
pub type HdcpKsv = crate::Reg<hdcp_ksv::HdcpKsvSpec>;
#[doc = "HDCP KSV Registers."]
pub mod hdcp_ksv;
#[doc = "HDCP_VH (rw) register accessor: HDCP SHA-1 VH Registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp_vh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcp_vh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcp_vh`]
module"]
#[doc(alias = "HDCP_VH")]
pub type HdcpVh = crate::Reg<hdcp_vh::HdcpVhSpec>;
#[doc = "HDCP SHA-1 VH Registers."]
pub mod hdcp_vh;
#[doc = "HDCP_REVOC_SIZE_0 (rw) register accessor: HDCP Revocation KSV List Size Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp_revoc_size_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcp_revoc_size_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcp_revoc_size_0`]
module"]
#[doc(alias = "HDCP_REVOC_SIZE_0")]
pub type HdcpRevocSize0 = crate::Reg<hdcp_revoc_size_0::HdcpRevocSize0Spec>;
#[doc = "HDCP Revocation KSV List Size Register 0"]
pub mod hdcp_revoc_size_0;
#[doc = "HDCP_REVOC_SIZE_1 (rw) register accessor: HDCP Revocation KSV List Size Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp_revoc_size_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcp_revoc_size_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcp_revoc_size_1`]
module"]
#[doc(alias = "HDCP_REVOC_SIZE_1")]
pub type HdcpRevocSize1 = crate::Reg<hdcp_revoc_size_1::HdcpRevocSize1Spec>;
#[doc = "HDCP Revocation KSV List Size Register 1"]
pub mod hdcp_revoc_size_1;
#[doc = "HDCP_REVOC_LIST (rw) register accessor: HDCP Revocation KSV Registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp_revoc_list::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcp_revoc_list::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcp_revoc_list`]
module"]
#[doc(alias = "HDCP_REVOC_LIST")]
pub type HdcpRevocList = crate::Reg<hdcp_revoc_list::HdcpRevocListSpec>;
#[doc = "HDCP Revocation KSV Registers."]
pub mod hdcp_revoc_list;
#[doc = "HDCPREG_BKSV0 (r) register accessor: HDCP KSV Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcpreg_bksv0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcpreg_bksv0`]
module"]
#[doc(alias = "HDCPREG_BKSV0")]
pub type HdcpregBksv0 = crate::Reg<hdcpreg_bksv0::HdcpregBksv0Spec>;
#[doc = "HDCP KSV Status Register 0"]
pub mod hdcpreg_bksv0;
#[doc = "HDCPREG_BKSV1 (r) register accessor: HDCP KSV Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcpreg_bksv1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcpreg_bksv1`]
module"]
#[doc(alias = "HDCPREG_BKSV1")]
pub type HdcpregBksv1 = crate::Reg<hdcpreg_bksv1::HdcpregBksv1Spec>;
#[doc = "HDCP KSV Status Register 1"]
pub mod hdcpreg_bksv1;
#[doc = "HDCPREG_BKSV2 (r) register accessor: HDCP KSV Status Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcpreg_bksv2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcpreg_bksv2`]
module"]
#[doc(alias = "HDCPREG_BKSV2")]
pub type HdcpregBksv2 = crate::Reg<hdcpreg_bksv2::HdcpregBksv2Spec>;
#[doc = "HDCP KSV Status Register 2"]
pub mod hdcpreg_bksv2;
#[doc = "HDCPREG_BKSV3 (r) register accessor: HDCP KSV Status Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcpreg_bksv3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcpreg_bksv3`]
module"]
#[doc(alias = "HDCPREG_BKSV3")]
pub type HdcpregBksv3 = crate::Reg<hdcpreg_bksv3::HdcpregBksv3Spec>;
#[doc = "HDCP KSV Status Register 3"]
pub mod hdcpreg_bksv3;
#[doc = "HDCPREG_BKSV4 (r) register accessor: HDCP KSV Status Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcpreg_bksv4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcpreg_bksv4`]
module"]
#[doc(alias = "HDCPREG_BKSV4")]
pub type HdcpregBksv4 = crate::Reg<hdcpreg_bksv4::HdcpregBksv4Spec>;
#[doc = "HDCP KSV Status Register 4"]
pub mod hdcpreg_bksv4;
#[doc = "HDCPREG_ANCONF (rw) register accessor: HDCP AN Bypass Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcpreg_anconf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_anconf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcpreg_anconf`]
module"]
#[doc(alias = "HDCPREG_ANCONF")]
pub type HdcpregAnconf = crate::Reg<hdcpreg_anconf::HdcpregAnconfSpec>;
#[doc = "HDCP AN Bypass Control Register"]
pub mod hdcpreg_anconf;
#[doc = "HDCPREG_AN0 (rw) register accessor: HDCP Forced AN Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcpreg_an0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_an0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcpreg_an0`]
module"]
#[doc(alias = "HDCPREG_AN0")]
pub type HdcpregAn0 = crate::Reg<hdcpreg_an0::HdcpregAn0Spec>;
#[doc = "HDCP Forced AN Register 0"]
pub mod hdcpreg_an0;
#[doc = "HDCPREG_AN1 (rw) register accessor: HDCP Forced AN Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcpreg_an1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_an1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcpreg_an1`]
module"]
#[doc(alias = "HDCPREG_AN1")]
pub type HdcpregAn1 = crate::Reg<hdcpreg_an1::HdcpregAn1Spec>;
#[doc = "HDCP Forced AN Register 1"]
pub mod hdcpreg_an1;
#[doc = "HDCPREG_AN2 (rw) register accessor: HDCP forced AN Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcpreg_an2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_an2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcpreg_an2`]
module"]
#[doc(alias = "HDCPREG_AN2")]
pub type HdcpregAn2 = crate::Reg<hdcpreg_an2::HdcpregAn2Spec>;
#[doc = "HDCP forced AN Register 2"]
pub mod hdcpreg_an2;
#[doc = "HDCPREG_AN3 (rw) register accessor: HDCP Forced AN Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcpreg_an3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_an3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcpreg_an3`]
module"]
#[doc(alias = "HDCPREG_AN3")]
pub type HdcpregAn3 = crate::Reg<hdcpreg_an3::HdcpregAn3Spec>;
#[doc = "HDCP Forced AN Register 3"]
pub mod hdcpreg_an3;
#[doc = "HDCPREG_AN4 (rw) register accessor: HDCP Forced AN Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcpreg_an4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_an4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcpreg_an4`]
module"]
#[doc(alias = "HDCPREG_AN4")]
pub type HdcpregAn4 = crate::Reg<hdcpreg_an4::HdcpregAn4Spec>;
#[doc = "HDCP Forced AN Register 4"]
pub mod hdcpreg_an4;
#[doc = "HDCPREG_AN5 (rw) register accessor: HDCP Forced AN Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcpreg_an5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_an5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcpreg_an5`]
module"]
#[doc(alias = "HDCPREG_AN5")]
pub type HdcpregAn5 = crate::Reg<hdcpreg_an5::HdcpregAn5Spec>;
#[doc = "HDCP Forced AN Register 5"]
pub mod hdcpreg_an5;
#[doc = "HDCPREG_AN6 (rw) register accessor: HDCP Forced AN Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcpreg_an6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_an6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcpreg_an6`]
module"]
#[doc(alias = "HDCPREG_AN6")]
pub type HdcpregAn6 = crate::Reg<hdcpreg_an6::HdcpregAn6Spec>;
#[doc = "HDCP Forced AN Register 6"]
pub mod hdcpreg_an6;
#[doc = "HDCPREG_AN7 (rw) register accessor: HDCP Forced AN Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcpreg_an7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_an7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcpreg_an7`]
module"]
#[doc(alias = "HDCPREG_AN7")]
pub type HdcpregAn7 = crate::Reg<hdcpreg_an7::HdcpregAn7Spec>;
#[doc = "HDCP Forced AN Register 7"]
pub mod hdcpreg_an7;
#[doc = "HDCPREG_RMLCTL (rw) register accessor: HDCP Encrypted Device Private Keys Control Register\n\nThis register is the control register for the software programmable encrypted DPK\n\nembedded storage feature. The required software configuration sequence is documented in\n\nthe Cores HDMI Transmitter User Guide in the \"Programming\" chapter, Section 3.2.4,\n\n\"Configure HDCP.\"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcpreg_rmlctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_rmlctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcpreg_rmlctl`]
module"]
#[doc(alias = "HDCPREG_RMLCTL")]
pub type HdcpregRmlctl = crate::Reg<hdcpreg_rmlctl::HdcpregRmlctlSpec>;
#[doc = "HDCP Encrypted Device Private Keys Control Register\n\nThis register is the control register for the software programmable encrypted DPK\n\nembedded storage feature. The required software configuration sequence is documented in\n\nthe Cores HDMI Transmitter User Guide in the \"Programming\" chapter, Section 3.2.4,\n\n\"Configure HDCP.\""]
pub mod hdcpreg_rmlctl;
#[doc = "HDCPREG_RMLSTS (r) register accessor: HDCP Encrypted DPK Status Register\n\nThe required software configuration sequence is documented in the Cores HDMI\n\nTransmitter User Guide in the \"Programming\" chapter, Section 3.2.4, \"Configure HDCP.\"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcpreg_rmlsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcpreg_rmlsts`]
module"]
#[doc(alias = "HDCPREG_RMLSTS")]
pub type HdcpregRmlsts = crate::Reg<hdcpreg_rmlsts::HdcpregRmlstsSpec>;
#[doc = "HDCP Encrypted DPK Status Register\n\nThe required software configuration sequence is documented in the Cores HDMI\n\nTransmitter User Guide in the \"Programming\" chapter, Section 3.2.4, \"Configure HDCP.\""]
pub mod hdcpreg_rmlsts;
#[doc = "HDCPREG_SEED0 (w) register accessor: HDCP Encrypted DPK Seed Register 0\n\nThis register contains a byte of the HDCP Encrypted DPK seed value used to encrypt the\n\nDevice Private Keys. The required software configuration sequence is documented in the\n\nCores HDMI Transmitter User Guide in the \"Programming\" chapter, Section 3.2.4,\n\n\"Configure HDCP.\"\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_seed0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcpreg_seed0`]
module"]
#[doc(alias = "HDCPREG_SEED0")]
pub type HdcpregSeed0 = crate::Reg<hdcpreg_seed0::HdcpregSeed0Spec>;
#[doc = "HDCP Encrypted DPK Seed Register 0\n\nThis register contains a byte of the HDCP Encrypted DPK seed value used to encrypt the\n\nDevice Private Keys. The required software configuration sequence is documented in the\n\nCores HDMI Transmitter User Guide in the \"Programming\" chapter, Section 3.2.4,\n\n\"Configure HDCP.\""]
pub mod hdcpreg_seed0;
#[doc = "HDCPREG_SEED1 (w) register accessor: HDCP Encrypted DPK Seed Register 1\n\nThis register contains a byte of the HDCP Encrypted DPK seed value used to encrypt the\n\nDevice Private Keys. The required software configuration sequence is documented in the\n\nCores HDMI Transmitter User Guide in the \"Programming\" chapter, Section 3.2.4,\n\n\"Configure HDCP.\"\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_seed1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcpreg_seed1`]
module"]
#[doc(alias = "HDCPREG_SEED1")]
pub type HdcpregSeed1 = crate::Reg<hdcpreg_seed1::HdcpregSeed1Spec>;
#[doc = "HDCP Encrypted DPK Seed Register 1\n\nThis register contains a byte of the HDCP Encrypted DPK seed value used to encrypt the\n\nDevice Private Keys. The required software configuration sequence is documented in the\n\nCores HDMI Transmitter User Guide in the \"Programming\" chapter, Section 3.2.4,\n\n\"Configure HDCP.\""]
pub mod hdcpreg_seed1;
#[doc = "HDCPREG_DPK0 (w) register accessor: HDCP Encrypted DPK Data Register 0\n\nThis register contains an HDCP DPK byte. The required software configuration sequence is\n\ndocumented in the Cores HDMI Transmitter User Guide in the \"Programming\" chapter,\n\nSection 3.2.4, \"Configure HDCP.\"\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_dpk0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcpreg_dpk0`]
module"]
#[doc(alias = "HDCPREG_DPK0")]
pub type HdcpregDpk0 = crate::Reg<hdcpreg_dpk0::HdcpregDpk0Spec>;
#[doc = "HDCP Encrypted DPK Data Register 0\n\nThis register contains an HDCP DPK byte. The required software configuration sequence is\n\ndocumented in the Cores HDMI Transmitter User Guide in the \"Programming\" chapter,\n\nSection 3.2.4, \"Configure HDCP.\""]
pub mod hdcpreg_dpk0;
#[doc = "HDCPREG_DPK1 (w) register accessor: HDCP Encrypted DPK Data Register 1\n\nThis register contains an HDCP DPK byte. The required software configuration sequence is\n\ndocumented in the Cores HDMI Transmitter User Guide in the \"Programming\" chapter,\n\nSection 3.2.4, \"Configure HDCP.\"\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_dpk1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcpreg_dpk1`]
module"]
#[doc(alias = "HDCPREG_DPK1")]
pub type HdcpregDpk1 = crate::Reg<hdcpreg_dpk1::HdcpregDpk1Spec>;
#[doc = "HDCP Encrypted DPK Data Register 1\n\nThis register contains an HDCP DPK byte. The required software configuration sequence is\n\ndocumented in the Cores HDMI Transmitter User Guide in the \"Programming\" chapter,\n\nSection 3.2.4, \"Configure HDCP.\""]
pub mod hdcpreg_dpk1;
#[doc = "HDCPREG_DPK2 (w) register accessor: HDCP Encrypted DPK Data Register 2\n\nThis register contains an HDCP DPK byte. The required software configuration sequence is\n\ndocumented in the Cores HDMI Transmitter User Guide in the \"Programming\" chapter,\n\nSection 3.2.4, \"Configure HDCP.\"\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_dpk2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcpreg_dpk2`]
module"]
#[doc(alias = "HDCPREG_DPK2")]
pub type HdcpregDpk2 = crate::Reg<hdcpreg_dpk2::HdcpregDpk2Spec>;
#[doc = "HDCP Encrypted DPK Data Register 2\n\nThis register contains an HDCP DPK byte. The required software configuration sequence is\n\ndocumented in the Cores HDMI Transmitter User Guide in the \"Programming\" chapter,\n\nSection 3.2.4, \"Configure HDCP.\""]
pub mod hdcpreg_dpk2;
#[doc = "HDCPREG_DPK3 (w) register accessor: HDCP Encrypted DPK Data Register 3\n\nThis register contains an HDCP DPK byte. The required software configuration sequence is\n\ndocumented in the Cores HDMI Transmitter User Guide in the \"Programming\" chapter,\n\nSection 3.2.4, \"Configure HDCP.\"\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_dpk3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcpreg_dpk3`]
module"]
#[doc(alias = "HDCPREG_DPK3")]
pub type HdcpregDpk3 = crate::Reg<hdcpreg_dpk3::HdcpregDpk3Spec>;
#[doc = "HDCP Encrypted DPK Data Register 3\n\nThis register contains an HDCP DPK byte. The required software configuration sequence is\n\ndocumented in the Cores HDMI Transmitter User Guide in the \"Programming\" chapter,\n\nSection 3.2.4, \"Configure HDCP.\""]
pub mod hdcpreg_dpk3;
#[doc = "HDCPREG_DPK4 (w) register accessor: HDCP Encrypted DPK Data Register 4\n\nThis register contains an HDCP DPK byte.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_dpk4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcpreg_dpk4`]
module"]
#[doc(alias = "HDCPREG_DPK4")]
pub type HdcpregDpk4 = crate::Reg<hdcpreg_dpk4::HdcpregDpk4Spec>;
#[doc = "HDCP Encrypted DPK Data Register 4\n\nThis register contains an HDCP DPK byte."]
pub mod hdcpreg_dpk4;
#[doc = "HDCPREG_DPK5 (w) register accessor: HDCP Encrypted DPK Data Register 5\n\nThis register contains an HDCP DPK byte. The required software configuration sequence is\n\ndocumented in the Cores HDMI Transmitter User Guide in the \"Programming\" chapter,\n\nSection 3.2.4, \"Configure HDCP.\"\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_dpk5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcpreg_dpk5`]
module"]
#[doc(alias = "HDCPREG_DPK5")]
pub type HdcpregDpk5 = crate::Reg<hdcpreg_dpk5::HdcpregDpk5Spec>;
#[doc = "HDCP Encrypted DPK Data Register 5\n\nThis register contains an HDCP DPK byte. The required software configuration sequence is\n\ndocumented in the Cores HDMI Transmitter User Guide in the \"Programming\" chapter,\n\nSection 3.2.4, \"Configure HDCP.\""]
pub mod hdcpreg_dpk5;
#[doc = "HDCPREG_DPK6 (w) register accessor: HDCP Encrypted DPK Data Register 6\n\nThis register contains an HDCP DPK byte. The required software configuration sequence is\n\ndocumented in the Cores HDMI Transmitter User Guide in the \"Programming\" chapter,\n\nSection 3.2.4, \"Configure HDCP.\"\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_dpk6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcpreg_dpk6`]
module"]
#[doc(alias = "HDCPREG_DPK6")]
pub type HdcpregDpk6 = crate::Reg<hdcpreg_dpk6::HdcpregDpk6Spec>;
#[doc = "HDCP Encrypted DPK Data Register 6\n\nThis register contains an HDCP DPK byte. The required software configuration sequence is\n\ndocumented in the Cores HDMI Transmitter User Guide in the \"Programming\" chapter,\n\nSection 3.2.4, \"Configure HDCP.\""]
pub mod hdcpreg_dpk6;
#[doc = "HDCP22REG_ID (r) register accessor: HDCP 2.2 Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp22reg_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcp22reg_id`]
module"]
#[doc(alias = "HDCP22REG_ID")]
pub type Hdcp22regId = crate::Reg<hdcp22reg_id::Hdcp22regIdSpec>;
#[doc = "HDCP 2.2 Identification Register"]
pub mod hdcp22reg_id;
#[doc = "HDCP22REG_CTRL (rw) register accessor: HDCP 2.2 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp22reg_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcp22reg_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcp22reg_ctrl`]
module"]
#[doc(alias = "HDCP22REG_CTRL")]
pub type Hdcp22regCtrl = crate::Reg<hdcp22reg_ctrl::Hdcp22regCtrlSpec>;
#[doc = "HDCP 2.2 Control Register"]
pub mod hdcp22reg_ctrl;
#[doc = "HDCP22REG_CTRL1 (rw) register accessor: HDCP 2.2 Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp22reg_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcp22reg_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcp22reg_ctrl1`]
module"]
#[doc(alias = "HDCP22REG_CTRL1")]
pub type Hdcp22regCtrl1 = crate::Reg<hdcp22reg_ctrl1::Hdcp22regCtrl1Spec>;
#[doc = "HDCP 2.2 Control Register 1"]
pub mod hdcp22reg_ctrl1;
#[doc = "HDCP22REG_STS (r) register accessor: HDCP 2.2 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp22reg_sts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcp22reg_sts`]
module"]
#[doc(alias = "HDCP22REG_STS")]
pub type Hdcp22regSts = crate::Reg<hdcp22reg_sts::Hdcp22regStsSpec>;
#[doc = "HDCP 2.2 Status Register"]
pub mod hdcp22reg_sts;
#[doc = "HDCP22REG_MASK (rw) register accessor: HDCP 2.2 Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp22reg_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcp22reg_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcp22reg_mask`]
module"]
#[doc(alias = "HDCP22REG_MASK")]
pub type Hdcp22regMask = crate::Reg<hdcp22reg_mask::Hdcp22regMaskSpec>;
#[doc = "HDCP 2.2 Interrupt Mask Register"]
pub mod hdcp22reg_mask;
#[doc = "HDCP22REG_STAT (rw) register accessor: HDCP 2.2 interrupt Sticky Bit Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp22reg_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcp22reg_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcp22reg_stat`]
module"]
#[doc(alias = "HDCP22REG_STAT")]
pub type Hdcp22regStat = crate::Reg<hdcp22reg_stat::Hdcp22regStatSpec>;
#[doc = "HDCP 2.2 interrupt Sticky Bit Status Register"]
pub mod hdcp22reg_stat;
#[doc = "HDCP22REG_MUTE (rw) register accessor: HDCP 2.2 Interrupt Mute Vector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp22reg_mute::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcp22reg_mute::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcp22reg_mute`]
module"]
#[doc(alias = "HDCP22REG_MUTE")]
pub type Hdcp22regMute = crate::Reg<hdcp22reg_mute::Hdcp22regMuteSpec>;
#[doc = "HDCP 2.2 Interrupt Mute Vector"]
pub mod hdcp22reg_mute;
#[doc = "CEC_CTRL (rw) register accessor: CEC Control Register\n\nThis register handles the main control of the CEC initiator.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cec_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cec_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cec_ctrl`]
module"]
#[doc(alias = "CEC_CTRL")]
pub type CecCtrl = crate::Reg<cec_ctrl::CecCtrlSpec>;
#[doc = "CEC Control Register\n\nThis register handles the main control of the CEC initiator."]
pub mod cec_ctrl;
#[doc = "CEC_MASK (rw) register accessor: CEC Interrupt Mask Register\n\nThis read/write register masks/unmasks the interrupt events. When the bit is set to 1\n\n(masked), the corresponding event does not trigger an interrupt signal at the system\n\ninterface. When the bit is reset to 0, the interrupt event is unmasked.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cec_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cec_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cec_mask`]
module"]
#[doc(alias = "CEC_MASK")]
pub type CecMask = crate::Reg<cec_mask::CecMaskSpec>;
#[doc = "CEC Interrupt Mask Register\n\nThis read/write register masks/unmasks the interrupt events. When the bit is set to 1\n\n(masked), the corresponding event does not trigger an interrupt signal at the system\n\ninterface. When the bit is reset to 0, the interrupt event is unmasked."]
pub mod cec_mask;
#[doc = "CEC_ADDR_L (rw) register accessor: CEC Logical Address Register Low\n\nThis register indicates the logical address(es) allocated to the CEC device.\n\nThis register is written by software when the logical allocation is finished. Bit value 1 means\n\nthe corresponding logical address is allocated to this device. Bit value 0 means the\n\ncorresponding logical address is not allocated to this device.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cec_addr_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cec_addr_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cec_addr_l`]
module"]
#[doc(alias = "CEC_ADDR_L")]
pub type CecAddrL = crate::Reg<cec_addr_l::CecAddrLSpec>;
#[doc = "CEC Logical Address Register Low\n\nThis register indicates the logical address(es) allocated to the CEC device.\n\nThis register is written by software when the logical allocation is finished. Bit value 1 means\n\nthe corresponding logical address is allocated to this device. Bit value 0 means the\n\ncorresponding logical address is not allocated to this device."]
pub mod cec_addr_l;
#[doc = "CEC_ADDR_H (rw) register accessor: CEC Logical Address Register High\n\nThis register indicates the logical address(es) allocated to the CEC device.\n\nThis register is written by software when the logical allocation is finished. Bit value 1 means\n\nthe corresponding logical address is allocated to this device. Bit value 0 means the\n\ncorresponding logical address is not allocated to this device.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cec_addr_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cec_addr_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cec_addr_h`]
module"]
#[doc(alias = "CEC_ADDR_H")]
pub type CecAddrH = crate::Reg<cec_addr_h::CecAddrHSpec>;
#[doc = "CEC Logical Address Register High\n\nThis register indicates the logical address(es) allocated to the CEC device.\n\nThis register is written by software when the logical allocation is finished. Bit value 1 means\n\nthe corresponding logical address is allocated to this device. Bit value 0 means the\n\ncorresponding logical address is not allocated to this device."]
pub mod cec_addr_h;
#[doc = "CEC_TX_CNT (rw) register accessor: CEC TX Frame Size Register\n\nThis register indicates the size of the frame in bytes (including header and data blocks),\n\nwhich are available in the transmitter data buffer.\n\nNote: When the value is zero, the CEC controller ignores the send command triggered by\n\nsoftware. When the transmission is done (no matter success or not), the current value is\n\nheld until it is overwritten by software.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cec_tx_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cec_tx_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cec_tx_cnt`]
module"]
#[doc(alias = "CEC_TX_CNT")]
pub type CecTxCnt = crate::Reg<cec_tx_cnt::CecTxCntSpec>;
#[doc = "CEC TX Frame Size Register\n\nThis register indicates the size of the frame in bytes (including header and data blocks),\n\nwhich are available in the transmitter data buffer.\n\nNote: When the value is zero, the CEC controller ignores the send command triggered by\n\nsoftware. When the transmission is done (no matter success or not), the current value is\n\nheld until it is overwritten by software."]
pub mod cec_tx_cnt;
#[doc = "CEC_RX_CNT (r) register accessor: CEC RX Frame Size Register\n\nThis register indicates the size of the frame in bytes (including header and data blocks),\n\nwhich are available in the receiver data buffer.\n\nNote: Only after the whole receiving process is finished successfully, the counter is\n\nrefreshed to the value which indicates the total number of data bytes in the Receiver Data\n\nRegister.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cec_rx_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cec_rx_cnt`]
module"]
#[doc(alias = "CEC_RX_CNT")]
pub type CecRxCnt = crate::Reg<cec_rx_cnt::CecRxCntSpec>;
#[doc = "CEC RX Frame Size Register\n\nThis register indicates the size of the frame in bytes (including header and data blocks),\n\nwhich are available in the receiver data buffer.\n\nNote: Only after the whole receiving process is finished successfully, the counter is\n\nrefreshed to the value which indicates the total number of data bytes in the Receiver Data\n\nRegister."]
pub mod cec_rx_cnt;
#[doc = "CEC_TX_DATA (rw) register accessor: CEC TX Data Register Array Address offset: i = 0 to 15\n\nThese registers (8 bits each) are the buffers used for storing the data waiting for\n\ntransmission (including header and data blocks).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cec_tx_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cec_tx_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cec_tx_data`]
module"]
#[doc(alias = "CEC_TX_DATA")]
pub type CecTxData = crate::Reg<cec_tx_data::CecTxDataSpec>;
#[doc = "CEC TX Data Register Array Address offset: i = 0 to 15\n\nThese registers (8 bits each) are the buffers used for storing the data waiting for\n\ntransmission (including header and data blocks)."]
pub mod cec_tx_data;
#[doc = "CEC_RX_DATA (r) register accessor: CEC RX Data Register Array Address offset: i =0 to 15\n\nThese registers (8 bit each) are the buffers used for storing the received data (including\n\nheader and data blocks).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cec_rx_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cec_rx_data`]
module"]
#[doc(alias = "CEC_RX_DATA")]
pub type CecRxData = crate::Reg<cec_rx_data::CecRxDataSpec>;
#[doc = "CEC RX Data Register Array Address offset: i =0 to 15\n\nThese registers (8 bit each) are the buffers used for storing the received data (including\n\nheader and data blocks)."]
pub mod cec_rx_data;
#[doc = "CEC_LOCK (rw) register accessor: CEC Buffer Lock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cec_lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cec_lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cec_lock`]
module"]
#[doc(alias = "CEC_LOCK")]
pub type CecLock = crate::Reg<cec_lock::CecLockSpec>;
#[doc = "CEC Buffer Lock Register"]
pub mod cec_lock;
#[doc = "CEC_WAKEUPCTRL (rw) register accessor: CEC Wake-up Control Register\n\nAfter receiving a message in the CEC_RX_DATA1 (OPCODE) registers, the CEC engine\n\nverifies the message opcode\\[7:0\\]
against one of the previously defined values to generate\n\nthe wake-up status:\n\nWakeupstatus is 1 when:\n\nreceived opcode is 0x04 and opcode0x04en is 1 or\n\nreceived opcode is 0x0D and opcode0x0Den is 1 or\n\nreceived opcode is 0x41 and opcode0x41en is 1 or\n\nreceived opcode is 0x42 and opcode0x42en is 1 or\n\nreceived opcode is 0x44 and opcode0x44en is 1 or\n\nreceived opcode is 0x70 and opcode0x70en is 1 or\n\nreceived opcode is 0x82 and opcode0x82en is 1 or\n\nreceived opcode is 0x86 and opcode0x86en is 1\n\nWakeupstatus is 0 when none of the previous conditions are true.\n\nThis formula means that the wake-up status (on CEC_STAT\\[6\\]
register) is only '1' if the\n\nopcode\\[7:0\\]
received is equal to one of the defined values and the corresponding enable bit\n\nof that defined value is set to '1'.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cec_wakeupctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cec_wakeupctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cec_wakeupctrl`]
module"]
#[doc(alias = "CEC_WAKEUPCTRL")]
pub type CecWakeupctrl = crate::Reg<cec_wakeupctrl::CecWakeupctrlSpec>;
#[doc = "CEC Wake-up Control Register\n\nAfter receiving a message in the CEC_RX_DATA1 (OPCODE) registers, the CEC engine\n\nverifies the message opcode\\[7:0\\]
against one of the previously defined values to generate\n\nthe wake-up status:\n\nWakeupstatus is 1 when:\n\nreceived opcode is 0x04 and opcode0x04en is 1 or\n\nreceived opcode is 0x0D and opcode0x0Den is 1 or\n\nreceived opcode is 0x41 and opcode0x41en is 1 or\n\nreceived opcode is 0x42 and opcode0x42en is 1 or\n\nreceived opcode is 0x44 and opcode0x44en is 1 or\n\nreceived opcode is 0x70 and opcode0x70en is 1 or\n\nreceived opcode is 0x82 and opcode0x82en is 1 or\n\nreceived opcode is 0x86 and opcode0x86en is 1\n\nWakeupstatus is 0 when none of the previous conditions are true.\n\nThis formula means that the wake-up status (on CEC_STAT\\[6\\]
register) is only '1' if the\n\nopcode\\[7:0\\]
received is equal to one of the defined values and the corresponding enable bit\n\nof that defined value is set to '1'."]
pub mod cec_wakeupctrl;
#[doc = "I2CM_SLAVE (rw) register accessor: I2C DDC Slave address Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_slave::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cm_slave::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm_slave`]
module"]
#[doc(alias = "I2CM_SLAVE")]
pub type I2cmSlave = crate::Reg<i2cm_slave::I2cmSlaveSpec>;
#[doc = "I2C DDC Slave address Configuration Register"]
pub mod i2cm_slave;
#[doc = "I2CM_ADDRESS (rw) register accessor: I2C DDC Address Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_address::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cm_address::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm_address`]
module"]
#[doc(alias = "I2CM_ADDRESS")]
pub type I2cmAddress = crate::Reg<i2cm_address::I2cmAddressSpec>;
#[doc = "I2C DDC Address Configuration Register"]
pub mod i2cm_address;
#[doc = "I2CM_DATAO (rw) register accessor: I2C DDC Data Write Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_datao::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cm_datao::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm_datao`]
module"]
#[doc(alias = "I2CM_DATAO")]
pub type I2cmDatao = crate::Reg<i2cm_datao::I2cmDataoSpec>;
#[doc = "I2C DDC Data Write Register"]
pub mod i2cm_datao;
#[doc = "I2CM_DATAI (r) register accessor: I2C DDC Data read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_datai::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm_datai`]
module"]
#[doc(alias = "I2CM_DATAI")]
pub type I2cmDatai = crate::Reg<i2cm_datai::I2cmDataiSpec>;
#[doc = "I2C DDC Data read Register"]
pub mod i2cm_datai;
#[doc = "I2CM_OPERATION (w) register accessor: I2C DDC RD/RD_EXT/WR Operation Register\n\nRead and write operation request. This register can only be written; reading this register\n\nalways results in 00h. Writing 1'b1 simultaneously to rd, rd_ext and wr requests is\n\nconsidered as a read (rd) request.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cm_operation::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm_operation`]
module"]
#[doc(alias = "I2CM_OPERATION")]
pub type I2cmOperation = crate::Reg<i2cm_operation::I2cmOperationSpec>;
#[doc = "I2C DDC RD/RD_EXT/WR Operation Register\n\nRead and write operation request. This register can only be written; reading this register\n\nalways results in 00h. Writing 1'b1 simultaneously to rd, rd_ext and wr requests is\n\nconsidered as a read (rd) request."]
pub mod i2cm_operation;
#[doc = "I2CM_INT (rw) register accessor: I2C DDC Done Interrupt Register This register configures the I2C master\n\ninterrupts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cm_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm_int`]
module"]
#[doc(alias = "I2CM_INT")]
pub type I2cmInt = crate::Reg<i2cm_int::I2cmIntSpec>;
#[doc = "I2C DDC Done Interrupt Register This register configures the I2C master\n\ninterrupts."]
pub mod i2cm_int;
#[doc = "I2CM_CTLINT (rw) register accessor: I2C DDC error Interrupt Register\n\nThis register configures the I2C master arbitration lost and not acknowledge error\n\ninterrupts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_ctlint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cm_ctlint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm_ctlint`]
module"]
#[doc(alias = "I2CM_CTLINT")]
pub type I2cmCtlint = crate::Reg<i2cm_ctlint::I2cmCtlintSpec>;
#[doc = "I2C DDC error Interrupt Register\n\nThis register configures the I2C master arbitration lost and not acknowledge error\n\ninterrupts."]
pub mod i2cm_ctlint;
#[doc = "I2CM_DIV (rw) register accessor: I2C DDC Speed Control Register\n\nThis register configures the division relation between master and scl clock.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cm_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm_div`]
module"]
#[doc(alias = "I2CM_DIV")]
pub type I2cmDiv = crate::Reg<i2cm_div::I2cmDivSpec>;
#[doc = "I2C DDC Speed Control Register\n\nThis register configures the division relation between master and scl clock."]
pub mod i2cm_div;
#[doc = "I2CM_SEGADDR (rw) register accessor: I2C DDC Segment Address Configuration Register\n\nThis register configures the segment address for extended R/W destination and is used for\n\nEDID reading operations, particularly for the Extended Data Read Operation for Enhanced\n\nDDC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_segaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cm_segaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm_segaddr`]
module"]
#[doc(alias = "I2CM_SEGADDR")]
pub type I2cmSegaddr = crate::Reg<i2cm_segaddr::I2cmSegaddrSpec>;
#[doc = "I2C DDC Segment Address Configuration Register\n\nThis register configures the segment address for extended R/W destination and is used for\n\nEDID reading operations, particularly for the Extended Data Read Operation for Enhanced\n\nDDC."]
pub mod i2cm_segaddr;
#[doc = "I2CM_SOFTRSTZ (rw) register accessor: I2C DDC Software Reset Control Register This register resets the I2C master.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_softrstz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cm_softrstz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm_softrstz`]
module"]
#[doc(alias = "I2CM_SOFTRSTZ")]
pub type I2cmSoftrstz = crate::Reg<i2cm_softrstz::I2cmSoftrstzSpec>;
#[doc = "I2C DDC Software Reset Control Register This register resets the I2C master."]
pub mod i2cm_softrstz;
#[doc = "I2CM_SEGPTR (rw) register accessor: I2C DDC Segment Pointer Register\n\nThis register configures the segment pointer for extended RD/WR request.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_segptr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cm_segptr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm_segptr`]
module"]
#[doc(alias = "I2CM_SEGPTR")]
pub type I2cmSegptr = crate::Reg<i2cm_segptr::I2cmSegptrSpec>;
#[doc = "I2C DDC Segment Pointer Register\n\nThis register configures the segment pointer for extended RD/WR request."]
pub mod i2cm_segptr;
#[doc = "I2CM_SS_SCL_HCNT_1_ADDR (rw) register accessor: I2C DDC Slow Speed SCL High Level Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_ss_scl_hcnt_1_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cm_ss_scl_hcnt_1_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm_ss_scl_hcnt_1_addr`]
module"]
#[doc(alias = "I2CM_SS_SCL_HCNT_1_ADDR")]
pub type I2cmSsSclHcnt1Addr = crate::Reg<i2cm_ss_scl_hcnt_1_addr::I2cmSsSclHcnt1AddrSpec>;
#[doc = "I2C DDC Slow Speed SCL High Level Control Register 1"]
pub mod i2cm_ss_scl_hcnt_1_addr;
#[doc = "I2CM_SS_SCL_HCNT_0_ADDR (rw) register accessor: I2C DDC Slow Speed SCL High Level Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_ss_scl_hcnt_0_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cm_ss_scl_hcnt_0_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm_ss_scl_hcnt_0_addr`]
module"]
#[doc(alias = "I2CM_SS_SCL_HCNT_0_ADDR")]
pub type I2cmSsSclHcnt0Addr = crate::Reg<i2cm_ss_scl_hcnt_0_addr::I2cmSsSclHcnt0AddrSpec>;
#[doc = "I2C DDC Slow Speed SCL High Level Control Register 0"]
pub mod i2cm_ss_scl_hcnt_0_addr;
#[doc = "I2CM_SS_SCL_LCNT_1_ADDR (rw) register accessor: I2C DDC Slow Speed SCL Low Level Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_ss_scl_lcnt_1_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cm_ss_scl_lcnt_1_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm_ss_scl_lcnt_1_addr`]
module"]
#[doc(alias = "I2CM_SS_SCL_LCNT_1_ADDR")]
pub type I2cmSsSclLcnt1Addr = crate::Reg<i2cm_ss_scl_lcnt_1_addr::I2cmSsSclLcnt1AddrSpec>;
#[doc = "I2C DDC Slow Speed SCL Low Level Control Register 1"]
pub mod i2cm_ss_scl_lcnt_1_addr;
#[doc = "I2CM_SS_SCL_LCNT_0_ADDR (rw) register accessor: I2C DDC Slow Speed SCL Low Level Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_ss_scl_lcnt_0_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cm_ss_scl_lcnt_0_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm_ss_scl_lcnt_0_addr`]
module"]
#[doc(alias = "I2CM_SS_SCL_LCNT_0_ADDR")]
pub type I2cmSsSclLcnt0Addr = crate::Reg<i2cm_ss_scl_lcnt_0_addr::I2cmSsSclLcnt0AddrSpec>;
#[doc = "I2C DDC Slow Speed SCL Low Level Control Register 0"]
pub mod i2cm_ss_scl_lcnt_0_addr;
#[doc = "I2CM_FS_SCL_HCNT_1_ADDR (rw) register accessor: I2C DDC Fast Speed SCL High Level Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_fs_scl_hcnt_1_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cm_fs_scl_hcnt_1_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm_fs_scl_hcnt_1_addr`]
module"]
#[doc(alias = "I2CM_FS_SCL_HCNT_1_ADDR")]
pub type I2cmFsSclHcnt1Addr = crate::Reg<i2cm_fs_scl_hcnt_1_addr::I2cmFsSclHcnt1AddrSpec>;
#[doc = "I2C DDC Fast Speed SCL High Level Control Register 1"]
pub mod i2cm_fs_scl_hcnt_1_addr;
#[doc = "I2CM_FS_SCL_HCNT_0_ADDR (rw) register accessor: I2C DDC Fast Speed SCL High Level Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_fs_scl_hcnt_0_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cm_fs_scl_hcnt_0_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm_fs_scl_hcnt_0_addr`]
module"]
#[doc(alias = "I2CM_FS_SCL_HCNT_0_ADDR")]
pub type I2cmFsSclHcnt0Addr = crate::Reg<i2cm_fs_scl_hcnt_0_addr::I2cmFsSclHcnt0AddrSpec>;
#[doc = "I2C DDC Fast Speed SCL High Level Control Register 0"]
pub mod i2cm_fs_scl_hcnt_0_addr;
#[doc = "I2CM_FS_SCL_LCNT_1_ADDR (rw) register accessor: I2C DDC Fast Speed SCL Low Level Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_fs_scl_lcnt_1_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cm_fs_scl_lcnt_1_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm_fs_scl_lcnt_1_addr`]
module"]
#[doc(alias = "I2CM_FS_SCL_LCNT_1_ADDR")]
pub type I2cmFsSclLcnt1Addr = crate::Reg<i2cm_fs_scl_lcnt_1_addr::I2cmFsSclLcnt1AddrSpec>;
#[doc = "I2C DDC Fast Speed SCL Low Level Control Register 1"]
pub mod i2cm_fs_scl_lcnt_1_addr;
#[doc = "I2CM_FS_SCL_LCNT_0_ADDR (rw) register accessor: I2C DDC Fast Speed SCL Low Level Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_fs_scl_lcnt_0_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cm_fs_scl_lcnt_0_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm_fs_scl_lcnt_0_addr`]
module"]
#[doc(alias = "I2CM_FS_SCL_LCNT_0_ADDR")]
pub type I2cmFsSclLcnt0Addr = crate::Reg<i2cm_fs_scl_lcnt_0_addr::I2cmFsSclLcnt0AddrSpec>;
#[doc = "I2C DDC Fast Speed SCL Low Level Control Register 0"]
pub mod i2cm_fs_scl_lcnt_0_addr;
#[doc = "I2CM_SDA_HOLD (rw) register accessor: I2C DDC SDA Hold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_sda_hold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cm_sda_hold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm_sda_hold`]
module"]
#[doc(alias = "I2CM_SDA_HOLD")]
pub type I2cmSdaHold = crate::Reg<i2cm_sda_hold::I2cmSdaHoldSpec>;
#[doc = "I2C DDC SDA Hold Register"]
pub mod i2cm_sda_hold;
#[doc = "I2CM_SCDC_READ_UPDATE (rw) register accessor: SCDC Control Register\n\nThis register configures the SCDC update status read through the I2C master interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_scdc_read_update::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cm_scdc_read_update::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm_scdc_read_update`]
module"]
#[doc(alias = "I2CM_SCDC_READ_UPDATE")]
pub type I2cmScdcReadUpdate = crate::Reg<i2cm_scdc_read_update::I2cmScdcReadUpdateSpec>;
#[doc = "SCDC Control Register\n\nThis register configures the SCDC update status read through the I2C master interface."]
pub mod i2cm_scdc_read_update;
#[doc = "I2CM_READ_BUFF0 (r) register accessor: I2C Master Sequential Read Buffer Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_read_buff0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm_read_buff0`]
module"]
#[doc(alias = "I2CM_READ_BUFF0")]
pub type I2cmReadBuff0 = crate::Reg<i2cm_read_buff0::I2cmReadBuff0Spec>;
#[doc = "I2C Master Sequential Read Buffer Register 0"]
pub mod i2cm_read_buff0;
#[doc = "I2CM_READ_BUFF1 (r) register accessor: I2C Master Sequential Read Buffer Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_read_buff1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm_read_buff1`]
module"]
#[doc(alias = "I2CM_READ_BUFF1")]
pub type I2cmReadBuff1 = crate::Reg<i2cm_read_buff1::I2cmReadBuff1Spec>;
#[doc = "I2C Master Sequential Read Buffer Register 1"]
pub mod i2cm_read_buff1;
#[doc = "I2CM_READ_BUFF2 (r) register accessor: I2C Master Sequential Read Buffer Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_read_buff2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm_read_buff2`]
module"]
#[doc(alias = "I2CM_READ_BUFF2")]
pub type I2cmReadBuff2 = crate::Reg<i2cm_read_buff2::I2cmReadBuff2Spec>;
#[doc = "I2C Master Sequential Read Buffer Register 2"]
pub mod i2cm_read_buff2;
#[doc = "I2CM_READ_BUFF3 (r) register accessor: I2C Master Sequential Read Buffer Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_read_buff3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm_read_buff3`]
module"]
#[doc(alias = "I2CM_READ_BUFF3")]
pub type I2cmReadBuff3 = crate::Reg<i2cm_read_buff3::I2cmReadBuff3Spec>;
#[doc = "I2C Master Sequential Read Buffer Register 3"]
pub mod i2cm_read_buff3;
#[doc = "I2CM_READ_BUFF4 (r) register accessor: I2C Master Sequential Read Buffer Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_read_buff4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm_read_buff4`]
module"]
#[doc(alias = "I2CM_READ_BUFF4")]
pub type I2cmReadBuff4 = crate::Reg<i2cm_read_buff4::I2cmReadBuff4Spec>;
#[doc = "I2C Master Sequential Read Buffer Register 4"]
pub mod i2cm_read_buff4;
#[doc = "I2CM_READ_BUFF5 (r) register accessor: I2C Master Sequential Read Buffer Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_read_buff5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm_read_buff5`]
module"]
#[doc(alias = "I2CM_READ_BUFF5")]
pub type I2cmReadBuff5 = crate::Reg<i2cm_read_buff5::I2cmReadBuff5Spec>;
#[doc = "I2C Master Sequential Read Buffer Register 5"]
pub mod i2cm_read_buff5;
#[doc = "I2CM_READ_BUFF6 (r) register accessor: I2C Master Sequential Read Buffer Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_read_buff6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm_read_buff6`]
module"]
#[doc(alias = "I2CM_READ_BUFF6")]
pub type I2cmReadBuff6 = crate::Reg<i2cm_read_buff6::I2cmReadBuff6Spec>;
#[doc = "I2C Master Sequential Read Buffer Register 6"]
pub mod i2cm_read_buff6;
#[doc = "I2CM_READ_BUFF7 (r) register accessor: I2C Master Sequential Read Buffer Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_read_buff7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm_read_buff7`]
module"]
#[doc(alias = "I2CM_READ_BUFF7")]
pub type I2cmReadBuff7 = crate::Reg<i2cm_read_buff7::I2cmReadBuff7Spec>;
#[doc = "I2C Master Sequential Read Buffer Register 7"]
pub mod i2cm_read_buff7;
#[doc = "I2CM_SCDC_UPDATE0 (r) register accessor: I2C SCDC Read Update Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_scdc_update0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm_scdc_update0`]
module"]
#[doc(alias = "I2CM_SCDC_UPDATE0")]
pub type I2cmScdcUpdate0 = crate::Reg<i2cm_scdc_update0::I2cmScdcUpdate0Spec>;
#[doc = "I2C SCDC Read Update Register 0"]
pub mod i2cm_scdc_update0;
#[doc = "I2CM_SCDC_UPDATE1 (r) register accessor: I2C SCDC Read Update Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_scdc_update1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm_scdc_update1`]
module"]
#[doc(alias = "I2CM_SCDC_UPDATE1")]
pub type I2cmScdcUpdate1 = crate::Reg<i2cm_scdc_update1::I2cmScdcUpdate1Spec>;
#[doc = "I2C SCDC Read Update Register 1"]
pub mod i2cm_scdc_update1;
