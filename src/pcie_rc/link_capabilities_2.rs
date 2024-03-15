#[doc = "Register `LINK_CAPABILITIES_2` reader"]
pub type R = crate::R<LinkCapabilities2Spec>;
#[doc = "Field `SLSV` reader - Supported Link Speeds Vector \\[SLSV\\]
This field indicates the supported link speeds of the core. For each bit, a value of 1 indicates that the corresponding link speed is supported, while a value of 0 indicates that the corresponding speed is not supported. This field is RsvdP for Gen1, Gen2 configurations."]
pub type SlsvR = crate::FieldReader;
impl R {
    #[doc = "Bits 1:2 - Supported Link Speeds Vector \\[SLSV\\]
This field indicates the supported link speeds of the core. For each bit, a value of 1 indicates that the corresponding link speed is supported, while a value of 0 indicates that the corresponding speed is not supported. This field is RsvdP for Gen1, Gen2 configurations."]
    #[inline(always)]
    pub fn slsv(&self) -> SlsvR {
        SlsvR::new(((self.bits >> 1) & 3) as u8)
    }
}
#[doc = "Link Capabilities Register 2 RSVD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`link_capabilities_2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LinkCapabilities2Spec;
impl crate::RegisterSpec for LinkCapabilities2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`link_capabilities_2::R`](R) reader structure"]
impl crate::Readable for LinkCapabilities2Spec {}
#[doc = "`reset()` method sets LINK_CAPABILITIES_2 to value 0"]
impl crate::Resettable for LinkCapabilities2Spec {
    const RESET_VALUE: u32 = 0;
}
