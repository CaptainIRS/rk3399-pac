#[doc = "Register `AWB_GAIN_G` reader"]
pub type R = crate::R<AwbGainGSpec>;
#[doc = "Register `AWB_GAIN_G` writer"]
pub type W = crate::W<AwbGainGSpec>;
#[doc = "Field `AWB_GAIN_GB` reader - gain value for green component in blue line 100h = 1,\n\nunsigned integer value, range 0 to 4 with 8 bit fractional\n\npart\n\n"]
pub type AwbGainGbR = crate::FieldReader<u16>;
#[doc = "Field `AWB_GAIN_GB` writer - gain value for green component in blue line 100h = 1,\n\nunsigned integer value, range 0 to 4 with 8 bit fractional\n\npart\n\n"]
pub type AwbGainGbW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `AWB_GAIN_GR` reader - gain value for green component in red line 100h = 1,\n\nunsigned integer value, range 0 to 4 with 8 bit fractional\n\npart"]
pub type AwbGainGrR = crate::FieldReader<u16>;
#[doc = "Field `AWB_GAIN_GR` writer - gain value for green component in red line 100h = 1,\n\nunsigned integer value, range 0 to 4 with 8 bit fractional\n\npart"]
pub type AwbGainGrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - gain value for green component in blue line 100h = 1,\n\nunsigned integer value, range 0 to 4 with 8 bit fractional\n\npart\n\n"]
    #[inline(always)]
    pub fn awb_gain_gb(&self) -> AwbGainGbR {
        AwbGainGbR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - gain value for green component in red line 100h = 1,\n\nunsigned integer value, range 0 to 4 with 8 bit fractional\n\npart"]
    #[inline(always)]
    pub fn awb_gain_gr(&self) -> AwbGainGrR {
        AwbGainGrR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - gain value for green component in blue line 100h = 1,\n\nunsigned integer value, range 0 to 4 with 8 bit fractional\n\npart\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn awb_gain_gb(&mut self) -> AwbGainGbW<AwbGainGSpec> {
        AwbGainGbW::new(self, 0)
    }
    #[doc = "Bits 16:25 - gain value for green component in red line 100h = 1,\n\nunsigned integer value, range 0 to 4 with 8 bit fractional\n\npart"]
    #[inline(always)]
    #[must_use]
    pub fn awb_gain_gr(&mut self) -> AwbGainGrW<AwbGainGSpec> {
        AwbGainGrW::new(self, 16)
    }
}
#[doc = "Auto white balance gain green\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awb_gain_g::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awb_gain_g::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AwbGainGSpec;
impl crate::RegisterSpec for AwbGainGSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awb_gain_g::R`](R) reader structure"]
impl crate::Readable for AwbGainGSpec {}
#[doc = "`write(|w| ..)` method takes [`awb_gain_g::W`](W) writer structure"]
impl crate::Writable for AwbGainGSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AWB_GAIN_G to value 0x0100_0100"]
impl crate::Resettable for AwbGainGSpec {
    const RESET_VALUE: u32 = 0x0100_0100;
}
