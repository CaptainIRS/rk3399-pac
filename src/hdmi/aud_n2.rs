#[doc = "Register `AUD_N2` reader"]
pub type R = crate::R<AudN2Spec>;
#[doc = "Register `AUD_N2` writer"]
pub type W = crate::W<AudN2Spec>;
#[doc = "Field `AUDN` reader - HDMI Audio Clock Regenerator N value"]
pub type AudnR = crate::FieldReader;
#[doc = "Field `AUDN` writer - HDMI Audio Clock Regenerator N value"]
pub type AudnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - HDMI Audio Clock Regenerator N value"]
    #[inline(always)]
    pub fn audn(&self) -> AudnR {
        AudnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - HDMI Audio Clock Regenerator N value"]
    #[inline(always)]
    #[must_use]
    pub fn audn(&mut self) -> AudnW<AudN2Spec> {
        AudnW::new(self, 0)
    }
}
#[doc = "Audio Clock Regenerator N Value Register 2 For N expected values, refer to\n\nthe HDMI 1.4b specification.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_n2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_n2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudN2Spec;
impl crate::RegisterSpec for AudN2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`aud_n2::R`](R) reader structure"]
impl crate::Readable for AudN2Spec {}
#[doc = "`write(|w| ..)` method takes [`aud_n2::W`](W) writer structure"]
impl crate::Writable for AudN2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets AUD_N2 to value 0"]
impl crate::Resettable for AudN2Spec {
    const RESET_VALUE: u8 = 0;
}
