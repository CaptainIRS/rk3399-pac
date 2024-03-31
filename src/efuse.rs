#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    dout: Dout,
    rf: Rf,
    _reserved3: [u8; 0x04],
    jtag_pass: JtagPass,
    strobe_finish_ctrl: StrobeFinishCtrl,
}
impl RegisterBlock {
    #[doc = "0x00 - e fuse control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - e fuse data out register"]
    #[inline(always)]
    pub const fn dout(&self) -> &Dout {
        &self.dout
    }
    #[doc = "0x08 - e fuse redundancy bit used indicator register"]
    #[inline(always)]
    pub const fn rf(&self) -> &Rf {
        &self.rf
    }
    #[doc = "0x10 - Jtag password"]
    #[inline(always)]
    pub const fn jtag_pass(&self) -> &JtagPass {
        &self.jtag_pass
    }
    #[doc = "0x14 - e fuse strobe finish control register"]
    #[inline(always)]
    pub const fn strobe_finish_ctrl(&self) -> &StrobeFinishCtrl {
        &self.strobe_finish_ctrl
    }
}
#[doc = "CTRL (rw) register accessor: e fuse control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "e fuse control register"]
pub mod ctrl;
#[doc = "DOUT (r) register accessor: e fuse data out register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout`]
module"]
#[doc(alias = "DOUT")]
pub type Dout = crate::Reg<dout::DoutSpec>;
#[doc = "e fuse data out register"]
pub mod dout;
#[doc = "RF (r) register accessor: e fuse redundancy bit used indicator register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf`]
module"]
#[doc(alias = "RF")]
pub type Rf = crate::Reg<rf::RfSpec>;
#[doc = "e fuse redundancy bit used indicator register"]
pub mod rf;
#[doc = "JTAG_PASS (rw) register accessor: Jtag password\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jtag_pass::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jtag_pass::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jtag_pass`]
module"]
#[doc(alias = "JTAG_PASS")]
pub type JtagPass = crate::Reg<jtag_pass::JtagPassSpec>;
#[doc = "Jtag password"]
pub mod jtag_pass;
#[doc = "STROBE_FINISH_CTRL (rw) register accessor: e fuse strobe finish control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`strobe_finish_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`strobe_finish_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@strobe_finish_ctrl`]
module"]
#[doc(alias = "STROBE_FINISH_CTRL")]
pub type StrobeFinishCtrl = crate::Reg<strobe_finish_ctrl::StrobeFinishCtrlSpec>;
#[doc = "e fuse strobe finish control register"]
pub mod strobe_finish_ctrl;
