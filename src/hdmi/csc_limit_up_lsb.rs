#[doc = "Register `CSC_LIMIT_UP_LSB` reader"]
pub type R = crate::R<CscLimitUpLsbSpec>;
#[doc = "Register `CSC_LIMIT_UP_LSB` writer"]
pub type W = crate::W<CscLimitUpLsbSpec>;
#[doc = "Field `CSC_LIMIT_UP_LSB` reader - Color Space Converter Matrix Output Upper Limit\n\nRegister LSB"]
pub type CscLimitUpLsbR = crate::FieldReader;
#[doc = "Field `CSC_LIMIT_UP_LSB` writer - Color Space Converter Matrix Output Upper Limit\n\nRegister LSB"]
pub type CscLimitUpLsbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Color Space Converter Matrix Output Upper Limit\n\nRegister LSB"]
    #[inline(always)]
    pub fn csc_limit_up_lsb(&self) -> CscLimitUpLsbR {
        CscLimitUpLsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Color Space Converter Matrix Output Upper Limit\n\nRegister LSB"]
    #[inline(always)]
    #[must_use]
    pub fn csc_limit_up_lsb(&mut self) -> CscLimitUpLsbW<CscLimitUpLsbSpec> {
        CscLimitUpLsbW::new(self, 0)
    }
}
#[doc = "Color Space Converter Matrix output Up Limit Register LSB\n\nFor more details, refer to the HDMI 1.4 specification, paragraph 6.6 Video Quantization\n\nRanges. For an RGB output of 8 bits, the expected limit is 254, and this register must be\n\nconfigured with 0xFE.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_limit_up_lsb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_limit_up_lsb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CscLimitUpLsbSpec;
impl crate::RegisterSpec for CscLimitUpLsbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`csc_limit_up_lsb::R`](R) reader structure"]
impl crate::Readable for CscLimitUpLsbSpec {}
#[doc = "`write(|w| ..)` method takes [`csc_limit_up_lsb::W`](W) writer structure"]
impl crate::Writable for CscLimitUpLsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CSC_LIMIT_UP_LSB to value 0xff"]
impl crate::Resettable for CscLimitUpLsbSpec {
    const RESET_VALUE: u8 = 0xff;
}
