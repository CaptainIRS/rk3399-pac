#[doc = "Register `SWREG45_STRMD_ERROR_STATUS` reader"]
pub type R = crate::R<Swreg45StrmdErrorStatusSpec>;
#[doc = "Register `SWREG45_STRMD_ERROR_STATUS` writer"]
pub type W = crate::W<Swreg45StrmdErrorStatusSpec>;
#[doc = "Field `SW_STRMD_ERROR_STATUS` reader - cabac error status\n\nstrmd error status\n\nin HEVC &amp; H264, it is called cabac error status"]
pub type SwStrmdErrorStatusR = crate::FieldReader<u32>;
#[doc = "Field `SW_STRMD_ERROR_STATUS` writer - cabac error status\n\nstrmd error status\n\nin HEVC &amp; H264, it is called cabac error status"]
pub type SwStrmdErrorStatusW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `SW_COLMV_ERROR_REF_PICIDX` reader - colmv error ref picidx\n\nwhen sw_colmv_ref_error_sta is 1'b1, these bits are used for tell\n\nwhich dpb frame is invalid but is read by inter module\n\nit is for H264 and HEVC"]
pub type SwColmvErrorRefPicidxR = crate::FieldReader;
#[doc = "Field `SW_COLMV_ERROR_REF_PICIDX` writer - colmv error ref picidx\n\nwhen sw_colmv_ref_error_sta is 1'b1, these bits are used for tell\n\nwhich dpb frame is invalid but is read by inter module\n\nit is for H264 and HEVC"]
pub type SwColmvErrorRefPicidxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:27 - cabac error status\n\nstrmd error status\n\nin HEVC &amp; H264, it is called cabac error status"]
    #[inline(always)]
    pub fn sw_strmd_error_status(&self) -> SwStrmdErrorStatusR {
        SwStrmdErrorStatusR::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bits 28:31 - colmv error ref picidx\n\nwhen sw_colmv_ref_error_sta is 1'b1, these bits are used for tell\n\nwhich dpb frame is invalid but is read by inter module\n\nit is for H264 and HEVC"]
    #[inline(always)]
    pub fn sw_colmv_error_ref_picidx(&self) -> SwColmvErrorRefPicidxR {
        SwColmvErrorRefPicidxR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:27 - cabac error status\n\nstrmd error status\n\nin HEVC &amp; H264, it is called cabac error status"]
    #[inline(always)]
    #[must_use]
    pub fn sw_strmd_error_status(&mut self) -> SwStrmdErrorStatusW<Swreg45StrmdErrorStatusSpec> {
        SwStrmdErrorStatusW::new(self, 0)
    }
    #[doc = "Bits 28:31 - colmv error ref picidx\n\nwhen sw_colmv_ref_error_sta is 1'b1, these bits are used for tell\n\nwhich dpb frame is invalid but is read by inter module\n\nit is for H264 and HEVC"]
    #[inline(always)]
    #[must_use]
    pub fn sw_colmv_error_ref_picidx(
        &mut self,
    ) -> SwColmvErrorRefPicidxW<Swreg45StrmdErrorStatusSpec> {
        SwColmvErrorRefPicidxW::new(self, 28)
    }
}
#[doc = "cabac error status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg45_strmd_error_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg45_strmd_error_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg45StrmdErrorStatusSpec;
impl crate::RegisterSpec for Swreg45StrmdErrorStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg45_strmd_error_status::R`](R) reader structure"]
impl crate::Readable for Swreg45StrmdErrorStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg45_strmd_error_status::W`](W) writer structure"]
impl crate::Writable for Swreg45StrmdErrorStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG45_STRMD_ERROR_STATUS to value 0"]
impl crate::Resettable for Swreg45StrmdErrorStatusSpec {
    const RESET_VALUE: u32 = 0;
}
