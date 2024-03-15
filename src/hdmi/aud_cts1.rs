#[doc = "Register `AUD_CTS1` reader"]
pub type R = crate::R<AudCts1Spec>;
#[doc = "Register `AUD_CTS1` writer"]
pub type W = crate::W<AudCts1Spec>;
#[doc = "Field `AUDCTS` reader - HDMI Audio Clock Regenerator CTS calculated value. This value can be manually set using the CTS_manual (AUD_CTS3) mechanism."]
pub type AudctsR = crate::FieldReader;
#[doc = "Field `AUDCTS` writer - HDMI Audio Clock Regenerator CTS calculated value. This value can be manually set using the CTS_manual (AUD_CTS3) mechanism."]
pub type AudctsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - HDMI Audio Clock Regenerator CTS calculated value. This value can be manually set using the CTS_manual (AUD_CTS3) mechanism."]
    #[inline(always)]
    pub fn audcts(&self) -> AudctsR {
        AudctsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - HDMI Audio Clock Regenerator CTS calculated value. This value can be manually set using the CTS_manual (AUD_CTS3) mechanism."]
    #[inline(always)]
    #[must_use]
    pub fn audcts(&mut self) -> AudctsW<AudCts1Spec> {
        AudctsW::new(self, 0)
    }
}
#[doc = "HDMI Audio Clock Regenerator CTS calculated value. This value can be manually set using the CTS_manual (AUD_CTS3) mechanism.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_cts1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_cts1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudCts1Spec;
impl crate::RegisterSpec for AudCts1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`aud_cts1::R`](R) reader structure"]
impl crate::Readable for AudCts1Spec {}
#[doc = "`write(|w| ..)` method takes [`aud_cts1::W`](W) writer structure"]
impl crate::Writable for AudCts1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets AUD_CTS1 to value 0"]
impl crate::Resettable for AudCts1Spec {
    const RESET_VALUE: u8 = 0;
}
