#[doc = "Register `SWREG20` reader"]
pub type R = crate::R<Swreg20Spec>;
#[doc = "Register `SWREG20` writer"]
pub type W = crate::W<Swreg20Spec>;
#[doc = "Field `SW_CR_IN_ST_ADR` reader - input cr component start address\n\nThe start address of topfield of the picture when data come from\n\nfields,extemal mode support only"]
pub type SwCrInStAdrR = crate::FieldReader<u32>;
#[doc = "Field `SW_CR_IN_ST_ADR` writer - input cr component start address\n\nThe start address of topfield of the picture when data come from\n\nfields,extemal mode support only"]
pub type SwCrInStAdrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - input cr component start address\n\nThe start address of topfield of the picture when data come from\n\nfields,extemal mode support only"]
    #[inline(always)]
    pub fn sw_cr_in_st_adr(&self) -> SwCrInStAdrR {
        SwCrInStAdrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - input cr component start address\n\nThe start address of topfield of the picture when data come from\n\nfields,extemal mode support only"]
    #[inline(always)]
    #[must_use]
    pub fn sw_cr_in_st_adr(&mut self) -> SwCrInStAdrW<Swreg20Spec> {
        SwCrInStAdrW::new(self, 2)
    }
}
#[doc = "input cr component address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg20::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg20::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg20Spec;
impl crate::RegisterSpec for Swreg20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg20::R`](R) reader structure"]
impl crate::Readable for Swreg20Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg20::W`](W) writer structure"]
impl crate::Writable for Swreg20Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG20 to value 0"]
impl crate::Resettable for Swreg20Spec {
    const RESET_VALUE: u32 = 0;
}
