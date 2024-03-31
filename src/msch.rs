#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    id_core_id: IdCoreId,
    id_revision_id: IdRevisionId,
    device_conf: DeviceConf,
    device_size: DeviceSize,
    ddr_timing_a0: DdrTimingA0,
    ddr_timing_b0: DdrTimingB0,
    ddr_timing_c0: DdrTimingC0,
    dev_to_dev0: DevToDev0,
    _reserved8: [u8; 0xf0],
    ddr_mode: DdrMode,
    _reserved9: [u8; 0x0eec],
    aging_x0: AgingX0,
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
    #[doc = "0x08 - ddr configuration pointers"]
    #[inline(always)]
    pub const fn device_conf(&self) -> &DeviceConf {
        &self.device_conf
    }
    #[doc = "0x0c - ddr configuration sizes."]
    #[inline(always)]
    pub const fn device_size(&self) -> &DeviceSize {
        &self.device_size
    }
    #[doc = "0x10 - DdrTimingA bank 0"]
    #[inline(always)]
    pub const fn ddr_timing_a0(&self) -> &DdrTimingA0 {
        &self.ddr_timing_a0
    }
    #[doc = "0x14 - DdrTimingB bank 0"]
    #[inline(always)]
    pub const fn ddr_timing_b0(&self) -> &DdrTimingB0 {
        &self.ddr_timing_b0
    }
    #[doc = "0x18 - DdrTimingC bank 0"]
    #[inline(always)]
    pub const fn ddr_timing_c0(&self) -> &DdrTimingC0 {
        &self.ddr_timing_c0
    }
    #[doc = "0x1c - Timing values concerning device to device data bus ownership c"]
    #[inline(always)]
    pub const fn dev_to_dev0(&self) -> &DevToDev0 {
        &self.dev_to_dev0
    }
    #[doc = "0x110 - ddr mode definition."]
    #[inline(always)]
    pub const fn ddr_mode(&self) -> &DdrMode {
        &self.ddr_mode
    }
    #[doc = "0x1000 - Aging threshold multiplicator."]
    #[inline(always)]
    pub const fn aging_x0(&self) -> &AgingX0 {
        &self.aging_x0
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
#[doc = "DeviceConf (rw) register accessor: ddr configuration pointers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`device_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`device_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@device_conf`]
module"]
pub type DeviceConf = crate::Reg<device_conf::DeviceConfSpec>;
#[doc = "ddr configuration pointers"]
pub mod device_conf;
#[doc = "DeviceSize (rw) register accessor: ddr configuration sizes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`device_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`device_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@device_size`]
module"]
pub type DeviceSize = crate::Reg<device_size::DeviceSizeSpec>;
#[doc = "ddr configuration sizes."]
pub mod device_size;
#[doc = "DdrTimingA0 (rw) register accessor: DdrTimingA bank 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_timing_a0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_timing_a0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_timing_a0`]
module"]
pub type DdrTimingA0 = crate::Reg<ddr_timing_a0::DdrTimingA0Spec>;
#[doc = "DdrTimingA bank 0"]
pub mod ddr_timing_a0;
#[doc = "DdrTimingB0 (rw) register accessor: DdrTimingB bank 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_timing_b0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_timing_b0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_timing_b0`]
module"]
pub type DdrTimingB0 = crate::Reg<ddr_timing_b0::DdrTimingB0Spec>;
#[doc = "DdrTimingB bank 0"]
pub mod ddr_timing_b0;
#[doc = "DdrTimingC0 (rw) register accessor: DdrTimingC bank 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_timing_c0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_timing_c0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_timing_c0`]
module"]
pub type DdrTimingC0 = crate::Reg<ddr_timing_c0::DdrTimingC0Spec>;
#[doc = "DdrTimingC bank 0"]
pub mod ddr_timing_c0;
#[doc = "DevToDev0 (rw) register accessor: Timing values concerning device to device data bus ownership c\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_to_dev0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dev_to_dev0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_to_dev0`]
module"]
pub type DevToDev0 = crate::Reg<dev_to_dev0::DevToDev0Spec>;
#[doc = "Timing values concerning device to device data bus ownership c"]
pub mod dev_to_dev0;
#[doc = "DdrMode (rw) register accessor: ddr mode definition.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_mode`]
module"]
pub type DdrMode = crate::Reg<ddr_mode::DdrModeSpec>;
#[doc = "ddr mode definition."]
pub mod ddr_mode;
#[doc = "AgingX0 (rw) register accessor: Aging threshold multiplicator.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aging_x0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aging_x0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aging_x0`]
module"]
pub type AgingX0 = crate::Reg<aging_x0::AgingX0Spec>;
#[doc = "Aging threshold multiplicator."]
pub mod aging_x0;
