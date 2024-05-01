#[doc = "Register `SWREG19` reader"]
pub type R = crate::R<Swreg19Spec>;
#[doc = "Register `SWREG19` writer"]
pub type W = crate::W<Swreg19Spec>;
#[doc = "Field `SW_CB_IN_ST_ADR` reader - input cb component start address\n\nThe start address of topfield of the picture when data come from\n\nfields,extemal mode support only"]
pub type SwCbInStAdrR = crate::FieldReader<u32>;
#[doc = "Field `SW_CB_IN_ST_ADR` writer - input cb component start address\n\nThe start address of topfield of the picture when data come from\n\nfields,extemal mode support only"]
pub type SwCbInStAdrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - input cb component start address\n\nThe start address of topfield of the picture when data come from\n\nfields,extemal mode support only"]
    #[inline(always)]
    pub fn sw_cb_in_st_adr(&self) -> SwCbInStAdrR {
        SwCbInStAdrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - input cb component start address\n\nThe start address of topfield of the picture when data come from\n\nfields,extemal mode support only"]
    #[inline(always)]
    #[must_use]
    pub fn sw_cb_in_st_adr(&mut self) -> SwCbInStAdrW<Swreg19Spec> {
        SwCbInStAdrW::new(self, 2)
    }
}
#[doc = "Base address for reading post-processing input picture Cb/Ch\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg19::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg19::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg19Spec;
impl crate::RegisterSpec for Swreg19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg19::R`](R) reader structure"]
impl crate::Readable for Swreg19Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg19::W`](W) writer structure"]
impl crate::Writable for Swreg19Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG19 to value 0"]
impl crate::Resettable for Swreg19Spec {
    const RESET_VALUE: u32 = 0;
}
