#[doc = "Register `HASH_TAB_LO` reader"]
pub type R = crate::R<HashTabLoSpec>;
#[doc = "Register `HASH_TAB_LO` writer"]
pub type W = crate::W<HashTabLoSpec>;
#[doc = "Field `HTL` reader - Hash Table Low\n\nThis field contains the lower 32 bits of Hash table"]
pub type HtlR = crate::FieldReader<u32>;
#[doc = "Field `HTL` writer - Hash Table Low\n\nThis field contains the lower 32 bits of Hash table"]
pub type HtlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash Table Low\n\nThis field contains the lower 32 bits of Hash table"]
    #[inline(always)]
    pub fn htl(&self) -> HtlR {
        HtlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash Table Low\n\nThis field contains the lower 32 bits of Hash table"]
    #[inline(always)]
    #[must_use]
    pub fn htl(&mut self) -> HtlW<HashTabLoSpec> {
        HtlW::new(self, 0)
    }
}
#[doc = "Hash Table Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_tab_lo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_tab_lo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashTabLoSpec;
impl crate::RegisterSpec for HashTabLoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_tab_lo::R`](R) reader structure"]
impl crate::Readable for HashTabLoSpec {}
#[doc = "`write(|w| ..)` method takes [`hash_tab_lo::W`](W) writer structure"]
impl crate::Writable for HashTabLoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_TAB_LO to value 0"]
impl crate::Resettable for HashTabLoSpec {
    const RESET_VALUE: u32 = 0;
}
