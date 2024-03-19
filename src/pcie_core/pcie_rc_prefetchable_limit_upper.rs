#[doc = "Register `PCIE_RC_PREFETCHABLE_LIMIT_UPPER` reader"]
pub type R = crate::R<PcieRcPrefetchableLimitUpperSpec>;
#[doc = "Field `PLRU` reader - Prefetchable Limit Register Upper \\[PLRU\\]\n\nThis field can be read and written\n\nfrom the local management APB bus\n\nif 64bit prefetchable memory is\n\nenabled in the Root Complex BAR\n\nconfiguration register, else it is\n\nhardwired to zero. Its value is not\n\nused within the core."]
pub type PlruR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Prefetchable Limit Register Upper \\[PLRU\\]\n\nThis field can be read and written\n\nfrom the local management APB bus\n\nif 64bit prefetchable memory is\n\nenabled in the Root Complex BAR\n\nconfiguration register, else it is\n\nhardwired to zero. Its value is not\n\nused within the core."]
    #[inline(always)]
    pub fn plru(&self) -> PlruR {
        PlruR::new(self.bits)
    }
}
#[doc = "Prefetchable Limit Upper\n\nThis field can be read and written\n\nfrom the local management APB bus\n\nif 64bit prefetchable memory is\n\nenabled in the Root Complex BAR\n\nconfiguration register, else it is\n\nhardwired to zero. Its value is not\n\nused within the core.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_prefetchable_limit_upper::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcPrefetchableLimitUpperSpec;
impl crate::RegisterSpec for PcieRcPrefetchableLimitUpperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_prefetchable_limit_upper::R`](R) reader structure"]
impl crate::Readable for PcieRcPrefetchableLimitUpperSpec {}
#[doc = "`reset()` method sets PCIE_RC_PREFETCHABLE_LIMIT_UPPER to value 0"]
impl crate::Resettable for PcieRcPrefetchableLimitUpperSpec {
    const RESET_VALUE: u32 = 0;
}
