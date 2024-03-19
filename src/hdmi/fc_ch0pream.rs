#[doc = "Register `FC_CH0PREAM` reader"]
pub type R = crate::R<FcCh0preamSpec>;
#[doc = "Register `FC_CH0PREAM` writer"]
pub type W = crate::W<FcCh0preamSpec>;
#[doc = "Field `CH0_PREAMBLE_FILTER` reader - When in control mode, configures 8 bits that fill the\n\nchannel 0 data lines not used to transmit the\n\npreamble (for more clarification, refer to the HDMI\n\n1.4b specification)."]
pub type Ch0PreambleFilterR = crate::FieldReader;
#[doc = "Field `CH0_PREAMBLE_FILTER` writer - When in control mode, configures 8 bits that fill the\n\nchannel 0 data lines not used to transmit the\n\npreamble (for more clarification, refer to the HDMI\n\n1.4b specification)."]
pub type Ch0PreambleFilterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - When in control mode, configures 8 bits that fill the\n\nchannel 0 data lines not used to transmit the\n\npreamble (for more clarification, refer to the HDMI\n\n1.4b specification)."]
    #[inline(always)]
    pub fn ch0_preamble_filter(&self) -> Ch0PreambleFilterR {
        Ch0PreambleFilterR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - When in control mode, configures 8 bits that fill the\n\nchannel 0 data lines not used to transmit the\n\npreamble (for more clarification, refer to the HDMI\n\n1.4b specification)."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_preamble_filter(&mut self) -> Ch0PreambleFilterW<FcCh0preamSpec> {
        Ch0PreambleFilterW::new(self, 0)
    }
}
#[doc = "Frame Composer Channel 0 Non-Preamble Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_ch0pream::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_ch0pream::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcCh0preamSpec;
impl crate::RegisterSpec for FcCh0preamSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_ch0pream::R`](R) reader structure"]
impl crate::Readable for FcCh0preamSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_ch0pream::W`](W) writer structure"]
impl crate::Writable for FcCh0preamSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_CH0PREAM to value 0"]
impl crate::Resettable for FcCh0preamSpec {
    const RESET_VALUE: u8 = 0;
}
