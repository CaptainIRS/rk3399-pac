#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    efuse_ctrl: EfuseCtrl,
    efuse_dout: EfuseDout,
    efuse_rf: EfuseRf,
    _reserved3: [u8; 0x04],
    efuse_jtag_pass: EfuseJtagPass,
    efuse_strobe_finish_ctrl: EfuseStrobeFinishCtrl,
}
impl RegisterBlock {
    #[doc = "0x00 - e fuse control register"]
    #[inline(always)]
    pub const fn efuse_ctrl(&self) -> &EfuseCtrl {
        &self.efuse_ctrl
    }
    #[doc = "0x04 - e fuse data out register"]
    #[inline(always)]
    pub const fn efuse_dout(&self) -> &EfuseDout {
        &self.efuse_dout
    }
    #[doc = "0x08 - e fuse redundancy bit used indicator register"]
    #[inline(always)]
    pub const fn efuse_rf(&self) -> &EfuseRf {
        &self.efuse_rf
    }
    #[doc = "0x10 - Jtag password"]
    #[inline(always)]
    pub const fn efuse_jtag_pass(&self) -> &EfuseJtagPass {
        &self.efuse_jtag_pass
    }
    #[doc = "0x14 - e fuse strobe finish control register"]
    #[inline(always)]
    pub const fn efuse_strobe_finish_ctrl(&self) -> &EfuseStrobeFinishCtrl {
        &self.efuse_strobe_finish_ctrl
    }
}
#[doc = "EFUSE_CTRL (rw) register accessor: e fuse control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuse_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuse_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_ctrl`]
module"]
#[doc(alias = "EFUSE_CTRL")]
pub type EfuseCtrl = crate::Reg<efuse_ctrl::EfuseCtrlSpec>;
#[doc = "e fuse control register"]
pub mod efuse_ctrl;
#[doc = "EFUSE_DOUT (r) register accessor: e fuse data out register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuse_dout::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_dout`]
module"]
#[doc(alias = "EFUSE_DOUT")]
pub type EfuseDout = crate::Reg<efuse_dout::EfuseDoutSpec>;
#[doc = "e fuse data out register"]
pub mod efuse_dout;
#[doc = "EFUSE_RF (r) register accessor: e fuse redundancy bit used indicator register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuse_rf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_rf`]
module"]
#[doc(alias = "EFUSE_RF")]
pub type EfuseRf = crate::Reg<efuse_rf::EfuseRfSpec>;
#[doc = "e fuse redundancy bit used indicator register"]
pub mod efuse_rf;
#[doc = "EFUSE_JTAG_PASS (rw) register accessor: Jtag password\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuse_jtag_pass::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuse_jtag_pass::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_jtag_pass`]
module"]
#[doc(alias = "EFUSE_JTAG_PASS")]
pub type EfuseJtagPass = crate::Reg<efuse_jtag_pass::EfuseJtagPassSpec>;
#[doc = "Jtag password"]
pub mod efuse_jtag_pass;
#[doc = "EFUSE_STROBE_FINISH_CTRL (rw) register accessor: e fuse strobe finish control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuse_strobe_finish_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuse_strobe_finish_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_strobe_finish_ctrl`]
module"]
#[doc(alias = "EFUSE_STROBE_FINISH_CTRL")]
pub type EfuseStrobeFinishCtrl = crate::Reg<efuse_strobe_finish_ctrl::EfuseStrobeFinishCtrlSpec>;
#[doc = "e fuse strobe finish control register"]
pub mod efuse_strobe_finish_ctrl;
