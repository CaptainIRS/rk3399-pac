#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    _reserved1: [u8; 0x04],
    addr: Addr,
    isr: Isr,
    _reserved3: [u8; 0x04],
    tose: Tose,
    tocm: Tocm,
    cmd_cfg: CmdCfg,
}
impl RegisterBlock {
    #[doc = "0x00 - DCF Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x08 - DCF DMA transaction starting address"]
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        &self.addr
    }
    #[doc = "0x0c - DCF Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x14 - dcf instruction timeout register"]
    #[inline(always)]
    pub const fn tose(&self) -> &Tose {
        &self.tose
    }
    #[doc = "0x18 - timeout command"]
    #[inline(always)]
    pub const fn tocm(&self) -> &Tocm {
        &self.tocm
    }
    #[doc = "0x1c - command config register"]
    #[inline(always)]
    pub const fn cmd_cfg(&self) -> &CmdCfg {
        &self.cmd_cfg
    }
}
#[doc = "CTRL (rw) register accessor: DCF Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "DCF Control Register"]
pub mod ctrl;
#[doc = "ADDR (rw) register accessor: DCF DMA transaction starting address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
#[doc(alias = "ADDR")]
pub type Addr = crate::Reg<addr::AddrSpec>;
#[doc = "DCF DMA transaction starting address"]
pub mod addr;
#[doc = "ISR (rw) register accessor: DCF Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "DCF Interrupt Status Register"]
pub mod isr;
#[doc = "TOSE (rw) register accessor: dcf instruction timeout register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tose::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tose::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tose`]
module"]
#[doc(alias = "TOSE")]
pub type Tose = crate::Reg<tose::ToseSpec>;
#[doc = "dcf instruction timeout register"]
pub mod tose;
#[doc = "TOCM (rw) register accessor: timeout command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tocm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tocm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tocm`]
module"]
#[doc(alias = "TOCM")]
pub type Tocm = crate::Reg<tocm::TocmSpec>;
#[doc = "timeout command"]
pub mod tocm;
#[doc = "CMD_CFG (rw) register accessor: command config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_cfg`]
module"]
#[doc(alias = "CMD_CFG")]
pub type CmdCfg = crate::Reg<cmd_cfg::CmdCfgSpec>;
#[doc = "command config register"]
pub mod cmd_cfg;
