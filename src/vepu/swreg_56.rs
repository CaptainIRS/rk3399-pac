#[doc = "Register `SWREG_56` reader"]
pub type R = crate::R<Swreg56Spec>;
#[doc = "Register `SWREG_56` writer"]
pub type W = crate::W<Swreg56Spec>;
#[doc = "Field `LUMA_REF_ST_ADR` reader - the luma reference frame start address\n\nthe luma reference frame start address"]
pub type LumaRefStAdrR = crate::FieldReader<u32>;
#[doc = "Field `LUMA_REF_ST_ADR` writer - the luma reference frame start address\n\nthe luma reference frame start address"]
pub type LumaRefStAdrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the luma reference frame start address\n\nthe luma reference frame start address"]
    #[inline(always)]
    pub fn luma_ref_st_adr(&self) -> LumaRefStAdrR {
        LumaRefStAdrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the luma reference frame start address\n\nthe luma reference frame start address"]
    #[inline(always)]
    #[must_use]
    pub fn luma_ref_st_adr(&mut self) -> LumaRefStAdrW<Swreg56Spec> {
        LumaRefStAdrW::new(self, 0)
    }
}
#[doc = "the luma reference frame start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_56::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_56::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg56Spec;
impl crate::RegisterSpec for Swreg56Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_56::R`](R) reader structure"]
impl crate::Readable for Swreg56Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_56::W`](W) writer structure"]
impl crate::Writable for Swreg56Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_56 to value 0"]
impl crate::Resettable for Swreg56Spec {
    const RESET_VALUE: u32 = 0;
}
