#[doc = "Register `SWREG_63` reader"]
pub type R = crate::R<Swreg63Spec>;
#[doc = "Register `SWREG_63` writer"]
pub type W = crate::W<Swreg63Spec>;
#[doc = "Field `RECON_LUMA_ST_ADR` reader - the reconstructed luma start address\n\nthe reconstructed luma start address"]
pub type ReconLumaStAdrR = crate::FieldReader<u32>;
#[doc = "Field `RECON_LUMA_ST_ADR` writer - the reconstructed luma start address\n\nthe reconstructed luma start address"]
pub type ReconLumaStAdrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the reconstructed luma start address\n\nthe reconstructed luma start address"]
    #[inline(always)]
    pub fn recon_luma_st_adr(&self) -> ReconLumaStAdrR {
        ReconLumaStAdrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the reconstructed luma start address\n\nthe reconstructed luma start address"]
    #[inline(always)]
    #[must_use]
    pub fn recon_luma_st_adr(&mut self) -> ReconLumaStAdrW<Swreg63Spec> {
        ReconLumaStAdrW::new(self, 0)
    }
}
#[doc = "the reconstructed luma start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_63::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_63::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg63Spec;
impl crate::RegisterSpec for Swreg63Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_63::R`](R) reader structure"]
impl crate::Readable for Swreg63Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_63::W`](W) writer structure"]
impl crate::Writable for Swreg63Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_63 to value 0"]
impl crate::Resettable for Swreg63Spec {
    const RESET_VALUE: u32 = 0;
}
