#[doc = "Register `PCIE_PF_LINK_CAPABILITIES_2` reader"]
pub type R = crate::R<PciePfLinkCapabilities2Spec>;
#[doc = "Field `SLSV` reader - Supported Link Speeds Vector \\[SLSV\\]\n\nThis field indicates the supported\n\nlink speeds of the core. For each bit,\n\na value of 1 indicates that the\n\ncorresponding link speed is\n\nsupported, while a value of 0\n\nindicates that the corresponding\n\nspeed is not supported. The bits\n\ncorresponding to various link speeds\n\nare: Bit 1 = Link Speed 2.5 GT/s,\n\nBit 2= Link Speed 5 GT/s, Bit 3 =\n\nLink Speed 8 GT/s. This field is\n\nhardwired to 001 (2.5 GT/s) when\n\nthe PCIE_GENERATION_SEL strap\n\npins of the core are set to 0, 011\n\n(2.5 and 5 GT/s) when the strap\n\nis set to 1. This field is RsvrdP for\n\nthe selected configuration."]
pub type SlsvR = crate::FieldReader;
impl R {
    #[doc = "Bits 1:2 - Supported Link Speeds Vector \\[SLSV\\]\n\nThis field indicates the supported\n\nlink speeds of the core. For each bit,\n\na value of 1 indicates that the\n\ncorresponding link speed is\n\nsupported, while a value of 0\n\nindicates that the corresponding\n\nspeed is not supported. The bits\n\ncorresponding to various link speeds\n\nare: Bit 1 = Link Speed 2.5 GT/s,\n\nBit 2= Link Speed 5 GT/s, Bit 3 =\n\nLink Speed 8 GT/s. This field is\n\nhardwired to 001 (2.5 GT/s) when\n\nthe PCIE_GENERATION_SEL strap\n\npins of the core are set to 0, 011\n\n(2.5 and 5 GT/s) when the strap\n\nis set to 1. This field is RsvrdP for\n\nthe selected configuration."]
    #[inline(always)]
    pub fn slsv(&self) -> SlsvR {
        SlsvR::new(((self.bits >> 1) & 3) as u8)
    }
}
#[doc = "Link Capabilities Register 2\n\nRSVD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_link_capabilities_2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfLinkCapabilities2Spec;
impl crate::RegisterSpec for PciePfLinkCapabilities2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_link_capabilities_2::R`](R) reader structure"]
impl crate::Readable for PciePfLinkCapabilities2Spec {}
#[doc = "`reset()` method sets PCIE_PF_LINK_CAPABILITIES_2 to value 0"]
impl crate::Resettable for PciePfLinkCapabilities2Spec {
    const RESET_VALUE: u32 = 0;
}
