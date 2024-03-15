#[doc = "Register `OUTBOUND_REGION_DESCRIPTOR_1` reader"]
pub type R = crate::R<OutboundRegionDescriptor1Spec>;
#[doc = "Register `OUTBOUND_REGION_DESCRIPTOR_1` writer"]
pub type W = crate::W<OutboundRegionDescriptor1Spec>;
#[doc = "Field `data` reader - Descriptor bits \\[63:32\\]
\\[data\\]
Lower middle 32-bits of Address Register for region N"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `data` writer - Descriptor bits \\[63:32\\]
\\[data\\]
Lower middle 32-bits of Address Register for region N"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Descriptor bits \\[63:32\\]
\\[data\\]
Lower middle 32-bits of Address Register for region N"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Descriptor bits \\[63:32\\]
\\[data\\]
Lower middle 32-bits of Address Register for region N"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<OutboundRegionDescriptor1Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Outbound Region Descriptor 1 Lower middle 32-bits of Address Register for region N\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outbound_region_descriptor_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outbound_region_descriptor_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutboundRegionDescriptor1Spec;
impl crate::RegisterSpec for OutboundRegionDescriptor1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outbound_region_descriptor_1::R`](R) reader structure"]
impl crate::Readable for OutboundRegionDescriptor1Spec {}
#[doc = "`write(|w| ..)` method takes [`outbound_region_descriptor_1::W`](W) writer structure"]
impl crate::Writable for OutboundRegionDescriptor1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUTBOUND_REGION_DESCRIPTOR_1 to value 0"]
impl crate::Resettable for OutboundRegionDescriptor1Spec {
    const RESET_VALUE: u32 = 0;
}
