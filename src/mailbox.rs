#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    a2b_inten: A2bInten,
    a2b_status: A2bStatus,
    a2b_cmd_0: A2bCmd0,
    a2b_dat_0: A2bDat0,
    a2b_cmd_1: A2bCmd1,
    a2b_dat_1: A2bDat1,
    a2b_cmd_2: A2bCmd2,
    a2b_dat_2: A2bDat2,
    a2b_cmd_3: A2bCmd3,
    a2b_dat_3: A2bDat3,
    b2a_inten: B2aInten,
    b2a_status: B2aStatus,
    b2a_cmd_0: B2aCmd0,
    b2a_dat_0: B2aDat0,
    b2a_cmd_1: B2aCmd1,
    b2a_dat_1: B2aDat1,
    b2a_cmd_2: B2aCmd2,
    b2a_dat_2: B2aDat2,
    b2a_cmd_3: B2aCmd3,
    b2a_dat_3: B2aDat3,
    _reserved20: [u8; 0xb0],
    atomic_lock_00: AtomicLock00,
    atomic_lock_01: AtomicLock01,
    atomic_lock_02: AtomicLock02,
    atomic_lock_03: AtomicLock03,
    atomic_lock_04: AtomicLock04,
    atomic_lock_05: AtomicLock05,
    atomic_lock_06: AtomicLock06,
    atomic_lock_07: AtomicLock07,
    atomic_lock_08: AtomicLock08,
    atomic_lock_09: AtomicLock09,
    atomic_lock_10: AtomicLock10,
    atomic_lock_11: AtomicLock11,
    atomic_lock_12: AtomicLock12,
    atomic_lock_13: AtomicLock13,
    atomic_lock_14: AtomicLock14,
    atomic_lock_15: AtomicLock15,
    atomic_lock_16: AtomicLock16,
    atomic_lock_17: AtomicLock17,
    atomic_lock_18: AtomicLock18,
    atomic_lock_19: AtomicLock19,
    atomic_lock_20: AtomicLock20,
    atomic_lock_21: AtomicLock21,
    atomic_lock_22: AtomicLock22,
    atomic_lock_23: AtomicLock23,
    atomic_lock_24: AtomicLock24,
    atomic_lock_25: AtomicLock25,
    atomic_lock_26: AtomicLock26,
    atomic_lock_27: AtomicLock27,
    atomic_lock_28: AtomicLock28,
    atomic_lock_29: AtomicLock29,
    atomic_lock_30: AtomicLock30,
    atomic_lock_31: AtomicLock31,
}
impl RegisterBlock {
    #[doc = "0x00 - Cortex-A53/Cortex-A72 to Cortex-M0 interrupt enable register"]
    #[inline(always)]
    pub const fn a2b_inten(&self) -> &A2bInten {
        &self.a2b_inten
    }
    #[doc = "0x04 - Cortex-A53/Cortex-A72 to Cortex-M0 interrupt status register"]
    #[inline(always)]
    pub const fn a2b_status(&self) -> &A2bStatus {
        &self.a2b_status
    }
    #[doc = "0x08 - Cortex-A53/Cortex-A72 to Cortex-M0 command 0"]
    #[inline(always)]
    pub const fn a2b_cmd_0(&self) -> &A2bCmd0 {
        &self.a2b_cmd_0
    }
    #[doc = "0x0c - Cortex-A53/Cortex-A72 to Cortex-M0 data 0"]
    #[inline(always)]
    pub const fn a2b_dat_0(&self) -> &A2bDat0 {
        &self.a2b_dat_0
    }
    #[doc = "0x10 - Cortex-A53/Cortex-A72 to Cortex-M0 command 1"]
    #[inline(always)]
    pub const fn a2b_cmd_1(&self) -> &A2bCmd1 {
        &self.a2b_cmd_1
    }
    #[doc = "0x14 - Cortex-A53/Cortex-A72 to Cortex-M0 data 1"]
    #[inline(always)]
    pub const fn a2b_dat_1(&self) -> &A2bDat1 {
        &self.a2b_dat_1
    }
    #[doc = "0x18 - Cortex-A53/Cortex-A72 to Cortex-M0 command 2"]
    #[inline(always)]
    pub const fn a2b_cmd_2(&self) -> &A2bCmd2 {
        &self.a2b_cmd_2
    }
    #[doc = "0x1c - Cortex-A53/Cortex-A72 to Cortex-M0 data 2"]
    #[inline(always)]
    pub const fn a2b_dat_2(&self) -> &A2bDat2 {
        &self.a2b_dat_2
    }
    #[doc = "0x20 - Cortex-A53/Cortex-A72 to Cortex-M0 command 3"]
    #[inline(always)]
    pub const fn a2b_cmd_3(&self) -> &A2bCmd3 {
        &self.a2b_cmd_3
    }
    #[doc = "0x24 - Cortex-A53/Cortex-A72 to Cortex-M0 data 3"]
    #[inline(always)]
    pub const fn a2b_dat_3(&self) -> &A2bDat3 {
        &self.a2b_dat_3
    }
    #[doc = "0x28 - Cortex-A53/Cortex-A72 to Cortex-M0 interrupt enable register"]
    #[inline(always)]
    pub const fn b2a_inten(&self) -> &B2aInten {
        &self.b2a_inten
    }
    #[doc = "0x2c - Cortex-M0 to Cortex-A53/Cortex-A72 interrupt status register"]
    #[inline(always)]
    pub const fn b2a_status(&self) -> &B2aStatus {
        &self.b2a_status
    }
    #[doc = "0x30 - Cortex-M0 to Cortex-A53/Cortex-A72 command 0"]
    #[inline(always)]
    pub const fn b2a_cmd_0(&self) -> &B2aCmd0 {
        &self.b2a_cmd_0
    }
    #[doc = "0x34 - Cortex-M0 to Cortex-A53/Cortex-A72 data 0"]
    #[inline(always)]
    pub const fn b2a_dat_0(&self) -> &B2aDat0 {
        &self.b2a_dat_0
    }
    #[doc = "0x38 - Cortex-M0 to Cortex-A53/Cortex-A72 command 1"]
    #[inline(always)]
    pub const fn b2a_cmd_1(&self) -> &B2aCmd1 {
        &self.b2a_cmd_1
    }
    #[doc = "0x3c - Cortex-M0 to Cortex-A53/Cortex-A72 data 1"]
    #[inline(always)]
    pub const fn b2a_dat_1(&self) -> &B2aDat1 {
        &self.b2a_dat_1
    }
    #[doc = "0x40 - Cortex-M0 to Cortex-A53/Cortex-A72 command 2"]
    #[inline(always)]
    pub const fn b2a_cmd_2(&self) -> &B2aCmd2 {
        &self.b2a_cmd_2
    }
    #[doc = "0x44 - Cortex-M0 to Cortex-A53/Cortex-A72 data 2"]
    #[inline(always)]
    pub const fn b2a_dat_2(&self) -> &B2aDat2 {
        &self.b2a_dat_2
    }
    #[doc = "0x48 - Cortex-M0 to Cortex-A53/Cortex-A72 command 3"]
    #[inline(always)]
    pub const fn b2a_cmd_3(&self) -> &B2aCmd3 {
        &self.b2a_cmd_3
    }
    #[doc = "0x4c - Cortex-M0 to Cortex-A53/Cortex-A72 data 3"]
    #[inline(always)]
    pub const fn b2a_dat_3(&self) -> &B2aDat3 {
        &self.b2a_dat_3
    }
    #[doc = "0x100 - Lock flag register 00"]
    #[inline(always)]
    pub const fn atomic_lock_00(&self) -> &AtomicLock00 {
        &self.atomic_lock_00
    }
    #[doc = "0x104 - Lock flag register 01"]
    #[inline(always)]
    pub const fn atomic_lock_01(&self) -> &AtomicLock01 {
        &self.atomic_lock_01
    }
    #[doc = "0x108 - Lock flag register 02"]
    #[inline(always)]
    pub const fn atomic_lock_02(&self) -> &AtomicLock02 {
        &self.atomic_lock_02
    }
    #[doc = "0x10c - Lock flag register 03"]
    #[inline(always)]
    pub const fn atomic_lock_03(&self) -> &AtomicLock03 {
        &self.atomic_lock_03
    }
    #[doc = "0x110 - Lock flag register 04"]
    #[inline(always)]
    pub const fn atomic_lock_04(&self) -> &AtomicLock04 {
        &self.atomic_lock_04
    }
    #[doc = "0x114 - Lock flag register 05"]
    #[inline(always)]
    pub const fn atomic_lock_05(&self) -> &AtomicLock05 {
        &self.atomic_lock_05
    }
    #[doc = "0x118 - Lock flag register 06"]
    #[inline(always)]
    pub const fn atomic_lock_06(&self) -> &AtomicLock06 {
        &self.atomic_lock_06
    }
    #[doc = "0x11c - Lock flag register 07"]
    #[inline(always)]
    pub const fn atomic_lock_07(&self) -> &AtomicLock07 {
        &self.atomic_lock_07
    }
    #[doc = "0x120 - Lock flag register 08"]
    #[inline(always)]
    pub const fn atomic_lock_08(&self) -> &AtomicLock08 {
        &self.atomic_lock_08
    }
    #[doc = "0x124 - Lock flag register 09"]
    #[inline(always)]
    pub const fn atomic_lock_09(&self) -> &AtomicLock09 {
        &self.atomic_lock_09
    }
    #[doc = "0x128 - Lock flag register 10"]
    #[inline(always)]
    pub const fn atomic_lock_10(&self) -> &AtomicLock10 {
        &self.atomic_lock_10
    }
    #[doc = "0x12c - Lock flag register 11"]
    #[inline(always)]
    pub const fn atomic_lock_11(&self) -> &AtomicLock11 {
        &self.atomic_lock_11
    }
    #[doc = "0x130 - Lock flag register 12"]
    #[inline(always)]
    pub const fn atomic_lock_12(&self) -> &AtomicLock12 {
        &self.atomic_lock_12
    }
    #[doc = "0x134 - Lock flag register 13"]
    #[inline(always)]
    pub const fn atomic_lock_13(&self) -> &AtomicLock13 {
        &self.atomic_lock_13
    }
    #[doc = "0x138 - Lock flag register 14"]
    #[inline(always)]
    pub const fn atomic_lock_14(&self) -> &AtomicLock14 {
        &self.atomic_lock_14
    }
    #[doc = "0x13c - Lock flag register 15"]
    #[inline(always)]
    pub const fn atomic_lock_15(&self) -> &AtomicLock15 {
        &self.atomic_lock_15
    }
    #[doc = "0x140 - Lock flag register 16"]
    #[inline(always)]
    pub const fn atomic_lock_16(&self) -> &AtomicLock16 {
        &self.atomic_lock_16
    }
    #[doc = "0x144 - Lock flag register 17"]
    #[inline(always)]
    pub const fn atomic_lock_17(&self) -> &AtomicLock17 {
        &self.atomic_lock_17
    }
    #[doc = "0x148 - Lock flag register 18"]
    #[inline(always)]
    pub const fn atomic_lock_18(&self) -> &AtomicLock18 {
        &self.atomic_lock_18
    }
    #[doc = "0x14c - Lock flag register 19"]
    #[inline(always)]
    pub const fn atomic_lock_19(&self) -> &AtomicLock19 {
        &self.atomic_lock_19
    }
    #[doc = "0x150 - Lock flag register 20"]
    #[inline(always)]
    pub const fn atomic_lock_20(&self) -> &AtomicLock20 {
        &self.atomic_lock_20
    }
    #[doc = "0x154 - Lock flag register 21"]
    #[inline(always)]
    pub const fn atomic_lock_21(&self) -> &AtomicLock21 {
        &self.atomic_lock_21
    }
    #[doc = "0x158 - Lock flag register 22"]
    #[inline(always)]
    pub const fn atomic_lock_22(&self) -> &AtomicLock22 {
        &self.atomic_lock_22
    }
    #[doc = "0x15c - Lock flag register 23"]
    #[inline(always)]
    pub const fn atomic_lock_23(&self) -> &AtomicLock23 {
        &self.atomic_lock_23
    }
    #[doc = "0x160 - Lock flag register 24"]
    #[inline(always)]
    pub const fn atomic_lock_24(&self) -> &AtomicLock24 {
        &self.atomic_lock_24
    }
    #[doc = "0x164 - Lock flag register 25"]
    #[inline(always)]
    pub const fn atomic_lock_25(&self) -> &AtomicLock25 {
        &self.atomic_lock_25
    }
    #[doc = "0x168 - Lock flag register 26"]
    #[inline(always)]
    pub const fn atomic_lock_26(&self) -> &AtomicLock26 {
        &self.atomic_lock_26
    }
    #[doc = "0x16c - Lock flag register 27"]
    #[inline(always)]
    pub const fn atomic_lock_27(&self) -> &AtomicLock27 {
        &self.atomic_lock_27
    }
    #[doc = "0x170 - Lock flag register 28"]
    #[inline(always)]
    pub const fn atomic_lock_28(&self) -> &AtomicLock28 {
        &self.atomic_lock_28
    }
    #[doc = "0x174 - Lock flag register 29"]
    #[inline(always)]
    pub const fn atomic_lock_29(&self) -> &AtomicLock29 {
        &self.atomic_lock_29
    }
    #[doc = "0x178 - Lock flag register 30"]
    #[inline(always)]
    pub const fn atomic_lock_30(&self) -> &AtomicLock30 {
        &self.atomic_lock_30
    }
    #[doc = "0x17c - Lock flag register 31"]
    #[inline(always)]
    pub const fn atomic_lock_31(&self) -> &AtomicLock31 {
        &self.atomic_lock_31
    }
}
#[doc = "A2B_INTEN (rw) register accessor: Cortex-A53/Cortex-A72 to Cortex-M0 interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a2b_inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a2b_inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a2b_inten`]
module"]
#[doc(alias = "A2B_INTEN")]
pub type A2bInten = crate::Reg<a2b_inten::A2bIntenSpec>;
#[doc = "Cortex-A53/Cortex-A72 to Cortex-M0 interrupt enable register"]
pub mod a2b_inten;
#[doc = "A2B_STATUS (rw) register accessor: Cortex-A53/Cortex-A72 to Cortex-M0 interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a2b_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a2b_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a2b_status`]
module"]
#[doc(alias = "A2B_STATUS")]
pub type A2bStatus = crate::Reg<a2b_status::A2bStatusSpec>;
#[doc = "Cortex-A53/Cortex-A72 to Cortex-M0 interrupt status register"]
pub mod a2b_status;
#[doc = "A2B_CMD_0 (rw) register accessor: Cortex-A53/Cortex-A72 to Cortex-M0 command 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a2b_cmd_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a2b_cmd_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a2b_cmd_0`]
module"]
#[doc(alias = "A2B_CMD_0")]
pub type A2bCmd0 = crate::Reg<a2b_cmd_0::A2bCmd0Spec>;
#[doc = "Cortex-A53/Cortex-A72 to Cortex-M0 command 0"]
pub mod a2b_cmd_0;
#[doc = "A2B_DAT_0 (rw) register accessor: Cortex-A53/Cortex-A72 to Cortex-M0 data 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a2b_dat_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a2b_dat_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a2b_dat_0`]
module"]
#[doc(alias = "A2B_DAT_0")]
pub type A2bDat0 = crate::Reg<a2b_dat_0::A2bDat0Spec>;
#[doc = "Cortex-A53/Cortex-A72 to Cortex-M0 data 0"]
pub mod a2b_dat_0;
#[doc = "A2B_CMD_1 (rw) register accessor: Cortex-A53/Cortex-A72 to Cortex-M0 command 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a2b_cmd_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a2b_cmd_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a2b_cmd_1`]
module"]
#[doc(alias = "A2B_CMD_1")]
pub type A2bCmd1 = crate::Reg<a2b_cmd_1::A2bCmd1Spec>;
#[doc = "Cortex-A53/Cortex-A72 to Cortex-M0 command 1"]
pub mod a2b_cmd_1;
#[doc = "A2B_DAT_1 (rw) register accessor: Cortex-A53/Cortex-A72 to Cortex-M0 data 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a2b_dat_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a2b_dat_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a2b_dat_1`]
module"]
#[doc(alias = "A2B_DAT_1")]
pub type A2bDat1 = crate::Reg<a2b_dat_1::A2bDat1Spec>;
#[doc = "Cortex-A53/Cortex-A72 to Cortex-M0 data 1"]
pub mod a2b_dat_1;
#[doc = "A2B_CMD_2 (rw) register accessor: Cortex-A53/Cortex-A72 to Cortex-M0 command 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a2b_cmd_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a2b_cmd_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a2b_cmd_2`]
module"]
#[doc(alias = "A2B_CMD_2")]
pub type A2bCmd2 = crate::Reg<a2b_cmd_2::A2bCmd2Spec>;
#[doc = "Cortex-A53/Cortex-A72 to Cortex-M0 command 2"]
pub mod a2b_cmd_2;
#[doc = "A2B_DAT_2 (rw) register accessor: Cortex-A53/Cortex-A72 to Cortex-M0 data 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a2b_dat_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a2b_dat_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a2b_dat_2`]
module"]
#[doc(alias = "A2B_DAT_2")]
pub type A2bDat2 = crate::Reg<a2b_dat_2::A2bDat2Spec>;
#[doc = "Cortex-A53/Cortex-A72 to Cortex-M0 data 2"]
pub mod a2b_dat_2;
#[doc = "A2B_CMD_3 (rw) register accessor: Cortex-A53/Cortex-A72 to Cortex-M0 command 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a2b_cmd_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a2b_cmd_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a2b_cmd_3`]
module"]
#[doc(alias = "A2B_CMD_3")]
pub type A2bCmd3 = crate::Reg<a2b_cmd_3::A2bCmd3Spec>;
#[doc = "Cortex-A53/Cortex-A72 to Cortex-M0 command 3"]
pub mod a2b_cmd_3;
#[doc = "A2B_DAT_3 (rw) register accessor: Cortex-A53/Cortex-A72 to Cortex-M0 data 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a2b_dat_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a2b_dat_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a2b_dat_3`]
module"]
#[doc(alias = "A2B_DAT_3")]
pub type A2bDat3 = crate::Reg<a2b_dat_3::A2bDat3Spec>;
#[doc = "Cortex-A53/Cortex-A72 to Cortex-M0 data 3"]
pub mod a2b_dat_3;
#[doc = "B2A_INTEN (rw) register accessor: Cortex-A53/Cortex-A72 to Cortex-M0 interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b2a_inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b2a_inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b2a_inten`]
module"]
#[doc(alias = "B2A_INTEN")]
pub type B2aInten = crate::Reg<b2a_inten::B2aIntenSpec>;
#[doc = "Cortex-A53/Cortex-A72 to Cortex-M0 interrupt enable register"]
pub mod b2a_inten;
#[doc = "B2A_STATUS (rw) register accessor: Cortex-M0 to Cortex-A53/Cortex-A72 interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b2a_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b2a_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b2a_status`]
module"]
#[doc(alias = "B2A_STATUS")]
pub type B2aStatus = crate::Reg<b2a_status::B2aStatusSpec>;
#[doc = "Cortex-M0 to Cortex-A53/Cortex-A72 interrupt status register"]
pub mod b2a_status;
#[doc = "B2A_CMD_0 (rw) register accessor: Cortex-M0 to Cortex-A53/Cortex-A72 command 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b2a_cmd_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b2a_cmd_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b2a_cmd_0`]
module"]
#[doc(alias = "B2A_CMD_0")]
pub type B2aCmd0 = crate::Reg<b2a_cmd_0::B2aCmd0Spec>;
#[doc = "Cortex-M0 to Cortex-A53/Cortex-A72 command 0"]
pub mod b2a_cmd_0;
#[doc = "B2A_DAT_0 (rw) register accessor: Cortex-M0 to Cortex-A53/Cortex-A72 data 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b2a_dat_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b2a_dat_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b2a_dat_0`]
module"]
#[doc(alias = "B2A_DAT_0")]
pub type B2aDat0 = crate::Reg<b2a_dat_0::B2aDat0Spec>;
#[doc = "Cortex-M0 to Cortex-A53/Cortex-A72 data 0"]
pub mod b2a_dat_0;
#[doc = "B2A_CMD_1 (rw) register accessor: Cortex-M0 to Cortex-A53/Cortex-A72 command 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b2a_cmd_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b2a_cmd_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b2a_cmd_1`]
module"]
#[doc(alias = "B2A_CMD_1")]
pub type B2aCmd1 = crate::Reg<b2a_cmd_1::B2aCmd1Spec>;
#[doc = "Cortex-M0 to Cortex-A53/Cortex-A72 command 1"]
pub mod b2a_cmd_1;
#[doc = "B2A_DAT_1 (rw) register accessor: Cortex-M0 to Cortex-A53/Cortex-A72 data 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b2a_dat_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b2a_dat_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b2a_dat_1`]
module"]
#[doc(alias = "B2A_DAT_1")]
pub type B2aDat1 = crate::Reg<b2a_dat_1::B2aDat1Spec>;
#[doc = "Cortex-M0 to Cortex-A53/Cortex-A72 data 1"]
pub mod b2a_dat_1;
#[doc = "B2A_CMD_2 (rw) register accessor: Cortex-M0 to Cortex-A53/Cortex-A72 command 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b2a_cmd_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b2a_cmd_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b2a_cmd_2`]
module"]
#[doc(alias = "B2A_CMD_2")]
pub type B2aCmd2 = crate::Reg<b2a_cmd_2::B2aCmd2Spec>;
#[doc = "Cortex-M0 to Cortex-A53/Cortex-A72 command 2"]
pub mod b2a_cmd_2;
#[doc = "B2A_DAT_2 (rw) register accessor: Cortex-M0 to Cortex-A53/Cortex-A72 data 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b2a_dat_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b2a_dat_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b2a_dat_2`]
module"]
#[doc(alias = "B2A_DAT_2")]
pub type B2aDat2 = crate::Reg<b2a_dat_2::B2aDat2Spec>;
#[doc = "Cortex-M0 to Cortex-A53/Cortex-A72 data 2"]
pub mod b2a_dat_2;
#[doc = "B2A_CMD_3 (rw) register accessor: Cortex-M0 to Cortex-A53/Cortex-A72 command 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b2a_cmd_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b2a_cmd_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b2a_cmd_3`]
module"]
#[doc(alias = "B2A_CMD_3")]
pub type B2aCmd3 = crate::Reg<b2a_cmd_3::B2aCmd3Spec>;
#[doc = "Cortex-M0 to Cortex-A53/Cortex-A72 command 3"]
pub mod b2a_cmd_3;
#[doc = "B2A_DAT_3 (rw) register accessor: Cortex-M0 to Cortex-A53/Cortex-A72 data 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b2a_dat_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b2a_dat_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b2a_dat_3`]
module"]
#[doc(alias = "B2A_DAT_3")]
pub type B2aDat3 = crate::Reg<b2a_dat_3::B2aDat3Spec>;
#[doc = "Cortex-M0 to Cortex-A53/Cortex-A72 data 3"]
pub mod b2a_dat_3;
#[doc = "ATOMIC_LOCK_00 (rw) register accessor: Lock flag register 00\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_00::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_00::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atomic_lock_00`]
module"]
#[doc(alias = "ATOMIC_LOCK_00")]
pub type AtomicLock00 = crate::Reg<atomic_lock_00::AtomicLock00Spec>;
#[doc = "Lock flag register 00"]
pub mod atomic_lock_00;
#[doc = "ATOMIC_LOCK_01 (rw) register accessor: Lock flag register 01\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atomic_lock_01`]
module"]
#[doc(alias = "ATOMIC_LOCK_01")]
pub type AtomicLock01 = crate::Reg<atomic_lock_01::AtomicLock01Spec>;
#[doc = "Lock flag register 01"]
pub mod atomic_lock_01;
#[doc = "ATOMIC_LOCK_02 (rw) register accessor: Lock flag register 02\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_02::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_02::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atomic_lock_02`]
module"]
#[doc(alias = "ATOMIC_LOCK_02")]
pub type AtomicLock02 = crate::Reg<atomic_lock_02::AtomicLock02Spec>;
#[doc = "Lock flag register 02"]
pub mod atomic_lock_02;
#[doc = "ATOMIC_LOCK_03 (rw) register accessor: Lock flag register 03\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_03::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_03::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atomic_lock_03`]
module"]
#[doc(alias = "ATOMIC_LOCK_03")]
pub type AtomicLock03 = crate::Reg<atomic_lock_03::AtomicLock03Spec>;
#[doc = "Lock flag register 03"]
pub mod atomic_lock_03;
#[doc = "ATOMIC_LOCK_04 (rw) register accessor: Lock flag register 04\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_04::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_04::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atomic_lock_04`]
module"]
#[doc(alias = "ATOMIC_LOCK_04")]
pub type AtomicLock04 = crate::Reg<atomic_lock_04::AtomicLock04Spec>;
#[doc = "Lock flag register 04"]
pub mod atomic_lock_04;
#[doc = "ATOMIC_LOCK_05 (rw) register accessor: Lock flag register 05\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_05::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_05::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atomic_lock_05`]
module"]
#[doc(alias = "ATOMIC_LOCK_05")]
pub type AtomicLock05 = crate::Reg<atomic_lock_05::AtomicLock05Spec>;
#[doc = "Lock flag register 05"]
pub mod atomic_lock_05;
#[doc = "ATOMIC_LOCK_06 (rw) register accessor: Lock flag register 06\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_06::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_06::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atomic_lock_06`]
module"]
#[doc(alias = "ATOMIC_LOCK_06")]
pub type AtomicLock06 = crate::Reg<atomic_lock_06::AtomicLock06Spec>;
#[doc = "Lock flag register 06"]
pub mod atomic_lock_06;
#[doc = "ATOMIC_LOCK_07 (rw) register accessor: Lock flag register 07\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_07::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_07::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atomic_lock_07`]
module"]
#[doc(alias = "ATOMIC_LOCK_07")]
pub type AtomicLock07 = crate::Reg<atomic_lock_07::AtomicLock07Spec>;
#[doc = "Lock flag register 07"]
pub mod atomic_lock_07;
#[doc = "ATOMIC_LOCK_08 (rw) register accessor: Lock flag register 08\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_08::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_08::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atomic_lock_08`]
module"]
#[doc(alias = "ATOMIC_LOCK_08")]
pub type AtomicLock08 = crate::Reg<atomic_lock_08::AtomicLock08Spec>;
#[doc = "Lock flag register 08"]
pub mod atomic_lock_08;
#[doc = "ATOMIC_LOCK_09 (rw) register accessor: Lock flag register 09\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_09::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_09::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atomic_lock_09`]
module"]
#[doc(alias = "ATOMIC_LOCK_09")]
pub type AtomicLock09 = crate::Reg<atomic_lock_09::AtomicLock09Spec>;
#[doc = "Lock flag register 09"]
pub mod atomic_lock_09;
#[doc = "ATOMIC_LOCK_10 (rw) register accessor: Lock flag register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atomic_lock_10`]
module"]
#[doc(alias = "ATOMIC_LOCK_10")]
pub type AtomicLock10 = crate::Reg<atomic_lock_10::AtomicLock10Spec>;
#[doc = "Lock flag register 10"]
pub mod atomic_lock_10;
#[doc = "ATOMIC_LOCK_11 (rw) register accessor: Lock flag register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atomic_lock_11`]
module"]
#[doc(alias = "ATOMIC_LOCK_11")]
pub type AtomicLock11 = crate::Reg<atomic_lock_11::AtomicLock11Spec>;
#[doc = "Lock flag register 11"]
pub mod atomic_lock_11;
#[doc = "ATOMIC_LOCK_12 (rw) register accessor: Lock flag register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atomic_lock_12`]
module"]
#[doc(alias = "ATOMIC_LOCK_12")]
pub type AtomicLock12 = crate::Reg<atomic_lock_12::AtomicLock12Spec>;
#[doc = "Lock flag register 12"]
pub mod atomic_lock_12;
#[doc = "ATOMIC_LOCK_13 (rw) register accessor: Lock flag register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atomic_lock_13`]
module"]
#[doc(alias = "ATOMIC_LOCK_13")]
pub type AtomicLock13 = crate::Reg<atomic_lock_13::AtomicLock13Spec>;
#[doc = "Lock flag register 13"]
pub mod atomic_lock_13;
#[doc = "ATOMIC_LOCK_14 (rw) register accessor: Lock flag register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atomic_lock_14`]
module"]
#[doc(alias = "ATOMIC_LOCK_14")]
pub type AtomicLock14 = crate::Reg<atomic_lock_14::AtomicLock14Spec>;
#[doc = "Lock flag register 14"]
pub mod atomic_lock_14;
#[doc = "ATOMIC_LOCK_15 (rw) register accessor: Lock flag register 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atomic_lock_15`]
module"]
#[doc(alias = "ATOMIC_LOCK_15")]
pub type AtomicLock15 = crate::Reg<atomic_lock_15::AtomicLock15Spec>;
#[doc = "Lock flag register 15"]
pub mod atomic_lock_15;
#[doc = "ATOMIC_LOCK_16 (rw) register accessor: Lock flag register 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atomic_lock_16`]
module"]
#[doc(alias = "ATOMIC_LOCK_16")]
pub type AtomicLock16 = crate::Reg<atomic_lock_16::AtomicLock16Spec>;
#[doc = "Lock flag register 16"]
pub mod atomic_lock_16;
#[doc = "ATOMIC_LOCK_17 (rw) register accessor: Lock flag register 17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atomic_lock_17`]
module"]
#[doc(alias = "ATOMIC_LOCK_17")]
pub type AtomicLock17 = crate::Reg<atomic_lock_17::AtomicLock17Spec>;
#[doc = "Lock flag register 17"]
pub mod atomic_lock_17;
#[doc = "ATOMIC_LOCK_18 (rw) register accessor: Lock flag register 18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atomic_lock_18`]
module"]
#[doc(alias = "ATOMIC_LOCK_18")]
pub type AtomicLock18 = crate::Reg<atomic_lock_18::AtomicLock18Spec>;
#[doc = "Lock flag register 18"]
pub mod atomic_lock_18;
#[doc = "ATOMIC_LOCK_19 (rw) register accessor: Lock flag register 19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atomic_lock_19`]
module"]
#[doc(alias = "ATOMIC_LOCK_19")]
pub type AtomicLock19 = crate::Reg<atomic_lock_19::AtomicLock19Spec>;
#[doc = "Lock flag register 19"]
pub mod atomic_lock_19;
#[doc = "ATOMIC_LOCK_20 (rw) register accessor: Lock flag register 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atomic_lock_20`]
module"]
#[doc(alias = "ATOMIC_LOCK_20")]
pub type AtomicLock20 = crate::Reg<atomic_lock_20::AtomicLock20Spec>;
#[doc = "Lock flag register 20"]
pub mod atomic_lock_20;
#[doc = "ATOMIC_LOCK_21 (rw) register accessor: Lock flag register 21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atomic_lock_21`]
module"]
#[doc(alias = "ATOMIC_LOCK_21")]
pub type AtomicLock21 = crate::Reg<atomic_lock_21::AtomicLock21Spec>;
#[doc = "Lock flag register 21"]
pub mod atomic_lock_21;
#[doc = "ATOMIC_LOCK_22 (rw) register accessor: Lock flag register 22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atomic_lock_22`]
module"]
#[doc(alias = "ATOMIC_LOCK_22")]
pub type AtomicLock22 = crate::Reg<atomic_lock_22::AtomicLock22Spec>;
#[doc = "Lock flag register 22"]
pub mod atomic_lock_22;
#[doc = "ATOMIC_LOCK_23 (rw) register accessor: Lock flag register 23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atomic_lock_23`]
module"]
#[doc(alias = "ATOMIC_LOCK_23")]
pub type AtomicLock23 = crate::Reg<atomic_lock_23::AtomicLock23Spec>;
#[doc = "Lock flag register 23"]
pub mod atomic_lock_23;
#[doc = "ATOMIC_LOCK_24 (rw) register accessor: Lock flag register 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atomic_lock_24`]
module"]
#[doc(alias = "ATOMIC_LOCK_24")]
pub type AtomicLock24 = crate::Reg<atomic_lock_24::AtomicLock24Spec>;
#[doc = "Lock flag register 24"]
pub mod atomic_lock_24;
#[doc = "ATOMIC_LOCK_25 (rw) register accessor: Lock flag register 25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atomic_lock_25`]
module"]
#[doc(alias = "ATOMIC_LOCK_25")]
pub type AtomicLock25 = crate::Reg<atomic_lock_25::AtomicLock25Spec>;
#[doc = "Lock flag register 25"]
pub mod atomic_lock_25;
#[doc = "ATOMIC_LOCK_26 (rw) register accessor: Lock flag register 26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atomic_lock_26`]
module"]
#[doc(alias = "ATOMIC_LOCK_26")]
pub type AtomicLock26 = crate::Reg<atomic_lock_26::AtomicLock26Spec>;
#[doc = "Lock flag register 26"]
pub mod atomic_lock_26;
#[doc = "ATOMIC_LOCK_27 (rw) register accessor: Lock flag register 27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atomic_lock_27`]
module"]
#[doc(alias = "ATOMIC_LOCK_27")]
pub type AtomicLock27 = crate::Reg<atomic_lock_27::AtomicLock27Spec>;
#[doc = "Lock flag register 27"]
pub mod atomic_lock_27;
#[doc = "ATOMIC_LOCK_28 (rw) register accessor: Lock flag register 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atomic_lock_28`]
module"]
#[doc(alias = "ATOMIC_LOCK_28")]
pub type AtomicLock28 = crate::Reg<atomic_lock_28::AtomicLock28Spec>;
#[doc = "Lock flag register 28"]
pub mod atomic_lock_28;
#[doc = "ATOMIC_LOCK_29 (rw) register accessor: Lock flag register 29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atomic_lock_29`]
module"]
#[doc(alias = "ATOMIC_LOCK_29")]
pub type AtomicLock29 = crate::Reg<atomic_lock_29::AtomicLock29Spec>;
#[doc = "Lock flag register 29"]
pub mod atomic_lock_29;
#[doc = "ATOMIC_LOCK_30 (rw) register accessor: Lock flag register 30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atomic_lock_30`]
module"]
#[doc(alias = "ATOMIC_LOCK_30")]
pub type AtomicLock30 = crate::Reg<atomic_lock_30::AtomicLock30Spec>;
#[doc = "Lock flag register 30"]
pub mod atomic_lock_30;
#[doc = "ATOMIC_LOCK_31 (rw) register accessor: Lock flag register 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atomic_lock_31`]
module"]
#[doc(alias = "ATOMIC_LOCK_31")]
pub type AtomicLock31 = crate::Reg<atomic_lock_31::AtomicLock31Spec>;
#[doc = "Lock flag register 31"]
pub mod atomic_lock_31;
