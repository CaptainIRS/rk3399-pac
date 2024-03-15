#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rp_inbound_bar_address_translation_0: RpInboundBarAddressTranslation0,
    rp_inbound_bar_address_translation_1: RpInboundBarAddressTranslation1,
    _reserved2: [u8; 0x1c],
    link_down_indication_bit: LinkDownIndicationBit,
}
impl RegisterBlock {
    #[doc = "0x00 - RP Inbound BAR Address Translation 0 Bits \\[31:8\\]
of Address Register for BAR N"]
    #[inline(always)]
    pub const fn rp_inbound_bar_address_translation_0(&self) -> &RpInboundBarAddressTranslation0 {
        &self.rp_inbound_bar_address_translation_0
    }
    #[doc = "0x04 - RP Inbound BAR Address Translation 1 Bits \\[63:32\\]
of Address Register for BAR N"]
    #[inline(always)]
    pub const fn rp_inbound_bar_address_translation_1(&self) -> &RpInboundBarAddressTranslation1 {
        &self.rp_inbound_bar_address_translation_1
    }
    #[doc = "0x24 - Link down indication bit RSVD"]
    #[inline(always)]
    pub const fn link_down_indication_bit(&self) -> &LinkDownIndicationBit {
        &self.link_down_indication_bit
    }
}
#[doc = "RP_INBOUND_BAR_ADDRESS_TRANSLATION_0 (rw) register accessor: RP Inbound BAR Address Translation 0 Bits \\[31:8\\]
of Address Register for BAR N\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rp_inbound_bar_address_translation_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rp_inbound_bar_address_translation_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rp_inbound_bar_address_translation_0`]
module"]
#[doc(alias = "RP_INBOUND_BAR_ADDRESS_TRANSLATION_0")]
pub type RpInboundBarAddressTranslation0 =
    crate::Reg<rp_inbound_bar_address_translation_0::RpInboundBarAddressTranslation0Spec>;
#[doc = "RP Inbound BAR Address Translation 0 Bits \\[31:8\\]
of Address Register for BAR N"]
pub mod rp_inbound_bar_address_translation_0;
#[doc = "RP_INBOUND_BAR_ADDRESS_TRANSLATION_1 (rw) register accessor: RP Inbound BAR Address Translation 1 Bits \\[63:32\\]
of Address Register for BAR N\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rp_inbound_bar_address_translation_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rp_inbound_bar_address_translation_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rp_inbound_bar_address_translation_1`]
module"]
#[doc(alias = "RP_INBOUND_BAR_ADDRESS_TRANSLATION_1")]
pub type RpInboundBarAddressTranslation1 =
    crate::Reg<rp_inbound_bar_address_translation_1::RpInboundBarAddressTranslation1Spec>;
#[doc = "RP Inbound BAR Address Translation 1 Bits \\[63:32\\]
of Address Register for BAR N"]
pub mod rp_inbound_bar_address_translation_1;
#[doc = "LINK_DOWN_INDICATION_BIT (rw) register accessor: Link down indication bit RSVD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`link_down_indication_bit::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`link_down_indication_bit::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link_down_indication_bit`]
module"]
#[doc(alias = "LINK_DOWN_INDICATION_BIT")]
pub type LinkDownIndicationBit = crate::Reg<link_down_indication_bit::LinkDownIndicationBitSpec>;
#[doc = "Link down indication bit RSVD"]
pub mod link_down_indication_bit;
