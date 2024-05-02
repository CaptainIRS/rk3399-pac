#[doc = "Register `AWB_GAIN_RB` reader"]
pub type R = crate::R<AwbGainRbSpec>;
#[doc = "Register `AWB_GAIN_RB` writer"]
pub type W = crate::W<AwbGainRbSpec>;
#[doc = "Field `AWB_GAIN_B` reader - gain value for blue component 100h = 1, unsigned\n\ninteger value, range 0 to 4 with 8 bit fractional part\n\n"]
pub type AwbGainBR = crate::FieldReader<u16>;
#[doc = "Field `AWB_GAIN_B` writer - gain value for blue component 100h = 1, unsigned\n\ninteger value, range 0 to 4 with 8 bit fractional part\n\n"]
pub type AwbGainBW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `AWB_GAIN_R` reader - gain value for red component 100h = 1, unsigned\n\ninteger value, range 0 to 4 with 8 bit fractional part"]
pub type AwbGainRR = crate::FieldReader<u16>;
#[doc = "Field `AWB_GAIN_R` writer - gain value for red component 100h = 1, unsigned\n\ninteger value, range 0 to 4 with 8 bit fractional part"]
pub type AwbGainRW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - gain value for blue component 100h = 1, unsigned\n\ninteger value, range 0 to 4 with 8 bit fractional part\n\n"]
    #[inline(always)]
    pub fn awb_gain_b(&self) -> AwbGainBR {
        AwbGainBR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - gain value for red component 100h = 1, unsigned\n\ninteger value, range 0 to 4 with 8 bit fractional part"]
    #[inline(always)]
    pub fn awb_gain_r(&self) -> AwbGainRR {
        AwbGainRR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - gain value for blue component 100h = 1, unsigned\n\ninteger value, range 0 to 4 with 8 bit fractional part\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn awb_gain_b(&mut self) -> AwbGainBW<AwbGainRbSpec> {
        AwbGainBW::new(self, 0)
    }
    #[doc = "Bits 16:25 - gain value for red component 100h = 1, unsigned\n\ninteger value, range 0 to 4 with 8 bit fractional part"]
    #[inline(always)]
    #[must_use]
    pub fn awb_gain_r(&mut self) -> AwbGainRW<AwbGainRbSpec> {
        AwbGainRW::new(self, 16)
    }
}
#[doc = "Auto white balance gain red and blue\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awb_gain_rb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awb_gain_rb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AwbGainRbSpec;
impl crate::RegisterSpec for AwbGainRbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awb_gain_rb::R`](R) reader structure"]
impl crate::Readable for AwbGainRbSpec {}
#[doc = "`write(|w| ..)` method takes [`awb_gain_rb::W`](W) writer structure"]
impl crate::Writable for AwbGainRbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AWB_GAIN_RB to value 0x0100_0100"]
impl crate::Resettable for AwbGainRbSpec {
    const RESET_VALUE: u32 = 0x0100_0100;
}
