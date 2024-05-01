#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    version: Version,
    pwr_up: PwrUp,
    clkmgr_cfg: ClkmgrCfg,
    dpi_vcid: DpiVcid,
    dpi_color_coding: DpiColorCoding,
    dpi_cfg_pol: DpiCfgPol,
    dpi_lp_cmd_tim: DpiLpCmdTim,
    _reserved7: [u8; 0x10],
    pckhdl_cfg: PckhdlCfg,
    gen_vcid: GenVcid,
    mode_cfg: ModeCfg,
    vid_mode_cfg: VidModeCfg,
    vid_pkt_size: VidPktSize,
    vid_num_chunks: VidNumChunks,
    vid_null_size: VidNullSize,
    vid_hsa_time: VidHsaTime,
    vid_hbp_time: VidHbpTime,
    vid_hline_time: VidHlineTime,
    vid_vsa_lines: VidVsaLines,
    vid_vbp_lines: VidVbpLines,
    vid_vfp_lines: VidVfpLines,
    vid_vactive_lines: VidVactiveLines,
    edpi_cmd_size: EdpiCmdSize,
    cmd_mode_cfg: CmdModeCfg,
    gen_hdr: GenHdr,
    gen_pld_data: GenPldData,
    cmd_pkt_status: CmdPktStatus,
    to_cnt_cfg: ToCntCfg,
    hs_rd_to_cnt: HsRdToCnt,
    lp_rd_to_cnt: LpRdToCnt,
    hs_wr_to_cnt: HsWrToCnt,
    lp_wr_to_cnt: LpWrToCnt,
    bta_to_cnt: BtaToCnt,
    _reserved32: [u8; 0x04],
    lpclk_ctrl: LpclkCtrl,
    phy_tmr_lpclk_cfg: PhyTmrLpclkCfg,
    phy_tmr_cfg: PhyTmrCfg,
    phy_rstz: PhyRstz,
    phy_if_cfg: PhyIfCfg,
    phy_ulps_ctrl: PhyUlpsCtrl,
    phy_tx_triggers: PhyTxTriggers,
    phy_status: PhyStatus,
    phy_tst_ctrl0: PhyTstCtrl0,
    phy_tst_ctrl1: PhyTstCtrl1,
    int_st0: IntSt0,
    int_st1: IntSt1,
    int_msk0: IntMsk0,
    int_msk1: IntMsk1,
    _reserved46: [u8; 0x0c],
    int_force0: IntForce0,
    int_force1: IntForce1,
}
impl RegisterBlock {
    #[doc = "0x00 - VERSION"]
    #[inline(always)]
    pub const fn version(&self) -> &Version {
        &self.version
    }
    #[doc = "0x04 - PWR_UP"]
    #[inline(always)]
    pub const fn pwr_up(&self) -> &PwrUp {
        &self.pwr_up
    }
    #[doc = "0x08 - Internal Clock Dividers Configuration Register"]
    #[inline(always)]
    pub const fn clkmgr_cfg(&self) -> &ClkmgrCfg {
        &self.clkmgr_cfg
    }
    #[doc = "0x0c - DPI Virtual Channel ID Register"]
    #[inline(always)]
    pub const fn dpi_vcid(&self) -> &DpiVcid {
        &self.dpi_vcid
    }
    #[doc = "0x10 - DPI Color Coding Register"]
    #[inline(always)]
    pub const fn dpi_color_coding(&self) -> &DpiColorCoding {
        &self.dpi_color_coding
    }
    #[doc = "0x14 - DPI Polarity Configuration Register"]
    #[inline(always)]
    pub const fn dpi_cfg_pol(&self) -> &DpiCfgPol {
        &self.dpi_cfg_pol
    }
    #[doc = "0x18 - Low-Power Command Timing Configuration Register"]
    #[inline(always)]
    pub const fn dpi_lp_cmd_tim(&self) -> &DpiLpCmdTim {
        &self.dpi_lp_cmd_tim
    }
    #[doc = "0x2c - Packet Handler Configuration Register"]
    #[inline(always)]
    pub const fn pckhdl_cfg(&self) -> &PckhdlCfg {
        &self.pckhdl_cfg
    }
    #[doc = "0x30 - Generic Interface Virtual Channel Id Register"]
    #[inline(always)]
    pub const fn gen_vcid(&self) -> &GenVcid {
        &self.gen_vcid
    }
    #[doc = "0x34 - Register0000 Abstract"]
    #[inline(always)]
    pub const fn mode_cfg(&self) -> &ModeCfg {
        &self.mode_cfg
    }
    #[doc = "0x38 - Video Mode Configuration Register"]
    #[inline(always)]
    pub const fn vid_mode_cfg(&self) -> &VidModeCfg {
        &self.vid_mode_cfg
    }
    #[doc = "0x3c - Video Packet Size Register"]
    #[inline(always)]
    pub const fn vid_pkt_size(&self) -> &VidPktSize {
        &self.vid_pkt_size
    }
    #[doc = "0x40 - Number Of Chunks Register"]
    #[inline(always)]
    pub const fn vid_num_chunks(&self) -> &VidNumChunks {
        &self.vid_num_chunks
    }
    #[doc = "0x44 - Null Packet Size Register"]
    #[inline(always)]
    pub const fn vid_null_size(&self) -> &VidNullSize {
        &self.vid_null_size
    }
    #[doc = "0x48 - Horizontal Sync Active Time Register"]
    #[inline(always)]
    pub const fn vid_hsa_time(&self) -> &VidHsaTime {
        &self.vid_hsa_time
    }
    #[doc = "0x4c - Register0005 Abstract"]
    #[inline(always)]
    pub const fn vid_hbp_time(&self) -> &VidHbpTime {
        &self.vid_hbp_time
    }
    #[doc = "0x50 - Line Time Register"]
    #[inline(always)]
    pub const fn vid_hline_time(&self) -> &VidHlineTime {
        &self.vid_hline_time
    }
    #[doc = "0x54 - VID_VSA_LINES"]
    #[inline(always)]
    pub const fn vid_vsa_lines(&self) -> &VidVsaLines {
        &self.vid_vsa_lines
    }
    #[doc = "0x58 - Vertical Back Porch Period Register"]
    #[inline(always)]
    pub const fn vid_vbp_lines(&self) -> &VidVbpLines {
        &self.vid_vbp_lines
    }
    #[doc = "0x5c - Vertical Front Porch Period Register"]
    #[inline(always)]
    pub const fn vid_vfp_lines(&self) -> &VidVfpLines {
        &self.vid_vfp_lines
    }
    #[doc = "0x60 - Vertical Resolution Register"]
    #[inline(always)]
    pub const fn vid_vactive_lines(&self) -> &VidVactiveLines {
        &self.vid_vactive_lines
    }
    #[doc = "0x64 - eDPI Packet Size Register"]
    #[inline(always)]
    pub const fn edpi_cmd_size(&self) -> &EdpiCmdSize {
        &self.edpi_cmd_size
    }
    #[doc = "0x68 - Command Mode Configuration Register"]
    #[inline(always)]
    pub const fn cmd_mode_cfg(&self) -> &CmdModeCfg {
        &self.cmd_mode_cfg
    }
    #[doc = "0x6c - Generic Packet Header Configuration Register"]
    #[inline(always)]
    pub const fn gen_hdr(&self) -> &GenHdr {
        &self.gen_hdr
    }
    #[doc = "0x70 - Generic Payload Data In And Out Register"]
    #[inline(always)]
    pub const fn gen_pld_data(&self) -> &GenPldData {
        &self.gen_pld_data
    }
    #[doc = "0x74 - Command Packet Status Register"]
    #[inline(always)]
    pub const fn cmd_pkt_status(&self) -> &CmdPktStatus {
        &self.cmd_pkt_status
    }
    #[doc = "0x78 - Timeout Timers Configuration Register"]
    #[inline(always)]
    pub const fn to_cnt_cfg(&self) -> &ToCntCfg {
        &self.to_cnt_cfg
    }
    #[doc = "0x7c - Peripheral Response Timeout Definition after Hi"]
    #[inline(always)]
    pub const fn hs_rd_to_cnt(&self) -> &HsRdToCnt {
        &self.hs_rd_to_cnt
    }
    #[doc = "0x80 - Peripheral Response Timeout Definition after Lo"]
    #[inline(always)]
    pub const fn lp_rd_to_cnt(&self) -> &LpRdToCnt {
        &self.lp_rd_to_cnt
    }
    #[doc = "0x84 - Peripheral Response Timeout Definition after Hi"]
    #[inline(always)]
    pub const fn hs_wr_to_cnt(&self) -> &HsWrToCnt {
        &self.hs_wr_to_cnt
    }
    #[doc = "0x88 - Peripheral Response Timeout Definition after Lo"]
    #[inline(always)]
    pub const fn lp_wr_to_cnt(&self) -> &LpWrToCnt {
        &self.lp_wr_to_cnt
    }
    #[doc = "0x8c - Peripheral Response Timeout Definition after B"]
    #[inline(always)]
    pub const fn bta_to_cnt(&self) -> &BtaToCnt {
        &self.bta_to_cnt
    }
    #[doc = "0x94 - Low-power in Clock Lane Register"]
    #[inline(always)]
    pub const fn lpclk_ctrl(&self) -> &LpclkCtrl {
        &self.lpclk_ctrl
    }
    #[doc = "0x98 - D-PHY Timing Configuration for the Clock Lane"]
    #[inline(always)]
    pub const fn phy_tmr_lpclk_cfg(&self) -> &PhyTmrLpclkCfg {
        &self.phy_tmr_lpclk_cfg
    }
    #[doc = "0x9c - D-PHY Data Lanes Timing Configuration Registe"]
    #[inline(always)]
    pub const fn phy_tmr_cfg(&self) -> &PhyTmrCfg {
        &self.phy_tmr_cfg
    }
    #[doc = "0xa0 - D-PHY Reset Control Register"]
    #[inline(always)]
    pub const fn phy_rstz(&self) -> &PhyRstz {
        &self.phy_rstz
    }
    #[doc = "0xa4 - D-PHY Interface Configuration Register"]
    #[inline(always)]
    pub const fn phy_if_cfg(&self) -> &PhyIfCfg {
        &self.phy_if_cfg
    }
    #[doc = "0xa8 - D-PHY Ultra Low-Power Control Register"]
    #[inline(always)]
    pub const fn phy_ulps_ctrl(&self) -> &PhyUlpsCtrl {
        &self.phy_ulps_ctrl
    }
    #[doc = "0xac - D-PHY Transmit Triggers Register"]
    #[inline(always)]
    pub const fn phy_tx_triggers(&self) -> &PhyTxTriggers {
        &self.phy_tx_triggers
    }
    #[doc = "0xb0 - Register0010 Abstract"]
    #[inline(always)]
    pub const fn phy_status(&self) -> &PhyStatus {
        &self.phy_status
    }
    #[doc = "0xb4 - D-PHY Test Interface Control 0 Register"]
    #[inline(always)]
    pub const fn phy_tst_ctrl0(&self) -> &PhyTstCtrl0 {
        &self.phy_tst_ctrl0
    }
    #[doc = "0xb8 - D-PHY Test Interface Control 1 Register"]
    #[inline(always)]
    pub const fn phy_tst_ctrl1(&self) -> &PhyTstCtrl1 {
        &self.phy_tst_ctrl1
    }
    #[doc = "0xbc - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn int_st0(&self) -> &IntSt0 {
        &self.int_st0
    }
    #[doc = "0xc0 - Interrupt Status Register 1"]
    #[inline(always)]
    pub const fn int_st1(&self) -> &IntSt1 {
        &self.int_st1
    }
    #[doc = "0xc4 - Masks the Interrupt Generation Triggered by the"]
    #[inline(always)]
    pub const fn int_msk0(&self) -> &IntMsk0 {
        &self.int_msk0
    }
    #[doc = "0xc8 - Masks the Interrupt Generation Triggered by the"]
    #[inline(always)]
    pub const fn int_msk1(&self) -> &IntMsk1 {
        &self.int_msk1
    }
    #[doc = "0xd8 - Force Interrupt Configuration Register"]
    #[inline(always)]
    pub const fn int_force0(&self) -> &IntForce0 {
        &self.int_force0
    }
    #[doc = "0xdc - Force Interrupt Configuration Register"]
    #[inline(always)]
    pub const fn int_force1(&self) -> &IntForce1 {
        &self.int_force1
    }
}
#[doc = "VERSION (rw) register accessor: VERSION\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`version::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@version`]
module"]
#[doc(alias = "VERSION")]
pub type Version = crate::Reg<version::VersionSpec>;
#[doc = "VERSION"]
pub mod version;
#[doc = "PWR_UP (rw) register accessor: PWR_UP\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_up::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_up::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_up`]
module"]
#[doc(alias = "PWR_UP")]
pub type PwrUp = crate::Reg<pwr_up::PwrUpSpec>;
#[doc = "PWR_UP"]
pub mod pwr_up;
#[doc = "CLKMGR_CFG (rw) register accessor: Internal Clock Dividers Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkmgr_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkmgr_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkmgr_cfg`]
module"]
#[doc(alias = "CLKMGR_CFG")]
pub type ClkmgrCfg = crate::Reg<clkmgr_cfg::ClkmgrCfgSpec>;
#[doc = "Internal Clock Dividers Configuration Register"]
pub mod clkmgr_cfg;
#[doc = "DPI_VCID (rw) register accessor: DPI Virtual Channel ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpi_vcid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpi_vcid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpi_vcid`]
module"]
#[doc(alias = "DPI_VCID")]
pub type DpiVcid = crate::Reg<dpi_vcid::DpiVcidSpec>;
#[doc = "DPI Virtual Channel ID Register"]
pub mod dpi_vcid;
#[doc = "DPI_COLOR_CODING (rw) register accessor: DPI Color Coding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpi_color_coding::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpi_color_coding::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpi_color_coding`]
module"]
#[doc(alias = "DPI_COLOR_CODING")]
pub type DpiColorCoding = crate::Reg<dpi_color_coding::DpiColorCodingSpec>;
#[doc = "DPI Color Coding Register"]
pub mod dpi_color_coding;
#[doc = "DPI_CFG_POL (rw) register accessor: DPI Polarity Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpi_cfg_pol::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpi_cfg_pol::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpi_cfg_pol`]
module"]
#[doc(alias = "DPI_CFG_POL")]
pub type DpiCfgPol = crate::Reg<dpi_cfg_pol::DpiCfgPolSpec>;
#[doc = "DPI Polarity Configuration Register"]
pub mod dpi_cfg_pol;
#[doc = "DPI_LP_CMD_TIM (rw) register accessor: Low-Power Command Timing Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpi_lp_cmd_tim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpi_lp_cmd_tim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpi_lp_cmd_tim`]
module"]
#[doc(alias = "DPI_LP_CMD_TIM")]
pub type DpiLpCmdTim = crate::Reg<dpi_lp_cmd_tim::DpiLpCmdTimSpec>;
#[doc = "Low-Power Command Timing Configuration Register"]
pub mod dpi_lp_cmd_tim;
#[doc = "PCKHDL_CFG (rw) register accessor: Packet Handler Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pckhdl_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pckhdl_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pckhdl_cfg`]
module"]
#[doc(alias = "PCKHDL_CFG")]
pub type PckhdlCfg = crate::Reg<pckhdl_cfg::PckhdlCfgSpec>;
#[doc = "Packet Handler Configuration Register"]
pub mod pckhdl_cfg;
#[doc = "GEN_VCID (rw) register accessor: Generic Interface Virtual Channel Id Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen_vcid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_vcid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen_vcid`]
module"]
#[doc(alias = "GEN_VCID")]
pub type GenVcid = crate::Reg<gen_vcid::GenVcidSpec>;
#[doc = "Generic Interface Virtual Channel Id Register"]
pub mod gen_vcid;
#[doc = "MODE_CFG (rw) register accessor: Register0000 Abstract\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode_cfg`]
module"]
#[doc(alias = "MODE_CFG")]
pub type ModeCfg = crate::Reg<mode_cfg::ModeCfgSpec>;
#[doc = "Register0000 Abstract"]
pub mod mode_cfg;
#[doc = "VID_MODE_CFG (rw) register accessor: Video Mode Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vid_mode_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_mode_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vid_mode_cfg`]
module"]
#[doc(alias = "VID_MODE_CFG")]
pub type VidModeCfg = crate::Reg<vid_mode_cfg::VidModeCfgSpec>;
#[doc = "Video Mode Configuration Register"]
pub mod vid_mode_cfg;
#[doc = "VID_PKT_SIZE (rw) register accessor: Video Packet Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vid_pkt_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_pkt_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vid_pkt_size`]
module"]
#[doc(alias = "VID_PKT_SIZE")]
pub type VidPktSize = crate::Reg<vid_pkt_size::VidPktSizeSpec>;
#[doc = "Video Packet Size Register"]
pub mod vid_pkt_size;
#[doc = "VID_NUM_CHUNKS (rw) register accessor: Number Of Chunks Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vid_num_chunks::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_num_chunks::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vid_num_chunks`]
module"]
#[doc(alias = "VID_NUM_CHUNKS")]
pub type VidNumChunks = crate::Reg<vid_num_chunks::VidNumChunksSpec>;
#[doc = "Number Of Chunks Register"]
pub mod vid_num_chunks;
#[doc = "VID_NULL_SIZE (rw) register accessor: Null Packet Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vid_null_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_null_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vid_null_size`]
module"]
#[doc(alias = "VID_NULL_SIZE")]
pub type VidNullSize = crate::Reg<vid_null_size::VidNullSizeSpec>;
#[doc = "Null Packet Size Register"]
pub mod vid_null_size;
#[doc = "VID_HSA_TIME (rw) register accessor: Horizontal Sync Active Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vid_hsa_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_hsa_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vid_hsa_time`]
module"]
#[doc(alias = "VID_HSA_TIME")]
pub type VidHsaTime = crate::Reg<vid_hsa_time::VidHsaTimeSpec>;
#[doc = "Horizontal Sync Active Time Register"]
pub mod vid_hsa_time;
#[doc = "VID_HBP_TIME (rw) register accessor: Register0005 Abstract\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vid_hbp_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_hbp_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vid_hbp_time`]
module"]
#[doc(alias = "VID_HBP_TIME")]
pub type VidHbpTime = crate::Reg<vid_hbp_time::VidHbpTimeSpec>;
#[doc = "Register0005 Abstract"]
pub mod vid_hbp_time;
#[doc = "VID_HLINE_TIME (rw) register accessor: Line Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vid_hline_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_hline_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vid_hline_time`]
module"]
#[doc(alias = "VID_HLINE_TIME")]
pub type VidHlineTime = crate::Reg<vid_hline_time::VidHlineTimeSpec>;
#[doc = "Line Time Register"]
pub mod vid_hline_time;
#[doc = "VID_VSA_LINES (rw) register accessor: VID_VSA_LINES\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vid_vsa_lines::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_vsa_lines::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vid_vsa_lines`]
module"]
#[doc(alias = "VID_VSA_LINES")]
pub type VidVsaLines = crate::Reg<vid_vsa_lines::VidVsaLinesSpec>;
#[doc = "VID_VSA_LINES"]
pub mod vid_vsa_lines;
#[doc = "VID_VBP_LINES (rw) register accessor: Vertical Back Porch Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vid_vbp_lines::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_vbp_lines::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vid_vbp_lines`]
module"]
#[doc(alias = "VID_VBP_LINES")]
pub type VidVbpLines = crate::Reg<vid_vbp_lines::VidVbpLinesSpec>;
#[doc = "Vertical Back Porch Period Register"]
pub mod vid_vbp_lines;
#[doc = "VID_VFP_LINES (rw) register accessor: Vertical Front Porch Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vid_vfp_lines::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_vfp_lines::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vid_vfp_lines`]
module"]
#[doc(alias = "VID_VFP_LINES")]
pub type VidVfpLines = crate::Reg<vid_vfp_lines::VidVfpLinesSpec>;
#[doc = "Vertical Front Porch Period Register"]
pub mod vid_vfp_lines;
#[doc = "VID_VACTIVE_LINES (rw) register accessor: Vertical Resolution Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vid_vactive_lines::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_vactive_lines::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vid_vactive_lines`]
module"]
#[doc(alias = "VID_VACTIVE_LINES")]
pub type VidVactiveLines = crate::Reg<vid_vactive_lines::VidVactiveLinesSpec>;
#[doc = "Vertical Resolution Register"]
pub mod vid_vactive_lines;
#[doc = "EDPI_CMD_SIZE (rw) register accessor: eDPI Packet Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edpi_cmd_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edpi_cmd_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edpi_cmd_size`]
module"]
#[doc(alias = "EDPI_CMD_SIZE")]
pub type EdpiCmdSize = crate::Reg<edpi_cmd_size::EdpiCmdSizeSpec>;
#[doc = "eDPI Packet Size Register"]
pub mod edpi_cmd_size;
#[doc = "CMD_MODE_CFG (rw) register accessor: Command Mode Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_mode_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_mode_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_mode_cfg`]
module"]
#[doc(alias = "CMD_MODE_CFG")]
pub type CmdModeCfg = crate::Reg<cmd_mode_cfg::CmdModeCfgSpec>;
#[doc = "Command Mode Configuration Register"]
pub mod cmd_mode_cfg;
#[doc = "GEN_HDR (rw) register accessor: Generic Packet Header Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen_hdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_hdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen_hdr`]
module"]
#[doc(alias = "GEN_HDR")]
pub type GenHdr = crate::Reg<gen_hdr::GenHdrSpec>;
#[doc = "Generic Packet Header Configuration Register"]
pub mod gen_hdr;
#[doc = "GEN_PLD_DATA (rw) register accessor: Generic Payload Data In And Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen_pld_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_pld_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen_pld_data`]
module"]
#[doc(alias = "GEN_PLD_DATA")]
pub type GenPldData = crate::Reg<gen_pld_data::GenPldDataSpec>;
#[doc = "Generic Payload Data In And Out Register"]
pub mod gen_pld_data;
#[doc = "CMD_PKT_STATUS (r) register accessor: Command Packet Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_pkt_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_pkt_status`]
module"]
#[doc(alias = "CMD_PKT_STATUS")]
pub type CmdPktStatus = crate::Reg<cmd_pkt_status::CmdPktStatusSpec>;
#[doc = "Command Packet Status Register"]
pub mod cmd_pkt_status;
#[doc = "TO_CNT_CFG (rw) register accessor: Timeout Timers Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`to_cnt_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`to_cnt_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@to_cnt_cfg`]
module"]
#[doc(alias = "TO_CNT_CFG")]
pub type ToCntCfg = crate::Reg<to_cnt_cfg::ToCntCfgSpec>;
#[doc = "Timeout Timers Configuration Register"]
pub mod to_cnt_cfg;
#[doc = "HS_RD_TO_CNT (rw) register accessor: Peripheral Response Timeout Definition after Hi\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hs_rd_to_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hs_rd_to_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hs_rd_to_cnt`]
module"]
#[doc(alias = "HS_RD_TO_CNT")]
pub type HsRdToCnt = crate::Reg<hs_rd_to_cnt::HsRdToCntSpec>;
#[doc = "Peripheral Response Timeout Definition after Hi"]
pub mod hs_rd_to_cnt;
#[doc = "LP_RD_TO_CNT (rw) register accessor: Peripheral Response Timeout Definition after Lo\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_rd_to_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_rd_to_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_rd_to_cnt`]
module"]
#[doc(alias = "LP_RD_TO_CNT")]
pub type LpRdToCnt = crate::Reg<lp_rd_to_cnt::LpRdToCntSpec>;
#[doc = "Peripheral Response Timeout Definition after Lo"]
pub mod lp_rd_to_cnt;
#[doc = "HS_WR_TO_CNT (rw) register accessor: Peripheral Response Timeout Definition after Hi\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hs_wr_to_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hs_wr_to_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hs_wr_to_cnt`]
module"]
#[doc(alias = "HS_WR_TO_CNT")]
pub type HsWrToCnt = crate::Reg<hs_wr_to_cnt::HsWrToCntSpec>;
#[doc = "Peripheral Response Timeout Definition after Hi"]
pub mod hs_wr_to_cnt;
#[doc = "LP_WR_TO_CNT (rw) register accessor: Peripheral Response Timeout Definition after Lo\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_wr_to_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_wr_to_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_wr_to_cnt`]
module"]
#[doc(alias = "LP_WR_TO_CNT")]
pub type LpWrToCnt = crate::Reg<lp_wr_to_cnt::LpWrToCntSpec>;
#[doc = "Peripheral Response Timeout Definition after Lo"]
pub mod lp_wr_to_cnt;
#[doc = "BTA_TO_CNT (rw) register accessor: Peripheral Response Timeout Definition after B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bta_to_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bta_to_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bta_to_cnt`]
module"]
#[doc(alias = "BTA_TO_CNT")]
pub type BtaToCnt = crate::Reg<bta_to_cnt::BtaToCntSpec>;
#[doc = "Peripheral Response Timeout Definition after B"]
pub mod bta_to_cnt;
#[doc = "LPCLK_CTRL (rw) register accessor: Low-power in Clock Lane Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpclk_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpclk_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpclk_ctrl`]
module"]
#[doc(alias = "LPCLK_CTRL")]
pub type LpclkCtrl = crate::Reg<lpclk_ctrl::LpclkCtrlSpec>;
#[doc = "Low-power in Clock Lane Register"]
pub mod lpclk_ctrl;
#[doc = "PHY_TMR_LPCLK_CFG (rw) register accessor: D-PHY Timing Configuration for the Clock Lane\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_tmr_lpclk_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_tmr_lpclk_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_tmr_lpclk_cfg`]
module"]
#[doc(alias = "PHY_TMR_LPCLK_CFG")]
pub type PhyTmrLpclkCfg = crate::Reg<phy_tmr_lpclk_cfg::PhyTmrLpclkCfgSpec>;
#[doc = "D-PHY Timing Configuration for the Clock Lane"]
pub mod phy_tmr_lpclk_cfg;
#[doc = "PHY_TMR_CFG (rw) register accessor: D-PHY Data Lanes Timing Configuration Registe\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_tmr_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_tmr_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_tmr_cfg`]
module"]
#[doc(alias = "PHY_TMR_CFG")]
pub type PhyTmrCfg = crate::Reg<phy_tmr_cfg::PhyTmrCfgSpec>;
#[doc = "D-PHY Data Lanes Timing Configuration Registe"]
pub mod phy_tmr_cfg;
#[doc = "PHY_RSTZ (rw) register accessor: D-PHY Reset Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_rstz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_rstz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_rstz`]
module"]
#[doc(alias = "PHY_RSTZ")]
pub type PhyRstz = crate::Reg<phy_rstz::PhyRstzSpec>;
#[doc = "D-PHY Reset Control Register"]
pub mod phy_rstz;
#[doc = "PHY_IF_CFG (rw) register accessor: D-PHY Interface Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_if_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_if_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_if_cfg`]
module"]
#[doc(alias = "PHY_IF_CFG")]
pub type PhyIfCfg = crate::Reg<phy_if_cfg::PhyIfCfgSpec>;
#[doc = "D-PHY Interface Configuration Register"]
pub mod phy_if_cfg;
#[doc = "PHY_ULPS_CTRL (rw) register accessor: D-PHY Ultra Low-Power Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_ulps_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_ulps_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_ulps_ctrl`]
module"]
#[doc(alias = "PHY_ULPS_CTRL")]
pub type PhyUlpsCtrl = crate::Reg<phy_ulps_ctrl::PhyUlpsCtrlSpec>;
#[doc = "D-PHY Ultra Low-Power Control Register"]
pub mod phy_ulps_ctrl;
#[doc = "PHY_TX_TRIGGERS (rw) register accessor: D-PHY Transmit Triggers Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_tx_triggers::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_tx_triggers::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_tx_triggers`]
module"]
#[doc(alias = "PHY_TX_TRIGGERS")]
pub type PhyTxTriggers = crate::Reg<phy_tx_triggers::PhyTxTriggersSpec>;
#[doc = "D-PHY Transmit Triggers Register"]
pub mod phy_tx_triggers;
#[doc = "PHY_STATUS (r) register accessor: Register0010 Abstract\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_status`]
module"]
#[doc(alias = "PHY_STATUS")]
pub type PhyStatus = crate::Reg<phy_status::PhyStatusSpec>;
#[doc = "Register0010 Abstract"]
pub mod phy_status;
#[doc = "PHY_TST_CTRL0 (rw) register accessor: D-PHY Test Interface Control 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_tst_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_tst_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_tst_ctrl0`]
module"]
#[doc(alias = "PHY_TST_CTRL0")]
pub type PhyTstCtrl0 = crate::Reg<phy_tst_ctrl0::PhyTstCtrl0Spec>;
#[doc = "D-PHY Test Interface Control 0 Register"]
pub mod phy_tst_ctrl0;
#[doc = "PHY_TST_CTRL1 (rw) register accessor: D-PHY Test Interface Control 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_tst_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_tst_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_tst_ctrl1`]
module"]
#[doc(alias = "PHY_TST_CTRL1")]
pub type PhyTstCtrl1 = crate::Reg<phy_tst_ctrl1::PhyTstCtrl1Spec>;
#[doc = "D-PHY Test Interface Control 1 Register"]
pub mod phy_tst_ctrl1;
#[doc = "INT_ST0 (r) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st0`]
module"]
#[doc(alias = "INT_ST0")]
pub type IntSt0 = crate::Reg<int_st0::IntSt0Spec>;
#[doc = "Interrupt Status Register 0"]
pub mod int_st0;
#[doc = "INT_ST1 (r) register accessor: Interrupt Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st1`]
module"]
#[doc(alias = "INT_ST1")]
pub type IntSt1 = crate::Reg<int_st1::IntSt1Spec>;
#[doc = "Interrupt Status Register 1"]
pub mod int_st1;
#[doc = "INT_MSK0 (r) register accessor: Masks the Interrupt Generation Triggered by the\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_msk0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_msk0`]
module"]
#[doc(alias = "INT_MSK0")]
pub type IntMsk0 = crate::Reg<int_msk0::IntMsk0Spec>;
#[doc = "Masks the Interrupt Generation Triggered by the"]
pub mod int_msk0;
#[doc = "INT_MSK1 (r) register accessor: Masks the Interrupt Generation Triggered by the\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_msk1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_msk1`]
module"]
#[doc(alias = "INT_MSK1")]
pub type IntMsk1 = crate::Reg<int_msk1::IntMsk1Spec>;
#[doc = "Masks the Interrupt Generation Triggered by the"]
pub mod int_msk1;
#[doc = "INT_FORCE0 (w) register accessor: Force Interrupt Configuration Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_force0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_force0`]
module"]
#[doc(alias = "INT_FORCE0")]
pub type IntForce0 = crate::Reg<int_force0::IntForce0Spec>;
#[doc = "Force Interrupt Configuration Register"]
pub mod int_force0;
#[doc = "INT_FORCE1 (w) register accessor: Force Interrupt Configuration Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_force1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_force1`]
module"]
#[doc(alias = "INT_FORCE1")]
pub type IntForce1 = crate::Reg<int_force1::IntForce1Spec>;
#[doc = "Force Interrupt Configuration Register"]
pub mod int_force1;
