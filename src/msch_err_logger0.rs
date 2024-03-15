#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    errlog_id_core_id: ErrlogIdCoreId,
    errlog_id_revision_id: ErrlogIdRevisionId,
    errlog_fault_en: ErrlogFaultEn,
    errlog_err_vld: ErrlogErrVld,
    errlog_err_clr: ErrlogErrClr,
    errlog_err_log0: ErrlogErrLog0,
    errlog_err_log1: ErrlogErrLog1,
    _reserved7: [u8; 0x04],
    errlog_err_log3: ErrlogErrLog3,
    _reserved8: [u8; 0x04],
    errlog_err_log5: ErrlogErrLog5,
    _reserved9: [u8; 0x04],
    errlog_err_log7: ErrlogErrLog7,
    _reserved10: [u8; 0x04],
    errlog_stall_en: ErrlogStallEn,
}
impl RegisterBlock {
    #[doc = "0x00 - This may be different for each error logger."]
    #[inline(always)]
    pub const fn errlog_id_core_id(&self) -> &ErrlogIdCoreId {
        &self.errlog_id_core_id
    }
    #[doc = "0x04 - It is the same for each error logger."]
    #[inline(always)]
    pub const fn errlog_id_revision_id(&self) -> &ErrlogIdRevisionId {
        &self.errlog_id_revision_id
    }
    #[doc = "0x08 - Error interrupt enable"]
    #[inline(always)]
    pub const fn errlog_fault_en(&self) -> &ErrlogFaultEn {
        &self.errlog_fault_en
    }
    #[doc = "0x0c - Error staus register"]
    #[inline(always)]
    pub const fn errlog_err_vld(&self) -> &ErrlogErrVld {
        &self.errlog_err_vld
    }
    #[doc = "0x10 - Error interrupt status clear register"]
    #[inline(always)]
    pub const fn errlog_err_clr(&self) -> &ErrlogErrClr {
        &self.errlog_err_clr
    }
    #[doc = "0x14 - Transport protocol header information register"]
    #[inline(always)]
    pub const fn errlog_err_log0(&self) -> &ErrlogErrLog0 {
        &self.errlog_err_log0
    }
    #[doc = "0x18 - Route ID register"]
    #[inline(always)]
    pub const fn errlog_err_log1(&self) -> &ErrlogErrLog1 {
        &self.errlog_err_log1
    }
    #[doc = "0x20 - Address register"]
    #[inline(always)]
    pub const fn errlog_err_log3(&self) -> &ErrlogErrLog3 {
        &self.errlog_err_log3
    }
    #[doc = "0x28 - User bits in transport protocol header"]
    #[inline(always)]
    pub const fn errlog_err_log5(&self) -> &ErrlogErrLog5 {
        &self.errlog_err_log5
    }
    #[doc = "0x30 - Securrity flag in transport protocol header"]
    #[inline(always)]
    pub const fn errlog_err_log7(&self) -> &ErrlogErrLog7 {
        &self.errlog_err_log7
    }
    #[doc = "0x38 - Error logger mode selection"]
    #[inline(always)]
    pub const fn errlog_stall_en(&self) -> &ErrlogStallEn {
        &self.errlog_stall_en
    }
}
#[doc = "ERRLOG_Id_CoreId (r) register accessor: This may be different for each error logger.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errlog_id_core_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errlog_id_core_id`]
module"]
#[doc(alias = "ERRLOG_Id_CoreId")]
pub type ErrlogIdCoreId = crate::Reg<errlog_id_core_id::ErrlogIdCoreIdSpec>;
#[doc = "This may be different for each error logger."]
pub mod errlog_id_core_id;
#[doc = "ERRLOG_Id_RevisionId (r) register accessor: It is the same for each error logger.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errlog_id_revision_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errlog_id_revision_id`]
module"]
#[doc(alias = "ERRLOG_Id_RevisionId")]
pub type ErrlogIdRevisionId = crate::Reg<errlog_id_revision_id::ErrlogIdRevisionIdSpec>;
#[doc = "It is the same for each error logger."]
pub mod errlog_id_revision_id;
#[doc = "ERRLOG_FaultEn (rw) register accessor: Error interrupt enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errlog_fault_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`errlog_fault_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errlog_fault_en`]
module"]
#[doc(alias = "ERRLOG_FaultEn")]
pub type ErrlogFaultEn = crate::Reg<errlog_fault_en::ErrlogFaultEnSpec>;
#[doc = "Error interrupt enable"]
pub mod errlog_fault_en;
#[doc = "ERRLOG_ErrVld (r) register accessor: Error staus register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errlog_err_vld::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errlog_err_vld`]
module"]
#[doc(alias = "ERRLOG_ErrVld")]
pub type ErrlogErrVld = crate::Reg<errlog_err_vld::ErrlogErrVldSpec>;
#[doc = "Error staus register"]
pub mod errlog_err_vld;
#[doc = "ERRLOG_ErrClr (rw) register accessor: Error interrupt status clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errlog_err_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`errlog_err_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errlog_err_clr`]
module"]
#[doc(alias = "ERRLOG_ErrClr")]
pub type ErrlogErrClr = crate::Reg<errlog_err_clr::ErrlogErrClrSpec>;
#[doc = "Error interrupt status clear register"]
pub mod errlog_err_clr;
#[doc = "ERRLOG_ErrLog0 (r) register accessor: Transport protocol header information register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errlog_err_log0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errlog_err_log0`]
module"]
#[doc(alias = "ERRLOG_ErrLog0")]
pub type ErrlogErrLog0 = crate::Reg<errlog_err_log0::ErrlogErrLog0Spec>;
#[doc = "Transport protocol header information register"]
pub mod errlog_err_log0;
#[doc = "ERRLOG_ErrLog1 (r) register accessor: Route ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errlog_err_log1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errlog_err_log1`]
module"]
#[doc(alias = "ERRLOG_ErrLog1")]
pub type ErrlogErrLog1 = crate::Reg<errlog_err_log1::ErrlogErrLog1Spec>;
#[doc = "Route ID register"]
pub mod errlog_err_log1;
#[doc = "ERRLOG_ErrLog3 (r) register accessor: Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errlog_err_log3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errlog_err_log3`]
module"]
#[doc(alias = "ERRLOG_ErrLog3")]
pub type ErrlogErrLog3 = crate::Reg<errlog_err_log3::ErrlogErrLog3Spec>;
#[doc = "Address register"]
pub mod errlog_err_log3;
#[doc = "ERRLOG_ErrLog5 (r) register accessor: User bits in transport protocol header\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errlog_err_log5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errlog_err_log5`]
module"]
#[doc(alias = "ERRLOG_ErrLog5")]
pub type ErrlogErrLog5 = crate::Reg<errlog_err_log5::ErrlogErrLog5Spec>;
#[doc = "User bits in transport protocol header"]
pub mod errlog_err_log5;
#[doc = "ERRLOG_ErrLog7 (r) register accessor: Securrity flag in transport protocol header\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errlog_err_log7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errlog_err_log7`]
module"]
#[doc(alias = "ERRLOG_ErrLog7")]
pub type ErrlogErrLog7 = crate::Reg<errlog_err_log7::ErrlogErrLog7Spec>;
#[doc = "Securrity flag in transport protocol header"]
pub mod errlog_err_log7;
#[doc = "ERRLOG_StallEn (rw) register accessor: Error logger mode selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errlog_stall_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`errlog_stall_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errlog_stall_en`]
module"]
#[doc(alias = "ERRLOG_StallEn")]
pub type ErrlogStallEn = crate::Reg<errlog_stall_en::ErrlogStallEnSpec>;
#[doc = "Error logger mode selection"]
pub mod errlog_stall_en;
