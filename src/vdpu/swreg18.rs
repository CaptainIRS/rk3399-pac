#[doc = "Register `SWREG18` reader"]
pub type R = crate::R<Swreg18Spec>;
#[doc = "Register `SWREG18` writer"]
pub type W = crate::W<Swreg18Spec>;
#[doc = "Field `SW_Y_IN_ST_ADR` reader - input y component start address\n\nThe start address of topfield of the picture when data come from\n\nfields.extemal mode support only"]
pub type SwYInStAdrR = crate::FieldReader<u32>;
#[doc = "Field `SW_Y_IN_ST_ADR` writer - input y component start address\n\nThe start address of topfield of the picture when data come from\n\nfields.extemal mode support only"]
pub type SwYInStAdrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - input y component start address\n\nThe start address of topfield of the picture when data come from\n\nfields.extemal mode support only"]
    #[inline(always)]
    pub fn sw_y_in_st_adr(&self) -> SwYInStAdrR {
        SwYInStAdrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - input y component start address\n\nThe start address of topfield of the picture when data come from\n\nfields.extemal mode support only"]
    #[inline(always)]
    #[must_use]
    pub fn sw_y_in_st_adr(&mut self) -> SwYInStAdrW<Swreg18Spec> {
        SwYInStAdrW::new(self, 2)
    }
}
#[doc = "base address for reading post-processing input picture uminan\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg18::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg18::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg18Spec;
impl crate::RegisterSpec for Swreg18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg18::R`](R) reader structure"]
impl crate::Readable for Swreg18Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg18::W`](W) writer structure"]
impl crate::Writable for Swreg18Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG18 to value 0"]
impl crate::Resettable for Swreg18Spec {
    const RESET_VALUE: u32 = 0;
}
