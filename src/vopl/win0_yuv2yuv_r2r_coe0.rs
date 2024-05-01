#[doc = "Register `WIN0_YUV2YUV_R2R_COE0` reader"]
pub type R = crate::R<Win0Yuv2yuvR2rCoe0Spec>;
#[doc = "Register `WIN0_YUV2YUV_R2R_COE0` writer"]
pub type W = crate::W<Win0Yuv2yuvR2rCoe0Spec>;
#[doc = "Field `CSC_COE00` reader - coefficient of 3x4 matrix"]
pub type CscCoe00R = crate::FieldReader<u16>;
#[doc = "Field `CSC_COE00` writer - coefficient of 3x4 matrix"]
pub type CscCoe00W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CSC_COE01` reader - coefficient of 3x4 matrix"]
pub type CscCoe01R = crate::FieldReader<u16>;
#[doc = "Field `CSC_COE01` writer - coefficient of 3x4 matrix"]
pub type CscCoe01W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - coefficient of 3x4 matrix"]
    #[inline(always)]
    pub fn csc_coe00(&self) -> CscCoe00R {
        CscCoe00R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - coefficient of 3x4 matrix"]
    #[inline(always)]
    pub fn csc_coe01(&self) -> CscCoe01R {
        CscCoe01R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - coefficient of 3x4 matrix"]
    #[inline(always)]
    #[must_use]
    pub fn csc_coe00(&mut self) -> CscCoe00W<Win0Yuv2yuvR2rCoe0Spec> {
        CscCoe00W::new(self, 0)
    }
    #[doc = "Bits 16:31 - coefficient of 3x4 matrix"]
    #[inline(always)]
    #[must_use]
    pub fn csc_coe01(&mut self) -> CscCoe01W<Win0Yuv2yuvR2rCoe0Spec> {
        CscCoe01W::new(self, 16)
    }
}
#[doc = "WIN0 yuv2yuv r2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_yuv2yuv_r2r_coe0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_yuv2yuv_r2r_coe0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win0Yuv2yuvR2rCoe0Spec;
impl crate::RegisterSpec for Win0Yuv2yuvR2rCoe0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win0_yuv2yuv_r2r_coe0::R`](R) reader structure"]
impl crate::Readable for Win0Yuv2yuvR2rCoe0Spec {}
#[doc = "`write(|w| ..)` method takes [`win0_yuv2yuv_r2r_coe0::W`](W) writer structure"]
impl crate::Writable for Win0Yuv2yuvR2rCoe0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN0_YUV2YUV_R2R_COE0 to value 0"]
impl crate::Resettable for Win0Yuv2yuvR2rCoe0Spec {
    const RESET_VALUE: u32 = 0;
}
