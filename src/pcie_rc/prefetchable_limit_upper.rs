#[doc = "Register `PREFETCHABLE_LIMIT_UPPER` reader"]
pub type R = crate::R<PrefetchableLimitUpperSpec>;
#[doc = "Field `PLRU` reader - Prefetchable Limit Register Upper \\[PLRU\\]
This field can be read and written from the local management APB bus if 64bit prefetchable memory is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core."]
pub type PlruR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Prefetchable Limit Register Upper \\[PLRU\\]
This field can be read and written from the local management APB bus if 64bit prefetchable memory is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core."]
    #[inline(always)]
    pub fn plru(&self) -> PlruR {
        PlruR::new(self.bits)
    }
}
#[doc = "Prefetchable Limit Upper This field can be read and written from the local management APB bus if 64bit prefetchable memory is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prefetchable_limit_upper::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrefetchableLimitUpperSpec;
impl crate::RegisterSpec for PrefetchableLimitUpperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prefetchable_limit_upper::R`](R) reader structure"]
impl crate::Readable for PrefetchableLimitUpperSpec {}
#[doc = "`reset()` method sets PREFETCHABLE_LIMIT_UPPER to value 0"]
impl crate::Resettable for PrefetchableLimitUpperSpec {
    const RESET_VALUE: u32 = 0;
}
