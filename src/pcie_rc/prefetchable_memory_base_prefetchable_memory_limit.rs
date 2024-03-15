#[doc = "Register `PREFETCHABLE_MEMORY_BASE_PREFETCHABLE_MEMORY_LIMIT` reader"]
pub type R = crate::R<PrefetchableMemoryBasePrefetchableMemoryLimitSpec>;
#[doc = "Field `PMBR` reader - Prefetchable Memory Base Register \\[PMBR\\]
This field can be read and written from the local management APB bus if prefetchable memory is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core."]
pub type PmbrR = crate::FieldReader<u16>;
#[doc = "Field `PMLR` reader - Prefetchable Memory Limit Register \\[PMLR\\]
This field can be read and written from the local management APB bus if prefetchable memory is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core."]
pub type PmlrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Prefetchable Memory Base Register \\[PMBR\\]
This field can be read and written from the local management APB bus if prefetchable memory is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core."]
    #[inline(always)]
    pub fn pmbr(&self) -> PmbrR {
        PmbrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Prefetchable Memory Limit Register \\[PMLR\\]
This field can be read and written from the local management APB bus if prefetchable memory is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core."]
    #[inline(always)]
    pub fn pmlr(&self) -> PmlrR {
        PmlrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Prefetchable Memory Base, Prefetchable Memory Limit This field can be read and written from the local management APB bus if prefetchable memory is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prefetchable_memory_base_prefetchable_memory_limit::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrefetchableMemoryBasePrefetchableMemoryLimitSpec;
impl crate::RegisterSpec for PrefetchableMemoryBasePrefetchableMemoryLimitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prefetchable_memory_base_prefetchable_memory_limit::R`](R) reader structure"]
impl crate::Readable for PrefetchableMemoryBasePrefetchableMemoryLimitSpec {}
#[doc = "`reset()` method sets PREFETCHABLE_MEMORY_BASE_PREFETCHABLE_MEMORY_LIMIT to value 0"]
impl crate::Resettable for PrefetchableMemoryBasePrefetchableMemoryLimitSpec {
    const RESET_VALUE: u32 = 0;
}
