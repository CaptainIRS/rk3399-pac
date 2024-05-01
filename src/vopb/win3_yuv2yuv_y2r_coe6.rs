#[doc = "Register `WIN3_YUV2YUV_Y2R_COE6` reader"]
pub type R = crate::R<Win3Yuv2yuvY2rCoe6Spec>;
#[doc = "Register `WIN3_YUV2YUV_Y2R_COE6` writer"]
pub type W = crate::W<Win3Yuv2yuvY2rCoe6Spec>;
#[doc = "Field `CSC_OFFSET1` reader - coefficient of 3x4 matrix"]
pub type CscOffset1R = crate::FieldReader<u32>;
#[doc = "Field `CSC_OFFSET1` writer - coefficient of 3x4 matrix"]
pub type CscOffset1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - coefficient of 3x4 matrix"]
    #[inline(always)]
    pub fn csc_offset1(&self) -> CscOffset1R {
        CscOffset1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - coefficient of 3x4 matrix"]
    #[inline(always)]
    #[must_use]
    pub fn csc_offset1(&mut self) -> CscOffset1W<Win3Yuv2yuvY2rCoe6Spec> {
        CscOffset1W::new(self, 0)
    }
}
#[doc = "WIN3 yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_yuv2yuv_y2r_coe6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_yuv2yuv_y2r_coe6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win3Yuv2yuvY2rCoe6Spec;
impl crate::RegisterSpec for Win3Yuv2yuvY2rCoe6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win3_yuv2yuv_y2r_coe6::R`](R) reader structure"]
impl crate::Readable for Win3Yuv2yuvY2rCoe6Spec {}
#[doc = "`write(|w| ..)` method takes [`win3_yuv2yuv_y2r_coe6::W`](W) writer structure"]
impl crate::Writable for Win3Yuv2yuvY2rCoe6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN3_YUV2YUV_Y2R_COE6 to value 0"]
impl crate::Resettable for Win3Yuv2yuvY2rCoe6Spec {
    const RESET_VALUE: u32 = 0;
}
