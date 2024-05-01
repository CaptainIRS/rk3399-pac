#[doc = "Register `SWREG_50` reader"]
pub type R = crate::R<Swreg50Spec>;
#[doc = "Register `SWREG_50` writer"]
pub type W = crate::W<Swreg50Spec>;
#[doc = "Field `CR_IN_ST_ADR` reader - input cr start address\n\ninput cr start address"]
pub type CrInStAdrR = crate::FieldReader<u32>;
#[doc = "Field `CR_IN_ST_ADR` writer - input cr start address\n\ninput cr start address"]
pub type CrInStAdrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - input cr start address\n\ninput cr start address"]
    #[inline(always)]
    pub fn cr_in_st_adr(&self) -> CrInStAdrR {
        CrInStAdrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - input cr start address\n\ninput cr start address"]
    #[inline(always)]
    #[must_use]
    pub fn cr_in_st_adr(&mut self) -> CrInStAdrW<Swreg50Spec> {
        CrInStAdrW::new(self, 0)
    }
}
#[doc = "input cr start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_50::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_50::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg50Spec;
impl crate::RegisterSpec for Swreg50Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_50::R`](R) reader structure"]
impl crate::Readable for Swreg50Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_50::W`](W) writer structure"]
impl crate::Writable for Swreg50Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_50 to value 0"]
impl crate::Resettable for Swreg50Spec {
    const RESET_VALUE: u32 = 0;
}
