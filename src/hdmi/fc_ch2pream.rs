#[doc = "Register `FC_CH2PREAM` reader"]
pub type R = crate::R<FcCh2preamSpec>;
#[doc = "Register `FC_CH2PREAM` writer"]
pub type W = crate::W<FcCh2preamSpec>;
#[doc = "Field `CH2_PREAMBLE_FILTER` reader - When in control mode, configures 6 bits that fill the channel 2 data lines not used to transmit the preamble (for more clarification, refer to the HDMI 1.4b specification)."]
pub type Ch2PreambleFilterR = crate::FieldReader;
#[doc = "Field `CH2_PREAMBLE_FILTER` writer - When in control mode, configures 6 bits that fill the channel 2 data lines not used to transmit the preamble (for more clarification, refer to the HDMI 1.4b specification)."]
pub type Ch2PreambleFilterW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - When in control mode, configures 6 bits that fill the channel 2 data lines not used to transmit the preamble (for more clarification, refer to the HDMI 1.4b specification)."]
    #[inline(always)]
    pub fn ch2_preamble_filter(&self) -> Ch2PreambleFilterR {
        Ch2PreambleFilterR::new(self.bits & 0x3f)
    }
}
impl W {
    #[doc = "Bits 0:5 - When in control mode, configures 6 bits that fill the channel 2 data lines not used to transmit the preamble (for more clarification, refer to the HDMI 1.4b specification)."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_preamble_filter(&mut self) -> Ch2PreambleFilterW<FcCh2preamSpec> {
        Ch2PreambleFilterW::new(self, 0)
    }
}
#[doc = "When in control mode, configures 6 bits that fill the channel 2 data lines not used to transmit the preamble (for more clarification, refer to the HDMI 1.4b specification).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_ch2pream::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_ch2pream::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcCh2preamSpec;
impl crate::RegisterSpec for FcCh2preamSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_ch2pream::R`](R) reader structure"]
impl crate::Readable for FcCh2preamSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_ch2pream::W`](W) writer structure"]
impl crate::Writable for FcCh2preamSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_CH2PREAM to value 0"]
impl crate::Resettable for FcCh2preamSpec {
    const RESET_VALUE: u8 = 0;
}
