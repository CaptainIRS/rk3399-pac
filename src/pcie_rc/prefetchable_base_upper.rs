#[doc = "Register `PREFETCHABLE_BASE_UPPER` reader"]
pub type R = crate::R<PrefetchableBaseUpperSpec>;
#[doc = "Field `PBRU` reader - Prefetchable Base Register Upper \\[PBRU\\]
This field can be read and written from the local management APB bus if 64bit prefetchable memory is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core."]
pub type PbruR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Prefetchable Base Register Upper \\[PBRU\\]
This field can be read and written from the local management APB bus if 64bit prefetchable memory is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core."]
    #[inline(always)]
    pub fn pbru(&self) -> PbruR {
        PbruR::new(self.bits)
    }
}
#[doc = "Prefetchable Base Upper This field can be read and written from the local management APB bus if 64bit prefetchable memory is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prefetchable_base_upper::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrefetchableBaseUpperSpec;
impl crate::RegisterSpec for PrefetchableBaseUpperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prefetchable_base_upper::R`](R) reader structure"]
impl crate::Readable for PrefetchableBaseUpperSpec {}
#[doc = "`reset()` method sets PREFETCHABLE_BASE_UPPER to value 0"]
impl crate::Resettable for PrefetchableBaseUpperSpec {
    const RESET_VALUE: u32 = 0;
}
