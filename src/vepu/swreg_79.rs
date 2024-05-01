#[doc = "Register `SWREG_79` reader"]
pub type R = crate::R<Swreg79Spec>;
#[doc = "Register `SWREG_79` writer"]
pub type W = crate::W<Swreg79Spec>;
#[doc = "Field `NEXT_LUMA_ST_ADR` reader - next picture luminance start address\n\nnext picture luminance start address"]
pub type NextLumaStAdrR = crate::FieldReader<u32>;
#[doc = "Field `NEXT_LUMA_ST_ADR` writer - next picture luminance start address\n\nnext picture luminance start address"]
pub type NextLumaStAdrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - next picture luminance start address\n\nnext picture luminance start address"]
    #[inline(always)]
    pub fn next_luma_st_adr(&self) -> NextLumaStAdrR {
        NextLumaStAdrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - next picture luminance start address\n\nnext picture luminance start address"]
    #[inline(always)]
    #[must_use]
    pub fn next_luma_st_adr(&mut self) -> NextLumaStAdrW<Swreg79Spec> {
        NextLumaStAdrW::new(self, 0)
    }
}
#[doc = "next picture luminance start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_79::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_79::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg79Spec;
impl crate::RegisterSpec for Swreg79Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_79::R`](R) reader structure"]
impl crate::Readable for Swreg79Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_79::W`](W) writer structure"]
impl crate::Writable for Swreg79Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_79 to value 0"]
impl crate::Resettable for Swreg79Spec {
    const RESET_VALUE: u32 = 0;
}
