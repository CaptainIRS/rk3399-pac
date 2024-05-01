#[doc = "Register `SWREG63` reader"]
pub type R = crate::R<Swreg63Spec>;
#[doc = "Register `SWREG63` writer"]
pub type W = crate::W<Swreg63Spec>;
#[doc = "Field `SW_DEC_OUT_ST_ADR` reader - write decoder output picture or field start address\n\nvideo:\n\nwrite decoder output picture or field start address\n\nJPEG snapshot:\n\nwirete decoder output luminance picture start address"]
pub type SwDecOutStAdrR = crate::FieldReader<u32>;
#[doc = "Field `SW_DEC_OUT_ST_ADR` writer - write decoder output picture or field start address\n\nvideo:\n\nwrite decoder output picture or field start address\n\nJPEG snapshot:\n\nwirete decoder output luminance picture start address"]
pub type SwDecOutStAdrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - write decoder output picture or field start address\n\nvideo:\n\nwrite decoder output picture or field start address\n\nJPEG snapshot:\n\nwirete decoder output luminance picture start address"]
    #[inline(always)]
    pub fn sw_dec_out_st_adr(&self) -> SwDecOutStAdrR {
        SwDecOutStAdrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - write decoder output picture or field start address\n\nvideo:\n\nwrite decoder output picture or field start address\n\nJPEG snapshot:\n\nwirete decoder output luminance picture start address"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_out_st_adr(&mut self) -> SwDecOutStAdrW<Swreg63Spec> {
        SwDecOutStAdrW::new(self, 2)
    }
}
#[doc = "write decoder output picture or field start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg63::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg63::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg63Spec;
impl crate::RegisterSpec for Swreg63Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg63::R`](R) reader structure"]
impl crate::Readable for Swreg63Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg63::W`](W) writer structure"]
impl crate::Writable for Swreg63Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG63 to value 0"]
impl crate::Resettable for Swreg63Spec {
    const RESET_VALUE: u32 = 0;
}
