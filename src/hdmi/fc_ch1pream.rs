#[doc = "Register `FC_CH1PREAM` reader"]
pub type R = crate::R<FcCh1preamSpec>;
#[doc = "Register `FC_CH1PREAM` writer"]
pub type W = crate::W<FcCh1preamSpec>;
#[doc = "Field `CH1_PREAMBLE_FILTER` reader - When in control mode, configures 6 bits that fill the\n\nchannel 1 data lines not used to transmit the\n\npreamble (for more clarification, refer to the HDMI\n\n1.4b specification)."]
pub type Ch1PreambleFilterR = crate::FieldReader;
#[doc = "Field `CH1_PREAMBLE_FILTER` writer - When in control mode, configures 6 bits that fill the\n\nchannel 1 data lines not used to transmit the\n\npreamble (for more clarification, refer to the HDMI\n\n1.4b specification)."]
pub type Ch1PreambleFilterW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - When in control mode, configures 6 bits that fill the\n\nchannel 1 data lines not used to transmit the\n\npreamble (for more clarification, refer to the HDMI\n\n1.4b specification)."]
    #[inline(always)]
    pub fn ch1_preamble_filter(&self) -> Ch1PreambleFilterR {
        Ch1PreambleFilterR::new(self.bits & 0x3f)
    }
}
impl W {
    #[doc = "Bits 0:5 - When in control mode, configures 6 bits that fill the\n\nchannel 1 data lines not used to transmit the\n\npreamble (for more clarification, refer to the HDMI\n\n1.4b specification)."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_preamble_filter(&mut self) -> Ch1PreambleFilterW<FcCh1preamSpec> {
        Ch1PreambleFilterW::new(self, 0)
    }
}
#[doc = "Frame Composer Channel 1 Non-Preamble Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_ch1pream::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_ch1pream::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcCh1preamSpec;
impl crate::RegisterSpec for FcCh1preamSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_ch1pream::R`](R) reader structure"]
impl crate::Readable for FcCh1preamSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_ch1pream::W`](W) writer structure"]
impl crate::Writable for FcCh1preamSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_CH1PREAM to value 0"]
impl crate::Resettable for FcCh1preamSpec {
    const RESET_VALUE: u8 = 0;
}
