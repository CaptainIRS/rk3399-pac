#[doc = "Register `PCIE_AT_OB_OUTBOUND_REGION_DESCRIPTOR_3` reader"]
pub type R = crate::R<PcieAtObOutboundRegionDescriptor3Spec>;
#[doc = "Field `data` reader - Descriptor bits \\[127:96\\]
\\[data\\]\n\nUpmost 32-bits of Address Register\n\nfor region N"]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Descriptor bits \\[127:96\\]
\\[data\\]\n\nUpmost 32-bits of Address Register\n\nfor region N"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
#[doc = "Outbound Region Descriptor 3\n\nUpmost 32-bits of Address Register\n\nfor region N\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_at_ob_outbound_region_descriptor_3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieAtObOutboundRegionDescriptor3Spec;
impl crate::RegisterSpec for PcieAtObOutboundRegionDescriptor3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_at_ob_outbound_region_descriptor_3::R`](R) reader structure"]
impl crate::Readable for PcieAtObOutboundRegionDescriptor3Spec {}
#[doc = "`reset()` method sets PCIE_AT_OB_OUTBOUND_REGION_DESCRIPTOR_3 to value 0"]
impl crate::Resettable for PcieAtObOutboundRegionDescriptor3Spec {
    const RESET_VALUE: u32 = 0;
}
