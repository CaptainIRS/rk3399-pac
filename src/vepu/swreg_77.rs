#[doc = "Register `SWREG_77` reader"]
pub type R = crate::R<Swreg77Spec>;
#[doc = "Register `SWREG_77` writer"]
pub type W = crate::W<Swreg77Spec>;
#[doc = "Field `OUTPUT_STRM_ST_ADR` reader - output stream start address\n\noutput stream start address"]
pub type OutputStrmStAdrR = crate::FieldReader<u32>;
#[doc = "Field `OUTPUT_STRM_ST_ADR` writer - output stream start address\n\noutput stream start address"]
pub type OutputStrmStAdrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - output stream start address\n\noutput stream start address"]
    #[inline(always)]
    pub fn output_strm_st_adr(&self) -> OutputStrmStAdrR {
        OutputStrmStAdrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - output stream start address\n\noutput stream start address"]
    #[inline(always)]
    #[must_use]
    pub fn output_strm_st_adr(&mut self) -> OutputStrmStAdrW<Swreg77Spec> {
        OutputStrmStAdrW::new(self, 0)
    }
}
#[doc = "output stream start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_77::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_77::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg77Spec;
impl crate::RegisterSpec for Swreg77Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_77::R`](R) reader structure"]
impl crate::Readable for Swreg77Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_77::W`](W) writer structure"]
impl crate::Writable for Swreg77Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_77 to value 0"]
impl crate::Resettable for Swreg77Spec {
    const RESET_VALUE: u32 = 0;
}
