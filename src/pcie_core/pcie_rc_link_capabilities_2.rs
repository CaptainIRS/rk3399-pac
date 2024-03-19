#[doc = "Register `PCIE_RC_LINK_CAPABILITIES_2` reader"]
pub type R = crate::R<PcieRcLinkCapabilities2Spec>;
#[doc = "Field `SLSV` reader - Supported Link Speeds Vector \\[SLSV\\]\n\nThis field indicates the supported\n\nlink speeds of the core. For each\n\nbit, a value of 1 indicates that the\n\ncorresponding link speed is\n\nsupported, while a value of 0\n\nindicates that the corresponding\n\nspeed is not supported. This field is\n\nRsvdP for Gen1, Gen2\n\nconfigurations."]
pub type SlsvR = crate::FieldReader;
impl R {
    #[doc = "Bits 1:2 - Supported Link Speeds Vector \\[SLSV\\]\n\nThis field indicates the supported\n\nlink speeds of the core. For each\n\nbit, a value of 1 indicates that the\n\ncorresponding link speed is\n\nsupported, while a value of 0\n\nindicates that the corresponding\n\nspeed is not supported. This field is\n\nRsvdP for Gen1, Gen2\n\nconfigurations."]
    #[inline(always)]
    pub fn slsv(&self) -> SlsvR {
        SlsvR::new(((self.bits >> 1) & 3) as u8)
    }
}
#[doc = "Link Capabilities Register 2\n\nRSVD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_link_capabilities_2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcLinkCapabilities2Spec;
impl crate::RegisterSpec for PcieRcLinkCapabilities2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_link_capabilities_2::R`](R) reader structure"]
impl crate::Readable for PcieRcLinkCapabilities2Spec {}
#[doc = "`reset()` method sets PCIE_RC_LINK_CAPABILITIES_2 to value 0"]
impl crate::Resettable for PcieRcLinkCapabilities2Spec {
    const RESET_VALUE: u32 = 0;
}
