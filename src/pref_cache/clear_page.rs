#[doc = "Register `CLEAR_PAGE` writer"]
pub type W = crate::W<ClearPageSpec>;
#[doc = "Field `CLEAR_PAGE` writer - writing an address, invlidates all lines in that page from the cache"]
pub type ClearPageW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - writing an address, invlidates all lines in that page from the cache"]
    #[inline(always)]
    #[must_use]
    pub fn clear_page(&mut self) -> ClearPageW<ClearPageSpec> {
        ClearPageW::new(self, 0)
    }
}
#[doc = "clear page register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clear_page::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClearPageSpec;
impl crate::RegisterSpec for ClearPageSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clear_page::W`](W) writer structure"]
impl crate::Writable for ClearPageSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLEAR_PAGE to value 0"]
impl crate::Resettable for ClearPageSpec {
    const RESET_VALUE: u32 = 0;
}
