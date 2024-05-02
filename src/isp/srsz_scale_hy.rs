#[doc = "Register `SRSZ_SCALE_HY` reader"]
pub type R = crate::R<SrszScaleHySpec>;
#[doc = "Register `SRSZ_SCALE_HY` writer"]
pub type W = crate::W<SrszScaleHySpec>;
#[doc = "Field `scale_hy` reader - This register is set to the horizontal luminance\n\ndownscale factor or to the reciprocal of the horizontal\n\nluminance upscale factor"]
pub type ScaleHyR = crate::FieldReader<u16>;
#[doc = "Field `scale_hy` writer - This register is set to the horizontal luminance\n\ndownscale factor or to the reciprocal of the horizontal\n\nluminance upscale factor"]
pub type ScaleHyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register is set to the horizontal luminance\n\ndownscale factor or to the reciprocal of the horizontal\n\nluminance upscale factor"]
    #[inline(always)]
    pub fn scale_hy(&self) -> ScaleHyR {
        ScaleHyR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is set to the horizontal luminance\n\ndownscale factor or to the reciprocal of the horizontal\n\nluminance upscale factor"]
    #[inline(always)]
    #[must_use]
    pub fn scale_hy(&mut self) -> ScaleHyW<SrszScaleHySpec> {
        ScaleHyW::new(self, 0)
    }
}
#[doc = "horizontal luminance scale factor register\n\nNote: The size of the output picture is calculated as follows: \n\nupscaling: (size_in - 1) / (size_out - 1)) = scale downscaling: (size_out - 1) / (size_in - 1)) \n\n\n\n= scale, \n\nwhere size_in/out is the width or height of the in/output picture. The value of the \n\n\n\nrespective SRSZ_SCALE register then has to be \n\nint(scale x 2^14) for upscaling and \n\nint(scale x 2^14)+1 for downscaling. \n\nFor downscaling this formula has no restriction. In upscaling processes the limit is factor 5. \n\n\n\nIf a format conversion is performed, the scale factors have to be different for the luminance \n\nand the chrominance component, respectively. For example, for a \n\n\n\nformat conversion from 4:2:2 to 4:2:0 the scale register value for the vertical \n\nchrominance component should be half of the vertical luminance scale register value. \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_scale_hy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srsz_scale_hy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrszScaleHySpec;
impl crate::RegisterSpec for SrszScaleHySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srsz_scale_hy::R`](R) reader structure"]
impl crate::Readable for SrszScaleHySpec {}
#[doc = "`write(|w| ..)` method takes [`srsz_scale_hy::W`](W) writer structure"]
impl crate::Writable for SrszScaleHySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRSZ_SCALE_HY to value 0"]
impl crate::Resettable for SrszScaleHySpec {
    const RESET_VALUE: u32 = 0;
}
