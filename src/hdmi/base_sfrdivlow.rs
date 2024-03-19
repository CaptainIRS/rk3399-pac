#[doc = "Register `BASE_SFRDIVLOW` reader"]
pub type R = crate::R<BaseSfrdivlowSpec>;
#[doc = "Register `BASE_SFRDIVLOW` writer"]
pub type W = crate::W<BaseSfrdivlowSpec>;
#[doc = "Field `BASE_SFRDIV_LO` reader - SFR clock divider Low\n\nThis register must be configured with the 8 least-\n\nsignificant bits of the value sfrclk frequency divided by\n\n1000 (for example, for 27 MHz base_sfrdiv\\[14:0\\]
=\n\n27027). The configured data is used to generate a\n\nreference pulse of 1ms period that is needed by several\n\ntimers within the controller."]
pub type BaseSfrdivLoR = crate::FieldReader;
#[doc = "Field `BASE_SFRDIV_LO` writer - SFR clock divider Low\n\nThis register must be configured with the 8 least-\n\nsignificant bits of the value sfrclk frequency divided by\n\n1000 (for example, for 27 MHz base_sfrdiv\\[14:0\\]
=\n\n27027). The configured data is used to generate a\n\nreference pulse of 1ms period that is needed by several\n\ntimers within the controller."]
pub type BaseSfrdivLoW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SFR clock divider Low\n\nThis register must be configured with the 8 least-\n\nsignificant bits of the value sfrclk frequency divided by\n\n1000 (for example, for 27 MHz base_sfrdiv\\[14:0\\]
=\n\n27027). The configured data is used to generate a\n\nreference pulse of 1ms period that is needed by several\n\ntimers within the controller."]
    #[inline(always)]
    pub fn base_sfrdiv_lo(&self) -> BaseSfrdivLoR {
        BaseSfrdivLoR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - SFR clock divider Low\n\nThis register must be configured with the 8 least-\n\nsignificant bits of the value sfrclk frequency divided by\n\n1000 (for example, for 27 MHz base_sfrdiv\\[14:0\\]
=\n\n27027). The configured data is used to generate a\n\nreference pulse of 1ms period that is needed by several\n\ntimers within the controller."]
    #[inline(always)]
    #[must_use]
    pub fn base_sfrdiv_lo(&mut self) -> BaseSfrdivLoW<BaseSfrdivlowSpec> {
        BaseSfrdivLoW::new(self, 0)
    }
}
#[doc = "SFR Clock Base Time Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`base_sfrdivlow::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`base_sfrdivlow::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BaseSfrdivlowSpec;
impl crate::RegisterSpec for BaseSfrdivlowSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`base_sfrdivlow::R`](R) reader structure"]
impl crate::Readable for BaseSfrdivlowSpec {}
#[doc = "`write(|w| ..)` method takes [`base_sfrdivlow::W`](W) writer structure"]
impl crate::Writable for BaseSfrdivlowSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets BASE_SFRDIVLOW to value 0x93"]
impl crate::Resettable for BaseSfrdivlowSpec {
    const RESET_VALUE: u8 = 0x93;
}
