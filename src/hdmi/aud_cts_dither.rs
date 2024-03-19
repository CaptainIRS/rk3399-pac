#[doc = "Register `AUD_CTS_DITHER` reader"]
pub type R = crate::R<AudCtsDitherSpec>;
#[doc = "Register `AUD_CTS_DITHER` writer"]
pub type W = crate::W<AudCtsDitherSpec>;
#[doc = "Field `DIVIDEND` reader - Dither dividend (4'd1 if no CTS Dither). This field\n\nshould be configured with the value of dividend from\n\nthe HDMI specification."]
pub type DividendR = crate::FieldReader;
#[doc = "Field `DIVIDEND` writer - Dither dividend (4'd1 if no CTS Dither). This field\n\nshould be configured with the value of dividend from\n\nthe HDMI specification."]
pub type DividendW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIVISOR` reader - Dither divisor (4'd1 if no CTS Dither). This field\n\nshould be configured with the value of divisor from\n\nthe HDMI specification."]
pub type DivisorR = crate::FieldReader;
#[doc = "Field `DIVISOR` writer - Dither divisor (4'd1 if no CTS Dither). This field\n\nshould be configured with the value of divisor from\n\nthe HDMI specification."]
pub type DivisorW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Dither dividend (4'd1 if no CTS Dither). This field\n\nshould be configured with the value of dividend from\n\nthe HDMI specification."]
    #[inline(always)]
    pub fn dividend(&self) -> DividendR {
        DividendR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Dither divisor (4'd1 if no CTS Dither). This field\n\nshould be configured with the value of divisor from\n\nthe HDMI specification."]
    #[inline(always)]
    pub fn divisor(&self) -> DivisorR {
        DivisorR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Dither dividend (4'd1 if no CTS Dither). This field\n\nshould be configured with the value of dividend from\n\nthe HDMI specification."]
    #[inline(always)]
    #[must_use]
    pub fn dividend(&mut self) -> DividendW<AudCtsDitherSpec> {
        DividendW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Dither divisor (4'd1 if no CTS Dither). This field\n\nshould be configured with the value of divisor from\n\nthe HDMI specification."]
    #[inline(always)]
    #[must_use]
    pub fn divisor(&mut self) -> DivisorW<AudCtsDitherSpec> {
        DivisorW::new(self, 4)
    }
}
#[doc = "Audio CTS Dither Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_cts_dither::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_cts_dither::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudCtsDitherSpec;
impl crate::RegisterSpec for AudCtsDitherSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`aud_cts_dither::R`](R) reader structure"]
impl crate::Readable for AudCtsDitherSpec {}
#[doc = "`write(|w| ..)` method takes [`aud_cts_dither::W`](W) writer structure"]
impl crate::Writable for AudCtsDitherSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets AUD_CTS_DITHER to value 0x11"]
impl crate::Resettable for AudCtsDitherSpec {
    const RESET_VALUE: u8 = 0x11;
}
