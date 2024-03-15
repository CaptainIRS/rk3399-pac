#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    outbound_region_address_0: OutboundRegionAddress0,
    outbound_region_address_1: OutboundRegionAddress1,
    outbound_region_descriptor_0: OutboundRegionDescriptor0,
    outbound_region_descriptor_1: OutboundRegionDescriptor1,
    outbound_region_descriptor_2: OutboundRegionDescriptor2,
    outbound_region_descriptor_3: OutboundRegionDescriptor3,
}
impl RegisterBlock {
    #[doc = "0x00 - Outbound Region Address 0 Lower 32-bits of Address Register for region N"]
    #[inline(always)]
    pub const fn outbound_region_address_0(&self) -> &OutboundRegionAddress0 {
        &self.outbound_region_address_0
    }
    #[doc = "0x04 - Outbound Region Address 1 Upper 32-bits of Address Register for region N"]
    #[inline(always)]
    pub const fn outbound_region_address_1(&self) -> &OutboundRegionAddress1 {
        &self.outbound_region_address_1
    }
    #[doc = "0x08 - Outbound Region Descriptor 0 Lowest 32-bits of Address Register for region N"]
    #[inline(always)]
    pub const fn outbound_region_descriptor_0(&self) -> &OutboundRegionDescriptor0 {
        &self.outbound_region_descriptor_0
    }
    #[doc = "0x0c - Outbound Region Descriptor 1 Lower middle 32-bits of Address Register for region N"]
    #[inline(always)]
    pub const fn outbound_region_descriptor_1(&self) -> &OutboundRegionDescriptor1 {
        &self.outbound_region_descriptor_1
    }
    #[doc = "0x10 - Outbound Region Descriptor 2 Upper middle 32-bits of Address Register for region N"]
    #[inline(always)]
    pub const fn outbound_region_descriptor_2(&self) -> &OutboundRegionDescriptor2 {
        &self.outbound_region_descriptor_2
    }
    #[doc = "0x14 - Outbound Region Descriptor 3 Upmost 32-bits of Address Register for region N"]
    #[inline(always)]
    pub const fn outbound_region_descriptor_3(&self) -> &OutboundRegionDescriptor3 {
        &self.outbound_region_descriptor_3
    }
}
#[doc = "OUTBOUND_REGION_ADDRESS_0 (rw) register accessor: Outbound Region Address 0 Lower 32-bits of Address Register for region N\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outbound_region_address_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outbound_region_address_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outbound_region_address_0`]
module"]
#[doc(alias = "OUTBOUND_REGION_ADDRESS_0")]
pub type OutboundRegionAddress0 = crate::Reg<outbound_region_address_0::OutboundRegionAddress0Spec>;
#[doc = "Outbound Region Address 0 Lower 32-bits of Address Register for region N"]
pub mod outbound_region_address_0;
#[doc = "OUTBOUND_REGION_ADDRESS_1 (rw) register accessor: Outbound Region Address 1 Upper 32-bits of Address Register for region N\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outbound_region_address_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outbound_region_address_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outbound_region_address_1`]
module"]
#[doc(alias = "OUTBOUND_REGION_ADDRESS_1")]
pub type OutboundRegionAddress1 = crate::Reg<outbound_region_address_1::OutboundRegionAddress1Spec>;
#[doc = "Outbound Region Address 1 Upper 32-bits of Address Register for region N"]
pub mod outbound_region_address_1;
#[doc = "OUTBOUND_REGION_DESCRIPTOR_0 (rw) register accessor: Outbound Region Descriptor 0 Lowest 32-bits of Address Register for region N\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outbound_region_descriptor_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outbound_region_descriptor_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outbound_region_descriptor_0`]
module"]
#[doc(alias = "OUTBOUND_REGION_DESCRIPTOR_0")]
pub type OutboundRegionDescriptor0 =
    crate::Reg<outbound_region_descriptor_0::OutboundRegionDescriptor0Spec>;
#[doc = "Outbound Region Descriptor 0 Lowest 32-bits of Address Register for region N"]
pub mod outbound_region_descriptor_0;
#[doc = "OUTBOUND_REGION_DESCRIPTOR_1 (rw) register accessor: Outbound Region Descriptor 1 Lower middle 32-bits of Address Register for region N\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outbound_region_descriptor_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outbound_region_descriptor_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outbound_region_descriptor_1`]
module"]
#[doc(alias = "OUTBOUND_REGION_DESCRIPTOR_1")]
pub type OutboundRegionDescriptor1 =
    crate::Reg<outbound_region_descriptor_1::OutboundRegionDescriptor1Spec>;
#[doc = "Outbound Region Descriptor 1 Lower middle 32-bits of Address Register for region N"]
pub mod outbound_region_descriptor_1;
#[doc = "OUTBOUND_REGION_DESCRIPTOR_2 (rw) register accessor: Outbound Region Descriptor 2 Upper middle 32-bits of Address Register for region N\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outbound_region_descriptor_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outbound_region_descriptor_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outbound_region_descriptor_2`]
module"]
#[doc(alias = "OUTBOUND_REGION_DESCRIPTOR_2")]
pub type OutboundRegionDescriptor2 =
    crate::Reg<outbound_region_descriptor_2::OutboundRegionDescriptor2Spec>;
#[doc = "Outbound Region Descriptor 2 Upper middle 32-bits of Address Register for region N"]
pub mod outbound_region_descriptor_2;
#[doc = "OUTBOUND_REGION_DESCRIPTOR_3 (r) register accessor: Outbound Region Descriptor 3 Upmost 32-bits of Address Register for region N\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outbound_region_descriptor_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outbound_region_descriptor_3`]
module"]
#[doc(alias = "OUTBOUND_REGION_DESCRIPTOR_3")]
pub type OutboundRegionDescriptor3 =
    crate::Reg<outbound_region_descriptor_3::OutboundRegionDescriptor3Spec>;
#[doc = "Outbound Region Descriptor 3 Upmost 32-bits of Address Register for region N"]
pub mod outbound_region_descriptor_3;
