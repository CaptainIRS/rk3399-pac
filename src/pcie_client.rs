#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pcie_client_basic_strap_conf: PcieClientBasicStrapConf,
    pcie_client_power_ctrl: PcieClientPowerCtrl,
    pcie_client_power_status: PcieClientPowerStatus,
    pcie_client_legacy_int_ctrl: PcieClientLegacyIntCtrl,
    pcie_client_err_ctrl: PcieClientErrCtrl,
    pcie_client_err_cnt: PcieClientErrCnt,
    pcie_client_hot_reset_ctrl: PcieClientHotResetCtrl,
    pcie_client_side_band_ctrl: PcieClientSideBandCtrl,
    pcie_client_side_band_status: PcieClientSideBandStatus,
    pcie_client_fc_level_rst_done: PcieClientFcLevelRstDone,
    pcie_client_flr_status: PcieClientFlrStatus,
    pcie_client_vf_status: PcieClientVfStatus,
    pcie_client_vf_pwr_status: PcieClientVfPwrStatus,
    pcie_client_vf_tph_status: PcieClientVfTphStatus,
    pcie_client_tph_status: PcieClientTphStatus,
    pcie_client_debug_out_0: PcieClientDebugOut0,
    pcie_client_debug_out_1: PcieClientDebugOut1,
    pcie_client_basic_status0: PcieClientBasicStatus0,
    pcie_client_basic_status1: PcieClientBasicStatus1,
    pcie_client_int_mask: PcieClientIntMask,
    pcie_client_int_status: PcieClientIntStatus,
    pcie_client_msg_ctrl: PcieClientMsgCtrl,
    pcie_client_msg_status: PcieClientMsgStatus,
    pcie_client_msg_code0: PcieClientMsgCode0,
    pcie_client_msg_code1: PcieClientMsgCode1,
    pcie_client_msg_data_len: PcieClientMsgDataLen,
    _reserved26: [u8; 0x98],
    pcie_client_msg_fifo_rd_data: PcieClientMsgFifoRdData,
    _reserved27: [u8; 0xfc],
    pcie_client_conf_nu0: PcieClientConfNu0,
    pcie_client_conf_nu1: PcieClientConfNu1,
}
impl RegisterBlock {
    #[doc = "0x00 - Basic strap configuration register"]
    #[inline(always)]
    pub const fn pcie_client_basic_strap_conf(&self) -> &PcieClientBasicStrapConf {
        &self.pcie_client_basic_strap_conf
    }
    #[doc = "0x04 - PCIe client power control configuration"]
    #[inline(always)]
    pub const fn pcie_client_power_ctrl(&self) -> &PcieClientPowerCtrl {
        &self.pcie_client_power_ctrl
    }
    #[doc = "0x08 - PCIe power management status"]
    #[inline(always)]
    pub const fn pcie_client_power_status(&self) -> &PcieClientPowerStatus {
        &self.pcie_client_power_status
    }
    #[doc = "0x0c - Legacy interrupt control"]
    #[inline(always)]
    pub const fn pcie_client_legacy_int_ctrl(&self) -> &PcieClientLegacyIntCtrl {
        &self.pcie_client_legacy_int_ctrl
    }
    #[doc = "0x10 - Error control register"]
    #[inline(always)]
    pub const fn pcie_client_err_ctrl(&self) -> &PcieClientErrCtrl {
        &self.pcie_client_err_ctrl
    }
    #[doc = "0x14 - Error counter"]
    #[inline(always)]
    pub const fn pcie_client_err_cnt(&self) -> &PcieClientErrCnt {
        &self.pcie_client_err_cnt
    }
    #[doc = "0x18 - Hot reset control"]
    #[inline(always)]
    pub const fn pcie_client_hot_reset_ctrl(&self) -> &PcieClientHotResetCtrl {
        &self.pcie_client_hot_reset_ctrl
    }
    #[doc = "0x1c - Side band control configuration"]
    #[inline(always)]
    pub const fn pcie_client_side_band_ctrl(&self) -> &PcieClientSideBandCtrl {
        &self.pcie_client_side_band_ctrl
    }
    #[doc = "0x20 - Side band status"]
    #[inline(always)]
    pub const fn pcie_client_side_band_status(&self) -> &PcieClientSideBandStatus {
        &self.pcie_client_side_band_status
    }
    #[doc = "0x24 - Generate function level reset done pulse"]
    #[inline(always)]
    pub const fn pcie_client_fc_level_rst_done(&self) -> &PcieClientFcLevelRstDone {
        &self.pcie_client_fc_level_rst_done
    }
    #[doc = "0x28 - Function level reset status"]
    #[inline(always)]
    pub const fn pcie_client_flr_status(&self) -> &PcieClientFlrStatus {
        &self.pcie_client_flr_status
    }
    #[doc = "0x2c - Virtual function status"]
    #[inline(always)]
    pub const fn pcie_client_vf_status(&self) -> &PcieClientVfStatus {
        &self.pcie_client_vf_status
    }
    #[doc = "0x30 - Virtual function power status"]
    #[inline(always)]
    pub const fn pcie_client_vf_pwr_status(&self) -> &PcieClientVfPwrStatus {
        &self.pcie_client_vf_pwr_status
    }
    #[doc = "0x34 - Virtual function TPH status"]
    #[inline(always)]
    pub const fn pcie_client_vf_tph_status(&self) -> &PcieClientVfTphStatus {
        &self.pcie_client_vf_tph_status
    }
    #[doc = "0x38 - Physical TPH status"]
    #[inline(always)]
    pub const fn pcie_client_tph_status(&self) -> &PcieClientTphStatus {
        &self.pcie_client_tph_status
    }
    #[doc = "0x3c - Debug information 0"]
    #[inline(always)]
    pub const fn pcie_client_debug_out_0(&self) -> &PcieClientDebugOut0 {
        &self.pcie_client_debug_out_0
    }
    #[doc = "0x40 - Debug information 1"]
    #[inline(always)]
    pub const fn pcie_client_debug_out_1(&self) -> &PcieClientDebugOut1 {
        &self.pcie_client_debug_out_1
    }
    #[doc = "0x44 - Basic status 0"]
    #[inline(always)]
    pub const fn pcie_client_basic_status0(&self) -> &PcieClientBasicStatus0 {
        &self.pcie_client_basic_status0
    }
    #[doc = "0x48 - Basic status 1"]
    #[inline(always)]
    pub const fn pcie_client_basic_status1(&self) -> &PcieClientBasicStatus1 {
        &self.pcie_client_basic_status1
    }
    #[doc = "0x4c - Interrupt mask"]
    #[inline(always)]
    pub const fn pcie_client_int_mask(&self) -> &PcieClientIntMask {
        &self.pcie_client_int_mask
    }
    #[doc = "0x50 - Interrupt status"]
    #[inline(always)]
    pub const fn pcie_client_int_status(&self) -> &PcieClientIntStatus {
        &self.pcie_client_int_status
    }
    #[doc = "0x54 - Message receive control register"]
    #[inline(always)]
    pub const fn pcie_client_msg_ctrl(&self) -> &PcieClientMsgCtrl {
        &self.pcie_client_msg_ctrl
    }
    #[doc = "0x58 - Message control status"]
    #[inline(always)]
    pub const fn pcie_client_msg_status(&self) -> &PcieClientMsgStatus {
        &self.pcie_client_msg_status
    }
    #[doc = "0x5c - Message code 0"]
    #[inline(always)]
    pub const fn pcie_client_msg_code0(&self) -> &PcieClientMsgCode0 {
        &self.pcie_client_msg_code0
    }
    #[doc = "0x60 - Message code 1"]
    #[inline(always)]
    pub const fn pcie_client_msg_code1(&self) -> &PcieClientMsgCode1 {
        &self.pcie_client_msg_code1
    }
    #[doc = "0x64 - Message data length"]
    #[inline(always)]
    pub const fn pcie_client_msg_data_len(&self) -> &PcieClientMsgDataLen {
        &self.pcie_client_msg_data_len
    }
    #[doc = "0x100 - Message fifo read data"]
    #[inline(always)]
    pub const fn pcie_client_msg_fifo_rd_data(&self) -> &PcieClientMsgFifoRdData {
        &self.pcie_client_msg_fifo_rd_data
    }
    #[doc = "0x200 - Configuration no used"]
    #[inline(always)]
    pub const fn pcie_client_conf_nu0(&self) -> &PcieClientConfNu0 {
        &self.pcie_client_conf_nu0
    }
    #[doc = "0x204 - Configuration no used"]
    #[inline(always)]
    pub const fn pcie_client_conf_nu1(&self) -> &PcieClientConfNu1 {
        &self.pcie_client_conf_nu1
    }
}
#[doc = "PCIE_CLIENT_BASIC_STRAP_CONF (rw) register accessor: Basic strap configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_basic_strap_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_client_basic_strap_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_client_basic_strap_conf`]
module"]
#[doc(alias = "PCIE_CLIENT_BASIC_STRAP_CONF")]
pub type PcieClientBasicStrapConf =
    crate::Reg<pcie_client_basic_strap_conf::PcieClientBasicStrapConfSpec>;
#[doc = "Basic strap configuration register"]
pub mod pcie_client_basic_strap_conf;
#[doc = "PCIE_CLIENT_POWER_CTRL (rw) register accessor: PCIe client power control configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_power_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_client_power_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_client_power_ctrl`]
module"]
#[doc(alias = "PCIE_CLIENT_POWER_CTRL")]
pub type PcieClientPowerCtrl = crate::Reg<pcie_client_power_ctrl::PcieClientPowerCtrlSpec>;
#[doc = "PCIe client power control configuration"]
pub mod pcie_client_power_ctrl;
#[doc = "PCIE_CLIENT_POWER_STATUS (r) register accessor: PCIe power management status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_power_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_client_power_status`]
module"]
#[doc(alias = "PCIE_CLIENT_POWER_STATUS")]
pub type PcieClientPowerStatus = crate::Reg<pcie_client_power_status::PcieClientPowerStatusSpec>;
#[doc = "PCIe power management status"]
pub mod pcie_client_power_status;
#[doc = "PCIE_CLIENT_LEGACY_INT_CTRL (rw) register accessor: Legacy interrupt control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_legacy_int_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_client_legacy_int_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_client_legacy_int_ctrl`]
module"]
#[doc(alias = "PCIE_CLIENT_LEGACY_INT_CTRL")]
pub type PcieClientLegacyIntCtrl =
    crate::Reg<pcie_client_legacy_int_ctrl::PcieClientLegacyIntCtrlSpec>;
#[doc = "Legacy interrupt control"]
pub mod pcie_client_legacy_int_ctrl;
#[doc = "PCIE_CLIENT_ERR_CTRL (rw) register accessor: Error control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_err_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_client_err_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_client_err_ctrl`]
module"]
#[doc(alias = "PCIE_CLIENT_ERR_CTRL")]
pub type PcieClientErrCtrl = crate::Reg<pcie_client_err_ctrl::PcieClientErrCtrlSpec>;
#[doc = "Error control register"]
pub mod pcie_client_err_ctrl;
#[doc = "PCIE_CLIENT_ERR_CNT (rw) register accessor: Error counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_err_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_client_err_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_client_err_cnt`]
module"]
#[doc(alias = "PCIE_CLIENT_ERR_CNT")]
pub type PcieClientErrCnt = crate::Reg<pcie_client_err_cnt::PcieClientErrCntSpec>;
#[doc = "Error counter"]
pub mod pcie_client_err_cnt;
#[doc = "PCIE_CLIENT_HOT_RESET_CTRL (rw) register accessor: Hot reset control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_hot_reset_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_client_hot_reset_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_client_hot_reset_ctrl`]
module"]
#[doc(alias = "PCIE_CLIENT_HOT_RESET_CTRL")]
pub type PcieClientHotResetCtrl =
    crate::Reg<pcie_client_hot_reset_ctrl::PcieClientHotResetCtrlSpec>;
#[doc = "Hot reset control"]
pub mod pcie_client_hot_reset_ctrl;
#[doc = "PCIE_CLIENT_SIDE_BAND_CTRL (rw) register accessor: Side band control configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_side_band_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_client_side_band_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_client_side_band_ctrl`]
module"]
#[doc(alias = "PCIE_CLIENT_SIDE_BAND_CTRL")]
pub type PcieClientSideBandCtrl =
    crate::Reg<pcie_client_side_band_ctrl::PcieClientSideBandCtrlSpec>;
#[doc = "Side band control configuration"]
pub mod pcie_client_side_band_ctrl;
#[doc = "PCIE_CLIENT_SIDE_BAND_STATUS (r) register accessor: Side band status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_side_band_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_client_side_band_status`]
module"]
#[doc(alias = "PCIE_CLIENT_SIDE_BAND_STATUS")]
pub type PcieClientSideBandStatus =
    crate::Reg<pcie_client_side_band_status::PcieClientSideBandStatusSpec>;
#[doc = "Side band status"]
pub mod pcie_client_side_band_status;
#[doc = "PCIE_CLIENT_FC_LEVEL_RST_DONE (w) register accessor: Generate function level reset done pulse\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_client_fc_level_rst_done::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_client_fc_level_rst_done`]
module"]
#[doc(alias = "PCIE_CLIENT_FC_LEVEL_RST_DONE")]
pub type PcieClientFcLevelRstDone =
    crate::Reg<pcie_client_fc_level_rst_done::PcieClientFcLevelRstDoneSpec>;
#[doc = "Generate function level reset done pulse"]
pub mod pcie_client_fc_level_rst_done;
#[doc = "PCIE_CLIENT_FLR_STATUS (r) register accessor: Function level reset status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_flr_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_client_flr_status`]
module"]
#[doc(alias = "PCIE_CLIENT_FLR_STATUS")]
pub type PcieClientFlrStatus = crate::Reg<pcie_client_flr_status::PcieClientFlrStatusSpec>;
#[doc = "Function level reset status"]
pub mod pcie_client_flr_status;
#[doc = "PCIE_CLIENT_VF_STATUS (r) register accessor: Virtual function status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_vf_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_client_vf_status`]
module"]
#[doc(alias = "PCIE_CLIENT_VF_STATUS")]
pub type PcieClientVfStatus = crate::Reg<pcie_client_vf_status::PcieClientVfStatusSpec>;
#[doc = "Virtual function status"]
pub mod pcie_client_vf_status;
#[doc = "PCIE_CLIENT_VF_PWR_STATUS (r) register accessor: Virtual function power status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_vf_pwr_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_client_vf_pwr_status`]
module"]
#[doc(alias = "PCIE_CLIENT_VF_PWR_STATUS")]
pub type PcieClientVfPwrStatus = crate::Reg<pcie_client_vf_pwr_status::PcieClientVfPwrStatusSpec>;
#[doc = "Virtual function power status"]
pub mod pcie_client_vf_pwr_status;
#[doc = "PCIE_CLIENT_VF_TPH_STATUS (r) register accessor: Virtual function TPH status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_vf_tph_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_client_vf_tph_status`]
module"]
#[doc(alias = "PCIE_CLIENT_VF_TPH_STATUS")]
pub type PcieClientVfTphStatus = crate::Reg<pcie_client_vf_tph_status::PcieClientVfTphStatusSpec>;
#[doc = "Virtual function TPH status"]
pub mod pcie_client_vf_tph_status;
#[doc = "PCIE_CLIENT_TPH_STATUS (r) register accessor: Physical TPH status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_tph_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_client_tph_status`]
module"]
#[doc(alias = "PCIE_CLIENT_TPH_STATUS")]
pub type PcieClientTphStatus = crate::Reg<pcie_client_tph_status::PcieClientTphStatusSpec>;
#[doc = "Physical TPH status"]
pub mod pcie_client_tph_status;
#[doc = "PCIE_CLIENT_DEBUG_OUT_0 (r) register accessor: Debug information 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_debug_out_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_client_debug_out_0`]
module"]
#[doc(alias = "PCIE_CLIENT_DEBUG_OUT_0")]
pub type PcieClientDebugOut0 = crate::Reg<pcie_client_debug_out_0::PcieClientDebugOut0Spec>;
#[doc = "Debug information 0"]
pub mod pcie_client_debug_out_0;
#[doc = "PCIE_CLIENT_DEBUG_OUT_1 (r) register accessor: Debug information 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_debug_out_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_client_debug_out_1`]
module"]
#[doc(alias = "PCIE_CLIENT_DEBUG_OUT_1")]
pub type PcieClientDebugOut1 = crate::Reg<pcie_client_debug_out_1::PcieClientDebugOut1Spec>;
#[doc = "Debug information 1"]
pub mod pcie_client_debug_out_1;
#[doc = "PCIE_CLIENT_BASIC_STATUS0 (r) register accessor: Basic status 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_basic_status0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_client_basic_status0`]
module"]
#[doc(alias = "PCIE_CLIENT_BASIC_STATUS0")]
pub type PcieClientBasicStatus0 = crate::Reg<pcie_client_basic_status0::PcieClientBasicStatus0Spec>;
#[doc = "Basic status 0"]
pub mod pcie_client_basic_status0;
#[doc = "PCIE_CLIENT_BASIC_STATUS1 (r) register accessor: Basic status 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_basic_status1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_client_basic_status1`]
module"]
#[doc(alias = "PCIE_CLIENT_BASIC_STATUS1")]
pub type PcieClientBasicStatus1 = crate::Reg<pcie_client_basic_status1::PcieClientBasicStatus1Spec>;
#[doc = "Basic status 1"]
pub mod pcie_client_basic_status1;
#[doc = "PCIE_CLIENT_INT_MASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_int_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_client_int_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_client_int_mask`]
module"]
#[doc(alias = "PCIE_CLIENT_INT_MASK")]
pub type PcieClientIntMask = crate::Reg<pcie_client_int_mask::PcieClientIntMaskSpec>;
#[doc = "Interrupt mask"]
pub mod pcie_client_int_mask;
#[doc = "PCIE_CLIENT_INT_STATUS (rw) register accessor: Interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_int_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_client_int_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_client_int_status`]
module"]
#[doc(alias = "PCIE_CLIENT_INT_STATUS")]
pub type PcieClientIntStatus = crate::Reg<pcie_client_int_status::PcieClientIntStatusSpec>;
#[doc = "Interrupt status"]
pub mod pcie_client_int_status;
#[doc = "PCIE_CLIENT_MSG_CTRL (rw) register accessor: Message receive control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_msg_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_client_msg_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_client_msg_ctrl`]
module"]
#[doc(alias = "PCIE_CLIENT_MSG_CTRL")]
pub type PcieClientMsgCtrl = crate::Reg<pcie_client_msg_ctrl::PcieClientMsgCtrlSpec>;
#[doc = "Message receive control register"]
pub mod pcie_client_msg_ctrl;
#[doc = "PCIE_CLIENT_MSG_STATUS (r) register accessor: Message control status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_msg_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_client_msg_status`]
module"]
#[doc(alias = "PCIE_CLIENT_MSG_STATUS")]
pub type PcieClientMsgStatus = crate::Reg<pcie_client_msg_status::PcieClientMsgStatusSpec>;
#[doc = "Message control status"]
pub mod pcie_client_msg_status;
#[doc = "PCIE_CLIENT_MSG_CODE0 (rw) register accessor: Message code 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_msg_code0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_client_msg_code0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_client_msg_code0`]
module"]
#[doc(alias = "PCIE_CLIENT_MSG_CODE0")]
pub type PcieClientMsgCode0 = crate::Reg<pcie_client_msg_code0::PcieClientMsgCode0Spec>;
#[doc = "Message code 0"]
pub mod pcie_client_msg_code0;
#[doc = "PCIE_CLIENT_MSG_CODE1 (rw) register accessor: Message code 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_msg_code1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_client_msg_code1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_client_msg_code1`]
module"]
#[doc(alias = "PCIE_CLIENT_MSG_CODE1")]
pub type PcieClientMsgCode1 = crate::Reg<pcie_client_msg_code1::PcieClientMsgCode1Spec>;
#[doc = "Message code 1"]
pub mod pcie_client_msg_code1;
#[doc = "PCIE_CLIENT_MSG_DATA_LEN (r) register accessor: Message data length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_msg_data_len::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_client_msg_data_len`]
module"]
#[doc(alias = "PCIE_CLIENT_MSG_DATA_LEN")]
pub type PcieClientMsgDataLen = crate::Reg<pcie_client_msg_data_len::PcieClientMsgDataLenSpec>;
#[doc = "Message data length"]
pub mod pcie_client_msg_data_len;
#[doc = "PCIE_CLIENT_MSG_FIFO_RD_DATA (r) register accessor: Message fifo read data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_msg_fifo_rd_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_client_msg_fifo_rd_data`]
module"]
#[doc(alias = "PCIE_CLIENT_MSG_FIFO_RD_DATA")]
pub type PcieClientMsgFifoRdData =
    crate::Reg<pcie_client_msg_fifo_rd_data::PcieClientMsgFifoRdDataSpec>;
#[doc = "Message fifo read data"]
pub mod pcie_client_msg_fifo_rd_data;
#[doc = "PCIE_CLIENT_CONF_NU0 (r) register accessor: Configuration no used\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_conf_nu0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_client_conf_nu0`]
module"]
#[doc(alias = "PCIE_CLIENT_CONF_NU0")]
pub type PcieClientConfNu0 = crate::Reg<pcie_client_conf_nu0::PcieClientConfNu0Spec>;
#[doc = "Configuration no used"]
pub mod pcie_client_conf_nu0;
#[doc = "PCIE_CLIENT_CONF_NU1 (r) register accessor: Configuration no used\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_conf_nu1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_client_conf_nu1`]
module"]
#[doc(alias = "PCIE_CLIENT_CONF_NU1")]
pub type PcieClientConfNu1 = crate::Reg<pcie_client_conf_nu1::PcieClientConfNu1Spec>;
#[doc = "Configuration no used"]
pub mod pcie_client_conf_nu1;
