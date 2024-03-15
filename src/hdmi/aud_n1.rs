#[doc = "Register `AUD_N1` reader"]
pub type R = crate::R<AudN1Spec>;
#[doc = "Register `AUD_N1` writer"]
pub type W = crate::W<AudN1Spec>;
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
    pub fn audn(&mut self) -> AudnW<AudN1Spec> {
        AudnW::new(self, 0)
    }
}
#[doc = "HDMI Audio Clock Regenerator N value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_n1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_n1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudN1Spec;
impl crate::RegisterSpec for AudN1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`aud_n1::R`](R) reader structure"]
impl crate::Readable for AudN1Spec {}
#[doc = "`write(|w| ..)` method takes [`aud_n1::W`](W) writer structure"]
impl crate::Writable for AudN1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets AUD_N1 to value 0"]
impl crate::Resettable for AudN1Spec {
    const RESET_VALUE: u8 = 0;
}
