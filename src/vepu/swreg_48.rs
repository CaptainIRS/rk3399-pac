#[doc = "Register `SWREG_48` reader"]
pub type R = crate::R<Swreg48Spec>;
#[doc = "Register `SWREG_48` writer"]
pub type W = crate::W<Swreg48Spec>;
#[doc = "Field `LUMA_IN_ST_ADR` reader - input luma start address\n\ninput luma start address"]
pub type LumaInStAdrR = crate::FieldReader<u32>;
#[doc = "Field `LUMA_IN_ST_ADR` writer - input luma start address\n\ninput luma start address"]
pub type LumaInStAdrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - input luma start address\n\ninput luma start address"]
    #[inline(always)]
    pub fn luma_in_st_adr(&self) -> LumaInStAdrR {
        LumaInStAdrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - input luma start address\n\ninput luma start address"]
    #[inline(always)]
    #[must_use]
    pub fn luma_in_st_adr(&mut self) -> LumaInStAdrW<Swreg48Spec> {
        LumaInStAdrW::new(self, 0)
    }
}
#[doc = "input luma start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_48::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_48::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg48Spec;
impl crate::RegisterSpec for Swreg48Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_48::R`](R) reader structure"]
impl crate::Readable for Swreg48Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_48::W`](W) writer structure"]
impl crate::Writable for Swreg48Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_48 to value 0"]
impl crate::Resettable for Swreg48Spec {
    const RESET_VALUE: u32 = 0;
}
