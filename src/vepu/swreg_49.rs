#[doc = "Register `SWREG_49` reader"]
pub type R = crate::R<Swreg49Spec>;
#[doc = "Register `SWREG_49` writer"]
pub type W = crate::W<Swreg49Spec>;
#[doc = "Field `CB_IN_ST_ADR` reader - input cb start address\n\ninput cb start address"]
pub type CbInStAdrR = crate::FieldReader<u32>;
#[doc = "Field `CB_IN_ST_ADR` writer - input cb start address\n\ninput cb start address"]
pub type CbInStAdrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - input cb start address\n\ninput cb start address"]
    #[inline(always)]
    pub fn cb_in_st_adr(&self) -> CbInStAdrR {
        CbInStAdrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - input cb start address\n\ninput cb start address"]
    #[inline(always)]
    #[must_use]
    pub fn cb_in_st_adr(&mut self) -> CbInStAdrW<Swreg49Spec> {
        CbInStAdrW::new(self, 0)
    }
}
#[doc = "input cb start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_49::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_49::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg49Spec;
impl crate::RegisterSpec for Swreg49Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_49::R`](R) reader structure"]
impl crate::Readable for Swreg49Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_49::W`](W) writer structure"]
impl crate::Writable for Swreg49Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_49 to value 0"]
impl crate::Resettable for Swreg49Spec {
    const RESET_VALUE: u32 = 0;
}
