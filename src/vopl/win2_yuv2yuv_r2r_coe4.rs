#[doc = "Register `WIN2_YUV2YUV_R2R_COE4` reader"]
pub type R = crate::R<Win2Yuv2yuvR2rCoe4Spec>;
#[doc = "Register `WIN2_YUV2YUV_R2R_COE4` writer"]
pub type W = crate::W<Win2Yuv2yuvR2rCoe4Spec>;
#[doc = "Field `CSC_COE22` reader - coefficient of 3x4 matrix"]
pub type CscCoe22R = crate::FieldReader<u16>;
#[doc = "Field `CSC_COE22` writer - coefficient of 3x4 matrix"]
pub type CscCoe22W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - coefficient of 3x4 matrix"]
    #[inline(always)]
    pub fn csc_coe22(&self) -> CscCoe22R {
        CscCoe22R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - coefficient of 3x4 matrix"]
    #[inline(always)]
    #[must_use]
    pub fn csc_coe22(&mut self) -> CscCoe22W<Win2Yuv2yuvR2rCoe4Spec> {
        CscCoe22W::new(self, 0)
    }
}
#[doc = "WIN2 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_yuv2yuv_r2r_coe4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_yuv2yuv_r2r_coe4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win2Yuv2yuvR2rCoe4Spec;
impl crate::RegisterSpec for Win2Yuv2yuvR2rCoe4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win2_yuv2yuv_r2r_coe4::R`](R) reader structure"]
impl crate::Readable for Win2Yuv2yuvR2rCoe4Spec {}
#[doc = "`write(|w| ..)` method takes [`win2_yuv2yuv_r2r_coe4::W`](W) writer structure"]
impl crate::Writable for Win2Yuv2yuvR2rCoe4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN2_YUV2YUV_R2R_COE4 to value 0"]
impl crate::Resettable for Win2Yuv2yuvR2rCoe4Spec {
    const RESET_VALUE: u32 = 0;
}
