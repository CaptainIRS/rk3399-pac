#[doc = "Register `PCIE_RC_PREFETCHABLE_BASE_UPPER` reader"]
pub type R = crate::R<PcieRcPrefetchableBaseUpperSpec>;
#[doc = "Field `PBRU` reader - Prefetchable Base Register Upper \\[PBRU\\]\n\nThis field can be read and written\n\nfrom the local management APB bus\n\nif 64bit prefetchable memory is\n\nenabled in the Root Complex BAR\n\nconfiguration register, else it is\n\nhardwired to zero. Its value is not\n\nused within the core."]
pub type PbruR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Prefetchable Base Register Upper \\[PBRU\\]\n\nThis field can be read and written\n\nfrom the local management APB bus\n\nif 64bit prefetchable memory is\n\nenabled in the Root Complex BAR\n\nconfiguration register, else it is\n\nhardwired to zero. Its value is not\n\nused within the core."]
    #[inline(always)]
    pub fn pbru(&self) -> PbruR {
        PbruR::new(self.bits)
    }
}
#[doc = "Prefetchable Base Upper\n\nThis field can be read and written\n\nfrom the local management APB bus\n\nif 64bit prefetchable memory is\n\nenabled in the Root Complex BAR\n\nconfiguration register, else it is\n\nhardwired to zero. Its value is not\n\nused within the core.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_prefetchable_base_upper::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcPrefetchableBaseUpperSpec;
impl crate::RegisterSpec for PcieRcPrefetchableBaseUpperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_prefetchable_base_upper::R`](R) reader structure"]
impl crate::Readable for PcieRcPrefetchableBaseUpperSpec {}
#[doc = "`reset()` method sets PCIE_RC_PREFETCHABLE_BASE_UPPER to value 0"]
impl crate::Resettable for PcieRcPrefetchableBaseUpperSpec {
    const RESET_VALUE: u32 = 0;
}
