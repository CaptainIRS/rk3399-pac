#[doc = "Register `FC_AUDSCHNL7` reader"]
pub type R = crate::R<FcAudschnl7Spec>;
#[doc = "Register `FC_AUDSCHNL7` writer"]
pub type W = crate::W<FcAudschnl7Spec>;
#[doc = "Field `OIEC_SAMPFREQ` reader - Sampling frequency"]
pub type OiecSampfreqR = crate::FieldReader;
#[doc = "Field `OIEC_SAMPFREQ` writer - Sampling frequency"]
pub type OiecSampfreqW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OIEC_CLKACCURACY` reader - Clock accuracy"]
pub type OiecClkaccuracyR = crate::FieldReader;
#[doc = "Field `OIEC_CLKACCURACY` writer - Clock accuracy"]
pub type OiecClkaccuracyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OIEC_SAMPFREQ_EXT` reader - Sampling frequency (channel status bits 31 and\n\n30)"]
pub type OiecSampfreqExtR = crate::FieldReader;
#[doc = "Field `OIEC_SAMPFREQ_EXT` writer - Sampling frequency (channel status bits 31 and\n\n30)"]
pub type OiecSampfreqExtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - Sampling frequency"]
    #[inline(always)]
    pub fn oiec_sampfreq(&self) -> OiecSampfreqR {
        OiecSampfreqR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - Clock accuracy"]
    #[inline(always)]
    pub fn oiec_clkaccuracy(&self) -> OiecClkaccuracyR {
        OiecClkaccuracyR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Sampling frequency (channel status bits 31 and\n\n30)"]
    #[inline(always)]
    pub fn oiec_sampfreq_ext(&self) -> OiecSampfreqExtR {
        OiecSampfreqExtR::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sampling frequency"]
    #[inline(always)]
    #[must_use]
    pub fn oiec_sampfreq(&mut self) -> OiecSampfreqW<FcAudschnl7Spec> {
        OiecSampfreqW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Clock accuracy"]
    #[inline(always)]
    #[must_use]
    pub fn oiec_clkaccuracy(&mut self) -> OiecClkaccuracyW<FcAudschnl7Spec> {
        OiecClkaccuracyW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Sampling frequency (channel status bits 31 and\n\n30)"]
    #[inline(always)]
    #[must_use]
    pub fn oiec_sampfreq_ext(&mut self) -> OiecSampfreqExtW<FcAudschnl7Spec> {
        OiecSampfreqExtW::new(self, 6)
    }
}
#[doc = "Frame Composer Audio Sample Channel Status Configuration Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audschnl7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_audschnl7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAudschnl7Spec;
impl crate::RegisterSpec for FcAudschnl7Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_audschnl7::R`](R) reader structure"]
impl crate::Readable for FcAudschnl7Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_audschnl7::W`](W) writer structure"]
impl crate::Writable for FcAudschnl7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_AUDSCHNL7 to value 0"]
impl crate::Resettable for FcAudschnl7Spec {
    const RESET_VALUE: u8 = 0;
}
