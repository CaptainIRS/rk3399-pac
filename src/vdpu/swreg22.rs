#[doc = "Register `SWREG22` reader"]
pub type R = crate::R<Swreg22Spec>;
#[doc = "Register `SWREG22` writer"]
pub type W = crate::W<Swreg22Spec>;
#[doc = "Field `SW_C_OUT_ST_ADR` reader - output chrominace component start address\n\nformat is uvuvuv...."]
pub type SwCOutStAdrR = crate::FieldReader<u32>;
#[doc = "Field `SW_C_OUT_ST_ADR` writer - output chrominace component start address\n\nformat is uvuvuv...."]
pub type SwCOutStAdrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - output chrominace component start address\n\nformat is uvuvuv...."]
    #[inline(always)]
    pub fn sw_c_out_st_adr(&self) -> SwCOutStAdrR {
        SwCOutStAdrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - output chrominace component start address\n\nformat is uvuvuv...."]
    #[inline(always)]
    #[must_use]
    pub fn sw_c_out_st_adr(&mut self) -> SwCOutStAdrW<Swreg22Spec> {
        SwCOutStAdrW::new(self, 0)
    }
}
#[doc = "Base address for writing post-processed picture Ch\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg22::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg22::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg22Spec;
impl crate::RegisterSpec for Swreg22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg22::R`](R) reader structure"]
impl crate::Readable for Swreg22Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg22::W`](W) writer structure"]
impl crate::Writable for Swreg22Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG22 to value 0"]
impl crate::Resettable for Swreg22Spec {
    const RESET_VALUE: u32 = 0;
}
