#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    id_core_id: IdCoreId,
    id_revision_id: IdRevisionId,
    priority: Priority,
    mode: Mode,
    bandwidth: Bandwidth,
    saturation: Saturation,
    ext_control: ExtControl,
}
impl RegisterBlock {
    #[doc = "0x00 - Core ID register"]
    #[inline(always)]
    pub const fn id_core_id(&self) -> &IdCoreId {
        &self.id_core_id
    }
    #[doc = "0x04 - Revision ID register"]
    #[inline(always)]
    pub const fn id_revision_id(&self) -> &IdRevisionId {
        &self.id_revision_id
    }
    #[doc = "0x08 - Priority register"]
    #[inline(always)]
    pub const fn priority(&self) -> &Priority {
        &self.priority
    }
    #[doc = "0x0c - Mode register"]
    #[inline(always)]
    pub const fn mode(&self) -> &Mode {
        &self.mode
    }
    #[doc = "0x10 - Bandwidth register"]
    #[inline(always)]
    pub const fn bandwidth(&self) -> &Bandwidth {
        &self.bandwidth
    }
    #[doc = "0x14 - Saturation register"]
    #[inline(always)]
    pub const fn saturation(&self) -> &Saturation {
        &self.saturation
    }
    #[doc = "0x18 - External inputs control"]
    #[inline(always)]
    pub const fn ext_control(&self) -> &ExtControl {
        &self.ext_control
    }
}
#[doc = "Id_CoreId (r) register accessor: Core ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id_core_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id_core_id`]
module"]
#[doc(alias = "Id_CoreId")]
pub type IdCoreId = crate::Reg<id_core_id::IdCoreIdSpec>;
#[doc = "Core ID register"]
pub mod id_core_id;
#[doc = "Id_RevisionId (r) register accessor: Revision ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id_revision_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id_revision_id`]
module"]
#[doc(alias = "Id_RevisionId")]
pub type IdRevisionId = crate::Reg<id_revision_id::IdRevisionIdSpec>;
#[doc = "Revision ID register"]
pub mod id_revision_id;
#[doc = "Priority (rw) register accessor: Priority register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority`]
module"]
pub type Priority = crate::Reg<priority::PrioritySpec>;
#[doc = "Priority register"]
pub mod priority;
#[doc = "Mode (rw) register accessor: Mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`]
module"]
pub type Mode = crate::Reg<mode::ModeSpec>;
#[doc = "Mode register"]
pub mod mode;
#[doc = "Bandwidth (rw) register accessor: Bandwidth register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bandwidth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bandwidth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bandwidth`]
module"]
pub type Bandwidth = crate::Reg<bandwidth::BandwidthSpec>;
#[doc = "Bandwidth register"]
pub mod bandwidth;
#[doc = "Saturation (rw) register accessor: Saturation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saturation::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saturation::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saturation`]
module"]
pub type Saturation = crate::Reg<saturation::SaturationSpec>;
#[doc = "Saturation register"]
pub mod saturation;
#[doc = "ExtControl (rw) register accessor: External inputs control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_control`]
module"]
pub type ExtControl = crate::Reg<ext_control::ExtControlSpec>;
#[doc = "External inputs control"]
pub mod ext_control;
