#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cic_ctrl0: CicCtrl0,
    cic_ctrl1: CicCtrl1,
    cic_idle_th: CicIdleTh,
    cic_cg_wait_th: CicCgWaitTh,
    cic_status0: CicStatus0,
    cic_status1: CicStatus1,
    cic_ctrl2: CicCtrl2,
    cic_ctrl3: CicCtrl3,
    cic_ctrl4: CicCtrl4,
    _reserved9: [u8; 0x1c],
    cic_status2: CicStatus2,
}
impl RegisterBlock {
    #[doc = "0x00 - DDR Controller LP Interface Control Register 0"]
    #[inline(always)]
    pub const fn cic_ctrl0(&self) -> &CicCtrl0 {
        &self.cic_ctrl0
    }
    #[doc = "0x04 - DDR Controller LP Interface Control Register 1"]
    #[inline(always)]
    pub const fn cic_ctrl1(&self) -> &CicCtrl1 {
        &self.cic_ctrl1
    }
    #[doc = "0x08 - DDR Controller LP Interface Idle Threshold in standby mode"]
    #[inline(always)]
    pub const fn cic_idle_th(&self) -> &CicIdleTh {
        &self.cic_idle_th
    }
    #[doc = "0x0c - DDR Controller LP Interface CG Wait Threshold in standby mode"]
    #[inline(always)]
    pub const fn cic_cg_wait_th(&self) -> &CicCgWaitTh {
        &self.cic_cg_wait_th
    }
    #[doc = "0x10 - DDR Controller LP Interface Status Register 0"]
    #[inline(always)]
    pub const fn cic_status0(&self) -> &CicStatus0 {
        &self.cic_status0
    }
    #[doc = "0x14 - DDR Controller LP Interface Status Register 1"]
    #[inline(always)]
    pub const fn cic_status1(&self) -> &CicStatus1 {
        &self.cic_status1
    }
    #[doc = "0x18 - DDR Controller LP Interface Control Register 2"]
    #[inline(always)]
    pub const fn cic_ctrl2(&self) -> &CicCtrl2 {
        &self.cic_ctrl2
    }
    #[doc = "0x1c - DDR Controller LP Interface Control Register 3"]
    #[inline(always)]
    pub const fn cic_ctrl3(&self) -> &CicCtrl3 {
        &self.cic_ctrl3
    }
    #[doc = "0x20 - DDR Controller LP Interface Control Register 4"]
    #[inline(always)]
    pub const fn cic_ctrl4(&self) -> &CicCtrl4 {
        &self.cic_ctrl4
    }
    #[doc = "0x40 - DDR Controller LP Interface Status Register 2"]
    #[inline(always)]
    pub const fn cic_status2(&self) -> &CicStatus2 {
        &self.cic_status2
    }
}
#[doc = "CIC_CTRL0 (rw) register accessor: DDR Controller LP Interface Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cic_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cic_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cic_ctrl0`]
module"]
#[doc(alias = "CIC_CTRL0")]
pub type CicCtrl0 = crate::Reg<cic_ctrl0::CicCtrl0Spec>;
#[doc = "DDR Controller LP Interface Control Register 0"]
pub mod cic_ctrl0;
#[doc = "CIC_CTRL1 (rw) register accessor: DDR Controller LP Interface Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cic_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cic_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cic_ctrl1`]
module"]
#[doc(alias = "CIC_CTRL1")]
pub type CicCtrl1 = crate::Reg<cic_ctrl1::CicCtrl1Spec>;
#[doc = "DDR Controller LP Interface Control Register 1"]
pub mod cic_ctrl1;
#[doc = "CIC_IDLE_TH (rw) register accessor: DDR Controller LP Interface Idle Threshold in standby mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cic_idle_th::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cic_idle_th::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cic_idle_th`]
module"]
#[doc(alias = "CIC_IDLE_TH")]
pub type CicIdleTh = crate::Reg<cic_idle_th::CicIdleThSpec>;
#[doc = "DDR Controller LP Interface Idle Threshold in standby mode"]
pub mod cic_idle_th;
#[doc = "CIC_CG_WAIT_TH (rw) register accessor: DDR Controller LP Interface CG Wait Threshold in standby mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cic_cg_wait_th::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cic_cg_wait_th::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cic_cg_wait_th`]
module"]
#[doc(alias = "CIC_CG_WAIT_TH")]
pub type CicCgWaitTh = crate::Reg<cic_cg_wait_th::CicCgWaitThSpec>;
#[doc = "DDR Controller LP Interface CG Wait Threshold in standby mode"]
pub mod cic_cg_wait_th;
#[doc = "CIC_STATUS0 (r) register accessor: DDR Controller LP Interface Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cic_status0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cic_status0`]
module"]
#[doc(alias = "CIC_STATUS0")]
pub type CicStatus0 = crate::Reg<cic_status0::CicStatus0Spec>;
#[doc = "DDR Controller LP Interface Status Register 0"]
pub mod cic_status0;
#[doc = "CIC_STATUS1 (r) register accessor: DDR Controller LP Interface Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cic_status1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cic_status1`]
module"]
#[doc(alias = "CIC_STATUS1")]
pub type CicStatus1 = crate::Reg<cic_status1::CicStatus1Spec>;
#[doc = "DDR Controller LP Interface Status Register 1"]
pub mod cic_status1;
#[doc = "CIC_CTRL2 (rw) register accessor: DDR Controller LP Interface Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cic_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cic_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cic_ctrl2`]
module"]
#[doc(alias = "CIC_CTRL2")]
pub type CicCtrl2 = crate::Reg<cic_ctrl2::CicCtrl2Spec>;
#[doc = "DDR Controller LP Interface Control Register 2"]
pub mod cic_ctrl2;
#[doc = "CIC_CTRL3 (rw) register accessor: DDR Controller LP Interface Control Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cic_ctrl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cic_ctrl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cic_ctrl3`]
module"]
#[doc(alias = "CIC_CTRL3")]
pub type CicCtrl3 = crate::Reg<cic_ctrl3::CicCtrl3Spec>;
#[doc = "DDR Controller LP Interface Control Register 3"]
pub mod cic_ctrl3;
#[doc = "CIC_CTRL4 (rw) register accessor: DDR Controller LP Interface Control Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cic_ctrl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cic_ctrl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cic_ctrl4`]
module"]
#[doc(alias = "CIC_CTRL4")]
pub type CicCtrl4 = crate::Reg<cic_ctrl4::CicCtrl4Spec>;
#[doc = "DDR Controller LP Interface Control Register 4"]
pub mod cic_ctrl4;
#[doc = "CIC_STATUS2 (r) register accessor: DDR Controller LP Interface Status Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cic_status2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cic_status2`]
module"]
#[doc(alias = "CIC_STATUS2")]
pub type CicStatus2 = crate::Reg<cic_status2::CicStatus2Spec>;
#[doc = "DDR Controller LP Interface Status Register 2"]
pub mod cic_status2;
