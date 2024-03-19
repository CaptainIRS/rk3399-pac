#[doc = "Register `PCIE_AT_OB_OUTBOUND_REGION_DESCRIPTOR_0` reader"]
pub type R = crate::R<PcieAtObOutboundRegionDescriptor0Spec>;
#[doc = "Register `PCIE_AT_OB_OUTBOUND_REGION_DESCRIPTOR_0` writer"]
pub type W = crate::W<PcieAtObOutboundRegionDescriptor0Spec>;
#[doc = "Field `data` reader - Descriptor bits \\[31:0\\]
\\[data\\]\n\nLowest 32-bits of Address Register\n\nfor region N"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `data` writer - Descriptor bits \\[31:0\\]
\\[data\\]\n\nLowest 32-bits of Address Register\n\nfor region N"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Descriptor bits \\[31:0\\]
\\[data\\]\n\nLowest 32-bits of Address Register\n\nfor region N"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Descriptor bits \\[31:0\\]
\\[data\\]\n\nLowest 32-bits of Address Register\n\nfor region N"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<PcieAtObOutboundRegionDescriptor0Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Outbound Region Descriptor 0\n\nLowest 32-bits of Address Register\n\nfor region N\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_at_ob_outbound_region_descriptor_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_at_ob_outbound_region_descriptor_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieAtObOutboundRegionDescriptor0Spec;
impl crate::RegisterSpec for PcieAtObOutboundRegionDescriptor0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_at_ob_outbound_region_descriptor_0::R`](R) reader structure"]
impl crate::Readable for PcieAtObOutboundRegionDescriptor0Spec {}
#[doc = "`write(|w| ..)` method takes [`pcie_at_ob_outbound_region_descriptor_0::W`](W) writer structure"]
impl crate::Writable for PcieAtObOutboundRegionDescriptor0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_AT_OB_OUTBOUND_REGION_DESCRIPTOR_0 to value 0"]
impl crate::Resettable for PcieAtObOutboundRegionDescriptor0Spec {
    const RESET_VALUE: u32 = 0;
}
