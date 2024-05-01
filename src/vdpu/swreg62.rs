#[doc = "Register `SWREG62` reader"]
pub type R = crate::R<Swreg62Spec>;
#[doc = "Register `SWREG62` writer"]
pub type W = crate::W<Swreg62Spec>;
#[doc = "Field `SW_DMMV_ST_ADR` reader - Direct mode motion vector write/read start address\n\nH264:\n\nDirect mode motion vector write/read start address\n\nProgressive JPEG:\n\nthe start address for ACDC coefficient read/write\n\nIf current round is for DC components :\n\nthis start address is pointing to luminance\n\nAC component rounds:\n\nthis start address is used for current type"]
pub type SwDmmvStAdrR = crate::FieldReader<u32>;
#[doc = "Field `SW_DMMV_ST_ADR` writer - Direct mode motion vector write/read start address\n\nH264:\n\nDirect mode motion vector write/read start address\n\nProgressive JPEG:\n\nthe start address for ACDC coefficient read/write\n\nIf current round is for DC components :\n\nthis start address is pointing to luminance\n\nAC component rounds:\n\nthis start address is used for current type"]
pub type SwDmmvStAdrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Direct mode motion vector write/read start address\n\nH264:\n\nDirect mode motion vector write/read start address\n\nProgressive JPEG:\n\nthe start address for ACDC coefficient read/write\n\nIf current round is for DC components :\n\nthis start address is pointing to luminance\n\nAC component rounds:\n\nthis start address is used for current type"]
    #[inline(always)]
    pub fn sw_dmmv_st_adr(&self) -> SwDmmvStAdrR {
        SwDmmvStAdrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Direct mode motion vector write/read start address\n\nH264:\n\nDirect mode motion vector write/read start address\n\nProgressive JPEG:\n\nthe start address for ACDC coefficient read/write\n\nIf current round is for DC components :\n\nthis start address is pointing to luminance\n\nAC component rounds:\n\nthis start address is used for current type"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dmmv_st_adr(&mut self) -> SwDmmvStAdrW<Swreg62Spec> {
        SwDmmvStAdrW::new(self, 2)
    }
}
#[doc = "Direct mode motion vector write/read start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg62::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg62::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg62Spec;
impl crate::RegisterSpec for Swreg62Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg62::R`](R) reader structure"]
impl crate::Readable for Swreg62Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg62::W`](W) writer structure"]
impl crate::Writable for Swreg62Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG62 to value 0"]
impl crate::Resettable for Swreg62Spec {
    const RESET_VALUE: u32 = 0;
}
