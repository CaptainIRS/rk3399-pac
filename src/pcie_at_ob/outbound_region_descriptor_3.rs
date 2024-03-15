#[doc = "Register `OUTBOUND_REGION_DESCRIPTOR_3` reader"]
pub type R = crate::R<OutboundRegionDescriptor3Spec>;
#[doc = "Field `data` reader - Descriptor bits \\[127:96\\]
\\[data\\]
Upmost 32-bits of Address Register for region N"]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Descriptor bits \\[127:96\\]
\\[data\\]
Upmost 32-bits of Address Register for region N"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
#[doc = "Outbound Region Descriptor 3 Upmost 32-bits of Address Register for region N\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outbound_region_descriptor_3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutboundRegionDescriptor3Spec;
impl crate::RegisterSpec for OutboundRegionDescriptor3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outbound_region_descriptor_3::R`](R) reader structure"]
impl crate::Readable for OutboundRegionDescriptor3Spec {}
#[doc = "`reset()` method sets OUTBOUND_REGION_DESCRIPTOR_3 to value 0"]
impl crate::Resettable for OutboundRegionDescriptor3Spec {
    const RESET_VALUE: u32 = 0;
}
