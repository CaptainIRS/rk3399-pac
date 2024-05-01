#[doc = "Register `WIN2_YUV2YUV_R2R_COE7` reader"]
pub type R = crate::R<Win2Yuv2yuvR2rCoe7Spec>;
#[doc = "Register `WIN2_YUV2YUV_R2R_COE7` writer"]
pub type W = crate::W<Win2Yuv2yuvR2rCoe7Spec>;
#[doc = "Field `CSC_OFFSET2` reader - coefficient of 3x4 matrix"]
pub type CscOffset2R = crate::FieldReader<u32>;
#[doc = "Field `CSC_OFFSET2` writer - coefficient of 3x4 matrix"]
pub type CscOffset2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - coefficient of 3x4 matrix"]
    #[inline(always)]
    pub fn csc_offset2(&self) -> CscOffset2R {
        CscOffset2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - coefficient of 3x4 matrix"]
    #[inline(always)]
    #[must_use]
    pub fn csc_offset2(&mut self) -> CscOffset2W<Win2Yuv2yuvR2rCoe7Spec> {
        CscOffset2W::new(self, 0)
    }
}
#[doc = "WIN2 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_yuv2yuv_r2r_coe7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_yuv2yuv_r2r_coe7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win2Yuv2yuvR2rCoe7Spec;
impl crate::RegisterSpec for Win2Yuv2yuvR2rCoe7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win2_yuv2yuv_r2r_coe7::R`](R) reader structure"]
impl crate::Readable for Win2Yuv2yuvR2rCoe7Spec {}
#[doc = "`write(|w| ..)` method takes [`win2_yuv2yuv_r2r_coe7::W`](W) writer structure"]
impl crate::Writable for Win2Yuv2yuvR2rCoe7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN2_YUV2YUV_R2R_COE7 to value 0"]
impl crate::Resettable for Win2Yuv2yuvR2rCoe7Spec {
    const RESET_VALUE: u32 = 0;
}
