#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    basic_strap_conf: BasicStrapConf,
    power_ctrl: PowerCtrl,
    power_status: PowerStatus,
    legacy_int_ctrl: LegacyIntCtrl,
    err_ctrl: ErrCtrl,
    err_cnt: ErrCnt,
    hot_reset_ctrl: HotResetCtrl,
    side_band_ctrl: SideBandCtrl,
    side_band_status: SideBandStatus,
    fc_level_rst_done: FcLevelRstDone,
    flr_status: FlrStatus,
    vf_status: VfStatus,
    vf_pwr_status: VfPwrStatus,
    vf_tph_status: VfTphStatus,
    tph_status: TphStatus,
    debug_out_0: DebugOut0,
    debug_out_1: DebugOut1,
    basic_status0: BasicStatus0,
    basic_status1: BasicStatus1,
    int_mask: IntMask,
    int_status: IntStatus,
    msg_ctrl: MsgCtrl,
    msg_status: MsgStatus,
    msg_code0: MsgCode0,
    msg_code1: MsgCode1,
    msg_data_len: MsgDataLen,
    _reserved26: [u8; 0x98],
    msg_fifo_rd_data: MsgFifoRdData,
}
impl RegisterBlock {
    #[doc = "0x00 - Basic strap configuration register"]
    #[inline(always)]
    pub const fn basic_strap_conf(&self) -> &BasicStrapConf {
        &self.basic_strap_conf
    }
    #[doc = "0x04 - PCIe client power control configuration"]
    #[inline(always)]
    pub const fn power_ctrl(&self) -> &PowerCtrl {
        &self.power_ctrl
    }
    #[doc = "0x08 - PCIe power management status"]
    #[inline(always)]
    pub const fn power_status(&self) -> &PowerStatus {
        &self.power_status
    }
    #[doc = "0x0c - Legacy interrupt control"]
    #[inline(always)]
    pub const fn legacy_int_ctrl(&self) -> &LegacyIntCtrl {
        &self.legacy_int_ctrl
    }
    #[doc = "0x10 - Error control register"]
    #[inline(always)]
    pub const fn err_ctrl(&self) -> &ErrCtrl {
        &self.err_ctrl
    }
    #[doc = "0x14 - Error counter"]
    #[inline(always)]
    pub const fn err_cnt(&self) -> &ErrCnt {
        &self.err_cnt
    }
    #[doc = "0x18 - Hot reset control"]
    #[inline(always)]
    pub const fn hot_reset_ctrl(&self) -> &HotResetCtrl {
        &self.hot_reset_ctrl
    }
    #[doc = "0x1c - Side band control configuration"]
    #[inline(always)]
    pub const fn side_band_ctrl(&self) -> &SideBandCtrl {
        &self.side_band_ctrl
    }
    #[doc = "0x20 - Side band status"]
    #[inline(always)]
    pub const fn side_band_status(&self) -> &SideBandStatus {
        &self.side_band_status
    }
    #[doc = "0x24 - Generate function level reset done pulse"]
    #[inline(always)]
    pub const fn fc_level_rst_done(&self) -> &FcLevelRstDone {
        &self.fc_level_rst_done
    }
    #[doc = "0x28 - Function level reset status"]
    #[inline(always)]
    pub const fn flr_status(&self) -> &FlrStatus {
        &self.flr_status
    }
    #[doc = "0x2c - Virtual function status"]
    #[inline(always)]
    pub const fn vf_status(&self) -> &VfStatus {
        &self.vf_status
    }
    #[doc = "0x30 - Virtual function power status"]
    #[inline(always)]
    pub const fn vf_pwr_status(&self) -> &VfPwrStatus {
        &self.vf_pwr_status
    }
    #[doc = "0x34 - Virtual function TPH status"]
    #[inline(always)]
    pub const fn vf_tph_status(&self) -> &VfTphStatus {
        &self.vf_tph_status
    }
    #[doc = "0x38 - Physical TPH status"]
    #[inline(always)]
    pub const fn tph_status(&self) -> &TphStatus {
        &self.tph_status
    }
    #[doc = "0x3c - Debug information 0"]
    #[inline(always)]
    pub const fn debug_out_0(&self) -> &DebugOut0 {
        &self.debug_out_0
    }
    #[doc = "0x40 - Debug information 1"]
    #[inline(always)]
    pub const fn debug_out_1(&self) -> &DebugOut1 {
        &self.debug_out_1
    }
    #[doc = "0x44 - Basic status 0"]
    #[inline(always)]
    pub const fn basic_status0(&self) -> &BasicStatus0 {
        &self.basic_status0
    }
    #[doc = "0x48 - Basic status 1"]
    #[inline(always)]
    pub const fn basic_status1(&self) -> &BasicStatus1 {
        &self.basic_status1
    }
    #[doc = "0x4c - Interrupt mask"]
    #[inline(always)]
    pub const fn int_mask(&self) -> &IntMask {
        &self.int_mask
    }
    #[doc = "0x50 - Interrupt status"]
    #[inline(always)]
    pub const fn int_status(&self) -> &IntStatus {
        &self.int_status
    }
    #[doc = "0x54 - Message receive control register"]
    #[inline(always)]
    pub const fn msg_ctrl(&self) -> &MsgCtrl {
        &self.msg_ctrl
    }
    #[doc = "0x58 - Message control status"]
    #[inline(always)]
    pub const fn msg_status(&self) -> &MsgStatus {
        &self.msg_status
    }
    #[doc = "0x5c - Message code 0"]
    #[inline(always)]
    pub const fn msg_code0(&self) -> &MsgCode0 {
        &self.msg_code0
    }
    #[doc = "0x60 - Message code 1"]
    #[inline(always)]
    pub const fn msg_code1(&self) -> &MsgCode1 {
        &self.msg_code1
    }
    #[doc = "0x64 - Message data length"]
    #[inline(always)]
    pub const fn msg_data_len(&self) -> &MsgDataLen {
        &self.msg_data_len
    }
    #[doc = "0x100 - Message fifo read data"]
    #[inline(always)]
    pub const fn msg_fifo_rd_data(&self) -> &MsgFifoRdData {
        &self.msg_fifo_rd_data
    }
}
#[doc = "BASIC_STRAP_CONF (rw) register accessor: Basic strap configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`basic_strap_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`basic_strap_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@basic_strap_conf`]
module"]
#[doc(alias = "BASIC_STRAP_CONF")]
pub type BasicStrapConf = crate::Reg<basic_strap_conf::BasicStrapConfSpec>;
#[doc = "Basic strap configuration register"]
pub mod basic_strap_conf;
#[doc = "POWER_CTRL (rw) register accessor: PCIe client power control configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_ctrl`]
module"]
#[doc(alias = "POWER_CTRL")]
pub type PowerCtrl = crate::Reg<power_ctrl::PowerCtrlSpec>;
#[doc = "PCIe client power control configuration"]
pub mod power_ctrl;
#[doc = "POWER_STATUS (r) register accessor: PCIe power management status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_status`]
module"]
#[doc(alias = "POWER_STATUS")]
pub type PowerStatus = crate::Reg<power_status::PowerStatusSpec>;
#[doc = "PCIe power management status"]
pub mod power_status;
#[doc = "LEGACY_INT_CTRL (rw) register accessor: Legacy interrupt control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`legacy_int_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`legacy_int_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@legacy_int_ctrl`]
module"]
#[doc(alias = "LEGACY_INT_CTRL")]
pub type LegacyIntCtrl = crate::Reg<legacy_int_ctrl::LegacyIntCtrlSpec>;
#[doc = "Legacy interrupt control"]
pub mod legacy_int_ctrl;
#[doc = "ERR_CTRL (rw) register accessor: Error control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_ctrl`]
module"]
#[doc(alias = "ERR_CTRL")]
pub type ErrCtrl = crate::Reg<err_ctrl::ErrCtrlSpec>;
#[doc = "Error control register"]
pub mod err_ctrl;
#[doc = "ERR_CNT (rw) register accessor: Error counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_cnt`]
module"]
#[doc(alias = "ERR_CNT")]
pub type ErrCnt = crate::Reg<err_cnt::ErrCntSpec>;
#[doc = "Error counter"]
pub mod err_cnt;
#[doc = "HOT_RESET_CTRL (rw) register accessor: Hot reset control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hot_reset_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hot_reset_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hot_reset_ctrl`]
module"]
#[doc(alias = "HOT_RESET_CTRL")]
pub type HotResetCtrl = crate::Reg<hot_reset_ctrl::HotResetCtrlSpec>;
#[doc = "Hot reset control"]
pub mod hot_reset_ctrl;
#[doc = "SIDE_BAND_CTRL (rw) register accessor: Side band control configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`side_band_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`side_band_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@side_band_ctrl`]
module"]
#[doc(alias = "SIDE_BAND_CTRL")]
pub type SideBandCtrl = crate::Reg<side_band_ctrl::SideBandCtrlSpec>;
#[doc = "Side band control configuration"]
pub mod side_band_ctrl;
#[doc = "SIDE_BAND_STATUS (r) register accessor: Side band status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`side_band_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@side_band_status`]
module"]
#[doc(alias = "SIDE_BAND_STATUS")]
pub type SideBandStatus = crate::Reg<side_band_status::SideBandStatusSpec>;
#[doc = "Side band status"]
pub mod side_band_status;
#[doc = "FC_LEVEL_RST_DONE (w) register accessor: Generate function level reset done pulse\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_level_rst_done::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_level_rst_done`]
module"]
#[doc(alias = "FC_LEVEL_RST_DONE")]
pub type FcLevelRstDone = crate::Reg<fc_level_rst_done::FcLevelRstDoneSpec>;
#[doc = "Generate function level reset done pulse"]
pub mod fc_level_rst_done;
#[doc = "FLR_STATUS (r) register accessor: Function level reset status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flr_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flr_status`]
module"]
#[doc(alias = "FLR_STATUS")]
pub type FlrStatus = crate::Reg<flr_status::FlrStatusSpec>;
#[doc = "Function level reset status"]
pub mod flr_status;
#[doc = "VF_STATUS (r) register accessor: Virtual function status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vf_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vf_status`]
module"]
#[doc(alias = "VF_STATUS")]
pub type VfStatus = crate::Reg<vf_status::VfStatusSpec>;
#[doc = "Virtual function status"]
pub mod vf_status;
#[doc = "VF_PWR_STATUS (r) register accessor: Virtual function power status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vf_pwr_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vf_pwr_status`]
module"]
#[doc(alias = "VF_PWR_STATUS")]
pub type VfPwrStatus = crate::Reg<vf_pwr_status::VfPwrStatusSpec>;
#[doc = "Virtual function power status"]
pub mod vf_pwr_status;
#[doc = "VF_TPH_STATUS (r) register accessor: Virtual function TPH status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vf_tph_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vf_tph_status`]
module"]
#[doc(alias = "VF_TPH_STATUS")]
pub type VfTphStatus = crate::Reg<vf_tph_status::VfTphStatusSpec>;
#[doc = "Virtual function TPH status"]
pub mod vf_tph_status;
#[doc = "TPH_STATUS (r) register accessor: Physical TPH status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tph_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tph_status`]
module"]
#[doc(alias = "TPH_STATUS")]
pub type TphStatus = crate::Reg<tph_status::TphStatusSpec>;
#[doc = "Physical TPH status"]
pub mod tph_status;
#[doc = "DEBUG_OUT_0 (r) register accessor: Debug information 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_out_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_out_0`]
module"]
#[doc(alias = "DEBUG_OUT_0")]
pub type DebugOut0 = crate::Reg<debug_out_0::DebugOut0Spec>;
#[doc = "Debug information 0"]
pub mod debug_out_0;
#[doc = "DEBUG_OUT_1 (r) register accessor: Debug information 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_out_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_out_1`]
module"]
#[doc(alias = "DEBUG_OUT_1")]
pub type DebugOut1 = crate::Reg<debug_out_1::DebugOut1Spec>;
#[doc = "Debug information 1"]
pub mod debug_out_1;
#[doc = "BASIC_STATUS0 (r) register accessor: Basic status 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`basic_status0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@basic_status0`]
module"]
#[doc(alias = "BASIC_STATUS0")]
pub type BasicStatus0 = crate::Reg<basic_status0::BasicStatus0Spec>;
#[doc = "Basic status 0"]
pub mod basic_status0;
#[doc = "BASIC_STATUS1 (r) register accessor: Basic status 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`basic_status1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@basic_status1`]
module"]
#[doc(alias = "BASIC_STATUS1")]
pub type BasicStatus1 = crate::Reg<basic_status1::BasicStatus1Spec>;
#[doc = "Basic status 1"]
pub mod basic_status1;
#[doc = "INT_MASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_mask`]
module"]
#[doc(alias = "INT_MASK")]
pub type IntMask = crate::Reg<int_mask::IntMaskSpec>;
#[doc = "Interrupt mask"]
pub mod int_mask;
#[doc = "INT_STATUS (rw) register accessor: Interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_status`]
module"]
#[doc(alias = "INT_STATUS")]
pub type IntStatus = crate::Reg<int_status::IntStatusSpec>;
#[doc = "Interrupt status"]
pub mod int_status;
#[doc = "MSG_CTRL (rw) register accessor: Message receive control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msg_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msg_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msg_ctrl`]
module"]
#[doc(alias = "MSG_CTRL")]
pub type MsgCtrl = crate::Reg<msg_ctrl::MsgCtrlSpec>;
#[doc = "Message receive control register"]
pub mod msg_ctrl;
#[doc = "MSG_STATUS (r) register accessor: Message control status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msg_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msg_status`]
module"]
#[doc(alias = "MSG_STATUS")]
pub type MsgStatus = crate::Reg<msg_status::MsgStatusSpec>;
#[doc = "Message control status"]
pub mod msg_status;
#[doc = "MSG_CODE0 (rw) register accessor: Message code 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msg_code0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msg_code0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msg_code0`]
module"]
#[doc(alias = "MSG_CODE0")]
pub type MsgCode0 = crate::Reg<msg_code0::MsgCode0Spec>;
#[doc = "Message code 0"]
pub mod msg_code0;
#[doc = "MSG_CODE1 (rw) register accessor: Message code 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msg_code1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msg_code1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msg_code1`]
module"]
#[doc(alias = "MSG_CODE1")]
pub type MsgCode1 = crate::Reg<msg_code1::MsgCode1Spec>;
#[doc = "Message code 1"]
pub mod msg_code1;
#[doc = "MSG_DATA_LEN (r) register accessor: Message data length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msg_data_len::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msg_data_len`]
module"]
#[doc(alias = "MSG_DATA_LEN")]
pub type MsgDataLen = crate::Reg<msg_data_len::MsgDataLenSpec>;
#[doc = "Message data length"]
pub mod msg_data_len;
#[doc = "MSG_FIFO_RD_DATA (r) register accessor: Message fifo read data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msg_fifo_rd_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msg_fifo_rd_data`]
module"]
#[doc(alias = "MSG_FIFO_RD_DATA")]
pub type MsgFifoRdData = crate::Reg<msg_fifo_rd_data::MsgFifoRdDataSpec>;
#[doc = "Message fifo read data"]
pub mod msg_fifo_rd_data;
