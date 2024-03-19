#[doc = "Register `AUD_CONF1` reader"]
pub type R = crate::R<AudConf1Spec>;
#[doc = "Register `AUD_CONF1` writer"]
pub type W = crate::W<AudConf1Spec>;
#[doc = "Field `I2S_WIDTH` reader - I2S input data width I2S_width\\[4:0\\]
| Action\n\n00000b-01111b | Not used\n\n10000b | 16 bit data samples at input\n\n10001b | 17 bit data samples at input\n\n10010b | 18 bit data samples at input\n\n10011b | 19 bit data samples at input\n\n10100b | 20 bit data samples at input\n\n10101b | 21 bit data samples at input\n\n10110b | 22 bit data samples at input\n\n10111b | 23 bit data samples at input\n\n11000b | 24 bit data samples at input 11001b-\n\n11111b | Not Used"]
pub type I2sWidthR = crate::FieldReader;
#[doc = "Field `I2S_WIDTH` writer - I2S input data width I2S_width\\[4:0\\]
| Action\n\n00000b-01111b | Not used\n\n10000b | 16 bit data samples at input\n\n10001b | 17 bit data samples at input\n\n10010b | 18 bit data samples at input\n\n10011b | 19 bit data samples at input\n\n10100b | 20 bit data samples at input\n\n10101b | 21 bit data samples at input\n\n10110b | 22 bit data samples at input\n\n10111b | 23 bit data samples at input\n\n11000b | 24 bit data samples at input 11001b-\n\n11111b | Not Used"]
pub type I2sWidthW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - I2S input data width I2S_width\\[4:0\\]
| Action\n\n00000b-01111b | Not used\n\n10000b | 16 bit data samples at input\n\n10001b | 17 bit data samples at input\n\n10010b | 18 bit data samples at input\n\n10011b | 19 bit data samples at input\n\n10100b | 20 bit data samples at input\n\n10101b | 21 bit data samples at input\n\n10110b | 22 bit data samples at input\n\n10111b | 23 bit data samples at input\n\n11000b | 24 bit data samples at input 11001b-\n\n11111b | Not Used"]
    #[inline(always)]
    pub fn i2s_width(&self) -> I2sWidthR {
        I2sWidthR::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4 - I2S input data width I2S_width\\[4:0\\]
| Action\n\n00000b-01111b | Not used\n\n10000b | 16 bit data samples at input\n\n10001b | 17 bit data samples at input\n\n10010b | 18 bit data samples at input\n\n10011b | 19 bit data samples at input\n\n10100b | 20 bit data samples at input\n\n10101b | 21 bit data samples at input\n\n10110b | 22 bit data samples at input\n\n10111b | 23 bit data samples at input\n\n11000b | 24 bit data samples at input 11001b-\n\n11111b | Not Used"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_width(&mut self) -> I2sWidthW<AudConf1Spec> {
        I2sWidthW::new(self, 0)
    }
}
#[doc = "Audio I2S Width Configuration Register 1 This register configures the data\n\nwidth of the input data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_conf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_conf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudConf1Spec;
impl crate::RegisterSpec for AudConf1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`aud_conf1::R`](R) reader structure"]
impl crate::Readable for AudConf1Spec {}
#[doc = "`write(|w| ..)` method takes [`aud_conf1::W`](W) writer structure"]
impl crate::Writable for AudConf1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets AUD_CONF1 to value 0x18"]
impl crate::Resettable for AudConf1Spec {
    const RESET_VALUE: u8 = 0x18;
}
