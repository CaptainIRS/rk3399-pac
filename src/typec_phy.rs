#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0003_0000],
    pma_lane_cfg: PmaLaneCfg,
    _reserved1: [u8; 0x02],
    pipe_cmn_ctrl1: PipeCmnCtrl1,
    _reserved2: [u8; 0x02],
    pipe_cmn_ctrl2: PipeCmnCtrl2,
    _reserved3: [u8; 0x02],
    pipe_com_lock_cfg1: PipeComLockCfg1,
    _reserved4: [u8; 0x02],
    pipe_com_lock_cfg2: PipeComLockCfg2,
    _reserved5: [u8; 0x02],
    pipe_rcv_det_inh: PipeRcvDetInh,
    _reserved6: [u8; 0x0a],
    dp_mode_ctl: DpModeCtl,
    _reserved7: [u8; 0x02],
    dp_clk_ctl: DpClkCtl,
    _reserved8: [u8; 0x16],
    sts: Sts,
    _reserved9: [u8; 0x0fc2],
    usb_ber_cnt: UsbBerCnt,
    _reserved10: [u8; 0x1e],
    dp_tx_ctl_lane0: DpTxCtlLane0,
    _reserved11: [u8; 0xfe],
    dp_tx_ctl_lane1: DpTxCtlLane1,
    _reserved12: [u8; 0xfe],
    dp_tx_ctl_lane2: DpTxCtlLane2,
    _reserved13: [u8; 0xfe],
    dp_tx_ctl_lane3: DpTxCtlLane3,
    _reserved14: [u8; 0x0cde],
    pma_cmn_ctrl1: PmaCmnCtrl1,
    _reserved15: [u8; 0x42],
    pma_iso_pll_ctrl0: PmaIsoPllCtrl0,
    _reserved16: [u8; 0x02],
    pma_iso_pll_ctrl1: PmaIsoPllCtrl1,
    _reserved17: [u8; 0x32],
    isolation_ctrl: IsolationCtrl,
    _reserved18: [u8; 0x0fc2],
    pma_iso_xcvr_ctrl_lane0: PmaIsoXcvrCtrlLane0,
    _reserved19: [u8; 0x02],
    pma_iso_tx_cfg_lane0: PmaIsoTxCfgLane0,
    _reserved20: [u8; 0x02],
    pma_iso_link_mode_lane0: PmaIsoLinkModeLane0,
    _reserved21: [u8; 0x02],
    pma_iso_pwrst_ctrl_lane0: PmaIsoPwrstCtrlLane0,
    _reserved22: [u8; 0x02],
    pma_iso_tx_data_lo_lane0: PmaIsoTxDataLoLane0,
    _reserved23: [u8; 0x02],
    pma_iso_tx_data_hi_lane0: PmaIsoTxDataHiLane0,
    _reserved24: [u8; 0x02],
    pma_iso_rx_data_lo_lane0: PmaIsoRxDataLoLane0,
    _reserved25: [u8; 0x02],
    pma_iso_rx_data_hi_lane0: PmaIsoRxDataHiLane0,
    _reserved26: [u8; 0xe2],
    pma_iso_xcvr_ctrl_lane1: PmaIsoXcvrCtrlLane1,
    _reserved27: [u8; 0x02],
    pma_iso_tx_cfg_lane1: PmaIsoTxCfgLane1,
    _reserved28: [u8; 0x02],
    pma_iso_link_mode_lane1: PmaIsoLinkModeLane1,
    _reserved29: [u8; 0x02],
    pma_iso_pwrst_ctrl_lane1: PmaIsoPwrstCtrlLane1,
    _reserved30: [u8; 0x02],
    pma_iso_tx_data_lo_lane1: PmaIsoTxDataLoLane1,
    _reserved31: [u8; 0x02],
    pma_iso_tx_data_hi_lane1: PmaIsoTxDataHiLane1,
    _reserved32: [u8; 0x02],
    pma_iso_rx_data_lo_lane1: PmaIsoRxDataLoLane1,
    _reserved33: [u8; 0x02],
    pma_iso_rx_data_hi_lane1: PmaIsoRxDataHiLane1,
    _reserved34: [u8; 0xe2],
    pma_iso_xcvr_ctrl_lane2: PmaIsoXcvrCtrlLane2,
    _reserved35: [u8; 0x02],
    pma_iso_tx_cfg_lane2: PmaIsoTxCfgLane2,
    _reserved36: [u8; 0x02],
    pma_iso_link_mode_lane2: PmaIsoLinkModeLane2,
    _reserved37: [u8; 0x02],
    pma_iso_pwrst_ctrl_lane2: PmaIsoPwrstCtrlLane2,
    _reserved38: [u8; 0x02],
    pma_iso_tx_data_lo_lane2: PmaIsoTxDataLoLane2,
    _reserved39: [u8; 0x02],
    pma_iso_tx_data_hi_lane2: PmaIsoTxDataHiLane2,
    _reserved40: [u8; 0x02],
    pma_iso_rx_data_lo_lane2: PmaIsoRxDataLoLane2,
    _reserved41: [u8; 0x02],
    pma_iso_rx_data_hi_lane2: PmaIsoRxDataHiLane2,
    _reserved42: [u8; 0xe2],
    pma_iso_xcvr_ctrl_lane3: PmaIsoXcvrCtrlLane3,
    _reserved43: [u8; 0x02],
    pma_iso_tx_cfg_lane3: PmaIsoTxCfgLane3,
    _reserved44: [u8; 0x02],
    pma_iso_link_mode_lane3: PmaIsoLinkModeLane3,
    _reserved45: [u8; 0x02],
    pma_iso_pwrst_ctrl_lane3: PmaIsoPwrstCtrlLane3,
    _reserved46: [u8; 0x02],
    pma_iso_tx_data_lo_lane3: PmaIsoTxDataLoLane3,
    _reserved47: [u8; 0x02],
    pma_iso_tx_data_hi_lane3: PmaIsoTxDataHiLane3,
    _reserved48: [u8; 0x02],
    pma_iso_rx_data_lo_lane3: PmaIsoRxDataLoLane3,
    _reserved49: [u8; 0x02],
    pma_iso_rx_data_hi_lane3: PmaIsoRxDataHiLane3,
}
impl RegisterBlock {
    #[doc = "0x30000 - PMA lane configuration register"]
    #[inline(always)]
    pub const fn pma_lane_cfg(&self) -> &PmaLaneCfg {
        &self.pma_lane_cfg
    }
    #[doc = "0x30004 - PIPE common control1 register"]
    #[inline(always)]
    pub const fn pipe_cmn_ctrl1(&self) -> &PipeCmnCtrl1 {
        &self.pipe_cmn_ctrl1
    }
    #[doc = "0x30008 - PIPE common control2 register"]
    #[inline(always)]
    pub const fn pipe_cmn_ctrl2(&self) -> &PipeCmnCtrl2 {
        &self.pipe_cmn_ctrl2
    }
    #[doc = "0x3000c - PIPE comma lock configuration1 register"]
    #[inline(always)]
    pub const fn pipe_com_lock_cfg1(&self) -> &PipeComLockCfg1 {
        &self.pipe_com_lock_cfg1
    }
    #[doc = "0x30010 - PIPE comma lock configuration2 register"]
    #[inline(always)]
    pub const fn pipe_com_lock_cfg2(&self) -> &PipeComLockCfg2 {
        &self.pipe_com_lock_cfg2
    }
    #[doc = "0x30014 - PIPE receiver detect inhibit register"]
    #[inline(always)]
    pub const fn pipe_rcv_det_inh(&self) -> &PipeRcvDetInh {
        &self.pipe_rcv_det_inh
    }
    #[doc = "0x30020 - DP Mode Control register"]
    #[inline(always)]
    pub const fn dp_mode_ctl(&self) -> &DpModeCtl {
        &self.dp_mode_ctl
    }
    #[doc = "0x30024 - DP Clock Control register"]
    #[inline(always)]
    pub const fn dp_clk_ctl(&self) -> &DpClkCtl {
        &self.dp_clk_ctl
    }
    #[doc = "0x3003c - PHY status register"]
    #[inline(always)]
    pub const fn sts(&self) -> &Sts {
        &self.sts
    }
    #[doc = "0x31000 - USB loopback slave BER count isolation register"]
    #[inline(always)]
    pub const fn usb_ber_cnt(&self) -> &UsbBerCnt {
        &self.usb_ber_cnt
    }
    #[doc = "0x31020 - DP Lane Configuration register"]
    #[inline(always)]
    pub const fn dp_tx_ctl_lane0(&self) -> &DpTxCtlLane0 {
        &self.dp_tx_ctl_lane0
    }
    #[doc = "0x31120 - DP Lane Configuration register"]
    #[inline(always)]
    pub const fn dp_tx_ctl_lane1(&self) -> &DpTxCtlLane1 {
        &self.dp_tx_ctl_lane1
    }
    #[doc = "0x31220 - DP Lane Configuration register"]
    #[inline(always)]
    pub const fn dp_tx_ctl_lane2(&self) -> &DpTxCtlLane2 {
        &self.dp_tx_ctl_lane2
    }
    #[doc = "0x31320 - DP Lane Configuration register"]
    #[inline(always)]
    pub const fn dp_tx_ctl_lane3(&self) -> &DpTxCtlLane3 {
        &self.dp_tx_ctl_lane3
    }
    #[doc = "0x32000 - PMA common control1 register"]
    #[inline(always)]
    pub const fn pma_cmn_ctrl1(&self) -> &PmaCmnCtrl1 {
        &self.pma_cmn_ctrl1
    }
    #[doc = "0x32044 - PMA PLL control0 isolation register"]
    #[inline(always)]
    pub const fn pma_iso_pll_ctrl0(&self) -> &PmaIsoPllCtrl0 {
        &self.pma_iso_pll_ctrl0
    }
    #[doc = "0x32048 - PMA PLL control1 isolation register"]
    #[inline(always)]
    pub const fn pma_iso_pll_ctrl1(&self) -> &PmaIsoPllCtrl1 {
        &self.pma_iso_pll_ctrl1
    }
    #[doc = "0x3207c - Isolation control register"]
    #[inline(always)]
    pub const fn isolation_ctrl(&self) -> &IsolationCtrl {
        &self.isolation_ctrl
    }
    #[doc = "0x33040 - PMA Isolation Tansceiver control register"]
    #[inline(always)]
    pub const fn pma_iso_xcvr_ctrl_lane0(&self) -> &PmaIsoXcvrCtrlLane0 {
        &self.pma_iso_xcvr_ctrl_lane0
    }
    #[doc = "0x33044 - PMA TX configuration register"]
    #[inline(always)]
    pub const fn pma_iso_tx_cfg_lane0(&self) -> &PmaIsoTxCfgLane0 {
        &self.pma_iso_tx_cfg_lane0
    }
    #[doc = "0x33048 - PMA Isolation mode control register"]
    #[inline(always)]
    pub const fn pma_iso_link_mode_lane0(&self) -> &PmaIsoLinkModeLane0 {
        &self.pma_iso_link_mode_lane0
    }
    #[doc = "0x3304c - PMA Isolation power state control register"]
    #[inline(always)]
    pub const fn pma_iso_pwrst_ctrl_lane0(&self) -> &PmaIsoPwrstCtrlLane0 {
        &self.pma_iso_pwrst_ctrl_lane0
    }
    #[doc = "0x33050 - PMA transmit low data isolation register"]
    #[inline(always)]
    pub const fn pma_iso_tx_data_lo_lane0(&self) -> &PmaIsoTxDataLoLane0 {
        &self.pma_iso_tx_data_lo_lane0
    }
    #[doc = "0x33054 - PMA transmit high data isolation register"]
    #[inline(always)]
    pub const fn pma_iso_tx_data_hi_lane0(&self) -> &PmaIsoTxDataHiLane0 {
        &self.pma_iso_tx_data_hi_lane0
    }
    #[doc = "0x33058 - PMA receive low data isolation register"]
    #[inline(always)]
    pub const fn pma_iso_rx_data_lo_lane0(&self) -> &PmaIsoRxDataLoLane0 {
        &self.pma_iso_rx_data_lo_lane0
    }
    #[doc = "0x3305c - PMA receive high data isolation register"]
    #[inline(always)]
    pub const fn pma_iso_rx_data_hi_lane0(&self) -> &PmaIsoRxDataHiLane0 {
        &self.pma_iso_rx_data_hi_lane0
    }
    #[doc = "0x33140 - PMA Isolation Tansceiver control register"]
    #[inline(always)]
    pub const fn pma_iso_xcvr_ctrl_lane1(&self) -> &PmaIsoXcvrCtrlLane1 {
        &self.pma_iso_xcvr_ctrl_lane1
    }
    #[doc = "0x33144 - PMA TX configuration register"]
    #[inline(always)]
    pub const fn pma_iso_tx_cfg_lane1(&self) -> &PmaIsoTxCfgLane1 {
        &self.pma_iso_tx_cfg_lane1
    }
    #[doc = "0x33148 - PMA Isolation mode control register"]
    #[inline(always)]
    pub const fn pma_iso_link_mode_lane1(&self) -> &PmaIsoLinkModeLane1 {
        &self.pma_iso_link_mode_lane1
    }
    #[doc = "0x3314c - PMA Isolation power state control register"]
    #[inline(always)]
    pub const fn pma_iso_pwrst_ctrl_lane1(&self) -> &PmaIsoPwrstCtrlLane1 {
        &self.pma_iso_pwrst_ctrl_lane1
    }
    #[doc = "0x33150 - PMA transmit low data isolation register"]
    #[inline(always)]
    pub const fn pma_iso_tx_data_lo_lane1(&self) -> &PmaIsoTxDataLoLane1 {
        &self.pma_iso_tx_data_lo_lane1
    }
    #[doc = "0x33154 - PMA transmit high data isolation register"]
    #[inline(always)]
    pub const fn pma_iso_tx_data_hi_lane1(&self) -> &PmaIsoTxDataHiLane1 {
        &self.pma_iso_tx_data_hi_lane1
    }
    #[doc = "0x33158 - PMA receive low data isolation register"]
    #[inline(always)]
    pub const fn pma_iso_rx_data_lo_lane1(&self) -> &PmaIsoRxDataLoLane1 {
        &self.pma_iso_rx_data_lo_lane1
    }
    #[doc = "0x3315c - PMA receive high data isolation register"]
    #[inline(always)]
    pub const fn pma_iso_rx_data_hi_lane1(&self) -> &PmaIsoRxDataHiLane1 {
        &self.pma_iso_rx_data_hi_lane1
    }
    #[doc = "0x33240 - PMA Isolation Tansceiver control register"]
    #[inline(always)]
    pub const fn pma_iso_xcvr_ctrl_lane2(&self) -> &PmaIsoXcvrCtrlLane2 {
        &self.pma_iso_xcvr_ctrl_lane2
    }
    #[doc = "0x33244 - PMA TX configuration register"]
    #[inline(always)]
    pub const fn pma_iso_tx_cfg_lane2(&self) -> &PmaIsoTxCfgLane2 {
        &self.pma_iso_tx_cfg_lane2
    }
    #[doc = "0x33248 - PMA Isolation mode control register"]
    #[inline(always)]
    pub const fn pma_iso_link_mode_lane2(&self) -> &PmaIsoLinkModeLane2 {
        &self.pma_iso_link_mode_lane2
    }
    #[doc = "0x3324c - PMA Isolation power state control register"]
    #[inline(always)]
    pub const fn pma_iso_pwrst_ctrl_lane2(&self) -> &PmaIsoPwrstCtrlLane2 {
        &self.pma_iso_pwrst_ctrl_lane2
    }
    #[doc = "0x33250 - PMA transmit low data isolation register"]
    #[inline(always)]
    pub const fn pma_iso_tx_data_lo_lane2(&self) -> &PmaIsoTxDataLoLane2 {
        &self.pma_iso_tx_data_lo_lane2
    }
    #[doc = "0x33254 - PMA transmit high data isolation register"]
    #[inline(always)]
    pub const fn pma_iso_tx_data_hi_lane2(&self) -> &PmaIsoTxDataHiLane2 {
        &self.pma_iso_tx_data_hi_lane2
    }
    #[doc = "0x33258 - PMA receive low data isolation register"]
    #[inline(always)]
    pub const fn pma_iso_rx_data_lo_lane2(&self) -> &PmaIsoRxDataLoLane2 {
        &self.pma_iso_rx_data_lo_lane2
    }
    #[doc = "0x3325c - PMA receive high data isolation register"]
    #[inline(always)]
    pub const fn pma_iso_rx_data_hi_lane2(&self) -> &PmaIsoRxDataHiLane2 {
        &self.pma_iso_rx_data_hi_lane2
    }
    #[doc = "0x33340 - PMA Isolation Tansceiver control register"]
    #[inline(always)]
    pub const fn pma_iso_xcvr_ctrl_lane3(&self) -> &PmaIsoXcvrCtrlLane3 {
        &self.pma_iso_xcvr_ctrl_lane3
    }
    #[doc = "0x33344 - PMA TX configuration register"]
    #[inline(always)]
    pub const fn pma_iso_tx_cfg_lane3(&self) -> &PmaIsoTxCfgLane3 {
        &self.pma_iso_tx_cfg_lane3
    }
    #[doc = "0x33348 - PMA Isolation mode control register"]
    #[inline(always)]
    pub const fn pma_iso_link_mode_lane3(&self) -> &PmaIsoLinkModeLane3 {
        &self.pma_iso_link_mode_lane3
    }
    #[doc = "0x3334c - PMA Isolation power state control register"]
    #[inline(always)]
    pub const fn pma_iso_pwrst_ctrl_lane3(&self) -> &PmaIsoPwrstCtrlLane3 {
        &self.pma_iso_pwrst_ctrl_lane3
    }
    #[doc = "0x33350 - PMA transmit low data isolation register"]
    #[inline(always)]
    pub const fn pma_iso_tx_data_lo_lane3(&self) -> &PmaIsoTxDataLoLane3 {
        &self.pma_iso_tx_data_lo_lane3
    }
    #[doc = "0x33354 - PMA transmit high data isolation register"]
    #[inline(always)]
    pub const fn pma_iso_tx_data_hi_lane3(&self) -> &PmaIsoTxDataHiLane3 {
        &self.pma_iso_tx_data_hi_lane3
    }
    #[doc = "0x33358 - PMA receive low data isolation register"]
    #[inline(always)]
    pub const fn pma_iso_rx_data_lo_lane3(&self) -> &PmaIsoRxDataLoLane3 {
        &self.pma_iso_rx_data_lo_lane3
    }
    #[doc = "0x3335c - PMA receive high data isolation register"]
    #[inline(always)]
    pub const fn pma_iso_rx_data_hi_lane3(&self) -> &PmaIsoRxDataHiLane3 {
        &self.pma_iso_rx_data_hi_lane3
    }
}
#[doc = "PMA_LANE_CFG (rw) register accessor: PMA lane configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_lane_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_lane_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_lane_cfg`]
module"]
#[doc(alias = "PMA_LANE_CFG")]
pub type PmaLaneCfg = crate::Reg<pma_lane_cfg::PmaLaneCfgSpec>;
#[doc = "PMA lane configuration register"]
pub mod pma_lane_cfg;
#[doc = "PIPE_CMN_CTRL1 (rw) register accessor: PIPE common control1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pipe_cmn_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pipe_cmn_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pipe_cmn_ctrl1`]
module"]
#[doc(alias = "PIPE_CMN_CTRL1")]
pub type PipeCmnCtrl1 = crate::Reg<pipe_cmn_ctrl1::PipeCmnCtrl1Spec>;
#[doc = "PIPE common control1 register"]
pub mod pipe_cmn_ctrl1;
#[doc = "PIPE_CMN_CTRL2 (rw) register accessor: PIPE common control2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pipe_cmn_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pipe_cmn_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pipe_cmn_ctrl2`]
module"]
#[doc(alias = "PIPE_CMN_CTRL2")]
pub type PipeCmnCtrl2 = crate::Reg<pipe_cmn_ctrl2::PipeCmnCtrl2Spec>;
#[doc = "PIPE common control2 register"]
pub mod pipe_cmn_ctrl2;
#[doc = "PIPE_COM_LOCK_CFG1 (rw) register accessor: PIPE comma lock configuration1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pipe_com_lock_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pipe_com_lock_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pipe_com_lock_cfg1`]
module"]
#[doc(alias = "PIPE_COM_LOCK_CFG1")]
pub type PipeComLockCfg1 = crate::Reg<pipe_com_lock_cfg1::PipeComLockCfg1Spec>;
#[doc = "PIPE comma lock configuration1 register"]
pub mod pipe_com_lock_cfg1;
#[doc = "PIPE_COM_LOCK_CFG2 (rw) register accessor: PIPE comma lock configuration2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pipe_com_lock_cfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pipe_com_lock_cfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pipe_com_lock_cfg2`]
module"]
#[doc(alias = "PIPE_COM_LOCK_CFG2")]
pub type PipeComLockCfg2 = crate::Reg<pipe_com_lock_cfg2::PipeComLockCfg2Spec>;
#[doc = "PIPE comma lock configuration2 register"]
pub mod pipe_com_lock_cfg2;
#[doc = "PIPE_RCV_DET_INH (rw) register accessor: PIPE receiver detect inhibit register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pipe_rcv_det_inh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pipe_rcv_det_inh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pipe_rcv_det_inh`]
module"]
#[doc(alias = "PIPE_RCV_DET_INH")]
pub type PipeRcvDetInh = crate::Reg<pipe_rcv_det_inh::PipeRcvDetInhSpec>;
#[doc = "PIPE receiver detect inhibit register"]
pub mod pipe_rcv_det_inh;
#[doc = "USB_BER_CNT (r) register accessor: USB loopback slave BER count isolation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_ber_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_ber_cnt`]
module"]
#[doc(alias = "USB_BER_CNT")]
pub type UsbBerCnt = crate::Reg<usb_ber_cnt::UsbBerCntSpec>;
#[doc = "USB loopback slave BER count isolation register"]
pub mod usb_ber_cnt;
#[doc = "DP_MODE_CTL (rw) register accessor: DP Mode Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_mode_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_mode_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_mode_ctl`]
module"]
#[doc(alias = "DP_MODE_CTL")]
pub type DpModeCtl = crate::Reg<dp_mode_ctl::DpModeCtlSpec>;
#[doc = "DP Mode Control register"]
pub mod dp_mode_ctl;
#[doc = "DP_CLK_CTL (rw) register accessor: DP Clock Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_clk_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_clk_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_clk_ctl`]
module"]
#[doc(alias = "DP_CLK_CTL")]
pub type DpClkCtl = crate::Reg<dp_clk_ctl::DpClkCtlSpec>;
#[doc = "DP Clock Control register"]
pub mod dp_clk_ctl;
#[doc = "STS (r) register accessor: PHY status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`]
module"]
#[doc(alias = "STS")]
pub type Sts = crate::Reg<sts::StsSpec>;
#[doc = "PHY status register"]
pub mod sts;
#[doc = "DP_TX_CTL_LANE0 (rw) register accessor: DP Lane Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_tx_ctl_lane0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_tx_ctl_lane0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_tx_ctl_lane0`]
module"]
#[doc(alias = "DP_TX_CTL_LANE0")]
pub type DpTxCtlLane0 = crate::Reg<dp_tx_ctl_lane0::DpTxCtlLane0Spec>;
#[doc = "DP Lane Configuration register"]
pub mod dp_tx_ctl_lane0;
#[doc = "DP_TX_CTL_LANE1 (rw) register accessor: DP Lane Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_tx_ctl_lane1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_tx_ctl_lane1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_tx_ctl_lane1`]
module"]
#[doc(alias = "DP_TX_CTL_LANE1")]
pub type DpTxCtlLane1 = crate::Reg<dp_tx_ctl_lane1::DpTxCtlLane1Spec>;
#[doc = "DP Lane Configuration register"]
pub mod dp_tx_ctl_lane1;
#[doc = "DP_TX_CTL_LANE2 (rw) register accessor: DP Lane Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_tx_ctl_lane2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_tx_ctl_lane2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_tx_ctl_lane2`]
module"]
#[doc(alias = "DP_TX_CTL_LANE2")]
pub type DpTxCtlLane2 = crate::Reg<dp_tx_ctl_lane2::DpTxCtlLane2Spec>;
#[doc = "DP Lane Configuration register"]
pub mod dp_tx_ctl_lane2;
#[doc = "DP_TX_CTL_LANE3 (rw) register accessor: DP Lane Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_tx_ctl_lane3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_tx_ctl_lane3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_tx_ctl_lane3`]
module"]
#[doc(alias = "DP_TX_CTL_LANE3")]
pub type DpTxCtlLane3 = crate::Reg<dp_tx_ctl_lane3::DpTxCtlLane3Spec>;
#[doc = "DP Lane Configuration register"]
pub mod dp_tx_ctl_lane3;
#[doc = "PMA_CMN_CTRL1 (rw) register accessor: PMA common control1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_cmn_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_cmn_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_cmn_ctrl1`]
module"]
#[doc(alias = "PMA_CMN_CTRL1")]
pub type PmaCmnCtrl1 = crate::Reg<pma_cmn_ctrl1::PmaCmnCtrl1Spec>;
#[doc = "PMA common control1 register"]
pub mod pma_cmn_ctrl1;
#[doc = "PMA_ISO_PLL_CTRL0 (rw) register accessor: PMA PLL control0 isolation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_pll_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_pll_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_pll_ctrl0`]
module"]
#[doc(alias = "PMA_ISO_PLL_CTRL0")]
pub type PmaIsoPllCtrl0 = crate::Reg<pma_iso_pll_ctrl0::PmaIsoPllCtrl0Spec>;
#[doc = "PMA PLL control0 isolation register"]
pub mod pma_iso_pll_ctrl0;
#[doc = "PMA_ISO_PLL_CTRL1 (rw) register accessor: PMA PLL control1 isolation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_pll_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_pll_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_pll_ctrl1`]
module"]
#[doc(alias = "PMA_ISO_PLL_CTRL1")]
pub type PmaIsoPllCtrl1 = crate::Reg<pma_iso_pll_ctrl1::PmaIsoPllCtrl1Spec>;
#[doc = "PMA PLL control1 isolation register"]
pub mod pma_iso_pll_ctrl1;
#[doc = "ISOLATION_CTRL (rw) register accessor: Isolation control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isolation_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isolation_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isolation_ctrl`]
module"]
#[doc(alias = "ISOLATION_CTRL")]
pub type IsolationCtrl = crate::Reg<isolation_ctrl::IsolationCtrlSpec>;
#[doc = "Isolation control register"]
pub mod isolation_ctrl;
#[doc = "PMA_ISO_XCVR_CTRL_LANE0 (rw) register accessor: PMA Isolation Tansceiver control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_xcvr_ctrl_lane0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_xcvr_ctrl_lane0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_xcvr_ctrl_lane0`]
module"]
#[doc(alias = "PMA_ISO_XCVR_CTRL_LANE0")]
pub type PmaIsoXcvrCtrlLane0 = crate::Reg<pma_iso_xcvr_ctrl_lane0::PmaIsoXcvrCtrlLane0Spec>;
#[doc = "PMA Isolation Tansceiver control register"]
pub mod pma_iso_xcvr_ctrl_lane0;
#[doc = "PMA_ISO_XCVR_CTRL_LANE1 (rw) register accessor: PMA Isolation Tansceiver control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_xcvr_ctrl_lane1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_xcvr_ctrl_lane1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_xcvr_ctrl_lane1`]
module"]
#[doc(alias = "PMA_ISO_XCVR_CTRL_LANE1")]
pub type PmaIsoXcvrCtrlLane1 = crate::Reg<pma_iso_xcvr_ctrl_lane1::PmaIsoXcvrCtrlLane1Spec>;
#[doc = "PMA Isolation Tansceiver control register"]
pub mod pma_iso_xcvr_ctrl_lane1;
#[doc = "PMA_ISO_XCVR_CTRL_LANE2 (rw) register accessor: PMA Isolation Tansceiver control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_xcvr_ctrl_lane2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_xcvr_ctrl_lane2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_xcvr_ctrl_lane2`]
module"]
#[doc(alias = "PMA_ISO_XCVR_CTRL_LANE2")]
pub type PmaIsoXcvrCtrlLane2 = crate::Reg<pma_iso_xcvr_ctrl_lane2::PmaIsoXcvrCtrlLane2Spec>;
#[doc = "PMA Isolation Tansceiver control register"]
pub mod pma_iso_xcvr_ctrl_lane2;
#[doc = "PMA_ISO_XCVR_CTRL_LANE3 (rw) register accessor: PMA Isolation Tansceiver control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_xcvr_ctrl_lane3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_xcvr_ctrl_lane3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_xcvr_ctrl_lane3`]
module"]
#[doc(alias = "PMA_ISO_XCVR_CTRL_LANE3")]
pub type PmaIsoXcvrCtrlLane3 = crate::Reg<pma_iso_xcvr_ctrl_lane3::PmaIsoXcvrCtrlLane3Spec>;
#[doc = "PMA Isolation Tansceiver control register"]
pub mod pma_iso_xcvr_ctrl_lane3;
#[doc = "PMA_ISO_TX_CFG_LANE0 (rw) register accessor: PMA TX configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_tx_cfg_lane0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_tx_cfg_lane0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_tx_cfg_lane0`]
module"]
#[doc(alias = "PMA_ISO_TX_CFG_LANE0")]
pub type PmaIsoTxCfgLane0 = crate::Reg<pma_iso_tx_cfg_lane0::PmaIsoTxCfgLane0Spec>;
#[doc = "PMA TX configuration register"]
pub mod pma_iso_tx_cfg_lane0;
#[doc = "PMA_ISO_TX_CFG_LANE1 (rw) register accessor: PMA TX configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_tx_cfg_lane1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_tx_cfg_lane1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_tx_cfg_lane1`]
module"]
#[doc(alias = "PMA_ISO_TX_CFG_LANE1")]
pub type PmaIsoTxCfgLane1 = crate::Reg<pma_iso_tx_cfg_lane1::PmaIsoTxCfgLane1Spec>;
#[doc = "PMA TX configuration register"]
pub mod pma_iso_tx_cfg_lane1;
#[doc = "PMA_ISO_TX_CFG_LANE2 (rw) register accessor: PMA TX configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_tx_cfg_lane2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_tx_cfg_lane2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_tx_cfg_lane2`]
module"]
#[doc(alias = "PMA_ISO_TX_CFG_LANE2")]
pub type PmaIsoTxCfgLane2 = crate::Reg<pma_iso_tx_cfg_lane2::PmaIsoTxCfgLane2Spec>;
#[doc = "PMA TX configuration register"]
pub mod pma_iso_tx_cfg_lane2;
#[doc = "PMA_ISO_TX_CFG_LANE3 (rw) register accessor: PMA TX configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_tx_cfg_lane3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_tx_cfg_lane3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_tx_cfg_lane3`]
module"]
#[doc(alias = "PMA_ISO_TX_CFG_LANE3")]
pub type PmaIsoTxCfgLane3 = crate::Reg<pma_iso_tx_cfg_lane3::PmaIsoTxCfgLane3Spec>;
#[doc = "PMA TX configuration register"]
pub mod pma_iso_tx_cfg_lane3;
#[doc = "PMA_ISO_LINK_MODE_LANE0 (rw) register accessor: PMA Isolation mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_link_mode_lane0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_link_mode_lane0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_link_mode_lane0`]
module"]
#[doc(alias = "PMA_ISO_LINK_MODE_LANE0")]
pub type PmaIsoLinkModeLane0 = crate::Reg<pma_iso_link_mode_lane0::PmaIsoLinkModeLane0Spec>;
#[doc = "PMA Isolation mode control register"]
pub mod pma_iso_link_mode_lane0;
#[doc = "PMA_ISO_LINK_MODE_LANE1 (rw) register accessor: PMA Isolation mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_link_mode_lane1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_link_mode_lane1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_link_mode_lane1`]
module"]
#[doc(alias = "PMA_ISO_LINK_MODE_LANE1")]
pub type PmaIsoLinkModeLane1 = crate::Reg<pma_iso_link_mode_lane1::PmaIsoLinkModeLane1Spec>;
#[doc = "PMA Isolation mode control register"]
pub mod pma_iso_link_mode_lane1;
#[doc = "PMA_ISO_LINK_MODE_LANE2 (rw) register accessor: PMA Isolation mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_link_mode_lane2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_link_mode_lane2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_link_mode_lane2`]
module"]
#[doc(alias = "PMA_ISO_LINK_MODE_LANE2")]
pub type PmaIsoLinkModeLane2 = crate::Reg<pma_iso_link_mode_lane2::PmaIsoLinkModeLane2Spec>;
#[doc = "PMA Isolation mode control register"]
pub mod pma_iso_link_mode_lane2;
#[doc = "PMA_ISO_LINK_MODE_LANE3 (rw) register accessor: PMA Isolation mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_link_mode_lane3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_link_mode_lane3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_link_mode_lane3`]
module"]
#[doc(alias = "PMA_ISO_LINK_MODE_LANE3")]
pub type PmaIsoLinkModeLane3 = crate::Reg<pma_iso_link_mode_lane3::PmaIsoLinkModeLane3Spec>;
#[doc = "PMA Isolation mode control register"]
pub mod pma_iso_link_mode_lane3;
#[doc = "PMA_ISO_PWRST_CTRL_LANE0 (rw) register accessor: PMA Isolation power state control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_pwrst_ctrl_lane0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_pwrst_ctrl_lane0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_pwrst_ctrl_lane0`]
module"]
#[doc(alias = "PMA_ISO_PWRST_CTRL_LANE0")]
pub type PmaIsoPwrstCtrlLane0 = crate::Reg<pma_iso_pwrst_ctrl_lane0::PmaIsoPwrstCtrlLane0Spec>;
#[doc = "PMA Isolation power state control register"]
pub mod pma_iso_pwrst_ctrl_lane0;
#[doc = "PMA_ISO_PWRST_CTRL_LANE1 (rw) register accessor: PMA Isolation power state control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_pwrst_ctrl_lane1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_pwrst_ctrl_lane1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_pwrst_ctrl_lane1`]
module"]
#[doc(alias = "PMA_ISO_PWRST_CTRL_LANE1")]
pub type PmaIsoPwrstCtrlLane1 = crate::Reg<pma_iso_pwrst_ctrl_lane1::PmaIsoPwrstCtrlLane1Spec>;
#[doc = "PMA Isolation power state control register"]
pub mod pma_iso_pwrst_ctrl_lane1;
#[doc = "PMA_ISO_PWRST_CTRL_LANE2 (rw) register accessor: PMA Isolation power state control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_pwrst_ctrl_lane2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_pwrst_ctrl_lane2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_pwrst_ctrl_lane2`]
module"]
#[doc(alias = "PMA_ISO_PWRST_CTRL_LANE2")]
pub type PmaIsoPwrstCtrlLane2 = crate::Reg<pma_iso_pwrst_ctrl_lane2::PmaIsoPwrstCtrlLane2Spec>;
#[doc = "PMA Isolation power state control register"]
pub mod pma_iso_pwrst_ctrl_lane2;
#[doc = "PMA_ISO_PWRST_CTRL_LANE3 (rw) register accessor: PMA Isolation power state control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_pwrst_ctrl_lane3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_pwrst_ctrl_lane3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_pwrst_ctrl_lane3`]
module"]
#[doc(alias = "PMA_ISO_PWRST_CTRL_LANE3")]
pub type PmaIsoPwrstCtrlLane3 = crate::Reg<pma_iso_pwrst_ctrl_lane3::PmaIsoPwrstCtrlLane3Spec>;
#[doc = "PMA Isolation power state control register"]
pub mod pma_iso_pwrst_ctrl_lane3;
#[doc = "PMA_ISO_TX_DATA_LO_LANE0 (rw) register accessor: PMA transmit low data isolation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_tx_data_lo_lane0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_tx_data_lo_lane0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_tx_data_lo_lane0`]
module"]
#[doc(alias = "PMA_ISO_TX_DATA_LO_LANE0")]
pub type PmaIsoTxDataLoLane0 = crate::Reg<pma_iso_tx_data_lo_lane0::PmaIsoTxDataLoLane0Spec>;
#[doc = "PMA transmit low data isolation register"]
pub mod pma_iso_tx_data_lo_lane0;
#[doc = "PMA_ISO_TX_DATA_LO_LANE1 (rw) register accessor: PMA transmit low data isolation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_tx_data_lo_lane1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_tx_data_lo_lane1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_tx_data_lo_lane1`]
module"]
#[doc(alias = "PMA_ISO_TX_DATA_LO_LANE1")]
pub type PmaIsoTxDataLoLane1 = crate::Reg<pma_iso_tx_data_lo_lane1::PmaIsoTxDataLoLane1Spec>;
#[doc = "PMA transmit low data isolation register"]
pub mod pma_iso_tx_data_lo_lane1;
#[doc = "PMA_ISO_TX_DATA_LO_LANE2 (rw) register accessor: PMA transmit low data isolation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_tx_data_lo_lane2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_tx_data_lo_lane2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_tx_data_lo_lane2`]
module"]
#[doc(alias = "PMA_ISO_TX_DATA_LO_LANE2")]
pub type PmaIsoTxDataLoLane2 = crate::Reg<pma_iso_tx_data_lo_lane2::PmaIsoTxDataLoLane2Spec>;
#[doc = "PMA transmit low data isolation register"]
pub mod pma_iso_tx_data_lo_lane2;
#[doc = "PMA_ISO_TX_DATA_LO_LANE3 (rw) register accessor: PMA transmit low data isolation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_tx_data_lo_lane3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_tx_data_lo_lane3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_tx_data_lo_lane3`]
module"]
#[doc(alias = "PMA_ISO_TX_DATA_LO_LANE3")]
pub type PmaIsoTxDataLoLane3 = crate::Reg<pma_iso_tx_data_lo_lane3::PmaIsoTxDataLoLane3Spec>;
#[doc = "PMA transmit low data isolation register"]
pub mod pma_iso_tx_data_lo_lane3;
#[doc = "PMA_ISO_TX_DATA_HI_LANE0 (rw) register accessor: PMA transmit high data isolation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_tx_data_hi_lane0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_tx_data_hi_lane0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_tx_data_hi_lane0`]
module"]
#[doc(alias = "PMA_ISO_TX_DATA_HI_LANE0")]
pub type PmaIsoTxDataHiLane0 = crate::Reg<pma_iso_tx_data_hi_lane0::PmaIsoTxDataHiLane0Spec>;
#[doc = "PMA transmit high data isolation register"]
pub mod pma_iso_tx_data_hi_lane0;
#[doc = "PMA_ISO_TX_DATA_HI_LANE1 (rw) register accessor: PMA transmit high data isolation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_tx_data_hi_lane1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_tx_data_hi_lane1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_tx_data_hi_lane1`]
module"]
#[doc(alias = "PMA_ISO_TX_DATA_HI_LANE1")]
pub type PmaIsoTxDataHiLane1 = crate::Reg<pma_iso_tx_data_hi_lane1::PmaIsoTxDataHiLane1Spec>;
#[doc = "PMA transmit high data isolation register"]
pub mod pma_iso_tx_data_hi_lane1;
#[doc = "PMA_ISO_TX_DATA_HI_LANE2 (rw) register accessor: PMA transmit high data isolation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_tx_data_hi_lane2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_tx_data_hi_lane2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_tx_data_hi_lane2`]
module"]
#[doc(alias = "PMA_ISO_TX_DATA_HI_LANE2")]
pub type PmaIsoTxDataHiLane2 = crate::Reg<pma_iso_tx_data_hi_lane2::PmaIsoTxDataHiLane2Spec>;
#[doc = "PMA transmit high data isolation register"]
pub mod pma_iso_tx_data_hi_lane2;
#[doc = "PMA_ISO_TX_DATA_HI_LANE3 (rw) register accessor: PMA transmit high data isolation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_tx_data_hi_lane3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_tx_data_hi_lane3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_tx_data_hi_lane3`]
module"]
#[doc(alias = "PMA_ISO_TX_DATA_HI_LANE3")]
pub type PmaIsoTxDataHiLane3 = crate::Reg<pma_iso_tx_data_hi_lane3::PmaIsoTxDataHiLane3Spec>;
#[doc = "PMA transmit high data isolation register"]
pub mod pma_iso_tx_data_hi_lane3;
#[doc = "PMA_ISO_RX_DATA_LO_LANE0 (r) register accessor: PMA receive low data isolation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_rx_data_lo_lane0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_rx_data_lo_lane0`]
module"]
#[doc(alias = "PMA_ISO_RX_DATA_LO_LANE0")]
pub type PmaIsoRxDataLoLane0 = crate::Reg<pma_iso_rx_data_lo_lane0::PmaIsoRxDataLoLane0Spec>;
#[doc = "PMA receive low data isolation register"]
pub mod pma_iso_rx_data_lo_lane0;
#[doc = "PMA_ISO_RX_DATA_LO_LANE1 (r) register accessor: PMA receive low data isolation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_rx_data_lo_lane1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_rx_data_lo_lane1`]
module"]
#[doc(alias = "PMA_ISO_RX_DATA_LO_LANE1")]
pub type PmaIsoRxDataLoLane1 = crate::Reg<pma_iso_rx_data_lo_lane1::PmaIsoRxDataLoLane1Spec>;
#[doc = "PMA receive low data isolation register"]
pub mod pma_iso_rx_data_lo_lane1;
#[doc = "PMA_ISO_RX_DATA_LO_LANE2 (r) register accessor: PMA receive low data isolation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_rx_data_lo_lane2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_rx_data_lo_lane2`]
module"]
#[doc(alias = "PMA_ISO_RX_DATA_LO_LANE2")]
pub type PmaIsoRxDataLoLane2 = crate::Reg<pma_iso_rx_data_lo_lane2::PmaIsoRxDataLoLane2Spec>;
#[doc = "PMA receive low data isolation register"]
pub mod pma_iso_rx_data_lo_lane2;
#[doc = "PMA_ISO_RX_DATA_LO_LANE3 (r) register accessor: PMA receive low data isolation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_rx_data_lo_lane3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_rx_data_lo_lane3`]
module"]
#[doc(alias = "PMA_ISO_RX_DATA_LO_LANE3")]
pub type PmaIsoRxDataLoLane3 = crate::Reg<pma_iso_rx_data_lo_lane3::PmaIsoRxDataLoLane3Spec>;
#[doc = "PMA receive low data isolation register"]
pub mod pma_iso_rx_data_lo_lane3;
#[doc = "PMA_ISO_RX_DATA_HI_LANE0 (r) register accessor: PMA receive high data isolation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_rx_data_hi_lane0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_rx_data_hi_lane0`]
module"]
#[doc(alias = "PMA_ISO_RX_DATA_HI_LANE0")]
pub type PmaIsoRxDataHiLane0 = crate::Reg<pma_iso_rx_data_hi_lane0::PmaIsoRxDataHiLane0Spec>;
#[doc = "PMA receive high data isolation register"]
pub mod pma_iso_rx_data_hi_lane0;
#[doc = "PMA_ISO_RX_DATA_HI_LANE1 (r) register accessor: PMA receive high data isolation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_rx_data_hi_lane1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_rx_data_hi_lane1`]
module"]
#[doc(alias = "PMA_ISO_RX_DATA_HI_LANE1")]
pub type PmaIsoRxDataHiLane1 = crate::Reg<pma_iso_rx_data_hi_lane1::PmaIsoRxDataHiLane1Spec>;
#[doc = "PMA receive high data isolation register"]
pub mod pma_iso_rx_data_hi_lane1;
#[doc = "PMA_ISO_RX_DATA_HI_LANE2 (r) register accessor: PMA receive high data isolation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_rx_data_hi_lane2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_rx_data_hi_lane2`]
module"]
#[doc(alias = "PMA_ISO_RX_DATA_HI_LANE2")]
pub type PmaIsoRxDataHiLane2 = crate::Reg<pma_iso_rx_data_hi_lane2::PmaIsoRxDataHiLane2Spec>;
#[doc = "PMA receive high data isolation register"]
pub mod pma_iso_rx_data_hi_lane2;
#[doc = "PMA_ISO_RX_DATA_HI_LANE3 (r) register accessor: PMA receive high data isolation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_rx_data_hi_lane3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_iso_rx_data_hi_lane3`]
module"]
#[doc(alias = "PMA_ISO_RX_DATA_HI_LANE3")]
pub type PmaIsoRxDataHiLane3 = crate::Reg<pma_iso_rx_data_hi_lane3::PmaIsoRxDataHiLane3Spec>;
#[doc = "PMA receive high data isolation register"]
pub mod pma_iso_rx_data_hi_lane3;
