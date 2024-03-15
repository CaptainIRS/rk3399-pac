#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mailbox_a2b_inten: MailboxA2bInten,
    mailbox_a2b_status: MailboxA2bStatus,
    mailbox_a2b_cmd_0: MailboxA2bCmd0,
    mailbox_a2b_dat_0: MailboxA2bDat0,
    mailbox_a2b_cmd_1: MailboxA2bCmd1,
    mailbox_a2b_dat_1: MailboxA2bDat1,
    mailbox_a2b_cmd_2: MailboxA2bCmd2,
    mailbox_a2b_dat_2: MailboxA2bDat2,
    mailbox_a2b_cmd_3: MailboxA2bCmd3,
    mailbox_a2b_dat_3: MailboxA2bDat3,
    mailbox_b2a_inten: MailboxB2aInten,
    mailbox_b2a_status: MailboxB2aStatus,
    mailbox_b2a_cmd_0: MailboxB2aCmd0,
    mailbox_b2a_dat_0: MailboxB2aDat0,
    mailbox_b2a_cmd_1: MailboxB2aCmd1,
    mailbox_b2a_dat_1: MailboxB2aDat1,
    mailbox_b2a_cmd_2: MailboxB2aCmd2,
    mailbox_b2a_dat_2: MailboxB2aDat2,
    mailbox_b2a_cmd_3: MailboxB2aCmd3,
    mailbox_b2a_dat_3: MailboxB2aDat3,
    _reserved20: [u8; 0xb0],
    mailbox_atomic_lock_00: MailboxAtomicLock00,
    mailbox_atomic_lock_01: MailboxAtomicLock01,
    mailbox_atomic_lock_02: MailboxAtomicLock02,
    mailbox_atomic_lock_03: MailboxAtomicLock03,
    mailbox_atomic_lock_04: MailboxAtomicLock04,
    mailbox_atomic_lock_05: MailboxAtomicLock05,
    mailbox_atomic_lock_06: MailboxAtomicLock06,
    mailbox_atomic_lock_07: MailboxAtomicLock07,
    mailbox_atomic_lock_08: MailboxAtomicLock08,
    mailbox_atomic_lock_09: MailboxAtomicLock09,
    mailbox_atomic_lock_10: MailboxAtomicLock10,
    mailbox_atomic_lock_11: MailboxAtomicLock11,
    mailbox_atomic_lock_12: MailboxAtomicLock12,
    mailbox_atomic_lock_13: MailboxAtomicLock13,
    mailbox_atomic_lock_14: MailboxAtomicLock14,
    mailbox_atomic_lock_15: MailboxAtomicLock15,
    mailbox_atomic_lock_16: MailboxAtomicLock16,
    mailbox_atomic_lock_17: MailboxAtomicLock17,
    mailbox_atomic_lock_18: MailboxAtomicLock18,
    mailbox_atomic_lock_19: MailboxAtomicLock19,
    mailbox_atomic_lock_20: MailboxAtomicLock20,
    mailbox_atomic_lock_21: MailboxAtomicLock21,
    mailbox_atomic_lock_22: MailboxAtomicLock22,
    mailbox_atomic_lock_23: MailboxAtomicLock23,
    mailbox_atomic_lock_24: MailboxAtomicLock24,
    mailbox_atomic_lock_25: MailboxAtomicLock25,
    mailbox_atomic_lock_26: MailboxAtomicLock26,
    mailbox_atomic_lock_27: MailboxAtomicLock27,
    mailbox_atomic_lock_28: MailboxAtomicLock28,
    mailbox_atomic_lock_29: MailboxAtomicLock29,
    mailbox_atomic_lock_30: MailboxAtomicLock30,
    mailbox_atomic_lock_31: MailboxAtomicLock31,
}
impl RegisterBlock {
    #[doc = "0x00 - Cortex-A53/Cortex-A72 to Cortex-M0 interrupt enable register"]
    #[inline(always)]
    pub const fn mailbox_a2b_inten(&self) -> &MailboxA2bInten {
        &self.mailbox_a2b_inten
    }
    #[doc = "0x04 - Cortex-A53/Cortex-A72 to Cortex-M0 interrupt status register"]
    #[inline(always)]
    pub const fn mailbox_a2b_status(&self) -> &MailboxA2bStatus {
        &self.mailbox_a2b_status
    }
    #[doc = "0x08 - Cortex-A53/Cortex-A72 to Cortex-M0 command 0"]
    #[inline(always)]
    pub const fn mailbox_a2b_cmd_0(&self) -> &MailboxA2bCmd0 {
        &self.mailbox_a2b_cmd_0
    }
    #[doc = "0x0c - Cortex-A53/Cortex-A72 to Cortex-M0 data 0"]
    #[inline(always)]
    pub const fn mailbox_a2b_dat_0(&self) -> &MailboxA2bDat0 {
        &self.mailbox_a2b_dat_0
    }
    #[doc = "0x10 - Cortex-A53/Cortex-A72 to Cortex-M0 command 1"]
    #[inline(always)]
    pub const fn mailbox_a2b_cmd_1(&self) -> &MailboxA2bCmd1 {
        &self.mailbox_a2b_cmd_1
    }
    #[doc = "0x14 - Cortex-A53/Cortex-A72 to Cortex-M0 data 1"]
    #[inline(always)]
    pub const fn mailbox_a2b_dat_1(&self) -> &MailboxA2bDat1 {
        &self.mailbox_a2b_dat_1
    }
    #[doc = "0x18 - Cortex-A53/Cortex-A72 to Cortex-M0 command 2"]
    #[inline(always)]
    pub const fn mailbox_a2b_cmd_2(&self) -> &MailboxA2bCmd2 {
        &self.mailbox_a2b_cmd_2
    }
    #[doc = "0x1c - Cortex-A53/Cortex-A72 to Cortex-M0 data 2"]
    #[inline(always)]
    pub const fn mailbox_a2b_dat_2(&self) -> &MailboxA2bDat2 {
        &self.mailbox_a2b_dat_2
    }
    #[doc = "0x20 - Cortex-A53/Cortex-A72 to Cortex-M0 command 3"]
    #[inline(always)]
    pub const fn mailbox_a2b_cmd_3(&self) -> &MailboxA2bCmd3 {
        &self.mailbox_a2b_cmd_3
    }
    #[doc = "0x24 - Cortex-A53/Cortex-A72 to Cortex-M0 data 3"]
    #[inline(always)]
    pub const fn mailbox_a2b_dat_3(&self) -> &MailboxA2bDat3 {
        &self.mailbox_a2b_dat_3
    }
    #[doc = "0x28 - Cortex-A53/Cortex-A72 to Cortex-M0 interrupt enable register"]
    #[inline(always)]
    pub const fn mailbox_b2a_inten(&self) -> &MailboxB2aInten {
        &self.mailbox_b2a_inten
    }
    #[doc = "0x2c - Cortex-M0 to Cortex-A53/Cortex-A72 interrupt status register"]
    #[inline(always)]
    pub const fn mailbox_b2a_status(&self) -> &MailboxB2aStatus {
        &self.mailbox_b2a_status
    }
    #[doc = "0x30 - Cortex-M0 to Cortex-A53/Cortex-A72 command 0"]
    #[inline(always)]
    pub const fn mailbox_b2a_cmd_0(&self) -> &MailboxB2aCmd0 {
        &self.mailbox_b2a_cmd_0
    }
    #[doc = "0x34 - Cortex-M0 to Cortex-A53/Cortex-A72 data 0"]
    #[inline(always)]
    pub const fn mailbox_b2a_dat_0(&self) -> &MailboxB2aDat0 {
        &self.mailbox_b2a_dat_0
    }
    #[doc = "0x38 - Cortex-M0 to Cortex-A53/Cortex-A72 command 1"]
    #[inline(always)]
    pub const fn mailbox_b2a_cmd_1(&self) -> &MailboxB2aCmd1 {
        &self.mailbox_b2a_cmd_1
    }
    #[doc = "0x3c - Cortex-M0 to Cortex-A53/Cortex-A72 data 1"]
    #[inline(always)]
    pub const fn mailbox_b2a_dat_1(&self) -> &MailboxB2aDat1 {
        &self.mailbox_b2a_dat_1
    }
    #[doc = "0x40 - Cortex-M0 to Cortex-A53/Cortex-A72 command 2"]
    #[inline(always)]
    pub const fn mailbox_b2a_cmd_2(&self) -> &MailboxB2aCmd2 {
        &self.mailbox_b2a_cmd_2
    }
    #[doc = "0x44 - Cortex-M0 to Cortex-A53/Cortex-A72 data 2"]
    #[inline(always)]
    pub const fn mailbox_b2a_dat_2(&self) -> &MailboxB2aDat2 {
        &self.mailbox_b2a_dat_2
    }
    #[doc = "0x48 - Cortex-M0 to Cortex-A53/Cortex-A72 command 3"]
    #[inline(always)]
    pub const fn mailbox_b2a_cmd_3(&self) -> &MailboxB2aCmd3 {
        &self.mailbox_b2a_cmd_3
    }
    #[doc = "0x4c - Cortex-M0 to Cortex-A53/Cortex-A72 data 3"]
    #[inline(always)]
    pub const fn mailbox_b2a_dat_3(&self) -> &MailboxB2aDat3 {
        &self.mailbox_b2a_dat_3
    }
    #[doc = "0x100 - Lock flag register 00"]
    #[inline(always)]
    pub const fn mailbox_atomic_lock_00(&self) -> &MailboxAtomicLock00 {
        &self.mailbox_atomic_lock_00
    }
    #[doc = "0x104 - Lock flag register 01"]
    #[inline(always)]
    pub const fn mailbox_atomic_lock_01(&self) -> &MailboxAtomicLock01 {
        &self.mailbox_atomic_lock_01
    }
    #[doc = "0x108 - Lock flag register 02"]
    #[inline(always)]
    pub const fn mailbox_atomic_lock_02(&self) -> &MailboxAtomicLock02 {
        &self.mailbox_atomic_lock_02
    }
    #[doc = "0x10c - Lock flag register 03"]
    #[inline(always)]
    pub const fn mailbox_atomic_lock_03(&self) -> &MailboxAtomicLock03 {
        &self.mailbox_atomic_lock_03
    }
    #[doc = "0x110 - Lock flag register 04"]
    #[inline(always)]
    pub const fn mailbox_atomic_lock_04(&self) -> &MailboxAtomicLock04 {
        &self.mailbox_atomic_lock_04
    }
    #[doc = "0x114 - Lock flag register 05"]
    #[inline(always)]
    pub const fn mailbox_atomic_lock_05(&self) -> &MailboxAtomicLock05 {
        &self.mailbox_atomic_lock_05
    }
    #[doc = "0x118 - Lock flag register 06"]
    #[inline(always)]
    pub const fn mailbox_atomic_lock_06(&self) -> &MailboxAtomicLock06 {
        &self.mailbox_atomic_lock_06
    }
    #[doc = "0x11c - Lock flag register 07"]
    #[inline(always)]
    pub const fn mailbox_atomic_lock_07(&self) -> &MailboxAtomicLock07 {
        &self.mailbox_atomic_lock_07
    }
    #[doc = "0x120 - Lock flag register 08"]
    #[inline(always)]
    pub const fn mailbox_atomic_lock_08(&self) -> &MailboxAtomicLock08 {
        &self.mailbox_atomic_lock_08
    }
    #[doc = "0x124 - Lock flag register 09"]
    #[inline(always)]
    pub const fn mailbox_atomic_lock_09(&self) -> &MailboxAtomicLock09 {
        &self.mailbox_atomic_lock_09
    }
    #[doc = "0x128 - Lock flag register 10"]
    #[inline(always)]
    pub const fn mailbox_atomic_lock_10(&self) -> &MailboxAtomicLock10 {
        &self.mailbox_atomic_lock_10
    }
    #[doc = "0x12c - Lock flag register 11"]
    #[inline(always)]
    pub const fn mailbox_atomic_lock_11(&self) -> &MailboxAtomicLock11 {
        &self.mailbox_atomic_lock_11
    }
    #[doc = "0x130 - Lock flag register 12"]
    #[inline(always)]
    pub const fn mailbox_atomic_lock_12(&self) -> &MailboxAtomicLock12 {
        &self.mailbox_atomic_lock_12
    }
    #[doc = "0x134 - Lock flag register 13"]
    #[inline(always)]
    pub const fn mailbox_atomic_lock_13(&self) -> &MailboxAtomicLock13 {
        &self.mailbox_atomic_lock_13
    }
    #[doc = "0x138 - Lock flag register 14"]
    #[inline(always)]
    pub const fn mailbox_atomic_lock_14(&self) -> &MailboxAtomicLock14 {
        &self.mailbox_atomic_lock_14
    }
    #[doc = "0x13c - Lock flag register 15"]
    #[inline(always)]
    pub const fn mailbox_atomic_lock_15(&self) -> &MailboxAtomicLock15 {
        &self.mailbox_atomic_lock_15
    }
    #[doc = "0x140 - Lock flag register 16"]
    #[inline(always)]
    pub const fn mailbox_atomic_lock_16(&self) -> &MailboxAtomicLock16 {
        &self.mailbox_atomic_lock_16
    }
    #[doc = "0x144 - Lock flag register 17"]
    #[inline(always)]
    pub const fn mailbox_atomic_lock_17(&self) -> &MailboxAtomicLock17 {
        &self.mailbox_atomic_lock_17
    }
    #[doc = "0x148 - Lock flag register 18"]
    #[inline(always)]
    pub const fn mailbox_atomic_lock_18(&self) -> &MailboxAtomicLock18 {
        &self.mailbox_atomic_lock_18
    }
    #[doc = "0x14c - Lock flag register 19"]
    #[inline(always)]
    pub const fn mailbox_atomic_lock_19(&self) -> &MailboxAtomicLock19 {
        &self.mailbox_atomic_lock_19
    }
    #[doc = "0x150 - Lock flag register 20"]
    #[inline(always)]
    pub const fn mailbox_atomic_lock_20(&self) -> &MailboxAtomicLock20 {
        &self.mailbox_atomic_lock_20
    }
    #[doc = "0x154 - Lock flag register 21"]
    #[inline(always)]
    pub const fn mailbox_atomic_lock_21(&self) -> &MailboxAtomicLock21 {
        &self.mailbox_atomic_lock_21
    }
    #[doc = "0x158 - Lock flag register 22"]
    #[inline(always)]
    pub const fn mailbox_atomic_lock_22(&self) -> &MailboxAtomicLock22 {
        &self.mailbox_atomic_lock_22
    }
    #[doc = "0x15c - Lock flag register 23"]
    #[inline(always)]
    pub const fn mailbox_atomic_lock_23(&self) -> &MailboxAtomicLock23 {
        &self.mailbox_atomic_lock_23
    }
    #[doc = "0x160 - Lock flag register 24"]
    #[inline(always)]
    pub const fn mailbox_atomic_lock_24(&self) -> &MailboxAtomicLock24 {
        &self.mailbox_atomic_lock_24
    }
    #[doc = "0x164 - Lock flag register 25"]
    #[inline(always)]
    pub const fn mailbox_atomic_lock_25(&self) -> &MailboxAtomicLock25 {
        &self.mailbox_atomic_lock_25
    }
    #[doc = "0x168 - Lock flag register 26"]
    #[inline(always)]
    pub const fn mailbox_atomic_lock_26(&self) -> &MailboxAtomicLock26 {
        &self.mailbox_atomic_lock_26
    }
    #[doc = "0x16c - Lock flag register 27"]
    #[inline(always)]
    pub const fn mailbox_atomic_lock_27(&self) -> &MailboxAtomicLock27 {
        &self.mailbox_atomic_lock_27
    }
    #[doc = "0x170 - Lock flag register 28"]
    #[inline(always)]
    pub const fn mailbox_atomic_lock_28(&self) -> &MailboxAtomicLock28 {
        &self.mailbox_atomic_lock_28
    }
    #[doc = "0x174 - Lock flag register 29"]
    #[inline(always)]
    pub const fn mailbox_atomic_lock_29(&self) -> &MailboxAtomicLock29 {
        &self.mailbox_atomic_lock_29
    }
    #[doc = "0x178 - Lock flag register 30"]
    #[inline(always)]
    pub const fn mailbox_atomic_lock_30(&self) -> &MailboxAtomicLock30 {
        &self.mailbox_atomic_lock_30
    }
    #[doc = "0x17c - Lock flag register 31"]
    #[inline(always)]
    pub const fn mailbox_atomic_lock_31(&self) -> &MailboxAtomicLock31 {
        &self.mailbox_atomic_lock_31
    }
}
#[doc = "MAILBOX_A2B_INTEN (rw) register accessor: Cortex-A53/Cortex-A72 to Cortex-M0 interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_a2b_inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_a2b_inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_a2b_inten`]
module"]
#[doc(alias = "MAILBOX_A2B_INTEN")]
pub type MailboxA2bInten = crate::Reg<mailbox_a2b_inten::MailboxA2bIntenSpec>;
#[doc = "Cortex-A53/Cortex-A72 to Cortex-M0 interrupt enable register"]
pub mod mailbox_a2b_inten;
#[doc = "MAILBOX_A2B_STATUS (rw) register accessor: Cortex-A53/Cortex-A72 to Cortex-M0 interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_a2b_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_a2b_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_a2b_status`]
module"]
#[doc(alias = "MAILBOX_A2B_STATUS")]
pub type MailboxA2bStatus = crate::Reg<mailbox_a2b_status::MailboxA2bStatusSpec>;
#[doc = "Cortex-A53/Cortex-A72 to Cortex-M0 interrupt status register"]
pub mod mailbox_a2b_status;
#[doc = "MAILBOX_A2B_CMD_0 (rw) register accessor: Cortex-A53/Cortex-A72 to Cortex-M0 command 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_a2b_cmd_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_a2b_cmd_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_a2b_cmd_0`]
module"]
#[doc(alias = "MAILBOX_A2B_CMD_0")]
pub type MailboxA2bCmd0 = crate::Reg<mailbox_a2b_cmd_0::MailboxA2bCmd0Spec>;
#[doc = "Cortex-A53/Cortex-A72 to Cortex-M0 command 0"]
pub mod mailbox_a2b_cmd_0;
#[doc = "MAILBOX_A2B_DAT_0 (rw) register accessor: Cortex-A53/Cortex-A72 to Cortex-M0 data 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_a2b_dat_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_a2b_dat_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_a2b_dat_0`]
module"]
#[doc(alias = "MAILBOX_A2B_DAT_0")]
pub type MailboxA2bDat0 = crate::Reg<mailbox_a2b_dat_0::MailboxA2bDat0Spec>;
#[doc = "Cortex-A53/Cortex-A72 to Cortex-M0 data 0"]
pub mod mailbox_a2b_dat_0;
#[doc = "MAILBOX_A2B_CMD_1 (rw) register accessor: Cortex-A53/Cortex-A72 to Cortex-M0 command 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_a2b_cmd_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_a2b_cmd_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_a2b_cmd_1`]
module"]
#[doc(alias = "MAILBOX_A2B_CMD_1")]
pub type MailboxA2bCmd1 = crate::Reg<mailbox_a2b_cmd_1::MailboxA2bCmd1Spec>;
#[doc = "Cortex-A53/Cortex-A72 to Cortex-M0 command 1"]
pub mod mailbox_a2b_cmd_1;
#[doc = "MAILBOX_A2B_DAT_1 (rw) register accessor: Cortex-A53/Cortex-A72 to Cortex-M0 data 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_a2b_dat_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_a2b_dat_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_a2b_dat_1`]
module"]
#[doc(alias = "MAILBOX_A2B_DAT_1")]
pub type MailboxA2bDat1 = crate::Reg<mailbox_a2b_dat_1::MailboxA2bDat1Spec>;
#[doc = "Cortex-A53/Cortex-A72 to Cortex-M0 data 1"]
pub mod mailbox_a2b_dat_1;
#[doc = "MAILBOX_A2B_CMD_2 (rw) register accessor: Cortex-A53/Cortex-A72 to Cortex-M0 command 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_a2b_cmd_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_a2b_cmd_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_a2b_cmd_2`]
module"]
#[doc(alias = "MAILBOX_A2B_CMD_2")]
pub type MailboxA2bCmd2 = crate::Reg<mailbox_a2b_cmd_2::MailboxA2bCmd2Spec>;
#[doc = "Cortex-A53/Cortex-A72 to Cortex-M0 command 2"]
pub mod mailbox_a2b_cmd_2;
#[doc = "MAILBOX_A2B_DAT_2 (rw) register accessor: Cortex-A53/Cortex-A72 to Cortex-M0 data 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_a2b_dat_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_a2b_dat_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_a2b_dat_2`]
module"]
#[doc(alias = "MAILBOX_A2B_DAT_2")]
pub type MailboxA2bDat2 = crate::Reg<mailbox_a2b_dat_2::MailboxA2bDat2Spec>;
#[doc = "Cortex-A53/Cortex-A72 to Cortex-M0 data 2"]
pub mod mailbox_a2b_dat_2;
#[doc = "MAILBOX_A2B_CMD_3 (rw) register accessor: Cortex-A53/Cortex-A72 to Cortex-M0 command 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_a2b_cmd_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_a2b_cmd_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_a2b_cmd_3`]
module"]
#[doc(alias = "MAILBOX_A2B_CMD_3")]
pub type MailboxA2bCmd3 = crate::Reg<mailbox_a2b_cmd_3::MailboxA2bCmd3Spec>;
#[doc = "Cortex-A53/Cortex-A72 to Cortex-M0 command 3"]
pub mod mailbox_a2b_cmd_3;
#[doc = "MAILBOX_A2B_DAT_3 (rw) register accessor: Cortex-A53/Cortex-A72 to Cortex-M0 data 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_a2b_dat_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_a2b_dat_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_a2b_dat_3`]
module"]
#[doc(alias = "MAILBOX_A2B_DAT_3")]
pub type MailboxA2bDat3 = crate::Reg<mailbox_a2b_dat_3::MailboxA2bDat3Spec>;
#[doc = "Cortex-A53/Cortex-A72 to Cortex-M0 data 3"]
pub mod mailbox_a2b_dat_3;
#[doc = "MAILBOX_B2A_INTEN (rw) register accessor: Cortex-A53/Cortex-A72 to Cortex-M0 interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_b2a_inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_b2a_inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_b2a_inten`]
module"]
#[doc(alias = "MAILBOX_B2A_INTEN")]
pub type MailboxB2aInten = crate::Reg<mailbox_b2a_inten::MailboxB2aIntenSpec>;
#[doc = "Cortex-A53/Cortex-A72 to Cortex-M0 interrupt enable register"]
pub mod mailbox_b2a_inten;
#[doc = "MAILBOX_B2A_STATUS (rw) register accessor: Cortex-M0 to Cortex-A53/Cortex-A72 interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_b2a_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_b2a_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_b2a_status`]
module"]
#[doc(alias = "MAILBOX_B2A_STATUS")]
pub type MailboxB2aStatus = crate::Reg<mailbox_b2a_status::MailboxB2aStatusSpec>;
#[doc = "Cortex-M0 to Cortex-A53/Cortex-A72 interrupt status register"]
pub mod mailbox_b2a_status;
#[doc = "MAILBOX_B2A_CMD_0 (rw) register accessor: Cortex-M0 to Cortex-A53/Cortex-A72 command 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_b2a_cmd_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_b2a_cmd_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_b2a_cmd_0`]
module"]
#[doc(alias = "MAILBOX_B2A_CMD_0")]
pub type MailboxB2aCmd0 = crate::Reg<mailbox_b2a_cmd_0::MailboxB2aCmd0Spec>;
#[doc = "Cortex-M0 to Cortex-A53/Cortex-A72 command 0"]
pub mod mailbox_b2a_cmd_0;
#[doc = "MAILBOX_B2A_DAT_0 (rw) register accessor: Cortex-M0 to Cortex-A53/Cortex-A72 data 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_b2a_dat_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_b2a_dat_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_b2a_dat_0`]
module"]
#[doc(alias = "MAILBOX_B2A_DAT_0")]
pub type MailboxB2aDat0 = crate::Reg<mailbox_b2a_dat_0::MailboxB2aDat0Spec>;
#[doc = "Cortex-M0 to Cortex-A53/Cortex-A72 data 0"]
pub mod mailbox_b2a_dat_0;
#[doc = "MAILBOX_B2A_CMD_1 (rw) register accessor: Cortex-M0 to Cortex-A53/Cortex-A72 command 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_b2a_cmd_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_b2a_cmd_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_b2a_cmd_1`]
module"]
#[doc(alias = "MAILBOX_B2A_CMD_1")]
pub type MailboxB2aCmd1 = crate::Reg<mailbox_b2a_cmd_1::MailboxB2aCmd1Spec>;
#[doc = "Cortex-M0 to Cortex-A53/Cortex-A72 command 1"]
pub mod mailbox_b2a_cmd_1;
#[doc = "MAILBOX_B2A_DAT_1 (rw) register accessor: Cortex-M0 to Cortex-A53/Cortex-A72 data 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_b2a_dat_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_b2a_dat_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_b2a_dat_1`]
module"]
#[doc(alias = "MAILBOX_B2A_DAT_1")]
pub type MailboxB2aDat1 = crate::Reg<mailbox_b2a_dat_1::MailboxB2aDat1Spec>;
#[doc = "Cortex-M0 to Cortex-A53/Cortex-A72 data 1"]
pub mod mailbox_b2a_dat_1;
#[doc = "MAILBOX_B2A_CMD_2 (rw) register accessor: Cortex-M0 to Cortex-A53/Cortex-A72 command 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_b2a_cmd_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_b2a_cmd_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_b2a_cmd_2`]
module"]
#[doc(alias = "MAILBOX_B2A_CMD_2")]
pub type MailboxB2aCmd2 = crate::Reg<mailbox_b2a_cmd_2::MailboxB2aCmd2Spec>;
#[doc = "Cortex-M0 to Cortex-A53/Cortex-A72 command 2"]
pub mod mailbox_b2a_cmd_2;
#[doc = "MAILBOX_B2A_DAT_2 (rw) register accessor: Cortex-M0 to Cortex-A53/Cortex-A72 data 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_b2a_dat_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_b2a_dat_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_b2a_dat_2`]
module"]
#[doc(alias = "MAILBOX_B2A_DAT_2")]
pub type MailboxB2aDat2 = crate::Reg<mailbox_b2a_dat_2::MailboxB2aDat2Spec>;
#[doc = "Cortex-M0 to Cortex-A53/Cortex-A72 data 2"]
pub mod mailbox_b2a_dat_2;
#[doc = "MAILBOX_B2A_CMD_3 (rw) register accessor: Cortex-M0 to Cortex-A53/Cortex-A72 command 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_b2a_cmd_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_b2a_cmd_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_b2a_cmd_3`]
module"]
#[doc(alias = "MAILBOX_B2A_CMD_3")]
pub type MailboxB2aCmd3 = crate::Reg<mailbox_b2a_cmd_3::MailboxB2aCmd3Spec>;
#[doc = "Cortex-M0 to Cortex-A53/Cortex-A72 command 3"]
pub mod mailbox_b2a_cmd_3;
#[doc = "MAILBOX_B2A_DAT_3 (rw) register accessor: Cortex-M0 to Cortex-A53/Cortex-A72 data 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_b2a_dat_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_b2a_dat_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_b2a_dat_3`]
module"]
#[doc(alias = "MAILBOX_B2A_DAT_3")]
pub type MailboxB2aDat3 = crate::Reg<mailbox_b2a_dat_3::MailboxB2aDat3Spec>;
#[doc = "Cortex-M0 to Cortex-A53/Cortex-A72 data 3"]
pub mod mailbox_b2a_dat_3;
#[doc = "MAILBOX_ATOMIC_LOCK_00 (rw) register accessor: Lock flag register 00\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_00::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_00::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_atomic_lock_00`]
module"]
#[doc(alias = "MAILBOX_ATOMIC_LOCK_00")]
pub type MailboxAtomicLock00 = crate::Reg<mailbox_atomic_lock_00::MailboxAtomicLock00Spec>;
#[doc = "Lock flag register 00"]
pub mod mailbox_atomic_lock_00;
#[doc = "MAILBOX_ATOMIC_LOCK_01 (rw) register accessor: Lock flag register 01\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_atomic_lock_01`]
module"]
#[doc(alias = "MAILBOX_ATOMIC_LOCK_01")]
pub type MailboxAtomicLock01 = crate::Reg<mailbox_atomic_lock_01::MailboxAtomicLock01Spec>;
#[doc = "Lock flag register 01"]
pub mod mailbox_atomic_lock_01;
#[doc = "MAILBOX_ATOMIC_LOCK_02 (rw) register accessor: Lock flag register 02\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_02::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_02::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_atomic_lock_02`]
module"]
#[doc(alias = "MAILBOX_ATOMIC_LOCK_02")]
pub type MailboxAtomicLock02 = crate::Reg<mailbox_atomic_lock_02::MailboxAtomicLock02Spec>;
#[doc = "Lock flag register 02"]
pub mod mailbox_atomic_lock_02;
#[doc = "MAILBOX_ATOMIC_LOCK_03 (rw) register accessor: Lock flag register 03\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_03::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_03::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_atomic_lock_03`]
module"]
#[doc(alias = "MAILBOX_ATOMIC_LOCK_03")]
pub type MailboxAtomicLock03 = crate::Reg<mailbox_atomic_lock_03::MailboxAtomicLock03Spec>;
#[doc = "Lock flag register 03"]
pub mod mailbox_atomic_lock_03;
#[doc = "MAILBOX_ATOMIC_LOCK_04 (rw) register accessor: Lock flag register 04\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_04::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_04::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_atomic_lock_04`]
module"]
#[doc(alias = "MAILBOX_ATOMIC_LOCK_04")]
pub type MailboxAtomicLock04 = crate::Reg<mailbox_atomic_lock_04::MailboxAtomicLock04Spec>;
#[doc = "Lock flag register 04"]
pub mod mailbox_atomic_lock_04;
#[doc = "MAILBOX_ATOMIC_LOCK_05 (rw) register accessor: Lock flag register 05\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_05::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_05::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_atomic_lock_05`]
module"]
#[doc(alias = "MAILBOX_ATOMIC_LOCK_05")]
pub type MailboxAtomicLock05 = crate::Reg<mailbox_atomic_lock_05::MailboxAtomicLock05Spec>;
#[doc = "Lock flag register 05"]
pub mod mailbox_atomic_lock_05;
#[doc = "MAILBOX_ATOMIC_LOCK_06 (rw) register accessor: Lock flag register 06\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_06::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_06::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_atomic_lock_06`]
module"]
#[doc(alias = "MAILBOX_ATOMIC_LOCK_06")]
pub type MailboxAtomicLock06 = crate::Reg<mailbox_atomic_lock_06::MailboxAtomicLock06Spec>;
#[doc = "Lock flag register 06"]
pub mod mailbox_atomic_lock_06;
#[doc = "MAILBOX_ATOMIC_LOCK_07 (rw) register accessor: Lock flag register 07\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_07::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_07::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_atomic_lock_07`]
module"]
#[doc(alias = "MAILBOX_ATOMIC_LOCK_07")]
pub type MailboxAtomicLock07 = crate::Reg<mailbox_atomic_lock_07::MailboxAtomicLock07Spec>;
#[doc = "Lock flag register 07"]
pub mod mailbox_atomic_lock_07;
#[doc = "MAILBOX_ATOMIC_LOCK_08 (rw) register accessor: Lock flag register 08\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_08::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_08::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_atomic_lock_08`]
module"]
#[doc(alias = "MAILBOX_ATOMIC_LOCK_08")]
pub type MailboxAtomicLock08 = crate::Reg<mailbox_atomic_lock_08::MailboxAtomicLock08Spec>;
#[doc = "Lock flag register 08"]
pub mod mailbox_atomic_lock_08;
#[doc = "MAILBOX_ATOMIC_LOCK_09 (rw) register accessor: Lock flag register 09\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_09::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_09::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_atomic_lock_09`]
module"]
#[doc(alias = "MAILBOX_ATOMIC_LOCK_09")]
pub type MailboxAtomicLock09 = crate::Reg<mailbox_atomic_lock_09::MailboxAtomicLock09Spec>;
#[doc = "Lock flag register 09"]
pub mod mailbox_atomic_lock_09;
#[doc = "MAILBOX_ATOMIC_LOCK_10 (rw) register accessor: Lock flag register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_atomic_lock_10`]
module"]
#[doc(alias = "MAILBOX_ATOMIC_LOCK_10")]
pub type MailboxAtomicLock10 = crate::Reg<mailbox_atomic_lock_10::MailboxAtomicLock10Spec>;
#[doc = "Lock flag register 10"]
pub mod mailbox_atomic_lock_10;
#[doc = "MAILBOX_ATOMIC_LOCK_11 (rw) register accessor: Lock flag register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_atomic_lock_11`]
module"]
#[doc(alias = "MAILBOX_ATOMIC_LOCK_11")]
pub type MailboxAtomicLock11 = crate::Reg<mailbox_atomic_lock_11::MailboxAtomicLock11Spec>;
#[doc = "Lock flag register 11"]
pub mod mailbox_atomic_lock_11;
#[doc = "MAILBOX_ATOMIC_LOCK_12 (rw) register accessor: Lock flag register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_atomic_lock_12`]
module"]
#[doc(alias = "MAILBOX_ATOMIC_LOCK_12")]
pub type MailboxAtomicLock12 = crate::Reg<mailbox_atomic_lock_12::MailboxAtomicLock12Spec>;
#[doc = "Lock flag register 12"]
pub mod mailbox_atomic_lock_12;
#[doc = "MAILBOX_ATOMIC_LOCK_13 (rw) register accessor: Lock flag register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_atomic_lock_13`]
module"]
#[doc(alias = "MAILBOX_ATOMIC_LOCK_13")]
pub type MailboxAtomicLock13 = crate::Reg<mailbox_atomic_lock_13::MailboxAtomicLock13Spec>;
#[doc = "Lock flag register 13"]
pub mod mailbox_atomic_lock_13;
#[doc = "MAILBOX_ATOMIC_LOCK_14 (rw) register accessor: Lock flag register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_atomic_lock_14`]
module"]
#[doc(alias = "MAILBOX_ATOMIC_LOCK_14")]
pub type MailboxAtomicLock14 = crate::Reg<mailbox_atomic_lock_14::MailboxAtomicLock14Spec>;
#[doc = "Lock flag register 14"]
pub mod mailbox_atomic_lock_14;
#[doc = "MAILBOX_ATOMIC_LOCK_15 (rw) register accessor: Lock flag register 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_atomic_lock_15`]
module"]
#[doc(alias = "MAILBOX_ATOMIC_LOCK_15")]
pub type MailboxAtomicLock15 = crate::Reg<mailbox_atomic_lock_15::MailboxAtomicLock15Spec>;
#[doc = "Lock flag register 15"]
pub mod mailbox_atomic_lock_15;
#[doc = "MAILBOX_ATOMIC_LOCK_16 (rw) register accessor: Lock flag register 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_atomic_lock_16`]
module"]
#[doc(alias = "MAILBOX_ATOMIC_LOCK_16")]
pub type MailboxAtomicLock16 = crate::Reg<mailbox_atomic_lock_16::MailboxAtomicLock16Spec>;
#[doc = "Lock flag register 16"]
pub mod mailbox_atomic_lock_16;
#[doc = "MAILBOX_ATOMIC_LOCK_17 (rw) register accessor: Lock flag register 17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_atomic_lock_17`]
module"]
#[doc(alias = "MAILBOX_ATOMIC_LOCK_17")]
pub type MailboxAtomicLock17 = crate::Reg<mailbox_atomic_lock_17::MailboxAtomicLock17Spec>;
#[doc = "Lock flag register 17"]
pub mod mailbox_atomic_lock_17;
#[doc = "MAILBOX_ATOMIC_LOCK_18 (rw) register accessor: Lock flag register 18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_atomic_lock_18`]
module"]
#[doc(alias = "MAILBOX_ATOMIC_LOCK_18")]
pub type MailboxAtomicLock18 = crate::Reg<mailbox_atomic_lock_18::MailboxAtomicLock18Spec>;
#[doc = "Lock flag register 18"]
pub mod mailbox_atomic_lock_18;
#[doc = "MAILBOX_ATOMIC_LOCK_19 (rw) register accessor: Lock flag register 19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_atomic_lock_19`]
module"]
#[doc(alias = "MAILBOX_ATOMIC_LOCK_19")]
pub type MailboxAtomicLock19 = crate::Reg<mailbox_atomic_lock_19::MailboxAtomicLock19Spec>;
#[doc = "Lock flag register 19"]
pub mod mailbox_atomic_lock_19;
#[doc = "MAILBOX_ATOMIC_LOCK_20 (rw) register accessor: Lock flag register 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_atomic_lock_20`]
module"]
#[doc(alias = "MAILBOX_ATOMIC_LOCK_20")]
pub type MailboxAtomicLock20 = crate::Reg<mailbox_atomic_lock_20::MailboxAtomicLock20Spec>;
#[doc = "Lock flag register 20"]
pub mod mailbox_atomic_lock_20;
#[doc = "MAILBOX_ATOMIC_LOCK_21 (rw) register accessor: Lock flag register 21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_atomic_lock_21`]
module"]
#[doc(alias = "MAILBOX_ATOMIC_LOCK_21")]
pub type MailboxAtomicLock21 = crate::Reg<mailbox_atomic_lock_21::MailboxAtomicLock21Spec>;
#[doc = "Lock flag register 21"]
pub mod mailbox_atomic_lock_21;
#[doc = "MAILBOX_ATOMIC_LOCK_22 (rw) register accessor: Lock flag register 22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_atomic_lock_22`]
module"]
#[doc(alias = "MAILBOX_ATOMIC_LOCK_22")]
pub type MailboxAtomicLock22 = crate::Reg<mailbox_atomic_lock_22::MailboxAtomicLock22Spec>;
#[doc = "Lock flag register 22"]
pub mod mailbox_atomic_lock_22;
#[doc = "MAILBOX_ATOMIC_LOCK_23 (rw) register accessor: Lock flag register 23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_atomic_lock_23`]
module"]
#[doc(alias = "MAILBOX_ATOMIC_LOCK_23")]
pub type MailboxAtomicLock23 = crate::Reg<mailbox_atomic_lock_23::MailboxAtomicLock23Spec>;
#[doc = "Lock flag register 23"]
pub mod mailbox_atomic_lock_23;
#[doc = "MAILBOX_ATOMIC_LOCK_24 (rw) register accessor: Lock flag register 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_atomic_lock_24`]
module"]
#[doc(alias = "MAILBOX_ATOMIC_LOCK_24")]
pub type MailboxAtomicLock24 = crate::Reg<mailbox_atomic_lock_24::MailboxAtomicLock24Spec>;
#[doc = "Lock flag register 24"]
pub mod mailbox_atomic_lock_24;
#[doc = "MAILBOX_ATOMIC_LOCK_25 (rw) register accessor: Lock flag register 25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_atomic_lock_25`]
module"]
#[doc(alias = "MAILBOX_ATOMIC_LOCK_25")]
pub type MailboxAtomicLock25 = crate::Reg<mailbox_atomic_lock_25::MailboxAtomicLock25Spec>;
#[doc = "Lock flag register 25"]
pub mod mailbox_atomic_lock_25;
#[doc = "MAILBOX_ATOMIC_LOCK_26 (rw) register accessor: Lock flag register 26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_atomic_lock_26`]
module"]
#[doc(alias = "MAILBOX_ATOMIC_LOCK_26")]
pub type MailboxAtomicLock26 = crate::Reg<mailbox_atomic_lock_26::MailboxAtomicLock26Spec>;
#[doc = "Lock flag register 26"]
pub mod mailbox_atomic_lock_26;
#[doc = "MAILBOX_ATOMIC_LOCK_27 (rw) register accessor: Lock flag register 27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_atomic_lock_27`]
module"]
#[doc(alias = "MAILBOX_ATOMIC_LOCK_27")]
pub type MailboxAtomicLock27 = crate::Reg<mailbox_atomic_lock_27::MailboxAtomicLock27Spec>;
#[doc = "Lock flag register 27"]
pub mod mailbox_atomic_lock_27;
#[doc = "MAILBOX_ATOMIC_LOCK_28 (rw) register accessor: Lock flag register 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_atomic_lock_28`]
module"]
#[doc(alias = "MAILBOX_ATOMIC_LOCK_28")]
pub type MailboxAtomicLock28 = crate::Reg<mailbox_atomic_lock_28::MailboxAtomicLock28Spec>;
#[doc = "Lock flag register 28"]
pub mod mailbox_atomic_lock_28;
#[doc = "MAILBOX_ATOMIC_LOCK_29 (rw) register accessor: Lock flag register 29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_atomic_lock_29`]
module"]
#[doc(alias = "MAILBOX_ATOMIC_LOCK_29")]
pub type MailboxAtomicLock29 = crate::Reg<mailbox_atomic_lock_29::MailboxAtomicLock29Spec>;
#[doc = "Lock flag register 29"]
pub mod mailbox_atomic_lock_29;
#[doc = "MAILBOX_ATOMIC_LOCK_30 (rw) register accessor: Lock flag register 30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_atomic_lock_30`]
module"]
#[doc(alias = "MAILBOX_ATOMIC_LOCK_30")]
pub type MailboxAtomicLock30 = crate::Reg<mailbox_atomic_lock_30::MailboxAtomicLock30Spec>;
#[doc = "Lock flag register 30"]
pub mod mailbox_atomic_lock_30;
#[doc = "MAILBOX_ATOMIC_LOCK_31 (rw) register accessor: Lock flag register 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox_atomic_lock_31`]
module"]
#[doc(alias = "MAILBOX_ATOMIC_LOCK_31")]
pub type MailboxAtomicLock31 = crate::Reg<mailbox_atomic_lock_31::MailboxAtomicLock31Spec>;
#[doc = "Lock flag register 31"]
pub mod mailbox_atomic_lock_31;
