#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    id_core_id: IdCoreId,
    id_revision_id: IdRevisionId,
    fault_en: FaultEn,
    err_vld: ErrVld,
    err_clr: ErrClr,
    err_log0: ErrLog0,
    err_log1: ErrLog1,
    _reserved7: [u8; 0x04],
    err_log3: ErrLog3,
    _reserved8: [u8; 0x04],
    err_log5: ErrLog5,
    err_log6: ErrLog6,
    err_log7: ErrLog7,
    _reserved11: [u8; 0x04],
    stall_en: StallEn,
}
impl RegisterBlock {
    #[doc = "0x00 - Contain CoreTypeId and CoreChecksum"]
    #[inline(always)]
    pub const fn id_core_id(&self) -> &IdCoreId {
        &self.id_core_id
    }
    #[doc = "0x04 - IP Revision ID"]
    #[inline(always)]
    pub const fn id_revision_id(&self) -> &IdRevisionId {
        &self.id_revision_id
    }
    #[doc = "0x08 - Error interrupt enable"]
    #[inline(always)]
    pub const fn fault_en(&self) -> &FaultEn {
        &self.fault_en
    }
    #[doc = "0x0c - Error staus register"]
    #[inline(always)]
    pub const fn err_vld(&self) -> &ErrVld {
        &self.err_vld
    }
    #[doc = "0x10 - Error interrupt status clear register"]
    #[inline(always)]
    pub const fn err_clr(&self) -> &ErrClr {
        &self.err_clr
    }
    #[doc = "0x14 - Transport protocol header information register"]
    #[inline(always)]
    pub const fn err_log0(&self) -> &ErrLog0 {
        &self.err_log0
    }
    #[doc = "0x18 - Route ID register"]
    #[inline(always)]
    pub const fn err_log1(&self) -> &ErrLog1 {
        &self.err_log1
    }
    #[doc = "0x20 - Address register"]
    #[inline(always)]
    pub const fn err_log3(&self) -> &ErrLog3 {
        &self.err_log3
    }
    #[doc = "0x28 - LSB user bits in transport protocol header"]
    #[inline(always)]
    pub const fn err_log5(&self) -> &ErrLog5 {
        &self.err_log5
    }
    #[doc = "0x2c - MSB user bits in transport protocol header"]
    #[inline(always)]
    pub const fn err_log6(&self) -> &ErrLog6 {
        &self.err_log6
    }
    #[doc = "0x30 - Securrity flag in transport protocol header"]
    #[inline(always)]
    pub const fn err_log7(&self) -> &ErrLog7 {
        &self.err_log7
    }
    #[doc = "0x38 - Error logger mode selection"]
    #[inline(always)]
    pub const fn stall_en(&self) -> &StallEn {
        &self.stall_en
    }
}
#[doc = "Id_CoreId (r) register accessor: Contain CoreTypeId and CoreChecksum\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id_core_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id_core_id`]
module"]
#[doc(alias = "Id_CoreId")]
pub type IdCoreId = crate::Reg<id_core_id::IdCoreIdSpec>;
#[doc = "Contain CoreTypeId and CoreChecksum"]
pub mod id_core_id;
#[doc = "Id_RevisionId (r) register accessor: IP Revision ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id_revision_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id_revision_id`]
module"]
#[doc(alias = "Id_RevisionId")]
pub type IdRevisionId = crate::Reg<id_revision_id::IdRevisionIdSpec>;
#[doc = "IP Revision ID"]
pub mod id_revision_id;
#[doc = "FaultEn (rw) register accessor: Error interrupt enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fault_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fault_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fault_en`]
module"]
pub type FaultEn = crate::Reg<fault_en::FaultEnSpec>;
#[doc = "Error interrupt enable"]
pub mod fault_en;
#[doc = "ErrVld (r) register accessor: Error staus register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_vld::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_vld`]
module"]
pub type ErrVld = crate::Reg<err_vld::ErrVldSpec>;
#[doc = "Error staus register"]
pub mod err_vld;
#[doc = "ErrClr (rw) register accessor: Error interrupt status clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_clr`]
module"]
pub type ErrClr = crate::Reg<err_clr::ErrClrSpec>;
#[doc = "Error interrupt status clear register"]
pub mod err_clr;
#[doc = "ErrLog0 (r) register accessor: Transport protocol header information register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_log0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_log0`]
module"]
pub type ErrLog0 = crate::Reg<err_log0::ErrLog0Spec>;
#[doc = "Transport protocol header information register"]
pub mod err_log0;
#[doc = "ErrLog1 (r) register accessor: Route ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_log1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_log1`]
module"]
pub type ErrLog1 = crate::Reg<err_log1::ErrLog1Spec>;
#[doc = "Route ID register"]
pub mod err_log1;
#[doc = "ErrLog3 (r) register accessor: Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_log3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_log3`]
module"]
pub type ErrLog3 = crate::Reg<err_log3::ErrLog3Spec>;
#[doc = "Address register"]
pub mod err_log3;
#[doc = "ErrLog5 (r) register accessor: LSB user bits in transport protocol header\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_log5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_log5`]
module"]
pub type ErrLog5 = crate::Reg<err_log5::ErrLog5Spec>;
#[doc = "LSB user bits in transport protocol header"]
pub mod err_log5;
#[doc = "ErrLog6 (r) register accessor: MSB user bits in transport protocol header\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_log6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_log6`]
module"]
pub type ErrLog6 = crate::Reg<err_log6::ErrLog6Spec>;
#[doc = "MSB user bits in transport protocol header"]
pub mod err_log6;
#[doc = "ErrLog7 (r) register accessor: Securrity flag in transport protocol header\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_log7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_log7`]
module"]
pub type ErrLog7 = crate::Reg<err_log7::ErrLog7Spec>;
#[doc = "Securrity flag in transport protocol header"]
pub mod err_log7;
#[doc = "StallEn (rw) register accessor: Error logger mode selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stall_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stall_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stall_en`]
module"]
pub type StallEn = crate::Reg<stall_en::StallEnSpec>;
#[doc = "Error logger mode selection"]
pub mod stall_en;
