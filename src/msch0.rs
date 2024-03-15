#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    msch_id_core_id: MschIdCoreId,
    msch_id_revision_id: MschIdRevisionId,
    msch_device_conf: MschDeviceConf,
    msch_device_size: MschDeviceSize,
    msch_ddr_timing_a0: MschDdrTimingA0,
    msch_ddr_timing_b0: MschDdrTimingB0,
    msch_ddr_timing_c0: MschDdrTimingC0,
    msch_dev_to_dev0: MschDevToDev0,
    _reserved8: [u8; 0xf0],
    msch_ddr_mode: MschDdrMode,
    _reserved9: [u8; 0x0eec],
    msch_aging_x0: MschAgingX0,
}
impl RegisterBlock {
    #[doc = "0x00 - Core ID register"]
    #[inline(always)]
    pub const fn msch_id_core_id(&self) -> &MschIdCoreId {
        &self.msch_id_core_id
    }
    #[doc = "0x04 - Revision ID register"]
    #[inline(always)]
    pub const fn msch_id_revision_id(&self) -> &MschIdRevisionId {
        &self.msch_id_revision_id
    }
    #[doc = "0x08 - ddr configuration pointers"]
    #[inline(always)]
    pub const fn msch_device_conf(&self) -> &MschDeviceConf {
        &self.msch_device_conf
    }
    #[doc = "0x0c - ddr configuration sizes."]
    #[inline(always)]
    pub const fn msch_device_size(&self) -> &MschDeviceSize {
        &self.msch_device_size
    }
    #[doc = "0x10 - DdrTimingA bank 0"]
    #[inline(always)]
    pub const fn msch_ddr_timing_a0(&self) -> &MschDdrTimingA0 {
        &self.msch_ddr_timing_a0
    }
    #[doc = "0x14 - DdrTimingB bank 0"]
    #[inline(always)]
    pub const fn msch_ddr_timing_b0(&self) -> &MschDdrTimingB0 {
        &self.msch_ddr_timing_b0
    }
    #[doc = "0x18 - DdrTimingC bank 0"]
    #[inline(always)]
    pub const fn msch_ddr_timing_c0(&self) -> &MschDdrTimingC0 {
        &self.msch_ddr_timing_c0
    }
    #[doc = "0x1c - Timing values concerning device to device data bus ownership c"]
    #[inline(always)]
    pub const fn msch_dev_to_dev0(&self) -> &MschDevToDev0 {
        &self.msch_dev_to_dev0
    }
    #[doc = "0x110 - ddr mode definition."]
    #[inline(always)]
    pub const fn msch_ddr_mode(&self) -> &MschDdrMode {
        &self.msch_ddr_mode
    }
    #[doc = "0x1000 - Aging threshold multiplicator."]
    #[inline(always)]
    pub const fn msch_aging_x0(&self) -> &MschAgingX0 {
        &self.msch_aging_x0
    }
}
#[doc = "MSCH_Id_CoreId (r) register accessor: Core ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msch_id_core_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msch_id_core_id`]
module"]
#[doc(alias = "MSCH_Id_CoreId")]
pub type MschIdCoreId = crate::Reg<msch_id_core_id::MschIdCoreIdSpec>;
#[doc = "Core ID register"]
pub mod msch_id_core_id;
#[doc = "MSCH_Id_RevisionId (r) register accessor: Revision ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msch_id_revision_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msch_id_revision_id`]
module"]
#[doc(alias = "MSCH_Id_RevisionId")]
pub type MschIdRevisionId = crate::Reg<msch_id_revision_id::MschIdRevisionIdSpec>;
#[doc = "Revision ID register"]
pub mod msch_id_revision_id;
#[doc = "MSCH_DeviceConf (rw) register accessor: ddr configuration pointers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msch_device_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msch_device_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msch_device_conf`]
module"]
#[doc(alias = "MSCH_DeviceConf")]
pub type MschDeviceConf = crate::Reg<msch_device_conf::MschDeviceConfSpec>;
#[doc = "ddr configuration pointers"]
pub mod msch_device_conf;
#[doc = "MSCH_DeviceSize (rw) register accessor: ddr configuration sizes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msch_device_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msch_device_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msch_device_size`]
module"]
#[doc(alias = "MSCH_DeviceSize")]
pub type MschDeviceSize = crate::Reg<msch_device_size::MschDeviceSizeSpec>;
#[doc = "ddr configuration sizes."]
pub mod msch_device_size;
#[doc = "MSCH_DdrTimingA0 (rw) register accessor: DdrTimingA bank 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msch_ddr_timing_a0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msch_ddr_timing_a0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msch_ddr_timing_a0`]
module"]
#[doc(alias = "MSCH_DdrTimingA0")]
pub type MschDdrTimingA0 = crate::Reg<msch_ddr_timing_a0::MschDdrTimingA0Spec>;
#[doc = "DdrTimingA bank 0"]
pub mod msch_ddr_timing_a0;
#[doc = "MSCH_DdrTimingB0 (rw) register accessor: DdrTimingB bank 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msch_ddr_timing_b0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msch_ddr_timing_b0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msch_ddr_timing_b0`]
module"]
#[doc(alias = "MSCH_DdrTimingB0")]
pub type MschDdrTimingB0 = crate::Reg<msch_ddr_timing_b0::MschDdrTimingB0Spec>;
#[doc = "DdrTimingB bank 0"]
pub mod msch_ddr_timing_b0;
#[doc = "MSCH_DdrTimingC0 (rw) register accessor: DdrTimingC bank 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msch_ddr_timing_c0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msch_ddr_timing_c0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msch_ddr_timing_c0`]
module"]
#[doc(alias = "MSCH_DdrTimingC0")]
pub type MschDdrTimingC0 = crate::Reg<msch_ddr_timing_c0::MschDdrTimingC0Spec>;
#[doc = "DdrTimingC bank 0"]
pub mod msch_ddr_timing_c0;
#[doc = "MSCH_DevToDev0 (rw) register accessor: Timing values concerning device to device data bus ownership c\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msch_dev_to_dev0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msch_dev_to_dev0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msch_dev_to_dev0`]
module"]
#[doc(alias = "MSCH_DevToDev0")]
pub type MschDevToDev0 = crate::Reg<msch_dev_to_dev0::MschDevToDev0Spec>;
#[doc = "Timing values concerning device to device data bus ownership c"]
pub mod msch_dev_to_dev0;
#[doc = "MSCH_DdrMode (rw) register accessor: ddr mode definition.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msch_ddr_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msch_ddr_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msch_ddr_mode`]
module"]
#[doc(alias = "MSCH_DdrMode")]
pub type MschDdrMode = crate::Reg<msch_ddr_mode::MschDdrModeSpec>;
#[doc = "ddr mode definition."]
pub mod msch_ddr_mode;
#[doc = "MSCH_AgingX0 (rw) register accessor: Aging threshold multiplicator.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msch_aging_x0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msch_aging_x0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msch_aging_x0`]
module"]
#[doc(alias = "MSCH_AgingX0")]
pub type MschAgingX0 = crate::Reg<msch_aging_x0::MschAgingX0Spec>;
#[doc = "Aging threshold multiplicator."]
pub mod msch_aging_x0;
