#[doc = "Register `WIN2_YUV2YUV_R2Y_COE5` reader"]
pub type R = crate::R<Win2Yuv2yuvR2yCoe5Spec>;
#[doc = "Register `WIN2_YUV2YUV_R2Y_COE5` writer"]
pub type W = crate::W<Win2Yuv2yuvR2yCoe5Spec>;
#[doc = "Field `CSC_OFFSET0` reader - coefficient of 3x4 matrix"]
pub type CscOffset0R = crate::FieldReader<u32>;
#[doc = "Field `CSC_OFFSET0` writer - coefficient of 3x4 matrix"]
pub type CscOffset0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - coefficient of 3x4 matrix"]
    #[inline(always)]
    pub fn csc_offset0(&self) -> CscOffset0R {
        CscOffset0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - coefficient of 3x4 matrix"]
    #[inline(always)]
    #[must_use]
    pub fn csc_offset0(&mut self) -> CscOffset0W<Win2Yuv2yuvR2yCoe5Spec> {
        CscOffset0W::new(self, 0)
    }
}
#[doc = "WIN2 yuv2yuv r2y cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_yuv2yuv_r2y_coe5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_yuv2yuv_r2y_coe5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win2Yuv2yuvR2yCoe5Spec;
impl crate::RegisterSpec for Win2Yuv2yuvR2yCoe5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win2_yuv2yuv_r2y_coe5::R`](R) reader structure"]
impl crate::Readable for Win2Yuv2yuvR2yCoe5Spec {}
#[doc = "`write(|w| ..)` method takes [`win2_yuv2yuv_r2y_coe5::W`](W) writer structure"]
impl crate::Writable for Win2Yuv2yuvR2yCoe5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN2_YUV2YUV_R2Y_COE5 to value 0"]
impl crate::Resettable for Win2Yuv2yuvR2yCoe5Spec {
    const RESET_VALUE: u32 = 0;
}
