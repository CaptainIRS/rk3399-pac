#[doc = "Register `SWREG_57` reader"]
pub type R = crate::R<Swreg57Spec>;
#[doc = "Register `SWREG_57` writer"]
pub type W = crate::W<Swreg57Spec>;
#[doc = "Field `CHROMA_REF_ST_ADR` reader - the chroma reference frame start address\n\nthe chroma reference frame start address"]
pub type ChromaRefStAdrR = crate::FieldReader<u32>;
#[doc = "Field `CHROMA_REF_ST_ADR` writer - the chroma reference frame start address\n\nthe chroma reference frame start address"]
pub type ChromaRefStAdrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the chroma reference frame start address\n\nthe chroma reference frame start address"]
    #[inline(always)]
    pub fn chroma_ref_st_adr(&self) -> ChromaRefStAdrR {
        ChromaRefStAdrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the chroma reference frame start address\n\nthe chroma reference frame start address"]
    #[inline(always)]
    #[must_use]
    pub fn chroma_ref_st_adr(&mut self) -> ChromaRefStAdrW<Swreg57Spec> {
        ChromaRefStAdrW::new(self, 0)
    }
}
#[doc = "the chroma reference frame start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_57::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_57::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg57Spec;
impl crate::RegisterSpec for Swreg57Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_57::R`](R) reader structure"]
impl crate::Readable for Swreg57Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_57::W`](W) writer structure"]
impl crate::Writable for Swreg57Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_57 to value 0"]
impl crate::Resettable for Swreg57Spec {
    const RESET_VALUE: u32 = 0;
}
