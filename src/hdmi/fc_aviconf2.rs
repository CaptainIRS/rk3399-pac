#[doc = "Register `FC_AVICONF2` reader"]
pub type R = crate::R<FcAviconf2Spec>;
#[doc = "Register `FC_AVICONF2` writer"]
pub type W = crate::W<FcAviconf2Spec>;
#[doc = "Field `NON_UNIFORM_PICTURE_SCALING` reader - Non-uniform picture scaling"]
pub type NonUniformPictureScalingR = crate::FieldReader;
#[doc = "Field `NON_UNIFORM_PICTURE_SCALING` writer - Non-uniform picture scaling"]
pub type NonUniformPictureScalingW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `QUANTIZATION_RANGE` reader - Quantization range"]
pub type QuantizationRangeR = crate::FieldReader;
#[doc = "Field `QUANTIZATION_RANGE` writer - Quantization range"]
pub type QuantizationRangeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EXTENDED_COLORIMETRY` reader - Extended colorimetry"]
pub type ExtendedColorimetryR = crate::FieldReader;
#[doc = "Field `EXTENDED_COLORIMETRY` writer - Extended colorimetry"]
pub type ExtendedColorimetryW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IT_CONTENT` reader - IT content"]
pub type ItContentR = crate::BitReader;
#[doc = "Field `IT_CONTENT` writer - IT content"]
pub type ItContentW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Non-uniform picture scaling"]
    #[inline(always)]
    pub fn non_uniform_picture_scaling(&self) -> NonUniformPictureScalingR {
        NonUniformPictureScalingR::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Quantization range"]
    #[inline(always)]
    pub fn quantization_range(&self) -> QuantizationRangeR {
        QuantizationRangeR::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:6 - Extended colorimetry"]
    #[inline(always)]
    pub fn extended_colorimetry(&self) -> ExtendedColorimetryR {
        ExtendedColorimetryR::new((self.bits >> 4) & 7)
    }
    #[doc = "Bit 7 - IT content"]
    #[inline(always)]
    pub fn it_content(&self) -> ItContentR {
        ItContentR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Non-uniform picture scaling"]
    #[inline(always)]
    #[must_use]
    pub fn non_uniform_picture_scaling(&mut self) -> NonUniformPictureScalingW<FcAviconf2Spec> {
        NonUniformPictureScalingW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Quantization range"]
    #[inline(always)]
    #[must_use]
    pub fn quantization_range(&mut self) -> QuantizationRangeW<FcAviconf2Spec> {
        QuantizationRangeW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Extended colorimetry"]
    #[inline(always)]
    #[must_use]
    pub fn extended_colorimetry(&mut self) -> ExtendedColorimetryW<FcAviconf2Spec> {
        ExtendedColorimetryW::new(self, 4)
    }
    #[doc = "Bit 7 - IT content"]
    #[inline(always)]
    #[must_use]
    pub fn it_content(&mut self) -> ItContentW<FcAviconf2Spec> {
        ItContentW::new(self, 7)
    }
}
#[doc = "Non-uniform picture scaling\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_aviconf2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_aviconf2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAviconf2Spec;
impl crate::RegisterSpec for FcAviconf2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_aviconf2::R`](R) reader structure"]
impl crate::Readable for FcAviconf2Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_aviconf2::W`](W) writer structure"]
impl crate::Writable for FcAviconf2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_AVICONF2 to value 0"]
impl crate::Resettable for FcAviconf2Spec {
    const RESET_VALUE: u8 = 0;
}
