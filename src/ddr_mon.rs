#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ip_version: IpVersion,
    ctrl: Ctrl,
    int_status: IntStatus,
    int_mask: IntMask,
    timer_count: TimerCount,
    floor_number: FloorNumber,
    top_number: TopNumber,
    ch0_dfi_act_num: Ch0DfiActNum,
    ch0_dfi_wr_num: Ch0DfiWrNum,
    ch0_dfi_rd_num: Ch0DfiRdNum,
    ch0_count_num: Ch0CountNum,
    ch0_dfi_access_num: Ch0DfiAccessNum,
    ch1_dfi_act_num: Ch1DfiActNum,
    ch1_dfi_wr_num: Ch1DfiWrNum,
    ch1_dfi_rd_num: Ch1DfiRdNum,
    ch1_count_num: Ch1CountNum,
    ch1_dfi_access_num: Ch1DfiAccessNum,
    _reserved17: [u8; 0x01bc],
    ddr_if_ctrl: DdrIfCtrl,
    _reserved18: [u8; 0x08],
    ch0_wr_start_addr: Ch0WrStartAddr,
    ch0_wr_end_addr: Ch0WrEndAddr,
    ch0_rd_start_addr: Ch0RdStartAddr,
    ch0_rd_end_addr: Ch0RdEndAddr,
    _reserved22: [u8; 0x08],
    ch1_wr_start_addr: Ch1WrStartAddr,
    ch1_wr_end_addr: Ch1WrEndAddr,
    ch1_rd_start_addr: Ch1RdStartAddr,
    ch1_rd_end_addr: Ch1RdEndAddr,
    _reserved26: [u8; 0x0c],
    ch0_ddr_fifo0_addr: Ch0DdrFifo0Addr,
    _reserved27: [u8; 0x04],
    ch0_ddr_fifo1_addr: Ch0DdrFifo1Addr,
    _reserved28: [u8; 0x04],
    ch0_ddr_fifo2_addr: Ch0DdrFifo2Addr,
    _reserved29: [u8; 0x04],
    ch0_ddr_fifo3_addr: Ch0DdrFifo3Addr,
    _reserved30: [u8; 0x04],
    ch1_ddr_fifo0_addr: Ch1DdrFifo0Addr,
    _reserved31: [u8; 0x04],
    ch1_ddr_fifo1_addr: Ch1DdrFifo1Addr,
    _reserved32: [u8; 0x04],
    ch1_ddr_fifo2_addr: Ch1DdrFifo2Addr,
    _reserved33: [u8; 0x04],
    ch1_ddr_fifo3_addr: Ch1DdrFifo3Addr,
}
impl RegisterBlock {
    #[doc = "0x00 - DDR Monitor IP Version"]
    #[inline(always)]
    pub const fn ip_version(&self) -> &IpVersion {
        &self.ip_version
    }
    #[doc = "0x04 - DDR Monitor Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x08 - Interrupt Status"]
    #[inline(always)]
    pub const fn int_status(&self) -> &IntStatus {
        &self.int_status
    }
    #[doc = "0x0c - Interrupt mask control"]
    #[inline(always)]
    pub const fn int_mask(&self) -> &IntMask {
        &self.int_mask
    }
    #[doc = "0x10 - The DFI Timer Threshold"]
    #[inline(always)]
    pub const fn timer_count(&self) -> &TimerCount {
        &self.timer_count
    }
    #[doc = "0x14 - The Low Threshold in the Comparison of DDR Access"]
    #[inline(always)]
    pub const fn floor_number(&self) -> &FloorNumber {
        &self.floor_number
    }
    #[doc = "0x18 - The High Threshold in the Comparison of DDR Access"]
    #[inline(always)]
    pub const fn top_number(&self) -> &TopNumber {
        &self.top_number
    }
    #[doc = "0x1c - Channel 0 DFI Active Command Number"]
    #[inline(always)]
    pub const fn ch0_dfi_act_num(&self) -> &Ch0DfiActNum {
        &self.ch0_dfi_act_num
    }
    #[doc = "0x20 - Channel 0 DFI write Command Number"]
    #[inline(always)]
    pub const fn ch0_dfi_wr_num(&self) -> &Ch0DfiWrNum {
        &self.ch0_dfi_wr_num
    }
    #[doc = "0x24 - Channel 0 DFI read Command Number"]
    #[inline(always)]
    pub const fn ch0_dfi_rd_num(&self) -> &Ch0DfiRdNum {
        &self.ch0_dfi_rd_num
    }
    #[doc = "0x28 - Channel 0 Timer Count Number"]
    #[inline(always)]
    pub const fn ch0_count_num(&self) -> &Ch0CountNum {
        &self.ch0_count_num
    }
    #[doc = "0x2c - Channel 0 DFI Read and Write Command Number"]
    #[inline(always)]
    pub const fn ch0_dfi_access_num(&self) -> &Ch0DfiAccessNum {
        &self.ch0_dfi_access_num
    }
    #[doc = "0x30 - Channel 1 DFI Active Command Number"]
    #[inline(always)]
    pub const fn ch1_dfi_act_num(&self) -> &Ch1DfiActNum {
        &self.ch1_dfi_act_num
    }
    #[doc = "0x34 - Channel 1 DFI write Command Number"]
    #[inline(always)]
    pub const fn ch1_dfi_wr_num(&self) -> &Ch1DfiWrNum {
        &self.ch1_dfi_wr_num
    }
    #[doc = "0x38 - Channel 1 DFI read Command Number"]
    #[inline(always)]
    pub const fn ch1_dfi_rd_num(&self) -> &Ch1DfiRdNum {
        &self.ch1_dfi_rd_num
    }
    #[doc = "0x3c - Channel 1 Timer Count Number"]
    #[inline(always)]
    pub const fn ch1_count_num(&self) -> &Ch1CountNum {
        &self.ch1_count_num
    }
    #[doc = "0x40 - Channel 1 DFI Read and Write Command Number"]
    #[inline(always)]
    pub const fn ch1_dfi_access_num(&self) -> &Ch1DfiAccessNum {
        &self.ch1_dfi_access_num
    }
    #[doc = "0x200 - DDR interface Control Register"]
    #[inline(always)]
    pub const fn ddr_if_ctrl(&self) -> &DdrIfCtrl {
        &self.ddr_if_ctrl
    }
    #[doc = "0x20c - Channel 0 Write Start Address"]
    #[inline(always)]
    pub const fn ch0_wr_start_addr(&self) -> &Ch0WrStartAddr {
        &self.ch0_wr_start_addr
    }
    #[doc = "0x210 - Channel 0 Write End Address"]
    #[inline(always)]
    pub const fn ch0_wr_end_addr(&self) -> &Ch0WrEndAddr {
        &self.ch0_wr_end_addr
    }
    #[doc = "0x214 - Channel 0 Read Start Address"]
    #[inline(always)]
    pub const fn ch0_rd_start_addr(&self) -> &Ch0RdStartAddr {
        &self.ch0_rd_start_addr
    }
    #[doc = "0x218 - Channel 0 Read End Address"]
    #[inline(always)]
    pub const fn ch0_rd_end_addr(&self) -> &Ch0RdEndAddr {
        &self.ch0_rd_end_addr
    }
    #[doc = "0x224 - Channel 1 Write Start Address"]
    #[inline(always)]
    pub const fn ch1_wr_start_addr(&self) -> &Ch1WrStartAddr {
        &self.ch1_wr_start_addr
    }
    #[doc = "0x228 - Channel 1 Write End Address"]
    #[inline(always)]
    pub const fn ch1_wr_end_addr(&self) -> &Ch1WrEndAddr {
        &self.ch1_wr_end_addr
    }
    #[doc = "0x22c - Channel 1 Read Start Address"]
    #[inline(always)]
    pub const fn ch1_rd_start_addr(&self) -> &Ch1RdStartAddr {
        &self.ch1_rd_start_addr
    }
    #[doc = "0x230 - Channel 1 Read End Address"]
    #[inline(always)]
    pub const fn ch1_rd_end_addr(&self) -> &Ch1RdEndAddr {
        &self.ch1_rd_end_addr
    }
    #[doc = "0x240 - DDR Channel 0 Controller Interface Address FIFO0"]
    #[inline(always)]
    pub const fn ch0_ddr_fifo0_addr(&self) -> &Ch0DdrFifo0Addr {
        &self.ch0_ddr_fifo0_addr
    }
    #[doc = "0x248 - DDR Channel 0 Controller Interface Address FIFO1"]
    #[inline(always)]
    pub const fn ch0_ddr_fifo1_addr(&self) -> &Ch0DdrFifo1Addr {
        &self.ch0_ddr_fifo1_addr
    }
    #[doc = "0x250 - DDR Channel 0 Controller Interface Address FIFO2"]
    #[inline(always)]
    pub const fn ch0_ddr_fifo2_addr(&self) -> &Ch0DdrFifo2Addr {
        &self.ch0_ddr_fifo2_addr
    }
    #[doc = "0x258 - DDR Channel 0 Controller Interface Address FIFO3"]
    #[inline(always)]
    pub const fn ch0_ddr_fifo3_addr(&self) -> &Ch0DdrFifo3Addr {
        &self.ch0_ddr_fifo3_addr
    }
    #[doc = "0x260 - DDR Channel 1 Controller Interface Address FIFO0"]
    #[inline(always)]
    pub const fn ch1_ddr_fifo0_addr(&self) -> &Ch1DdrFifo0Addr {
        &self.ch1_ddr_fifo0_addr
    }
    #[doc = "0x268 - DDR Channel 1 Controller Interface Address FIFO1"]
    #[inline(always)]
    pub const fn ch1_ddr_fifo1_addr(&self) -> &Ch1DdrFifo1Addr {
        &self.ch1_ddr_fifo1_addr
    }
    #[doc = "0x270 - DDR Channel 1 Controller Interface Address FIFO2"]
    #[inline(always)]
    pub const fn ch1_ddr_fifo2_addr(&self) -> &Ch1DdrFifo2Addr {
        &self.ch1_ddr_fifo2_addr
    }
    #[doc = "0x278 - DDR Channel 1 Controller Interface Address FIFO3"]
    #[inline(always)]
    pub const fn ch1_ddr_fifo3_addr(&self) -> &Ch1DdrFifo3Addr {
        &self.ch1_ddr_fifo3_addr
    }
}
#[doc = "IP_VERSION (r) register accessor: DDR Monitor IP Version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ip_version::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ip_version`]
module"]
#[doc(alias = "IP_VERSION")]
pub type IpVersion = crate::Reg<ip_version::IpVersionSpec>;
#[doc = "DDR Monitor IP Version"]
pub mod ip_version;
#[doc = "CTRL (rw) register accessor: DDR Monitor Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "DDR Monitor Control Register"]
pub mod ctrl;
#[doc = "INT_STATUS (r) register accessor: Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_status`]
module"]
#[doc(alias = "INT_STATUS")]
pub type IntStatus = crate::Reg<int_status::IntStatusSpec>;
#[doc = "Interrupt Status"]
pub mod int_status;
#[doc = "INT_MASK (rw) register accessor: Interrupt mask control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_mask`]
module"]
#[doc(alias = "INT_MASK")]
pub type IntMask = crate::Reg<int_mask::IntMaskSpec>;
#[doc = "Interrupt mask control"]
pub mod int_mask;
#[doc = "TIMER_COUNT (r) register accessor: The DFI Timer Threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_count::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_count`]
module"]
#[doc(alias = "TIMER_COUNT")]
pub type TimerCount = crate::Reg<timer_count::TimerCountSpec>;
#[doc = "The DFI Timer Threshold"]
pub mod timer_count;
#[doc = "FLOOR_NUMBER (r) register accessor: The Low Threshold in the Comparison of DDR Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`floor_number::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@floor_number`]
module"]
#[doc(alias = "FLOOR_NUMBER")]
pub type FloorNumber = crate::Reg<floor_number::FloorNumberSpec>;
#[doc = "The Low Threshold in the Comparison of DDR Access"]
pub mod floor_number;
#[doc = "TOP_NUMBER (r) register accessor: The High Threshold in the Comparison of DDR Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`top_number::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@top_number`]
module"]
#[doc(alias = "TOP_NUMBER")]
pub type TopNumber = crate::Reg<top_number::TopNumberSpec>;
#[doc = "The High Threshold in the Comparison of DDR Access"]
pub mod top_number;
#[doc = "CH0_DFI_ACT_NUM (r) register accessor: Channel 0 DFI Active Command Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_dfi_act_num::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_dfi_act_num`]
module"]
#[doc(alias = "CH0_DFI_ACT_NUM")]
pub type Ch0DfiActNum = crate::Reg<ch0_dfi_act_num::Ch0DfiActNumSpec>;
#[doc = "Channel 0 DFI Active Command Number"]
pub mod ch0_dfi_act_num;
#[doc = "CH0_DFI_WR_NUM (r) register accessor: Channel 0 DFI write Command Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_dfi_wr_num::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_dfi_wr_num`]
module"]
#[doc(alias = "CH0_DFI_WR_NUM")]
pub type Ch0DfiWrNum = crate::Reg<ch0_dfi_wr_num::Ch0DfiWrNumSpec>;
#[doc = "Channel 0 DFI write Command Number"]
pub mod ch0_dfi_wr_num;
#[doc = "CH0_DFI_RD_NUM (r) register accessor: Channel 0 DFI read Command Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_dfi_rd_num::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_dfi_rd_num`]
module"]
#[doc(alias = "CH0_DFI_RD_NUM")]
pub type Ch0DfiRdNum = crate::Reg<ch0_dfi_rd_num::Ch0DfiRdNumSpec>;
#[doc = "Channel 0 DFI read Command Number"]
pub mod ch0_dfi_rd_num;
#[doc = "CH0_COUNT_NUM (r) register accessor: Channel 0 Timer Count Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_count_num::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_count_num`]
module"]
#[doc(alias = "CH0_COUNT_NUM")]
pub type Ch0CountNum = crate::Reg<ch0_count_num::Ch0CountNumSpec>;
#[doc = "Channel 0 Timer Count Number"]
pub mod ch0_count_num;
#[doc = "CH0_DFI_ACCESS_NUM (r) register accessor: Channel 0 DFI Read and Write Command Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_dfi_access_num::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_dfi_access_num`]
module"]
#[doc(alias = "CH0_DFI_ACCESS_NUM")]
pub type Ch0DfiAccessNum = crate::Reg<ch0_dfi_access_num::Ch0DfiAccessNumSpec>;
#[doc = "Channel 0 DFI Read and Write Command Number"]
pub mod ch0_dfi_access_num;
#[doc = "CH1_DFI_ACT_NUM (r) register accessor: Channel 1 DFI Active Command Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_dfi_act_num::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_dfi_act_num`]
module"]
#[doc(alias = "CH1_DFI_ACT_NUM")]
pub type Ch1DfiActNum = crate::Reg<ch1_dfi_act_num::Ch1DfiActNumSpec>;
#[doc = "Channel 1 DFI Active Command Number"]
pub mod ch1_dfi_act_num;
#[doc = "CH1_DFI_WR_NUM (r) register accessor: Channel 1 DFI write Command Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_dfi_wr_num::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_dfi_wr_num`]
module"]
#[doc(alias = "CH1_DFI_WR_NUM")]
pub type Ch1DfiWrNum = crate::Reg<ch1_dfi_wr_num::Ch1DfiWrNumSpec>;
#[doc = "Channel 1 DFI write Command Number"]
pub mod ch1_dfi_wr_num;
#[doc = "CH1_DFI_RD_NUM (r) register accessor: Channel 1 DFI read Command Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_dfi_rd_num::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_dfi_rd_num`]
module"]
#[doc(alias = "CH1_DFI_RD_NUM")]
pub type Ch1DfiRdNum = crate::Reg<ch1_dfi_rd_num::Ch1DfiRdNumSpec>;
#[doc = "Channel 1 DFI read Command Number"]
pub mod ch1_dfi_rd_num;
#[doc = "CH1_COUNT_NUM (r) register accessor: Channel 1 Timer Count Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_count_num::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_count_num`]
module"]
#[doc(alias = "CH1_COUNT_NUM")]
pub type Ch1CountNum = crate::Reg<ch1_count_num::Ch1CountNumSpec>;
#[doc = "Channel 1 Timer Count Number"]
pub mod ch1_count_num;
#[doc = "CH1_DFI_ACCESS_NUM (r) register accessor: Channel 1 DFI Read and Write Command Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_dfi_access_num::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_dfi_access_num`]
module"]
#[doc(alias = "CH1_DFI_ACCESS_NUM")]
pub type Ch1DfiAccessNum = crate::Reg<ch1_dfi_access_num::Ch1DfiAccessNumSpec>;
#[doc = "Channel 1 DFI Read and Write Command Number"]
pub mod ch1_dfi_access_num;
#[doc = "DDR_IF_CTRL (rw) register accessor: DDR interface Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_if_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_if_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_if_ctrl`]
module"]
#[doc(alias = "DDR_IF_CTRL")]
pub type DdrIfCtrl = crate::Reg<ddr_if_ctrl::DdrIfCtrlSpec>;
#[doc = "DDR interface Control Register"]
pub mod ddr_if_ctrl;
#[doc = "CH0_WR_START_ADDR (rw) register accessor: Channel 0 Write Start Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_wr_start_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_wr_start_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_wr_start_addr`]
module"]
#[doc(alias = "CH0_WR_START_ADDR")]
pub type Ch0WrStartAddr = crate::Reg<ch0_wr_start_addr::Ch0WrStartAddrSpec>;
#[doc = "Channel 0 Write Start Address"]
pub mod ch0_wr_start_addr;
#[doc = "CH0_WR_END_ADDR (rw) register accessor: Channel 0 Write End Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_wr_end_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_wr_end_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_wr_end_addr`]
module"]
#[doc(alias = "CH0_WR_END_ADDR")]
pub type Ch0WrEndAddr = crate::Reg<ch0_wr_end_addr::Ch0WrEndAddrSpec>;
#[doc = "Channel 0 Write End Address"]
pub mod ch0_wr_end_addr;
#[doc = "CH0_RD_START_ADDR (rw) register accessor: Channel 0 Read Start Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_rd_start_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_rd_start_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_rd_start_addr`]
module"]
#[doc(alias = "CH0_RD_START_ADDR")]
pub type Ch0RdStartAddr = crate::Reg<ch0_rd_start_addr::Ch0RdStartAddrSpec>;
#[doc = "Channel 0 Read Start Address"]
pub mod ch0_rd_start_addr;
#[doc = "CH0_RD_END_ADDR (rw) register accessor: Channel 0 Read End Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_rd_end_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_rd_end_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_rd_end_addr`]
module"]
#[doc(alias = "CH0_RD_END_ADDR")]
pub type Ch0RdEndAddr = crate::Reg<ch0_rd_end_addr::Ch0RdEndAddrSpec>;
#[doc = "Channel 0 Read End Address"]
pub mod ch0_rd_end_addr;
#[doc = "CH1_WR_START_ADDR (rw) register accessor: Channel 1 Write Start Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_wr_start_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_wr_start_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_wr_start_addr`]
module"]
#[doc(alias = "CH1_WR_START_ADDR")]
pub type Ch1WrStartAddr = crate::Reg<ch1_wr_start_addr::Ch1WrStartAddrSpec>;
#[doc = "Channel 1 Write Start Address"]
pub mod ch1_wr_start_addr;
#[doc = "CH1_WR_END_ADDR (rw) register accessor: Channel 1 Write End Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_wr_end_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_wr_end_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_wr_end_addr`]
module"]
#[doc(alias = "CH1_WR_END_ADDR")]
pub type Ch1WrEndAddr = crate::Reg<ch1_wr_end_addr::Ch1WrEndAddrSpec>;
#[doc = "Channel 1 Write End Address"]
pub mod ch1_wr_end_addr;
#[doc = "CH1_RD_START_ADDR (rw) register accessor: Channel 1 Read Start Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_rd_start_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_rd_start_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_rd_start_addr`]
module"]
#[doc(alias = "CH1_RD_START_ADDR")]
pub type Ch1RdStartAddr = crate::Reg<ch1_rd_start_addr::Ch1RdStartAddrSpec>;
#[doc = "Channel 1 Read Start Address"]
pub mod ch1_rd_start_addr;
#[doc = "CH1_RD_END_ADDR (rw) register accessor: Channel 1 Read End Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_rd_end_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_rd_end_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_rd_end_addr`]
module"]
#[doc(alias = "CH1_RD_END_ADDR")]
pub type Ch1RdEndAddr = crate::Reg<ch1_rd_end_addr::Ch1RdEndAddrSpec>;
#[doc = "Channel 1 Read End Address"]
pub mod ch1_rd_end_addr;
#[doc = "CH0_DDR_FIFO0_ADDR (r) register accessor: DDR Channel 0 Controller Interface Address FIFO0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_ddr_fifo0_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_ddr_fifo0_addr`]
module"]
#[doc(alias = "CH0_DDR_FIFO0_ADDR")]
pub type Ch0DdrFifo0Addr = crate::Reg<ch0_ddr_fifo0_addr::Ch0DdrFifo0AddrSpec>;
#[doc = "DDR Channel 0 Controller Interface Address FIFO0"]
pub mod ch0_ddr_fifo0_addr;
#[doc = "CH0_DDR_FIFO1_ADDR (r) register accessor: DDR Channel 0 Controller Interface Address FIFO1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_ddr_fifo1_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_ddr_fifo1_addr`]
module"]
#[doc(alias = "CH0_DDR_FIFO1_ADDR")]
pub type Ch0DdrFifo1Addr = crate::Reg<ch0_ddr_fifo1_addr::Ch0DdrFifo1AddrSpec>;
#[doc = "DDR Channel 0 Controller Interface Address FIFO1"]
pub mod ch0_ddr_fifo1_addr;
#[doc = "CH0_DDR_FIFO2_ADDR (r) register accessor: DDR Channel 0 Controller Interface Address FIFO2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_ddr_fifo2_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_ddr_fifo2_addr`]
module"]
#[doc(alias = "CH0_DDR_FIFO2_ADDR")]
pub type Ch0DdrFifo2Addr = crate::Reg<ch0_ddr_fifo2_addr::Ch0DdrFifo2AddrSpec>;
#[doc = "DDR Channel 0 Controller Interface Address FIFO2"]
pub mod ch0_ddr_fifo2_addr;
#[doc = "CH0_DDR_FIFO3_ADDR (r) register accessor: DDR Channel 0 Controller Interface Address FIFO3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_ddr_fifo3_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_ddr_fifo3_addr`]
module"]
#[doc(alias = "CH0_DDR_FIFO3_ADDR")]
pub type Ch0DdrFifo3Addr = crate::Reg<ch0_ddr_fifo3_addr::Ch0DdrFifo3AddrSpec>;
#[doc = "DDR Channel 0 Controller Interface Address FIFO3"]
pub mod ch0_ddr_fifo3_addr;
#[doc = "CH1_DDR_FIFO0_ADDR (r) register accessor: DDR Channel 1 Controller Interface Address FIFO0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_ddr_fifo0_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_ddr_fifo0_addr`]
module"]
#[doc(alias = "CH1_DDR_FIFO0_ADDR")]
pub type Ch1DdrFifo0Addr = crate::Reg<ch1_ddr_fifo0_addr::Ch1DdrFifo0AddrSpec>;
#[doc = "DDR Channel 1 Controller Interface Address FIFO0"]
pub mod ch1_ddr_fifo0_addr;
#[doc = "CH1_DDR_FIFO1_ADDR (r) register accessor: DDR Channel 1 Controller Interface Address FIFO1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_ddr_fifo1_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_ddr_fifo1_addr`]
module"]
#[doc(alias = "CH1_DDR_FIFO1_ADDR")]
pub type Ch1DdrFifo1Addr = crate::Reg<ch1_ddr_fifo1_addr::Ch1DdrFifo1AddrSpec>;
#[doc = "DDR Channel 1 Controller Interface Address FIFO1"]
pub mod ch1_ddr_fifo1_addr;
#[doc = "CH1_DDR_FIFO2_ADDR (r) register accessor: DDR Channel 1 Controller Interface Address FIFO2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_ddr_fifo2_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_ddr_fifo2_addr`]
module"]
#[doc(alias = "CH1_DDR_FIFO2_ADDR")]
pub type Ch1DdrFifo2Addr = crate::Reg<ch1_ddr_fifo2_addr::Ch1DdrFifo2AddrSpec>;
#[doc = "DDR Channel 1 Controller Interface Address FIFO2"]
pub mod ch1_ddr_fifo2_addr;
#[doc = "CH1_DDR_FIFO3_ADDR (r) register accessor: DDR Channel 1 Controller Interface Address FIFO3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_ddr_fifo3_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_ddr_fifo3_addr`]
module"]
#[doc(alias = "CH1_DDR_FIFO3_ADDR")]
pub type Ch1DdrFifo3Addr = crate::Reg<ch1_ddr_fifo3_addr::Ch1DdrFifo3AddrSpec>;
#[doc = "DDR Channel 1 Controller Interface Address FIFO3"]
pub mod ch1_ddr_fifo3_addr;
