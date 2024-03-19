#[doc = "Register `PCIE_RC_PREFETCHABLE_MEMORY_BASE_PREFETCHABLE_MEMORY_LIMIT` reader"]
pub type R = crate::R<PcieRcPrefetchableMemoryBasePrefetchableMemoryLimitSpec>;
#[doc = "Field `PMBR` reader - Prefetchable Memory Base Register \\[PMBR\\]\n\nThis field can be read and written\n\nfrom the local management APB bus\n\nif prefetchable memory is enabled in\n\nthe Root Complex BAR configuration\n\nregister, else it is hardwired to zero.\n\nIts value is not used within the core."]
pub type PmbrR = crate::FieldReader<u16>;
#[doc = "Field `PMLR` reader - Prefetchable Memory Limit Register \\[PMLR\\]\n\nThis field can be read and written\n\nfrom the local management APB bus\n\nif prefetchable memory is enabled in\n\nthe Root Complex BAR configuration\n\nregister, else it is hardwired to zero.\n\nIts value is not used within the core."]
pub type PmlrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Prefetchable Memory Base Register \\[PMBR\\]\n\nThis field can be read and written\n\nfrom the local management APB bus\n\nif prefetchable memory is enabled in\n\nthe Root Complex BAR configuration\n\nregister, else it is hardwired to zero.\n\nIts value is not used within the core."]
    #[inline(always)]
    pub fn pmbr(&self) -> PmbrR {
        PmbrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Prefetchable Memory Limit Register \\[PMLR\\]\n\nThis field can be read and written\n\nfrom the local management APB bus\n\nif prefetchable memory is enabled in\n\nthe Root Complex BAR configuration\n\nregister, else it is hardwired to zero.\n\nIts value is not used within the core."]
    #[inline(always)]
    pub fn pmlr(&self) -> PmlrR {
        PmlrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Prefetchable Memory Base, Prefetchable Memory Limit\n\nThis field can be read and written\n\nfrom the local management APB bus\n\nif prefetchable memory is enabled in\n\nthe Root Complex BAR configuration\n\nregister, else it is hardwired to zero.\n\nIts value is not used within the core.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_prefetchable_memory_base_prefetchable_memory_limit::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcPrefetchableMemoryBasePrefetchableMemoryLimitSpec;
impl crate::RegisterSpec for PcieRcPrefetchableMemoryBasePrefetchableMemoryLimitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_prefetchable_memory_base_prefetchable_memory_limit::R`](R) reader structure"]
impl crate::Readable for PcieRcPrefetchableMemoryBasePrefetchableMemoryLimitSpec {}
#[doc = "`reset()` method sets PCIE_RC_PREFETCHABLE_MEMORY_BASE_PREFETCHABLE_MEMORY_LIMIT to value 0"]
impl crate::Resettable for PcieRcPrefetchableMemoryBasePrefetchableMemoryLimitSpec {
    const RESET_VALUE: u32 = 0;
}
