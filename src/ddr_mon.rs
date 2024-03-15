#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ddrmon_ip_version: DdrmonIpVersion,
    ddrmon_ctrl: DdrmonCtrl,
    ddrmon_int_status: DdrmonIntStatus,
    ddrmon_int_mask: DdrmonIntMask,
    ddrmon_timer_count: DdrmonTimerCount,
    ddrmon_floor_number: DdrmonFloorNumber,
    ddrmon_top_number: DdrmonTopNumber,
    ddrmon_ch0_dfi_act_num: DdrmonCh0DfiActNum,
    ddrmon_ch0_dfi_wr_num: DdrmonCh0DfiWrNum,
    ddrmon_ch0_dfi_rd_num: DdrmonCh0DfiRdNum,
    ddrmon_ch0_count_num: DdrmonCh0CountNum,
    ddrmon_ch0_dfi_access_num: DdrmonCh0DfiAccessNum,
    ddrmon_ch1_dfi_act_num: DdrmonCh1DfiActNum,
    ddrmon_ch1_dfi_wr_num: DdrmonCh1DfiWrNum,
    ddrmon_ch1_dfi_rd_num: DdrmonCh1DfiRdNum,
    ddrmon_ch1_count_num: DdrmonCh1CountNum,
    ddrmon_ch1_dfi_access_num: DdrmonCh1DfiAccessNum,
    _reserved17: [u8; 0x01bc],
    ddrmon_ddr_if_ctrl: DdrmonDdrIfCtrl,
    _reserved18: [u8; 0x08],
    ddrmon_ch0_wr_start_addr: DdrmonCh0WrStartAddr,
    ddrmon_ch0_wr_end_addr: DdrmonCh0WrEndAddr,
    ddrmon_ch0_rd_start_addr: DdrmonCh0RdStartAddr,
    ddrmon_ch0_rd_end_addr: DdrmonCh0RdEndAddr,
    _reserved22: [u8; 0x08],
    ddrmon_ch1_wr_start_addr: DdrmonCh1WrStartAddr,
    ddrmon_ch1_wr_end_addr: DdrmonCh1WrEndAddr,
    ddrmon_ch1_rd_start_addr: DdrmonCh1RdStartAddr,
    ddrmon_ch1_rd_end_addr: DdrmonCh1RdEndAddr,
    _reserved26: [u8; 0x0c],
    ddrmon_ch0_ddr_fifo0_addr: DdrmonCh0DdrFifo0Addr,
    _reserved27: [u8; 0x04],
    ddrmon_ch0_ddr_fifo1_addr: DdrmonCh0DdrFifo1Addr,
    _reserved28: [u8; 0x04],
    ddrmon_ch0_ddr_fifo2_addr: DdrmonCh0DdrFifo2Addr,
    _reserved29: [u8; 0x04],
    ddrmon_ch0_ddr_fifo3_addr: DdrmonCh0DdrFifo3Addr,
    _reserved30: [u8; 0x04],
    ddrmon_ch1_ddr_fifo0_addr: DdrmonCh1DdrFifo0Addr,
    _reserved31: [u8; 0x04],
    ddrmon_ch1_ddr_fifo1_addr: DdrmonCh1DdrFifo1Addr,
    _reserved32: [u8; 0x04],
    ddrmon_ch1_ddr_fifo2_addr: DdrmonCh1DdrFifo2Addr,
    _reserved33: [u8; 0x04],
    ddrmon_ch1_ddr_fifo3_addr: DdrmonCh1DdrFifo3Addr,
}
impl RegisterBlock {
    #[doc = "0x00 - DDR Monitor IP Version"]
    #[inline(always)]
    pub const fn ddrmon_ip_version(&self) -> &DdrmonIpVersion {
        &self.ddrmon_ip_version
    }
    #[doc = "0x04 - DDR Monitor Control Register"]
    #[inline(always)]
    pub const fn ddrmon_ctrl(&self) -> &DdrmonCtrl {
        &self.ddrmon_ctrl
    }
    #[doc = "0x08 - Interrupt Status"]
    #[inline(always)]
    pub const fn ddrmon_int_status(&self) -> &DdrmonIntStatus {
        &self.ddrmon_int_status
    }
    #[doc = "0x0c - Interrupt mask control"]
    #[inline(always)]
    pub const fn ddrmon_int_mask(&self) -> &DdrmonIntMask {
        &self.ddrmon_int_mask
    }
    #[doc = "0x10 - The DFI Timer Threshold"]
    #[inline(always)]
    pub const fn ddrmon_timer_count(&self) -> &DdrmonTimerCount {
        &self.ddrmon_timer_count
    }
    #[doc = "0x14 - The Low Threshold in the Comparison of DDR Access"]
    #[inline(always)]
    pub const fn ddrmon_floor_number(&self) -> &DdrmonFloorNumber {
        &self.ddrmon_floor_number
    }
    #[doc = "0x18 - The High Threshold in the Comparison of DDR Access"]
    #[inline(always)]
    pub const fn ddrmon_top_number(&self) -> &DdrmonTopNumber {
        &self.ddrmon_top_number
    }
    #[doc = "0x1c - Channel 0 DFI Active Command Number"]
    #[inline(always)]
    pub const fn ddrmon_ch0_dfi_act_num(&self) -> &DdrmonCh0DfiActNum {
        &self.ddrmon_ch0_dfi_act_num
    }
    #[doc = "0x20 - Channel 0 DFI write Command Number"]
    #[inline(always)]
    pub const fn ddrmon_ch0_dfi_wr_num(&self) -> &DdrmonCh0DfiWrNum {
        &self.ddrmon_ch0_dfi_wr_num
    }
    #[doc = "0x24 - Channel 0 DFI read Command Number"]
    #[inline(always)]
    pub const fn ddrmon_ch0_dfi_rd_num(&self) -> &DdrmonCh0DfiRdNum {
        &self.ddrmon_ch0_dfi_rd_num
    }
    #[doc = "0x28 - Channel 0 Timer Count Number"]
    #[inline(always)]
    pub const fn ddrmon_ch0_count_num(&self) -> &DdrmonCh0CountNum {
        &self.ddrmon_ch0_count_num
    }
    #[doc = "0x2c - Channel 0 DFI Read and Write Command Number"]
    #[inline(always)]
    pub const fn ddrmon_ch0_dfi_access_num(&self) -> &DdrmonCh0DfiAccessNum {
        &self.ddrmon_ch0_dfi_access_num
    }
    #[doc = "0x30 - Channel 1 DFI Active Command Number"]
    #[inline(always)]
    pub const fn ddrmon_ch1_dfi_act_num(&self) -> &DdrmonCh1DfiActNum {
        &self.ddrmon_ch1_dfi_act_num
    }
    #[doc = "0x34 - Channel 1 DFI write Command Number"]
    #[inline(always)]
    pub const fn ddrmon_ch1_dfi_wr_num(&self) -> &DdrmonCh1DfiWrNum {
        &self.ddrmon_ch1_dfi_wr_num
    }
    #[doc = "0x38 - Channel 1 DFI read Command Number"]
    #[inline(always)]
    pub const fn ddrmon_ch1_dfi_rd_num(&self) -> &DdrmonCh1DfiRdNum {
        &self.ddrmon_ch1_dfi_rd_num
    }
    #[doc = "0x3c - Channel 1 Timer Count Number"]
    #[inline(always)]
    pub const fn ddrmon_ch1_count_num(&self) -> &DdrmonCh1CountNum {
        &self.ddrmon_ch1_count_num
    }
    #[doc = "0x40 - Channel 1 DFI Read and Write Command Number"]
    #[inline(always)]
    pub const fn ddrmon_ch1_dfi_access_num(&self) -> &DdrmonCh1DfiAccessNum {
        &self.ddrmon_ch1_dfi_access_num
    }
    #[doc = "0x200 - DDR interface Control Register"]
    #[inline(always)]
    pub const fn ddrmon_ddr_if_ctrl(&self) -> &DdrmonDdrIfCtrl {
        &self.ddrmon_ddr_if_ctrl
    }
    #[doc = "0x20c - Channel 0 Write Start Address"]
    #[inline(always)]
    pub const fn ddrmon_ch0_wr_start_addr(&self) -> &DdrmonCh0WrStartAddr {
        &self.ddrmon_ch0_wr_start_addr
    }
    #[doc = "0x210 - Channel 0 Write End Address"]
    #[inline(always)]
    pub const fn ddrmon_ch0_wr_end_addr(&self) -> &DdrmonCh0WrEndAddr {
        &self.ddrmon_ch0_wr_end_addr
    }
    #[doc = "0x214 - Channel 0 Read Start Address"]
    #[inline(always)]
    pub const fn ddrmon_ch0_rd_start_addr(&self) -> &DdrmonCh0RdStartAddr {
        &self.ddrmon_ch0_rd_start_addr
    }
    #[doc = "0x218 - Channel 0 Read End Address"]
    #[inline(always)]
    pub const fn ddrmon_ch0_rd_end_addr(&self) -> &DdrmonCh0RdEndAddr {
        &self.ddrmon_ch0_rd_end_addr
    }
    #[doc = "0x224 - Channel 1 Write Start Address"]
    #[inline(always)]
    pub const fn ddrmon_ch1_wr_start_addr(&self) -> &DdrmonCh1WrStartAddr {
        &self.ddrmon_ch1_wr_start_addr
    }
    #[doc = "0x228 - Channel 1 Write End Address"]
    #[inline(always)]
    pub const fn ddrmon_ch1_wr_end_addr(&self) -> &DdrmonCh1WrEndAddr {
        &self.ddrmon_ch1_wr_end_addr
    }
    #[doc = "0x22c - Channel 1 Read Start Address"]
    #[inline(always)]
    pub const fn ddrmon_ch1_rd_start_addr(&self) -> &DdrmonCh1RdStartAddr {
        &self.ddrmon_ch1_rd_start_addr
    }
    #[doc = "0x230 - Channel 1 Read End Address"]
    #[inline(always)]
    pub const fn ddrmon_ch1_rd_end_addr(&self) -> &DdrmonCh1RdEndAddr {
        &self.ddrmon_ch1_rd_end_addr
    }
    #[doc = "0x240 - DDR Channel 0 Controller Interface Address FIFO0"]
    #[inline(always)]
    pub const fn ddrmon_ch0_ddr_fifo0_addr(&self) -> &DdrmonCh0DdrFifo0Addr {
        &self.ddrmon_ch0_ddr_fifo0_addr
    }
    #[doc = "0x248 - DDR Channel 0 Controller Interface Address FIFO1"]
    #[inline(always)]
    pub const fn ddrmon_ch0_ddr_fifo1_addr(&self) -> &DdrmonCh0DdrFifo1Addr {
        &self.ddrmon_ch0_ddr_fifo1_addr
    }
    #[doc = "0x250 - DDR Channel 0 Controller Interface Address FIFO2"]
    #[inline(always)]
    pub const fn ddrmon_ch0_ddr_fifo2_addr(&self) -> &DdrmonCh0DdrFifo2Addr {
        &self.ddrmon_ch0_ddr_fifo2_addr
    }
    #[doc = "0x258 - DDR Channel 0 Controller Interface Address FIFO3"]
    #[inline(always)]
    pub const fn ddrmon_ch0_ddr_fifo3_addr(&self) -> &DdrmonCh0DdrFifo3Addr {
        &self.ddrmon_ch0_ddr_fifo3_addr
    }
    #[doc = "0x260 - DDR Channel 1 Controller Interface Address FIFO0"]
    #[inline(always)]
    pub const fn ddrmon_ch1_ddr_fifo0_addr(&self) -> &DdrmonCh1DdrFifo0Addr {
        &self.ddrmon_ch1_ddr_fifo0_addr
    }
    #[doc = "0x268 - DDR Channel 1 Controller Interface Address FIFO1"]
    #[inline(always)]
    pub const fn ddrmon_ch1_ddr_fifo1_addr(&self) -> &DdrmonCh1DdrFifo1Addr {
        &self.ddrmon_ch1_ddr_fifo1_addr
    }
    #[doc = "0x270 - DDR Channel 1 Controller Interface Address FIFO2"]
    #[inline(always)]
    pub const fn ddrmon_ch1_ddr_fifo2_addr(&self) -> &DdrmonCh1DdrFifo2Addr {
        &self.ddrmon_ch1_ddr_fifo2_addr
    }
    #[doc = "0x278 - DDR Channel 1 Controller Interface Address FIFO3"]
    #[inline(always)]
    pub const fn ddrmon_ch1_ddr_fifo3_addr(&self) -> &DdrmonCh1DdrFifo3Addr {
        &self.ddrmon_ch1_ddr_fifo3_addr
    }
}
#[doc = "DDRMON_IP_VERSION (r) register accessor: DDR Monitor IP Version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ip_version::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_ip_version`]
module"]
#[doc(alias = "DDRMON_IP_VERSION")]
pub type DdrmonIpVersion = crate::Reg<ddrmon_ip_version::DdrmonIpVersionSpec>;
#[doc = "DDR Monitor IP Version"]
pub mod ddrmon_ip_version;
#[doc = "DDRMON_CTRL (rw) register accessor: DDR Monitor Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrmon_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_ctrl`]
module"]
#[doc(alias = "DDRMON_CTRL")]
pub type DdrmonCtrl = crate::Reg<ddrmon_ctrl::DdrmonCtrlSpec>;
#[doc = "DDR Monitor Control Register"]
pub mod ddrmon_ctrl;
#[doc = "DDRMON_INT_STATUS (r) register accessor: Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_int_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_int_status`]
module"]
#[doc(alias = "DDRMON_INT_STATUS")]
pub type DdrmonIntStatus = crate::Reg<ddrmon_int_status::DdrmonIntStatusSpec>;
#[doc = "Interrupt Status"]
pub mod ddrmon_int_status;
#[doc = "DDRMON_INT_MASK (rw) register accessor: Interrupt mask control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_int_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrmon_int_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_int_mask`]
module"]
#[doc(alias = "DDRMON_INT_MASK")]
pub type DdrmonIntMask = crate::Reg<ddrmon_int_mask::DdrmonIntMaskSpec>;
#[doc = "Interrupt mask control"]
pub mod ddrmon_int_mask;
#[doc = "DDRMON_TIMER_COUNT (r) register accessor: The DFI Timer Threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_timer_count::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_timer_count`]
module"]
#[doc(alias = "DDRMON_TIMER_COUNT")]
pub type DdrmonTimerCount = crate::Reg<ddrmon_timer_count::DdrmonTimerCountSpec>;
#[doc = "The DFI Timer Threshold"]
pub mod ddrmon_timer_count;
#[doc = "DDRMON_FLOOR_NUMBER (r) register accessor: The Low Threshold in the Comparison of DDR Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_floor_number::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_floor_number`]
module"]
#[doc(alias = "DDRMON_FLOOR_NUMBER")]
pub type DdrmonFloorNumber = crate::Reg<ddrmon_floor_number::DdrmonFloorNumberSpec>;
#[doc = "The Low Threshold in the Comparison of DDR Access"]
pub mod ddrmon_floor_number;
#[doc = "DDRMON_TOP_NUMBER (r) register accessor: The High Threshold in the Comparison of DDR Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_top_number::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_top_number`]
module"]
#[doc(alias = "DDRMON_TOP_NUMBER")]
pub type DdrmonTopNumber = crate::Reg<ddrmon_top_number::DdrmonTopNumberSpec>;
#[doc = "The High Threshold in the Comparison of DDR Access"]
pub mod ddrmon_top_number;
#[doc = "DDRMON_CH0_DFI_ACT_NUM (r) register accessor: Channel 0 DFI Active Command Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch0_dfi_act_num::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_ch0_dfi_act_num`]
module"]
#[doc(alias = "DDRMON_CH0_DFI_ACT_NUM")]
pub type DdrmonCh0DfiActNum = crate::Reg<ddrmon_ch0_dfi_act_num::DdrmonCh0DfiActNumSpec>;
#[doc = "Channel 0 DFI Active Command Number"]
pub mod ddrmon_ch0_dfi_act_num;
#[doc = "DDRMON_CH0_DFI_WR_NUM (r) register accessor: Channel 0 DFI write Command Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch0_dfi_wr_num::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_ch0_dfi_wr_num`]
module"]
#[doc(alias = "DDRMON_CH0_DFI_WR_NUM")]
pub type DdrmonCh0DfiWrNum = crate::Reg<ddrmon_ch0_dfi_wr_num::DdrmonCh0DfiWrNumSpec>;
#[doc = "Channel 0 DFI write Command Number"]
pub mod ddrmon_ch0_dfi_wr_num;
#[doc = "DDRMON_CH0_DFI_RD_NUM (r) register accessor: Channel 0 DFI read Command Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch0_dfi_rd_num::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_ch0_dfi_rd_num`]
module"]
#[doc(alias = "DDRMON_CH0_DFI_RD_NUM")]
pub type DdrmonCh0DfiRdNum = crate::Reg<ddrmon_ch0_dfi_rd_num::DdrmonCh0DfiRdNumSpec>;
#[doc = "Channel 0 DFI read Command Number"]
pub mod ddrmon_ch0_dfi_rd_num;
#[doc = "DDRMON_CH0_COUNT_NUM (r) register accessor: Channel 0 Timer Count Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch0_count_num::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_ch0_count_num`]
module"]
#[doc(alias = "DDRMON_CH0_COUNT_NUM")]
pub type DdrmonCh0CountNum = crate::Reg<ddrmon_ch0_count_num::DdrmonCh0CountNumSpec>;
#[doc = "Channel 0 Timer Count Number"]
pub mod ddrmon_ch0_count_num;
#[doc = "DDRMON_CH0_DFI_ACCESS_NUM (r) register accessor: Channel 0 DFI Read and Write Command Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch0_dfi_access_num::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_ch0_dfi_access_num`]
module"]
#[doc(alias = "DDRMON_CH0_DFI_ACCESS_NUM")]
pub type DdrmonCh0DfiAccessNum = crate::Reg<ddrmon_ch0_dfi_access_num::DdrmonCh0DfiAccessNumSpec>;
#[doc = "Channel 0 DFI Read and Write Command Number"]
pub mod ddrmon_ch0_dfi_access_num;
#[doc = "DDRMON_CH1_DFI_ACT_NUM (r) register accessor: Channel 1 DFI Active Command Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch1_dfi_act_num::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_ch1_dfi_act_num`]
module"]
#[doc(alias = "DDRMON_CH1_DFI_ACT_NUM")]
pub type DdrmonCh1DfiActNum = crate::Reg<ddrmon_ch1_dfi_act_num::DdrmonCh1DfiActNumSpec>;
#[doc = "Channel 1 DFI Active Command Number"]
pub mod ddrmon_ch1_dfi_act_num;
#[doc = "DDRMON_CH1_DFI_WR_NUM (r) register accessor: Channel 1 DFI write Command Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch1_dfi_wr_num::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_ch1_dfi_wr_num`]
module"]
#[doc(alias = "DDRMON_CH1_DFI_WR_NUM")]
pub type DdrmonCh1DfiWrNum = crate::Reg<ddrmon_ch1_dfi_wr_num::DdrmonCh1DfiWrNumSpec>;
#[doc = "Channel 1 DFI write Command Number"]
pub mod ddrmon_ch1_dfi_wr_num;
#[doc = "DDRMON_CH1_DFI_RD_NUM (r) register accessor: Channel 1 DFI read Command Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch1_dfi_rd_num::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_ch1_dfi_rd_num`]
module"]
#[doc(alias = "DDRMON_CH1_DFI_RD_NUM")]
pub type DdrmonCh1DfiRdNum = crate::Reg<ddrmon_ch1_dfi_rd_num::DdrmonCh1DfiRdNumSpec>;
#[doc = "Channel 1 DFI read Command Number"]
pub mod ddrmon_ch1_dfi_rd_num;
#[doc = "DDRMON_CH1_COUNT_NUM (r) register accessor: Channel 1 Timer Count Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch1_count_num::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_ch1_count_num`]
module"]
#[doc(alias = "DDRMON_CH1_COUNT_NUM")]
pub type DdrmonCh1CountNum = crate::Reg<ddrmon_ch1_count_num::DdrmonCh1CountNumSpec>;
#[doc = "Channel 1 Timer Count Number"]
pub mod ddrmon_ch1_count_num;
#[doc = "DDRMON_CH1_DFI_ACCESS_NUM (r) register accessor: Channel 1 DFI Read and Write Command Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch1_dfi_access_num::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_ch1_dfi_access_num`]
module"]
#[doc(alias = "DDRMON_CH1_DFI_ACCESS_NUM")]
pub type DdrmonCh1DfiAccessNum = crate::Reg<ddrmon_ch1_dfi_access_num::DdrmonCh1DfiAccessNumSpec>;
#[doc = "Channel 1 DFI Read and Write Command Number"]
pub mod ddrmon_ch1_dfi_access_num;
#[doc = "DDRMON_DDR_IF_CTRL (rw) register accessor: DDR interface Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ddr_if_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrmon_ddr_if_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_ddr_if_ctrl`]
module"]
#[doc(alias = "DDRMON_DDR_IF_CTRL")]
pub type DdrmonDdrIfCtrl = crate::Reg<ddrmon_ddr_if_ctrl::DdrmonDdrIfCtrlSpec>;
#[doc = "DDR interface Control Register"]
pub mod ddrmon_ddr_if_ctrl;
#[doc = "DDRMON_CH0_WR_START_ADDR (rw) register accessor: Channel 0 Write Start Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch0_wr_start_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrmon_ch0_wr_start_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_ch0_wr_start_addr`]
module"]
#[doc(alias = "DDRMON_CH0_WR_START_ADDR")]
pub type DdrmonCh0WrStartAddr = crate::Reg<ddrmon_ch0_wr_start_addr::DdrmonCh0WrStartAddrSpec>;
#[doc = "Channel 0 Write Start Address"]
pub mod ddrmon_ch0_wr_start_addr;
#[doc = "DDRMON_CH0_WR_END_ADDR (rw) register accessor: Channel 0 Write End Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch0_wr_end_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrmon_ch0_wr_end_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_ch0_wr_end_addr`]
module"]
#[doc(alias = "DDRMON_CH0_WR_END_ADDR")]
pub type DdrmonCh0WrEndAddr = crate::Reg<ddrmon_ch0_wr_end_addr::DdrmonCh0WrEndAddrSpec>;
#[doc = "Channel 0 Write End Address"]
pub mod ddrmon_ch0_wr_end_addr;
#[doc = "DDRMON_CH0_RD_START_ADDR (rw) register accessor: Channel 0 Read Start Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch0_rd_start_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrmon_ch0_rd_start_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_ch0_rd_start_addr`]
module"]
#[doc(alias = "DDRMON_CH0_RD_START_ADDR")]
pub type DdrmonCh0RdStartAddr = crate::Reg<ddrmon_ch0_rd_start_addr::DdrmonCh0RdStartAddrSpec>;
#[doc = "Channel 0 Read Start Address"]
pub mod ddrmon_ch0_rd_start_addr;
#[doc = "DDRMON_CH0_RD_END_ADDR (rw) register accessor: Channel 0 Read End Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch0_rd_end_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrmon_ch0_rd_end_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_ch0_rd_end_addr`]
module"]
#[doc(alias = "DDRMON_CH0_RD_END_ADDR")]
pub type DdrmonCh0RdEndAddr = crate::Reg<ddrmon_ch0_rd_end_addr::DdrmonCh0RdEndAddrSpec>;
#[doc = "Channel 0 Read End Address"]
pub mod ddrmon_ch0_rd_end_addr;
#[doc = "DDRMON_CH1_WR_START_ADDR (rw) register accessor: Channel 1 Write Start Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch1_wr_start_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrmon_ch1_wr_start_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_ch1_wr_start_addr`]
module"]
#[doc(alias = "DDRMON_CH1_WR_START_ADDR")]
pub type DdrmonCh1WrStartAddr = crate::Reg<ddrmon_ch1_wr_start_addr::DdrmonCh1WrStartAddrSpec>;
#[doc = "Channel 1 Write Start Address"]
pub mod ddrmon_ch1_wr_start_addr;
#[doc = "DDRMON_CH1_WR_END_ADDR (rw) register accessor: Channel 1 Write End Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch1_wr_end_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrmon_ch1_wr_end_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_ch1_wr_end_addr`]
module"]
#[doc(alias = "DDRMON_CH1_WR_END_ADDR")]
pub type DdrmonCh1WrEndAddr = crate::Reg<ddrmon_ch1_wr_end_addr::DdrmonCh1WrEndAddrSpec>;
#[doc = "Channel 1 Write End Address"]
pub mod ddrmon_ch1_wr_end_addr;
#[doc = "DDRMON_CH1_RD_START_ADDR (rw) register accessor: Channel 1 Read Start Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch1_rd_start_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrmon_ch1_rd_start_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_ch1_rd_start_addr`]
module"]
#[doc(alias = "DDRMON_CH1_RD_START_ADDR")]
pub type DdrmonCh1RdStartAddr = crate::Reg<ddrmon_ch1_rd_start_addr::DdrmonCh1RdStartAddrSpec>;
#[doc = "Channel 1 Read Start Address"]
pub mod ddrmon_ch1_rd_start_addr;
#[doc = "DDRMON_CH1_RD_END_ADDR (rw) register accessor: Channel 1 Read End Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch1_rd_end_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrmon_ch1_rd_end_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_ch1_rd_end_addr`]
module"]
#[doc(alias = "DDRMON_CH1_RD_END_ADDR")]
pub type DdrmonCh1RdEndAddr = crate::Reg<ddrmon_ch1_rd_end_addr::DdrmonCh1RdEndAddrSpec>;
#[doc = "Channel 1 Read End Address"]
pub mod ddrmon_ch1_rd_end_addr;
#[doc = "DDRMON_CH0_DDR_FIFO0_ADDR (r) register accessor: DDR Channel 0 Controller Interface Address FIFO0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch0_ddr_fifo0_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_ch0_ddr_fifo0_addr`]
module"]
#[doc(alias = "DDRMON_CH0_DDR_FIFO0_ADDR")]
pub type DdrmonCh0DdrFifo0Addr = crate::Reg<ddrmon_ch0_ddr_fifo0_addr::DdrmonCh0DdrFifo0AddrSpec>;
#[doc = "DDR Channel 0 Controller Interface Address FIFO0"]
pub mod ddrmon_ch0_ddr_fifo0_addr;
#[doc = "DDRMON_CH0_DDR_FIFO1_ADDR (r) register accessor: DDR Channel 0 Controller Interface Address FIFO1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch0_ddr_fifo1_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_ch0_ddr_fifo1_addr`]
module"]
#[doc(alias = "DDRMON_CH0_DDR_FIFO1_ADDR")]
pub type DdrmonCh0DdrFifo1Addr = crate::Reg<ddrmon_ch0_ddr_fifo1_addr::DdrmonCh0DdrFifo1AddrSpec>;
#[doc = "DDR Channel 0 Controller Interface Address FIFO1"]
pub mod ddrmon_ch0_ddr_fifo1_addr;
#[doc = "DDRMON_CH0_DDR_FIFO2_ADDR (r) register accessor: DDR Channel 0 Controller Interface Address FIFO2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch0_ddr_fifo2_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_ch0_ddr_fifo2_addr`]
module"]
#[doc(alias = "DDRMON_CH0_DDR_FIFO2_ADDR")]
pub type DdrmonCh0DdrFifo2Addr = crate::Reg<ddrmon_ch0_ddr_fifo2_addr::DdrmonCh0DdrFifo2AddrSpec>;
#[doc = "DDR Channel 0 Controller Interface Address FIFO2"]
pub mod ddrmon_ch0_ddr_fifo2_addr;
#[doc = "DDRMON_CH0_DDR_FIFO3_ADDR (r) register accessor: DDR Channel 0 Controller Interface Address FIFO3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch0_ddr_fifo3_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_ch0_ddr_fifo3_addr`]
module"]
#[doc(alias = "DDRMON_CH0_DDR_FIFO3_ADDR")]
pub type DdrmonCh0DdrFifo3Addr = crate::Reg<ddrmon_ch0_ddr_fifo3_addr::DdrmonCh0DdrFifo3AddrSpec>;
#[doc = "DDR Channel 0 Controller Interface Address FIFO3"]
pub mod ddrmon_ch0_ddr_fifo3_addr;
#[doc = "DDRMON_CH1_DDR_FIFO0_ADDR (r) register accessor: DDR Channel 1 Controller Interface Address FIFO0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch1_ddr_fifo0_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_ch1_ddr_fifo0_addr`]
module"]
#[doc(alias = "DDRMON_CH1_DDR_FIFO0_ADDR")]
pub type DdrmonCh1DdrFifo0Addr = crate::Reg<ddrmon_ch1_ddr_fifo0_addr::DdrmonCh1DdrFifo0AddrSpec>;
#[doc = "DDR Channel 1 Controller Interface Address FIFO0"]
pub mod ddrmon_ch1_ddr_fifo0_addr;
#[doc = "DDRMON_CH1_DDR_FIFO1_ADDR (r) register accessor: DDR Channel 1 Controller Interface Address FIFO1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch1_ddr_fifo1_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_ch1_ddr_fifo1_addr`]
module"]
#[doc(alias = "DDRMON_CH1_DDR_FIFO1_ADDR")]
pub type DdrmonCh1DdrFifo1Addr = crate::Reg<ddrmon_ch1_ddr_fifo1_addr::DdrmonCh1DdrFifo1AddrSpec>;
#[doc = "DDR Channel 1 Controller Interface Address FIFO1"]
pub mod ddrmon_ch1_ddr_fifo1_addr;
#[doc = "DDRMON_CH1_DDR_FIFO2_ADDR (r) register accessor: DDR Channel 1 Controller Interface Address FIFO2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch1_ddr_fifo2_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_ch1_ddr_fifo2_addr`]
module"]
#[doc(alias = "DDRMON_CH1_DDR_FIFO2_ADDR")]
pub type DdrmonCh1DdrFifo2Addr = crate::Reg<ddrmon_ch1_ddr_fifo2_addr::DdrmonCh1DdrFifo2AddrSpec>;
#[doc = "DDR Channel 1 Controller Interface Address FIFO2"]
pub mod ddrmon_ch1_ddr_fifo2_addr;
#[doc = "DDRMON_CH1_DDR_FIFO3_ADDR (r) register accessor: DDR Channel 1 Controller Interface Address FIFO3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch1_ddr_fifo3_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrmon_ch1_ddr_fifo3_addr`]
module"]
#[doc(alias = "DDRMON_CH1_DDR_FIFO3_ADDR")]
pub type DdrmonCh1DdrFifo3Addr = crate::Reg<ddrmon_ch1_ddr_fifo3_addr::DdrmonCh1DdrFifo3AddrSpec>;
#[doc = "DDR Channel 1 Controller Interface Address FIFO3"]
pub mod ddrmon_ch1_ddr_fifo3_addr;
