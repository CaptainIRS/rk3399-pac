#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ep_inbound_bar_address_translation_0: EpInboundBarAddressTranslation0,
    ep_inbound_bar_address_translation_1: EpInboundBarAddressTranslation1,
}
impl RegisterBlock {
    #[doc = "0x00 - EP Inbound BAR Address Translation 0 Bits \\[31:0\\]
of Address Register for BAR N"]
    #[inline(always)]
    pub const fn ep_inbound_bar_address_translation_0(&self) -> &EpInboundBarAddressTranslation0 {
        &self.ep_inbound_bar_address_translation_0
    }
    #[doc = "0x04 - EP Inbound BAR Address Translation 1 Bits \\[63:32\\]
of Address Register for BAR N"]
    #[inline(always)]
    pub const fn ep_inbound_bar_address_translation_1(&self) -> &EpInboundBarAddressTranslation1 {
        &self.ep_inbound_bar_address_translation_1
    }
}
#[doc = "EP_INBOUND_BAR_ADDRESS_TRANSLATION_0 (rw) register accessor: EP Inbound BAR Address Translation 0 Bits \\[31:0\\]
of Address Register for BAR N\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_inbound_bar_address_translation_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_inbound_bar_address_translation_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_inbound_bar_address_translation_0`]
module"]
#[doc(alias = "EP_INBOUND_BAR_ADDRESS_TRANSLATION_0")]
pub type EpInboundBarAddressTranslation0 =
    crate::Reg<ep_inbound_bar_address_translation_0::EpInboundBarAddressTranslation0Spec>;
#[doc = "EP Inbound BAR Address Translation 0 Bits \\[31:0\\]
of Address Register for BAR N"]
pub mod ep_inbound_bar_address_translation_0;
#[doc = "EP_INBOUND_BAR_ADDRESS_TRANSLATION_1 (rw) register accessor: EP Inbound BAR Address Translation 1 Bits \\[63:32\\]
of Address Register for BAR N\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_inbound_bar_address_translation_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_inbound_bar_address_translation_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_inbound_bar_address_translation_1`]
module"]
#[doc(alias = "EP_INBOUND_BAR_ADDRESS_TRANSLATION_1")]
pub type EpInboundBarAddressTranslation1 =
    crate::Reg<ep_inbound_bar_address_translation_1::EpInboundBarAddressTranslation1Spec>;
#[doc = "EP Inbound BAR Address Translation 1 Bits \\[63:32\\]
of Address Register for BAR N"]
pub mod ep_inbound_bar_address_translation_1;
