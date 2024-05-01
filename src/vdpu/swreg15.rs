#[doc = "Register `SWREG15` reader"]
pub type R = crate::R<Swreg15Spec>;
#[doc = "Register `SWREG15` writer"]
pub type W = crate::W<Swreg15Spec>;
#[doc = "Field `SW_RANGEMAP_Y` reader - the value of Y component range map\n\nVC- 1:y range map value +9"]
pub type SwRangemapYR = crate::FieldReader;
#[doc = "Field `SW_RANGEMAP_Y` writer - the value of Y component range map\n\nVC- 1:y range map value +9"]
pub type SwRangemapYW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SW_YUV_CONV_RANGE` reader - to declaration the range of YCbCr when RGB conversion\n\nY:\n\n0:16~235\n\n1:0~255\n\nC:\n\n0:16~240\n\n1:0~255"]
pub type SwYuvConvRangeR = crate::BitReader;
#[doc = "Field `SW_YUV_CONV_RANGE` writer - to declaration the range of YCbCr when RGB conversion\n\nY:\n\n0:16~235\n\n1:0~255\n\nC:\n\n0:16~240\n\n1:0~255"]
pub type SwYuvConvRangeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_RANGEMAP_COEF_C` reader - the value of chrominance component range map\n\nVC- 1:c range map value +9"]
pub type SwRangemapCoefCR = crate::FieldReader;
#[doc = "Field `SW_RANGEMAP_COEF_C` writer - the value of chrominance component range map\n\nVC- 1:c range map value +9"]
pub type SwRangemapCoefCW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - the value of Y component range map\n\nVC- 1:y range map value +9"]
    #[inline(always)]
    pub fn sw_rangemap_y(&self) -> SwRangemapYR {
        SwRangemapYR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - to declaration the range of YCbCr when RGB conversion\n\nY:\n\n0:16~235\n\n1:0~255\n\nC:\n\n0:16~240\n\n1:0~255"]
    #[inline(always)]
    pub fn sw_yuv_conv_range(&self) -> SwYuvConvRangeR {
        SwYuvConvRangeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:12 - the value of chrominance component range map\n\nVC- 1:c range map value +9"]
    #[inline(always)]
    pub fn sw_rangemap_coef_c(&self) -> SwRangemapCoefCR {
        SwRangemapCoefCR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - the value of Y component range map\n\nVC- 1:y range map value +9"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rangemap_y(&mut self) -> SwRangemapYW<Swreg15Spec> {
        SwRangemapYW::new(self, 0)
    }
    #[doc = "Bit 5 - to declaration the range of YCbCr when RGB conversion\n\nY:\n\n0:16~235\n\n1:0~255\n\nC:\n\n0:16~240\n\n1:0~255"]
    #[inline(always)]
    #[must_use]
    pub fn sw_yuv_conv_range(&mut self) -> SwYuvConvRangeW<Swreg15Spec> {
        SwYuvConvRangeW::new(self, 5)
    }
    #[doc = "Bits 8:12 - the value of chrominance component range map\n\nVC- 1:c range map value +9"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rangemap_coef_c(&mut self) -> SwRangemapCoefCW<Swreg15Spec> {
        SwRangemapCoefCW::new(self, 8)
    }
}
#[doc = "range map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg15Spec;
impl crate::RegisterSpec for Swreg15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg15::R`](R) reader structure"]
impl crate::Readable for Swreg15Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg15::W`](W) writer structure"]
impl crate::Writable for Swreg15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG15 to value 0"]
impl crate::Resettable for Swreg15Spec {
    const RESET_VALUE: u32 = 0;
}
