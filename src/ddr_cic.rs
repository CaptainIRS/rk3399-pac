#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl0: Ctrl0,
    ctrl1: Ctrl1,
    idle_th: IdleTh,
    cg_wait_th: CgWaitTh,
    status0: Status0,
    status1: Status1,
    ctrl2: Ctrl2,
    ctrl3: Ctrl3,
    ctrl4: Ctrl4,
    _reserved9: [u8; 0x1c],
    status2: Status2,
}
impl RegisterBlock {
    #[doc = "0x00 - DDR Controller LP Interface Control Register 0"]
    #[inline(always)]
    pub const fn ctrl0(&self) -> &Ctrl0 {
        &self.ctrl0
    }
    #[doc = "0x04 - DDR Controller LP Interface Control Register 1"]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &Ctrl1 {
        &self.ctrl1
    }
    #[doc = "0x08 - DDR Controller LP Interface Idle Threshold in standby mode"]
    #[inline(always)]
    pub const fn idle_th(&self) -> &IdleTh {
        &self.idle_th
    }
    #[doc = "0x0c - DDR Controller LP Interface CG Wait Threshold in standby mode"]
    #[inline(always)]
    pub const fn cg_wait_th(&self) -> &CgWaitTh {
        &self.cg_wait_th
    }
    #[doc = "0x10 - DDR Controller LP Interface Status Register 0"]
    #[inline(always)]
    pub const fn status0(&self) -> &Status0 {
        &self.status0
    }
    #[doc = "0x14 - DDR Controller LP Interface Status Register 1"]
    #[inline(always)]
    pub const fn status1(&self) -> &Status1 {
        &self.status1
    }
    #[doc = "0x18 - DDR Controller LP Interface Control Register 2"]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &Ctrl2 {
        &self.ctrl2
    }
    #[doc = "0x1c - DDR Controller LP Interface Control Register 3"]
    #[inline(always)]
    pub const fn ctrl3(&self) -> &Ctrl3 {
        &self.ctrl3
    }
    #[doc = "0x20 - DDR Controller LP Interface Control Register 4"]
    #[inline(always)]
    pub const fn ctrl4(&self) -> &Ctrl4 {
        &self.ctrl4
    }
    #[doc = "0x40 - DDR Controller LP Interface Status Register 2"]
    #[inline(always)]
    pub const fn status2(&self) -> &Status2 {
        &self.status2
    }
}
#[doc = "CTRL0 (rw) register accessor: DDR Controller LP Interface Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl0`]
module"]
#[doc(alias = "CTRL0")]
pub type Ctrl0 = crate::Reg<ctrl0::Ctrl0Spec>;
#[doc = "DDR Controller LP Interface Control Register 0"]
pub mod ctrl0;
#[doc = "CTRL1 (rw) register accessor: DDR Controller LP Interface Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`]
module"]
#[doc(alias = "CTRL1")]
pub type Ctrl1 = crate::Reg<ctrl1::Ctrl1Spec>;
#[doc = "DDR Controller LP Interface Control Register 1"]
pub mod ctrl1;
#[doc = "IDLE_TH (rw) register accessor: DDR Controller LP Interface Idle Threshold in standby mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idle_th::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idle_th::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idle_th`]
module"]
#[doc(alias = "IDLE_TH")]
pub type IdleTh = crate::Reg<idle_th::IdleThSpec>;
#[doc = "DDR Controller LP Interface Idle Threshold in standby mode"]
pub mod idle_th;
#[doc = "CG_WAIT_TH (rw) register accessor: DDR Controller LP Interface CG Wait Threshold in standby mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cg_wait_th::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cg_wait_th::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cg_wait_th`]
module"]
#[doc(alias = "CG_WAIT_TH")]
pub type CgWaitTh = crate::Reg<cg_wait_th::CgWaitThSpec>;
#[doc = "DDR Controller LP Interface CG Wait Threshold in standby mode"]
pub mod cg_wait_th;
#[doc = "STATUS0 (r) register accessor: DDR Controller LP Interface Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status0`]
module"]
#[doc(alias = "STATUS0")]
pub type Status0 = crate::Reg<status0::Status0Spec>;
#[doc = "DDR Controller LP Interface Status Register 0"]
pub mod status0;
#[doc = "STATUS1 (r) register accessor: DDR Controller LP Interface Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status1`]
module"]
#[doc(alias = "STATUS1")]
pub type Status1 = crate::Reg<status1::Status1Spec>;
#[doc = "DDR Controller LP Interface Status Register 1"]
pub mod status1;
#[doc = "CTRL2 (rw) register accessor: DDR Controller LP Interface Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`]
module"]
#[doc(alias = "CTRL2")]
pub type Ctrl2 = crate::Reg<ctrl2::Ctrl2Spec>;
#[doc = "DDR Controller LP Interface Control Register 2"]
pub mod ctrl2;
#[doc = "CTRL3 (rw) register accessor: DDR Controller LP Interface Control Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl3`]
module"]
#[doc(alias = "CTRL3")]
pub type Ctrl3 = crate::Reg<ctrl3::Ctrl3Spec>;
#[doc = "DDR Controller LP Interface Control Register 3"]
pub mod ctrl3;
#[doc = "CTRL4 (rw) register accessor: DDR Controller LP Interface Control Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl4`]
module"]
#[doc(alias = "CTRL4")]
pub type Ctrl4 = crate::Reg<ctrl4::Ctrl4Spec>;
#[doc = "DDR Controller LP Interface Control Register 4"]
pub mod ctrl4;
#[doc = "STATUS2 (r) register accessor: DDR Controller LP Interface Status Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status2`]
module"]
#[doc(alias = "STATUS2")]
pub type Status2 = crate::Reg<status2::Status2Spec>;
#[doc = "DDR Controller LP Interface Status Register 2"]
pub mod status2;
