#[doc = "Register `CSC_SCALE` reader"]
pub type R = crate::R<CscScaleSpec>;
#[doc = "Register `CSC_SCALE` writer"]
pub type W = crate::W<CscScaleSpec>;
#[doc = "Field `CSCSCALE` reader - Defines the cscscale\\[1:0\\]
scale factor to apply to all coefficients in Color Space Conversion. This scale factor is expressed in the number of left shifts to apply to each of the coefficients, ranging from 0 to 2."]
pub type CscscaleR = crate::FieldReader;
#[doc = "Field `CSCSCALE` writer - Defines the cscscale\\[1:0\\]
scale factor to apply to all coefficients in Color Space Conversion. This scale factor is expressed in the number of left shifts to apply to each of the coefficients, ranging from 0 to 2."]
pub type CscscaleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPARE` reader - The is a Reserved as \"spare\" register with no associated functionality."]
pub type SpareR = crate::FieldReader;
#[doc = "Field `SPARE` writer - The is a Reserved as \"spare\" register with no associated functionality."]
pub type SpareW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CSC_COLOR_DEPTH` reader - Color space converter color depth configuration: csc_colordepth\\[3:0\\]
| Action 0000 | 24 bit per pixel video (8 bit per component). 0001-0011 | Reserved. Not used. 0100 | 24 bit per pixel video (8 bit per component). 0101 | 30 bit per pixel video (10 bit per component). 0110 | 36 bit per pixel video (12 bit per component). 0111 | 48 bit per pixel video (16 bit per component). other | Reserved. Not used."]
pub type CscColorDepthR = crate::FieldReader;
#[doc = "Field `CSC_COLOR_DEPTH` writer - Color space converter color depth configuration: csc_colordepth\\[3:0\\]
| Action 0000 | 24 bit per pixel video (8 bit per component). 0001-0011 | Reserved. Not used. 0100 | 24 bit per pixel video (8 bit per component). 0101 | 30 bit per pixel video (10 bit per component). 0110 | 36 bit per pixel video (12 bit per component). 0111 | 48 bit per pixel video (16 bit per component). other | Reserved. Not used."]
pub type CscColorDepthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Defines the cscscale\\[1:0\\]
scale factor to apply to all coefficients in Color Space Conversion. This scale factor is expressed in the number of left shifts to apply to each of the coefficients, ranging from 0 to 2."]
    #[inline(always)]
    pub fn cscscale(&self) -> CscscaleR {
        CscscaleR::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - The is a Reserved as \"spare\" register with no associated functionality."]
    #[inline(always)]
    pub fn spare(&self) -> SpareR {
        SpareR::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:7 - Color space converter color depth configuration: csc_colordepth\\[3:0\\]
| Action 0000 | 24 bit per pixel video (8 bit per component). 0001-0011 | Reserved. Not used. 0100 | 24 bit per pixel video (8 bit per component). 0101 | 30 bit per pixel video (10 bit per component). 0110 | 36 bit per pixel video (12 bit per component). 0111 | 48 bit per pixel video (16 bit per component). other | Reserved. Not used."]
    #[inline(always)]
    pub fn csc_color_depth(&self) -> CscColorDepthR {
        CscColorDepthR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Defines the cscscale\\[1:0\\]
scale factor to apply to all coefficients in Color Space Conversion. This scale factor is expressed in the number of left shifts to apply to each of the coefficients, ranging from 0 to 2."]
    #[inline(always)]
    #[must_use]
    pub fn cscscale(&mut self) -> CscscaleW<CscScaleSpec> {
        CscscaleW::new(self, 0)
    }
    #[doc = "Bits 2:3 - The is a Reserved as \"spare\" register with no associated functionality."]
    #[inline(always)]
    #[must_use]
    pub fn spare(&mut self) -> SpareW<CscScaleSpec> {
        SpareW::new(self, 2)
    }
    #[doc = "Bits 4:7 - Color space converter color depth configuration: csc_colordepth\\[3:0\\]
| Action 0000 | 24 bit per pixel video (8 bit per component). 0001-0011 | Reserved. Not used. 0100 | 24 bit per pixel video (8 bit per component). 0101 | 30 bit per pixel video (10 bit per component). 0110 | 36 bit per pixel video (12 bit per component). 0111 | 48 bit per pixel video (16 bit per component). other | Reserved. Not used."]
    #[inline(always)]
    #[must_use]
    pub fn csc_color_depth(&mut self) -> CscColorDepthW<CscScaleSpec> {
        CscColorDepthW::new(self, 4)
    }
}
#[doc = "Defines the cscscale\\[1:0\\]
scale factor to apply to all coefficients in Color Space Conversion. This scale factor is expressed in the number of left shifts to apply to each of the coefficients, ranging from 0 to 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_scale::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_scale::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CscScaleSpec;
impl crate::RegisterSpec for CscScaleSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`csc_scale::R`](R) reader structure"]
impl crate::Readable for CscScaleSpec {}
#[doc = "`write(|w| ..)` method takes [`csc_scale::W`](W) writer structure"]
impl crate::Writable for CscScaleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CSC_SCALE to value 0x01"]
impl crate::Resettable for CscScaleSpec {
    const RESET_VALUE: u8 = 0x01;
}
