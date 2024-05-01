#[doc = "Register `SWREG_64` reader"]
pub type R = crate::R<Swreg64Spec>;
#[doc = "Register `SWREG_64` writer"]
pub type W = crate::W<Swreg64Spec>;
#[doc = "Field `RECON_CHROMA_ST_ADR` reader - the reconstructed chroma start address\n\nthe reconstructed chroma start address"]
pub type ReconChromaStAdrR = crate::FieldReader<u32>;
#[doc = "Field `RECON_CHROMA_ST_ADR` writer - the reconstructed chroma start address\n\nthe reconstructed chroma start address"]
pub type ReconChromaStAdrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the reconstructed chroma start address\n\nthe reconstructed chroma start address"]
    #[inline(always)]
    pub fn recon_chroma_st_adr(&self) -> ReconChromaStAdrR {
        ReconChromaStAdrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the reconstructed chroma start address\n\nthe reconstructed chroma start address"]
    #[inline(always)]
    #[must_use]
    pub fn recon_chroma_st_adr(&mut self) -> ReconChromaStAdrW<Swreg64Spec> {
        ReconChromaStAdrW::new(self, 0)
    }
}
#[doc = "the reconstructed chroma start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_64::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_64::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg64Spec;
impl crate::RegisterSpec for Swreg64Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_64::R`](R) reader structure"]
impl crate::Readable for Swreg64Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_64::W`](W) writer structure"]
impl crate::Writable for Swreg64Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_64 to value 0"]
impl crate::Resettable for Swreg64Spec {
    const RESET_VALUE: u32 = 0;
}
