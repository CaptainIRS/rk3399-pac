#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    qos_id_core_id: QosIdCoreId,
    qos_id_revision_id: QosIdRevisionId,
    qos_priority: QosPriority,
    qos_mode: QosMode,
    qos_bandwidth: QosBandwidth,
    qos_saturation: QosSaturation,
    qos_ext_control: QosExtControl,
}
impl RegisterBlock {
    #[doc = "0x00 - Core ID register"]
    #[inline(always)]
    pub const fn qos_id_core_id(&self) -> &QosIdCoreId {
        &self.qos_id_core_id
    }
    #[doc = "0x04 - Revision ID register"]
    #[inline(always)]
    pub const fn qos_id_revision_id(&self) -> &QosIdRevisionId {
        &self.qos_id_revision_id
    }
    #[doc = "0x08 - Priority register"]
    #[inline(always)]
    pub const fn qos_priority(&self) -> &QosPriority {
        &self.qos_priority
    }
    #[doc = "0x0c - Mode register"]
    #[inline(always)]
    pub const fn qos_mode(&self) -> &QosMode {
        &self.qos_mode
    }
    #[doc = "0x10 - Bandwidth register"]
    #[inline(always)]
    pub const fn qos_bandwidth(&self) -> &QosBandwidth {
        &self.qos_bandwidth
    }
    #[doc = "0x14 - Saturation register"]
    #[inline(always)]
    pub const fn qos_saturation(&self) -> &QosSaturation {
        &self.qos_saturation
    }
    #[doc = "0x18 - External inputs control"]
    #[inline(always)]
    pub const fn qos_ext_control(&self) -> &QosExtControl {
        &self.qos_ext_control
    }
}
#[doc = "QOS_Id_CoreId (r) register accessor: Core ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_id_core_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_id_core_id`]
module"]
#[doc(alias = "QOS_Id_CoreId")]
pub type QosIdCoreId = crate::Reg<qos_id_core_id::QosIdCoreIdSpec>;
#[doc = "Core ID register"]
pub mod qos_id_core_id;
#[doc = "QOS_Id_RevisionId (r) register accessor: Revision ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_id_revision_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_id_revision_id`]
module"]
#[doc(alias = "QOS_Id_RevisionId")]
pub type QosIdRevisionId = crate::Reg<qos_id_revision_id::QosIdRevisionIdSpec>;
#[doc = "Revision ID register"]
pub mod qos_id_revision_id;
#[doc = "QOS_Priority (rw) register accessor: Priority register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_priority::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_priority::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_priority`]
module"]
#[doc(alias = "QOS_Priority")]
pub type QosPriority = crate::Reg<qos_priority::QosPrioritySpec>;
#[doc = "Priority register"]
pub mod qos_priority;
#[doc = "QOS_Mode (rw) register accessor: Mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_mode`]
module"]
#[doc(alias = "QOS_Mode")]
pub type QosMode = crate::Reg<qos_mode::QosModeSpec>;
#[doc = "Mode register"]
pub mod qos_mode;
#[doc = "QOS_Bandwidth (rw) register accessor: Bandwidth register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_bandwidth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_bandwidth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_bandwidth`]
module"]
#[doc(alias = "QOS_Bandwidth")]
pub type QosBandwidth = crate::Reg<qos_bandwidth::QosBandwidthSpec>;
#[doc = "Bandwidth register"]
pub mod qos_bandwidth;
#[doc = "QOS_Saturation (rw) register accessor: Saturation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_saturation::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_saturation::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_saturation`]
module"]
#[doc(alias = "QOS_Saturation")]
pub type QosSaturation = crate::Reg<qos_saturation::QosSaturationSpec>;
#[doc = "Saturation register"]
pub mod qos_saturation;
#[doc = "QOS_ExtControl (rw) register accessor: External inputs control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_ext_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_ext_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_ext_control`]
module"]
#[doc(alias = "QOS_ExtControl")]
pub type QosExtControl = crate::Reg<qos_ext_control::QosExtControlSpec>;
#[doc = "External inputs control"]
pub mod qos_ext_control;
