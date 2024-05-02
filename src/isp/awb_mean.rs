#[doc = "Register `AWB_MEAN` reader"]
pub type R = crate::R<AwbMeanSpec>;
#[doc = "Field `AWB_MEAN_CR` reader - R mean value of Cr within window and frames\n\nmean value of red, if RGB measurement mode is\n\nselected\n\n"]
pub type AwbMeanCrR = crate::FieldReader;
#[doc = "Field `AWB_MEAN_CB` reader - B mean value of Cb within window and frames\n\nmean value of blue, if RGB measurement mode is\n\nselected"]
pub type AwbMeanCbR = crate::FieldReader;
#[doc = "Field `AWB_MEAN_Y` reader - G mean value of Y within window and frames\n\nmean value of green, if RGB measurement mode is\n\nselected"]
pub type AwbMeanYR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - R mean value of Cr within window and frames\n\nmean value of red, if RGB measurement mode is\n\nselected\n\n"]
    #[inline(always)]
    pub fn awb_mean_cr(&self) -> AwbMeanCrR {
        AwbMeanCrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - B mean value of Cb within window and frames\n\nmean value of blue, if RGB measurement mode is\n\nselected"]
    #[inline(always)]
    pub fn awb_mean_cb(&self) -> AwbMeanCbR {
        AwbMeanCbR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - G mean value of Y within window and frames\n\nmean value of green, if RGB measurement mode is\n\nselected"]
    #[inline(always)]
    pub fn awb_mean_y(&self) -> AwbMeanYR {
        AwbMeanYR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Auto white balance measured mean value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awb_mean::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AwbMeanSpec;
impl crate::RegisterSpec for AwbMeanSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awb_mean::R`](R) reader structure"]
impl crate::Readable for AwbMeanSpec {}
#[doc = "`reset()` method sets AWB_MEAN to value 0"]
impl crate::Resettable for AwbMeanSpec {
    const RESET_VALUE: u32 = 0;
}
