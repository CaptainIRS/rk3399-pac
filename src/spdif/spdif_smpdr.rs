#[doc = "Register `SPDIF_SMPDR` reader"]
pub type R = crate::R<SpdifSmpdrSpec>;
#[doc = "Register `SPDIF_SMPDR` writer"]
pub type W = crate::W<SpdifSmpdrSpec>;
#[doc = "Field `SMPDR` reader - Sample Data Register\n\nSample Data Register"]
pub type SmpdrR = crate::FieldReader<u32>;
#[doc = "Field `SMPDR` writer - Sample Data Register\n\nSample Data Register"]
pub type SmpdrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Sample Data Register\n\nSample Data Register"]
    #[inline(always)]
    pub fn smpdr(&self) -> SmpdrR {
        SmpdrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Sample Data Register\n\nSample Data Register"]
    #[inline(always)]
    #[must_use]
    pub fn smpdr(&mut self) -> SmpdrW<SpdifSmpdrSpec> {
        SmpdrW::new(self, 0)
    }
}
#[doc = "Sample Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdif_smpdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdif_smpdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdifSmpdrSpec;
impl crate::RegisterSpec for SpdifSmpdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdif_smpdr::R`](R) reader structure"]
impl crate::Readable for SpdifSmpdrSpec {}
#[doc = "`write(|w| ..)` method takes [`spdif_smpdr::W`](W) writer structure"]
impl crate::Writable for SpdifSmpdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPDIF_SMPDR to value 0"]
impl crate::Resettable for SpdifSmpdrSpec {
    const RESET_VALUE: u32 = 0;
}
