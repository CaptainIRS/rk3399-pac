#[doc = "Register `BASE_SFRDIVHIGH` reader"]
pub type R = crate::R<BaseSfrdivhighSpec>;
#[doc = "Register `BASE_SFRDIVHIGH` writer"]
pub type W = crate::W<BaseSfrdivhighSpec>;
#[doc = "Field `BASE_SFRDIV_HI` reader - SFR clock divider High\n\nThis register must be configured with the 7 most-\n\nsignificant bits of the value sfrclk frequency divided by\n\n1000 (for example, for 27 MHz base_sfrdiv\\[14:0\\]\n\n= 27027). The configured data is used to generate a\n\nreference pulse of 1ms period that is needed by several\n\ntimers within the controller."]
pub type BaseSfrdivHiR = crate::FieldReader;
#[doc = "Field `BASE_SFRDIV_HI` writer - SFR clock divider High\n\nThis register must be configured with the 7 most-\n\nsignificant bits of the value sfrclk frequency divided by\n\n1000 (for example, for 27 MHz base_sfrdiv\\[14:0\\]\n\n= 27027). The configured data is used to generate a\n\nreference pulse of 1ms period that is needed by several\n\ntimers within the controller."]
pub type BaseSfrdivHiW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - SFR clock divider High\n\nThis register must be configured with the 7 most-\n\nsignificant bits of the value sfrclk frequency divided by\n\n1000 (for example, for 27 MHz base_sfrdiv\\[14:0\\]\n\n= 27027). The configured data is used to generate a\n\nreference pulse of 1ms period that is needed by several\n\ntimers within the controller."]
    #[inline(always)]
    pub fn base_sfrdiv_hi(&self) -> BaseSfrdivHiR {
        BaseSfrdivHiR::new(self.bits & 0x7f)
    }
}
impl W {
    #[doc = "Bits 0:6 - SFR clock divider High\n\nThis register must be configured with the 7 most-\n\nsignificant bits of the value sfrclk frequency divided by\n\n1000 (for example, for 27 MHz base_sfrdiv\\[14:0\\]\n\n= 27027). The configured data is used to generate a\n\nreference pulse of 1ms period that is needed by several\n\ntimers within the controller."]
    #[inline(always)]
    #[must_use]
    pub fn base_sfrdiv_hi(&mut self) -> BaseSfrdivHiW<BaseSfrdivhighSpec> {
        BaseSfrdivHiW::new(self, 0)
    }
}
#[doc = "SFR Clock Base Time Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`base_sfrdivhigh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`base_sfrdivhigh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BaseSfrdivhighSpec;
impl crate::RegisterSpec for BaseSfrdivhighSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`base_sfrdivhigh::R`](R) reader structure"]
impl crate::Readable for BaseSfrdivhighSpec {}
#[doc = "`write(|w| ..)` method takes [`base_sfrdivhigh::W`](W) writer structure"]
impl crate::Writable for BaseSfrdivhighSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets BASE_SFRDIVHIGH to value 0x69"]
impl crate::Resettable for BaseSfrdivhighSpec {
    const RESET_VALUE: u8 = 0x69;
}
