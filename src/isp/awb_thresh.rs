#[doc = "Register `AWB_THRESH` reader"]
pub type R = crate::R<AwbThreshSpec>;
#[doc = "Register `AWB_THRESH` writer"]
pub type W = crate::W<AwbThreshSpec>;
#[doc = "Field `AWB_MIN_C` reader - Chrominance minimum value, only consider pixels\n\nwith Cb/Cr each greater than threshold value for WB\n\nmeasurements"]
pub type AwbMinCR = crate::FieldReader;
#[doc = "Field `AWB_MIN_C` writer - Chrominance minimum value, only consider pixels\n\nwith Cb/Cr each greater than threshold value for WB\n\nmeasurements"]
pub type AwbMinCW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AWB_MAX_CSUM` reader - Chrominance sum maximum value, only consider\n\npixels with Cb+Cr smaller than threshold for WB\n\nmeasurements"]
pub type AwbMaxCsumR = crate::FieldReader;
#[doc = "Field `AWB_MAX_CSUM` writer - Chrominance sum maximum value, only consider\n\npixels with Cb+Cr smaller than threshold for WB\n\nmeasurements"]
pub type AwbMaxCsumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AWB_MIN_Y` reader - MAX_G\n\nLuminance minimum value, only consider pixels with\n\nluminance greater than threshold for the WB\n\nmeasurement\n\nmaximum green value, if RGB measurement mode is"]
pub type AwbMinYR = crate::FieldReader;
#[doc = "Field `AWB_MIN_Y` writer - MAX_G\n\nLuminance minimum value, only consider pixels with\n\nluminance greater than threshold for the WB\n\nmeasurement\n\nmaximum green value, if RGB measurement mode is"]
pub type AwbMinYW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AWB_MAX_Y` reader - Luminance maximum value, only consider pixels with\n\nluminance smaller than threshold for the WB\n\nmeasurement (must be enabled by register AWB_MODE\n\nbit AWB_MAX_EN). Not valid for RGB measurement mode."]
pub type AwbMaxYR = crate::FieldReader;
#[doc = "Field `AWB_MAX_Y` writer - Luminance maximum value, only consider pixels with\n\nluminance smaller than threshold for the WB\n\nmeasurement (must be enabled by register AWB_MODE\n\nbit AWB_MAX_EN). Not valid for RGB measurement mode."]
pub type AwbMaxYW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Chrominance minimum value, only consider pixels\n\nwith Cb/Cr each greater than threshold value for WB\n\nmeasurements"]
    #[inline(always)]
    pub fn awb_min_c(&self) -> AwbMinCR {
        AwbMinCR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Chrominance sum maximum value, only consider\n\npixels with Cb+Cr smaller than threshold for WB\n\nmeasurements"]
    #[inline(always)]
    pub fn awb_max_csum(&self) -> AwbMaxCsumR {
        AwbMaxCsumR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - MAX_G\n\nLuminance minimum value, only consider pixels with\n\nluminance greater than threshold for the WB\n\nmeasurement\n\nmaximum green value, if RGB measurement mode is"]
    #[inline(always)]
    pub fn awb_min_y(&self) -> AwbMinYR {
        AwbMinYR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Luminance maximum value, only consider pixels with\n\nluminance smaller than threshold for the WB\n\nmeasurement (must be enabled by register AWB_MODE\n\nbit AWB_MAX_EN). Not valid for RGB measurement mode."]
    #[inline(always)]
    pub fn awb_max_y(&self) -> AwbMaxYR {
        AwbMaxYR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Chrominance minimum value, only consider pixels\n\nwith Cb/Cr each greater than threshold value for WB\n\nmeasurements"]
    #[inline(always)]
    #[must_use]
    pub fn awb_min_c(&mut self) -> AwbMinCW<AwbThreshSpec> {
        AwbMinCW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Chrominance sum maximum value, only consider\n\npixels with Cb+Cr smaller than threshold for WB\n\nmeasurements"]
    #[inline(always)]
    #[must_use]
    pub fn awb_max_csum(&mut self) -> AwbMaxCsumW<AwbThreshSpec> {
        AwbMaxCsumW::new(self, 8)
    }
    #[doc = "Bits 16:23 - MAX_G\n\nLuminance minimum value, only consider pixels with\n\nluminance greater than threshold for the WB\n\nmeasurement\n\nmaximum green value, if RGB measurement mode is"]
    #[inline(always)]
    #[must_use]
    pub fn awb_min_y(&mut self) -> AwbMinYW<AwbThreshSpec> {
        AwbMinYW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Luminance maximum value, only consider pixels with\n\nluminance smaller than threshold for the WB\n\nmeasurement (must be enabled by register AWB_MODE\n\nbit AWB_MAX_EN). Not valid for RGB measurement mode."]
    #[inline(always)]
    #[must_use]
    pub fn awb_max_y(&mut self) -> AwbMaxYW<AwbThreshSpec> {
        AwbMaxYW::new(self, 24)
    }
}
#[doc = "Auto white balance threshold values\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awb_thresh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awb_thresh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AwbThreshSpec;
impl crate::RegisterSpec for AwbThreshSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awb_thresh::R`](R) reader structure"]
impl crate::Readable for AwbThreshSpec {}
#[doc = "`write(|w| ..)` method takes [`awb_thresh::W`](W) writer structure"]
impl crate::Writable for AwbThreshSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AWB_THRESH to value 0xe9c0_1010"]
impl crate::Resettable for AwbThreshSpec {
    const RESET_VALUE: u32 = 0xe9c0_1010;
}
